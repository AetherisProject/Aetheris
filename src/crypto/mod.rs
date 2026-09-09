//! Cryptography engine for Aetheris.
//!
//! Provides key derivation (Argon2id), authenticated encryption (XChaCha20-Poly1305),
//! digital signatures (Ed25519), key exchange (X25519), Shamir Secret Sharing,
//! post-quantum hybrid mode, memory encryption, and plausible deniability.

pub mod cipher;
pub mod duress;
pub mod hardware;
pub mod key_exchange;
pub mod signatures;
pub mod kdf;
pub mod memory;
pub mod post_quantum;
pub mod shamir;

pub use shamir::{generate_shares, reconstruct_secret, verify_shares, Secret, MIN_SHARES, MAX_SHARES};

use anyhow::Result;

    use oqs::kem::kyber::{KyberKeypair, KyberPublicKey, KyberSecretKey, KyberCiphertext, KyberSharedSecret, generate_keypair, encrypt, decrypt, diffie_hellman};
    use oqs::sig::dilithium::{generate_keypair as dilithium_generate_keypair, sign, verify, DilithiumSignature, DilithiumPublicKey, DilithiumPrivateKey};
    use rand::Rng;
    use hkdf::Hkdf;
    use chacha20poly1305::aead::{Aead, KeyInit, OsRng};

    /// Post-quantum cryptographic primitives.
    pub mod post_quantum {
        use super::*;
        use anyhow::Result;
        use oqs::kem::kyber::{KyberKeypair, KyberPublicKey, KyberSecretKey, KyberCiphertext, KyberSharedSecret};
        use oqs::sig::dilithium::{DilithiumSignature, DilithiumPublicKey, DilithiumPrivateKey};
        use rand::Rng;
        use hkdf::Hkdf;
        use sha2::Sha256;
        use chacha20poly1305::aead::{Aead, KeyInit, OsRng};

        /// Generate a Kyber keypair.
        pub fn generate_kyber_keypair() -> KyberKeypair {
            let keypair = generate_keypair();
            KyberKeypair {
                public_key: KyberPublicKey(keypair.public_key),
                secret_key: KyberSecretKey(keypair.secret_key),
            }
        }

        /// Encrypt a message using Kyber KEM.
        pub fn encrypt_kyber(public_key: &KyberPublicKey, message: &[u8]) -> Result<Vec<u8>> {
            let ciphertext = encrypt(&public_key.public_key, message)?;
            Ok(KyberCiphertext(ciphertext).to_bytes())
        }

        /// Decrypt a message using Kyber KEM.
        pub fn decrypt_kyber(secret_key: &KyberSecretKey, ciphertext: &[u8]) -> Result<Vec<u8>> {
            let ciphertext = KyberCiphertext::from_bytes(ciphertext)?;
            let plaintext = decrypt(&secret_key.secret_key, &ciphertext.ciphertext)?;
            Ok(plaintext.to_bytes())
        }

        /// Perform Kyber key exchange.
        pub fn kyber_exchange(secret_key: &KyberSecretKey, remote_public_key: &KyberPublicKey) -> Result<KyberSharedSecret> {
            let shared_secret = diffie_hellman(&secret_key.secret_key, &remote_public_key.public_key);
            Ok(KyberSharedSecret(shared_secret))
        }

        /// Generate a Dilithium keypair.
        pub fn generate_dilithium_keypair() -> (DilithiumPublicKey, DilithiumPrivateKey) {
            dilithium_generate_keypair()
        }

        /// Sign a message with Dilithium.
        pub fn sign_dilithium(message: &[u8], private_key: &DilithiumPrivateKey) -> Result<DilithiumSignature> {
            let signature = sign(message, &private_key.private_key)?;
            Ok(signature)
        }

        /// Verify a Dilithium signature.
        pub fn verify_dilithium(message: &[u8], signature: &DilithiumSignature, public_key: &DilithiumPublicKey) -> Result<bool> {
            let result = verify(message, &signature.signature, &public_key.public_key)?;
            Ok(result)
        }

        /// Hybrid encryption using Kyber KEM and AES-GCM.
        pub fn hybrid_encrypt(message: &[u8], public_key: &KyberPublicKey) -> Result<(Vec<u8>, Vec<u8>)> {
            let ciphertext = encrypt_kyber(public_key, message)?;
            let shared_secret = kyber_exchange(&KyberSecretKey(ciphertext), public_key)?;
            let symmetric_key = shared_secret.to_bytes();
            let symmetric_key = Hkdf::new(
                &Sha256::new(),
                symmetric_key.as_slice(),
            )
            .derive_key(128);
            let iv = OsRng.gen::<[u8; 12]>();
            let encrypted_message = chacha20poly1305::encrypt(&symmetric_key, &iv, message)?;
            Ok((ciphertext, iv.to_vec()))
        }

        /// Hybrid decryption using Kyber KEM and AES-GCM.
        pub fn hybrid_decrypt(ciphertext: &[u8], iv: &[u8], shared_secret: &KyberSharedSecret, encrypted_message: &[u8]) -> Result<Vec<u8>> {
            let symmetric_key = shared_secret.to_bytes();
            let symmetric_key = Hkdf::new(
                &Sha256::new(),
                symmetric_key.as_slice(),
            )
            .derive_key(128);
            let plaintext = chacha20poly1305::decrypt(&symmetric_key, iv, encrypted_message)?;
            Ok(plaintext)
        }
    }
    
    impl CryptoEngine {
        /// Generate a Kyber keypair.
        pub fn generate_kyber_keypair(&self) -> Result<KyberKeypair> {
            let keypair = post_quantum::generate_kyber_keypair();
            Ok(KyberKeypair {
                public_key: KyberPublicKey(keypair.public_key),
                secret_key: KyberSecretKey(keypair.secret_key),
            })
        }

        /// Encrypt a message using Kyber KEM.
        pub fn kyber_encrypt(&self, public_key: &KyberPublicKey, message: &[u8]) -> Result<Vec<u8>> {
            let ciphertext = post_quantum::encrypt_kyber(public_key, message)?;
            Ok(ciphertext)
        }

        /// Decrypt a message using Kyber KEM.
        pub fn kyber_decrypt(&self, secret_key: &KyberSecretKey, ciphertext: &[u8]) -> Result<Vec<u8>> {
            let plaintext = post_quantum::decrypt_kyber(secret_key, ciphertext)?;
            Ok(plaintext)
        }

        /// Perform Kyber key exchange.
        pub fn kyber_exchange(&self, secret_key: &KyberSecretKey, remote_public_key: &KyberPublicKey) -> Result<KyberSharedSecret> {
            let shared_secret = post_quantum::kyber_exchange(secret_key, remote_public_key)?;
            Ok(shared_secret)
        }

        /// Generate a Dilithium keypair.
        pub fn generate_dilithium_keypair(&self) -> Result<(DilithiumPublicKey, DilithiumPrivateKey)> {
            let keypair = post_quantum::generate_dilithium_keypair();
            Ok(keypair)
        }

        /// Sign a message with Dilithium.
        pub fn sign_dilithium(&self, message: &[u8], private_key: &DilithiumPrivateKey) -> Result<DilithiumSignature> {
            let signature = post_quantum::sign_dilithium(message, private_key)?;
            Ok(signature)
        }

        /// Verify a Dilithium signature.
        pub fn verify_dilithium(&self, message: &[u8], signature: &DilithiumSignature, public_key: &DilithiumPublicKey) -> Result<bool> {
            let result = post_quantum::verify_dilithium(message, signature, public_key)?;
            Ok(result)
        }

        /// Hybrid encryption using Kyber KEM and AES-GCM.
        pub fn hybrid_encrypt(&self, message: &[u8], public_key: &KyberPublicKey) -> Result<(Vec<u8>, Vec<u8>)> {
            let (ciphertext, iv) = post_quantum::hybrid_encrypt(message, public_key)?;
            Ok((ciphertext, iv))
        }

        /// Hybrid decryption using Kyber KEM and AES-GCM.
        pub fn hybrid_decrypt(&self, ciphertext: &[u8], iv: &[u8], shared_secret: &KyberSharedSecret, encrypted_message: &[u8]) -> Result<Vec<u8>> {
            let plaintext = post_quantum::hybrid_decrypt(ciphertext, iv, shared_secret, encrypted_message)?;
            Ok(plaintext)
        }


/// The main crypto engine that provides all cryptographic operations.
    /// Post-quantum cryptographic primitives.
    pub mod post_quantum {
        use super::*;
        use anyhow::Result;
        use oqs::kem::kyber::{KyberKeypair, KyberPublicKey, KyberSecretKey, KyberCiphertext, KyberSharedSecret, generate_keypair, encrypt, decrypt, diffie_hellman};
        use oqs::sig::dilithium::{generate_keypair as dilithium_generate_keypair, sign, verify, DilithiumSignature, DilithiumPublicKey, DilithiumPrivateKey};
        use rand::Rng;
        use hkdf::Hkdf;
        use sha2::Sha256;
        use chacha20poly1305::aead::{Aead, KeyInit, OsRng};

        /// Generate a Kyber keypair.
        pub fn generate_kyber_keypair() -> KyberKeypair {
            let keypair = generate_keypair();
            KyberKeypair {
                public_key: KyberPublicKey(keypair.public_key),
                secret_key: KyberSecretKey(keypair.secret_key),
            }
        }

        /// Encrypt a message using Kyber KEM.
        pub fn encrypt_kyber(public_key: &KyberPublicKey, message: &[u8]) -> Result<Vec<u8>> {
            let ciphertext = encrypt(&public_key.public_key, message)?;
            Ok(KyberCiphertext(ciphertext).to_bytes())
        }

        /// Decrypt a message using Kyber KEM.
        pub fn decrypt_kyber(secret_key: &KyberSecretKey, ciphertext: &[u8]) -> Result<Vec<u8>> {
            let ciphertext = KyberCiphertext::from_bytes(ciphertext)?;
            let plaintext = decrypt(&secret_key.secret_key, &ciphertext.ciphertext)?;
            Ok(plaintext.to_bytes())
        }

        /// Perform Kyber key exchange.
        pub fn kyber_exchange(secret_key: &KyberSecretKey, remote_public_key: &KyberPublicKey) -> Result<KyberSharedSecret> {
            let shared_secret = diffie_hellman(&secret_key.secret_key, &remote_public_key.public_key);
            Ok(KyberSharedSecret(shared_secret))
        }

        /// Generate a Dilithium keypair.
        pub fn generate_dilithium_keypair() -> (DilithiumPublicKey, DilithiumPrivateKey) {
            dilithium_generate_keypair()
        }

        /// Sign a message with Dilithium.
        pub fn sign_dilithium(message: &[u8], private_key: &DilithiumPrivateKey) -> Result<DilithiumSignature> {
            let signature = sign(message, &private_key.private_key)?;
            Ok(signature)
        }

        /// Verify a Dilithium signature.
        pub fn verify_dilithium(message: &[u8], signature: &DilithiumSignature, public_key: &DilithiumPublicKey) -> Result<bool> {
            let result = verify(message, &signature.signature, &public_key.public_key)?;
            Ok(result)
        }

        /// Hybrid encryption using Kyber KEM and AES-GCM.
        pub fn hybrid_encrypt(message: &[u8], public_key: &KyberPublicKey) -> Result<(Vec<u8>, Vec<u8>)> {
            let ciphertext = encrypt_kyber(public_key, message)?;
            let shared_secret = kyber_exchange(&KyberSecretKey(ciphertext), public_key)?;
            let symmetric_key = shared_secret.to_bytes();
            let symmetric_key = Hkdf::new(
                &Sha256::new(),
                symmetric_key.as_slice(),
            )
            .derive_key(128);
            let iv = OsRng.gen::<[u8; 12]>();
            let encrypted_message = chacha20poly1305::encrypt(&symmetric_key, &iv, message)?;
            Ok((ciphertext, iv.to_vec()))
        }

        /// Hybrid decryption using Kyber KEM and AES-GCM.
        pub fn hybrid_decrypt(ciphertext: &[u8], iv: &[u8], shared_secret: &KyberSharedSecret, encrypted_message: &[u8]) -> Result<Vec<u8>> {
            let symmetric_key = shared_secret.to_bytes();
            let symmetric_key = Hkdf::new(
                &Sha256::new(),
                symmetric_key.as_slice(),
            )
            .derive_key(128);
            let plaintext = chacha20poly1305::decrypt(&symmetric_key, iv, encrypted_message)?;
            Ok(plaintext)
        }
    }
    
    impl CryptoEngine {
        /// Generate a Kyber keypair.
        pub fn generate_kyber_keypair(&self) -> Result<KyberKeypair> {
            let keypair = post_quantum::generate_kyber_keypair();
            Ok(KyberKeypair {
                public_key: KyberPublicKey(keypair.public_key),
                secret_key: KyberSecretKey(keypair.secret_key),
            })
        }

        /// Encrypt a message using Kyber KEM.
        pub fn kyber_encrypt(&self, public_key: &KyberPublicKey, message: &[u8]) -> Result<Vec<u8>> {
            let ciphertext = post_quantum::encrypt_kyber(public_key, message)?;
            Ok(ciphertext)
        }

        /// Decrypt a message using Kyber KEM.
        pub fn kyber_decrypt(&self, secret_key: &KyberSecretKey, ciphertext: &[u8]) -> Result<Vec<u8>> {
            let plaintext = post_quantum::decrypt_kyber(secret_key, ciphertext)?;
            Ok(plaintext)
        }

        /// Perform Kyber key exchange.
        pub fn kyber_exchange(&self, secret_key: &KyberSecretKey, remote_public_key: &KyberPublicKey) -> Result<KyberSharedSecret> {
            let shared_secret = post_quantum::kyber_exchange(secret_key, remote_public_key)?;
            Ok(shared_secret)
        }

        /// Generate a Dilithium keypair.
        pub fn generate_dilithium_keypair(&self) -> Result<(DilithiumPublicKey, DilithiumPrivateKey)> {
            let keypair = post_quantum::generate_dilithium_keypair();
            Ok(keypair)
        }

        /// Sign a message with Dilithium.
        pub fn sign_dilithium(&self, message: &[u8], private_key: &DilithiumPrivateKey) -> Result<DilithiumSignature> {
            let signature = post_quantum::sign_dilithium(message, private_key)?;
            Ok(signature)
        }

        /// Verify a Dilithium signature.
        pub fn verify_dilithium(&self, message: &[u8], signature: &DilithiumSignature, public_key: &DilithiumPublicKey) -> Result<bool> {
            let result = post_quantum::verify_dilithium(message, signature, public_key)?;
            Ok(result)
        }

        /// Hybrid encryption using Kyber KEM and AES-GCM.
        pub fn hybrid_encrypt(&self, message: &[u8], public_key: &KyberPublicKey) -> Result<(Vec<u8>, Vec<u8>)> {
            let (ciphertext, iv) = post_quantum::hybrid_encrypt(message, public_key)?;
            Ok((ciphertext, iv))
        }

        /// Hybrid decryption using Kyber KEM and AES-GCM.
        pub fn hybrid_decrypt(&self, ciphertext: &[u8], iv: &[u8], shared_secret: &KyberSharedSecret, encrypted_message: &[u8]) -> Result<Vec<u8>> {
            let plaintext = post_quantum::hybrid_decrypt(ciphertext, iv, shared_secret, encrypted_message)?;
            Ok(plaintext)
        }
    }
    
    /// Remove duplicate imports.
    ///
    /// Remove the following lines:
    use oqs::sig::dilithium::{generate_keypair as dilithium_generate_keypair, sign, verify, DilithiumSignature, DilithiumPublicKey, DilithiumPrivateKey};
    use rand::Rng;
    use hkdf::Hkdf;
    use sha2::Sha256;
    use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
    
    /// Re-add imports for other modules.
    use cipher::EncryptionKey;
    use cipher::Ciphertext;
    use cipher::generate_nonce;
    use cipher::chacha20poly1305;
    use cipher::chacha20poly1305::aead;
    use cipher::chacha20poly1305::aead::Aead;
    
    /// Re-add the rest of the imports for other modules.
    use signatures::Ed25519Keypair;
    use signatures::signatures;
    
    /// Remove the post-quantum module from the imports list.
    // Remove the following lines:
    // use oqs::kem::kyber::{KyberKeypair, KyberPublicKey, KyberSecretKey, KyberCiphertext, KyberSharedSecret, generate_keypair, encrypt, decrypt, diffie_hellman};
    //
    // Restore the original imports.
    use anyhow::Result;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use zeroize::Zeroize;
    
    // Restore the rest of the imports from the original file
    use cipher::EncryptionKey;
    use cipher::Ciphertext;
    use cipher::generate_nonce;
    use cipher::chacha20poly1305;
    use cipher::chacha20poly1305::aead;
    use cipher::hkdf;
    use cipher::hkdf::Hkdf;
    use cipher::sha2;
    use cipher::sha2::Sha256;
    use cipher::rand;
    use cipher::rand::Rng;
    use cipher::zeroize;
    use cipher::zeroize::Zeroize;
    
    // Remove the redundant post-quantum module from the imports list
    // and redefine the imports as needed for the rest of the file.
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
    pub fn encrypt(
        &self,
        plaintext: &[u8],
        key: &EncryptionKey,
        nonce: &[u8],
    ) -> Result<Ciphertext> {
        cipher::encrypt(plaintext, key, nonce)
    }

    /// Decrypt data using XChaCha20-Poly1305.
    pub fn decrypt(
        &self,
        ciphertext: &Ciphertext,
        key: &EncryptionKey,
        nonce: &[u8],
    ) -> Result<Vec<u8>> {
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

    /// Generate a new Ed25519 keypair.
    pub fn generate_ed25519_keypair(&self) -> Ed25519Keypair {
        signatures::generate_keypair()
    }
    /// Encrypt memory using authenticated encryption.
    pub fn encrypt_memory(&self, key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = self.derive_key(key)?;
        let iv = rand::OsRng.gen::<[u8; 12]>();
        let cipher = chacha20poly1305::Chacha20Poly1305::new(&key);
        let ciphertext = cipher.encrypt(&iv, plaintext)?;
        Ok((iv.to_vec(), ciphertext))
    }
    /// Split the master key into shares for Duress mode.
    pub fn split_master_key(&self, key: &[u8], threshold: usize) -> Result<(Vec<u8>, Vec<(Vec<u8>, usize)>)> {
        let shares = self.split_master_key_implementation(key, threshold)?;
        Ok((key.to_vec(), shares))
    }
    /// Reconstruct the master key from shares.
    pub fn reconstruct_master_key(&self, shares: &[(Vec<u8>, usize)], threshold: usize) -> Result<Vec<u8>> {
        let reconstructed = self.reconstruct_master_key_implementation(shares, threshold)?;
        Ok(reconstructed)
    }

    /// Reconstruct the master key from shares.
    pub fn reconstruct_master_key(&self, shares: &[(Vec<u8>, usize)], threshold: usize) -> Result<Vec<u8>> {
        let reconstructed = self.reconstruct_master_key_implementation(shares, threshold)?;
        Ok(reconstructed)
    }

    /// Internal method to split the master key into shares.

    /// Internal method to reconstruct the master key.
    fn reconstruct_master_key_implementation(&self, shares: &[(Vec<u8>, usize)], threshold: usize) -> Result<Vec<u8>> {
        let reconstructed = self.duress::reconstruct_master_key(shares, threshold)?;
        Ok(reconstructed)
    }

    /// Use a placeholder for actual Shamir Secret Sharing logic.
    fn reconstruct_share(&self, key: &[u8], share: &[u8], x: u64, threshold: usize) -> Vec<u8> {
        self.duress::reconstruct_share(key, share, x, threshold)
    }

    /// Decrypt memory using authenticated encryption.
    pub fn decrypt_memory(&self, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let key = self.derive_key(key)?;
        let cipher = chacha20poly1305::Chacha20Poly1305::new(&key);
        let plaintext = cipher.decrypt(iv, ciphertext)?;
        Ok(plaintext)
    }

    /// Derive a key from a raw key using HKDF.
    fn derive_key(&self, key: &[u8]) -> Result<[u8; 32]> {
        let mut key_bytes = key.to_vec();
        if key_bytes.len() < 32 {
            key_bytes.resize(32, 0);
        }
        let derived_key = hkdf::Hkdf::<sha2::Sha256>::new(
            &sha2::Sha256::new(),
            key_bytes.as_slice(),
        )
        .derive_key(32);
        Ok(derived_key)
    }


    /// Generate a new Ed25519 seed.
    pub fn generate_ed25519_seed(&self) -> [u8; SEED_SIZE] {
        signatures::generate_seed()
    }

    /// Sign a message with an Ed25519 keypair.
    pub fn sign_ed25519(&self, keypair: &Ed25519Keypair, message: &[u8]) -> Ed25519Signature {
        signatures::sign_message(keypair, message)
    }

    /// Verify an Ed25519 signature with a public key.
    pub fn verify_ed25519(
        &self,
        public_key: &Ed25519PublicKey,
        message: &[u8],
        signature: &Ed25519Signature,
    ) -> bool {
        signatures::verify_signature(public_key, message, signature)
    }

    /// Generate a new X25519 static keypair.
    pub fn generate_x25519_static_keypair(&self) -> X25519StaticKeypair {
        key_exchange::generate_static_keypair()
    }

    /// Generate a new X25519 ephemeral keypair.
    pub fn generate_x25519_ephemeral_keypair(&self) -> X25519EphemeralKeypair {
        key_exchange::generate_ephemeral_keypair()
    }

    /// Perform X25519 key exchange with a static keypair.
    pub fn exchange_x25519_static(
        &self,
        keypair: &X25519StaticKeypair,
        remote_public_key: &X25519PublicKey,
    ) -> X25519SharedSecret {
        key_exchange::exchange_static(keypair, remote_public_key)
    }

    /// Perform X25519 key exchange with an ephemeral keypair.
    pub fn exchange_x25519_ephemeral(
        &self,
        keypair: &X25519EphemeralKeypair,
        remote_public_key: &X25519PublicKey,
    ) -> X25519SharedSecret {
        key_exchange::exchange_ephemeral(keypair, remote_public_key)
    }
}

    /// Generate M-of-N shares for a secret.
    pub fn generate_shamir_shares(&self, secret: &[u8], m: usize, n: usize) -> Result<Vec<(u64, u64)>> {
        let secret = Secret::generate(secret.len())
            .map_err(|e| anyhow::anyhow!("Failed to generate secret: {}", e))?;
        let shares = shamir::generate_shares(&secret, m, n)
            .map(|shares| shares.into_iter().map(|s| (s.value, s.x)).collect())
            .map_err(|e| anyhow::anyhow!("Failed to generate shares: {}", e));
        Ok(shares)
    }

    /// Reconstruct a secret from M shares.
    pub fn reconstruct_shamir_secret(&self, shares: &[(u64, u64)], m: usize) -> Result<Vec<u8>> {
        let reconstructed = shamir::reconstruct_secret(shares, m)
            .map(|secret| secret.into_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to reconstruct secret: {}", e));
        Ok(reconstructed)
    }

    /// Verify that a set of shares can reconstruct the secret.
    pub fn verify_shamir_shares(&self, shares: &[(u64, u64)], m: usize) -> bool {
        let share_list = shares.to_vec();
        shamir::verify_shares(&share_list, m)
    }


    /// Generate a Kyber keypair.
    pub fn generate_kyber_keypair(&self) -> Result<KyberKeypair> {
        let keypair = oqs::kem::kyber::generate_keypair();
        Ok(KyberKeypair {
            public_key: KyberPublicKey(keypair.public_key),
            secret_key: KyberSecretKey(keypair.secret_key),
        })
    }

    /// Encrypt a message using Kyber KEM.
    pub fn kyber_encrypt(&self, public_key: &KyberPublicKey, message: &[u8]) -> Result<Vec<u8>> {
        let ciphertext = oqs::kem::kyber::encrypt(&public_key.public_key, message)?;
        Ok(KyberCiphertext(ciphertext).to_bytes())
    }

    /// Decrypt a message using Kyber KEM.
    pub fn kyber_decrypt(&self, secret_key: &KyberSecretKey, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let ciphertext = KyberCiphertext::from_bytes(ciphertext)?;
        let plaintext = oqs::kem::kyber::decrypt(&secret_key.secret_key, &ciphertext.ciphertext)?;
        Ok(plaintext.to_bytes())
    }

    /// Perform Kyber key exchange.
    pub fn kyber_exchange(&self, secret_key: &KyberSecretKey, remote_public_key: &KyberPublicKey) -> Result<KyberSharedSecret> {
        let shared_secret = oqs::kem::kyber::diffie_hellman(&secret_key.secret_key, &remote_public_key.public_key);
        Ok(KyberSharedSecret(shared_secret))
    }

    /// Generate a Dilithium keypair.
    pub fn generate_dilithium_keypair(&self) -> Result<(DilithiumPublicKey, DilithiumPrivateKey)> {
        let keypair = oqs::sig::dilithium::generate_keypair();
        Ok((
            DilithiumPublicKey(keypair.public_key),
            DilithiumPrivateKey(keypair.private_key),
        ))
    }

    /// Sign a message with Dilithium.
    pub fn sign_dilithium(&self, message: &[u8], private_key: &DilithiumPrivateKey) -> Result<DilithiumSignature> {
        let signature = oqs::sig::dilithium::sign(message, &private_key.private_key)?;
        Ok(DilithiumSignature(signature))
    }

    /// Verify a Dilithium signature.
    pub fn verify_dilithium(&self, message: &[u8], signature: &DilithiumSignature, public_key: &DilithiumPublicKey) -> Result<bool> {
        let result = oqs::sig::dilithium::verify(message, &signature.signature, &public_key.public_key)?;
        Ok(result)
    }

    /// Hybrid encryption using Kyber KEM and AES-GCM.
    pub fn hybrid_encrypt(&self, message: &[u8], public_key: &KyberPublicKey) -> Result<(Vec<u8>, Vec<u8>)> {
        let ciphertext = self.kyber_encrypt(public_key, message)?;
        let shared_secret = self.kyber_exchange(&KyberSecretKey(ciphertext), public_key)?;
        let symmetric_key = shared_secret.to_bytes();
        let symmetric_key = hkdf::Hkdf::new(
            &sha2::Sha256::new(),
            symmetric_key.as_slice(),
        )
        .derive_key(128);
        let iv = rand::rngs::OsRng.gen::<[u8; 12]>();
        let encrypted_message = chacha20poly1305::encrypt(&symmetric_key, &iv, message)?;
        Ok((ciphertext, iv.to_vec()))
    }

    /// Hybrid decryption using Kyber KEM and AES-GCM.
    pub fn hybrid_decrypt(&self, ciphertext: &[u8], iv: &[u8], shared_secret: &KyberSharedSecret, encrypted_message: &[u8]) -> Result<Vec<u8>> {
        let symmetric_key = shared_secret.to_bytes();
        let symmetric_key = hkdf::Hkdf::new(
            &sha2::Sha256::new(),
            symmetric_key.as_slice(),
        )
        .derive_key(128);
        let plaintext = chacha20poly1305::decrypt(&symmetric_key, iv, encrypted_message)?;
        Ok(plaintext)
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
    /// Derive a key from a raw key using HKDF.
    fn derive_key(&self, key: &[u8]) -> Result<[u8; 32]> {
        let mut key_bytes = key.to_vec();
        if key_bytes.len() < 32 {
            key_bytes.resize(32, 0);
        }
        let derived_key = hkdf::Hkdf::<sha2::Sha256>::new(
            &sha2::Sha256::new(),
            key_bytes.as_slice(),
        )
        .derive_key(32);
        Ok(derived_key)
    }
    /// Encrypt memory using authenticated encryption.
    pub fn encrypt_memory(&self, key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = self.derive_key(key)?;
        let iv = rand::OsRng.gen::<[u8; 12]>();
        let cipher = chacha20poly1305::Chacha20Poly1305::new(&key);
        let ciphertext = cipher.encrypt(&iv, plaintext)?;
        Ok((iv.to_vec(), ciphertext))
    }

        let key = self.derive_key(key)?;
        let iv = rand::OsRng.gen::<[u8; 12]>();
        let cipher = chacha20poly1305::Chacha20Poly1305::new(&key);
        let ciphertext = cipher.encrypt(&iv, plaintext)?;
        Ok((iv.to_vec(), ciphertext))
    }

    #[test]
    fn test_secret_vec() {
        let secret = SecretVec::new(vec![1, 2, 3, 4]);
        assert_eq!(secret.expose(), &[1, 2, 3, 4]);
    }

    #[test]
    /// Decrypt memory using authenticated encryption.
    pub fn decrypt_memory(&self, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let key = self.derive_key(key)?;
        let cipher = chacha20poly1305::Chacha20Poly1305::new(&key);
        let plaintext = cipher.decrypt(iv, ciphertext)?;
        Ok(plaintext)
    }
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
