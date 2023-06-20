use axum::{
    extract::{Json, Path, Query, State},
    http::{header::HeaderMap, StatusCode},
    response::{IntoResponse, Response as AxumResponse},
};
use jwt::VerifyWithKey;
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{context::Context, jwt_manager::JwtError};

#[derive(Debug, Deserialize, Serialize)]
pub struct XsyncToken {
    exp: i32,
    iat: i32,
    organization_id: i32,
    user_id: i32,
}

pub async fn login(headers: HeaderMap, State(state): State<Arc<Context>>) -> AxumResponse {
    let auth = headers.get("authorization");
    if auth.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            "You should include Authorization header with Bearer token",
        )
            .into_response();
    }
    let auth_val = auth.unwrap().to_str().unwrap();
    let jwt = &auth_val[7..];
    let mut jwt_manager = state.jwt_manager.lock().await;
    // let mut jwt_manager = state.jwt_manager;
    let decoded = jwt_manager.decode(jwt);
    if decoded.is_err() {
        match decoded.unwrap_err() {
            JwtError::NoSession(t) => {
                // TODO check acl
                debug!("No session, inserting");
                jwt_manager.insert_session(jwt);
            }
            JwtError::Expired => {
                debug!("Bearer token expired");
                return (StatusCode::UNAUTHORIZED, "Bearer token expired").into_response();
            }
            JwtError::Other(err) => {
                debug!("Invalid Bearer token");
                println!("{:?}", err);
                return (StatusCode::UNAUTHORIZED, "Invalid Bearer token").into_response();
            }
        }
    }
    (StatusCode::OK).into_response()
}
