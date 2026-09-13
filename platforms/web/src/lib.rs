#![allow(dead_code)]
use wasm_bindgen::prelude::*;
use aetheris_core::{vault::VaultItem, crypto::CryptoEngine};
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct AetherisWeb {
    engine: CryptoEngine,
}

#[wasm_bindgen]
impl AetherisWeb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: CryptoEngine::new().expect("Failed to initialize CryptoEngine"),
        }
    }
    
    #[wasm_bindgen]
    pub fn generate_keypair(&self) -> Result<Vec<u8>, JsValue> {
        let (pk, _) = self.engine.generate_kyber_keypair()?;
        Ok(pk)
    }
    
    #[wasm_bindgen]
    pub fn create_vault_item(&self, name: String, data: &[u8]) -> Result<JsValue, JsValue> {
        let item = VaultItem::new(name, data.to_vec(), vec![]);
        Ok(serde_wasm_bindgen::to_value(&item).unwrap())
    }
}

// Chat-related structures and functions
#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    model: String,
    object: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    finish_reason: Option<String>,
    index: usize,
    text: String,
}

#[wasm_bindgen]
pub struct ChatClient {
    base_url: String,
}

#[wasm_bindgen]
impl ChatClient {
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    #[wasm_bindgen]
    pub async fn list_models(&self) -> Result<JsValue, JsValue> {
        let url = format!("{}/models", self.base_url);
        let response = reqwest::blocking::get(&url)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let models: serde_json::Value = response.json()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(serde_wasm_bindgen::to_value(&models).unwrap())
    }
    
    #[wasm_bindgen]
    pub async fn send_chat(&self, model: String, prompt: String) -> Result<JsValue, JsValue> {
        let url = format!("{}/chat/completions", self.base_url);
        let client = reqwest::blocking::Client::new();
        
        let request = ChatRequest {
            model,
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
        };
        
        let response = client.post(&url)
            .json(&request)
            .send()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let chat_response: ChatResponse = response.json()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let result = chat_response.choices
            .into_iter()
            .next()
            .map(|c| c.text)
            .ok_or_else(|| JsValue::from_str("No response text"))?;
        
        Ok(JsValue::from_str(&result))
    }
}
