//! API key provider management for Aetheris.
//!
//! Implements listing and managing supported API key providers.

use anyhow::{Context, Result};
use std::collections::HashMap;

use crate::apikey::Provider;

/// List all supported API key providers.
pub fn list_providers() -> Vec<String> {
    vec![
        "Openai".to_string(),
        "Anthropic".to_string(),
        "Google".to_string(),
        "AWS".to_string(),
        "Github".to_string(),
        "Gitlab".to_string(),
        "Azure".to_string(),
        "Nvidia".to_string(),
        "Huggingface".to_string(),
        "Mistral".to_string(),
        "Openrouter".to_string(),
        "Groq".to_string(),
        "Cohere".to_string(),
        "Stability".to_string(),
        "Custom".to_string(),
    ]
}

/// Get a provider by its name.
pub fn provider_by_name(name: &str) -> Option<Provider> {
    match name.to_lowercase().as_str() {
        "openai" => Some(Provider::Openai),
        "anthropic" => Some(Provider::Anthropic),
        "google" => Some(Provider::Google),
        "aws" => Some(Provider::Aws),
        "github" => Some(Provider::Github),
        "gitlab" => Some(Provider::Gitlab),
        "azure" => Some(Provider::Azure),
        "nvidia" => Some(Provider::Nvidia),
        "huggingface" => Some(Provider::Huggingface),
        "mistral" => Some(Provider::Mistral),
        "openrouter" => Some(Provider::Openrouter),
        "groq" => Some(Provider::Groq),
        "cohere" => Some(Provider::Cohere),
        "stability" => Some(Provider::Stability),
        "custom" => Some(Provider::Custom(name.to_string())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_list_providers() {
        let providers = list_providers();
        assert!(providers.contains(&"Openai".to_string()));
        assert!(providers.contains(&"GitHub".to_string()));
        assert!(providers.contains(&"Custom".to_string()));
    }
    
    #[test]
    fn test_provider_by_name() {
        assert_eq!(provider_by_name("Openai"), Some(Provider::Openai));
        assert_eq!(provider_by_name("GitHub"), Some(Provider::Github));
        assert_eq!(provider_by_name("custom"), Some(Provider::Custom("custom".to_string())));
        assert_eq!(provider_by_name("unknown"), None);
    }
}