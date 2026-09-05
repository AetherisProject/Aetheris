//! Vault tests for Aetheris.

use aetheris::vault::item::VaultItem;
use aetheris::vault::store::VaultStore;
use aetheris::vault::item::PasswordItem;

#[test]
fn test_password_item_new() {
    let item = PasswordItem::new("GitHub".to_string(), "user@example.com".to_string());
    assert_eq!(item.title, "GitHub");
    assert_eq!(item.username, "user@example.com");
}

#[test]
fn test_vault_item_type() {
    let item = VaultItem::Password(PasswordItem::new("Test".to_string(), "user".to_string()));
    assert_eq!(item.item_type(), "password");
}

#[test]
fn test_vault_store_new() {
    let _store = VaultStore::new(":memory:").unwrap();
}
