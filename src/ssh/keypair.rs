//! SSH Keypair management for Aetheris.
//!
//! Provides key generation (Ed25519, RSA), formatting, OpenSSH export,
//! fingerprint calculation, and secure zeroization of private keys.

use anyhow::{bail, Context, Result};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Supported SSH key algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeyType {
    /// Ed25519 high-security Edwards curve (recommended).
    Ed25519,
    /// Standard RSA (2048, 3072, or 4096 bits).
    Rsa,
}

/// Secure SSH Keypair representation with zeroize-on-drop protection.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SshKeypair {
    /// Key algorithm type (not zeroized directly).
    #[zeroize(skip)]
    pub key_type: KeyType,
    /// Raw private key bytes.
    pub private_key_bytes: Vec<u8>,
    /// Raw public key bytes.
    #[zeroize(skip)]
    pub public_key_bytes: Vec<u8>,
    /// Optional comment for OpenSSH public key format (e.g. user@host).
    #[zeroize(skip)]
    pub comment: String,
}

impl std::fmt::Debug for SshKeypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SshKeypair")
            .field("key_type", &self.key_type)
            .field("public_key_bytes_len", &self.public_key_bytes.len())
            .field("comment", &self.comment)
            .field("private_key", &"[REDACTED]")
            .finish()
    }
}

impl SshKeypair {
    /// Generate a new Ed25519 keypair with an optional comment.
    pub fn generate_ed25519(comment: Option<&str>) -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Self {
            key_type: KeyType::Ed25519,
            private_key_bytes: signing_key.to_bytes().to_vec(),
            public_key_bytes: verifying_key.to_bytes().to_vec(),
            comment: comment.unwrap_or("aetheris-agent").to_string(),
        }
    }

    /// Compute the standard SHA256 public key fingerprint (base64 encoded, e.g. SHA256:...).
    pub fn fingerprint_sha256(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.public_key_bytes);
        let hash = hasher.finalize();
        format!("SHA256:{}", base64::Engine::encode(&base64::engine::general_purpose::STANDARD_NO_PAD, hash))
    }

    /// Export the public key in standard OpenSSH format (`ssh-ed25519 AAAA... comment`).
    pub fn to_openssh_public_key(&self) -> Result<String> {
        match self.key_type {
            KeyType::Ed25519 => {
                // OpenSSH wire format for ed25519:
                // string "ssh-ed25519"
                // string public_key (32 bytes)
                let mut wire = Vec::new();
                append_ssh_string(&mut wire, b"ssh-ed25519");
                append_ssh_string(&mut wire, &self.public_key_bytes);

                let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, wire);
                Ok(format!("ssh-ed25519 {} {}", b64, self.comment))
            }
            KeyType::Rsa => bail!("RSA OpenSSH format export not currently implemented"),
        }
    }

    /// Export raw private key for memory-only injection into SSH clients or process env.
    pub fn raw_private_key(&self) -> &[u8] {
        &self.private_key_bytes
    }
}

/// Helper to serialize SSH wire string (4-byte length prefix + data).
fn append_ssh_string(buf: &mut Vec<u8>, data: &[u8]) {
    let len = (data.len() as u32).to_be_bytes();
    buf.extend_from_slice(&len);
    buf.extend_from_slice(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ed25519() {
        let keypair = SshKeypair::generate_ed25519(Some("user@aetheris"));
        assert_eq!(keypair.key_type, KeyType::Ed25519);
        assert_eq!(keypair.private_key_bytes.len(), 32);
        assert_eq!(keypair.public_key_bytes.len(), 32);
        assert_eq!(keypair.comment, "user@aetheris");
    }

    #[test]
    fn test_openssh_public_key_export() {
        let keypair = SshKeypair::generate_ed25519(Some("test@cluster"));
        let pub_str = keypair.to_openssh_public_key().expect("export openssh");
        assert!(pub_str.starts_with("ssh-ed25519 "));
        assert!(pub_str.ends_with(" test@cluster"));
    }

    #[test]
    fn test_fingerprint() {
        let keypair = SshKeypair::generate_ed25519(None);
        let fp = keypair.fingerprint_sha256();
        assert!(fp.starts_with("SHA256:"));
    }
}
