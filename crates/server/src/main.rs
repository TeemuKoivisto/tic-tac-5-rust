mod game;
mod state;
mod ws;

use crate::state::lobby_handle::LobbyHandle;
use crate::ws::ws_session::run_session;
use crate::ws::ws_session_handle::WsSessionHandle;

use log::info;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();
    let port = std::env::var("PORT").unwrap_or_else(|_| "6634".to_string());
    let sentry_url = std::env::var("SENTRY_URL").unwrap_or_else(|_| "".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Listening to TCP failed.");

    let lobby = Arc::new(LobbyHandle::new());

    println!("Listening on: {}", addr);

    // A counter to use as client ids.
    let mut socket_id = 0;

    // Accept new clients.
    while let Ok((stream, peer)) = listener.accept().await {
        match tokio_tungstenite::accept_async(stream).await {
            Err(e) => info!("Websocket connection error : {}", e),
            Ok(ws_stream) => {
                socket_id += 1;
                info!("New Connection {} Socket ID {}", peer, socket_id);
                let session = WsSessionHandle::new(ws_stream, socket_id);
                let _ = session.subscribe(&lobby.client_sender);
                let _ = lobby.subscribe(&session.lobby_sender);
                run_session(session.actor);
            }
        }
    }
}
