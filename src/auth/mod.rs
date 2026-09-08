//! Authentication system module for Aetheris.
pub mod account;
pub mod session;
pub mod mfa;
pub mod oauth;
pub mod totp;
pub mod recovery;
pub mod consensus;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct UserAccount {
    pub id: String,
    pub email: String,
}

pub struct AuthManager {
    account_store: crate::auth::account::AccountStore,
    session_store: crate::auth::session::SessionStore,
}

impl AuthManager {
    pub fn new() -> Self {
        Self { account_store: crate::auth::account::AccountStore::new(), session_store: crate::auth::session::SessionStore::new() }
    }
    pub fn register(&mut self, email: &str, password: &str) -> Result<crate::auth::account::UserAccount> {
        self.account_store.register(email, password)
    }
    pub fn login(&mut self, email: &str, password: &str) -> Result<String> {
        let token = self.account_store.login(email, password)?;
        let user_id = email.split('@').next().unwrap_or("unknown").to_string();
        let session = self.session_store.create(&user_id)?;
        Ok(session.token)
    }
    pub fn change_password(&mut self, email: &str, old: &str, new: &str) -> Result<()> {
        let id = email.split('@').next().unwrap_or("unknown").to_string();
        if let Some(mut acc) = self.account_store.get_account(&format!("user_{}", id)) {
            acc.change_password(old, new)?;
            Ok(())
        } else { anyhow::bail!("user not found") }
    }
    pub fn reset_password(&mut self, email: &str, new_password: &str) -> Result<()> {
        self.account_store.reset_password(email, new_password)
    }
}

impl Default for AuthManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_auth_manager_new() { let _a = AuthManager::new(); }
}
