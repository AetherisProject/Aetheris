//! Encrypted vault store for Aetheris.
pub mod item;
pub mod store;

pub use item::*;
pub use store::VaultStore;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::EncryptionKey;

    #[test]
    fn test_vault_item_type() {
        let item = VaultItem::Password(PasswordItem::new("Test".to_string(), "user".to_string()));
        assert_eq!(item.item_type(), "password");
    }

    #[test]
    fn test_vault_store_new() {
        let _store = VaultStore::new("./test_vault_db").unwrap();
    }

    #[test]
    fn test_vault_encryption_cycle() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_vault");
        let db_path_str = db_path.to_str().unwrap();
        
        // Create vault store
        let mut store = VaultStore::new(db_path_str).unwrap();
        
        // Initialize with a master key
        let master_key = EncryptionKey::generate();
        store.initialize(master_key.clone()).unwrap();
        
        // Create a test password item
        let mut password_item = PasswordItem::new("Test Site".to_string(), "testuser".to_string());
        password_item.password = "testpassword123".to_string();
        password_item.notes = "Test notes".to_string();
        
        let item = VaultItem::Password(password_item);
        
        // Insert item
        store.insert(item.clone()).unwrap();
        
        // Retrieve item
        let retrieved_item = store.get(item.id()).unwrap().unwrap();
        
        // Verify decrypted item matches original
        assert_eq!(retrieved_item, item);
        
        // Test search
        let results = store.search("Test Site").unwrap();
        assert_eq!(results.len(), 1);
        
        // Test delete
        store.delete(item.id()).unwrap();
        let deleted_item = store.get(item.id()).unwrap();
        assert!(deleted_item.is_none());
    }
}