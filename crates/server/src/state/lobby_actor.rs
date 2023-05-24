use log::{debug, error, info};
use std::collections::HashMap;
use tic_tac_5::proto::proto_all::*;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::game::game::Game;
use crate::state::events::{ClientEvent, LobbyEvent};

pub struct Subscriber {
    socket_id: u32,
    sender: broadcast::Sender<LobbyEvent>,
}

pub struct LobbyActor {
    pub games: HashMap<Uuid, Game>,
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
            games: HashMap::new(),
            lobby_players: Vec::new(),
            lobby_chat: Vec::new(),
            lobby_sender,
            client_receiver,
            subscribers: Vec::new(),
        }
    }

    pub fn lobby_state(&self) -> Vec<LobbyGame> {
        let mut games = Vec::new();
        for game in self.games.values() {
            games.insert(
                0,
                LobbyGame {
                    game_id: game.id.to_string(),
                    players: game.joined_players.len() as u32,
                    max_players: game.state.options.players,
                },
            );
        }
        games
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
