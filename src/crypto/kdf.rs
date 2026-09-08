//! Key Derivation Function implementations using Argon2id and HKDF.

use anyhow::Result;
use argon2::{self, password_hash::SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use hkdf::Hkdf;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use std::sync::OnceLock;

/// Argon2id parameters from .memory/security.md: t=3, m=64MB, p=4
/// m=64MB = 65536 KiB
const ARGON2_TIME_COST: u32 = 3;
const ARGON2_MEMORY_COST: u32 = 65536; // 64MB in KiB
const ARGON2_PARALLELISM: u32 = 4;
const ARGON2_OUTPUT_LENGTH: usize = 32; // 32 bytes = 256 bits

/// Get the Argon2 parameters as a static value
fn argon2_params() -> &'static Params {
    static PARAMS: OnceLock<Params> = OnceLock::new();
    PARAMS.get_or_init(|| {
        Params::new(
            ARGON2_MEMORY_COST,
            ARGON2_TIME_COST,
            ARGON2_PARALLELISM,
            Some(ARGON2_OUTPUT_LENGTH),
        )
        .expect("Invalid Argon2 parameters")
    })
}

/// Generate a cryptographically secure random salt using OsRng.
pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 32]; // 256-bit salt
    let mut rng = OsRng;
    rng.fill_bytes(&mut salt);
    salt
}

/// Generate a derived key from a password using Argon2id.
///
/// Uses security compliant parameters: t=3, m=64MB, p=4
/// Returns the derived key as a byte vector.
pub fn derive_key_from_password(password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params().clone());

    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| anyhow::anyhow!("Failed to encode salt for Argon2id: {}", e))?;

    let password_hash = argon2
        .hash_password(password, &salt_string)
        .map_err(|e| anyhow::anyhow!("Argon2id hashing failed: {}", e))?;

    let hash_bytes = password_hash
        .hash
        .ok_or_else(|| anyhow::anyhow!("Argon2id hashing produced no output"))?
        .as_bytes()
        .to_vec();

    Ok(hash_bytes[..ARGON2_OUTPUT_LENGTH.min(hash_bytes.len())].to_vec())
}

/// Derive a key from a master key using HKDF-SHA256.
pub fn derive_key(master_key: &[u8], context: &[u8], output_length: usize) -> Result<Vec<u8>> {
    let hkdf = Hkdf::<Sha256>::new(Some(master_key), context);
    let mut output = vec![0u8; output_length];
    hkdf.expand(&[], &mut output)
        .map_err(|e| anyhow::anyhow!("HKDF expansion failed: {}", e))?;
    Ok(output)
}

/// Generate a master key from a password with a random salt.
pub fn generate_master_key(password: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let salt = generate_salt();
    let key = derive_key_from_password(password, &salt)?;
    Ok((key, salt))
}

/// Verify a password against an Argon2id hash string.
pub fn verify_password(password: &[u8], encoded_hash: &str) -> Result<bool> {
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params().clone());
    let parsed_hash = PasswordHash::new(encoded_hash)
        .map_err(|e| anyhow::anyhow!("Failed to parse Argon2id hash: {}", e))?;
    Ok(argon2.verify_password(password, &parsed_hash).is_ok())
}

/// Key derivation contexts for standard use cases
pub mod contexts {
    pub const VAULT_ENCRYPTION: &str = "aetheris_vault_encryption_v1";
    pub const VAULT_HMAC: &str = "aetheris_vault_hmac_v1";
    pub const SYNC_ENCRYPTION: &str = "aetheris_sync_encryption_v1";
    pub const SYNC_HMAC: &str = "aetheris_sync_hmac_v1";
    pub const AUTHENTICATION: &str = "aetheris_authentication_v1";
    pub const SIGNING: &str = "aetheris_signing_v1";
    pub const KEY_EXCHANGE: &str = "aetheris_key_exchange_v1";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_salt() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();
        assert_eq!(salt1.len(), 32);
        assert_eq!(salt2.len(), 32);
    }

    #[test]
    fn test_derive_key_from_password() {
        let password = b"test_password_123";
        let salt = generate_salt();
        let derived_key = derive_key_from_password(password, &salt).unwrap();
        assert_eq!(derived_key.len(), 32);
    }

    #[test]
    fn test_generate_master_key() {
        let password = b"my_secure_password";
        let (key, salt) = generate_master_key(password).unwrap();
        assert_eq!(key.len(), 32);
        assert_eq!(salt.len(), 32);
    }

    #[test]
    fn test_hkdf_derivation() {
        let master_key = vec![0x42u8; 32]; // 32-byte master key
        let context = b"test_context";

        let derived = derive_key(&master_key, context, 32).unwrap();
        assert_eq!(derived.len(), 32);

        let derived2 = derive_key(&master_key, context, 32).unwrap();
        assert_eq!(derived, derived2);

        let derived3 = derive_key(&master_key, b"different_context", 32).unwrap();
        assert_ne!(derived, derived3);
    }

    #[test]
    fn test_argon2_parameters() {
        let params = argon2_params();
        assert_eq!(params.m_cost(), ARGON2_MEMORY_COST);
        assert_eq!(params.t_cost(), ARGON2_TIME_COST);
        assert_eq!(params.p_cost(), ARGON2_PARALLELISM);
    }
}
