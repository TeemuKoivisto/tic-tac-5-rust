use axum::extract::ws::WebSocket;
use rand::{
    rngs::{OsRng, StdRng},
    Rng, RngCore, SeedableRng,
};

use crate::state::jwt_manager::TicTac5Token;

use super::session_handle::SessionHandle;

pub struct Connection {
    pub handle: SessionHandle,
    player_id: u32,
    socket_id: u32,
    last_seen: u64,
}

pub struct SessionManager {
    next_socket_id: u32,
    rng: StdRng,
    disconnected: Vec<Connection>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            next_socket_id: 0,
            rng: StdRng::from_seed(OsRng.gen()),
            disconnected: Vec::new(),
        }
    }

    pub fn add_disconnected(&mut self, session: SessionHandle) {
        let player_id = session.actor.state.player_id;
        let socket_id = session.actor.socket_id;
        self.disconnected.push(Connection {
            handle: session,
            player_id,
            socket_id,
            last_seen: chrono::Utc::now().timestamp_millis() as u64 / 1000,
        })
    }

    pub fn pop_disconnected(&mut self, token: &TicTac5Token) -> Option<Connection> {
        let dis = self
            .disconnected
            .iter()
            .map(|d| (d.player_id, d.socket_id))
            .collect::<Vec<(u32, u32)>>();
        println!("DISCONNECTED: {:?}", dis);
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

    pub fn create_session(&mut self, socket: WebSocket) -> SessionHandle {
        let socket_id = self.next_socket_id;
        self.next_socket_id += 1;
        let handle = SessionHandle::new(socket, socket_id);
        handle
    }

    pub async fn restore_session(
        &mut self,
        socket: WebSocket,
        mut conn: Connection,
    ) -> SessionHandle {
        conn.handle.restore(socket).await;
        conn.handle
    }

    pub fn get_next_player_id(&mut self) -> u32 {
        self.rng.next_u32()
    }
}
