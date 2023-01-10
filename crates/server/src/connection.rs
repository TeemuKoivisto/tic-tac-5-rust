use futures_util::sink::Send;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use log::{debug, info};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

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
            rooms: vec!["*".to_string()],
        }
    }
    pub fn is_in_room(&self, room: &String) -> bool {
        self.rooms.contains(room)
    }
    pub fn join_room(&mut self, room: String) {
        self.rooms.push(room);
    }
    // TODO: call this function
    pub fn leave_room(&mut self, room: String) {
        self.rooms.retain(|r| r != &room);
    }
    pub fn send(
        &mut self,
        msg: Message,
    ) -> Send<'_, SplitSink<WebSocketStream<TcpStream>, Message>, Message> {
        self.sender.send(msg)
    }
}

#[derive(Debug)]
pub struct ConnectionManager {
    connections: Vec<Connection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }
    pub fn add(&mut self, conn: Connection) {
        self.connections.push(conn);
    }
    pub fn remove(&mut self, id: u32) {
        self.connections.retain(|c| c.id != id);
    }
    pub fn join_conn_to_room(&mut self, id: u32, room: String) {
        info!("join conn {:?} to room {:?}", id, room);
        self.connections
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .join_room(room)
    }
    pub fn remove_conn_from_room(&mut self, id: u32, room: String) {
        self.connections
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .leave_room(room)
    }
    pub fn get(&self, id: u32) -> &Connection {
        self.connections.iter().find(|c| c.id == id).unwrap()
    }
    pub async fn send(&mut self, msg: Message, socket_id: u32) {
        for conn in self.connections.iter_mut() {
            if conn.id == socket_id {
                let _ = conn.send(msg.clone()).await;
            }
        }
    }
    pub async fn broadcast(&mut self, msg: Message, room: String) {
        debug!("hello broadcast to room {:?}", room);
        for conn in self.connections.iter_mut() {
            debug!("broadcasting to conn {:?}", conn);
            if conn.is_in_room(&room) {
                let _ = conn.send(msg.clone()).await;
            }
        }
    }
}
