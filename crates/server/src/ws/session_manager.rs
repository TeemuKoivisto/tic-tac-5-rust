use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::WebSocket;
use rand::{
    rngs::{OsRng, StdRng},
    Rng, RngCore, SeedableRng,
};

use crate::state::jwt_manager::TicTac5Token;

use super::{session::Session, session_handle::SessionHandle};

pub struct Connection {
    handle: SessionHandle,
    player_id: u32,
    socket_id: u32,
    connected: bool,
    last_seen: u32,
}

// list of session_senders
// SessionManagerToClient

pub struct SessionManager {
    sessions: Vec<SessionHandle>,
    session_map: HashMap<String, u32>,
    next_socket_id: u32,
    rng: StdRng,
    disconnected: Vec<Connection>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            session_map: HashMap::new(),
            next_socket_id: 0,
            rng: StdRng::from_seed(OsRng.gen()),
            disconnected: Vec::new(),
        }
    }

    pub fn add_disconnected(&mut self, session: SessionHandle) {
        let player_id = session.actor.client.player_id;
        let socket_id = session.actor.client.socket_id;
        self.disconnected.push(Connection {
            handle: session,
            player_id,
            socket_id,
            connected: false,
            last_seen: 0,
        })
    }

    pub fn get_disconnected(&mut self, token: TicTac5Token) -> Option<Connection> {
        let idx = self
            .disconnected
            .iter()
            .position(|s| s.player_id == token.player_id);
        if idx.is_some() {
            Some(self.disconnected.remove(idx.unwrap()))
        } else {
            None
        }
    }

    pub fn find_session(&self, token: TicTac5Token) -> Option<&SessionHandle> {
        self.sessions
            .iter()
            .find(|s| s.actor.client.player_id == token.player_id)
    }

    pub fn create_session(&mut self, socket: WebSocket, token: TicTac5Token) -> SessionHandle {
        let mut socket_id = self.next_socket_id;
        self.next_socket_id += 1;
        let handle = SessionHandle::new(socket, socket_id);
        handle
    }

    pub fn find_connection(&self, player_id: u32) -> Option<&Connection> {
        self.disconnected.iter().find(|c| c.player_id == player_id)
    }

    pub fn get_next_player_id(&mut self) -> u32 {
        self.rng.next_u32()
    }
}
