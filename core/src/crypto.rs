#![allow(dead_code)]
use oqs::kem::{Algorithm, Kem};
use secrecy::SecretVec;

const KEM_ALGORITHM: Algorithm = Algorithm::Kyber512;

pub struct CryptoEngine {
    secret_key: SecretVec<u8>,
}

impl CryptoEngine {
    pub fn new() -> Result<Self, String> {
        oqs::init();
        let kem = Kem::new(KEM_ALGORITHM).map_err(|e| format!("Failed to initialize Kyber KEM: {}", e))?;
        let (_, sk) = kem.keypair().map_err(|e| format!("Failed to generate Kyber keypair: {}", e))?;
        Ok(Self {
            secret_key: SecretVec::from(sk.into_vec()),
        })
    }

    pub fn generate_kyber_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), String> {
        let kem = Kem::new(KEM_ALGORITHM).map_err(|e| format!("Failed to initialize Kyber KEM: {}", e))?;
        let (pk, sk) = kem.keypair().map_err(|e| format!("Failed to generate keypair: {}", e))?;
        Ok((pk.into_vec(), sk.into_vec()))
    }

    fn derive_keystream(shared_secret: &[u8], len: usize) -> Vec<u8> {
        let mut keystream = Vec::with_capacity(len);
        let mut i: u64 = 0;
        while keystream.len() < len {
            let mut block = shared_secret.to_vec();
            block.extend_from_slice(&i.to_le_bytes());
            let digest = simple_hash(&block);
            keystream.extend_from_slice(&digest);
            i += 1;
        }
        keystream.truncate(len);
        keystream
    }

    pub fn hybrid_encrypt(&self, data: &[u8], public_key: &[u8]) -> Result<Vec<u8>, String> {
        let kem = Kem::new(KEM_ALGORITHM).map_err(|e| format!("Failed to initialize Kyber KEM: {}", e))?;
        let pk = kem.public_key_from_bytes(public_key).ok_or("Invalid public key length")?;
        let (ct, ss) = kem.encapsulate(&pk).map_err(|e| format!("Encapsulation failed: {}", e))?;
        let ss = ss.into_vec();
        let keystream = Self::derive_keystream(&ss, data.len());
        let mut output = ct.into_vec();
        for (i, byte) in data.iter().enumerate() {
            output.push(byte ^ keystream[i]);
        }
        Ok(output)
    }

    pub fn hybrid_decrypt(&self, ciphertext: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, String> {
        let kem = Kem::new(KEM_ALGORITHM).map_err(|e| format!("Failed to initialize Kyber KEM: {}", e))?;
        let ct_len = kem.length_ciphertext();
        if ciphertext.len() < ct_len {
            return Err("Ciphertext too short".to_string());
        }
        let sk = kem.secret_key_from_bytes(secret_key).ok_or("Invalid secret key length")?;
        let ct = kem.ciphertext_from_bytes(&ciphertext[..ct_len]).ok_or("Invalid ciphertext length")?;
        let ss = kem.decapsulate(&sk, &ct).map_err(|e| format!("Decapsulation failed: {}", e))?;
        let ss = ss.into_vec();
        let keystream = Self::derive_keystream(&ss, ciphertext.len() - ct_len);
        let mut plaintext = Vec::with_capacity(keystream.len());
        for (i, byte) in ciphertext[ct_len..].iter().enumerate() {
            plaintext.push(byte ^ keystream[i]);
        }
        Ok(plaintext)
    }
}

impl Default for CryptoEngine {
    fn default() -> Self {
        Self::new().expect("Failed to initialize CryptoEngine")
    }
}

fn simple_hash(data: &[u8]) -> Vec<u8> {
    let mut state: u64 = 0xcbf29ce484222325;
    for byte in data {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100000001b3);
    }
    state.to_le_bytes().to_vec()
}