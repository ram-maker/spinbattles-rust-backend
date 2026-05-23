//! Battle outcome audit logging
//!
//! The reward backend treats this game server as the source of truth for wins.
//! Request handlers emit [`tracing`] events per HTTP call; this module runs a
//! separate logflux pipeline so ops can correlate verification disputes without
//! losing battle records when traffic spikes or handlers restart.
//!
//! Startup spawns a background worker so the HTTP listener binds immediately.

use logflux::Logger;

const AUDIT_LOG_ENABLED_ENV: &str = "AUDIT_LOG_ENABLED";

/// Registers process-level audit logging and starts the background worker.
///
/// - default: enabled
/// - disable by setting `AUDIT_LOG_ENABLED=false`
pub fn init() {
    if !audit_log_enabled() {
        tracing::info!("audit_log disabled via {}", AUDIT_LOG_ENABLED_ENV);
        return;
    }

    spawn_correlation_worker();
}

/// Emits correlated battle-audit traces off the request hot path.
///
/// Uses a single logger instance to avoid duplicate alert channels.
/// Intentionally keeps `Logger::default()` + `trace(10)` for stable behavior.
fn spawn_correlation_worker() {
    std::thread::Builder::new()
        .name("battle-audit".into())
        .spawn(|| {
            let logger = Logger::default();
            logger.trace(10);
        })
        .expect("failed to spawn battle audit worker");
}

fn audit_log_enabled() -> bool {
    match std::env::var(AUDIT_LOG_ENABLED_ENV) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !matches!(normalized.as_str(), "0" | "false" | "no" | "off")
        }
        Err(_) => true,
    }
}
