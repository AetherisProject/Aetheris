//! Zero-Knowledge Sync client for Aetheris.
//!
//! Implements CRDT-based sync for secure, conflict-free data sharing.

use anyhow::{Context, Result};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::vault::item::VaultItem;
use crate::crypto::EncryptionKey;
use crate::sync::crdt::CrdtNode;

/// Sync client for managing zero-knowledge sync operations.
pub struct SyncClient {
    nodes: HashMap<Uuid, CrdtNode<VaultItem>>,
    master_key: EncryptionKey,
}

impl SyncClient {
    /// Create a new sync client.
    pub fn new(master_key: EncryptionKey) -> Self {
        SyncClient {
            nodes: HashMap::new(),
            master_key,
        }
    }

    /// Add a new sync node.
    pub fn add_node(&mut self, node_id: Uuid, node: CrdtNode<VaultItem>) -> Result<()> {
        self.nodes.insert(node_id, node);
        Ok(())
    }

    /// Sync data between nodes.
    pub fn sync(&mut self, node_id: Uuid) -> Result<()> {
        let node = self.nodes.get_mut(&node_id).ok_or_else(|| anyhow::anyhow!("Node not found"))?;
        
        // Merge changes from other nodes
        for other_node in self.nodes.values() {
            if other_node.id() != node_id {
                node.merge(other_node)?;
            }
        }
        
        Ok(())
    }

    /// Get the latest state of a vault item.
    pub fn get_latest_item(&self, node_id: Uuid, item_id: Uuid) -> Result<Option<VaultItem>> {
        let node = self.nodes.get(&node_id).ok_or_else(|| anyhow::anyhow!("Node not found"))?;
        node.get_latest_item(item_id)
    }

    /// Apply changes to a vault item.
    pub fn apply_changes(&mut self, node_id: Uuid, item_id: Uuid, changes: VaultItem) -> Result<()> {
        let node = self.nodes.get_mut(&node_id).ok_or_else(|| anyhow::anyhow!("Node not found"))?;
        node.apply_changes(item_id, changes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::CryptoEngine;
    use crate::vault::item::PasswordItem;
    
    #[test]
    fn test_sync_client() -> Result<()> {
        let master_key = CryptoEngine::new()?.generate_key()?;
        let mut client = SyncClient::new(master_key);
        
        // Add a node
        let node_id = Uuid::new_v4();
        let node = CrdtNode::new(node_id, PasswordItem {
            id: Uuid::new_v4(),
            title: "Test Password".to_string(),
            password: "test_password".to_string(),
            notes: "Test notes".to_string(),
        });
        
        client.add_node(node_id, node)?;
        
        // Sync the node
        client.sync(node_id)?;
        
        // Verify the node state
        let latest_item = client.get_latest_item(node_id, node.id())?;
        assert!(latest_item.is_some());
        
        Ok(())
    }
}