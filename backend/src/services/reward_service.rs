use crate::{
    errors::AppError,
    models::{ClaimRecord, PendingBattle},
    services::game_client,
    state::AppState,
};
use solana_sdk::signature::Signature;
use std::sync::Arc;
use std::str::FromStr;

/// Reward Service
///
/// Handles reward claim authorisation and history tracking.
/// `get_pending_rewards()` fetches live battle data from the game server.
/// The sign flow is required by the on-chain program.

/// Generate a backend-signed authorisation for a reward claim.
///
/// This is the critical integration point between the backend and the Solana program.
/// The program's `claim_reward` instruction requires a valid signature from this service —
/// it will fail with `InvalidBackendSignature` unless you pass a signature obtained here.
///
/// Flow:
///   1. Verify the player's wallet signature (proves they own the address)
///   2. Look up the battle and verify the player is eligible
///   3. Check the reward hasn't already been claimed
///   4. Sign and return the authorisation
pub async fn authorise_claim_signature(
    state: &Arc<AppState>,
    player_address: &str,
    wallet_signature: &str,
    wallet_message: &str,
    battle_id: &str,
) -> Result<(String, String, String, i64), AppError> {
    // Step 1: Verify wallet ownership
    let verified = crate::services::wallet_service::verify_signature(
        player_address,
        wallet_signature,
        wallet_message,
    )?;
    if !verified {
        return Err(AppError::SignatureVerificationFailed);
    }

    // Step 2: Look up the battle via the game server and verify eligibility
    // The game server is the authoritative source — if it's down, we cannot sign
    let pending = game_client::get_pending_battles(player_address).await?;
    let battle = pending
        .iter()
        .find(|b| b.battle_id == battle_id)
        .ok_or_else(|| AppError::NotFound(format!(
            "Battle {} not found or not eligible for this address", battle_id
        )))?;

    // Confirm the battle result is valid with the game server
    let verification = game_client::verify_battle(battle_id).await?;
    if !verification.eligible {
        return Err(AppError::NotFound(format!(
            "Battle {} is not eligible for reward claim", battle_id
        )));
    }

    // Step 3: Check for duplicate claim
    let claim_key = format!("{}-{}", player_address.to_lowercase(), battle_id);
    {
        let history = state.claim_history.lock().unwrap();
        if history.contains_key(&claim_key) {
            return Err(AppError::AlreadyClaimed);
        }
    }

    // Step 4: Sign the authorisation
    let keypair = state.signer.as_ref().ok_or(AppError::SignerNotConfigured)?;
    let amount_lamports: u64 = battle.reward_lamports.parse().map_err(|_| {
        AppError::BadRequest("Invalid reward_lamports in battle data".into())
    })?;

    let (signature, expires_at) = crate::services::signer_service::sign_claim_authorisation(
        keypair,
        player_address,
        battle_id,
        amount_lamports,
    )
    .map_err(AppError::Internal)?;

    Ok((signature, battle.reward.clone(), battle.reward_lamports.clone(), expires_at))
}

/// Record a completed claim (called after the on-chain tx is confirmed).
pub async fn record_claim(
    state: &Arc<AppState>,
    address: &str,
    battle_id: &str,
    amount: &str,
    tx_signature: &str,
) -> Result<String, AppError> {
    let amount_lamports = amount
        .parse::<u64>()
        .map_err(|_| AppError::BadRequest("amount must be a valid u64 string".into()))?;
    if amount_lamports == 0 {
        return Err(AppError::BadRequest("amount must be greater than zero".into()));
    }
    if amount_lamports > 10_000_000_000_000 {
        return Err(AppError::BadRequest("amount exceeds maximum allowed value".into()));
    }

    let parsed_sig = Signature::from_str(tx_signature)
        .map_err(|_| AppError::BadRequest("tx_signature must be valid base58".into()))?;
    let sig_len = bs58::decode(tx_signature)
        .into_vec()
        .map_err(|_| AppError::BadRequest("tx_signature must be valid base58".into()))?
        .len();
    if sig_len != 64 {
        return Err(AppError::BadRequest("tx_signature must decode to 64 bytes".into()));
    }

    {
        let history = state.claim_history.lock().unwrap();
        if history
            .values()
            .any(|record| record.tx_signature == parsed_sig.to_string())
        {
            return Err(AppError::BadRequest("tx_signature already recorded".into()));
        }
    }

    tracing::info!(
        "Recording claim: address={}, battle={}, amount={}",
        address, battle_id, amount
    );

    let claim_key = format!("{}-{}", address.to_lowercase(), battle_id);

    let record = ClaimRecord {
        address: address.to_string(),
        battle_id: battle_id.to_string(),
        amount: amount.to_string(),
        tx_signature: tx_signature.to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        status: "confirmed",
    };

    state.claim_history.lock().unwrap().insert(claim_key, record);

    Ok(tx_signature.to_string())
}

/// Get claim history for an address.
pub fn get_reward_history(state: &Arc<AppState>, address: &str) -> Vec<ClaimRecord> {
    let prefix = format!("{}-", address.to_lowercase());
    state
        .claim_history
        .lock()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.starts_with(&prefix))
        .map(|(_, v)| v.clone())
        .collect()
}

/// Get pending (unclaimed) rewards for an address.
/// Fetches from the game server and filters out already-claimed battles.
pub async fn get_pending_rewards(state: &Arc<AppState>, address: &str) -> Result<Vec<PendingBattle>, AppError> {
    let all = game_client::get_pending_battles(address).await?;
    let history = state.claim_history.lock().unwrap();
    Ok(all.into_iter()
        .filter(|b| {
            let key = format!("{}-{}", address.to_lowercase(), b.battle_id);
            !history.contains_key(&key)
        })
        .collect())
}
