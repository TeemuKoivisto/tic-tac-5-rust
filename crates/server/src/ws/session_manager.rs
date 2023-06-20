use std::collections::HashMap;

use super::session::Session;

pub struct Connection {
    player_id: u32,
    socket_id: u32,
    connected: bool,
    last_seen: u32,
}

pub struct SessionManager {
    sessions: Vec<Session>,
    session_map: HashMap<String, u32>,
    disconnected: Vec<Connection>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            session_map: HashMap::new(),
            disconnected: Vec::new(),
        }
    }

    pub fn set_disconnected(&mut self, session: &Session) {
        self.disconnected.push(Connection {
            player_id: session.client.player_id,
            socket_id: session.client.socket_id,
            connected: false,
            last_seen: 0,
        })
    }

    pub fn create_session() {}

    pub fn find_connection(&self, player_id: u32) -> Option<&Connection> {
        self.disconnected.iter().find(|c| c.player_id == player_id)
    }
}
