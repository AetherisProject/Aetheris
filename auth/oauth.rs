//! OAuth2 provider and client for Aetheris.
//!
//! Implements OAuth2 provider and client for secure authentication.

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::auth::session::Session;
use crate::vault::store::VaultStore;
use crate::crypto::CryptoEngine;

/// OAuth2 provider type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Provider {
    GitHub,
    Google,
    Microsoft,
    Azure,
    GitLab,
    OpenIdConnect,
    Custom(String),
}

impl Provider {
    /// Get the authorization URL for this provider.
    pub fn authorization_url(&self) -> String {
        match self {
            Provider::GitHub => "https://github.com/login/oauth/authorize",
            Provider::Google => "https://accounts.google.com/o/oauth2/v2/auth",
            Provider::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
            Provider::Azure => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
            Provider::GitLab => "https://gitlab.com/oauth/authorize",
            Provider::OpenIdConnect => "https://openid.net/connect/authenticate",
            Provider::Custom(name) => format!("https://{}/oauth/authorize", name),
        }
    }

    /// Get the token URL for this provider.
    pub fn token_url(&self) -> String {
        match self {
            Provider::GitHub => "https://github.com/login/oauth/access_token",
            Provider::Google => "https://oauth2.googleapis.com/token",
            Provider::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            Provider::Azure => "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            Provider::GitLab => "https://gitlab.com/oauth/token",
            Provider::OpenIdConnect => "https://openid.net/connect/token",
            Provider::Custom(name) => format!("https://{}/oauth/token", name),
        }
    }

    /// Get the client ID for this provider.
    pub fn client_id(&self) -> String {
        match self {
            Provider::GitHub => "github_client_id",
            Provider::Google => "google_client_id",
            Provider::Microsoft => "microsoft_client_id",
            Provider::Azure => "azure_client_id",
            Provider::GitLab => "gitlab_client_id",
            Provider::OpenIdConnect => "openid_connect_client_id",
            Provider::Custom(name) => name.clone(),
        }
    }

    /// Get the client secret for this provider.
    pub fn client_secret(&self) -> String {
        match self {
            Provider::GitHub => "github_client_secret",
            Provider::Google => "google_client_secret",
            Provider::Microsoft => "microsoft_client_secret",
            Provider::Azure => "azure_client_secret",
            Provider::GitLab => "gitlab_client_secret",
            Provider::OpenIdConnect => "openid_connect_client_secret",
            Provider::Custom(name) => format!("{}-client-secret", name),
        }
    }
}

/// OAuth2 token response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
    refresh_token: Option<String>,
}

impl TokenResponse {
    /// Parse a token response from JSON.
    pub fn from_json(json: &str) -> Result<Self> {
        let response: serde_json::Value = serde_json::from_str(json)?;
        Ok(TokenResponse {
            access_token: response["access_token"].as_str().ok_or_else(|| anyhow::anyhow!("Missing access token"))?.to_string(),
            token_type: response["token_type"].as_str().ok_or_else(|| anyhow::anyhow!("Missing token type"))?.to_string(),
            expires_in: response["expires_in"].as_u64().ok_or_else(|| anyhow::anyhow!("Missing expires_in"))?,
            scope: response["scope"].as_str().ok_or_else(|| anyhow::anyhow!("Missing scope"))?.to_string(),
            refresh_token: response["refresh_token"].as_str().map(|s| s.to_string()),
        })
    }
}

/// OAuth2 client for Aetheris.
pub struct OAuth2Client {
    provider: Provider,
    vault_store: VaultStore,
    crypto_engine: CryptoEngine,
    session_store: VaultStore,
}

impl OAuth2Client {
    /// Create a new OAuth2 client.
    pub fn new(provider: Provider, vault_store: VaultStore, session_store: VaultStore) -> Self {
        OAuth2Client {
            provider,
            vault_store,
            crypto_engine: CryptoEngine::new().unwrap(),
            session_store,
        }
    }

    /// Get the authorization URL for this provider.
    pub fn get_authorization_url(&self) -> String {
        self.provider.authorization_url()
    }

    /// Exchange authorization code for tokens.
    pub async fn exchange_code(&self, code: String) -> Result<TokenResponse> {
        let token_url = self.provider.token_url();
        let client_id = self.provider.client_id();
        let client_secret = self.provider.client_secret();
        
        // In a real implementation, this would call the provider's API
        // to exchange the authorization code for tokens.
        let token_response = TokenResponse {
            access_token: format!("access_token_{}", code),
            token_type: "Bearer".to_string(),
            expires_in: 3600, // 1 hour
            scope: "read write".to_string(),
            refresh_token: None,
        };
        
        Ok(token_response)
    }

    /// Create a session from tokens.
    pub async fn create_session(&self, token_response: &TokenResponse) -> Result<Session> {
        let session_id = Uuid::new_v4();
        let session = Session {
            id: session_id,
            user_id: "user123".to_string(), // Placeholder
            provider: self.provider.clone(),
            access_token: token_response.access_token.clone(),
            expires_at: Utc::now() + chrono::Duration::seconds(token_response.expires_in as i64),
            created_at: Utc::now(),
        };
        
        // Store the session
        let session_item = crate::auth::session::SessionItem {
            id: session_id,
            user_id: session.user_id,
            provider: session.provider,
            access_token: session.access_token,
            expires_at: session.expires_at,
            created_at: session.created_at,
        };
        
        self.session_store.insert(session_item)?;
        
        Ok(session)
    }

    /// Validate a session.
    pub async fn validate_session(&self, session_id: Uuid) -> Result<bool> {
        let session_item = self.session_store.get(&session_id)?;
        if let Some(session_item) = session_item {
            let session = Session {
                id: session_item.id,
                user_id: session_item.user_id,
                provider: session_item.provider,
                access_token: session_item.access_token,
                expires_at: session_item.expires_at,
                created_at: session_item.created_at,
            };
            
            // In a real implementation, validate the token with the provider
            Ok(session.expires_at > Utc::now())
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_oauth2_client() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_oauth")?;
        let session_store = VaultStore::new(".session_store_test")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        session_store.initialize(master_key)?;
        
        let provider = Provider::GitHub;
        let oauth2_client = OAuth2Client::new(provider, vault_store, session_store);
        
        // Test authorization URL
        let url = oauth2_client.get_authorization_url();
        assert_eq!(url, "https://github.com/login/oauth/authorize");
        
        // Test token exchange
        let token_response = oauth2_client.exchange_code("test_code".to_string()).await?;
        assert_eq!(token_response.token_type, "Bearer");
        
        // Test session creation
        let session = oauth2_client.create_session(&token_response).await?;
        assert!(oauth2_client.validate_session(session.id).await?);
        
        Ok(())
    }
}