use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::connection::Connection;
use crate::connection::ConnectionManager;
use crate::game::game::Game;
use crate::game::game_manager::GameManager;
use crate::lobby::Lobby;
use tic_tac_5::{events::ServerEvent, proto::proto_all::*};

pub struct Context {
    // pub players: Arc<Mutex<Vec<Arc<Mutex<&PlayerContext>>>>>,
    pub game_manager_mutex: Arc<Mutex<GameManager>>,
    pub conn_manager_mutex: Arc<Mutex<ConnectionManager>>,
    lobby: Arc<Mutex<Lobby>>,
}

impl Context {
    pub fn new(g: Arc<Mutex<GameManager>>, c: Arc<Mutex<ConnectionManager>>) -> Self {
        Self {
            // players: Arc::new(Mutex::new(Vec::new())),
            game_manager_mutex: g,
            conn_manager_mutex: c,
            lobby: Arc::new(Mutex::new(Lobby::new())),
        }
    }

    pub async fn player_connect(&self, socket_id: u32, conn: Arc<Mutex<Connection>>) {
        // self.conn_manager_mutex
        //     .lock()
        //     .await
        //     .broadcast_server_event(ServerEvent::ClientConnected(socket_id))
        //     .await;
        let mut cm = self.conn_manager_mutex.lock().await;
        cm.add(socket_id, conn);
        cm.join_conn_to_room(&socket_id, &"*".to_string()).await;
    }

    // @TODO use socket_id, rooms as parameters instead(?)
    pub async fn player_disconnect(&self, conn_mut: Arc<Mutex<Connection>>) {
        let mut ended_games: Vec<Uuid> = Vec::new();
        let conn = conn_mut.lock().await;
        let socket_id = conn.id;
        let rooms = &conn.rooms;
        for room in rooms {
            let uuid = Uuid::parse_str(&room);
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
                .find(|p| p.socket_id == Some(socket_id));
            if player.is_none() {
                continue;
            }
            let player_id = player.unwrap().player_id;
            let player_leave = PlayerLeaveGame {
                game_id: game_id.to_string(),
                player_id,
            };
            game.handle_player_disconnect(&player_id);
            self.conn_manager_mutex
                .lock()
                .await
                .broadcast_server_event(ServerEvent::Quit(player_leave))
                .await;
            if game.is_waiting_and_empty() {
                ended_games.push(game_id);
            }
        }
        let mut game_manager = self.game_manager_mutex.lock().await;
        for game_id in ended_games {
            game_manager.remove_game(game_id);
        }
    }

    // @TODO use socket_id, rooms as parameters instead(?)
    pub async fn remove_player_connection(&self, conn_mut: Arc<Mutex<Connection>>) {
        self.conn_manager_mutex.lock().await.remove(conn_mut).await;
    }

    pub async fn join_lobby(&self, socket_id: u32, data: PlayerJoinLobby) {
        self.lobby.lock().await.player_join_lobby(data);
        let mut conn_manager = self.conn_manager_mutex.lock().await;
        conn_manager
            .join_conn_to_room(&socket_id, &"lobby".to_string())
            .await;
    }

    pub async fn create_lobby_game(
        &self,
        socket_id: u32,
        create_game: PlayerCreateGame,
    ) -> (bool, Arc<Mutex<Game>>) {
        let mut game_manager = self.game_manager_mutex.lock().await;
        let game_mut = game_manager
            .find_or_create_game(create_game.options.as_ref().unwrap())
            .await;
        let gm = game_mut.clone();
        let game_id = game_mut.lock().await.id;
        let mut game = gm.lock().await;
        let mut conn_manager = self.conn_manager_mutex.lock().await;
        conn_manager
            .join_conn_to_room(&socket_id, &game_id.to_string())
            .await;
        conn_manager
            .remove_conn_from_room(&socket_id, &"lobby".to_string())
            .await;
        self.lobby
            .lock()
            .await
            .player_leave_lobby(create_game.player_id);
        let player_join = PlayerJoinGame {
            game_id: game.id.to_string(),
            player_id: create_game.player_id,
            name: create_game.name,
            options: create_game.options,
        };
        let started = game.handle_player_join(&player_join, socket_id);
        conn_manager
            .broadcast_server_event(ServerEvent::PlayerJoin(player_join))
            .await;
        (started, game_mut)
    }

    pub async fn join_lobby_game(
        &self,
        socket_id: u32,
        payload: PlayerJoinGame,
    ) -> (bool, Arc<Mutex<Game>>) {
        let mut game_manager = self.game_manager_mutex.lock().await;
        let game_mut = game_manager
            .find_or_create_game(payload.options.as_ref().unwrap())
            .await;
        let gm = game_mut.clone();
        let game_id = game_mut.lock().await.id;
        let mut game = gm.lock().await;
        let mut conn_manager = self.conn_manager_mutex.lock().await;
        conn_manager
            .join_conn_to_room(&socket_id, &game_id.to_string())
            .await;
        conn_manager
            .remove_conn_from_room(&socket_id, &"lobby".to_string())
            .await;
        self.lobby
            .lock()
            .await
            .player_leave_lobby(payload.player_id);

        let started = game.handle_player_join(&payload, socket_id);
        conn_manager
            .broadcast_server_event(ServerEvent::PlayerJoin(payload))
            .await;
        (started, game_mut)
    }

    pub async fn start_game(&self, game_mut: Arc<Mutex<Game>>) {
        // let mut game_manager = self.game_manager_mutex.lock().await;
        // let _ = game_manager
        //     .broadcast
        //     .send(ServerEvent::GameStart(game.get_game_start()));
        self.conn_manager_mutex
            .lock()
            .await
            .broadcast_server_event(ServerEvent::GameStart(
                game_mut.lock().await.get_game_start(),
            ))
            .await;

        // let game_id = game_mut.lock().await.id;
        // let mut game_manager = self.game_manager_mutex.lock().await;
        // self.conn_manager_mutex
        //     .lock()
        //     .await
        //     .broadcast_server_event(ServerEvent::GameStart(
        //         game_mut.lock().await.get_game_start(),
        //     ))
        //     .await;
        // let game_receiver = game_manager.start_game(game_id).await;
        // let gm = self.game_manager_mutex.clone();
        // let cm = self.conn_manager_mutex.clone();
        // tokio::spawn(async move {
        //     let res = game_loop(game_mut, cm.clone(), game_receiver).await;
        //     cm.lock()
        //         .await
        //         .broadcast_server_event(ServerEvent::GameEnd(res.as_ref().unwrap().clone()))
        //         .await;
        //     let mut game_manager = gm.lock().await;
        //     let game_id = Uuid::parse_str(&res.unwrap().game_id).unwrap();
        //     game_manager.remove_game(game_id);
        //     let players = game_manager.lobby_players.clone();
        //     let games = game_manager.lobby_state().await;
        //     drop(game_manager);
        //     cm.lock().await.remove_room(game_id.to_string());
        //     cm.lock()
        //         .await
        //         .broadcast_server_event(ServerEvent::LobbyGames(LobbyState { games, players }))
        //         .await;
        // });
    }

    pub async fn end_game(&self, game_id: Uuid) {
        let game_manager = self.game_manager_mutex.lock().await;
        let game_mut = game_manager.get_game(&game_id).unwrap();
        let mut game = game_mut.lock().await;
        let (result, winner) = game.end_game();
        let mut cm = self.conn_manager_mutex.lock().await;
        cm.broadcast_server_event(ServerEvent::GameEnd(GameEnd {
            game_id: game_id.to_string(),
            result,
            winner: winner.cloned(),
        }))
        .await;
    }

    pub async fn remove_game(&self, game_id: Uuid) {
        let mut game_manager = self.game_manager_mutex.lock().await;
        game_manager.remove_game(game_id);
        let mut cm = self.conn_manager_mutex.lock().await;
        cm.remove_room(&game_id.to_string()).await;
    }

    pub async fn player_leave_game(&self, socket_id: u32, payload: PlayerLeaveGame) -> bool {
        let game_manager = self.game_manager_mutex.lock().await;
        let game_id = Uuid::parse_str(&payload.game_id).unwrap();
        let game_mut = game_manager.get_game(&game_id);
        if game_mut.is_some() {
            let mut game = game_mut.unwrap().lock().await;
            game.handle_player_leave(&payload.player_id);
            return game.is_waiting_and_empty();
        }
        self.conn_manager_mutex
            .lock()
            .await
            .remove_conn_from_room(&socket_id, &payload.game_id)
            .await;
        false
    }

    pub async fn broadcast_lobby_state(&self) {
        let game_manager = self.game_manager_mutex.lock().await;
        let games = game_manager.lobby_state().await;
        let lobby = self.lobby.lock().await;
        let players = lobby.lobby_players.clone();
        drop(game_manager);
        self.conn_manager_mutex
            .lock()
            .await
            .broadcast_server_event(ServerEvent::LobbyGames(LobbyState { games, players }))
            .await;
    }
}
