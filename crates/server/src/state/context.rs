use std::sync::Arc;
use tokio::sync::RwLock;

use super::{jwt_manager::JwtManager, lobby_handle::LobbyHandle};
use crate::ws::session_manager::SessionManager;

pub struct Context {
    pub session_manager: Arc<RwLock<SessionManager>>,
    pub lobby: Arc<RwLock<LobbyHandle>>,
    pub jwt_manager: Arc<RwLock<JwtManager>>,
}

impl Context {
    pub fn new(
        d: Arc<RwLock<SessionManager>>,
        s: Arc<RwLock<LobbyHandle>>,
        j: Arc<RwLock<JwtManager>>,
    ) -> Self {
        Self {
            session_manager: d,
            lobby: s,
            jwt_manager: j,
        }
    }
}
