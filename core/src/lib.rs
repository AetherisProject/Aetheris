// Aetheris Core Library
// Shared logic for all platforms (Desktop, Mobile, Web, CLI)

pub mod vault;
pub mod crypto;
pub mod ssh;
pub mod sync;
pub mod error;

// Re-export key types
pub use vault::VaultItem;
pub use crypto::CryptoEngine;
pub use ssh::SshClient;
pub use sync::SyncClient;
pub use error::AetherisError;
