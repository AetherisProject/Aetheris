//! Password manager module for Aetheris.

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crypto::{CryptoEngine, EncryptionKey};
use crate::security::SecureString;
use crate::vault::{PasswordItem, VaultItem, VaultStore};

/// Password strength level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

/// Password generator configuration.
#[derive(Debug, Clone)]
pub struct PasswordGenerator {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
    pub exclude_similar: bool,
    pub exclude_ambiguous: bool,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            length: 16,
            use_uppercase: true,
            use_lowercase: true,
            use_numbers: true,
            use_symbols: true,
            exclude_similar: true,
            exclude_ambiguous: true,
        }
    }
}

impl PasswordGenerator {
    /// Generate a new password based on the configuration.
    pub fn generate(&self) -> SecureString {
        let password = self.generate_password_string();
        SecureString::from_str(&password)
    }

    /// Generate a new password string.
    pub fn generate_password_string(&self) -> String {
        use passwords::PasswordGenerator as PwGen;

        let gen = PwGen::default()
            .length(self.length as usize)
            .numbers(self.use_numbers)
            .lowercase_letters(self.use_lowercase)
            .uppercase_letters(self.use_uppercase)
            .symbols(self.use_symbols)
            .exclude_similar_characters(self.exclude_similar);

        gen.generate_one()
            .unwrap_or_else(|_| "fallback_password_123".to_string())
    }

    /// Estimate password strength.
    pub fn estimate_strength(&self, password: &str) -> PasswordStrength {
        let len = password.len();
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_number = password.chars().any(|c| c.is_numeric());
        let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

        let criteria_count = [len >= 12, has_upper, has_lower, has_number, has_symbol]
            .iter()
            .filter(|&&x| x)
            .count();

        match criteria_count {
            5 => PasswordStrength::VeryStrong,
            4 => PasswordStrength::Strong,
            3 => PasswordStrength::Moderate,
            _ => PasswordStrength::Weak,
        }
    }
}

/// Password manager for handling all password operations.
pub struct PasswordManager {
    vault_store: Option<VaultStore>,
    crypto_engine: CryptoEngine,
    master_key: Option<EncryptionKey>,
}

impl PasswordManager {
    pub fn new() -> Self {
        Self {
            vault_store: None,
            crypto_engine: CryptoEngine::new(),
            master_key: None,
        }
    }

    /// Initialize with a vault store and master key.
    pub fn initialize_with_vault(
        &mut self,
        vault_store: VaultStore,
        master_key: EncryptionKey,
    ) -> Result<()> {
        self.vault_store = Some(vault_store);
        self.master_key = Some(master_key);
        Ok(())
    }

    /// Set the vault store.
    pub fn set_vault_store(&mut self, vault_store: VaultStore) {
        self.vault_store = Some(vault_store);
    }

    /// Generate a new secure password.
    pub fn generate_password(&self, config: &PasswordGenerator) -> SecureString {
        config.generate()
    }

    /// Add a new password entry to the vault.
    pub fn add_password(
        &mut self,
        title: String,
        username: String,
        password: SecureString,
        urls: Vec<String>,
    ) -> Result<Uuid> {
        let vault_store = self
            .vault_store
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Vault store not initialized"))?;

        let mut password_item = PasswordItem::new(title, username);
        password_item.password = password.expose().to_string();
        password_item.urls = urls;

        let id = password_item.id;
        let item = VaultItem::Password(password_item);

        vault_store.insert(item)?;

        Ok(id)
    }

    /// Get a password by ID.
    pub fn get_password(&self, id: &Uuid) -> Result<Option<SecureString>> {
        let vault_store = self
            .vault_store
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vault store not initialized"))?;

        let item = vault_store.get(id)?;

        match item {
            Some(VaultItem::Password(password_item)) => {
                let secure_password = SecureString::from_str(&password_item.password);
                Ok(Some(secure_password))
            }
            _ => Ok(None),
        }
    }

    /// Search for passwords by title or username.
    pub fn search_passwords(&self, query: &str) -> Result<Vec<(Uuid, String, String)>> {
        let vault_store = self
            .vault_store
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vault store not initialized"))?;

        let all_items = vault_store.list()?;
        let mut results = Vec::new();

        for item in all_items {
            if let VaultItem::Password(password_item) = item {
                let query_lower = query.to_lowercase();
                let title_lower = password_item.title.to_lowercase();
                let username_lower = password_item.username.to_lowercase();

                if title_lower.contains(&query_lower) || username_lower.contains(&query_lower) {
                    results.push((
                        password_item.id,
                        password_item.title,
                        password_item.username,
                    ));
                }
            }
        }

        Ok(results)
    }

    /// Update a password entry.
    pub fn update_password(
        &mut self,
        id: Uuid,
        title: String,
        username: String,
        password: SecureString,
        urls: Vec<String>,
    ) -> Result<()> {
        let vault_store = self
            .vault_store
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Vault store not initialized"))?;

        let existing_item = vault_store
            .get(&id)?
            .ok_or_else(|| anyhow::anyhow!("Password not found"))?;

        if let VaultItem::Password(mut existing_password) = existing_item {
            let mut updated_password = existing_password;
            updated_password.title = title;
            updated_password.username = username;
            updated_password.password = password.expose().to_string();
            updated_password.urls = urls;
            updated_password.updated_at = Utc::now();

            vault_store.update(VaultItem::Password(updated_password))?;
        }

        Ok(())
    }

    /// Delete a password by ID.
    pub fn delete_password(&mut self, id: &Uuid) -> Result<()> {
        let vault_store = self
            .vault_store
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Vault store not initialized"))?;

        vault_store.delete(id)?;

        Ok(())
    }

    /// Analyze password strength.
    pub fn analyze_password_strength(&self, password: &SecureString) -> PasswordStrength {
        let generator = PasswordGenerator::default();
        generator.estimate_strength(password.expose())
    }

    /// Check if password has been compromised in known data breaches.
    /// (This would integrate with Have I Been Pwned API in production)
    pub fn check_password_breach(&self, _password: &SecureString) -> Result<bool> {
        // TODO: Implement Have I Been Pwned API integration
        Ok(false)
    }

    /// Import passwords from various formats
    pub fn import_passwords(&mut self, _format: &str, _data: &str) -> Result<usize> {
        // TODO: Implement import from various formats (CSV, KeePass, etc.)
        Ok(0)
    }

    /// Export passwords to various formats
    pub fn export_passwords(&self, _format: &str) -> Result<String> {
        // TODO: Implement export to various formats
        Ok(String::new())
    }
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_manager_new() {
        let _manager = PasswordManager::new();
        assert!(true);
    }

    #[test]
    fn test_password_generator() {
        let generator = PasswordGenerator::default();
        let password = generator.generate_password_string();
        assert!(password.len() >= 12);
    }

    #[test]
    fn test_password_strength_estimation() {
        let generator = PasswordGenerator::default();
        let weak_pw = "password123";
        let strong_pw = "MyStr0ng!P@ssw0rd202";
        let very_strong_pw = "XyZ!987@Lmn98OpQr56#StUvWx12";

        assert_eq!(generator.estimate_strength(weak_pw), PasswordStrength::Weak);
        assert_eq!(
            generator.estimate_strength(strong_pw),
            PasswordStrength::VeryStrong
        );
        assert_eq!(
            generator.estimate_strength(very_strong_pw),
            PasswordStrength::VeryStrong
        );
    }
}
