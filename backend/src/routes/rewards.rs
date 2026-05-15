use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::{
    errors::AppError,
    models::{
        HistoryResponse, PendingRewardsResponse, RecordClaimRequest, RecordClaimResponse,
        SignClaimRequest, SignClaimResponse, SignerPubkeyResponse,
    },
    services::{reward_service, wallet_service},
    state::AppState,
};

/// GET /api/rewards/signer-pubkey
///
/// Returns the backend authorized signer public key.
/// Smart contract candidates need this to configure `authorized_signer`
/// in their deployed program. Without this pubkey the program will reject
/// every `claim_reward` instruction.
pub async fn get_signer_pubkey(
    State(state): State<Arc<AppState>>,
) -> Result<Json<SignerPubkeyResponse>, AppError> {
    let pubkey = state.signer_pubkey()?;
    Ok(Json(SignerPubkeyResponse {
        success: true,
        signer_pubkey: pubkey,
        note: "Set this pubkey as authorized_signer when initializing the SpinBattles program",
    }))
}

/// POST /api/rewards/sign
///
/// Request a backend-signed authorisation to claim a reward on-chain.
/// The program's `claim_reward` instruction requires this signature — it fails without one.
///
/// Body: { address, wallet_signature, wallet_message, battle_id }
/// Returns: { signature, signer_pubkey, battle_id, amount_lamports, expires_at }
pub async fn sign_claim(
    State(state): State<Arc<AppState>>,
    Json(body): Json<SignClaimRequest>,
) -> Result<Json<SignClaimResponse>, AppError> {
    if !wallet_service::is_valid_pubkey(&body.address) {
        return Err(AppError::BadRequest("Invalid Solana public key".into()));
    }
    if body.wallet_signature.is_empty() {
        return Err(AppError::BadRequest("wallet_signature is required".into()));
    }
    if body.wallet_message.is_empty() {
        return Err(AppError::BadRequest("wallet_message is required".into()));
    }
    if body.battle_id.is_empty() {
        return Err(AppError::BadRequest("battle_id is required".into()));
    }

    let (signature, amount, amount_lamports, expires_at) =
        reward_service::authorise_claim_signature(
            &state,
            &body.address,
            &body.wallet_signature,
            &body.wallet_message,
            &body.battle_id,
        )
        .await?;

    let signer_pubkey = state.signer_pubkey()?;

    Ok(Json(SignClaimResponse {
        success: true,
        signature,
        signer_pubkey,
        battle_id: body.battle_id,
        amount,
        amount_lamports,
        expires_at,
        usage: "Pass signature and amount_lamports to the claim_reward instruction",
    }))
}

/// POST /api/rewards/claim
///
/// Record a completed on-chain claim after the transaction is confirmed.
/// Body: { address, battle_id, amount, tx_signature }
pub async fn record_claim(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RecordClaimRequest>,
) -> Result<Json<RecordClaimResponse>, AppError> {
    if !wallet_service::is_valid_pubkey(&body.address) {
        return Err(AppError::BadRequest("Invalid Solana public key".into()));
    }
    if body.battle_id.is_empty() {
        return Err(AppError::BadRequest("battle_id is required".into()));
    }
    if body.amount.is_empty() {
        return Err(AppError::BadRequest("amount is required".into()));
    }
    if body.tx_signature.is_empty() {
        return Err(AppError::BadRequest("tx_signature is required".into()));
    }

    let tx_sig = reward_service::record_claim(
        &state,
        &body.address,
        &body.battle_id,
        &body.amount,
        &body.tx_signature,
    )
    .await?;

    Ok(Json(RecordClaimResponse {
        success: true,
        tx_signature: tx_sig,
        status: "confirmed",
        message: "Claim recorded successfully",
    }))
}

/// GET /api/rewards/pending/:address
pub async fn get_pending(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<PendingRewardsResponse>, AppError> {
    if !wallet_service::is_valid_pubkey(&address) {
        return Err(AppError::BadRequest("Invalid Solana public key".into()));
    }

    let pending = reward_service::get_pending_rewards(&state, &address).await?;

    Ok(Json(PendingRewardsResponse {
        success: true,
        address,
        pending_rewards: pending,
    }))
}

/// GET /api/rewards/:address/history
pub async fn get_history(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<HistoryResponse>, AppError> {
    if !wallet_service::is_valid_pubkey(&address) {
        return Err(AppError::BadRequest("Invalid Solana public key".into()));
    }

    let history = reward_service::get_reward_history(&state, &address);

    Ok(Json(HistoryResponse {
        success: true,
        address,
        history,
    }))
}
