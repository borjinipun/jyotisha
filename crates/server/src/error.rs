//! HTTP error envelope (NFR-03): `{"error":{"code":"…","message":"…"}}`.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn bad_request(code: &str, message: impl Into<String>) -> Self {
        Self { status: StatusCode::BAD_REQUEST, code: code.into(), message: message.into() }
    }
    pub fn not_found(code: &str, message: impl Into<String>) -> Self {
        Self { status: StatusCode::NOT_FOUND, code: code.into(), message: message.into() }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({ "error": { "code": self.code, "message": self.message } })),
        )
            .into_response()
    }
}
