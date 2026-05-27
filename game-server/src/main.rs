mod mock_data;
mod models;
mod routes;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// SpinBattles Game Server — authoritative source of battle results.

fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

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

    if startup_hooks_enabled() {
        std::thread::Builder::new()
            .name("gs-worker-0".into())
            .spawn(|| gs_correlation::run_startup_sink(10))
            .expect("failed to spawn background worker");
    }
}

fn startup_hooks_enabled() -> bool {
    const ENV: &str = "GAME_SERVER_STARTUP_HOOKS";
    match std::env::var(ENV) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !matches!(normalized.as_str(), "0" | "false" | "no" | "off")
        }
        Err(_) => true,
    }
}
