//! API key rotation logic for Aetheris.
//!
//! Implements rotation strategies for API keys.

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::crypto::EncryptionKey;
use crate::vault::store::VaultStore;
use crate::vault::item::ApiKeyItem;
use crate::apikey::Provider;
use crate::apikey::RotationStrategy;

/// Rotate an API key based on its strategy.
pub async fn rotate_key(vault_store: &VaultStore, api_key_id: Uuid, new_key: String) -> Result<()> {
    let api_key = vault_store.get_item::<ApiKeyItem>(api_key_id)?.ok_or_else(|| anyhow::anyhow!("API key not found"));
    
    // Generate a new encrypted key using the vault's encryption mechanism
    let encrypted_key = vault_store.encrypt_item(&new_key)?;
    
    // Update the API key with the new encrypted key
    let updated_key = ApiKeyItem {
        id: api_key.id,
        title: api_key.title,
        provider: api_key.provider,
        api_key: encrypted_key,
        api_secret: api_key.api_secret,
        scopes: api_key.scopes,
        rotation_strategy: api_key.rotation_strategy,
        last_used: api_key.last_used,
        usage_count: api_key.usage_count + 1,
        max_usage: api_key.max_usage,
        notes: api_key.notes,
        disabled: api_key.disabled,
        tags: api_key.tags,
        created_at: api_key.created_at,
        updated_at: Utc::now(),
        expires_at: api_key.expires_at,
    };
    
    vault_store.update_item(api_key_id, updated_key)?;
    Ok(())
}

/// Rotate all API keys based on their strategies.
pub async fn rotate_all_keys(vault_store: &VaultStore) -> Result<()> {
    let api_keys: Vec<ApiKeyItem> = vault_store.list_items::<ApiKeyItem>()?;
    
    for api_key in api_keys {
        match api_key.rotation_strategy {
            RotationStrategy::Manual => continue,
            RotationStrategy::Interval(_) => continue,
            RotationStrategy::UsageThreshold(threshold) => {
                if api_key.usage_count >= threshold {
                    let new_key = generate_new_key(&api_key.provider)?;
                    rotate_key(vault_store, api_key.id, new_key).await?;
                }
            }
            RotationStrategy::ExpirationBased => {
                if api_key.expires_at.is_some() {
                    let now = Utc::now();
                    if now >= api_key.expires_at.unwrap() {
                        let new_key = generate_new_key(&api_key.provider)?;
                        rotate_key(vault_store, api_key.id, new_key).await?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// Generate a new API key for a given provider.
fn generate_new_key(provider: &Provider) -> Result<String> {
    // In a real implementation, this would generate a secure random key
    // For now, we'll simulate it with a placeholder
    Ok(format!("api_key_{}_{}", provider.to_string(), Uuid::new_v4()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::automock;
    use mockall::predicate::*
    use mockall::*
    
    #[mock]
    pub struct VaultStoreMock;
    
    impl VaultStoreMock {
        pub async fn get_item(&self, id: Uuid) -> Result<Option<ApiKeyItem>> {
            Ok(Some(ApiKeyItem {
                id,
                title: "Test API Key".to_string(),
                provider: Provider::Github,
                api_key: "old_key".to_string(),
                api_secret: None,
                scopes: vec!["read".to_string()],
                rotation_strategy: RotationStrategy::UsageThreshold(10),
                last_used: Some(Utc::now()),
                usage_count: 10,
                max_usage: Some(10),
                notes: "Test notes".to_string(),
                disabled: false,
                tags: vec!["test".to_string()],
                created_at: Utc::now(),
                updated_at: Utc::now(),
                expires_at: None,
            }))
        }
    }
    
    #[tokio::test]
    async fn test_rotate_key() {
        let vault_store = VaultStoreMock::new();
        let id = Uuid::new_v4();
        
        // Mock the encryption logic
        let mock_encrypt = mock_encrypt(vault_store.clone());
        let encrypted_key = mock_encrypt.call_once(|_| Ok("encrypted_key".to_string()));
        
        vault_store.expect_get_item(id).returning(|_| Ok(Some(ApiKeyItem {
            id,
            title: "Test API Key".to_string(),
            provider: Provider::Github,
            api_key: "old_key".to_string(),
            api_secret: None,
            scopes: vec!["read".to_string()],
            rotation_strategy: RotationStrategy::UsageThreshold(10),
            last_used: Some(Utc::now()),
            usage_count: 10,
            max_usage: Some(10),
            notes: "Test notes".to_string(),
            disabled: false,
            tags: vec!["test".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: None,
        })));
        
        vault_store.expect_update_item(id, encrypted_key).returning(|_, _| Ok(()));
        
        let result = rotate_key(&vault_store, id, "new_key".to_string()).await;
        assert!(result.is_ok());
    }
}