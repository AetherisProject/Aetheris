//! Sync module — provides B2 and local sync clients.

pub mod b2;

pub use b2::B2SyncClient;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::error::AetherisError;

/// Legacy in-memory sync client (stub — use B2SyncClient for real sync).
pub struct SyncClient {
    sync_data: Arc<Mutex<HashMap<String, String>>>,
}

impl SyncClient {
    pub fn new() -> Self {
        Self {
            sync_data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn sync(&self, sync_id: &str) -> Result<(), AetherisError> {
        let mut data = self.sync_data.lock().map_err(|e| AetherisError::SyncError(e.to_string()))?;
        data.insert(sync_id.to_string(), "synced".to_string());
        Ok(())
    }

    pub fn get_sync_state(&self, sync_id: &str) -> Result<String, AetherisError> {
        let data = self.sync_data.lock().map_err(|e| AetherisError::SyncError(e.to_string()))?;
        data.get(sync_id).cloned().ok_or_else(|| AetherisError::SyncError("Sync state not found".into()))
    }

    pub fn update_sync_state(&self, sync_id: &str, state: &str) -> Result<(), AetherisError> {
        let mut data = self.sync_data.lock().map_err(|e| AetherisError::SyncError(e.to_string()))?;
        data.insert(sync_id.to_string(), state.to_string());
        Ok(())
    }
}

impl Default for SyncClient {
    fn default() -> Self {
        Self::new()
    }
}
