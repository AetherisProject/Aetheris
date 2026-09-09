//! Key exchange using X25519 (Curve25519).
//!
//! Provides X25519 elliptic curve Diffie-Hellman key exchange with security-compliant
//! implementations. All private keys are zeroized on drop.

use anyhow::Result;
use x25519_dalek::{
    EphemeralSecret, PublicKey, ReusableSecret, SharedSecret,
};
use rand::rngs::OsRng;
use zeroize::Zeroize;

/// X25519 public key size in bytes (32 bytes = 256 bits).
pub const PUBLIC_KEY_SIZE: usize = 32;

/// X25519 private key size in bytes (32 bytes = 256 bits).
pub const PRIVATE_KEY_SIZE: usize = 32;

/// X25519 shared secret size in bytes (32 bytes = 256 bits).
pub const SHARED_SECRET_SIZE: usize = 32;

/// A wrapper for X25519 static keypairs (long-term keys).
#[derive(Debug, Clone)]
pub struct X25519StaticKeypair {
    pub public_key: PublicKey,
    pub reusable_secret: ReusableSecret,
}

impl X25519StaticKeypair {
    /// Generate a new random X25519 static keypair using OsRng.
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let reusable_secret = ReusableSecret::random(&mut csprng);
        let public_key = PublicKey::from(&reusable_secret);
        Self {
            public_key,
            reusable_secret,
        }
    }

    /// Generate a static keypair from a private key (32-byte array).
    pub fn from_private_key(private_key: &[u8; PRIVATE_KEY_SIZE]) -> Result<Self> {
        let reusable_secret = ReusableSecret::from_bytes(*private_key)
            .map_err(|e| anyhow::anyhow!("Invalid X25519 private key: {}", e))?;
        let public_key = PublicKey::from(&reusable_secret);
        Ok(Self {
            public_key,
            reusable_secret,
        })
    }

    /// Get the public key bytes.
    pub fn public_key_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.public_key.to_bytes()
    }

    /// Get the private key bytes (consumes self for security).
    pub fn into_private_key_bytes(self) -> [u8; PRIVATE_KEY_SIZE] {
        let bytes = self.reusable_secret.to_bytes();
        // The struct will be dropped, zeroizing the reusable_secret.
        bytes
    }

    /// Perform key exchange with a remote public key.
    /// Returns a shared secret (32 bytes).
    pub fn exchange(&self, remote_public_key: &PublicKey) -> SharedSecret {
        self.reusable_secret.diffie_hellman(remote_public_key)
    }
}

impl Zeroize for X25519StaticKeypair {
    fn zeroize(&mut self) {
        self.reusable_secret.zeroize();
    }
}

/// A wrapper for X25519 ephemeral keypairs (short-term keys for forward secrecy).
#[derive(Debug, Clone)]
pub struct X25519EphemeralKeypair {
    pub public_key: PublicKey,
    pub ephemeral_secret: EphemeralSecret,
}

impl X25519EphemeralKeypair {
    /// Generate a new random X25519 ephemeral keypair using OsRng.
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let ephemeral_secret = EphemeralSecret::random(&mut csprng);
        let public_key = PublicKey::from(&ephemeral_secret);
        Self {
            public_key,
            ephemeral_secret,
        }
    }

    /// Get the public key bytes.
    pub fn public_key_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.public_key.to_bytes()
    }

    /// Perform key exchange with a remote public key.
    /// Returns a shared secret (32 bytes).
    pub fn exchange(&self, remote_public_key: &PublicKey) -> SharedSecret {
        self.ephemeral_secret.diffie_hellman(remote_public_key)
    }
}

impl Zeroize for X25519EphemeralKeypair {
    fn zeroize(&mut self) {
        self.ephemeral_secret.zeroize();
    }
}

/// A wrapper for X25519 public keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X25519PublicKey(pub PublicKey);

impl X25519PublicKey {
    /// Create a new public key from bytes.
    pub fn from_bytes(bytes: &[u8; PUBLIC_KEY_SIZE]) -> Result<Self> {
        PublicKey::from_bytes(*bytes)
            .map(X25519PublicKey)
            .map_err(|e| anyhow::anyhow!("Invalid X25519 public key: {}", e))
    }

    /// Get the public key bytes.
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.0.to_bytes()
    }
}

/// A wrapper for X25519 shared secrets.
#[derive(Debug, Clone, Copy)]
pub struct X25519SharedSecret(pub SharedSecret);

impl X25519SharedSecret {
    /// Get the shared secret bytes.
    pub fn to_bytes(&self) -> [u8; SHARED_SECRET_SIZE] {
        self.0.to_bytes()
    }
}

/// Generate a new random X25519 static keypair.
pub fn generate_static_keypair() -> X25519StaticKeypair {
    X25519StaticKeypair::generate()
}

/// Generate a new random X25519 ephemeral keypair.
pub fn generate_ephemeral_keypair() -> X25519EphemeralKeypair {
    X25519EphemeralKeypair::generate()
}

/// Perform X25519 key exchange between a static keypair and a remote public key.
pub fn exchange_static(
    keypair: &X25519StaticKeypair,
    remote_public_key: &X25519PublicKey,
) -> X25519SharedSecret {
    X25519SharedSecret(keypair.exchange(&remote_public_key.0))
}

/// Perform X25519 key exchange between an ephemeral keypair and a remote public key.
pub fn exchange_ephemeral(
    keypair: &X25519EphemeralKeypair,
    remote_public_key: &X25519PublicKey,
) -> X25519SharedSecret {
    X25519SharedSecret(keypair.exchange(&remote_public_key.0))
}

/// Derive a shared secret directly from private and public keys.
pub fn derive_shared_secret(
    private_key: &[u8; PRIVATE_KEY_SIZE],
    public_key: &[u8; PUBLIC_KEY_SIZE],
) -> Result<X25519SharedSecret> {
    let reusable_secret = ReusableSecret::from_bytes(*private_key)
        .map_err(|e| anyhow::anyhow!("Invalid X25519 private key: {}", e))?;
    let remote_public_key = PublicKey::from_bytes(*public_key)
        .map_err(|e| anyhow::anyhow!("Invalid X25519 public key: {}", e))?;
    Ok(X25519SharedSecret(
        reusable_secret.diffie_hellman(&remote_public_key),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_static_keypair() {
        let keypair = generate_static_keypair();
        assert_eq!(keypair.public_key_bytes().len(), PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_generate_ephemeral_keypair() {
        let keypair = generate_ephemeral_keypair();
        assert_eq!(keypair.public_key_bytes().len(), PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_static_key_exchange() {
        let alice = generate_static_keypair();
        let bob = generate_static_keypair();

        let alice_shared = exchange_static(&alice, &X25519PublicKey(bob.public_key));
        let bob_shared = exchange_static(&bob, &X25519PublicKey(alice.public_key));

        assert_eq!(
            alice_shared.to_bytes(),
            bob_shared.to_bytes()
        );
    }

    #[test]
    fn test_ephemeral_key_exchange() {
        let alice = generate_ephemeral_keypair();
        let bob = generate_ephemeral_keypair();

        let alice_shared = exchange_ephemeral(&alice, &X25519PublicKey(bob.public_key));
        let bob_shared = exchange_ephemeral(&bob, &X25519PublicKey(alice.public_key));

        assert_eq!(
            alice_shared.to_bytes(),
            bob_shared.to_bytes()
        );
    }

    #[test]
    fn test_static_ephemeral_key_exchange() {
        let alice_static = generate_static_keypair();
        let bob_ephemeral = generate_ephemeral_keypair();

        let alice_shared = exchange_static(&alice_static, &X25519PublicKey(bob_ephemeral.public_key));
        let bob_shared = exchange_ephemeral(&bob_ephemeral, &X25519PublicKey(alice_static.public_key));

        assert_eq!(
            alice_shared.to_bytes(),
            bob_shared.to_bytes()
        );
    }

    #[test]
    fn test_derive_shared_secret() {
        let alice = generate_static_keypair();
        let bob = generate_static_keypair();

        let shared = derive_shared_secret(
            &alice.reusable_secret.to_bytes(),
            &bob.public_key.to_bytes(),
        )
        .unwrap();

        let expected = exchange_static(&alice, &X25519PublicKey(bob.public_key));
        assert_eq!(shared.to_bytes(), expected.to_bytes());
    }

    #[test]
    fn test_public_key_roundtrip() {
        let keypair = generate_static_keypair();
        let public_key_bytes = keypair.public_key_bytes();
        let public_key_restored = X25519PublicKey::from_bytes(&public_key_bytes).unwrap();
        assert_eq!(public_key_restored.to_bytes(), public_key_bytes);
    }

    #[test]
    fn test_from_private_key() {
        let keypair = generate_static_keypair();
        let private_key_bytes = keypair.into_private_key_bytes();
        let keypair_restored = X25519StaticKeypair::from_private_key(&private_key_bytes).unwrap();
        assert_eq!(
            keypair_restored.public_key_bytes(),
            keypair.public_key_bytes()
        );
    }
}
