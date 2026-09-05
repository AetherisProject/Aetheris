//! API key engine module for Aetheris.
pub mod health;
pub mod injection;
pub mod providers;
pub mod rotation;

use anyhow::Result;

/// Supported API key providers.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

/// API key health status.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HealthStatus {
    Healthy,
    ExpiringSoon,
    Expired,
    RateLimited,
    Revoked,
    Unknown,
}

/// API key manager.
pub struct ApiKeyManager;

impl ApiKeyManager {
    pub fn new() -> Self { Self }
    pub fn check_health(&self, _provider: &Provider) -> Result<HealthStatus> {
        unimplemented!("check_health not yet implemented")
    }
    pub fn rotate(&self, _provider: &Provider) -> Result<()> {
        unimplemented!("rotate not yet implemented")
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_api_key_manager_new() { let _m = ApiKeyManager::new(); }
}
