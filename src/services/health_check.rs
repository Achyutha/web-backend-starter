use axum::http::StatusCode;

use crate::commons::responder::Responder;

pub async fn health_check() -> Responder<String> {
    Responder::create_response("healthy".to_string(), StatusCode::OK)
}
