use log::{debug, error, info, warn};
use tic_tac_5::{
    game_state::*,
    proto::{game::*, server_events::*},
};
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use crate::state::events::{ClientToGameEvent, GameToClientEvent, GameToLobbyEvent, PlayerMove};

use super::listed_game::{JoinedPlayer, ListedGame};

#[derive(Debug)]
pub struct Subscriber {
    socket_id: u32,
    sender: broadcast::Sender<GameToClientEvent>,
}

pub struct Game {
    pub id: Uuid,
    pub state: GameState,
    pub joined_players: Vec<JoinedPlayer>,
    start_time: u64,
    game_sender: broadcast::Sender<GameToClientEvent>,
    game_to_lobby_sender: broadcast::Sender<GameToLobbyEvent>,
    pub client_receiver: broadcast::Receiver<ClientToGameEvent>,
    subscribers: Vec<Subscriber>,
}

impl Game {
    pub fn new(
        lobby_game: &ListedGame,
        rng_seed: Option<[u8; 32]>,
        game_sender: broadcast::Sender<GameToClientEvent>,
        game_to_lobby_sender: broadcast::Sender<GameToLobbyEvent>,
        client_receiver: broadcast::Receiver<ClientToGameEvent>,
    ) -> Self {
        Self {
            id: lobby_game.id,
            state: GameState::new(&lobby_game.options, rng_seed),
            joined_players: lobby_game.joined_players.clone(),
            start_time: chrono::Utc::now().timestamp_millis() as u64 / 1000,
            game_sender,
            game_to_lobby_sender,
            client_receiver,
            subscribers: Vec::new(),
        }
    }

    pub fn get_player_in_turn(&self) -> &Player {
        &self.state.players[(self.state.player_in_turn - 1) as usize]
    }

    pub fn get_player_game_state(&self) -> PlayerInGameState {
        if self.state.status == GameStatus::X_TURN {
            PlayerInGameState::x_turn
        } else if self.state.status == GameStatus::O_TURN {
            PlayerInGameState::o_turn
        } else if self.state.status == GameStatus::WAITING {
            PlayerInGameState::waiting_player
        } else {
            PlayerInGameState::ended
        }
    }

    pub fn get_board_state(&self) -> BoardState {
        BoardState {
            game_id: self.id.to_string(),
            start_time: self.start_time,
            player_in_turn: self.get_player_in_turn().id,
            players: self.state.players.clone(),
            cells: self.state.get_cells(),
            state: self.get_player_game_state(),
        }
    }

    pub fn check_if_running(&mut self) -> bool {
        if self.state.status == GameStatus::X_TURN || self.state.status == GameStatus::O_TURN {
            let time = chrono::Utc::now().timestamp_millis() as u64 / 1000;
            let mut disconnected = 0;
            for player in self.joined_players.iter() {
                if !player.connected && time > player.last_seen.unwrap() + 10 {
                    let game_player = self.state.get_player(player.player_id);
                    disconnected += 1;
                    if disconnected == 2 {
                        self.state.status = GameStatus::TIE
                    } else if game_player.symbol == "X" {
                        self.state.status = GameStatus::O_WON
                    } else if game_player.symbol == "O" {
                        self.state.status = GameStatus::X_WON
                    }
                }
            }
        }
        self.state.status == GameStatus::X_TURN || self.state.status == GameStatus::O_TURN
    }

    pub fn handle_player_leave(&mut self, player_id: &u32) {
        // TODO set disconnected & last connected time, remove later in game_loop if not reconnected before eg 15s timeout
        self.joined_players.retain(|p| p.player_id != *player_id);
        if self.state.status != GameStatus::WAITING {
            let player = self.state.get_player(*player_id);
            self.state.remove_player(player.player_number);
            let remaining = self.state.players.iter().filter(|p| !p.dead).count();
            if remaining == 1 {
                // TODO right player
                self.state.status = GameStatus::X_WON;
                // TODO send winner maybe
            } else if remaining == 0 {
                self.state.status = GameStatus::TIE;
            }
        }
    }

    pub fn handle_player_disconnect(&mut self, player_id: &u32) {
        // @TODO remove later in game_loop if not reconnected before eg 15s timeout
        for mut player in self.joined_players.iter_mut() {
            if &player.player_id == player_id {
                player.connected = false;
                player.last_seen = Some(chrono::Utc::now().timestamp_millis() as u64 / 1000);
            }
        }
    }

    pub fn handle_player_reconnect(&mut self, player_id: &u32) {
        for mut player in self.joined_players.iter_mut() {
            if &player.player_id == player_id {
                player.connected = true;
                player.last_seen = None;
            }
        }
    }

    fn get_game_end(&self) -> GameEnd {
        let winner_id;
        if self.state.status == GameStatus::X_WON {
            winner_id = self.state.players[0].id;
        } else if self.state.status == GameStatus::O_WON {
            winner_id = self.state.players[1].id;
        } else {
            winner_id = 0;
        }
        println!("game end {:?}", self.state.status);
        GameEnd {
            game_id: self.id.to_string(),
            result: self.state.status,
            winner_id,
            state: PlayerInGameState::ended,
        }
    }

    pub async fn handle_client_event(&mut self, msg: ClientToGameEvent) {
        info!("Game -> ClientToGameEvent {:?}", msg);
        match msg {
            ClientToGameEvent::SubscribeToGame(client, sender) => {
                self.subscribers.push(Subscriber {
                    socket_id: client.socket_id,
                    sender,
                });
                self.state
                    .add_player(&client.player_id, client.name, Some(client.socket_id));
                println!(
                    ">>> ClientToGameEvent::SubscribeToGame {} {}",
                    self.subscribers.len(),
                    self.joined_players.len()
                );
                if self.subscribers.len() == self.joined_players.len() {
                    self.state.status = GameStatus::X_TURN;
                    self.send(GameToClientEvent::GameStart(self.get_board_state()));
                }
            }
            ClientToGameEvent::Disconnected(_socket_id, player_id) => {
                // self.subscribers.retain(|sub| sub.socket_id != socket_id);
                self.handle_player_disconnect(&player_id);
                let player = self.state.get_player(player_id);
                println!("disconnected {:?}", player);
                self.send(GameToClientEvent::PlayerDisconnected(
                    GamePlayerDisconnected {
                        game_id: self.id.to_string(),
                        player_id,
                        symbol: player.symbol.clone(),
                        name: player.name.clone(),
                        state: PlayerInGameState::waiting_player,
                    },
                ));
            }
            ClientToGameEvent::Reconnected(socket_id, player_id) => {
                self.handle_player_reconnect(&player_id);
                self.send_to(
                    GameToClientEvent::GameStart(self.get_board_state()),
                    socket_id,
                );
                self.send(GameToClientEvent::PlayerReconnected(
                    GamePlayerReconnected {
                        game_id: self.id.to_string(),
                        player_id,
                        state: self.get_player_game_state(),
                    },
                ));
            }
            ClientToGameEvent::SelectCell(_socket_id, payload) => {
                let player_number = self.state.get_player(payload.player_id).player_number;
                let valid_move = self
                    .state
                    .handle_player_move(payload.x, payload.y, player_number);
                if valid_move.is_ok() {
                    let (did_win, next_in_turn) = valid_move.unwrap();
                    if did_win {
                        self.send_multiple(vec![
                            GameToClientEvent::GameUpdate(GameMove {
                                player_number: player_number,
                                next_in_turn,
                                x: payload.x,
                                y: payload.y,
                                state: PlayerInGameState::ended,
                            }),
                            GameToClientEvent::GameEnd(self.get_game_end()),
                        ]);
                        let _ = self
                            .game_to_lobby_sender
                            .send(GameToLobbyEvent::GameEnded(self.id));
                    } else {
                        println!(
                            "player {} moved, next in turn {} and enum {:?}",
                            player_number,
                            next_in_turn,
                            self.get_player_game_state()
                        );
                        self.send(GameToClientEvent::GameUpdate(GameMove {
                            player_number: player_number,
                            next_in_turn,
                            x: payload.x,
                            y: payload.y,
                            state: self.get_player_game_state(),
                        }));
                    }
                }
            }
            ClientToGameEvent::LeaveGame() => todo!(),
        }
    }

    fn send(&mut self, event: GameToClientEvent) {
        self.subscribers
            .retain(|sub| sub.sender.send(event.clone()).is_ok());
    }

    pub fn send_end_game(&mut self) {
        self.send(GameToClientEvent::GameEnd(self.get_game_end()));
        let _ = self
            .game_to_lobby_sender
            .send(GameToLobbyEvent::GameEnded(self.id));
    }

    fn send_to(&mut self, event: GameToClientEvent, socket_id: u32) {
        self.subscribers
            .retain(|sub| sub.socket_id != socket_id || sub.sender.send(event.clone()).is_ok());
    }

    fn send_multiple(&mut self, events: Vec<GameToClientEvent>) {
        for event in events {
            self.subscribers
                .retain(|sub| sub.sender.send(event.clone()).is_ok());
        }
    }
}
