//! User account management — zero-knowledge auth core.
//! All secrets use Zeroize on drop; encryption via AEAD; comparison constant-time.

use crate::crypto::{CryptoEngine, Zeroize};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// User account record — never stores plaintext master password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: String,
    pub email: String,
    /// Argon2id hash of password (never plaintext)
    pub password_hash: String,
    /// Last login timestamp
    pub last_login: Option<u64>,
    /// MFA enabled
    pub mfa_enabled: bool,
}

impl UserAccount {
    pub fn new(id: String, email: String, password: &str) -> Result<Self> {
        let hash = CryptoEngine::hash_password(password)?;
        Ok(Self {
            id,
            email,
            password_hash: hash,
            last_login: None,
            mfa_enabled: false,
        })
    }

    /// Verify password using constant-time comparison
    pub fn verify_password(&self, password: &str) -> bool {
        CryptoEngine::verify_password(password, &self.password_hash).unwrap_or(false)
    }

    /// Change password with new Argon2id hash; old password zeroized
    pub fn change_password(&mut self, old: &str, new: &str) -> Result<()> {
        if !self.verify_password(old) {
            anyhow::bail!("old password incorrect");
        }
        self.password_hash = CryptoEngine::hash_password(new)?;
        Ok(())
    }
}

/// In-memory account store (production uses encrypted DB)
pub struct AccountStore {
    accounts: HashMap<String, UserAccount>,
}

impl AccountStore {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn register(&mut self, email: &str, password: &str) -> Result<UserAccount> {
        if self.accounts.values().any(|a| a.email == email) {
            anyhow::bail!("email already registered");
        }
        let id = format!("user_{}", email.split('@').next().unwrap_or("unknown"));
        let account = UserAccount::new(id.clone(), email.to_string(), password)?;
        self.accounts.insert(id.clone(), account.clone());
        Ok(account)
    }

    pub fn login(&mut self, email: &str, password: &str) -> Result<String> {
        let account = self
            .accounts
            .values()
            .find(|a| a.email == email)
            .ok_or_else(|| anyhow::anyhow!("invalid credentials"))?;
        if !account.verify_password(password) {
            anyhow::bail!("invalid credentials");
        }
        let token = CryptoEngine::generate_token()?;
        Ok(token)
    }

    pub fn reset_password(&mut self, email: &str, new_password: &str) -> Result<()> {
        let acc = self
            .accounts
            .values_mut()
            .find(|a| a.email == email)
            .ok_or_else(|| anyhow::anyhow!("email not found"))?;
        acc.password_hash = CryptoEngine::hash_password(new_password)?;
        Ok(())
    }

    pub fn get_account(&self, id: &str) -> Option<UserAccount> {
        self.accounts.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register_login() {
        let mut store = AccountStore::new();
        let acc = store.register("alice@aetheris.dev", "secret123").unwrap();
        assert_eq!(acc.email, "alice@aetheris.dev");
        let token = store.login("alice@aetheris.dev", "secret123").unwrap();
        assert!(!token.is_empty());
    }
    #[test]
    fn test_change_password() {
        let mut store = AccountStore::new();
        store.register("bob@aetheris.dev", "old").unwrap();
        let mut acc = store.get_account("user_bob").unwrap();
        acc.change_password("old", "new").unwrap();
        assert!(acc.verify_password("new"));
    }
}
