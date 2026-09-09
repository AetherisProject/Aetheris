//! CRDT (Conflict-Free Replicated Data Types) implementation for Aetheris.
//!
//! Implements CRDT-based data structures for conflict-free sync.

use anyhow::{Context, Result};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// CRDT node for managing replicated data.
pub struct CrdtNode<T> {
    id: Uuid,
    data: HashMap<Uuid, T>,
    version: AtomicBool,
    last_updated: AtomicBool,
    
    // Track changes
    changes: HashMap<Uuid, T>,
    
    // Lock for thread safety
    lock: std::sync::Mutex<()>, 
}

impl<T> CrdtNode<T> {
    /// Create a new CRDT node.
    pub fn new(id: Uuid, initial_data: T) -> Self {
        CrdtNode {
            id,
            data: HashMap::new(),
            version: AtomicBool::new(false),
            last_updated: AtomicBool::new(false),
            changes: HashMap::new(),
            lock: std::sync::Mutex::new(()),
        }
    }

    /// Get the node ID.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Add or update data.
    pub fn add_item(&self, item_id: Uuid, item: T) -> Result<()> {
        let mut lock = self.lock.lock()?;
        self.data.insert(item_id, item);
        self.last_updated.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Merge changes from another node.
    pub fn merge(&mut self, other: &CrdtNode<T>) -> Result<()> {
        let mut lock = self.lock.lock()?;
        
        // Merge data from other node
        for (item_id, item) in other.data.iter() {
            self.data.insert(item_id.clone(), item.clone());
        }
        
        // Merge changes
        for (item_id, item) in other.changes.iter() {
            self.changes.insert(item_id.clone(), item.clone());
        }
        
        self.version.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Apply changes to the node.
    pub fn apply_changes(&mut self, item_id: Uuid, changes: T) -> Result<()> {
        let mut lock = self.lock.lock()?;
        self.changes.insert(item_id, changes);
        Ok(())
    }

    /// Get the latest item.
    pub fn get_latest_item(&self, item_id: Uuid) -> Result<Option<T>> {
        let mut lock = self.lock.lock()?;
        self.data.get(&item_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::item::PasswordItem;
    
    #[test]
    fn test_crdt_node() -> Result<()> {
        let node = CrdtNode::new(Uuid::new_v4(), PasswordItem {
            id: Uuid::new_v4(),
            title: "Test Password".to_string(),
            password: "test_password".to_string(),
            notes: "Test notes".to_string(),
        });
        
        // Add an item
        let item_id = Uuid::new_v4();
        node.add_item(item_id, PasswordItem {
            id: item_id,
            title: "Updated Password".to_string(),
            password: "updated_password".to_string(),
            notes: "Updated notes".to_string(),
        })?;
        
        // Get the latest item
        let latest_item = node.get_latest_item(item_id)?;
        assert!(latest_item.is_some());
        assert_eq!(latest_item.unwrap().title, "Updated Password");
        
        Ok(())
    }
}