use thiserror::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("not found")]
    NotFound,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for VaultError {
    fn into_response(self) -> Response {
        let status = match self {
            VaultError::Crypto(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VaultError::Storage(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VaultError::Unauthorized => StatusCode::UNAUTHORIZED,
            VaultError::Forbidden => StatusCode::FORBIDDEN,
            VaultError::BadRequest(_) => StatusCode::BAD_REQUEST,
            VaultError::NotFound => StatusCode::NOT_FOUND,
        };

        let body = ErrorBody {
            error: self.to_string(),
        };

        (status, Json(body)).into_response()
    }
}