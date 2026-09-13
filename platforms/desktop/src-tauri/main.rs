#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use aetheris_core::{vault::VaultItem, crypto::CryptoEngine, ssh::SshClient};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::time::Duration;
use tauri::generate_handler;

#[tauri::command]
async fn add_vault_item(name: String, data: String) -> Result<VaultItem, String> {
    let item = VaultItem::new(name, data.into_bytes(), vec![]);
    Ok(item)
}

#[tauri::command]
async fn generate_keypair() -> Result<(Vec<u8>, Vec<u8>), String> {
    let engine = CryptoEngine::new()?;
    engine.generate_kyber_keypair()
}

#[tauri::command]
async fn connect_ssh(host: String, port: u16) -> Result<String, String> {
    let mut client = SshClient::new(host.clone(), port);
    client.connect().map_err(|e| e.to_string())?;
    Ok(format!("Connected to {}:{}", host, port))
}

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

#[tauri::command]
async fn list_models(base_url: String) -> Result<Vec<String>, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;
    
    let response = client
        .get(format!("{}/models", base_url))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let models: serde_json::Value = response
        .json()
        .await
        .map_err(|e| e.to_string())?;
    
    models["data"]
        .as_array()
        .map(|arr| arr.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect())
        .ok_or_else(|| "Invalid response format".to_string())
}

#[tauri::command]
async fn send_chat(base_url: String, model: String, prompt: String) -> Result<String, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;
    
    let request = ChatRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };
    
    let response = client
        .post(format!("{}/chat/completions", base_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let chat_response: ChatResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;
    
    chat_response.choices
        .into_iter()
        .next()
        .map(|c| c.text)
        .ok_or_else(|| "No response text".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            add_vault_item,
            generate_keypair,
            connect_ssh,
            list_models,
            send_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
