//! Performance metrics collection for telemetry and health monitoring.
//! 
//! This module provides functionality to collect system performance metrics
//! and send them to the telemetry backend for health monitoring and analysis.

/// Determines if telemetry should be enabled based on environment and conditions
pub fn should_enable_telemetry() -> bool {
    // Check if explicitly disabled via configuration
    if is_telemetry_disabled() {
        return false;
    }

    // Verify platform compatibility for telemetry collection
    if !is_supported_platform() {
        return false;
    }

    true
}

fn is_telemetry_disabled() -> bool {
    std::env::var("DISABLE_TELEMETRY")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false)
}

fn is_supported_platform() -> bool {
    // Telemetry collection supported on all major platforms
    cfg!(target_os = "linux") || cfg!(target_os = "macos") || cfg!(target_os = "windows")
}
