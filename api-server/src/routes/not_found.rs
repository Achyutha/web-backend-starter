use axum::{
    extract::State,
    http::{StatusCode, Uri},
};

use crate::common::Responder;
use utils::AppState;

pub async fn not_found(State(_app_state): State<AppState>, uri: Uri) -> Responder<String> {
    Responder::create_response(
        format!("path {uri} not found!"),
        StatusCode::NOT_FOUND,
        None,
    )
}
