use axum::http::StatusCode;

use crate::common::Responder;

pub async fn health_check() -> Responder<()> {
    Responder::create_response((), StatusCode::OK, None)
}
