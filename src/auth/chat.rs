use crate::crypto::{CryptoEngine, Zeroize};
use anyhow::{Context, Result};
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// A single chat message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,      // "user" or "assistant"
    pub content: String,
    pub timestamp: u64,
    pub model: String,     // Model used for this message
}

/// A conversation with multiple messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub user_id: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Chat history store for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistory {
    pub conversations: HashMap<String, Conversation>,
}

/// Chat session for tracking active conversations
#[derive(Debug, Clone)]
pub struct ChatSession {
    pub user_id: String,
    pub active_conversation_id: Option<String>,
    pub rate_limit: RateLimiter,
}

/// Rate limiter for chat requests
#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub requests: Vec<u64>,  // Timestamps of recent requests
    pub max_requests: usize,
    pub window_seconds: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            requests: Vec::new(),
            max_requests,
            window_seconds,
        }
    }
    
    /// Check if a request is allowed
    pub fn allow(&mut self, current_time: u64) -> bool {
        // Remove old requests outside the window
        self.requests.retain(|&ts| current_time.saturating_sub(ts) <= self.window_seconds);
        
        if self.requests.len() >= self.max_requests {
            return false;
        }
        
        self.requests.push(current_time);
        true
    }
    
    /// Get remaining requests
    pub fn remaining(&self, current_time: u64) -> usize {
        let count = self.requests.iter()
            .filter(|&&ts| current_time.saturating_sub(ts) <= self.window_seconds)
            .count();
        self.max_requests.saturating_sub(count)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(100, 60)  // 100 requests per minute by default
    }
}

/// Chat manager for handling chat sessions and history
pub struct ChatManager {
    chat_store: Arc<Mutex<HashMap<String, ChatHistory>>>,
    sessions: Arc<Mutex<HashMap<String, ChatSession>>>,
    crypto_engine: CryptoEngine,
}

impl ChatManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            chat_store: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            crypto_engine: CryptoEngine::new()?,
        })
    }
    
    /// Create a new conversation for a user
    pub fn create_conversation(&self, user_id: &str) -> Result<String> {
        let mut store = self.chat_store.lock();
        let history = store.entry(user_id.to_string())
            .or_insert_with(|| ChatHistory {
                conversations: HashMap::new()
            });
        
        let conversation_id = format!("conv_{}", uuid::Uuid::new_v4());
        let conversation = Conversation {
            id: conversation_id.clone(),
            user_id: user_id.to_string(),
            messages: Vec::new(),
            created_at: self.current_timestamp(),
            updated_at: self.current_timestamp(),
        };
        
        history.conversations.insert(conversation_id.clone(), conversation);
        Ok(conversation_id)
    }
    
    /// Add a message to a conversation
    pub fn add_message(
        &self,
        user_id: &str,
        conversation_id: &str,
        role: String,
        content: String,
        model: String,
    ) -> Result<()> {
        let mut store = self.chat_store.lock();
        if let Some(history) = store.get_mut(user_id) {
            if let Some(conversation) = history.conversations.get_mut(conversation_id) {
                conversation.messages.push(ChatMessage {
                    role,
                    content,
                    timestamp: self.current_timestamp(),
                    model,
                });
                conversation.updated_at = self.current_timestamp();
                Ok(())
            } else {
                anyhow::bail!("Conversation not found");
            }
        } else {
            anyhow::bail!("User history not found");
        }
    }
    
    /// Get conversation messages
    pub fn get_conversation(
        &self,
        user_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ChatMessage>> {
        let store = self.chat_store.lock();
        if let Some(history) = store.get(user_id) {
            if let Some(conversation) = history.conversations.get(conversation_id) {
                Ok(conversation.messages.clone())
            } else {
                anyhow::bail!("Conversation not found");
            }
        } else {
            anyhow::bail!("User history not found");
        }
    }
    
    /// List all conversations for a user
    pub fn list_conversations(&self, user_id: &str) -> Result<Vec<Conversation>> {
        let store = self.chat_store.lock();
        if let Some(history) = store.get(user_id) {
            Ok(history.conversations.values().cloned().collect())
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Create or get a chat session for a user
    pub fn get_session(&self, user_id: &str) -> Result<ChatSession> {
        let mut sessions = self.sessions.lock();
        if let Some(session) = sessions.get(user_id) {
            Ok(session.clone())
        } else {
            let session = ChatSession {
                user_id: user_id.to_string(),
                active_conversation_id: None,
                rate_limit: RateLimiter::default(),
            };
            sessions.insert(user_id.to_string(), session.clone());
            Ok(session)
        }
    }
    
    /// Set active conversation for a session
    pub fn set_active_conversation(
        &self,
        user_id: &str,
        conversation_id: Option<String>,
    ) -> Result<()> {
        let mut sessions = self.sessions.lock();
        if let Some(session) = sessions.get_mut(user_id) {
            session.active_conversation_id = conversation_id;
            Ok(())
        } else {
            anyhow::bail!("Session not found");
        }
    }
    
    /// Check rate limit for a user
    pub fn check_rate_limit(&self, user_id: &str) -> Result<bool> {
        let mut sessions = self.sessions.lock();
        if let Some(session) = sessions.get_mut(user_id) {
            Ok(session.rate_limit.allow(self.current_timestamp()))
        } else {
            // Create a new session if one doesn't exist
            let session = ChatSession {
                user_id: user_id.to_string(),
                active_conversation_id: None,
                rate_limit: RateLimiter::default(),
            };
            let allowed = session.rate_limit.allow(self.current_timestamp());
            sessions.insert(user_id.to_string(), session);
            Ok(allowed)
        }
    }
    
    /// Get current timestamp
    fn current_timestamp(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl Default for ChatManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ChatManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(5, 10);
        let now = 1000u64;
        
        // Should allow 5 requests
        for _ in 0..5 {
            assert!(limiter.allow(now));
        }
        
        // 6th request should be denied
        assert!(!limiter.allow(now));
        
        // After window passes, should allow again
        assert!(limiter.allow(now + 11));
    }
    
    #[test]
    fn test_chat_manager_create_conversation() {
        let manager = ChatManager::new().unwrap();
        let conversation_id = manager.create_conversation("user_1").unwrap();
        assert!(!conversation_id.is_empty());
    }
    
    #[test]
    fn test_chat_manager_add_message() {
        let manager = ChatManager::new().unwrap();
        let conversation_id = manager.create_conversation("user_1").unwrap();
        
        manager.add_message(
            "user_1",
            &conversation_id,
            "user".to_string(),
            "Hello".to_string(),
            "puter/claude-fable-5.1".to_string(),
        ).unwrap();
        
        let messages = manager.get_conversation("user_1", &conversation_id).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello");
    }
}
