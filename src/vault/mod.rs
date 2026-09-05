//! Encrypted vault store for Aetheris.
pub mod item;
pub mod store;

pub use item::*;
pub use store::VaultStore;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_item_type() {
        let item = VaultItem::Password(PasswordItem::new("Test".to_string(), "user".to_string()));
        assert_eq!(item.item_type(), "password");
    }

    #[test]
    fn test_vault_store_new() {
        let _store = VaultStore::new(":memory:").unwrap();
    }
}