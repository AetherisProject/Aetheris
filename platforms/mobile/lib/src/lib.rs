// Aetheris Mobile - Rust FFI for Flutter
// Provides chat functionality for Puter models

use serde::{Serialize, Deserialize};

// Re-export core types for FFI
pub use aetheris_core::{vault::VaultItem, crypto::CryptoEngine, ssh::SshClient};

// Chat structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    pub model: String,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub finish_reason: Option<String>,
    pub index: usize,
    pub text: String,
}

/// Chat client for mobile FFI
pub struct ChatClient {
    base_url: String,
    client: reqwest::Client,
}

impl ChatClient {
    /// Create a new chat client
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
    
    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>, String> {
        let url = format!("{}/models", self.base_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let models: serde_json::Value = response.json()
            .await
            .map_err(|e| e.to_string())?;
        
        models["data"]
            .as_array()
            .map(|arr| arr.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect())
            .ok_or_else(|| "Invalid response format".to_string())
    }
    
    /// Send a chat message and get response
    pub async fn send_chat(&self, model: String, prompt: String) -> Result<String, String> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let request = ChatRequest {
            model,
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
        };
        
        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let chat_response: ChatResponse = response.json()
            .await
            .map_err(|e| e.to_string())?;
        
        chat_response.choices
            .into_iter()
            .next()
            .map(|c| c.text)
            .ok_or_else(|| "No response text".to_string())
    }
}

// Simple FFI-safe wrapper functions
#[no_mangle]
pub extern "C" fn create_chat_client(base_url: *const u8, base_url_len: usize) -> *mut ChatClient {
    let base_url_str = unsafe { 
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(base_url, base_url_len))
    };
    Box::into_raw(Box::new(ChatClient::new(base_url_str.to_string())))
}

#[no_mangle]
pub extern "C" fn free_chat_client(client: *mut ChatClient) {
    unsafe { let _ = Box::from_raw(client); }
}
