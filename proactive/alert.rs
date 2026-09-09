//! Proactive alert system for Aetheris.
//!
//! Implements background monitoring and alerting for proactive security.

use anyhow::{Context, Result};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use chrono::{DateTime, Utc};

use crate::proactive::monitor::MonitorConfig;
use crate::vault::store::VaultStore;
use crate::crypto::CryptoEngine;

/// Proactive alert system.
pub struct ProactiveAlertSystem {
    monitor_config: Arc<Mutex<MonitorConfig>>,
    vault_store: VaultStore,
    crypto_engine: CryptoEngine,
    
    // Track running tasks
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl ProactiveAlertSystem {
    /// Create a new proactive alert system.
    pub fn new(vault_store: VaultStore) -> Self {
        let crypto_engine = CryptoEngine::new().unwrap();
        let monitor_config = MonitorConfig::new(vault_store);
        ProactiveAlertSystem {
            monitor_config: Arc::new(Mutex::new(monitor_config)),
            vault_store,
            crypto_engine,
        }
    }

    /// Start monitoring for suspicious activities.
    pub fn start_monitoring(&mut self) -> Result<()> {
        let monitor_config = self.monitor_config.clone();
        let vault_store = self.vault_store.clone();
        let crypto_engine = self.crypto_engine.clone();
        
        // Start monitoring tasks
        let task = task::spawn(async move {
            loop {
                // Check for suspicious activities
                let suspicious = monitor_config.lock().await.check_suspicious_activities().unwrap();
                
                if !suspicious.is_empty() {
                    monitor_config.lock().await.alert_suspicious_activities(suspicious).unwrap();
                }
                
                // Check for data exfiltration
                monitor_config.lock().await.monitor_data_exfiltration().unwrap();
                
                // Check for unusual access patterns
                monitor_config.lock().await.monitor_unusual_access().unwrap();
                
                // Wait for the next check
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });
        
        self.tasks.push(task);
        Ok(())
    }

    /// Stop monitoring.
    pub fn stop_monitoring(&mut self) -> Result<()> {
        for task in &mut self.tasks {
            task.abort();
        }
        self.tasks.clear();
        Ok(())
    }

    /// Add a policy.
    pub fn add_policy(&self, policy: crate::proactive::policies::Policy) -> Result<()> {
        let mut monitor_config = self.monitor_config.lock().await;
        monitor_config.add_policy(policy)?;
        Ok(())
    }

    /// Remove a policy.
    pub fn remove_policy(&self, policy_id: &Uuid) -> Result<()> {
        let mut monitor_config = self.monitor_config.lock().await;
        monitor_config.remove_policy(policy_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_proactive_alert_system() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_alert")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let mut system = ProactiveAlertSystem::new(vault_store);
        
        // Add a policy
        let policy = crate::proactive::policies::Policy::new(
            crate::proactive::policies::PolicyType::SuspiciousActivity,
            "Test Policy".to_string(),
            "Test description".to_string(),
            10,
        );
        system.add_policy(policy)?;
        
        // Start monitoring
        system.start_monitoring()?;
        
        // Wait for monitoring to complete (simplified test)
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        Ok(())
    }
}