//! SSH Health and connectivity monitoring for Aetheris.
//!
//! Provides latency tracking, TCP reachability probing, connection stability,
//! and automated heartbeat diagnostics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Target host connectivity health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unreachable,
    Unknown,
}

/// Diagnostic health report for a remote SSH endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshHealthReport {
    pub host: String,
    pub port: u16,
    pub status: HealthStatus,
    pub round_trip_ms: Option<u64>,
    pub checked_at: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// Probe the TCP connectivity and latency of an SSH endpoint.
pub async fn probe_ssh_endpoint(host: &str, port: u16, timeout_duration: Duration) -> SshHealthReport {
    let addr = format!("{}:{}", host, port);
    let start = std::time::Instant::now();
    let checked_at = Utc::now();

    match timeout(timeout_duration, TcpStream::connect(&addr)).await {
        Ok(Ok(_stream)) => {
            let latency = start.elapsed().as_millis() as u64;
            let status = if latency > 350 {
                HealthStatus::Degraded
            } else {
                HealthStatus::Healthy
            };

            SshHealthReport {
                host: host.to_string(),
                port,
                status,
                round_trip_ms: Some(latency),
                checked_at,
                error_message: None,
            }
        }
        Ok(Err(e)) => SshHealthReport {
            host: host.to_string(),
            port,
            status: HealthStatus::Unreachable,
            round_trip_ms: None,
            checked_at,
            error_message: Some(e.to_string()),
        },
        Err(_) => SshHealthReport {
            host: host.to_string(),
            port,
            status: HealthStatus::Unreachable,
            round_trip_ms: None,
            checked_at,
            error_message: Some("Connection timed out".to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_probe_ssh_endpoint_timeout() {
        // Probe an unreachable RFC 5737 TEST-NET IP with very short timeout
        let report = probe_ssh_endpoint("192.0.2.1", 22, Duration::from_millis(50)).await;
        assert_eq!(report.status, HealthStatus::Unreachable);
        assert!(report.error_message.is_some());
    }
}
