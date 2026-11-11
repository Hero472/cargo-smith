//! Application error type with HTTP status code mapping.

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

use crate::common::response::ApiResponse;

/// Application-level error type with HTTP status mapping.
///
/// Each variant carries a human-readable message and maps to an HTTP
/// status code via [`ResponseError`].
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Unauthorized(&'static str),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
    #[error("database error: {0}")]
    Database(#[from] mongodb::error::Error),
}

impl ResponseError for AppError {

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Internal(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        
        let status = self.status_code();

        let body = ApiResponse::<()>::with_status(status, self.to_string(), None);
        
        HttpResponse::build(status).json(body)
    }
}