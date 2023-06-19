use log::{debug, error, info, warn};
use std::{collections::HashMap, sync::Arc};
use tic_tac_5::{game_state::*, proto::proto_all::*};
use tokio::sync::Mutex;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use crate::state::events::{
    ClientToGameEvent, GameToClientEvent, GameToLobbyEvent, LobbyToClientEvent,
};

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

    pub fn handle_game_start(&mut self) {
        for player in &self.joined_players {
            self.state
                .add_player(&player.player_id, player.name.clone(), player.socket_id);
        }
        self.state.status = GameStatus::X_TURN;
    }

    pub fn end_game(&mut self) -> (GameStatus, Option<&Player>) {
        let status = self.state.status;
        if status == GameStatus::X_WON {
            return (GameStatus::X_WON, Some(&self.state.players[0]));
        } else if status == GameStatus::O_WON {
            return (GameStatus::O_WON, Some(&self.state.players[1]));
        } else {
            self.state.status = GameStatus::TIE;
        }
        (GameStatus::TIE, None)
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
                self.state.status = GameStatus::X_WON
                // TODO send winner maybe
            } else if remaining == 0 {
                self.state.status = GameStatus::TIE;
            }
        }
    }

    pub fn handle_player_disconnect(&mut self, player_id: &u32) {
        // @TODO set disconnected & last connected time, remove later in game_loop if not reconnected before eg 15s timeout
        self.handle_player_leave(player_id);
    }

    pub fn get_game_start(&self) -> GameStart {
        GameStart {
            game_id: self.id.to_string(),
            players: self.state.players.clone(),
            cells: self.state.get_cells(),
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
        GameEnd {
            game_id: self.id.to_string(),
            result: self.state.status,
            winner_id,
        }
    }

    pub async fn handle_client_event(&mut self, msg: ClientToGameEvent) {
        info!("Game -> ClientToGameEvent {:?}", msg);
        match msg {
            ClientToGameEvent::Connected(_) => todo!(),
            ClientToGameEvent::Disconnected(_) => todo!(),
            ClientToGameEvent::SelectCell(socket_id, payload) => {
                let result = self.handle_player_move(&payload);
                if result.is_ok() {
                    if result.unwrap() {
                        //     player.remove_joined_game().await;
                        //     let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                        //     ctx.end_game(game_id).await;
                        //     ctx.remove_game(game_id).await;
                        //     ctx.broadcast_lobby_state().await;
                        // }
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

    fn send_multiple(&mut self, events: Vec<GameToClientEvent>) {
        for event in events {
            self.subscribers
                .retain(|sub| sub.sender.send(event.clone()).is_ok());
        }
    }
}
