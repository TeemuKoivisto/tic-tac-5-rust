use axum::{
    routing::{get, post},
    Router,
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod game;
mod handlers;
mod state;
mod ws;

use crate::state::{context::Context, jwt_manager::JwtManager, lobby_handle::LobbyHandle};
use crate::{handlers::auth::login, ws::session_manager::SessionManager};
use handlers::ws::ws_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "".into()))
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

    let lobby = Arc::new(RwLock::new(LobbyHandle::new()));
    let session_manager = Arc::new(RwLock::new(SessionManager::new()));
    let jwt_manager = Arc::new(RwLock::new(JwtManager::new(&jwt_secret)));
    let ctx = Arc::new(Context::new(session_manager, lobby, jwt_manager));

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/login", post(login))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(ctx);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

    println!("Listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
