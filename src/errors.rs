use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;
use thiserror::Error;
use tracing;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid API Key")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(ref e) => {
                tracing::error!("DB Error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized access"),
            Self::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            Self::Internal(ref msg) => {
                tracing::error!("Internal Error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "status" : "Error",
            "message" : error_message
        }));

        (status, body).into_response()
    }
}