use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

use crate::{
    state::{context::Context, session_manager},
    ws::{ws_session::run_session, ws_session_handle::WsSessionHandle},
};

pub async fn ws_handler(socket: WebSocketStream<TcpStream>, socket_id: u32, state: Arc<Context>) {
    let session = WsSessionHandle::new(socket, socket_id);
    // TODO add session to session manager
    tokio::select! {
        _ = (run_session(session.actor)) => {},
    };
}
