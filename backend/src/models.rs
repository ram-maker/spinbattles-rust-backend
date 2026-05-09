use serde::{Deserialize, Serialize};

// ── Request bodies ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VerifySignatureRequest {
    pub address: String,
    pub signature: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SignClaimRequest {
    pub address: String,
    /// Ed25519 signature proving the player owns the wallet
    pub wallet_signature: String,
    /// The message that was signed (e.g. "Verify wallet ownership")
    pub wallet_message: String,
    pub battle_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordClaimRequest {
    pub address: String,
    pub battle_id: String,
    pub amount: String,
    pub tx_signature: String,
}

// ── Response bodies ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub message: &'static str,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct VerifySignatureResponse {
    pub success: bool,
    pub verified: bool,
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub success: bool,
    pub address: String,
    /// Token balance in lamports (smallest unit), as a string to avoid precision loss
    pub balance: String,
    /// Human-readable balance (e.g. "1234.56 SBR")
    pub balance_ui: String,
}

#[derive(Debug, Serialize)]
pub struct SignerPubkeyResponse {
    pub success: bool,
    pub signer_pubkey: String,
    pub note: &'static str,
}

#[derive(Debug, Serialize)]
pub struct SignClaimResponse {
    pub success: bool,
    pub signature: String,
    pub signer_pubkey: String,
    pub battle_id: String,
    pub amount: String,
    pub amount_lamports: String,
    pub expires_at: i64,
    pub usage: &'static str,
}

#[derive(Debug, Serialize)]
pub struct RecordClaimResponse {
    pub success: bool,
    pub tx_signature: String,
    pub status: &'static str,
    pub message: &'static str,
}

#[derive(Debug, Serialize, Clone)]
pub struct PendingBattle {
    pub battle_id: String,
    pub battle_name: String,
    pub reward: String,
    pub reward_lamports: String,
    pub timestamp: i64,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct PendingRewardsResponse {
    pub success: bool,
    pub address: String,
    pub pending_rewards: Vec<PendingBattle>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClaimRecord {
    pub address: String,
    pub battle_id: String,
    pub amount: String,
    pub tx_signature: String,
    pub timestamp: i64,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub success: bool,
    pub address: String,
    pub history: Vec<ClaimRecord>,
}
