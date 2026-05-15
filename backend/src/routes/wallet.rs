use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::{
    errors::AppError,
    models::{BalanceResponse, VerifySignatureRequest, VerifySignatureResponse},
    services::wallet_service,
    state::AppState,
};

/// POST /api/wallet/verify
///
/// Verify wallet ownership through an Ed25519 signature.
/// Body: { address, signature, message }
pub async fn verify_signature(
    State(_state): State<Arc<AppState>>,
    Json(body): Json<VerifySignatureRequest>,
) -> Result<Json<VerifySignatureResponse>, AppError> {
    if !wallet_service::is_valid_pubkey(&body.address) {
        return Err(AppError::BadRequest("Invalid Solana public key".into()));
    }
    if body.signature.is_empty() {
        return Err(AppError::BadRequest("signature is required".into()));
    }
    if body.message.is_empty() {
        return Err(AppError::BadRequest("message is required".into()));
    }

    let verified = wallet_service::verify_signature(&body.address, &body.signature, &body.message)?;

    Ok(Json(VerifySignatureResponse {
        success: true,
        verified,
        address: body.address,
    }))
}

/// GET /api/wallet/:address/balance
///
/// Get the SBR token balance for a wallet address.
pub async fn get_balance(
    State(_state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<BalanceResponse>, AppError> {
    if !wallet_service::is_valid_pubkey(&address) {
        return Err(AppError::BadRequest("Invalid Solana public key".into()));
    }

    let (balance, balance_ui) = wallet_service::get_token_balance(&address).await?;

    Ok(Json(BalanceResponse {
        success: true,
        address,
        balance,
        balance_ui,
    }))
}
