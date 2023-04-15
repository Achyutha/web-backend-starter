use axum::http::StatusCode;

use crate::commons::responder::Responder;

pub async fn health_check() -> Responder<()> {
    Responder::create_response((), StatusCode::OK, None)
}
