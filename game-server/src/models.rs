use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

#[derive(Serialize, Clone)]
pub struct BattleResult {
    pub battle_id: String,
    pub battle_name: String,
    pub player: String,
    pub reward_sbr: String,
    pub reward_lamports: String,
    pub outcome: &'static str,
    pub played_at: i64,
}

#[derive(Serialize)]
pub struct BattlesResponse {
    pub success: bool,
    pub address: String,
    pub battles: Vec<BattleResult>,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub success: bool,
    pub battle_id: String,
    pub eligible: bool,
    pub reward_lamports: String,
    pub reason: &'static str,
}
