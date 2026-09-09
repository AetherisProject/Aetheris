//! Memory encryption using authenticated encryption modes.
//!
//! Provides secure memory encryption and decryption using authenticated encryption.

use anyhow::Result;
use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
use rand::Rng;
use sha2::Sha256;
use hkdf::Hkdf;
use zeroize::Zeroize;

/// Encrypts memory using authenticated encryption (AES-GCM).
pub fn encrypt_memory(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = derive_key(key)?;
    let iv = OsRng.gen::<[u8; 12]>();
    let cipher = chacha20poly1305::Chacha20Poly1305::new(&key);
    let ciphertext = cipher.encrypt(&iv, plaintext)?;
    Ok((iv.to_vec(), ciphertext))
}

/// Decrypts memory using authenticated encryption (AES-GCM).
pub fn decrypt_memory(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let key = derive_key(key)?;
    let cipher = chacha20poly1305::Chacha20Poly1305::new(&key);
    let plaintext = cipher.decrypt(iv, ciphertext)?;
    Ok(plaintext)
}

/// Derives a key from a raw key using HKDF.
fn derive_key(key: &[u8]) -> Result<[u8; 32]> {
    let mut key_bytes = key.to_vec();
    if key_bytes.len() < 32 {
        key_bytes.resize(32, 0);
    }
    let derived_key = Hkdf::<Sha256>::new(&Sha256::new(), key_bytes.as_slice()).derive_key(32);
    Ok(derived_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    #[test]
    fn test_encrypt_decrypt_memory() {
        let key = rand::thread_rng().sample_iter(&Alphanumeric::default).take(32).collect::<Vec<u8>>();
        let plaintext = b"This is a secret message for memory encryption.";

        let (iv, ciphertext) = encrypt_memory(&key, plaintext).unwrap();
        let decrypted = decrypt_memory(&key, &iv, &ciphertext).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_derive_key() {
        let raw_key = b"test_key_for_derivation";
        let derived_key = derive_key(raw_key).unwrap();
        assert_eq!(derived_key.len(), 32);
    }
}