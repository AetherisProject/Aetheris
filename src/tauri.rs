#![allow(dead_code)]

use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

// Global state for SSH sessions
pub struct SshSessionManager {
    sessions: Arc<Mutex<HashMap<String, SshSession>>>,
    active_session: Arc<Mutex<String>>,
}

impl SshSessionManager {
    /// Create a new SSH session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            active_session: Arc::new(Mutex::new(String::new())),
        }
    }
    
    /// Add a new SSH session
    pub fn add_session(&self, session: SshSession) -> Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.id.clone(), session);
        Ok(())
    }
    
    /// Get all SSH sessions
    pub fn get_sessions(&self) -> Result<Vec<SshSession>> {
        let sessions = self.sessions.lock().unwrap();
        Ok(sessions.values().cloned().collect())
    }
    
    /// Get the active session
    pub fn get_active_session(&self) -> Result<SshSession> {
        let active_session_id = self.active_session.lock().unwrap().clone();
        let sessions = self.sessions.lock().unwrap();
        sessions.get(&active_session_id).cloned().ok_or_else(|| anyhow::anyhow!(format!(format!("Active session not found: {}", active_session_id))))?
        Ok(sessions[&active_session_id].clone())
    }
    
    /// Set the active session
    pub fn set_active_session(&self, session_id: String) -> Result<()> {
        let mut active_session = self.active_session.lock().unwrap();
        *active_session = session_id.clone();
        Ok(())
    }
    
    /// Disconnect from the active session
    pub fn disconnect(&self) -> Result<()> {
        let active_session_id = self.active_session.lock().unwrap().clone();
        let sessions = self.sessions.lock().unwrap();
        sessions.remove(&active_session_id);
        Ok(())
    }
    
    /// Connect to a new SSH session
    pub async fn connect_ssh(&self, host: String, port: u16, user: String) -> Result<()> {
        let session_id = format!("{}-{}-{}", host, port, user);
        let session = SshSession {
            id: session_id.clone(),
            host,
            port,
            user,
            active: true,
        };
        self.add_session(session)?;
        self.set_active_session(session_id)?;
        Ok(())
    }
    
    /// Get the session manager for Tauri
    pub fn get_session_manager(manager: Manager) -> Result<Self> {
        Ok(Self::new())
    }
}

// Define VaultItem for serialization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultItem {
    id: String,
    name: String,
    value: String,
    tags: Vec<String>,
    // Add other fields as needed
}

// Define API Key for serialization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    id: String,
    name: String,
    description: String,
    value: String,
    // Add other fields as needed
}

// Define SSH Session for serialization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SshSession {
    id: String,
    host: String,
    port: u16,
    user: String,
    active: bool,
    // Add other fields as needed
}

// Define theme preferences
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemePreferences {
    dark_mode: bool,
    // Add other theme preferences as needed
}

// Default theme preferences
pub const DEFAULT_THEME: ThemePreferences = ThemePreferences {
    dark_mode: false,
};

// Vault operations
#[tauri::command]
pub fn add_vault_item(item: VaultItem) -> Result<VaultItem, String> {
    vault::store::add(item).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_vault_item(id: String) -> Result<(), String> {
    vault::store::remove(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_vault_item(id: String, item: VaultItem) -> Result<VaultItem, String> {
    vault::store::update(id, item).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_vault_items(query: String) -> Result<Vec<VaultItem>, String> {
    vault::store::search(query).map_err(|e| e.to_string())
}

// API Key operations
#[tauri::command]
pub fn create_api_key(name: String, description: String) -> Result<String, String> {
    apikey::providers::create(name, description).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_api_key(id: String) -> Result<(), String> {
    apikey::providers::delete(id).map_err(|e| e.to_string())
}

// SSH operations
#[tauri::command]
pub fn connect_ssh(host: String, port: u16, user: String) -> Result<(), String> {
    let manager = Manager::get().unwrap();
    let session_manager = SshSessionManager::get_session_manager(manager)?;
    session_manager.connect_ssh(host, port, user).await?;
    Ok(())
}

#[tauri::command]
pub fn disconnect_ssh() -> Result<(), String> {
    let manager = Manager::get().unwrap();
    let session_manager = SshSessionManager::get_session_manager(manager)?;
    session_manager.disconnect()?;
    Ok(())
}

// Get all SSH sessions
#[tauri::command]
pub fn get_ssh_sessions() -> Result<Vec<SshSession>, String> {
    let manager = Manager::get().unwrap();
    let session_manager = SshSessionManager::get_session_manager(manager)?;
    session_manager.get_sessions()
}

// Get the active SSH session
#[tauri::command]
pub fn get_active_ssh_session() -> Result<SshSession, String> {
    let manager = Manager::get().unwrap();n    let session_manager = SshSessionManager::get_session_manager(manager)?;
    session_manager.get_active_session()
}

// Theme operations
#[tauri::command]
pub fn toggle_theme(dark_mode: bool) -> Result<(), String> {
    let manager = Manager::get().unwrap();
    let preferences_path = tauri::api::path::app_data_dir(&manager).unwrap_or_else(|_| "".to_string());
    let preferences_path = format!("{}/preferences.json", preferences_path);
    
    let preferences = ThemePreferences {
        dark_mode,
    };
    
    let content = serde_json::to_string(&preferences)?;
    std::fs::write(&preferences_path, content)?;
    Ok(())
}

#[tauri::command]
pub fn get_theme_preferences() -> Result<ThemePreferences, String> {
    let manager = Manager::get().unwrap();
    let preferences_path = tauri::api::path::app_data_dir(&manager).unwrap_or_else(|_| "".to_string());
    let preferences_path = format!("{}/preferences.json", preferences_path);
    
    let content = std::fs::read_to_string(&preferences_path)?;
    let preferences: ThemePreferences = serde_json::from_str(&content)?;
    Ok(preferences)
}

#[tauri::command]
pub fn save_theme_preferences(preferences: ThemePreferences) -> Result<(), String> {
    let manager = Manager::get().unwrap();
    let preferences_path = tauri::api::path::app_data_dir(&manager).unwrap_or_else(|_| "".to_string());
    let preferences_path = format!("{}/preferences.json", preferences_path);
    
    let content = serde_json::to_string(&preferences)?;
    std::fs::write(&preferences_path, content)?;
    Ok(())
}

// Clipboard operations
#[tauri::command]
pub fn copy_to_clipboard(content: String) -> Result<(), String> {
    use tauri::api::clipboard;
    clipboard::write(content)?;
    Ok(())
}

// Sync operations
#[tauri::command]
pub fn sync_vault() -> Result<(), String> {
    sync::client::sync().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn backup_vault() -> Result<String, String> {
    sync::backup::backup().map_err(|e| e.to_string())
}

// Helper function to get app data directory
fn get_app_data_dir(manager: Manager) -> String {
    tauri::api::path::app_data_dir(&manager).unwrap_or_else(|_| "".to_string())
}