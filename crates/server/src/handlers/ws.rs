use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, Query, State,
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
    ws::{session::run_session, session_handle::SessionHandle},
};

#[derive(Deserialize)]
pub struct Params {
    jwt: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    // Query(params): Query<Params>,
    State(state): State<Arc<Context>>,
) -> AxumResponse {
    tracing::info!("haloo");
    // ws.on_upgrade(|socket| websocket2(socket, state))
    // let jwt = &params.jwt;
    // let mut jwt_manager = state.jwt_manager.lock().await;
    // let decoded = jwt_manager.decode(jwt);
    // let token: TicTac5Token;
    // if decoded.as_ref().is_err() {
    //     match decoded.as_ref().unwrap_err() {
    //         JwtError::NoSession(t) => {
    //             token = t.clone();
    //             jwt_manager.insert_session(jwt);
    //             warn!("No session, inserting");
    //             // return (StatusCode::UNAUTHORIZED, "No session found").into_response();
    //         }
    //         JwtError::Expired => {
    //             warn!("Bearer token expired");
    //             return (StatusCode::UNAUTHORIZED, "Bearer token expired").into_response();
    //         }
    //         JwtError::Other(err) => {
    //             warn!("Invalid Bearer token");
    //             println!("{:?}", err);
    //             return (StatusCode::UNAUTHORIZED, "Invalid Bearer token").into_response();
    //         }
    //     }
    // } else {
    //     token = decoded.unwrap();
    // }
    // drop(jwt_manager);
    ws.on_upgrade(move |socket| websocket(socket, state))
}

pub async fn websocket(socket: WebSocket, state: Arc<Context>) {
    println!("websocket");
    let lobby = state.lobby.read().await;
    let session = state.session_manager.write().await.create_session(socket);
    // let session = SessionHandle::new(socket, socket_id);
    let _ = session.subscribe(&lobby.client_sender);
    let _ = lobby.subscribe(&session.lobby_sender);
    drop(lobby);
    tokio::select! {
        _ = (run_session(session.actor)) => {},
    };
}
