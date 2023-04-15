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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
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
            warnings: self.warnings,
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
    pub fn create_response(
        body: T,
        status_code: StatusCode,
        warnings: Option<Vec<String>>,
    ) -> Responder<T> {
        let mut warnings_to_add: Vec<String> = Vec::new();
        if let Some(warnings) = warnings {
            warnings_to_add = warnings;
        }
        let success = !matches!(
            status_code,
            StatusCode::BAD_REQUEST
                | StatusCode::UNAUTHORIZED
                | StatusCode::PAYMENT_REQUIRED
                | StatusCode::FORBIDDEN
                | StatusCode::NOT_FOUND
                | StatusCode::METHOD_NOT_ALLOWED
                | StatusCode::NOT_ACCEPTABLE
                | StatusCode::PROXY_AUTHENTICATION_REQUIRED
                | StatusCode::REQUEST_TIMEOUT
                | StatusCode::CONFLICT
                | StatusCode::GONE
                | StatusCode::LENGTH_REQUIRED
                | StatusCode::PRECONDITION_FAILED
                | StatusCode::PAYLOAD_TOO_LARGE
                | StatusCode::URI_TOO_LONG
                | StatusCode::UNSUPPORTED_MEDIA_TYPE
                | StatusCode::RANGE_NOT_SATISFIABLE
                | StatusCode::EXPECTATION_FAILED
                | StatusCode::IM_A_TEAPOT
                | StatusCode::MISDIRECTED_REQUEST
                | StatusCode::UNPROCESSABLE_ENTITY
                | StatusCode::LOCKED
                | StatusCode::FAILED_DEPENDENCY
                | StatusCode::UPGRADE_REQUIRED
                | StatusCode::PRECONDITION_REQUIRED
                | StatusCode::TOO_MANY_REQUESTS
                | StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                | StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                | StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::NOT_IMPLEMENTED
                | StatusCode::BAD_GATEWAY
                | StatusCode::INSUFFICIENT_STORAGE
                | StatusCode::LOOP_DETECTED
                | StatusCode::NOT_EXTENDED
                | StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
        );

        Responder {
            success,
            body,
            status_code,
            warnings: warnings_to_add,
        }
    }
}
