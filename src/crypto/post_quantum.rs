//! Post-quantum cryptography using CRYSTALS-Kyber and CRYSTALS-Dilithium.
//!
//! Provides key encapsulation mechanisms (KEM) and digital signatures using NIST
//! post-quantum cryptography standards.

use anyhow::Result;
use oqs::kem::kyber::{
    generate_keypair,
    encrypt,
    decrypt,
    KyberKeypair,
    KyberPublicKey,
    KyberSecretKey,
    KyberCiphertext,
    KyberSharedSecret,
};
use oqs::sig::dilithium::{
    generate_keypair as dilithium_signature_keypair,
    sign,
    verify,
    DilithiumSignature,
    DilithiumPublicKey,
    DilithiumPrivateKey,
};
use rand::rngs::OsRng;
use zeroize::Zeroize;

/// A wrapper for Kyber KEM keypairs.
#[derive(Debug, Clone)]
pub struct KyberKeypair {
    pub public_key: KyberPublicKey,
    pub secret_key: KyberSecretKey,
}

impl KyberKeypair {
    /// Generate a new Kyber keypair using OsRng.
    pub fn generate() -> Self {
        let (public_key, secret_key) = generate_keypair();
        KyberKeypair {
            public_key,
            secret_key,
        }
    }

    /// Get the public key bytes.
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.to_bytes()
    }

    /// Get the private key bytes (consumes self for security).
    pub fn into_private_key_bytes(self) -> Vec<u8> {
        self.secret_key.to_bytes()
    }
}

impl Zeroize for KyberKeypair {
    fn zeroize(&mut self) {
        self.secret_key.zeroize();
    }
}

/// A wrapper for Kyber KEM public keys.
#[derive(Debug, Clone)]
pub struct KyberPublicKey(pub KyberPublicKey);

impl KyberPublicKey {
    /// Create a new Kyber public key from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let public_key = KyberPublicKey::from_bytes(bytes)
            .map_err(|e| anyhow::anyhow!("Invalid Kyber public key: {}", e))?;
        Ok(Self(public_key))
    }

    /// Get the public key bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

/// A wrapper for Kyber KEM shared secrets.
#[derive(Debug, Clone)]
pub struct KyberSharedSecret(pub KyberSharedSecret);

impl KyberSharedSecret {
    /// Get the shared secret bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

/// A wrapper for Kyber KEM ciphertexts.
#[derive(Debug, Clone)]
pub struct KyberCiphertext(pub KyberCiphertext);

impl KyberCiphertext {
    /// Get the ciphertext bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl KyberCiphertext {
    /// Encrypt a message using Kyber KEM.
    pub fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>> {
        let ciphertext = encrypt(&self.secret_key, &self.public_key, message)?;
        Ok(ciphertext.to_bytes())
    }

    /// Decrypt a message using Kyber KEM.
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let ciphertext = KyberCiphertext::from_bytes(ciphertext)?;
        let plaintext = decrypt(&self.secret_key, &ciphertext)?;
        Ok(plaintext.to_bytes())
    }
}

/// Perform Kyber key exchange.
pub fn kyber_exchange(keypair: &KyberKeypair, remote_public_key: &KyberPublicKey) -> Result<KyberSharedSecret> {
    let shared_secret = kyber_exchange_keypair(keypair, remote_public_key)?;
    Ok(KyberSharedSecret(shared_secret))
}

/// Perform Kyber key exchange using a keypair and remote public key.
pub fn kyber_exchange_keypair(keypair: &KyberKeypair, remote_public_key: &KyberPublicKey) -> Result<KyberSharedSecret> {
    let shared_secret = kyber_exchange_keypair_raw(keypair.secret_key, remote_public_key.public_key)?;
    Ok(KyberSharedSecret(shared_secret))
}

/// Perform Kyber key exchange using raw keys.
pub fn kyber_exchange_keypair_raw(secret_key: KyberSecretKey, remote_public_key: KyberPublicKey) -> Result<KyberSharedSecret> {
    let shared_secret = secret_key.diffie_hellman(&remote_public_key);
    Ok(KyberSharedSecret(shared_secret))
}

/// A wrapper for Dilithium signatures.
#[derive(Debug, Clone)]
pub struct DilithiumSignature(pub DilithiumSignature);

impl DilithiumSignature {
    /// Get the signature bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

/// A wrapper for Dilithium private keys.
#[derive(Debug, Clone)]
pub struct DilithiumPrivateKey(pub DilithiumPrivateKey);

impl DilithiumPrivateKey {
    /// Get the private key bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

/// A wrapper for Dilithium public keys.
#[derive(Debug, Clone)]
pub struct DilithiumPublicKey(pub DilithiumPublicKey);

impl DilithiumPublicKey {
    /// Get the public key bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl DilithiumPublicKey {
    /// Create a new Dilithium public key from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let public_key = DilithiumPublicKey::from_bytes(bytes)
            .map_err(|e| anyhow::anyhow!("Invalid Dilithium public key: {}", e))?;
        Ok(Self(public_key))
    }
}

/// Generate a new Dilithium signature keypair.
pub fn dilithium_signature_keypair() -> (DilithiumPublicKey, DilithiumPrivateKey) {
    dilithium_signature_keypair()
}

/// Sign a message with Dilithium.
pub fn sign_dilithium(message: &[u8], private_key: &DilithiumPrivateKey) -> Result<DilithiumSignature> {
    let signature = sign(message, &private_key.0)?;
    Ok(DilithiumSignature(signature))
}

/// Verify a Dilithium signature.
pub fn verify_dilithium(
    message: &[u8],
    signature: &DilithiumSignature,
    public_key: &DilithiumPublicKey,
) -> Result<bool> {
    let result = verify(message, &signature.0, &public_key.0)?;
    Ok(result)
}

/// Generate a new Kyber keypair.
pub fn generate_kyber_keypair() -> KyberKeypair {
    KyberKeypair::generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_keypair_generation() {
        let keypair = generate_kyber_keypair();
        assert_eq!(keypair.public_key_bytes().len(), 48); // Kyber-768 public key size
    }

    #[test]
    fn test_kyber_exchange() {
        let alice = generate_kyber_keypair();
        let bob = generate_kyber_keypair();

        let alice_shared = kyber_exchange(&alice, &KyberPublicKey(bob.public_key));
        let bob_shared = kyber_exchange(&bob, &KyberPublicKey(alice.public_key));

        assert_eq!(alice_shared.unwrap().to_bytes(), bob_shared.unwrap().to_bytes());
    }

    #[test]
    fn test_dilithium_signature() {
        let (public_key, private_key) = dilithium_signature_keypair();
        let message = b"Test message for Dilithium signature";
        let signature = sign_dilithium(message, &DilithiumPrivateKey(private_key)).unwrap();

        assert!(verify_dilithium(message, &signature, &DilithiumPublicKey(public_key)).unwrap());
    }

    #[test]
    fn test_dilithium_verify_fails() {
        let (public_key, private_key) = dilithium_signature_keypair();
        let message = b"Test message for Dilithium signature";
        let signature = sign_dilithium(message, &DilithiumPrivateKey(private_key)).unwrap();

        let tampered_message = b"Tampered message";
        assert!(!verify_dilithium(tampered_message, &signature, &DilithiumPublicKey(public_key)).unwrap());
    }
}
