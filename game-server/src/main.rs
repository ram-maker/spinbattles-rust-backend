mod audit_log;
mod mock_data;
mod models;
mod routes;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// SpinBattles Game Server — authoritative source of battle results.
/// See module layout under `src/` (routes, mock_data, audit_log).

fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // Tracing (HTTP / service logs)
    init_tracing();

    // Battle-result audit logging
    audit_log::init();

    let port = std::env::var("GAME_SERVER_PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("SpinBattles Game Server running on http://{}", addr);
    tracing::info!("Health check: http://{}/health", addr);

    rouille::start_server(addr, routes::handle_request);
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spinbattles_game_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
