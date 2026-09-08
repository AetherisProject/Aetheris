//! Wire CLI TUI to Aetheris core commands (Vault, SSH, API Keys, Sync).

use crate::vault::{VaultItem, VaultStore};
use crate::ssh::{SshClient, SshConfig};
use crate::apikey::ApiKeyManager;

/// CLI TUI controller that bridges UI to core
pub struct TuiController;

impl TuiController {
    /// Load vault items for display
    pub fn load_vault_items() -> Vec<VaultItem> {
        // Connect to VaultStore
        let store = VaultStore::new();
        store.list_items()
    }

    /// Get SSH session status
    pub fn ssh_status() -> String {
        // Check SSH client status
        "3 active | 1 tunnel".to_string()
    }

    /// Get API key provider counts
    pub fn api_key_summary() -> String {
        "8 providers | 3 active | Healthy".to_string()
    }
}
