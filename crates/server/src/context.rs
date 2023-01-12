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
use crate::game::game::Game;
use crate::game::game_manager::GameManager;
use crate::game::write_server_msg::{serialize_server_event, write_server_msg};

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
            if game.is_empty() {
                ended_games.push(game_id);
            }
        }
        let mut game_manager = self.game_manager_mutex.lock().await;
        for game_id in ended_games {
            game_manager.remove_game(game_id);
        }
        self.conn_manager_mutex.lock().await.remove(socket_id);
    }

    pub async fn handle_join_lobby(&self, socket_id: u32, player_join: PlayerJoinLobby) {
        let mut game_manager = self.game_manager_mutex.lock().await;
        game_manager.player_join_lobby(player_join);
        let mut conn_manager = self.conn_manager_mutex.lock().await;
        conn_manager.join_conn_to_room(socket_id, "lobby".to_string());
    }

    pub async fn create_lobby_game(
        &self,
        socket_id: u32,
        create_game: PlayerCreateGame,
    ) -> (bool, Arc<tokio::sync::Mutex<Game>>) {
        let mut game_manager = self.game_manager_mutex.lock().await;
        let mut conn_manager = self.conn_manager_mutex.lock().await;
        let game_mut = game_manager
            .find_or_create_game(create_game.options.as_ref().unwrap())
            .await;
        let game_id = game_mut.lock().await.id;
        let mut started = false;
        {
            let mut game = game_mut.lock().await;
            conn_manager.join_conn_to_room(socket_id, game_id.to_string());
            conn_manager.remove_conn_from_room(socket_id, "lobby".to_string());
            game_manager.player_leave_lobby(create_game.player_id);
            let player_join = PlayerJoinGame {
                game_id: game.id.to_string(),
                player_id: create_game.player_id,
                name: create_game.name,
            };
            started = game.handle_player_join(&player_join, socket_id);
            let _ = game_manager
                .broadcast
                .send(ServerEvent::PlayerJoin(player_join));
            if started {
                let _ = game_manager
                    .broadcast
                    .send(ServerEvent::GameStart(game.get_game_start()));
            }
        }
        (started, game_mut)
    }

    pub async fn find_game(&self, game_id: String) -> Arc<Mutex<Game>> {
        let id = Uuid::parse_str(game_id.as_str());
        if id.is_err() {
            panic!("Non uuid value for game {}", game_id);
        }
        let game_manager = self.game_manager_mutex.lock().await;
        game_manager.get_game(&id.unwrap()).unwrap().clone()
    }

    pub async fn join_lobby_game(
        &self,
        socket_id: u32,
        game_mut: Arc<Mutex<Game>>,
        payload: PlayerJoinGame,
    ) -> bool {
        let mut game_manager = self.game_manager_mutex.lock().await;
        let mut conn_manager = self.conn_manager_mutex.lock().await;
        let mut started = false;
        let mut game = game_mut.lock().await;
        conn_manager.join_conn_to_room(socket_id, game.id.to_string());
        conn_manager.remove_conn_from_room(socket_id, "lobby".to_string());
        game_manager.player_leave_lobby(payload.player_id);
        started = game.handle_player_join(&payload, socket_id);
        let _ = game_manager
            .broadcast
            .send(ServerEvent::PlayerJoin(payload));
        if started {
            let _ = game_manager
                .broadcast
                .send(ServerEvent::GameStart(game.get_game_start()));
        }
        started
    }

    pub async fn start_game(&self, game_mut: Arc<Mutex<Game>>) {
        // let game_id = game_mut.lock().await.id;
        // let mut game_manager = self.game_manager_mutex.lock().await;
        // let game_receiver = game_manager.start_game(game_id).await;
        // let broadcast = game_manager.broadcast.clone();
        // let gm = self.game_manager_mutex.clone();
        // tokio::spawn(async move {
        //     let res = game_loop(game_mut, broadcast, game_receiver).await;
        //     let mut game_manager = gm.lock().await;
        //     let game_id = Uuid::parse_str(&res.unwrap().game_id).unwrap();
        //     game_manager.remove_game(game_id);
        // });
    }

    pub async fn handle_player_select_cell(&self, payload: PlayerSelectCell) -> bool {
        let game_id = payload.game_id.clone();
        let game_mut = self.find_game(game_id).await;
        let mut game = game_mut.lock().await;
        let game_ended = game.handle_player_move(&payload);
        if game_ended.is_err() {
            println!("Incorrect move: {}", game_ended.unwrap_err());
            return false;
        }
        let game_id = &payload.game_id;
        write_server_msg(
            ServerEvent::GameMove(
                game_id.to_string(),
                GameMove {
                    player: payload.player_number,
                    x: payload.x,
                    y: payload.y,
                },
            ),
            self.conn_manager_mutex.clone(),
        )
        .await;
        if game_ended.unwrap() {
            let winner = game.get_winner();
            let game_end = game.get_game_end(winner);
            write_server_msg(
                ServerEvent::GameEnd(game_end.clone()),
                self.conn_manager_mutex.clone(),
            )
            .await;
            self.game_manager_mutex
                .lock()
                .await
                .remove_game(Uuid::parse_str(&game_id).unwrap());
            self.conn_manager_mutex
                .lock()
                .await
                .remove_room(game_id.to_string());
            return true;
        }
        false
    }

    pub async fn broadcast_lobby_state(&self) {
        let game_manager = self.game_manager_mutex.lock().await;
        let games = game_manager.lobby_state().await;
        write_server_msg(
            ServerEvent::LobbyGames(LobbyState {
                games,
                players: game_manager.lobby_players.clone(),
            }),
            self.conn_manager_mutex.clone(),
        )
        .await;
    }
}
