use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::game::game::Game;
use tic_tac_5::proto::proto_all::*;

use super::lobby::Lobby;

pub struct Context {
    pub lobby: Arc<Lobby>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            lobby: Arc::new(Lobby::new()),
        }
    }

    pub async fn player_connect(&self, socket_id: u32, conn: Arc<Mutex<u32>>) {}

    // @TODO use socket_id, rooms as parameters instead(?)
    pub async fn player_disconnect(&self, conn_mut: Arc<Mutex<u32>>) {}

    // @TODO use socket_id, rooms as parameters instead(?)
    pub async fn remove_player_connection(&self, conn_mut: Arc<Mutex<u32>>) {}

    pub async fn join_lobby(&self, socket_id: u32, data: PlayerJoinLobby) {}

    pub async fn create_lobby_game() {}

    pub async fn join_lobby_game(&self, socket_id: u32, payload: PlayerJoinGame) {}

    pub async fn start_game(&self, game_mut: Arc<Mutex<Game>>) {}

    pub async fn end_game(&self, game_id: Uuid) {}

    pub async fn remove_game(&self, game_id: Uuid) {}

    pub async fn player_leave_game(&self, socket_id: u32, payload: PlayerLeaveGame) {}
}
