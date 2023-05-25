use log::{debug, error, info};
use std::collections::HashMap;
use std::f32::consts::E;
use tic_tac_5::proto::proto_all::*;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::game::game::Game;
use crate::game::game_handle::GameHandle;
use crate::game::listed_game::ListedGame;
use crate::state::events::{ClientEvent, LobbyEvent};

pub struct Subscriber {
    socket_id: u32,
    sender: broadcast::Sender<LobbyEvent>,
}

pub struct LobbyActor {
    pub running_games: HashMap<Uuid, GameHandle>,
    pub lobby_games: Vec<ListedGame>,
    pub lobby_players: Vec<LobbyPlayer>,
    pub lobby_chat: Vec<String>,
    lobby_sender: broadcast::Sender<LobbyEvent>,
    client_receiver: broadcast::Receiver<ClientEvent>,
    subscribers: Vec<Subscriber>,
}

impl LobbyActor {
    pub fn new(
        lobby_sender: broadcast::Sender<LobbyEvent>,
        client_receiver: broadcast::Receiver<ClientEvent>,
    ) -> Self {
        Self {
            running_games: HashMap::new(),
            lobby_games: Vec::new(),
            lobby_players: Vec::new(),
            lobby_chat: Vec::new(),
            lobby_sender,
            client_receiver,
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

        // let mut games = Vec::new();
        // for game in self.running_games.values() {
        //     games.insert(
        //         0,
        //         LobbyGame {
        //             game_id: game.id.to_string(),
        //             players: game.joined_players.len() as u32,
        //             max_players: game.state.options.players,
        //         },
        //     );
        // }
        // games
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

    pub async fn handle_client_event(&mut self, msg: ClientEvent) {
        info!("Lobby -> ClientEvent {:?}", msg);
        match msg {
            ClientEvent::Connected(socket_id, sender, _) => {
                let _ = sender.send(LobbyEvent::LobbyState(LobbyState {
                    games: self.lobby_state(),
                    players: self.lobby_players.clone(),
                }));
                self.subscribers.push(Subscriber { socket_id, sender });
            }
            ClientEvent::Disconnected(socket_id) => {
                self.subscribers.retain(|sub| sub.socket_id != socket_id);
            }
            ClientEvent::PlayerCreateGame(socket_id, create_game) => {
                let game_id = self.find_or_create_game(create_game.options.as_ref().unwrap());
                let player_join = PlayerJoinGame {
                    game_id: game_id.to_string(),
                    player_id: create_game.player_id,
                    name: create_game.name,
                    options: create_game.options,
                };
                // self.lobby_games[0].handle_player_join(&player_join, socket_id);
                // self.lobby_games.into_iter().
                // self.lobby_games.iter_mut().find(|game| {
                //     if game.id == game_id {
                //         game.handle_player_join(&player_join, socket_id);
                //     }
                //     game.id == game_id
                // });
                for game in self.lobby_games.iter_mut() {
                    if game.id == game_id {
                        game.handle_player_join(&player_join, socket_id);
                    }
                }
                // game.handle_player_join(&player_join, socket_id);
                let sub = self.subscribers.iter().find(|s| s.socket_id == socket_id);
                if sub.is_some() {
                    let _ = sub
                        .unwrap()
                        .sender
                        .send(LobbyEvent::PlayerJoinGame(player_join));
                }
            }
            ClientEvent::PlayerJoinGame(socket_id, payload) => {
                let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                let found = self.lobby_games.iter_mut().find(|g| g.id == game_id);
                if found.is_none() {
                    return;
                }
                let game = found.unwrap();
                let sub = self.subscribers.iter().find(|s| s.socket_id == socket_id);
                if game.handle_player_join(&payload, socket_id) {
                    let game = GameHandle::new(&game);
                    // TODO subscribe game to players and vice-versa
                    // also leave lobby
                    self.running_games.insert(game.id, game);
                }
            }
            ClientEvent::SelectCell() => todo!(),
            ClientEvent::LeaveGame() => todo!(),
            ClientEvent::PlayerJoinLobby(data) => {
                self.lobby_players.insert(
                    0,
                    LobbyPlayer {
                        player_id: data.player_id,
                        name: data.name,
                    },
                );
            }
        }
    }
    fn send(&mut self, event: LobbyEvent) {
        self.subscribers
            .retain(|sub| sub.sender.send(event.clone()).is_ok());
    }
}

pub fn run_lobby(mut actor: LobbyActor) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(ev) = actor.client_receiver.recv() => {
                    actor.handle_client_event(ev).await;
                },
                else => {
                    break;
                },
            }
        }
    })
}
