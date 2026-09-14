//! Symmetric encryption engine for data at rest.
//!
//! Uses Argon2id for key derivation from a master password and AES-256-GCM
//! for authenticated encryption. All key material is zeroed on drop.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;
use secrecy::{ExposeSecret, SecretVec};
use zeroize::Zeroize;

use crate::error::AetherisError;

/// Size of the AES-256 key in bytes.
const KEY_SIZE: usize = 32;
/// Size of the AES-GCM nonce in bytes (96 bits).
const NONCE_SIZE: usize = 12;
/// Size of the salt used for key derivation.
const SALT_SIZE: usize = 32;
/// Argon2id memory cost in KiB (64 MB).
const ARGON2_M_COST: u32 = 65536;
/// Argon2id time cost (iterations).
const ARGON2_T_COST: u32 = 3;
/// Argon2id parallelism.
const ARGON2_P_COST: u32 = 4;

/// Vault encryption engine. Derives a key from a master password via Argon2id
/// and performs AES-256-GCM authenticated encryption.
pub struct CryptoEngine {
    /// The derived encryption key. Stored as a secret.
    key: SecretVec<u8>,
    /// The salt used for key derivation (stored for re-derivation).
    salt: Vec<u8>,
}

impl CryptoEngine {
    /// Creates a new engine with a random salt and derives the key from the
    /// provided master password.
    ///
    /// # Errors
    ///
    /// Returns `AetherisError::CryptoError` if key derivation fails.
    ///
    /// # Example
    ///
    /// ```
    /// use aetheris_core::crypto::engine::CryptoEngine;
    /// let engine = CryptoEngine::new("master-password-hunter2").unwrap();
    /// ```
    pub fn new(master_password: &str) -> Result<Self, AetherisError> {
        let mut salt = vec![0u8; SALT_SIZE];
        OsRng.fill_bytes(&mut salt);
        let key = Self::derive_key(master_password, &salt)?;
        Ok(Self {
            key: SecretVec::new(key),
            salt,
        })
    }

    /// Recreates an engine from a previously stored salt. Used when reopening
    /// an existing vault.
    pub fn from_salt(master_password: &str, salt: Vec<u8>) -> Result<Self, AetherisError> {
        let key = Self::derive_key(master_password, &salt)?;
        Ok(Self {
            key: SecretVec::new(key),
            salt,
        })
    }

    /// Derives a 256-bit key from password + salt using Argon2id.
    fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>, AetherisError> {
        let params = Params::new(
            ARGON2_M_COST,
            ARGON2_T_COST,
            ARGON2_P_COST,
            Some(KEY_SIZE),
        )
        .map_err(|e| AetherisError::CryptoError(format!("Invalid Argon2 params: {e}")))?;

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        let mut key = vec![0u8; KEY_SIZE];
        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| AetherisError::CryptoError(format!("Argon2id derivation failed: {e}")))?;

        Ok(key)
    }

    /// Encrypts plaintext using AES-256-GCM.
    ///
    /// Output format: `[nonce (12 bytes) || ciphertext + tag]`.
    ///
    /// A fresh random nonce is generated for every call.
    ///
    /// # Errors
    ///
    /// Returns `AetherisError::CryptoError` on encryption failure.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, AetherisError> {
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let cipher_key = Key::<Aes256Gcm>::from_slice(self.key.expose_secret());
        let cipher = Aes256Gcm::new(cipher_key);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| AetherisError::CryptoError(format!("AES-256-GCM encryption failed: {e}")))?;

        let mut output = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        output.extend_from_slice(&nonce_bytes);
        output.extend_from_slice(&ciphertext);
        Ok(output)
    }

    /// Decrypts ciphertext produced by [`Self::encrypt`].
    ///
    /// Expects input format: `[nonce (12 bytes) || ciphertext + tag]`.
    ///
    /// # Errors
    ///
    /// Returns `AetherisError::CryptoError` if decryption or authentication fails.
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, AetherisError> {
        if ciphertext.len() < NONCE_SIZE {
            return Err(AetherisError::CryptoError(
                "Ciphertext too short: missing nonce".into(),
            ));
        }

        let (nonce_bytes, encrypted) = ciphertext.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher_key = Key::<Aes256Gcm>::from_slice(self.key.expose_secret());
        let cipher = Aes256Gcm::new(cipher_key);

        cipher
            .decrypt(nonce, encrypted)
            .map_err(|_| AetherisError::CryptoError(
                "Decryption failed: wrong password or corrupted data".into(),
            ))
    }

    /// Returns the salt used for key derivation. Persist this alongside the vault.
    pub fn salt(&self) -> &[u8] {
        &self.salt
    }
}

impl Drop for CryptoEngine {
    fn drop(&mut self) {
        self.salt.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_with_salt() {
        let engine = CryptoEngine::new("test-password").unwrap();
        assert_eq!(engine.salt().len(), SALT_SIZE);
    }

    #[test]
    fn from_salt_reproduces() {
        let engine = CryptoEngine::new("hunter2").unwrap();
        let salt = engine.salt().to_vec();
        let engine2 = CryptoEngine::from_salt("hunter2", salt).unwrap();
        assert_eq!(engine.salt(), engine2.salt());
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let engine = CryptoEngine::new("correct horse battery staple").unwrap();
        let plaintext = b"sk-proj-abcdef123456";
        let ciphertext = engine.encrypt(plaintext).unwrap();
        assert_ne!(ciphertext, plaintext);
        let decrypted = engine.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn decrypt_with_wrong_password_fails() {
        let engine1 = CryptoEngine::new("password-A").unwrap();
        let engine2 = CryptoEngine::new("password-B").unwrap();
        let ciphertext = engine1.encrypt(b"secret").unwrap();
        assert!(engine2.decrypt(&ciphertext).is_err());
    }

    #[test]
    fn decrypt_truncated_input_fails() {
        let engine = CryptoEngine::new("pass").unwrap();
        assert!(engine.decrypt(&[1, 2, 3]).is_err());
    }

    #[test]
    fn decrypt_corrupted_ciphertext_fails() {
        let engine = CryptoEngine::new("pass").unwrap();
        let mut ciphertext = engine.encrypt(b"data").unwrap();
        // Corrupt a byte in the ciphertext (not the nonce).
        if ciphertext.len() > NONCE_SIZE + 5 {
            ciphertext[NONCE_SIZE + 5] ^= 0xff;
        }
        assert!(engine.decrypt(&ciphertext).is_err());
    }

    #[test]
    fn each_encrypt_uses_unique_nonce() {
        let engine = CryptoEngine::new("pass").unwrap();
        let ct1 = engine.encrypt(b"same plaintext").unwrap();
        let ct2 = engine.encrypt(b"same plaintext").unwrap();
        // Nonce is the first 12 bytes; must differ.
        assert_ne!(&ct1[..NONCE_SIZE], &ct2[..NONCE_SIZE]);
    }

    #[test]
    fn salt_is_unique_per_instance() {
        let e1 = CryptoEngine::new("same").unwrap();
        let e2 = CryptoEngine::new("same").unwrap();
        assert_ne!(e1.salt(), e2.salt());
    }

    #[test]
    fn empty_plaintext_roundtrip() {
        let engine = CryptoEngine::new("p").unwrap();
        let ct = engine.encrypt(b"").unwrap();
        let pt = engine.decrypt(&ct).unwrap();
        assert!(pt.is_empty());
    }

    #[test]
    fn large_plaintext_roundtrip() {
        let engine = CryptoEngine::new("p").unwrap();
        let data = vec![0xABu8; 1024 * 1024];
        let ct = engine.encrypt(&data).unwrap();
        let pt = engine.decrypt(&ct).unwrap();
        assert_eq!(pt, data);
    }
}
