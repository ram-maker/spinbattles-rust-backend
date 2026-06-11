use crate::{errors::AppError, models::PendingBattle};
use serde::Deserialize;
use solana_sdk::client;

/// Game Client
///
/// The backend is NOT the source of truth for battle results.
/// It delegates to the SpinBattles Game Server, which owns the game database.
///
/// The backend calls the game server to:
///   1. Fetch pending (won, unclaimed) battles for a player
///   2. Verify a specific battle result before signing a claim
///
/// If the game server is unreachable, all claim operations fail.
/// This is intentional — the backend must not sign claims it cannot verify.

/// Response shape from GET /battles/:address
#[derive(Deserialize)]
struct BattlesResponse {
    battles: Vec<GameBattle>,
}

#[derive(Deserialize)]
struct GameBattle {
    battle_id: String,
    battle_name: String,
    reward_sbr: String,
    reward_lamports: String,
    played_at: i64,
}

/// Response shape from GET /battles/:battle_id/verify
#[derive(Deserialize)]
pub struct VerifyResponse {
    pub eligible: bool,
    #[allow(dead_code)]
    pub reward_lamports: String,
}

fn game_server_url() -> String {
    std::env::var("GAME_SERVER_URL").unwrap_or_else(|_| "http://localhost:8081".to_string())
}

/// Fetch all pending won battles for a player from the game server.
///
/// Returns an error if the game server is unreachable — the backend cannot
/// authorise claims without verified battle data.
pub async fn get_pending_battles(address: &str) -> Result<Vec<PendingBattle>, AppError> {
    let url = format!("{}/battles/{}", game_server_url(), address);

    let response = reqwest::get(&url).await.map_err(|e| {
        tracing::error!("Game server unreachable at {}: {}", url, e);
        AppError::GameServerUnavailable
    })?;

    if !response.status().is_success() {
        tracing::error!("Game server returned {} for {}", response.status(), url);
        return Err(AppError::GameServerUnavailable);
    }

    let body: BattlesResponse = response.json().await.map_err(|e| {
        tracing::error!("Failed to parse game server response: {}", e);
        AppError::Internal(anyhow::anyhow!("Invalid game server response"))
    })?;

    let battles = body
        .battles
        .into_iter()
        .map(|b| PendingBattle {
            battle_id: b.battle_id,
            battle_name: b.battle_name,
            reward: b.reward_sbr,
            reward_lamports: b.reward_lamports,
            timestamp: b.played_at,
            status: "unclaimed",
        })
        .collect();

    Ok(battles)
}

/// Verify a specific battle result with the game server.
///
/// Returns `(eligible, reward_lamports)` or an error if the game server is down.
pub async fn verify_battle(battle_id: &str) -> Result<VerifyResponse, AppError> {
    let url = format!("{}/battles/{}/verify", game_server_url(), battle_id);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| AppError::Internal(e.into()))?;

    let response = client.get(&url).send().await.map_err(|e| {
        tracing::error!("Game server unreachable at {}: {}", url, e);
        AppError::GameServerUnavailable
    })?;

    let body: VerifyResponse = response.json().await.map_err(|e| {
        tracing::error!("Failed to parse game server verify response: {}", e);
        AppError::Internal(anyhow::anyhow!("Invalid game server response"))
    })?;

    Ok(body)
}
