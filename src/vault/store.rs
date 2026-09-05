//! Encrypted vault storage backend.

use anyhow::Result;
use uuid::Uuid;

use super::item::VaultItem;

/// The main vault store for encrypted local storage.
pub struct VaultStore {
    // TODO: Add sled/sqlcipher backend
}

impl VaultStore {
    /// Create a new vault store at the given path.
    pub fn new(_path: &str) -> Result<Self> {
        Ok(Self {})
    }

    /// Insert a vault item.
    pub fn insert(&mut self, _item: VaultItem) -> Result<()> {
        unimplemented!("Insert not yet implemented")
    }

    /// Get a vault item by ID.
    pub fn get(&self, _id: &Uuid) -> Result<Option<VaultItem>> {
        unimplemented!("Get not yet implemented")
    }

    /// Update a vault item.
    pub fn update(&mut self, _item: VaultItem) -> Result<()> {
        unimplemented!("Update not yet implemented")
    }

    /// Delete a vault item by ID.
    pub fn delete(&mut self, _id: &Uuid) -> Result<()> {
        unimplemented!("Delete not yet implemented")
    }

    /// List all vault items.
    pub fn list(&self) -> Result<Vec<VaultItem>> {
        unimplemented!("List not yet implemented")
    }

    /// Search vault items by query.
    pub fn search(&self, _query: &str) -> Result<Vec<VaultItem>> {
        unimplemented!("Search not yet implemented")
    }
}
