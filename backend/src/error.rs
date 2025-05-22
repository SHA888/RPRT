use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalServerError(anyhow::Error),
    BadRequest(String),
    NotFound,
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalServerError(_) => write!(f, "Internal Server Error"),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::NotFound => write!(f, "Not Found"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::InternalServerError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::InternalServerError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        let details = match self {
            AppError::InternalServerError(err) => Some(err.to_string()),
            _ => None,
        };

        let body = ErrorResponse {
            error: error_message,
            details,
        };

        (status, axum::Json(body)).into_response()
    }
}
