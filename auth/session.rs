//! Session-based authentication for Aetheris.
//!
//! Implements session-based authentication and management.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::auth::oauth::Provider;
use crate::vault::store::VaultStore;
use crate::vault::item::VaultItem;

/// Session for session-based authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: String,
    pub provider: Provider,
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Session item for storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionItem {
    pub id: Uuid,
    pub user_id: String,
    pub provider: Provider,
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl SessionItem {
    /// Create a new session item.
    pub fn new(session: &Session) -> Self {
        SessionItem {
            id: session.id,
            user_id: session.user_id.clone(),
            provider: session.provider.clone(),
            access_token: session.access_token.clone(),
            expires_at: session.expires_at,
            created_at: session.created_at,
        }
    }
}

/// Session manager for session-based authentication.
pub struct SessionManager {
    vault_store: VaultStore,
    crypto_engine: crate::crypto::CryptoEngine,
}

impl SessionManager {
    /// Create a new session manager.
    pub fn new(vault_store: VaultStore) -> Self {
        SessionManager {
            vault_store,
            crypto_engine: crate::crypto::CryptoEngine::new().unwrap(),
        }
    }

    /// Create a new session.
    pub async fn create_session(&self, session: Session) -> Result<()> {
        let session_item = SessionItem::new(&session);
        self.vault_store.insert(session_item)?;
        Ok(())
    }

    /// Get a session by ID.
    pub async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        let session_item = self.vault_store.get(&session_id)?;
        session_item.map(|item| Session {
            id: item.id,
            user_id: item.user_id,
            provider: item.provider,
            access_token: item.access_token,
            expires_at: item.expires_at,
            created_at: item.created_at,
        })
    }

    /// Validate a session.
    pub async fn validate_session(&self, session_id: Uuid) -> Result<bool> {
        let session_item = self.vault_store.get(&session_id)?;
        if let Some(session_item) = session_item {
            Ok(session_item.expires_at > Utc::now())
        } else {
            Ok(false)
        }
    }

    /// Delete a session.
    pub async fn delete_session(&self, session_id: Uuid) -> Result<()> {
        self.vault_store.delete(&session_id)?;
        Ok(())
    }

    /// List all sessions.
    pub async fn list_sessions(&self) -> Result<Vec<SessionItem>> {
        self.vault_store.list_items::<SessionItem>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_session_manager() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_session")?;
        let crypto_engine = crate::crypto::CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let session_manager = SessionManager::new(vault_store);
        
        // Create a session
        let session = Session {
            id: Uuid::new_v4(),
            user_id: "user123".to_string(),
            provider: Provider::GitHub,
            access_token: "test_token".to_string(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            created_at: Utc::now(),
        };
        
        session_manager.create_session(session).await?;
        
        // Get the session
        let session_item = session_manager.get_session(session.id).await?;
        assert!(session_item.is_some());
        
        // Validate the session
        assert!(session_manager.validate_session(session.id).await?);
        
        Ok(())
    }
}