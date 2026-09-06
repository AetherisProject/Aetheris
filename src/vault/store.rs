//! Encrypted vault storage backend.

use anyhow::{Context, Result};
use bincode::{deserialize, serialize};
use sled::Db;
use uuid::Uuid;

use super::item::VaultItem;
use crate::crypto::{cipher, CryptoEngine, EncryptionKey};

/// The main vault store for encrypted local storage.
pub struct VaultStore {
    db: Db,
    crypto_engine: CryptoEngine,
    master_key: Option<EncryptionKey>,
}

impl VaultStore {
    /// Create a new vault store at the given path.
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)
            .with_context(|| format!("Failed to open sled database at: {}", path))?;
        
        Ok(Self {
            db,
            crypto_engine: CryptoEngine::new(),
            master_key: None,
        })
    }

    /// Initialize vault with a master encryption key.
    pub fn initialize(&mut self, master_key: EncryptionKey) -> Result<()> {
        self.master_key = Some(master_key);
        Ok(())
    }

    /// Get the encryption key or fail if not initialized.
    fn get_master_key(&self) -> Result<&EncryptionKey> {
        self.master_key
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vault not initialized with master key"))
    }

    /// Encrypt vault item data.
    fn encrypt_item(&self, item: &VaultItem) -> Result<Vec<u8>> {
        let master_key = self.get_master_key()?;
        let serialized = serialize(item)
            .map_err(|e| anyhow::anyhow!("Failed to serialize vault item: {}", e))?;
        
        let (ciphertext, nonce) = self.crypto_engine
            .encrypt_with_random_nonce(&serialized, master_key)?;
        
        // Combine nonce + ciphertext for storage
        let mut encrypted_data = nonce;
        encrypted_data.extend(ciphertext.into_bytes());
        
        Ok(encrypted_data)
    }

    /// Decrypt vault item data.
    fn decrypt_item(&self, encrypted_data: &[u8]) -> Result<VaultItem> {
        let master_key = self.get_master_key()?;
        
        if encrypted_data.len() < 24 { // nonce is 24 bytes
            return Err(anyhow::anyhow!("Encrypted data too short: expected at least 24 bytes for nonce"));
        }
        
        let (nonce, ciphertext_bytes) = encrypted_data.split_at(24);
        let ciphertext = cipher::Ciphertext::new(ciphertext_bytes.to_vec());
        
        let decrypted = self.crypto_engine
            .decrypt(&ciphertext, master_key, nonce)?;
        
        deserialize(&decrypted)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize vault item: {}", e))
    }

    /// Insert a vault item.
    pub fn insert(&mut self, item: VaultItem) -> Result<()> {
        let id = item.id();
        let encrypted_data = self.encrypt_item(&item)?;
        
        self.db.insert(
            id.to_string().as_bytes(),
            encrypted_data
        )
        .map_err(|e| anyhow::anyhow!("Failed to insert vault item: {}", e))?;
        
        Ok(())
    }

    /// Get a vault item by ID.
    pub fn get(&self, id: &Uuid) -> Result<Option<VaultItem>> {
        let encrypted_data = self.db.get(id.to_string().as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to get vault item: {}", e))?;
        
        match encrypted_data {
            Some(data) => {
                let item = self.decrypt_item(&data)?;
                Ok(Some(item))
            }
            None => Ok(None),
        }
    }

    /// Update a vault item.
    pub fn update(&mut self, item: VaultItem) -> Result<()> {
        self.insert(item)
    }

    /// Delete a vault item by ID.
    pub fn delete(&mut self, id: &Uuid) -> Result<()> {
        self.db.remove(id.to_string().as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to delete vault item: {}", e))?;
        
        Ok(())
    }

    /// List all vault items.
    pub fn list(&self) -> Result<Vec<VaultItem>> {
        let mut items = Vec::new();
        
        for result in self.db.iter() {
            let (_, encrypted_data) = result
                .map_err(|e| anyhow::anyhow!("Failed to iterate vault items: {}", e))?;
            
            let item = self.decrypt_item(&encrypted_data)?;
            items.push(item);
        }
        
        Ok(items)
    }

    /// Search vault items by query.
    pub fn search(&self, query: &str) -> Result<Vec<VaultItem>> {
        let all_items = self.list()?;
        let query_lower = query.to_lowercase();
        
        let filtered_items: Vec<VaultItem> = all_items.into_iter()
            .filter(|item| {
                let title = item.title().to_lowercase();
                title.contains(&query_lower)
            })
            .collect();
        
        Ok(filtered_items)
    }
}
