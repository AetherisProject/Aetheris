//! SSH client module for Aetheris.
pub mod client;
pub mod session;
pub mod keypair;
pub mod forward;
pub mod sftp;
pub mod health;

use anyhow::Result;

/// SSH connection configuration.
#[derive(Debug, Clone)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub private_key_path: Option<String>,
    pub connect_timeout: u64, // seconds
    pub keep_alive: u64, // seconds
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
    PrivateKey(String), // Path to private key file
}

/// SSH client for managing connections.
#[derive(Default)]
pub struct SshClient {
    config: SshConfig,
    authentication: Option<Authentication>,
}

impl SshClient {
    pub fn new() -> Self {
        Self {
            config: SshConfig::default(),
            authentication: None,
        }
    }

    pub fn with_config(config: SshConfig) -> Self {
        Self {
            config,
            authentication: None,
        }
    }
}

/// An active SSH session.
pub struct SshSession;

impl SshSession {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exec(&self, _cmd: &str) -> Result<String> {
        unimplemented!("SSH exec functionality requires russh async runtime - TODO")
    }
    
    pub fn close(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Check if the session is still active.
    pub fn is_connected(&self) -> bool {
        false // TODO: implement proper connection status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ssh_client_new() { let _c = SshClient::new(); }
    #[test]
    fn test_ssh_config() {
        let config = SshConfig { host: "example.com".into(), port: 22, username: "admin".into(), ..Default::default() };
        assert_eq!(config.host, "example.com");
    }
    #[test]
    fn test_ssh_session() {
        let session = SshSession::new();
        assert!(!session.is_connected());
    }
}