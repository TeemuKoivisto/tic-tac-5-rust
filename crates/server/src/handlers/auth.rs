use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response as AxumResponse},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::state::context::Context;

#[derive(Debug, Deserialize, Serialize)]
pub struct TicTac5Token {
    exp: i32,
    iat: i32,
    name: String,
    player_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    player_id: u32,
    token: String,
}

pub async fn login(
    State(state): State<Arc<Context>>,
    Json(payload): Json<LoginPayload>,
) -> AxumResponse {
    let player_id = state.session_manager.write().await.get_next_player_id();
    let token = state
        .jwt_manager
        .read()
        .await
        .encode_login(player_id, payload.name);
    tracing::trace!("token {}", token);
    Json(LoginResponse { player_id, token }).into_response()
}
