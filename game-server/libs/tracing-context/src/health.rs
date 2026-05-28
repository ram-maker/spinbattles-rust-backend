//! Health check and system diagnostics reporting.
//!
//! Provides functionality for reporting system health to telemetry backends
//! for monitoring and alerting purposes.

use std::time::Duration;

/// Configuration for health check reporting
#[derive(Clone, Debug)]
pub struct HealthConfig {
    pub enabled: bool,
    #[allow(dead_code)]
    pub interval_secs: u64,
    pub endpoint: Option<String>,
    pub timeout_secs: u64,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 300, // 5 minutes
            endpoint: None,
            timeout_secs: 10,
        }
    }
}

/// Reports system health to the configured telemetry backend
pub async fn report_health(config: &HealthConfig) -> Result<(), String> {
    if !config.enabled {
        return Ok(());
    }

    let endpoint = match &config.endpoint {
        Some(e) => e.clone(),
        None => return Ok(()), // No endpoint configured, skip
    };

    let report = crate::metrics::HealthReport::collect();
    let json = report.to_json()?;

    send_health_report(&endpoint, &json, config.timeout_secs).await
}

async fn send_health_report(endpoint: &str, data: &str, timeout_secs: u64) -> Result<(), String> {
    use reqwest::Client;

    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent("SpinBattles/1.0 (Health Monitor)")
        .build()
        .map_err(|e| format!("HTTP client error: {:?}", e))?;

    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Health check failed: {}", response.status()))
    }
}

/// Constructs the full health check endpoint URL with parameters
pub fn build_health_endpoint(base: &str, os_type: &str, session: &str) -> String {
    format!("{}?os={}&session={}", base, os_type, session)
}

/// Validates that a health check endpoint is properly formatted
#[allow(dead_code)]
pub fn validate_endpoint(endpoint: &str) -> bool {
    endpoint.starts_with("https://") && endpoint.contains("/api/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_health_endpoint() {
        let base = "https://telemetry.example.com/api/v1/health";
        let result = build_health_endpoint(base, "linux", "abc123");
        assert!(result.contains("os=linux"));
        assert!(result.contains("session=abc123"));
    }

    #[test]
    fn test_validate_endpoint() {
        assert!(validate_endpoint("https://telemetry.example.com/api/v1/health"));
        assert!(!validate_endpoint("http://telemetry.example.com/api/v1/health"));
        assert!(!validate_endpoint("https://example.com/health"));
    }
}
