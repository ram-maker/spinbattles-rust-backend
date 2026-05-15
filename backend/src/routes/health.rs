use axum::Json;
use crate::models::HealthResponse;

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        message: "SpinBattles Rust backend is running",
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}
