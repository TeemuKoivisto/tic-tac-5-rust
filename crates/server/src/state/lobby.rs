use log::{debug, error, info};
use std::collections::HashMap;
use tic_tac_5::proto::{client_events::*, game::*, server_events::*};
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::game::game_handle::GameHandle;
use crate::game::listed_game::ListedGame;
use crate::state::events::{ClientToLobbyEvent, LobbyToClientEvent};

use super::events::{ClientConnected, GameToClientEvent, GameToLobbyEvent};

#[derive(Debug)]
pub struct ClientSubscriber {
    socket_id: u32,
    game_sender: broadcast::Sender<GameToClientEvent>,
    lobby_sender: broadcast::Sender<LobbyToClientEvent>,
}

pub struct Lobby {
    pub running_games: HashMap<String, GameHandle>,
    pub lobby_games: Vec<ListedGame>,
    pub lobby_players: Vec<LobbyPlayer>,
    pub lobby_chat: Vec<String>,
    lobby_sender: broadcast::Sender<LobbyToClientEvent>,
    client_receiver: broadcast::Receiver<ClientToLobbyEvent>,
    game_sender: broadcast::Sender<GameToLobbyEvent>,
    game_receiver: broadcast::Receiver<GameToLobbyEvent>,
    subscribers: Vec<ClientConnected>,
}

impl Lobby {
    pub fn new(
        lobby_sender: broadcast::Sender<LobbyToClientEvent>,
        client_receiver: broadcast::Receiver<ClientToLobbyEvent>,
    ) -> Self {
        let (game_sender, game_receiver) = broadcast::channel(64);
        Self {
            running_games: HashMap::new(),
            lobby_games: Vec::new(),
            lobby_players: Vec::new(),
            lobby_chat: Vec::new(),
            lobby_sender,
            client_receiver,
            game_sender,
            game_receiver,
            subscribers: Vec::new(),
        }
    }

    pub fn lobby_state(&self) -> Vec<LobbyGame> {
        self.lobby_games
            .iter()
            .map(|game| LobbyGame {
                game_id: game.id.to_string(),
                players: game.joined_players.len() as u32,
                max_players: game.options.players,
            })
            .collect()
    }

    pub fn remove_player(&mut self, player_id: &u32, socket_id: u32) {
        self.lobby_players.retain(|p| &p.player_id != player_id);
        self.subscribers.retain(|s| &s.player_id != player_id);
        let mut removed = Vec::new();
        for game in self.lobby_games.iter_mut() {
            let empty = game.handle_player_leave(socket_id);
            if empty {
                removed.push(game.id);
            }
        }
        self.lobby_games.retain(|g| !removed.contains(&g.id));
    }

    pub fn find_game(&self, user_options: &GameOptions) -> Option<Uuid> {
        for game in &self.lobby_games {
            if game.allows_joining() && game.matches_player_options(user_options) {
                debug!("Joining existing game {:?}", game.id);
                return Some(game.id);
            }
        }
        None
    }

    pub fn create_game(&mut self, options: &GameOptions) -> Uuid {
        let game = ListedGame::new(options.clone());
        let game_id = game.id;
        debug!("Create new game id: {:?}", game_id);
        self.lobby_games.insert(0, game);
        game_id
    }

    pub fn find_or_create_game(&mut self, user_options: &GameOptions) -> Uuid {
        let found_game = self.find_game(user_options);
        if found_game.is_none() {
            self.create_game(user_options)
        } else {
            found_game.unwrap()
        }
    }

    pub fn find_and_join_listed_game(
        &mut self,
        game_id: &Uuid,
        payload: &PlayerJoinGame,
        socket_id: u32,
    ) -> Option<bool> {
        let found = self.lobby_games.iter_mut().find(|g| &g.id == game_id);
        if found.is_none() {
            return None;
        }
        let listed = found.unwrap();
        if listed.handle_player_join(&payload, socket_id) {
            let game = GameHandle::new(&listed, self.game_sender.clone());
            let mut left: Vec<u32> = Vec::new();
            for sub in &self.subscribers {
                let found = listed
                    .joined_players
                    .iter()
                    .find(|p| p.socket_id == Some(sub.socket_id))
                    .is_some();
                if found {
                    let _ = sub.game_sender.send(GameToClientEvent::Subscribe(
                        game.id.to_string(),
                        game.client_sender.clone(),
                    ));
                    left.push(sub.socket_id);
                }
            }
            self.send(LobbyToClientEvent::LeaveLobby(left.clone()));
            self.subscribers
                .retain(|sub| left.iter().find(|s| *s == &sub.socket_id).is_none());
            // TODO subscribe game to players and vice-versa
            // also leave lobby
            self.running_games.insert(game.id.to_string(), game);
            Some(true)
        } else {
            Some(false)
        }
    }

    pub fn start_game(&mut self, listed: &ListedGame) {
        let game = GameHandle::new(&listed, self.game_sender.clone());
        let mut left: Vec<u32> = Vec::new();
        for sub in &self.subscribers {
            let found = listed
                .joined_players
                .iter()
                .find(|p| p.socket_id == Some(sub.socket_id))
                .is_some();
            if found {
                let _ = sub.game_sender.send(GameToClientEvent::Subscribe(
                    game.id.to_string(),
                    game.client_sender.clone(),
                ));
                left.push(sub.socket_id);
            }
        }
        self.send(LobbyToClientEvent::LeaveLobby(left.clone()));
        self.subscribers
            .retain(|sub| left.iter().find(|s| *s == &sub.socket_id).is_none());
        // TODO subscribe game to players and vice-versa
        // also leave lobby
        self.running_games.insert(game.id.to_string(), game);
    }

    pub async fn handle_client_event(&mut self, msg: ClientToLobbyEvent) {
        info!("Lobby -> ClientToLobbyEvent {:?}", msg);
        match msg {
            ClientToLobbyEvent::Connected(payload) => {
                let mut join_lobby = true;
                if payload.waiting_game.is_some() {
                    let game_id = payload.waiting_game.as_ref().unwrap();
                    let found = self
                        .lobby_games
                        .iter_mut()
                        .find(|g| &g.id.to_string() == game_id);
                    if found.is_some() {
                        let game = found.unwrap();
                        let started = game.handle_player_join(
                            &PlayerJoinGame {
                                game_id: game_id.to_string(),
                                player_id: payload.player_id,
                                name: "".to_string(),
                                options: None,
                            },
                            payload.socket_id,
                        );
                        if !started {
                            let _ =
                                payload
                                    .lobby_sender
                                    .send(LobbyToClientEvent::PlayerJoinedGame(PlayerJoinedGame {
                                        game_id: game_id.to_string(),
                                        state: PlayerAppState::waiting_game_start,
                                    }));
                        }
                        join_lobby = !started;
                    }
                } else if payload.subscribed_games.len() > 0 {
                    for game_id in &payload.subscribed_games {
                        let found = self.running_games.get(game_id);
                        if found.is_some() {
                            let game = found.unwrap();
                            // @TODO send LeaveLobby and (re)subscribe to the game
                            let _ = payload.game_sender.send(GameToClientEvent::Subscribe(
                                game.id.to_string(),
                                game.client_sender.clone(),
                            ));
                            join_lobby = false;
                        }
                    }
                }
                if join_lobby {
                    let found = self
                        .subscribers
                        .iter()
                        .find(|s| s.player_id == payload.player_id);
                    // @TODO shouldnt have to check as disconnect removes the player
                    println!("ADD PLAYER {:?} {:?}", found, self.subscribers);
                    if found.is_none() {
                        self.subscribers.push(payload);
                    }
                } else {
                    self.send(LobbyToClientEvent::LeaveLobby(vec![payload.socket_id]));
                }
                self.send_lobby_state();
            }
            ClientToLobbyEvent::Disconnected(socket_id, player_id) => {
                self.remove_player(&player_id, socket_id);
            }
            ClientToLobbyEvent::PlayerCreateGame(socket_id, create_game) => {
                let game_id = self.find_or_create_game(create_game.options.as_ref().unwrap());
                let player_join = PlayerJoinGame {
                    game_id: game_id.to_string(),
                    player_id: create_game.player_id,
                    name: create_game.name,
                    options: create_game.options,
                };
                let found = self.find_and_join_listed_game(&game_id, &player_join, socket_id);
                info!("player {} create game found is {:?}", socket_id, found);
                if found.is_none() {
                    let sub = self.subscribers.iter().find(|s| s.socket_id == socket_id);
                    if sub.is_some() {
                        let _ =
                            sub.unwrap()
                                .lobby_sender
                                .send(LobbyToClientEvent::PlayerJoinedGame(PlayerJoinedGame {
                                    game_id: game_id.to_string(),
                                    state: PlayerAppState::waiting_game_start,
                                }));
                    }
                }
                self.send_lobby_state();
            }
            ClientToLobbyEvent::PlayerJoinGame(socket_id, payload) => {
                let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                let found = self.find_and_join_listed_game(&game_id, &payload, socket_id);
                if found.is_some() && !found.unwrap() {
                    // @TODO should send PlayerJoinedGame
                }
                self.send_lobby_state();
            }
            ClientToLobbyEvent::PlayerJoinLobby(data) => {
                self.lobby_players.insert(
                    0,
                    LobbyPlayer {
                        player_id: data.player_id,
                        name: data.name,
                    },
                );
                self.send_lobby_state();
            }
        }
    }

    pub async fn handle_game_event(&mut self, msg: GameToLobbyEvent) {
        info!("Lobby -> GameToLobbyEvent {:?}", msg);
        match msg {
            GameToLobbyEvent::GameEnded(game_id) => {
                self.running_games.remove(&game_id.to_string());
                self.lobby_games.retain(|g| g.id != game_id);
                self.send_lobby_state();
            }
        }
    }

    fn send(&mut self, event: LobbyToClientEvent) {
        self.subscribers
            .retain(|sub| sub.lobby_sender.send(event.clone()).is_ok());
    }

    fn send_lobby_state(&mut self) {
        let state = LobbyToClientEvent::LobbyState(LobbyState {
            games: self.lobby_state(),
            players: self.lobby_players.clone(),
        });
        self.subscribers
            .retain(|sub| sub.lobby_sender.send(state.clone()).is_ok());
    }
}

pub fn run_lobby(mut actor: Lobby) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(ev) = actor.client_receiver.recv() => {
                    actor.handle_client_event(ev).await;
                },
                Ok(ev) = actor.game_receiver.recv() => {
                    actor.handle_game_event(ev).await;
                },
                else => {
                    break;
                },
            }
        }
    })
}
