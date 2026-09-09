use anyhow::{Context, Result};
use uuid::Uuid;
use aetheris::vault::item::PasswordItem;
use aetheris::web::client::WebClient;
use aetheris::vault::store::VaultStore;
use aetheris::crypto::CryptoEngine;

#[tokio::test]
async fn test_web_client_insert() -> Result<()> {
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
    assert_eq!(item.unwrap().title, "Test Password");
    
    Ok(())
}

#[tokio::test]
async fn test_web_client_sync() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_web_sync")?;
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
    
    web_client.insert_item(password_item).await?;
    
    // Sync data
    web_client.sync_with_extension().await?;
    
    Ok(())
}