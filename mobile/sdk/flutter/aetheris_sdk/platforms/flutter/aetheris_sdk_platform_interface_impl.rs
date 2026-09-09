//! Rust implementation of the Aetheris SDK platform interface.
//!
//! Implements the Rust backend for the Flutter SDK.

use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde_json::json;
use parking_lot::Mutex as ParkingLotMutex;
use crate::vault::store::VaultStore;
use crate::crypto::CryptoEngine;
use crate::vault::item::PasswordItem;
use crate::web::client::WebClient;

/// Rust implementation of the Aetheris SDK platform interface.
pub struct AetherisSdkPlatformImpl {
    vault_store: Arc<ParkingLotMutex<VaultStore>>,
    web_client: WebClient,
}

impl AetherisSdkPlatformImpl {
    /// Create a new Aetheris SDK platform implementation.
    pub fn new(vault_path: String) -> Result<Self> {
        let vault_store = VaultStore::new(&vault_path)?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let web_client = WebClient::new(vault_store, "extension_id".to_string(), "extension_url".to_string());
        
        Ok(AetherisSdkPlatformImpl {
            vault_store: Arc::new(ParkingLotMutex::new(vault_store)),
            web_client,
        })
    }

    /// Initialize the Aetheris SDK.
    pub async fn initialize(&self, vault_path: String) -> Result<()> {
        let mut vault_store = self.vault_store.lock();
        vault_store.initialize(vault_path)?;
        Ok(())
    }

    /// Insert a vault item.
    pub async fn insert_item(&self, title: String, password: String, notes: String) -> Result<Map<String, dynamic>> {
        let password_item = PasswordItem {
            id: Uuid::new_v4(),
            title,
            password,
            notes,
        };
        
        let inserted_id = self.web_client.insert_item(password_item).await?;
        Ok(json!({"id": inserted_id.to_string()}))
    }

    /// Get a vault item by ID.
    pub async fn get_item(&self, item_id: String) -> Result<Map<String, dynamic>?> {
        let item = self.web_client.get_item(Uuid::parse_str(&item_id)?).await?;
        if let Some(item) = item {
            Ok(json!({
                "title": item.title,
                "password": item.password,
                "notes": item.notes,
            }))
        } else {
            Ok(null)
        }
    }

    /// Update a vault item.
    pub async fn update_item(&self, item_id: String, item: Map<String, dynamic>) -> Result<()> {
        let item = PasswordItem {
            id: Uuid::parse_str(&item_id)?,
            title: item["title"].to_string(),
            password: item["password"].to_string(),
            notes: item["notes"].to_string(),
        };
        
        self.web_client.update_item(item.id, item).await?;
        Ok(())
    }

    /// Delete a vault item by ID.
    pub async fn delete_item(&self, item_id: String) -> Result<()> {
        self.web_client.delete_item(Uuid::parse_str(&item_id)?).await?;
        Ok(())
    }

    /// Sync data with the Aetheris vault.
    pub async fn sync_data(&self) -> Result<()> {
        self.web_client.sync_with_extension().await?;
        Ok(())
    }
}