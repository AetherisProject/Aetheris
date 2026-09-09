//! Digital signatures using Ed25519.
//!
//! Provides Ed25519 signature generation and verification with security-compliant
//! implementations. All private keys are zeroized on drop.

use anyhow::Result;
use ed25519_dalek::{
    SigningKey, VerifyingKey, Signature, Signer, Verifier,
};
use rand::rngs::OsRng;
use zeroize::Zeroize;

/// Ed25519 signature size in bytes (64 bytes = 512 bits).
pub const SIGNATURE_SIZE: usize = 64;

/// Ed25519 public key size in bytes (32 bytes = 256 bits).
pub const PUBLIC_KEY_SIZE: usize = 32;

/// Ed25519 private key size in bytes (32 bytes = 256 bits).
pub const PRIVATE_KEY_SIZE: usize = 32;

/// Ed25519 seed size in bytes (32 bytes = 256 bits).
pub const SEED_SIZE: usize = 32;

/// A wrapper for Ed25519 keypairs that zeroizes the private key on drop.
#[derive(Debug, Clone)]
pub struct Ed25519Keypair {
    pub verifying_key: VerifyingKey,
    pub signing_key: SigningKey,
}

impl Ed25519Keypair {
    /// Generate a new random Ed25519 keypair using OsRng.
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        Self {
            verifying_key,
            signing_key,
        }
    }

    /// Generate a keypair from a seed (32-byte array).
    /// The seed is zeroized after key generation.
    pub fn from_seed(seed: &[u8; SEED_SIZE]) -> Result<Self> {
        let mut seed_copy = *seed;
        let signing_key = SigningKey::from_bytes(&seed_copy)
            .map_err(|e| anyhow::anyhow!("Invalid Ed25519 seed: {}", e))?;
        let verifying_key = signing_key.verifying_key();
        // Zeroize the seed copy
        seed_copy.zeroize();
        Ok(Self {
            verifying_key,
            signing_key,
        })
    }

    /// Get the public key bytes.
    pub fn public_key_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.verifying_key.to_bytes()
    }

    /// Get the private key bytes (consumes self for security).
    pub fn into_private_key_bytes(self) -> [u8; PRIVATE_KEY_SIZE] {
        let bytes = self.signing_key.to_bytes();
        // The struct will be dropped, but we explicitly zeroize the signing_key
        // by consuming it (Drop impl handles this via Zeroize).
        bytes
    }

    /// Sign a message using this keypair.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    /// Verify a signature on a message using this keypair's verifying key.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.verifying_key
            .verify(message, signature)
            .is_ok()
    }
}

impl Zeroize for Ed25519Keypair {
    fn zeroize(&mut self) {
        self.signing_key.zeroize();
    }
}

/// A wrapper for Ed25519 public keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ed25519PublicKey(pub VerifyingKey);

impl Ed25519PublicKey {
    /// Create a new public key from bytes.
    pub fn from_bytes(bytes: &[u8; PUBLIC_KEY_SIZE]) -> Result<Self> {
        VerifyingKey::from_bytes(bytes)
            .map(Ed25519PublicKey)
            .map_err(|e| anyhow::anyhow!("Invalid Ed25519 public key: {}", e))
    }

    /// Get the public key bytes.
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.0.to_bytes()
    }

    /// Verify a signature on a message.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.0.verify(message, signature).is_ok()
    }
}

/// A wrapper for Ed25519 signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ed25519Signature(pub Signature);

impl Ed25519Signature {
    /// Create a new signature from bytes.
    pub fn from_bytes(bytes: &[u8; SIGNATURE_SIZE]) -> Result<Self> {
        Signature::from_bytes(bytes)
            .map(Ed25519Signature)
            .map_err(|e| anyhow::anyhow!("Invalid Ed25519 signature: {}", e))
    }

    /// Get the signature bytes.
    pub fn to_bytes(&self) -> [u8; SIGNATURE_SIZE] {
        self.0.to_bytes()
    }
}

/// Generate a new random Ed25519 keypair.
pub fn generate_keypair() -> Ed25519Keypair {
    Ed25519Keypair::generate()
}

/// Generate a new random Ed25519 seed.
pub fn generate_seed() -> [u8; SEED_SIZE] {
    let mut seed = [0u8; SEED_SIZE];
    let mut csprng = OsRng;
    csprng.fill_bytes(&mut seed);
    seed
}

/// Sign a message with an Ed25519 keypair.
pub fn sign_message(keypair: &Ed25519Keypair, message: &[u8]) -> Ed25519Signature {
    Ed25519Signature(keypair.sign(message))
}

/// Verify a message signature with an Ed25519 public key.
pub fn verify_signature(
    public_key: &Ed25519PublicKey,
    message: &[u8],
    signature: &Ed25519Signature,
) -> bool {
    public_key.verify(message, &signature.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let keypair = generate_keypair();
        assert_eq!(keypair.public_key_bytes().len(), PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_keypair();
        let message = b"Hello, Ed25519!";
        let signature = sign_message(&keypair, message);
        assert!(verify_signature(
            &Ed25519PublicKey(keypair.verifying_key),
            message,
            &signature
        ));
    }

    #[test]
    fn test_verify_fails_on_tampered_message() {
        let keypair = generate_keypair();
        let message = b"Hello, Ed25519!";
        let signature = sign_message(&keypair, message);
        let tampered_message = b"Hello, NOT Ed25519!";
        assert!(!verify_signature(
            &Ed25519PublicKey(keypair.verifying_key),
            tampered_message,
            &signature
        ));
    }

    #[test]
    fn test_verify_fails_on_wrong_public_key() {
        let keypair1 = generate_keypair();
        let keypair2 = generate_keypair();
        let message = b"Hello, Ed25519!";
        let signature = sign_message(&keypair1, message);
        assert!(!verify_signature(
            &Ed25519PublicKey(keypair2.verifying_key),
            message,
            &signature
        ));
    }

    #[test]
    fn test_from_seed() {
        let seed = generate_seed();
        let keypair = Ed25519Keypair::from_seed(&seed).unwrap();
        let message = b"Seed-based signature";
        let signature = sign_message(&keypair, message);
        assert!(verify_signature(
            &Ed25519PublicKey(keypair.verifying_key),
            message,
            &signature
        ));
    }

    #[test]
    fn test_signature_roundtrip() {
        let keypair = generate_keypair();
        let message = b"Roundtrip test";
        let signature = sign_message(&keypair, message);
        let signature_bytes = signature.to_bytes();
        let signature_restored = Ed25519Signature::from_bytes(&signature_bytes).unwrap();
        assert!(verify_signature(
            &Ed25519PublicKey(keypair.verifying_key),
            message,
            &signature_restored
        ));
    }

    #[test]
    fn test_public_key_roundtrip() {
        let keypair = generate_keypair();
        let public_key_bytes = keypair.public_key_bytes();
        let public_key_restored = Ed25519PublicKey::from_bytes(&public_key_bytes).unwrap();
        assert_eq!(public_key_restored.to_bytes(), public_key_bytes);
    }
}
