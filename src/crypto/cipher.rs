//! Authenticated encryption using XChaCha20-Poly1305.

use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use std::convert::TryFrom;

/// Key sizes for XChaCha20-Poly1305
pub const KEY_SIZE: usize = 32; // 256 bits
pub const NONCE_SIZE: usize = 24; // 192 bits for XChaCha20-Poly1305
pub const TAG_SIZE: usize = 16; // 128 bits

/// Encryption key wrapper.
#[derive(Debug, Clone)]
pub struct EncryptionKey(pub Vec<u8>);

impl EncryptionKey {
    /// Create a new encryption key from bytes.
    pub fn new(key: Vec<u8>) -> Result<Self> {
        if key.len() != KEY_SIZE {
            return Err(anyhow::anyhow!(
                "Encryption key must be {} bytes, got {}",
                KEY_SIZE,
                key.len()
            ));
        }
        Ok(Self(key))
    }

    /// Generate a new random encryption key.
    pub fn generate() -> Self {
        let mut key = vec![0u8; KEY_SIZE];
        let mut rng = OsRng;
        rng.fill_bytes(&mut key);
        Self(key)
    }

    /// Get the key bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert to raw bytes (consumes self).
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl TryFrom<Vec<u8>> for EncryptionKey {
    type Error = anyhow::Error;

    fn try_from(key: Vec<u8>) -> Result<Self> {
        Self::new(key)
    }
}

/// Ciphertext wrapper that includes the authentication tag.
#[derive(Debug, Clone)]
pub struct Ciphertext(pub Vec<u8>);

impl Ciphertext {
    /// Create a new ciphertext from bytes.
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Get the ciphertext bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert to raw bytes (consumes self).
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Get the length of the ciphertext.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the ciphertext is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<u8>> for Ciphertext {
    fn from(data: Vec<u8>) -> Self {
        Self::new(data)
    }
}

impl From<Ciphertext> for Vec<u8> {
    fn from(ciphertext: Ciphertext) -> Self {
        ciphertext.0
    }
}

/// Encrypt plaintext using XChaCha20-Poly1305.
///
/// Uses authenticated encryption (AEAD) with the provided nonce.
/// The nonce should be unique for each encryption operation.
pub fn encrypt(plaintext: &[u8], key: &EncryptionKey, nonce: &[u8]) -> Result<Ciphertext> {
    let cipher = XChaCha20Poly1305::new_from_slice(key.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

    let nonce = XNonce::from_slice(nonce);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    Ok(Ciphertext::new(ciphertext))
}

/// Decrypt ciphertext using XChaCha20-Poly1305.
///
/// The nonce must match the one used during encryption.
/// Returns an error if authentication fails.
pub fn decrypt(ciphertext: &Ciphertext, key: &EncryptionKey, nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new_from_slice(key.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

    let nonce = XNonce::from_slice(nonce);
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    Ok(plaintext)
}

/// Generate a new random nonce for XChaCha20-Poly1305.
///
/// The nonce is 24 bytes (192 bits) and must be unique for each encryption.
pub fn generate_nonce() -> Vec<u8> {
    let mut nonce = vec![0u8; NONCE_SIZE];
    let mut rng = OsRng;
    rng.fill_bytes(&mut nonce);
    nonce
}

/// Encrypt plaintext with a randomly generated nonce.
///
/// Returns the ciphertext and the nonce used.
pub fn encrypt_with_random_nonce(
    plaintext: &[u8],
    key: &EncryptionKey,
) -> Result<(Ciphertext, Vec<u8>)> {
    let nonce = generate_nonce();
    let ciphertext = encrypt(plaintext, key, &nonce)?;
    Ok((ciphertext, nonce))
}

/// Simple encryption/decryption with managed nonce handling.
///
/// For cases where nonce management is not critical.
pub fn encrypt_simple(plaintext: &[u8], key: &EncryptionKey) -> Result<(Ciphertext, Vec<u8>)> {
    encrypt_with_random_nonce(plaintext, key)
}

/// Decrypt with simple nonce handling.
pub fn decrypt_simple(
    ciphertext: &Ciphertext,
    key: &EncryptionKey,
    nonce: &[u8],
) -> Result<Vec<u8>> {
    decrypt(ciphertext, key, nonce)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_key_creation() {
        let key = EncryptionKey::generate();
        assert_eq!(key.as_bytes().len(), KEY_SIZE);
    }

    #[test]
    fn test_encryption_key_from_bytes() {
        let bytes = vec![0x42u8; KEY_SIZE];
        let key = EncryptionKey::new(bytes).unwrap();
        assert_eq!(key.as_bytes().len(), KEY_SIZE);
    }

    #[test]
    fn test_encryption_key_wrong_size() {
        let bytes = vec![0x42u8; KEY_SIZE - 1];
        let result = EncryptionKey::new(bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = EncryptionKey::generate();
        let nonce = generate_nonce();
        let plaintext = b"Hello, world! This is a test of XChaCha20-Poly1305 encryption.";

        let ciphertext = encrypt(plaintext, &key, &nonce).unwrap();
        let decrypted = decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_encrypt_with_random_nonce() {
        let key = EncryptionKey::generate();
        let plaintext = b"Test message";

        let (ciphertext1, nonce1) = encrypt_with_random_nonce(plaintext, &key).unwrap();
        let (ciphertext2, nonce2) = encrypt_with_random_nonce(plaintext, &key).unwrap();

        // Different nonces should produce different ciphertexts
        assert_ne!(nonce1, nonce2);
        assert_ne!(ciphertext1.as_bytes(), ciphertext2.as_bytes());

        // Both should decrypt correctly
        let decrypted1 = decrypt(&ciphertext1, &key, &nonce1).unwrap();
        let decrypted2 = decrypt(&ciphertext2, &key, &nonce2).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted1.as_slice());
        assert_eq!(plaintext.as_slice(), decrypted2.as_slice());
    }

    #[test]
    fn test_encryption_with_wrong_key_fails() {
        let key1 = EncryptionKey::generate();
        let key2 = EncryptionKey::generate();
        let nonce = generate_nonce();
        let plaintext = b"Secret message";

        let ciphertext = encrypt(plaintext, &key1, &nonce).unwrap();
        let result = decrypt(&ciphertext, &key2, &nonce);

        assert!(result.is_err());
    }

    #[test]
    fn test_encryption_with_wrong_nonce_fails() {
        let key = EncryptionKey::generate();
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        let plaintext = b"Secret message";

        let ciphertext = encrypt(plaintext, &key, &nonce1).unwrap();
        let result = decrypt(&ciphertext, &key, &nonce2);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_nonce_size() {
        let nonce = generate_nonce();
        assert_eq!(nonce.len(), NONCE_SIZE);
    }

    #[test]
    fn test_ciphertext_conversions() {
        let data = vec![1, 2, 3, 4, 5];
        let ciphertext = Ciphertext::new(data.clone());

        assert_eq!(ciphertext.as_bytes(), data.as_slice());
        assert_eq!(Vec::<u8>::from(ciphertext.clone()), data);
        assert_eq!(ciphertext.into_bytes(), data);
    }
}
