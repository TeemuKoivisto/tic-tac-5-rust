use log::{debug, error, info, warn};
use tic_tac_5::{
    game_state::*,
    proto::{client_events::*, game::*, server_events::*},
};
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use crate::state::events::{ClientToGameEvent, GameToClientEvent, GameToLobbyEvent};

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
    pub fn get_winner(&self) -> Option<&Player> {
        if self.state.status == GameStatus::X_WON {
            return Some(&self.state.players[0]);
        } else if self.state.status == GameStatus::O_WON {
            return Some(&self.state.players[1]);
        }
        None
    }
    pub fn get_player_in_turn(&self) -> &Player {
        &self.state.players[(self.state.player_in_turn - 1) as usize]
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

    pub fn is_valid_move(&mut self, payload: &PlayerSelectCell) -> Option<String> {
        if payload.player_number != self.state.player_in_turn {
            return Some(format!(
                "Player {} tried to move when it was {} turn",
                payload.player_number, self.state.player_in_turn
            ));
        } else if !self
            .state
            .board
            .is_within_board(payload.x as i32, payload.y as i32)
        {
            return Some("Move's x, y weren't inside the board".to_string());
        }
        let cell = self.state.board.get_cell_at(payload.x, payload.y);
        if cell.owner != 0 {
            return Some(format!(
                "Cell at {} {} was already selected",
                cell.x, cell.y
            ));
        }
        None
    }

    pub fn handle_player_move(&mut self, payload: &PlayerSelectCell) -> Result<bool, String> {
        let err = self.is_valid_move(payload);
        if err.is_some() {
            return Err(err.unwrap());
        }
        self.state
            .player_move(payload.x, payload.y, payload.player_number);
        let did_win = self.state.check_win(payload.x, payload.y);
        if payload.player_number == self.state.options.players {
            self.state.player_in_turn = 1;
            self.state.status = if did_win {
                GameStatus::O_WON
            } else {
                GameStatus::O_TURN
            };
        } else {
            self.state.player_in_turn = payload.player_number + 1;
            self.state.status = if did_win {
                GameStatus::X_WON
            } else {
                GameStatus::X_TURN
            };
        }
        Ok(did_win)
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
        }
    }

    pub async fn handle_client_event(&mut self, msg: ClientToGameEvent) {
        info!("Game -> ClientToGameEvent {:?}", msg);
        match msg {
            ClientToGameEvent::Disconnected(socket_id, player_id) => {
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
                    },
                ));
            }
            ClientToGameEvent::Reconnected(socket_id, player_id) => {
                self.handle_player_reconnect(&player_id);
                self.send_to(
                    GameToClientEvent::GameStart(GameStart {
                        game_id: self.id.to_string(),
                        players: self.state.players.clone(),
                        cells: self.state.get_cells(),
                    }),
                    socket_id,
                );
                self.send(GameToClientEvent::PlayerReconnected(
                    GamePlayerReconnected {
                        game_id: self.id.to_string(),
                        player_id,
                    },
                ));
            }
            ClientToGameEvent::SelectCell(socket_id, payload) => {
                let result = self.handle_player_move(&payload);
                if result.is_ok() {
                    if result.unwrap() {
                        self.send_multiple(vec![
                            GameToClientEvent::GameUpdate(payload),
                            GameToClientEvent::GameEnd(self.get_game_end()),
                        ]);
                        let _ = self
                            .game_to_lobby_sender
                            .send(GameToLobbyEvent::GameEnded(self.id));
                    } else {
                        self.send(GameToClientEvent::GameUpdate(payload));
                    }
                }
            }
            ClientToGameEvent::LeaveGame() => todo!(),
            ClientToGameEvent::SubscribeToGame(client, sender) => {
                self.subscribers.push(Subscriber {
                    socket_id: client.socket_id,
                    sender,
                });
                self.state
                    .add_player(&client.player_id, client.name, Some(client.socket_id));
                println!(">>> ClientToGameEvent::SubscribeToGame");
                if self.subscribers.len() == self.joined_players.len() {
                    self.state.status = GameStatus::X_TURN;
                    self.send(GameToClientEvent::GameStart(GameStart {
                        game_id: self.id.to_string(),
                        players: self.state.players.clone(),
                        cells: self.state.get_cells(),
                    }));
                }
            }
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
