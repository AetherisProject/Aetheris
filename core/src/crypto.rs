#![allow(dead_code)]
use oqs::kyber;
use secrecy::SecretVec;
use std::error::Error;

pub struct CryptoEngine {
    secret_key: SecretVec<u8>,
}

impl CryptoEngine {
    pub fn new() -> Result<Self, String> {
        let (_, sk) = kyber::keypair().map_err(|e| format!("Failed to generate Kyber keypair: {}", e))?;
        Ok(Self {
            secret_key: SecretVec::from(sk.to_bytes()),
        })
    }
    
    pub fn generate_kyber_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), String> {
        let (pk, sk) = kyber::keypair().map_err(|e| format!("Failed to generate keypair: {}", e))?;
        Ok((pk.to_bytes().to_vec(), sk.to_bytes().to_vec()))
    }
    
    pub fn hybrid_encrypt(&self, data: &[u8], public_key: &[u8]) -> Result<Vec<u8>, String> {
        let ciphertext = kyber::encrypt(public_key, data).map_err(|e| format!("Encryption failed: {}", e))?;
        Ok(ciphertext.to_bytes().to_vec())
    }
    
    pub fn hybrid_decrypt(&self, ciphertext: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, String> {
        let plaintext = kyber::decrypt(secret_key, ciphertext).map_err(|e| format!("Decryption failed: {}", e))?;
        Ok(plaintext.to_bytes().to_vec())
    }
}