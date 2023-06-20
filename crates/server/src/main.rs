use axum::{routing::get, Router};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod game;
mod handlers;
mod state;
mod ws;

use crate::state::{context::Context, jwt_manager::JwtManager, lobby_handle::LobbyHandle};
use crate::{handlers::auth_handlers::login, ws::session_manager::SessionManager};
use handlers::ws_handler::ws_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "6634".to_string())
        .parse::<u16>()
        .unwrap();
    let sentry_url = std::env::var("SENTRY_URL").unwrap_or_else(|_| "".to_string());
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let lobby = Arc::new(Mutex::new(LobbyHandle::new()));
    let session_manager = Arc::new(Mutex::new(SessionManager::new()));
    let jwt_manager = Arc::new(Mutex::new(JwtManager::new(&jwt_secret)));
    let ctx = Arc::new(Context::new(session_manager, lobby, jwt_manager));

    let app = Router::new()
        .route("/login", get(login))
        .route("/ws", get(ws_handler))
        .with_state(ctx);
    // .with_state(app_state);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());

    println!("Listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
