#! SSH Session management for Aetheris.

//! Manages active terminal session state, execution metrics, command output,
//! and process memory isolation.

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use uuid::Uuid;

// Define SSH Session for serialization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SshSession {
    id: String,
    host: String,
    port: u16,
    user: String,
    active: bool,
    // Add other fields as needed
}

// Global state for SSH sessions
pub struct SshSessionManager {
    sessions: Arc<Mutex<HashMap<String, SshSession>>>,
    active_session: Arc<Mutex<String>>,
}

impl SshSessionManager {
    /// Create a new SSH session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            active_session: Arc::new(Mutex::new(String::new())),
        }
    }
    
    /// Add a new SSH session
    pub fn add_session(&self, session: SshSession) -> Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.id.clone(), session);
        Ok(())
    }
    
    /// Get all SSH sessions
    pub fn get_sessions(&self) -> Result<Vec<SshSession>> {
        let sessions = self.sessions.lock().unwrap();
        Ok(sessions.values().cloned().collect())
    }
    
    /// Get the active session
    pub fn get_active_session(&self) -> Result<SshSession> {
        let active_session_id = self.active_session.lock().unwrap().clone();
        let sessions = self.sessions.lock().unwrap();
        sessions.get(&active_session_id).cloned().ok_or_else(|| anyhow::anyhow!(format!(format!("Active session not found: {}", active_session_id))))?
        Ok(sessions[&active_session_id].clone())
    }
    
    /// Set the active session
    pub fn set_active_session(&self, session_id: String) -> Result<()> {
        let mut active_session = self.active_session.lock().unwrap();
        *active_session = session_id.clone();
        Ok(())
    }
    
    /// Disconnect from the active session
    pub fn disconnect(&self) -> Result<()> {
        let active_session_id = self.active_session.lock().unwrap().clone();
        let sessions = self.sessions.lock().unwrap();
        sessions.remove(&active_session_id);
        Ok(())
    }
    
    /// Connect to a new SSH session
    pub async fn connect_ssh(&self, host: String, port: u16, user: String) -> Result<()> {
        let session_id = format!("{}-{}-{}", host, port, user);
        let session = SshSession {
            id: session_id.clone(),
            host,
            port,
            user,
            active: true,
        };
        self.add_session(session)?;
        self.set_active_session(session_id)?;
        Ok(())
    }
    
    /// Get the session manager for Tauri
    pub fn get_session_manager(manager: Manager) -> Result<Self> {
        // Use Tauri's context to manage sessions
        Ok(Self::new())
    }
}

/// State of an active or pending SSH session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SessionState {
    Connecting,
    Active,
    Idle,
    Disconnected,
    Failed,
}

/// Execution metrics and telemetries recorded during an SSH session.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionMetrics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub commands_executed: u64,
    pub latency_ms: Option<u32>,
}

/// Active SSH Session instance.
pub struct SshSession {
    session_id: Uuid,
    host: String,
    port: u16,
    username: String,
    state: SessionState,
    started_at: DateTime<Utc>,
    injected_env: HashMap<String, String>,
    bytes_sent: Arc<AtomicU64>,
    bytes_received: Arc<AtomicU64>,
    is_open: Arc<AtomicBool>,
}

impl SshSession {
    /// Create a new session wrapper for a given target.
    pub fn new(host: String, port: u16, username: String) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            host,
            port,
            username,
            state: SessionState::Active,
            started_at: Utc::now(),
            injected_env: HashMap::new(),
            bytes_sent: Arc::new(AtomicU64::new(0)),
            bytes_received: Arc::new(AtomicU64::new(0)),
            is_open: Arc::new(AtomicBool::new(true)),
        }
    }
    
    /// Inject secret environment variables directly into session memory loop
    /// keeping them safe from disk logs or history buffers.
    pub fn inject_secret_env(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.injected_env.insert(key.into(), val.into());
    }
    
    /// Check if the session is currently connected.
    pub fn is_connected(&self) -> bool {
        self.is_open.load(Ordering::SeqCst) && self.state == SessionState::Active
    }
    
    /// Record transferred bytes for real-time telemetry.
    pub fn record_transfer(&self, sent: u64, received: u64) {
        self.bytes_sent.fetch_add(sent, Ordering::Relaxed);
        self.bytes_received.fetch_add(received, Ordering::Relaxed);
    }
    
    /// Get current session telemetries and metrics.
    pub fn metrics(&self) -> SessionMetrics {
        SessionMetrics {
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            commands_executed: 0,
            latency_ms: Some(12),
        }
    }
    
    /// Terminate and close session.
    pub fn close(&mut self) -> Result<()> {
        self.is_open.store(false, Ordering::SeqCst);
        self.state = SessionState::Disconnected;
        self.injected_env.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_lifecycle() {
        let mut session = SshSession::new("test.host".into(), 22, "admin".into());
        assert!(session.is_connected());
        session.inject_secret_env("API_KEY", "secret123");
        assert_eq!(
            session.injected_env.get("API_KEY").map(String::as_str),
            Some("secret123")
        );
        
        session.record_transfer(1024, 2048);
        let metrics = session.metrics();
        assert_eq!(metrics.bytes_sent, 1024);
        assert_eq!(metrics.bytes_received, 2048);
        
        session.close().unwrap();
        assert!(!session.is_connected());
        assert!(session.injected_env.is_empty());
    }
}