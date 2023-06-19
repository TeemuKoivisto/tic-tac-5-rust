use log::{debug, error, info};
use std::collections::HashMap;
use tic_tac_5::proto::proto_all::*;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::game::game_handle::GameHandle;
use crate::game::listed_game::ListedGame;
use crate::state::events::{ClientToLobbyEvent, LobbyToClientEvent};

use super::events::{GameToClientEvent, GameToLobbyEvent};

#[derive(Debug)]
pub struct ClientSubscriber {
    socket_id: u32,
    game_sender: broadcast::Sender<GameToClientEvent>,
    lobby_sender: broadcast::Sender<LobbyToClientEvent>,
}

pub struct LobbyActor {
    pub running_games: HashMap<Uuid, GameHandle>,
    pub lobby_games: Vec<ListedGame>,
    pub lobby_players: Vec<LobbyPlayer>,
    pub lobby_chat: Vec<String>,
    lobby_sender: broadcast::Sender<LobbyToClientEvent>,
    client_receiver: broadcast::Receiver<ClientToLobbyEvent>,
    game_sender: broadcast::Sender<GameToLobbyEvent>,
    game_receiver: broadcast::Receiver<GameToLobbyEvent>,
    subscribers: Vec<ClientSubscriber>,
}

impl LobbyActor {
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

    // pub fn remove_game(&mut self, game_id: Uuid) {
    //     self.games.remove(&game_id);
    // }

    // pub fn get_game(&self, game_id: &Uuid) -> Option<&Game> {
    //     self.games.get(game_id)
    // }

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

    pub async fn handle_client_event(&mut self, msg: ClientToLobbyEvent) {
        info!("Lobby -> ClientToLobbyEvent {:?}", msg);
        match msg {
            ClientToLobbyEvent::Connected(socket_id, lobby_sender, game_sender) => {
                info!("connected {}", socket_id);
                let _ = lobby_sender.send(LobbyToClientEvent::LobbyState(LobbyState {
                    games: self.lobby_state(),
                    players: self.lobby_players.clone(),
                }));
                self.subscribers.push(ClientSubscriber {
                    socket_id,
                    lobby_sender,
                    game_sender,
                });
            }
            ClientToLobbyEvent::Disconnected(socket_id) => {
                self.subscribers.retain(|sub| sub.socket_id != socket_id);
            }
            ClientToLobbyEvent::PlayerCreateGame(socket_id, create_game) => {
                let game_id = self.find_or_create_game(create_game.options.as_ref().unwrap());
                let player_join = PlayerJoinGame {
                    game_id: game_id.to_string(),
                    player_id: create_game.player_id,
                    name: create_game.name,
                    options: create_game.options,
                };
                info!("player {} create game", socket_id);
                for game in self.lobby_games.iter_mut() {
                    if game.id == game_id {
                        game.handle_player_join(&player_join, socket_id);
                    }
                }
                let sub = self.subscribers.iter().find(|s| s.socket_id == socket_id);
                if sub.is_some() {
                    let _ = sub
                        .unwrap()
                        .lobby_sender
                        .send(LobbyToClientEvent::PlayerJoinGame(player_join));
                }
                for sub in &self.subscribers {
                    let _ = sub
                        .lobby_sender
                        .send(LobbyToClientEvent::LobbyState(LobbyState {
                            games: self.lobby_state(),
                            players: self.lobby_players.clone(),
                        }));
                }
            }
            ClientToLobbyEvent::PlayerJoinGame(socket_id, payload) => {
                let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                let found = self.lobby_games.iter_mut().find(|g| g.id == game_id);
                debug!("Found game {:?}", found);
                if found.is_none() {
                    return;
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
                    self.running_games.insert(game.id, game);
                }
            }
            ClientToLobbyEvent::SelectCell(_, _) => todo!(),
            ClientToLobbyEvent::LeaveGame() => todo!(),
            ClientToLobbyEvent::PlayerJoinLobby(data) => {
                self.lobby_players.insert(
                    0,
                    LobbyPlayer {
                        player_id: data.player_id,
                        name: data.name,
                    },
                );
            }
            ClientToLobbyEvent::SubscribeToGame(_, _) => todo!(),
        }
    }

    pub async fn handle_game_event(&mut self, msg: GameToLobbyEvent) {
        info!("Lobby -> GameToLobbyEvent {:?}", msg);
        match msg {
            GameToLobbyEvent::GameEnded(game_id) => {
                self.running_games.remove(&game_id);
                self.lobby_games.retain(|g| g.id != game_id);
                for sub in &self.subscribers {
                    let _ = sub
                        .lobby_sender
                        .send(LobbyToClientEvent::LobbyState(LobbyState {
                            games: self.lobby_state(),
                            players: self.lobby_players.clone(),
                        }));
                }
            }
        }
    }

    fn send(&mut self, event: LobbyToClientEvent) {
        self.subscribers
            .retain(|sub| sub.lobby_sender.send(event.clone()).is_ok());
    }
}

pub fn run_lobby(mut actor: LobbyActor) -> JoinHandle<()> {
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
