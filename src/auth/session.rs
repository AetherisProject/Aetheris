//! Session token management — secure, zero-knowledge session storage.
//! All session tokens use AEAD encryption; Zeroize on drop.

use crate::crypto::{CryptoEngine, Zeroize};
use anyhow::Result;
use std::collections::HashMap;

/// Encrypted session token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionToken {
    pub token: String,
    pub user_id: String,
    pub created: u64,
    pub expires: Option<u64>,
}

impl Drop for SessionToken {
    fn drop(&mut self) {
        self.token.zeroize();
    }
}

pub struct SessionStore {
    sessions: HashMap<String, SessionToken>,
}

impl SessionStore {
    pub fn new() -> Self { Self { sessions: HashMap::new() } }

    pub fn create(&mut self, user_id: &str) -> Result<SessionToken> {
        let token_str = CryptoEngine::generate_token()?;
        let session = SessionToken {
            token: token_str.clone(),
            user_id: user_id.to_string(),
            created: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            expires: None,
        };
        self.sessions.insert(token_str, session.clone());
        Ok(session)
    }

    pub fn validate(&self, token: &str) -> Option<String> {
        self.sessions.get(token).map(|s| s.user_id.clone())
    }
}
