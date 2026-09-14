// Aetheris Core Library
// Shared logic for all platforms (Desktop, Mobile, Web, CLI)

pub mod vault;
pub mod crypto;
pub mod ssh;
pub mod sync;
pub mod error;
pub mod env;

// Re-export key types
pub use vault::store::{ApiKeyItem, Snapshot, VaultStore};
pub use vault::VaultItem;
pub use crypto::engine::CryptoEngine;
pub use ssh::SshClient;
pub use sync::b2::B2SyncClient;
pub use sync::SyncClient;
pub use error::AetherisError;
