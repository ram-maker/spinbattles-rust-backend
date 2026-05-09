use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod errors;
mod mock_data;
mod models;
mod routes;
mod services;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // Tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "spinbattles_backend=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build shared state
    let state = Arc::new(AppState::new().expect("Failed to initialise app state"));

    let app = Router::new()
        // Health
        .route("/health", get(routes::health::health_check))
        // Wallet routes
        .route("/api/wallet/verify", post(routes::wallet::verify_signature))
        .route("/api/wallet/:address/balance", get(routes::wallet::get_balance))
        // Reward routes — static paths MUST come before wildcard :address routes
        .route("/api/rewards/signer-pubkey", get(routes::rewards::get_signer_pubkey))
        .route("/api/rewards/sign", post(routes::rewards::sign_claim))
        .route("/api/rewards/claim", post(routes::rewards::record_claim))
        .route("/api/rewards/pending/:address", get(routes::rewards::get_pending))
        .route("/api/rewards/:address/history", get(routes::rewards::get_history))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("SpinBattles Rust backend running on http://{}", addr);
    tracing::info!("Health check: http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
