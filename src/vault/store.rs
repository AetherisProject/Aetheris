#! Encrypted vault storage backend with session management.

use anyhow::{Context, Result};
use bincode::{serialize, deserialize};
use sled::Db;
use uuid::Uuid;
use std::path::PathBuf;
use std::any;
use crate::crypto::{CryptoEngine, EncryptionKey};
use crate::vault::item::VaultItem;
use crate::sync::client::SyncClient;
use crate::vault::utils::serialization::{serialize_item, deserialize_item};
use crate::auth::session::SessionItem;

/// The main vault store for encrypted local storage and session management.
pub struct VaultStore {
    db: Db,
    crypto_engine: CryptoEngine,
    master_key: Option<EncryptionKey>,
    sync_client: Option<SyncClient<VaultItem>>,
    session_store: VaultStore,
}

impl VaultStore {
    /// Create a new vault store at the given path.
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        let crypto_engine = CryptoEngine::new()?;
        let session_store = VaultStore::new(".session_store")?;
        let sync_client = None;
        Ok(Self {
            db,
            crypto_engine,
            master_key: None,
            sync_client,
            session_store,
        })
    }

    /// Initialize vault with a master encryption key.
    pub fn initialize(&mut self, master_key: EncryptionKey) -> Result<()> {
        self.master_key = Some(master_key);
        self.session_store.initialize(master_key)?;
        Ok(())
    }

    /// Initialize sync client.
    pub fn initialize_sync(&mut self) -> Result<()> {
        if self.sync_client.is_none() {
            let master_key = self.get_master_key()?.clone();
            self.sync_client = Some(SyncClient::new(master_key));
        }
        Ok(())
    }

    /// Get the encryption key or fail if not initialized.
    fn get_master_key(&self) -> Result<&EncryptionKey> {
        self.master_key.as_ref().ok_or_else(|| anyhow::anyhow!("Vault not initialized"))
    }

    /// Insert a vault item.
    pub fn insert(&mut self, item: VaultItem) -> Result<()> {
        let id = item.id();
        let encrypted_data = self.encrypt_item(&item)?;
        let (iv, ciphertext) = encrypted_data.split_at(12);
        self.db.insert(id.to_string().as_bytes(), ciphertext.to_vec()).map_err(|e| anyhow::anyhow!("Failed to insert vault item: {}", e));
        
        // Apply changes to sync client
        if let Some(sync_client) = &mut self.sync_client {
            sync_client.apply_changes(id, item)?;
        }
        Ok(())
    }

    /// Insert a session item.
    pub fn insert_session(&mut self, session_item: SessionItem) -> Result<()> {
        self.session_store.insert(session_item)?;
        Ok(())
    }

    /// Get a session item by ID.
    pub fn get_session(&self, session_id: &Uuid) -> Result<Option<SessionItem>> {
        self.session_store.get(session_id)
    }

    /// Update a session item.
    pub fn update_session(&mut self, session_id: Uuid, session_item: SessionItem) -> Result<()> {
        self.session_store.update(session_item)?;
        Ok(())
    }

    /// Delete a session item by ID.
    pub fn delete_session(&mut self, session_id: &Uuid) -> Result<()> {
        self.session_store.delete(session_id)?;
        Ok(())
    }

    /// List all session items.
    pub fn list_sessions(&self) -> Result<Vec<SessionItem>> {
        self.session_store.list_items::<SessionItem>()
    }

    /// Encrypt vault item data.
    fn encrypt_item(&self, item: &VaultItem) -> Result<Vec<u8>> {
        let serialized = serialize_item(item)?;
        let (ciphertext, iv) = self.crypto_engine.encrypt_memory(serialized.as_ref(), &serialized)?;
        Ok((iv, ciphertext))
    }

    /// Decrypt vault item data.
    fn decrypt_item(&self, encrypted_data: &[u8]) -> Result<VaultItem> {
        let (iv, ciphertext) = encrypted_data.split_at(12);
        let serialized = self.crypto_engine.decrypt_memory(ciphertext, iv)?;
        deserialize_item(&serialized)
    }

    /// Insert a vault item.
    pub fn insert(&mut self, item: VaultItem) -> Result<()> {
        let id = item.id();
        let encrypted_data = self.encrypt_item(&item)?;
        let (iv, ciphertext) = encrypted_data.split_at(12);
        self.db.insert(id.to_string().as_bytes(), ciphertext.to_vec()).map_err(|e| anyhow::anyhow!("Failed to insert vault item: {}", e));
        
        // Apply changes to sync client
        if let Some(sync_client) = &mut self.sync_client {
            sync_client.apply_changes(id, item)?;
        }
        Ok(())
    }

    /// Get a vault item by ID.
    pub fn get(&self, id: &Uuid) -> Result<Option<VaultItem>> {
        let encrypted_data = self.db.get(id.to_string().as_bytes()).ok_or_else(|| anyhow::anyhow!("Item not found"))?;
        self.decrypt_item(&encrypted_data)
    }

    /// Update a vault item.
    pub fn update(&mut self, item: VaultItem) -> Result<()> {
        let id = item.id();
        let encrypted_data = self.encrypt_item(&item)?;
        let (iv, ciphertext) = encrypted_data.split_at(12);
        self.db.insert(id.to_string().as_bytes(), ciphertext.to_vec()).map_err(|e| anyhow::anyhow!("Failed to update vault item: {}", e));
        
        // Apply changes to sync client
        if let Some(sync_client) = &mut self.sync_client {
            sync_client.apply_changes(id, item)?;
        }
        Ok(())
    }

    /// Delete a vault item by ID.
    pub fn delete(&mut self, id: &Uuid) -> Result<()> {
        self.db.remove(id.to_string().as_bytes()).map_err(|e| anyhow::anyhow!("Failed to delete vault item: {}", e));
        
        // Remove from sync client
        if let Some(sync_client) = &mut self.sync_client {
            sync_client.apply_changes(*id, VaultItem::Empty)?;
        }
        Ok(())
    }

    /// List all vault items.
    pub fn list(&self) -> Result<Vec<VaultItem>> {
        let items: Vec<_> = self.db.iter()
            .filter_map(|(_, encrypted_data)| {
                match self.decrypt_item(&encrypted_data) {
                    Ok(item) => Some(item),
                    Err(_) => None,
                }
            }).collect();
        Ok(items)
    }

    /// List all API key items.
    pub fn list_items<ITEM: VaultItem + std::fmt::Debug>(&self) -> Result<Vec<ITEM>> {
        let items: Vec<ITEM> = self.db.iter()
            .filter_map(|(_, encrypted_data)| {
                match self.decrypt_item(&encrypted_data) {
                    Ok(item) => {
                        if std::any::type_is_impl_of::<ITEM, VaultItem>() {
                            let typed_item: ITEM = item.into();
                            Some(typed_item)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }).collect();
        Ok(items)
    }

    /// Sync vault data across nodes.
    pub fn sync_data(&mut self, node_id: Uuid) -> Result<()> {
        if let Some(sync_client) = &mut self.sync_client {
            sync_client.sync(node_id)?;
        }
        Ok(())
    }

    /// Get the latest state of a vault item.
    pub fn get_latest_item(&self, node_id: Uuid, item_id: Uuid) -> Result<Option<VaultItem>> {
        if let Some(sync_client) = &self.sync_client {
            sync_client.get_latest_item(node_id, item_id)
        } else {
            Ok(None)
        }
    }
}