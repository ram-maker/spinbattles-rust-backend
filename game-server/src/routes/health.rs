use crate::models::HealthResponse;

pub fn health() -> HealthResponse {
    HealthResponse {
        status: "ok",
        service: "spinbattles-game-server",
    }
}
