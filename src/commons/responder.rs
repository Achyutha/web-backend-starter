use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Responder<T: Serialize> {
    pub success: bool,
    pub body: T,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
}

impl<T> IntoResponse for Responder<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let response_body = Responder {
            body: self.body,
            success: self.success,
            status_code: self.status_code,
        };

        let data = serde_json::to_string(&response_body).unwrap();

        (
            self.status_code,
            [(header::CONTENT_TYPE, "application/json")],
            data,
        )
            .into_response()
    }
}

impl<T> Responder<T>
where
    T: Serialize,
{
    pub fn create_response(body: T, status_code: StatusCode) -> Responder<T> {
        let success = match status_code {
            StatusCode::BAD_REQUEST => false,
            StatusCode::UNAUTHORIZED => false,
            StatusCode::PAYMENT_REQUIRED => false,
            StatusCode::FORBIDDEN => false,
            StatusCode::NOT_FOUND => false,
            StatusCode::METHOD_NOT_ALLOWED => false,
            StatusCode::NOT_ACCEPTABLE => false,
            StatusCode::PROXY_AUTHENTICATION_REQUIRED => false,
            StatusCode::REQUEST_TIMEOUT => false,
            StatusCode::CONFLICT => false,
            StatusCode::GONE => false,
            StatusCode::LENGTH_REQUIRED => false,
            StatusCode::PRECONDITION_FAILED => false,
            StatusCode::PAYLOAD_TOO_LARGE => false,
            StatusCode::URI_TOO_LONG => false,
            StatusCode::UNSUPPORTED_MEDIA_TYPE => false,
            StatusCode::RANGE_NOT_SATISFIABLE => false,
            StatusCode::EXPECTATION_FAILED => false,
            StatusCode::IM_A_TEAPOT => false,
            StatusCode::MISDIRECTED_REQUEST => false,
            StatusCode::UNPROCESSABLE_ENTITY => false,
            StatusCode::LOCKED => false,
            StatusCode::FAILED_DEPENDENCY => false,
            StatusCode::UPGRADE_REQUIRED => false,
            StatusCode::PRECONDITION_REQUIRED => false,
            StatusCode::TOO_MANY_REQUESTS => false,
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => false,
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => false,
            StatusCode::INTERNAL_SERVER_ERROR => false,
            StatusCode::NOT_IMPLEMENTED => false,
            StatusCode::BAD_GATEWAY => false,
            StatusCode::INSUFFICIENT_STORAGE => false,
            StatusCode::LOOP_DETECTED => false,
            StatusCode::NOT_EXTENDED => false,
            StatusCode::NETWORK_AUTHENTICATION_REQUIRED => false,
            _ => true,
        };

        Responder {
            success,
            body,
            status_code,
        }
    }
}
