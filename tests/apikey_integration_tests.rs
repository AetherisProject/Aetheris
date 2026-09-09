use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use aetheris::apikey::{ApiKeyItem, Provider, RotationStrategy};
use aetheris::vault::store::VaultStore;
use aetheris::crypto::CryptoEngine;
use aetheris::apikey::rotation;
use aetheris::apikey::providers;

#[tokio::test]
async fn test_api_key_rotation() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_rotation")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    
    // Insert an API key
    let api_key_item = ApiKeyItem {
        id: Uuid::new_v4(),
        title: "Test API Key".to_string(),
        provider: Provider::Github,
        api_key: "test_key".to_string(),
        api_secret: None,
        scopes: vec!["read"],
        rotation_strategy: RotationStrategy::Manual,
        last_used: None,
        usage_count: 0,
        max_usage: None,
        notes: "Test notes".to_string(),
        disabled: false,
        tags: vec!["test"],
        created_at: Utc::now(),
        updated_at: Utc::now(),
        expires_at: None,
    };
    
    vault_store.insert(api_key_item)?;
    
    // Rotate the API key
    let new_key = "new_test_key".to_string();
    rotation::rotate_key(&vault_store, api_key_item.id, new_key.clone()).await?;
    
    // Verify rotation
    let updated_item = vault_store.get(&api_key_item.id)?.unwrap();
    assert_ne!(updated_item.api_key, api_key_item.api_key);
    assert_eq!(updated_item.title, "Test API Key");
    
    Ok(())
}

#[tokio::test]
async fn test_api_key_list() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_list")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    
    // Insert API keys
    let api_key_item1 = ApiKeyItem {
        id: Uuid::new_v4(),
        title: "API Key 1".to_string(),
        provider: Provider::Github,
        api_key: "key1".to_string(),
        api_secret: None,
        scopes: vec!["read"],
        rotation_strategy: RotationStrategy::Manual,
        last_used: None,
        usage_count: 0,
        max_usage: None,
        notes: "Test notes 1".to_string(),
        disabled: false,
        tags: vec!["test"],
        created_at: Utc::now(),
        updated_at: Utc::now(),
        expires_at: None,
    };
    
    let api_key_item2 = ApiKeyItem {
        id: Uuid::new_v4(),
        title: "API Key 2".to_string(),
        provider: Provider::Google,
        api_key: "key2".to_string(),
        api_secret: None,
        scopes: vec!["write"],
        rotation_strategy: RotationStrategy::Manual,
        last_used: None,
        usage_count: 0,
        max_usage: None,
        notes: "Test notes 2".to_string(),
        disabled: false,
        tags: vec!["test"],
        created_at: Utc::now(),
        updated_at: Utc::now(),
        expires_at: None,
    };
    
    vault_store.insert(api_key_item1)?;
    vault_store.insert(api_key_item2)?;
    
    // List all API keys
    let api_keys = vault_store.list_items::<ApiKeyItem>()?;
    assert_eq!(api_keys.len(), 2);
    
    Ok(())
}

#[tokio::test]
async fn test_provider_listing() -> Result<()> {
    let providers = providers::list_providers();
    assert!(providers.contains(&"Openai".to_string()));
    assert!(providers.contains(&"GitHub".to_string()));
    assert!(providers.contains(&"Custom".to_string()));
    
    Ok(())
}