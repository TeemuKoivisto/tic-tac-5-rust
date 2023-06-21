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

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    player_id: u32,
    token: String,
    expires: u64,
}

pub async fn login(State(state): State<Arc<Context>>) -> AxumResponse {
    let player_id = state.session_manager.write().await.get_next_player_id();
    let (token, expires) = state.jwt_manager.read().await.encode_login(player_id);
    Json(LoginResponse {
        player_id,
        token,
        expires,
    })
    .into_response()
}
