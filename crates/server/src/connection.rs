use std::{collections::HashMap, sync::Arc};

use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use log::{debug, error, info, warn};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use tic_tac_5::{events::ServerEvent, proto::proto_all::*};

use crate::ws::serialize_server_event::serialize_server_event;

#[derive(Debug)]
pub struct Connection {
    pub id: u32,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    pub rooms: Vec<String>,
}

impl Connection {
    pub fn new(id: u32, sender: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self {
            id,
            sender,
            rooms: Vec::new(),
        }
    }
    pub fn is_in_room(&self, room: &String) -> bool {
        self.rooms.contains(room)
    }
    pub fn join_room(&mut self, room: &String) {
        self.rooms.push(room.to_string());
    }
    pub fn leave_room(&mut self, room: &String) {
        self.rooms.retain(|r| r != room);
    }
    pub async fn send(
        &mut self,
        msg: Message,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        let res = self.sender.send(msg).await;
        if res.is_err() {
            warn!("{:?}", res.as_ref().err());
        }
        res
    }
}

#[derive(Debug)]
pub struct ConnectionManager {
    connections: HashMap<u32, Arc<Mutex<Connection>>>,
    rooms: HashMap<String, Vec<u32>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
    pub fn add(&mut self, socket_id: u32, conn: Arc<Mutex<Connection>>) {
        self.connections.insert(socket_id, conn);
    }
    pub async fn remove(&mut self, conn_mut: Arc<Mutex<Connection>>) {
        let conn = conn_mut.lock().await;
        let socket_id = &conn.id;
        for room in conn.rooms.iter() {
            let connected = self.rooms.entry(room.clone()).or_insert(Vec::new());
            connected.retain(|id| id != socket_id);
            if connected.is_empty() {
                self.rooms.remove(room);
            }
        }
        self.connections.remove(socket_id);
    }
    pub async fn join_conn_to_room(&mut self, socket_id: &u32, room: &String) {
        info!("join conn {:?} to room {:?}", socket_id, room);
        let conn = self.connections.get(socket_id);
        if conn.is_none() {
            error!(
                "Tried to join non-existent connection {} to room {}",
                socket_id, room
            );
            return;
        }
        let locked = conn.unwrap().try_lock();
        if locked.is_err() {
            error!(
                "Tried to join a locked connection to room: {} {:?}",
                room,
                locked.err()
            );
        } else {
            let _ = locked.unwrap().join_room(room);
        }
        let connected = self.rooms.entry(room.to_string()).or_insert(Vec::new());
        connected.push(*socket_id);
    }
    pub async fn remove_conn_from_room(&mut self, socket_id: &u32, room: &String) {
        let conn = self.connections.get(socket_id);
        if conn.is_none() {
            error!(
                "Tried to remove non-existent connection {} from room {}",
                socket_id, room
            );
            return;
        }
        let locked = conn.unwrap().try_lock();
        if locked.is_err() {
            error!(
                "Tried to remove a locked connection from room: {} {:?}",
                room,
                locked.err()
            );
        } else {
            let _ = locked.unwrap().leave_room(room);
        }
        let connected = self.rooms.entry(room.clone()).or_insert(Vec::new());
        connected.retain(|id| id != socket_id);
        if connected.is_empty() {
            self.rooms.remove(room);
        }
    }
    pub async fn remove_room(&mut self, room: &String) {
        let sockets = self.rooms.get(room);
        if sockets.is_some() {
            for socket_id in sockets.unwrap() {
                let conn = self.connections.get(socket_id);
                let locked = conn.unwrap().try_lock();
                if locked.is_err() {
                    error!(
                        "Tried to remove a room with a locked connection! {:?}",
                        locked.err()
                    );
                } else {
                    let _ = locked.unwrap().leave_room(room);
                }
            }
        }
        self.rooms.remove(room);
    }
    // pub fn get(&self, id: &u32) -> Option<&Arc<tokio::sync::Mutex<Connection>>> {
    //     self.connection_map.get(id)
    // }
    // pub async fn send(&mut self, msg: Message, socket_id: &u32) {
    //     let conn = self.connection_map.get(socket_id);
    //     if conn.is_none() {
    //         error!("Tried to send to non-existent connection {}", socket_id);
    //         return;
    //     }
    //     let _ = conn.unwrap().lock().await.send(msg.clone()).await;
    // }
    pub async fn broadcast(&mut self, msg: Message, room: &String) {
        let sockets = self.rooms.get(room);
        if sockets.is_some() {
            for socket_id in sockets.unwrap() {
                let conn = self.connections.get(socket_id);
                if conn.is_some() {
                    let locked = conn.unwrap().try_lock();
                    if locked.is_err() {
                        error!(
                            "Tried to send to an already locked connection! {:?}",
                            locked.err()
                        );
                    } else {
                        let _ = locked.unwrap().send(msg.clone()).await;
                    }
                }
            }
        }
    }
    pub async fn broadcast_server_event(&mut self, event: ServerEvent) {
        match event {
            ServerEvent::ClientConnected(_client_id) => {
                // debug!("ServerEvents::ClientConnected {:?}", conn);
                // self.add(conn);
            }
            ServerEvent::ClientDisconnected(socket_id) => {
                debug!("ServerEvents::ClientDisconnected");
                // self.remove(&socket_id);
            }
            ServerEvent::LobbyGames(payload) => {
                debug!("ServerEvents::LobbyGames");
                self.broadcast(
                    serialize_server_event(ServerMsgType::lobby_state, &payload),
                    &"lobby".to_string(),
                )
                .await;
            }
            ServerEvent::PlayerMsg(_payload) => {}
            ServerEvent::PlayerJoinLobby(_payload) => {}
            ServerEvent::PlayerCreateGame(_payload) => {}
            ServerEvent::PlayerLeaveLobby(_payload) => {}
            ServerEvent::LobbyGameUpdated(_payload) => {}
            ServerEvent::PlayerJoin(_payload) => {}
            ServerEvent::GameStart(start) => {
                debug!("ServerEvents::GameStart");
                self.broadcast(
                    serialize_server_event(ServerMsgType::game_start, &start),
                    &start.game_id,
                )
                .await;
            }
            ServerEvent::GameEnd(payload) => {
                debug!("ServerEvents::GameEnd");
                self.broadcast(
                    serialize_server_event(ServerMsgType::game_end, &payload),
                    &payload.game_id,
                )
                .await;
            }
            ServerEvent::GameMove(game_id, payload) => {
                self.broadcast(
                    serialize_server_event(ServerMsgType::game_player_move, &payload),
                    &game_id,
                )
                .await;
            }
            ServerEvent::Quit(payload) => {
                debug!("ServerEvents::Quit");
                self.broadcast(
                    serialize_server_event(ServerMsgType::player_left, &payload),
                    &payload.game_id,
                )
                .await;
            }
        }
    }
}
