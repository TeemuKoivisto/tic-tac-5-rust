use hmac::Hmac;
use sha2::Sha512;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{jwt_manager::JwtManager, lobby_handle::LobbyHandle};
use crate::ws::session_manager::SessionManager;

pub struct Context {
    pub session_manager: Arc<Mutex<SessionManager>>,
    pub lobby: Arc<Mutex<LobbyHandle>>,
    pub jwt_manager: Arc<Mutex<JwtManager>>,
}

impl Context {
    pub fn new(
        d: Arc<Mutex<SessionManager>>,
        s: Arc<Mutex<LobbyHandle>>,
        j: Arc<Mutex<JwtManager>>,
    ) -> Self {
        Self {
            session_manager: d,
            lobby: s,
            jwt_manager: j,
        }
    }
}
