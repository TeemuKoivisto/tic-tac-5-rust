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
    tracing::info!("haloo {}", params.jwt);
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
                // return (StatusCode::UNAUTHORIZED, "No session found").into_response();
                // return ws.on_upgrade(move |socket| websocket(socket, t.clone(), state));
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
    let mut sm = &state.session_manager;
    let session = sm.write().await.create_session(socket, token);
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
