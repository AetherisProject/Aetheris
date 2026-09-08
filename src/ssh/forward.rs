//! SSH Port Forwarding module for Aetheris.
//!
//! Supports Local, Remote, and Dynamic (SOCKS5) port tunneling configurations.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Port forwarding tunnel direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForwardType {
    /// -L: Local port forwarded to remote host:port
    Local,
    /// -R: Remote port forwarded to local host:port
    Remote,
    /// -D: Dynamic SOCKS5 proxy port
    Dynamic,
}

/// Configuration for an SSH port forward.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForward {
    pub forward_type: ForwardType,
    pub bind_host: String,
    pub bind_port: u16,
    pub target_host: Option<String>,
    pub target_port: Option<u16>,
    pub is_active: bool,
}

impl PortForward {
    /// Create a new local port forwarding rule (e.g. 127.0.0.1:8080 -> db:5432).
    pub fn local(bind_port: u16, target_host: impl Into<String>, target_port: u16) -> Self {
        Self {
            forward_type: ForwardType::Local,
            bind_host: "127.0.0.1".into(),
            bind_port,
            target_host: Some(target_host.into()),
            target_port: Some(target_port),
            is_active: false,
        }
    }

    /// Create a dynamic SOCKS5 proxy rule.
    pub fn dynamic(bind_port: u16) -> Self {
        Self {
            forward_type: ForwardType::Dynamic,
            bind_host: "127.0.0.1".into(),
            bind_port,
            target_host: None,
            target_port: None,
            is_active: false,
        }
    }

    /// Mark the tunnel as active.
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate the tunnel.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_forward_creation() {
        let mut fwd = PortForward::local(8080, "db.internal", 5432);
        assert_eq!(fwd.forward_type, ForwardType::Local);
        assert_eq!(fwd.bind_port, 8080);
        assert_eq!(fwd.target_host.as_deref(), Some("db.internal"));
        assert_eq!(fwd.target_port, Some(5432));
        assert!(!fwd.is_active);

        fwd.activate();
        assert!(fwd.is_active);
    }

    #[test]
    fn test_dynamic_socks5_forward() {
        let fwd = PortForward::dynamic(1080);
        assert_eq!(fwd.forward_type, ForwardType::Dynamic);
        assert_eq!(fwd.bind_port, 1080);
        assert!(fwd.target_host.is_none());
    }
}
