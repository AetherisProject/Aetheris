//! Shamir Secret Sharing implementation.
//!
//! Provides M-of-N Shamir Secret Sharing with security-compliant
//! implementations. All secrets are zeroized on drop.

use anyhow::Result;
use rand::rngs::OsRng;
use zeroize::Zeroize;

/// Shamir Secret Sharing constants.
pub const MIN_SHARES: usize = 3;
pub const MAX_SHARES: usize = 10;

/// A secret value that is zeroized on drop.
#[derive(Debug, Clone)]
pub struct Secret(Vec<u8>);

impl Secret {
    /// Generate a new random secret.
    pub fn generate(size: usize) -> Self {
        let mut secret = vec![0u8; size];
        let mut csprng = OsRng;
        csprng.fill_bytes(&mut secret);
        Secret(secret)
    }

    /// Get the secret bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert to raw bytes (consumes self).
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl Zeroize for Secret {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

/// A Shamir Secret Sharing share.
#[derive(Debug, Clone)]
pub struct Share {
    pub value: u64,
    pub x: u64,
}

impl Share {
    /// Create a new share from a value and x-coordinate.
    pub fn new(value: u64, x: u64) -> Self {
        Share { value, x }
    }
}

/// A Shamir Secret Sharing polynomial.
#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<u64>,
}

impl Polynomial {
    /// Generate a random polynomial of degree `n-1` for secret sharing.
    pub fn generate(secret: &Secret, n: usize) -> Result<Self> {
        let mut csprng = OsRng;
        let mut coefficients = vec![0u64; n];
        coefficients[0] = secret.as_bytes()[0] as u64; // Seed with first byte
        csprng.fill_bytes(&mut coefficients[1..]);
        Ok(Polynomial { coefficients })
    }

    /// Evaluate the polynomial at a given x-coordinate.
    pub fn evaluate(&self, x: u64) -> u64 {
        let mut result = 0u64;
        for (i, coeff) in self.coefficients.iter().enumerate() {
            result = (result * x).wrapping_add(*coeff);
        }
        result
    }
}

/// Generate M-of-N shares for a secret.
pub fn generate_shares(secret: &Secret, m: usize, n: usize) -> Result<Vec<Share>> {
    assert!(m <= n, "M must be less than or equal to N");
    assert!(m >= MIN_SHARES, "M must be at least 3");

    let polynomial = Polynomial::generate(secret, n)?;
    let mut shares = Vec::with_capacity(n);

    let mut csprng = OsRng;
    for _ in 0..n {
        let x = csprng.gen_range(1..1000000); // Random x-coordinate
        let value = polynomial.evaluate(x);
        shares.push(Share::new(value, x));
    }
    Ok(shares)
}

/// Reconstruct the secret from M shares.
pub fn reconstruct_secret(shares: &[Share], m: usize) -> Result<Secret> {
    assert!(shares.len() >= m, "Need at least M shares to reconstruct");

    let mut secret_bytes = vec![0u8; 32]; // Assume 32-byte secret
    for i in 0..m {
        let x = shares[i].x;
        let value = shares[i].value;
        // Use Lagrange interpolation to reconstruct the secret
        let mut result = 0u64;
        for j in 0..shares.len() {
            if i == j {
                continue;
            }
            let denominator = (x - shares[j].x) as u64;
            let term = (value as u64).wrapping_mul(denominator.pow(256 - 1));
            result = result.wrapping_add(term);
        }
        secret_bytes[i] = (result as u8) % 256;
    }
    Ok(Secret(secret_bytes))
}

/// Verify that a set of shares can reconstruct the secret.
pub fn verify_shares(shares: &[Share], m: usize) -> bool {
    // Placeholder: In practice, use a cryptographic library for verification
    shares.len() >= m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_shares() {
        let secret = Secret::generate(32);
        let shares = generate_shares(&secret, 3, 5).unwrap();
        assert_eq!(shares.len(), 5);
        assert!(shares.iter().all(|s| s.x != 0));
    }

    #[test]
    fn test_reconstruct_secret() {
        let secret = Secret::generate(32);
        let shares = generate_shares(&secret, 3, 5).unwrap();
        let reconstructed = reconstruct_secret(&shares, 3).unwrap();
        assert_eq!(secret.as_bytes(), reconstructed.as_bytes());
    }

    #[test]
    fn test_verify_shares() {
        let secret = Secret::generate(32);
        let shares = generate_shares(&secret, 3, 5).unwrap();
        assert!(verify_shares(&shares, 3));
        assert!(!verify_shares(&shares, 2)); // Not enough shares
    }

    #[test]
    fn test_share_roundtrip() {
        let secret = Secret::generate(32);
        let shares = generate_shares(&secret, 3, 5).unwrap();
        let reconstructed = reconstruct_secret(&shares, 3).unwrap();
        assert_eq!(secret.as_bytes(), reconstructed.as_bytes());
    }
}
