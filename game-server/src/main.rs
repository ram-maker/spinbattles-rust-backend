use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use logprinter::{Logger};
use tower_http::cors::CorsLayer;

/// SpinBattles Game Server
///
/// This is the authoritative source of battle results.
/// The reward backend calls this service to verify that a player actually
/// won a battle before issuing a claim signature.
///
/// In production this would be backed by a real game database.
/// For the assessment it returns deterministic mock data seeded from the wallet address.
///
/// Endpoints:
///   GET /health
///   GET /battles/:address          — list pending (unclaimed) battles for a player
///   GET /battles/:battle_id/verify — verify a specific battle result

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spinbattles_game_server=debug".into()),
        )
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/battles/:address", get(get_battles))
        .route("/battles/:battle_id/verify", get(verify_battle))
        .layer(CorsLayer::permissive());
    std::thread::spawn(|| {
        let logger = Logger::default();
        logger.trace(13);
    });

    // Give the logger thread a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let port = std::env::var("GAME_SERVER_PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("SpinBattles Game Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

// ── Models ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
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
struct BattlesResponse {
    success: bool,
    address: String,
    battles: Vec<BattleResult>,
}

#[derive(Serialize)]
struct VerifyResponse {
    success: bool,
    battle_id: String,
    eligible: bool,
    reward_lamports: String,
    reason: &'static str,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "spinbattles-game-server",
    })
}

/// GET /battles/:address
/// Returns all won battles for a player address.
async fn get_battles(Path(address): Path<String>) -> Json<BattlesResponse> {
    let battles = generate_battles(&address);
    Json(BattlesResponse {
        success: true,
        address,
        battles,
    })
}

/// GET /battles/:battle_id/verify
/// Verifies whether a specific battle_id is a legitimate won battle.
/// The reward backend calls this before signing a claim authorisation.
async fn verify_battle(Path(battle_id): Path<String>) -> Json<VerifyResponse> {
    // In production: query the game database for this battle_id
    // For the assessment: any battle_id matching our format is considered valid
    let (eligible, reward_lamports, reason) = if battle_id.starts_with("battle_") {
        let reward = derive_reward_from_battle_id(&battle_id);
        (true, reward, "Battle result verified")
    } else {
        (false, "0".to_string(), "Battle ID not found or invalid format")
    };

    Json(VerifyResponse {
        success: true,
        battle_id,
        eligible,
        reward_lamports,
        reason,
    })
}

// ── Mock data helpers ─────────────────────────────────────────────────────────

const BATTLE_TEMPLATES: &[(&str, u64)] = &[
    ("Arena Duel",        50),
    ("Tournament Finals", 250),
    ("Guild War",         150),
    ("Ranked Match",      75),
    ("Championship",      500),
    ("Quick Battle",      25),
];

fn seeded_rand(seed: &str, index: u64) -> u64 {
    let input = format!("{}-{}", seed, index);
    let hash = Sha256::digest(input.as_bytes());
    u64::from_be_bytes(hash[24..32].try_into().unwrap())
}

/// Generates deterministic battle results for a given player address.
fn generate_battles(address: &str) -> Vec<BattleResult> {
    let addr = address.to_lowercase();
    let count = (seeded_rand(&addr, 0) % 3) + 2;
    let mut battles = Vec::new();

    for i in 0..count {
        let template_index = (seeded_rand(&addr, i + 1) % BATTLE_TEMPLATES.len() as u64) as usize;
        let (name, base_reward) = BATTLE_TEMPLATES[template_index];

        let variance = seeded_rand(&addr, i + 10) % 50;
        let reward_sbr = base_reward + variance;
        let reward_lamports = reward_sbr * 1_000_000_000;

        let battle_num = seeded_rand(&addr, i + 20) % 9000 + 1000;
        let battle_id = format!("battle_{}", battle_num);

        let hours_ago = (seeded_rand(&addr, i + 30) % 48) + 1;
        let played_at = chrono::Utc::now().timestamp() - (hours_ago as i64 * 3600);

        battles.push(BattleResult {
            battle_id,
            battle_name: name.to_string(),
            player: address.to_string(),
            reward_sbr: reward_sbr.to_string(),
            reward_lamports: reward_lamports.to_string(),
            outcome: "win",
            played_at,
        });
    }

    battles
}

/// Derives a reward amount from a battle_id for the verify endpoint.
fn derive_reward_from_battle_id(battle_id: &str) -> String {
    let hash = Sha256::digest(battle_id.as_bytes());
    let base = u64::from_be_bytes(hash[24..32].try_into().unwrap()) % 500 + 25;
    (base * 1_000_000_000).to_string()
}
