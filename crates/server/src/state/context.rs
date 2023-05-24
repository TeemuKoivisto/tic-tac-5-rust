use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::connection::Connection;
use crate::game::game::Game;
use tic_tac_5::{events::ServerEvent, proto::proto_all::*};

use super::{game_manager::GameManager, lobby::Lobby};

pub struct Context {
    pub game_manager: Arc<GameManager>,
    pub lobby: Arc<Lobby>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            game_manager: Arc::new(GameManager::new()),
            lobby: Arc::new(Lobby::new()),
        }
    }

    pub async fn player_connect(&self, socket_id: u32, conn: Arc<Mutex<Connection>>) {}

    // @TODO use socket_id, rooms as parameters instead(?)
    pub async fn player_disconnect(&self, conn_mut: Arc<Mutex<Connection>>) {}

    // @TODO use socket_id, rooms as parameters instead(?)
    pub async fn remove_player_connection(&self, conn_mut: Arc<Mutex<Connection>>) {}

    pub async fn join_lobby(&self, socket_id: u32, data: PlayerJoinLobby) {}

    pub async fn create_lobby_game() {}

    pub async fn join_lobby_game(&self, socket_id: u32, payload: PlayerJoinGame) {}

    pub async fn start_game(&self, game_mut: Arc<Mutex<Game>>) {}

    pub async fn end_game(&self, game_id: Uuid) {}

    pub async fn remove_game(&self, game_id: Uuid) {}

    pub async fn player_leave_game(&self, socket_id: u32, payload: PlayerLeaveGame) {}
}
