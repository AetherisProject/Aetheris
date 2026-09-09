#! API key engine module for Aetheris.

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::crypto::{CryptoEngine, EncryptionKey};
use crate::vault::store::VaultStore;
use crate::vault::item::VaultItem;

/// Supported API key providers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Provider {
    Openai,
    Anthropic,
    Google,
    Aws,
    Github,
    Gitlab,
    Azure,
    Nvidia,
    Huggingface,
    Mistral,
    Openrouter,
    Groq,
    Cohere,
    Stability,
    Custom(String),
}

impl Provider {
    /// Convert provider to string representation.
    pub fn to_string(&self) -> String {
        match self {
            Provider::Openai => "OpenAI".to_string(),
            Provider::Anthropic => "Anthropic".to_string(),
            Provider::Google => "Google".to_string(),
            Provider::Aws => "AWS".to_string(),
            Provider::Github => "GitHub".to_string(),
            Provider::Gitlab => "GitLab".to_string(),
            Provider::Azure => "Azure".to_string(),
            Provider::Nvidia => "NVIDIA".to_string(),
            Provider::Huggingface => "HuggingFace".to_string(),
            Provider::Mistral => "Mistral".to_string(),
            Provider::Openrouter => "OpenRouter".to_string(),
            Provider::Groq => "Groq".to_string(),
            Provider::Cohere => "Cohere".to_string(),
            Provider::Stability => "Stability".to_string(),
            Provider::Custom(name) => name.clone(),
        }
    }

    /// Get API endpoint for this provider.
    pub fn api_endpoint(&self) -> String {
        match self {
            Provider::Openai => "https://api.openai.com/v1".to_string(),
            Provider::Anthropic => "https://api.anthropic.com/v1".to_string(),
            Provider::Google => "https://ai.googleapis.com/v1".to_string(),
            Provider::Aws => "https://bedrock.us-east-1.amazonaws.com".to_string(),
            Provider::Github => "https://api.github.com".to_string(),
            Provider::Gitlab => "https://gitlab.com/api/v4".to_string(),
            Provider::Azure => "https://management.azure.com".to_string(),
            Provider::Nvidia => "https://api.nvidia.com".to_string(),
            Provider::Huggingface => "https://api-inference.huggingface.co".to_string(),
            Provider::Mistral => "https://api.mistral.ai/v1".to_string(),
            Provider::Openrouter => "https://openrouter.ai/api/v1".to_string(),
            Provider::Groq => "https://api.groq.com/v1".to_string(),
            Provider::Cohere => "https://api.cohere.ai/v1".to_string(),
            Provider::Stability => "https://api.stability.ai/v1".to_string(),
            Provider::Custom(_) => String::new(),
        }
    }
}

/// API key health status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Revoked,
    Expired,
    ExpiringSoon,
    RateLimited,
}

/// API key rotation strategy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RotationStrategy {
    /// Never rotate automatically
    Manual,
    /// Rotate after a fixed interval
    Interval(Duration),
    /// Rotate when usage threshold is reached
    UsageThreshold(u64),
    /// Rotate when expiration date is near
    ExpirationBased,
}

/// API key configuration and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    pub id: Uuid,
    pub provider: Provider,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub rotation_strategy: RotationStrategy,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
    pub max_usage: Option<u64>,
    pub tags: Vec<String>,
    pub enabled: bool,
}

impl ApiKeyConfig {
    pub fn should_rotate(&self) -> bool {
        match &self.rotation_strategy {
            RotationStrategy::Manual => false,
            RotationStrategy::Interval(_) => false,
            RotationStrategy::UsageThreshold(threshold) => self.usage_count >= *threshold,
            RotationStrategy::ExpirationBased => self.expires_at.is_some() && Utc::now() >= self.expires_at.unwrap(),
        }
    }
    pub fn is_expired(&self) -> bool {
        self.expires_at.is_some() && Utc::now() >= self.expires_at.unwrap()
    }
    pub fn is_expiring_soon(&self) -> bool {
        self.expires_at.is_some() && Utc::now() + Duration::hours(24) >= self.expires_at.unwrap()
    }
}

/// API key manager.
pub struct ApiKeyManager {
    vault_store: VaultStore,
    crypto_engine: CryptoEngine,
    keys: HashMap<Uuid, ApiKeyConfig>,
    master_key: Option<EncryptionKey>,
}

impl ApiKeyManager {
    /// Create a new API key manager.
    pub fn new(vault_store: VaultStore, crypto_engine: CryptoEngine) -> Self {
        Self {
            vault_store,
            crypto_engine,
            keys: HashMap::new(),
            master_key: None,
        }
    }

    /// Initialize with master encryption key for secure storage.
    pub fn initialize(&mut self, master_key: EncryptionKey) -> Result<()> {
        self.vault_store.initialize(master_key)?;
        self.master_key = Some(master_key);
        Ok(())
    }

    /// Insert a new API key.
    pub async fn insert(&mut self, api_key: ApiKeyItem) -> Result<Uuid> {
        let id = api_key.id();
        self.vault_store.insert(api_key)?;
        let config = ApiKeyConfig {
            id: api_key.id,
            provider: api_key.provider,
            key: api_key.api_key,
            name: api_key.title,
            description: api_key.notes.into(),
            created_at: api_key.created_at,
            expires_at: api_key.expires_at,
            rotation_strategy: api_key.rotation_strategy,
            last_used: api_key.last_used,
            usage_count: api_key.usage_count,
            max_usage: api_key.max_usage,
            tags: api_key.tags,
            enabled: !api_key.disabled,
        };
        self.keys.insert(id, config);
        Ok(id)
    }

    /// Get an API key by ID.
    pub async fn get(&self, id: Uuid) -> Result<Option<ApiKeyItem>> {
        let item = self.vault_store.get(&id)?;
        item.map(|item| item.into())
    }

    /// Update an API key.
    pub async fn update(&mut self, id: Uuid, api_key: ApiKeyItem) -> Result<()> {
        let config = ApiKeyConfig {
            id: api_key.id,
            provider: api_key.provider,
            key: api_key.api_key,
            name: api_key.title,
            description: api_key.notes.into(),
            created_at: api_key.created_at,
            expires_at: api_key.expires_at,
            rotation_strategy: api_key.rotation_strategy,
            last_used: api_key.last_used,
            usage_count: api_key.usage_count,
            max_usage: api_key.max_usage,
            tags: api_key.tags,
            enabled: !api_key.disabled,
        };
        self.keys.insert(id, config);
        self.vault_store.update(api_key)?;
        Ok(())
    }

    /// Delete an API key by ID.
    pub async fn delete(&mut self, id: Uuid) -> Result<()> {
        self.keys.remove(&id);
        self.vault_store.delete(&id)?;
        Ok(())
    }

    /// List all API keys.
    pub async fn list(&self) -> Result<Vec<ApiKeyItem>> {
        let items = self.vault_store.list_items::<ApiKeyItem>()?;
        Ok(items)
    }

    /// Rotate an API key.
    pub async fn rotate_key(&self, id: Uuid, new_key: String) -> Result<()> {
        let vault_store = self.vault_store.clone();
        crate::apikey::rotation::rotate_key(&vault_store, id, new_key).await
    }

    /// Rotate all API keys based on their strategies.
    pub async fn rotate_all_keys(&self) -> Result<()> {
        let vault_store = self.vault_store.clone();
        crate::apikey::rotation::rotate_all_keys(&vault_store).await
    }

    /// List all supported API key providers.
    pub fn list_providers() -> Vec<String> {
        crate::apikey::providers::list_providers()
    }

    /// Get a provider by name.
    pub fn provider_by_name(name: &str) -> Option<Provider> {
        crate::apikey::providers::provider_by_name(name)
    }

    /// Load API keys from a vault store.
    pub fn load_from_vault(vault_store: &VaultStore) -> Result<()> {
        let items = vault_store.list()?;
        let mut self_keys = HashMap::new();

        for item in items {
            if let VaultItem::ApiKey(api_key_item) = item {
                let config = ApiKeyConfig {
                    id: api_key_item.id,
                    provider: api_key_item.provider.clone(),
                    key: api_key_item.api_key.clone(),
                    name: api_key_item.title.clone(),
                    description: Some(api_key_item.notes.clone()),
                    created_at: api_key_item.created_at,
                    expires_at: api_key_item.expires_at,
                    rotation_strategy: api_key_item.rotation_strategy,
                    last_used: api_key_item.last_used,
                    usage_count: api_key_item.usage_count,
                    max_usage: api_key_item.max_usage,
                    tags: api_key_item.tags.clone(),
                    enabled: !api_key_item.disabled,
                };
                self_keys.insert(config.id, config);
            }
        }
        self.keys = self_keys;
        Ok(())
    }

    /// Save API keys to a vault store.
    pub fn save_to_vault(&self, vault_store: &mut VaultStore) -> Result<()> {
        for config in self.keys.values() {
            let api_key_item = ApiKeyItem {
                id: config.id,
                title: config.name.clone(),
                provider: config.provider.clone(),
                api_key: config.key.clone(),
                api_secret: None,
                scopes: vec![],
                rotation_strategy: config.rotation_strategy.clone(),
                last_used: config.last_used,
                usage_count: config.usage_count,
                max_usage: config.max_usage,
                tags: config.tags.clone(),
                notes: config.description.clone().unwrap_or_default(),
                disabled: !config.enabled,
                created_at: config.created_at,
                updated_at: Utc::now(),
                expires_at: config.expires_at,
            };
            let item = VaultItem::ApiKey(api_key_item);
            vault_store.insert(item)?;
        }
        Ok(())
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self {
            vault_store: VaultStore::new(".vault").unwrap(),
            crypto_engine: CryptoEngine::new().unwrap(),
            keys: HashMap::new(),
            master_key: None,
        }
    }
}