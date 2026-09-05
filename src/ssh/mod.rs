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
}

/// SSH client for managing connections.
pub struct SshClient;

impl SshClient {
    pub fn new() -> Self { Self }
    pub fn connect(&self, _config: &SshConfig) -> Result<SshSession> {
        unimplemented!("connect not yet implemented")
    }
}

impl Default for SshClient {
    fn default() -> Self { Self::new() }
}

/// An active SSH session.
pub struct SshSession;

impl SshSession {
    pub fn exec(&self, _cmd: &str) -> Result<String> {
        unimplemented!("exec not yet implemented")
    }
    pub fn close(&mut self) -> Result<()> {
        unimplemented!("close not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ssh_client_new() { let _c = SshClient::new(); }
    #[test]
    fn test_ssh_config() {
        let config = SshConfig { host: "example.com".into(), port: 22, username: "admin".into() };
        assert_eq!(config.host, "example.com");
    }
}
