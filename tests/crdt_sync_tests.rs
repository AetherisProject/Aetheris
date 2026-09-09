use anyhow::{Context, Result};
use uuid::Uuid;
use aetheris::vault::item::PasswordItem;
use aetheris::sync::client::SyncClient;
use aetheris::vault::store::VaultStore;
use aetheris::crypto::CryptoEngine;

#[tokio::test]
async fn test_crdt_node_operations() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_crdt")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    
    // Initialize sync client
    vault_store.initialize_sync()?;
    
    // Create a password item
    let password_item = PasswordItem {
        id: Uuid::new_v4(),
        title: "Test Password".to_string(),
        password: "test_password".to_string(),
        notes: "Test notes".to_string(),
    };
    
    vault_store.insert(password_item)?;
    
    // Sync data
    vault_store.sync_data(Uuid::new_v4())?;
    
    // Verify the item
    let latest_item = vault_store.get_latest_item(Uuid::new_v4(), password_item.id)?;
    assert!(latest_item.is_some());
    assert_eq!(latest_item.unwrap().title, "Test Password");
    
    Ok(())
}

#[tokio::test]
async fn test_sync_client_merge() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_sync_merge")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    
    // Create two nodes
    let node_id1 = Uuid::new_v4();
    let node_id2 = Uuid::new_v4();
    
    // Insert items into node 1
    let password_item1 = PasswordItem {
        id: Uuid::new_v4(),
        title: "Password 1".to_string(),
        password: "password1".to_string(),
        notes: "Notes for password 1".to_string(),
    };
    vault_store.insert(password_item1)?;
    
    // Insert items into node 2
    let password_item2 = PasswordItem {
        id: Uuid::new_v4(),
        title: "Password 2".to_string(),
        password: "password2".to_string(),
        notes: "Notes for password 2".to_string(),
    };
    vault_store.insert(password_item2)?;
    
    // Sync data
    vault_store.sync_data(node_id1)?;
    vault_store.sync_data(node_id2)?;
    
    // Verify merged items
    let merged_items = vault_store.list()?;
    assert_eq!(merged_items.len(), 2);
    
    Ok(())
}