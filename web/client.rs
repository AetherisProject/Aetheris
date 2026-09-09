//! Web client for Aetheris.
//!
//! Implements a web client to interact with the Aetheris vault securely.

use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use parking_lot::Mutex as ParkingLotMutex;
use crate::vault::store::VaultStore;
use crate::crypto::CryptoEngine;
use crate::vault::item::VaultItem;

/// Web client for interacting with the Aetheris vault.
pub struct WebClient {
    vault_store: Arc<ParkingLotMutex<VaultStore>>,
    crypto_engine: CryptoEngine,
    master_key: Option<EncryptionKey>,
    
    // Web-specific configuration
    extension_id: String,
    extension_url: String,
}

impl WebClient {
    /// Create a new web client.
    pub fn new(vault_store: VaultStore, extension_id: String, extension_url: String) -> Self {
        let vault_store = Arc::new(ParkingLotMutex::new(vault_store));
        let crypto_engine = CryptoEngine::new().unwrap();
        let master_key = vault_store.lock().unwrap().get_master_key().cloned().unwrap();
        
        WebClient {
            vault_store,
            crypto_engine,
            master_key,
            extension_id,
            extension_url,
        }
    }

    /// Insert a vault item.
    pub async fn insert_item(&self, item: VaultItem) -> Result<Uuid> {
        let mut vault_store = self.vault_store.lock();
        vault_store.insert(item)?;
        Ok(item.id())
    }

    /// Get a vault item by ID.
    pub async fn get_item(&self, id: Uuid) -> Result<Option<VaultItem>> {
        let mut vault_store = self.vault_store.lock();
        vault_store.get(&id)
    }

    /// Update a vault item.
    pub async fn update_item(&self, id: Uuid, item: VaultItem) -> Result<()> {
        let mut vault_store = self.vault_store.lock();
        vault_store.update(item)?;
        Ok(())
    }

    /// Delete a vault item by ID.
    pub async fn delete_item(&self, id: Uuid) -> Result<()> {
        let mut vault_store = self.vault_store.lock();
        vault_store.delete(&id)?;
        Ok(())
    }

    /// Sync data with the web extension.
    pub async fn sync_with_extension(&self) -> Result<()> {
        // In a real implementation, this would communicate with the web extension
        // via browser APIs or WebSocket.
        let mut vault_store = self.vault_store.lock();
        vault_store.sync_data(Uuid::new_v4())?;
        Ok(())
    }

    /// Get the latest item from the vault.
    pub async fn get_latest_item(&self, node_id: Uuid, item_id: Uuid) -> Result<Option<VaultItem>> {
        let mut vault_store = self.vault_store.lock();
        vault_store.get_latest_item(node_id, item_id)
    }

    /// Check if the web extension is available.
    pub fn is_extension_available(&self) -> bool {
        // In a real implementation, this would check if the extension is installed
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::item::PasswordItem;
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_web_client() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_web")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let web_client = WebClient::new(vault_store, "extension_id".to_string(), "extension_url".to_string());
        
        // Insert a password item
        let password_item = PasswordItem {
            id: Uuid::new_v4(),
            title: "Test Password".to_string(),
            password: "test_password".to_string(),
            notes: "Test notes".to_string(),
        };
        
        let inserted_id = web_client.insert_item(password_item).await?;
        
        // Get the item
        let item = web_client.get_item(inserted_id).await?;
        assert!(item.is_some());
        
        Ok(())
    }
}