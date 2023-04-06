use axum::{
    extract::State,
    http::{StatusCode, Uri},
};

use crate::globals::AppState;

pub async fn not_found(State(_app_state): State<AppState>, uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("path {uri} not found!"))
}
