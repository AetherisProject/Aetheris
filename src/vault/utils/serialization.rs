//! Serialization utilities for VaultItem variants.
//!
//! Provides serialization and encryption utilities for VaultItem variants.

use anyhow::Result;
use bincode::{serialize, deserialize};
use crate::crypto::CryptoEngine;
use crate::vault::item::VaultItem;

/// Serialize a VaultItem to bytes.
pub fn serialize_item(item: &VaultItem) -> Result<Vec<u8>> {
    serialize(item)
}

/// Deserialize a VaultItem from bytes.
pub fn deserialize_item(data: &[u8]) -> Result<VaultItem> {
    deserialize(data)
}

/// Encrypt a serialized VaultItem using the provided CryptoEngine.
pub fn encrypt_item(item: &VaultItem, crypto_engine: &CryptoEngine) -> Result<Vec<u8>> {
    let serialized = serialize_item(item)?;
    let (ciphertext, iv) = crypto_engine.encrypt_memory(serialized.as_ref(), &serialized)?;
    Ok((iv, ciphertext))
}

/// Decrypt a VaultItem from encrypted bytes using the provided CryptoEngine.
pub fn decrypt_item(ciphertext: &[u8], iv: &[u8], crypto_engine: &CryptoEngine) -> Result<VaultItem> {
    let serialized = crypto_engine.decrypt_memory(ciphertext, iv)?;
    deserialize_item(&serialized)
}