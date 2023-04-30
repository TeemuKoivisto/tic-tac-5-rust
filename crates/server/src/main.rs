mod connection;
mod context;
mod game;
mod lobby;
mod player_context;
mod socket;

use crate::connection::*;
use crate::context::Context;
use crate::game::game_manager::GameManager;
use crate::socket::*;

use log::info;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

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

    let game_manager = Arc::new(Mutex::new(GameManager::new()));
    let conn_manager = Arc::new(Mutex::new(ConnectionManager::new()));
    let ctx = Arc::new(Context::new(game_manager, conn_manager));

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
                tokio::spawn(listen(ctx.clone(), ws_stream, socket_id));
            }
        }
    }
}
