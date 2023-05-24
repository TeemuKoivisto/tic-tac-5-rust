use log::{debug, error};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::connection::Connection;
use crate::game::game::Game;
use crate::ws::serialize_server_event::serialize_server_event;
use tic_tac_5::proto::proto_all::*;

pub struct PlayerContext {
    pub socket_id: u32,
    // pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    // pub rooms: Vec<String>,
    pub joined_game: Option<Arc<Mutex<Game>>>,
    pub connection: Arc<Mutex<Connection>>,
}

impl PlayerContext {
    pub fn new(socket_id: u32, conn: Arc<Mutex<Connection>>) -> Self {
        Self {
            socket_id,
            joined_game: None,
            connection: conn,
        }
    }

    pub async fn set_joined_game(&mut self, game: Arc<Mutex<Game>>) {
        self.joined_game = Some(game);
    }

    pub async fn remove_joined_game(&mut self) {
        if self.joined_game.is_some() {
            // let g = self.joined_game.as_ref().unwrap().as_ref();
            let mut game = self.joined_game.as_ref().unwrap().lock().await;
            game.remove_player_connection(&self.socket_id);
        }
        self.joined_game = None;
    }

    pub async fn player_select_cell(&self, payload: &PlayerSelectCell) -> bool {
        if self.joined_game.is_none() {
            error!(
                "Tried to select a cell for a game that player had already left: {:?}",
                payload
            );
            return false;
        }
        let mut game = self.joined_game.as_ref().unwrap().lock().await;
        let game_ended = game.handle_player_move(&payload);
        if game_ended.is_err() {
            debug!("Incorrect move: {}", game_ended.unwrap_err());
            return false;
        }
        let msg = serialize_server_event(
            ServerMsgType::game_player_move,
            &GameMove {
                player: payload.player_number,
                x: payload.x,
                y: payload.y,
            },
        );
        game.broadcast_game_msg(msg).await;
        game_ended.unwrap()
    }
}
