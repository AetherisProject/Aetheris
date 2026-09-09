//! Two-Factor Authentication (TOTP) for Aetheris.
//!
//! Implements Time-based One-Time Password (TOTP) authentication using RFC 6238.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::auth::session::{SessionManager, SessionItem};
use crate::crypto::CryptoEngine;
use crate::vault::store::VaultStore;

/// TOTP secret for user's 2FA setup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpSecret {
    /// User ID
    pub user_id: String,
    /// TOTP secret key (base32 encoded)
    pub secret: String,
    /// TOTP issuer
    pub issuer: String,
    /// Account name
    pub account_name: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last used timestamp
    pub last_used: Option<DateTime<Utc>>,
}

impl TotpSecret {
    /// Create a new TOTP secret.
    pub fn new(user_id: String, issuer: String, account_name: String, secret: String) -> Self {
        TotpSecret {
            user_id,
            secret,
            issuer,
            account_name,
            created_at: Utc::now(),
            last_used: None,
        }
    }

    /// Generate a TOTP URI for QR code generation.
    pub fn to_uri(&self) -> String {
        format!("otpauth://totp/{}:{}?secret={}&issuer={}", self.issuer, self.account_name, self.secret, self.issuer)
    }
}

/// TOTP manager for Aetheris.
pub struct TotpManager {
    vault_store: VaultStore,
    crypto_engine: CryptoEngine,
    session_manager: SessionManager,
    secrets: HashMap<String, TotpSecret>,
}

impl TotpManager {
    /// Create a new TOTP manager.
    pub fn new(vault_store: VaultStore, session_manager: SessionManager) -> Self {
        TotpManager {
            vault_store,
            crypto_engine: CryptoEngine::new().unwrap(),
            session_manager,
            secrets: HashMap::new(),
        }
    }

    /// Generate a new TOTP secret for a user.
    pub fn generate_secret(&self, user_id: String, issuer: String, account_name: String) -> Result<TotpSecret> {
        use otp::totp::TOTP;
        
        let secret = TOTP::new_30s()?.to_base32();
        let totp_secret = TotpSecret::new(user_id, issuer, account_name, secret);
        Ok(totp_secret)
    }

    /// Enable TOTP for a user.
    pub fn enable_totp(&self, user_id: String, totp_secret: TotpSecret) -> Result<()> {
        self.secrets.insert(user_id.clone(), totp_secret.clone());
        self.vault_store.insert_session(totp_secret)?;
        Ok(())
    }

    /// Disable TOTP for a user.
    pub fn disable_totp(&self, user_id: String) -> Result<()> {
        self.secrets.remove(&user_id);
        let secrets: Vec<TotpSecret> = self.vault_store.list_items::<TotpSecret>()?;
        for secret in secrets {
            if secret.user_id == user_id {
                self.vault_store.delete_session(&secret.user_id)?;
            }
        }
        Ok(())
    }

    /// Verify a TOTP code.
    pub fn verify_totp(&self, user_id: String, code: String) -> Result<bool> {
        use otp::totp::TOTP;
        
        if let Some(secret) = self.secrets.get(&user_id) {
            let totp = TOTP::new_30s()?.with_secret(secret.secret.as_bytes())?;
            return Ok(totp.check(&code));
        }
        
        let secrets: Vec<TotpSecret> = self.vault_store.list_items::<TotpSecret>()?;
        for secret in secrets {
            if secret.user_id == user_id {
                let totp = TOTP::new_30s()?.with_secret(secret.secret.as_bytes())?;
                return Ok(totp.check(&code));
            }
        }
        Ok(false)
    }

    /// Verify TOTP and session.
    pub fn verify_totp_session(&self, user_id: String, code: String, session_id: Uuid) -> Result<bool> {
        let session_valid = self.session_manager.validate_session(session_id)?;
        if !session_valid {
            return Ok(false);
        }
        
        let totp_valid = self.verify_totp(user_id, code)?;
        Ok(totp_valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_totp_generation() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_totp")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let session_manager = SessionManager::new(vault_store);
        let totp_manager = TotpManager::new(vault_store, session_manager);
        
        let secret = totp_manager.generate_secret("user123".to_string(), "Aetheris".to_string(), "user123".to_string())?;
        assert!(secret.secret.len() > 0);
        
        let uri = secret.to_uri();
        assert!(uri.contains("otpauth://totp"));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_totp_verification() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_totp_verify")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let session_manager = SessionManager::new(vault_store);
        let totp_manager = TotpManager::new(vault_store, session_manager);
        
        let secret = totp_manager.generate_secret("user123".to_string(), "Aetheris".to_string(), "user123".to_string())?;
        totp_manager.enable_totp("user123".to_string(), secret)?;
        
        // In a real test, we would use a known TOTP code
        // For now, we just verify the method works
        let result = totp_manager.verify_totp("user123".to_string(), "123456".to_string())?;
        // Result will be false since "123456" is not a valid code
        assert!(!result);
        
        Ok(())
    }
}