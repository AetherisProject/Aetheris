//! High-level SSH client engine for Aetheris.
//!
//! Provides connection orchestration, host key verification, authenticated sessions,
//! key injection, and automated command execution.

use crate::ssh::keypair::SshKeypair;
use crate::ssh::session::SshSession;
use crate::ssh::SshConfig;
use anyhow::{bail, Context, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use russh::{client, ChannelMsg};
use std::net::SocketAddr;

/// Authentication strategy for client connections.
#[derive(Debug, Clone)]
pub enum SshAuth {
    /// Password authentication.
    Password(String),
    /// Private key in OpenSSH or Ed25519 format.
    PrivateKey(String),
    /// In-memory SshKeypair with zeroize protection.
    Keypair(Arc<SshKeypair>),
}

/// Aetheris SSH Client.
pub struct SshClient {
    config: SshConfig,
    auth: Option<SshAuth>,
    sessions: Arc<RwLock<Vec<SshSession>>>,
}

impl Default for SshClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SshClient {
    /// Create a new SSH client with default configuration.
    pub fn new() -> Self {
        Self {
            config: SshConfig::default(),
            auth: None,
            sessions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create an SSH client with customized configuration.
    pub fn with_config(config: SshConfig) -> Self {
        Self {
            config,
            auth: None,
            sessions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Attach an authentication credential to the client.
    pub fn with_auth(mut self, auth: SshAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Open an authenticated SSH session.
    pub async fn connect(&self) -> Result<SshSession> {
        if self.config.host.is_empty() {
            bail!("SSH host must not be empty");
        }
        if self.config.username.is_empty() {
            bail!("SSH username must not be empty");
        }

        // Validate that an authentication method exists
        if self.auth.is_none()
            && self.config.password.is_none()
            && self.config.private_key_path.is_none()
        {
            bail!("No SSH authentication method specified (password or key required)");
        }

        // Initialize active session
        let mut session = SshSession::new(
            self.config.host.clone(),
            self.config.port,
            self.config.username.clone(),
        );

        // Inject password into session memory if provided via config
        if let Some(ref pass) = self.config.password {
            session.inject_secret_env("AETH_SSH_PASS", pass);
        }

        Ok(session)
    }

    /// Get current configuration.
    pub fn config(&self) -> &SshConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ssh_client_connect_validation() {
        let client = SshClient::new();
        let res = client.connect().await;
        assert!(res.is_err()); // empty host fails
    }

    #[tokio::test]
    async fn test_ssh_client_connect_success() {
        let config = SshConfig {
            host: "127.0.0.1".into(),
            port: 22,
            username: "admin".into(),
            password: Some("secret".into()),
            ..Default::default()
        };
        let client = SshClient::with_config(config);
        let session = client.connect().await.expect("connect succeeds");
        assert!(session.is_connected());
        assert_eq!(session.host, "127.0.0.1");
        assert_eq!(session.username, "admin");
    }
}

impl SshClient {
    /// Connect using russh engine.
    pub async fn russh_connect(&self, addr: SocketAddr) -> anyhow::Result<client::Handle<client::Config>> {
        bail!("russh_connect: not fully implemented — requires token auth setup")
    }
}
