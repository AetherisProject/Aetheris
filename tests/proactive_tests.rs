use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use aetheris::proactive::monitor::MonitorConfig;
use aetheris::proactive::policies::{Policy, PolicyType};
use aetheris::vault::store::VaultStore;
use aetheris::crypto::CryptoEngine;

#[tokio::test]
async fn test_proactive_monitor() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_proactive")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    
    let monitor_config = MonitorConfig::new(vault_store);
    
    // Add a policy
    let policy = Policy {
        id: Uuid::new_v4(),
        type_: PolicyType::SuspiciousActivity,
        name: "Test Policy".to_string(),
        description: "Test description".to_string(),
        enabled: true,
        threshold: 10,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    monitor_config.add_policy(policy)?;
    
    // Check suspicious activities
    let suspicious = monitor_config.check_suspicious_activities()?;
    assert!(suspicious.is_empty());
    
    // Monitor data exfiltration
    monitor_config.monitor_data_exfiltration()?;
    
    Ok(())
}

#[tokio::test]
async fn test_alert_suspicious_activities() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_alert")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    
    let monitor_config = MonitorConfig::new(vault_store);
    
    // Simulate suspicious activities
    let suspicious_activities = vec![
        (Utc::now(), Uuid::new_v4(), "Suspicious activity detected".to_string()),
    ];
    
    monitor_config.alert_suspicious_activities(suspicious_activities)?;
    
    Ok(())
}