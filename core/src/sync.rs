#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SyncClient {
    sync_data: Arc<Mutex<HashMap<String, String>>>, // Key: sync_id, Value: sync_state
}

impl SyncClient {
    pub fn new() -> Self {
        Self {
            sync_data: Arc::new(Mutex::new(HashMap::new()))
        }
    }
    
    pub fn sync(&self, sync_id: &str) -> Result<(), String> {
        let mut data = self.sync_data.lock().map_err(|e| e.to_string())?;
        data.insert(sync_id.to_string(), "synced".to_string());
        Ok(())
    }
    
    pub fn get_sync_state(&self, sync_id: &str) -> Result<String, String> {
        let data = self.sync_data.lock().map_err(|e| e.to_string())?;
        data.get(sync_id).cloned().ok_or_else(|| "Sync state not found".to_string())
    }
    
    pub fn update_sync_state(&self, sync_id: &str, state: &str) -> Result<(), String> {
        let mut data = self.sync_data.lock().map_err(|e| e.to_string())?;
        data.insert(sync_id.to_string(), state.to_string());
        Ok(())
    }
}