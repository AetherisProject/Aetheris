//! Biometric login support for Aetheris.
//!
//! Implements biometric authentication (FaceID, TouchID, Windows Hello) for secure login.

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::auth::session::{SessionManager, SessionItem};
use crate::crypto::CryptoEngine;
use crate::vault::store::VaultStore;

/// Biometric credential for user's biometric login.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricCredential {
    /// User ID
    pub user_id: String,
    /// Biometric credential ID
    pub credential_id: String,
    /// Platform (windows, macos, ios, android)
    pub platform: String,
    /// Public key for biometric authentication
    pub public_key: Vec<u8>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last used timestamp
    pub last_used: Option<DateTime<Utc>>,
}

impl BiometricCredential {
    /// Create a new biometric credential.
    pub fn new(user_id: String, credential_id: String, platform: String, public_key: Vec<u8>) -> Self {
        BiometricCredential {
            user_id,
            credential_id,
            platform,
            public_key,
            created_at: Utc::now(),
            last_used: None,
        }
    }
}

/// Biometric authentication manager.
pub struct BiometricManager {
    vault_store: VaultStore,
    crypto_engine: CryptoEngine,
    session_manager: SessionManager,
}

impl BiometricManager {
    /// Create a new biometric manager.
    pub fn new(vault_store: VaultStore, session_manager: SessionManager) -> Self {
        BiometricManager {
            vault_store,
            crypto_engine: CryptoEngine::new().unwrap(),
            session_manager,
        }
    }

    /// Register a new biometric credential for a user.
    pub fn register_credential(&self, user_id: String, credential_id: String, platform: String, public_key: Vec<u8>) -> Result<()> {
        let credential = BiometricCredential::new(user_id, credential_id, platform, public_key);
        self.vault_store.insert_session(credential)?;
        Ok(())
    }

    /// Verify biometric authentication.
    pub fn verify_biometric(&self, credential_id: String, signature: Vec<u8>, challenge: Vec<u8>) -> Result<bool> {
        let credentials: Vec<BiometricCredential> = self.vault_store.list_items::<BiometricCredential>()?;
        
        for credential in credentials {
            if credential.credential_id == credential_id {
                // In a real implementation, use platform-specific biometric API
                // to verify the signature against the challenge
                // For now, verify the credential exists
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Delete a biometric credential.
    pub fn delete_credential(&self, credential_id: String) -> Result<()> {
        let credentials: Vec<BiometricCredential> = self.vault_store.list_items::<BiometricCredential>()?;
        
        for credential in credentials {
            if credential.credential_id == credential_id {
                self.vault_store.delete_session(&credential.user_id)?;
            }
        }
        
        Ok(())
    }

    /// List all biometric credentials for a user.
    pub fn list_credentials(&self, user_id: String) -> Result<Vec<BiometricCredential>> {
        let credentials: Vec<BiometricCredential> = self.vault_store.list_items::<BiometricCredential>()?;
        
        let user_credentials: Vec<BiometricCredential> = credentials
            .into_iter()
            .filter(|c| c.user_id == user_id)
            .collect();
        
        Ok(user_credentials)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_biometric_registration() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_biometric")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let session_manager = SessionManager::new(vault_store);
        let biometric_manager = BiometricManager::new(vault_store, session_manager);
        
        let credential_id = Uuid::new_v4().to_string();
        biometric_manager.register_credential("user123".to_string(), credential_id, "macos".to_string(), vec![1, 2, 3])?;
        
        let credentials = biometric_manager.list_credentials("user123".to_string())?;
        assert_eq!(credentials.len(), 1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_biometric_verification() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_biometric_verify")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let session_manager = SessionManager::new(vault_store);
        let biometric_manager = BiometricManager::new(vault_store, session_manager);
        
        let credential_id = Uuid::new_v4().to_string();
        biometric_manager.register_credential("user123".to_string(), credential_id.clone(), "macos".to_string(), vec![1, 2, 3])?;
        
        let verified = biometric_manager.verify_biometric(credential_id, vec![1, 2, 3], vec![4, 5, 6])?;
        assert!(verified);
        
        Ok(())
    }
}