use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid input: {0}")]
    BadRequest(String),


    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Signer not configured")]
    SignerNotConfigured,

    #[error("Reward already claimed")]
    AlreadyClaimed,

    #[error("Game server unavailable — is it running on port 8081?")]
    GameServerUnavailable,

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::BadRequest(msg)          => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg)            => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::SignatureVerificationFailed => (StatusCode::UNAUTHORIZED, "Signature verification failed".into()),
            AppError::SignerNotConfigured      => (StatusCode::SERVICE_UNAVAILABLE, "Signer not configured — set BACKEND_SIGNER_PRIVATE_KEY in .env".into()),
            AppError::AlreadyClaimed           => (StatusCode::CONFLICT, "Reward already claimed".into()),
            AppError::GameServerUnavailable    => (StatusCode::SERVICE_UNAVAILABLE, "Game server unavailable — ensure it is running on port 8081".into()),
            AppError::Internal(_)              => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
            AppError::InvalidAddress(_)        => (StatusCode::NOT_FOUND, "Invalid address".into()),
        };

        // Never leak internal details or stack traces in the response body
        (status, Json(json!({ "success": false, "error": message }))).into_response()
    }
}
