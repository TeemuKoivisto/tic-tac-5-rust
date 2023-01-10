use futures_util::stream::SplitSink;
use log::{debug, error};
use std::sync::Arc;
use tic_tac_5::{
    events::{GameEvent, ServerEvent},
    proto::proto_all::*,
};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;

use crate::connection::Connection;
use crate::connection::ConnectionManager;
use crate::game::game_manager::GameManager;
use crate::game::write_server_msg::serialize_server_event;

pub struct Context {
    pub game_manager_mutex: Arc<Mutex<GameManager>>,
    pub conn_manager_mutex: Arc<Mutex<ConnectionManager>>,
}

impl Context {
    pub fn new(g: Arc<Mutex<GameManager>>, c: Arc<Mutex<ConnectionManager>>) -> Self {
        Self {
            game_manager_mutex: g,
            conn_manager_mutex: c,
        }
    }

    pub async fn handle_join_lobby(&self, socket_id: u32, player_join: PlayerJoinLobby) {
        let mut game_manager = self.game_manager_mutex.lock().await;
        game_manager.player_join_lobby(player_join);
        let mut conn_manager = self.conn_manager_mutex.lock().await;
        conn_manager.join_conn_to_room(socket_id, "lobby".to_string());
        let games = game_manager.lobby_state().await;
        conn_manager
            .send(
                serialize_server_event(
                    ServerMsgType::lobby_state,
                    &LobbyState {
                        games,
                        players: game_manager.lobby_players.clone(),
                    },
                ),
                socket_id,
            )
            .await;
    }

    pub async fn handle_player_connect(
        &self,
        socket_id: u32,
        sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    ) {
        let conn = Connection::new(socket_id, sender);
        self.game_manager_mutex
            .lock()
            .await
            .broadcast
            .send(ServerEvent::ClientConnected(socket_id))
            .unwrap();
        self.conn_manager_mutex.lock().await.add(conn);
    }

    pub async fn handle_player_disconnect(&self, socket_id: u32) {
        let mut ended_games: Vec<Uuid> = Vec::new();
        for room in &self.conn_manager_mutex.lock().await.get(socket_id).rooms {
            let uuid = Uuid::parse_str(room);
            if uuid.is_err() {
                continue;
            }
            let game_id = uuid.unwrap();
            let game_manager = self.game_manager_mutex.lock().await;
            let game_mut = game_manager.get_game(&game_id);
            if game_mut.is_none() {
                continue;
            }
            let mut game = game_mut.unwrap().lock().await;
            let player = game
                .joined_players
                .iter()
                .find(|p| p.socket_id == socket_id)
                .unwrap();
            let player_id = player.player_id;
            let player_leave = PlayerLeave {
                game_id: game_id.to_string(),
                player_id: player_id,
            };
            game.handle_player_disconnect(&player_id);
            let _ = game_manager.broadcast.send(ServerEvent::Quit(player_leave));
            if game.is_waiting_and_empty() {
                ended_games.push(game_id);
            }
        }
        let mut game_manager = self.game_manager_mutex.lock().await;
        for game_id in ended_games {
            game_manager.remove_game(game_id);
        }
        self.conn_manager_mutex.lock().await.remove(socket_id);
    }
}
