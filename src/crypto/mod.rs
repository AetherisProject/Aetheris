//! Cryptography engine for Aetheris.
//!
//! Provides key derivation (Argon2id), authenticated encryption (XChaCha20-Poly1305),
//! digital signatures (Ed25519), key exchange (X25519), Shamir Secret Sharing,
//! post-quantum hybrid mode, memory encryption, and plausible deniability.

pub mod cipher;
pub mod duress;
pub mod hardware;
pub mod kdf;
pub mod memory;
pub mod post_quantum;
pub mod shamir;

// Re-export commonly used types from submodules
pub use cipher::{generate_nonce, Ciphertext, EncryptionKey, NONCE_SIZE};
pub use kdf::{generate_salt, generate_master_key, derive_key_from_password, derive_key, contexts};
use crate::security::secure_cmp::SecureCompare;

use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroize;

/// HMAC-SHA256 type alias.
pub type HmacSha256 = Hmac<Sha256>;

/// The main crypto engine that provides all cryptographic operations.
#[derive(Debug, Clone)]
pub struct CryptoEngine {
    // CryptoEngine doesn't hold state, it's a stateless utility
}

impl CryptoEngine {
    /// Create a new crypto engine with default parameters.
    pub fn new() -> Self {
        Self {}
    }

    /// Derive a key from a password using Argon2id.
    pub fn derive_key_from_password(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
        kdf::derive_key_from_password(password, salt)
    }

    /// Generate a master key from a password with a random salt.
    pub fn generate_master_key(&self, password: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        kdf::generate_master_key(password)
    }

    /// Encrypt data using XChaCha20-Poly1305.
    pub fn encrypt(&self, plaintext: &[u8], key: &EncryptionKey, nonce: &[u8]) -> Result<Ciphertext> {
        cipher::encrypt(plaintext, key, nonce)
    }

    /// Decrypt data using XChaCha20-Poly1305.
    pub fn decrypt(&self, ciphertext: &Ciphertext, key: &EncryptionKey, nonce: &[u8]) -> Result<Vec<u8>> {
        cipher::decrypt(ciphertext, key, nonce)
    }

    /// Encrypt with a randomly generated nonce.
    pub fn encrypt_with_random_nonce(
        &self,
        plaintext: &[u8],
        key: &EncryptionKey,
    ) -> Result<(Ciphertext, Vec<u8>)> {
        cipher::encrypt_with_random_nonce(plaintext, key)
    }

    /// Generate a new random nonce.
    pub fn generate_nonce(&self) -> Vec<u8> {
        cipher::generate_nonce()
    }

    /// Generate a random salt for key derivation.
    pub fn generate_salt(&self) -> Vec<u8> {
        kdf::generate_salt()
    }

    /// Compute HMAC-SHA256 for data integrity verification.
    pub fn compute_hmac(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        let mut hmac = HmacSha256::new_from_slice(key)
            .map_err(|e| anyhow::anyhow!("Failed to create HMAC: {}", e))?;
        hmac.update(data);
        Ok(hmac.finalize().into_bytes().to_vec())
    }

    /// Verify HMAC-SHA256 using constant-time comparison.
    pub fn verify_hmac(&self, data: &[u8], key: &[u8], expected_hmac: &[u8]) -> Result<bool> {
        let computed = self.compute_hmac(data, key)?;
        // Use constant-time comparison for security
        Ok(computed.as_slice().secure_eq(expected_hmac))
    }
}

impl Default for CryptoEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// A secret value that is zeroized when dropped.
#[derive(Clone, Debug)]
pub struct SecretVec(Vec<u8>);

impl SecretVec {
    /// Create a new secret from bytes.
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Get a reference to the secret bytes.
    pub fn expose(&self) -> &[u8] {
        &self.0
    }
}

impl Zeroize for SecretVec {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl Drop for SecretVec {
    fn drop(&mut self) {
        self.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_engine_new() {
        let _engine = CryptoEngine::new();
        assert!(true); // Engine creation test
    }

    #[test]
    fn test_secret_vec() {
        let secret = SecretVec::new(vec![1, 2, 3, 4]);
        assert_eq!(secret.expose(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_encryption_roundtrip_through_engine() {
        let engine = CryptoEngine::new();
        let key = EncryptionKey::generate();
        let nonce = engine.generate_nonce();
        let plaintext = b"Hello, CryptoEngine!";

        let ciphertext = engine.encrypt(plaintext, &key, &nonce).unwrap();
        let decrypted = engine.decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_encryption_with_random_nonce_through_engine() {
        let engine = CryptoEngine::new();
        let key = EncryptionKey::generate();
        let plaintext = b"Test message";

        let (ciphertext, nonce) = engine.encrypt_with_random_nonce(plaintext, &key).unwrap();
        let decrypted = engine.decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_hmac_roundtrip() {
        let engine = CryptoEngine::new();
        let key = b"secret_key_for_hmac";
        let data = b"data_to_mac";

        let hmac = engine.compute_hmac(data, key).unwrap();
        assert_eq!(hmac.len(), 32); // SHA-256 produces 32 bytes

        let verified = engine.verify_hmac(data, key, &hmac).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_hmac_wrong_data_fails() {
        let engine = CryptoEngine::new();
        let key = b"secret_key_for_hmac";
        let data = b"original_data";
        let wrong_data = b"different_data";

        let hmac = engine.compute_hmac(data, key).unwrap();
        let verified = engine.verify_hmac(wrong_data, key, &hmac).unwrap();
        assert!(!verified);
    }

    #[test]
    fn test_hmac_wrong_key_fails() {
        let engine = CryptoEngine::new();
        let key = b"original_key";
        let wrong_key = b"wrong_key";
        let data = b"data_to_mac";

        let hmac = engine.compute_hmac(data, key).unwrap();
        let verified = engine.verify_hmac(data, wrong_key, &hmac).unwrap();
        assert!(!verified);
    }
}
