//! SSH client subsystem for Aetheris: The Secrets Operating System.
//!
//! Provides a unified, ultra-low-latency SSH terminal engine with
//! hardware-grade security, client-side zero-knowledge keypair management,
//! transparent process-memory secret injection, port tunneling, SFTP,
//! and proactive connection health telemetry.

pub mod client;
pub mod forward;
pub mod health;
pub mod keypair;
pub mod session;
pub mod sftp;

pub use client::{SshAuth, SshClient};
pub use forward::{ForwardType, PortForward};
pub use health::{probe_ssh_endpoint, HealthStatus, SshHealthReport};
pub use keypair::{KeyType, SshKeypair};
pub use session::{SessionMetrics, SessionState, SshSession};

/// SSH connection and client configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SshConfig {
    /// Remote target hostname or IP address.
    pub host: String,
    /// Remote SSH port (default 22).
    pub port: u16,
    /// Remote SSH username.
    pub username: String,
    /// Optional password authentication secret.
    pub password: Option<String>,
    /// Optional path to private key on disk.
    pub private_key_path: Option<String>,
    /// TCP connection timeout in seconds.
    pub connect_timeout: u64,
    /// Heartbeat keepalive interval in seconds.
    pub keep_alive: u64,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: String::new(),
            password: None,
            private_key_path: None,
            connect_timeout: 30,
            keep_alive: 60,
        }
    }
}

/// Authentication method for SSH connections.
#[derive(Debug, Clone)]
pub enum Authentication {
    Password(String),
    PrivateKey(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_client_new() {
        let _c = SshClient::new();
    }

    #[test]
    fn test_ssh_config() {
        let config = SshConfig {
            host: "example.com".into(),
            port: 22,
            username: "admin".into(),
            ..Default::default()
        };
        assert_eq!(config.host, "example.com");
    }

    #[test]
    fn test_ssh_session() {
        let session = SshSession::new("test".into(), 22, "admin".into());
        assert!(session.is_connected());
    }
}
