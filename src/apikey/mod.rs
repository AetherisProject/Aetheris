//! API key engine module for Aetheris.
pub mod health;
pub mod injection;
pub mod providers;
pub mod rotation;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::crypto::EncryptionKey;
use crate::vault::VaultItem;

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
    ExpiringSoon,
    Expired,
    RateLimited,
    Revoked,
    Unknown,
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
    pub fn new(provider: Provider, key: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            provider: provider.clone(),
            key,
            name: format!("{} API Key", provider.to_string()),
            description: None,
            created_at: Utc::now(),
            expires_at: None,
            rotation_strategy: RotationStrategy::Manual,
            last_used: None,
            usage_count: 0,
            max_usage: None,
            tags: vec![],
            enabled: true,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires) => Utc::now() >= expires,
            None => false,
        }
    }

    pub fn is_expiring_soon(&self) -> bool {
        match self.expires_at {
            Some(expires) => {
                let now = Utc::now();
                let twenty_four_hours = chrono::Duration::hours(24);
                expires - now <= twenty_four_hours
            }
            None => false,
        }
    }

    pub fn should_rotate(&self) -> bool {
        match &self.rotation_strategy {
            RotationStrategy::Manual => false,
            RotationStrategy::Interval(interval) => {
                let last_used = self.last_used.unwrap_or(self.created_at);
                let now = Utc::now();
                let duration_since_usage = now.signed_duration_since(last_used);
                // Convert chrono::Duration to std::time::Duration for comparison
                let std_duration =
                    std::time::Duration::from_secs(duration_since_usage.num_seconds() as u64);
                std_duration >= *interval
            }
            RotationStrategy::UsageThreshold(threshold) => self.usage_count >= *threshold,
            RotationStrategy::ExpirationBased => self.is_expiring_soon(),
        }
    }
}

/// API key manager.
pub struct ApiKeyManager {
    keys: HashMap<Uuid, ApiKeyConfig>,
    master_key: Option<EncryptionKey>,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            master_key: None,
        }
    }

    /// Initialize with master encryption key for secure storage.
    pub fn initialize(&mut self, master_key: EncryptionKey) -> Result<()> {
        self.master_key = Some(master_key);
        Ok(())
    }

    /// Add a new API key.
    pub fn add_key(&mut self, config: ApiKeyConfig) -> Result<Uuid> {
        let id = config.id;
        self.keys.insert(id, config);
        Ok(id)
    }

    /// Get API key by ID.
    pub fn get_key(&self, id: &Uuid) -> Option<&ApiKeyConfig> {
        self.keys.get(id)
    }

    /// Get API key by mutable reference.
    pub fn get_key_mut(&mut self, id: &Uuid) -> Option<&mut ApiKeyConfig> {
        self.keys.get_mut(id)
    }

    /// Remove an API key.
    pub fn remove_key(&mut self, id: &Uuid) -> Option<ApiKeyConfig> {
        self.keys.remove(id)
    }

    /// List all API keys for a specific provider.
    pub fn list_keys_by_provider(&self, provider: &Provider) -> Vec<&ApiKeyConfig> {
        self.keys
            .values()
            .filter(|config| &config.provider == provider)
            .collect()
    }

    /// List all API keys.
    pub fn list_all_keys(&self) -> Vec<&ApiKeyConfig> {
        self.keys.values().collect()
    }

    /// Check the health of an API key.
    pub fn check_health(&self, id: &Uuid) -> Result<HealthStatus> {
        let config = self
            .keys
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("API key not found"))?;

        if !config.enabled {
            return Ok(HealthStatus::Revoked);
        }

        if config.is_expired() {
            return Ok(HealthStatus::Expired);
        }

        if config.is_expiring_soon() {
            return Ok(HealthStatus::ExpiringSoon);
        }

        // TODO: Implement actual health checking by calling provider APIs
        if config.should_rotate() {
            return Ok(HealthStatus::RateLimited); // Placeholder
        }

        Ok(HealthStatus::Healthy)
    }

    /// Rotate an API key (generate new key and replace old one).
    pub fn rotate(&mut self, id: &Uuid) -> Result<String> {
        let mut config = self
            .keys
            .remove(id)
            .ok_or_else(|| anyhow::anyhow!("API key not found"))?;

        // Generate new key (placeholder - in reality this would call the provider's API)
        let new_key = self.generate_key_for_provider(&config.provider)?;

        // Update the key
        config.key = new_key.clone();
        config.last_used = Some(Utc::now());
        config.usage_count = 0;
        config.created_at = Utc::now();

        // Put the config back
        self.keys.insert(id.clone(), config);

        Ok(new_key)
    }

    /// Generate a new API key for the given provider (mock implementation).
    fn generate_key_for_provider(&self, provider: &Provider) -> Result<String> {
        // In a real implementation, this would call the provider's API
        // to generate new keys. For now, we'll generate a mock key.
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let prefix = match provider {
            Provider::Openai => "sk-",
            Provider::Anthropic => "sk-",
            Provider::Google => "AIza",
            Provider::Aws => "AKIA",
            Provider::Github => "ghp_",
            Provider::Gitlab => "glpat-",
            Provider::Azure => "azure-",
            Provider::Nvidia => "nv-",
            Provider::Huggingface => "hf_",
            Provider::Mistral => "mistral-",
            Provider::Openrouter => "sk-",
            Provider::Groq => "gsk-",
            Provider::Cohere => "cohere-",
            Provider::Stability => "stability-",
            Provider::Custom(name) => &format!("{}-", name.to_lowercase()),
        };

        let random_suffix: String = (0..32)
            .map(|_| {
                const CHARSET: &[u8] =
                    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        Ok(format!("{}{}", prefix, random_suffix))
    }

    /// Get an API key for a specific provider (useful for automatic injection).
    pub fn get_key_for_provider(&self, provider: &Provider) -> Option<&ApiKeyConfig> {
        // Get the first enabled key for this provider
        self.list_keys_by_provider(provider)
            .into_iter()
            .find(|config| config.enabled && !config.is_expired())
    }

    /// Get as environment variable suitable for injection.
    pub fn get_key_as_env(&self, provider: &Provider, env_var_name: &str) -> Option<String> {
        let config = self.get_key_for_provider(provider)?;
        Some(format!("{}={}", env_var_name, config.key))
    }

    /// Get all keys as environment variables.
    pub fn get_all_keys_as_env(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();

        for config in self.list_all_keys() {
            if !config.enabled {
                continue;
            }

            let var_name = format!("{}_API_KEY", config.provider.to_string().to_uppercase());
            env_vars.insert(var_name, config.key.clone());
        }

        env_vars
    }

    /// Increment usage count for an API key.
    pub fn increment_usage(&mut self, id: &Uuid) -> Result<()> {
        let config = self
            .keys
            .get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("API key not found"))?;
        config.usage_count += 1;
        config.last_used = Some(Utc::now());
        Ok(())
    }

    /// Load API keys from a vault store.
    pub fn load_from_vault(&mut self, vault_store: &crate::vault::store::VaultStore) -> Result<()> {
        let items = vault_store.list()?;

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

                self.keys.insert(config.id, config);
            }
        }

        Ok(())
    }

    /// Save API keys to a vault store.
    pub fn save_to_vault(&self, vault_store: &mut crate::vault::store::VaultStore) -> Result<()> {
        for config in self.list_all_keys() {
            let api_key_item = crate::vault::ApiKeyItem {
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
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_api_key_manager_new() {
        let _m = ApiKeyManager::new();
    }
}
