use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
};
// use log::{debug, error};
use log::warn;
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    state::{
        context::Context,
        jwt_manager::{JwtError, TicTac5Token},
    },
    ws::session::run_session,
};

#[derive(Deserialize)]
pub struct Params {
    jwt: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<Params>,
    State(state): State<Arc<Context>>,
) -> AxumResponse {
    let decoded = state.jwt_manager.read().await.decode(&params.jwt);
    let token: TicTac5Token;
    if decoded.as_ref().is_err() {
        match decoded.as_ref().unwrap_err() {
            JwtError::NoSession(t) => {
                token = t.clone();
                state
                    .jwt_manager
                    .write()
                    .await
                    .insert_session(params.jwt, t);
                tracing::info!("No session found for player {}", t.player_id);
            }
            JwtError::Expired => {
                warn!("Bearer token expired");
                return (StatusCode::UNAUTHORIZED, "Bearer token expired").into_response();
            }
            JwtError::Other(err) => {
                warn!("Invalid Bearer token");
                println!("{:?}", err);
                return (StatusCode::UNAUTHORIZED, "Invalid Bearer token").into_response();
            }
        }
    } else {
        tracing::info!("Session found!");
        token = decoded.unwrap();
    }
    ws.on_upgrade(move |socket| websocket(socket, token, state))
}

pub async fn websocket(socket: WebSocket, token: TicTac5Token, state: Arc<Context>) {
    let sm = &state.session_manager;
    let old = sm.write().await.pop_disconnected(&token);
    let session;
    if old.is_some() {
        session = sm.write().await.restore_session(socket, old.unwrap()).await;
        tracing::info!("RECONNECTED");
    } else {
        session = sm.write().await.create_session(socket);
        tracing::info!("CREATED");
    }
    let lobby = state.lobby.read().await;
    let _ = session.subscribe(&lobby.client_sender);
    let _ = lobby.subscribe(&session.lobby_sender);
    drop(lobby);
    tokio::select! {
        sess = (run_session(session)) => {
            if sess.is_ok() {
                sm.write().await.add_disconnected(sess.unwrap());
            }
        },
    };
}
