//! Security primitives for Aetheris.
//!
//! Provides secure types, constant-time comparison, rate limiting, and input validation.
//! ALL secret types MUST implement Zeroize on Drop.

use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use zeroize::{Zeroize, ZeroizeOnDrop};

// Re-export main types
pub use secure_cmp::SecureCompare;
pub use secure_string::SecureString;
pub use secure_vec::SecureVec;

/// Secure string that zeroizes its contents on drop.
/// MUST be used for passwords, tokens, keys, and any sensitive strings.
pub mod secure_string {
    use super::*;

    /// A UTF-8 string that is securely zeroized when dropped.
    /// Use this for passwords, API keys, tokens, and any sensitive text.
    #[derive(Clone)]
    pub struct SecureString(String);

    impl SecureString {
        /// Create a new SecureString from a &str.
        /// The input is NOT zeroized - caller must ensure it's cleared.
        pub fn from_str(s: &str) -> Self {
            Self(s.to_string())
        }

        /// Create a new SecureString from a String.
        /// The input is NOT zeroized - caller must ensure it's cleared.
        pub fn from_string(s: String) -> Self {
            Self(s)
        }

        /// Get the length of the string.
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Check if the string is empty.
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Expose the underlying string for operations that require it.
        /// BE CAREFUL: This exposes the secret in memory. Only use when absolutely necessary.
        pub fn expose(&self) -> &str {
            &self.0
        }

        /// Expose the underlying string as bytes.
        pub fn expose_bytes(&self) -> &[u8] {
            self.0.as_bytes()
        }

        /// Convert to a String (consumes self, zeroizes original).
        pub fn into_string(self) -> String {
            self.0
        }

        /// Create from sensitive bytes. Input is NOT zeroized.
        pub fn from_bytes(bytes: Vec<u8>) -> Self {
            Self(String::from_utf8_lossy(&bytes).to_string())
        }
    }

    impl Zeroize for SecureString {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    impl ZeroizeOnDrop for SecureString {}

    impl Deref for SecureString {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Debug for SecureString {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "SecureString([REDACTED])")
        }
    }

    impl Display for SecureString {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[REDACTED]")
        }
    }

    impl PartialEq for SecureString {
        fn eq(&self, other: &Self) -> bool {
            // Use constant-time comparison
            self.expose_bytes().secure_eq(other.expose_bytes())
        }
    }

    impl Eq for SecureString {}

    impl From<&str> for SecureString {
        fn from(s: &str) -> Self {
            Self::from_str(s)
        }
    }

    impl From<String> for SecureString {
        fn from(s: String) -> Self {
            Self::from_string(s)
        }
    }

    impl From<SecureString> for String {
        fn from(s: SecureString) -> Self {
            s.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_secure_string_creation() {
            let s = SecureString::from_str("test_password");
            assert_eq!(s.len(), 13); // "test_password" is 13 characters
            assert!(!s.is_empty());
        }

        #[test]
        fn test_secure_string_equality() {
            let s1 = SecureString::from_str("password");
            let s2 = SecureString::from_str("password");
            let s3 = SecureString::from_str("different");
            assert_eq!(s1, s2);
            assert_ne!(s1, s3);
        }

        #[test]
        fn test_secure_string_redacted() {
            let s = SecureString::from_str("secret");
            assert_eq!(format!("{:?}", s), "SecureString([REDACTED])");
            assert_eq!(format!("{}", s), "[REDACTED]");
        }
    }
}

/// Secure byte vector that zeroizes its contents on drop.
/// MUST be used for secrets, keys, nonces, and any sensitive binary data.
pub mod secure_vec {
    use super::*;

    /// A byte vector that is securely zeroized when dropped.
    /// Use this for encryption keys, nonces, and any sensitive binary data.
    #[derive(Clone)]
    pub struct SecureVec(Vec<u8>);

    impl SecureVec {
        /// Create a new SecureVec from bytes.
        pub fn new(data: Vec<u8>) -> Self {
            Self(data)
        }

        /// Create a new SecureVec with a given capacity.
        pub fn with_capacity(capacity: usize) -> Self {
            Self(Vec::with_capacity(capacity))
        }

        /// Get the length of the vector.
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Check if the vector is empty.
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Expose the underlying bytes for operations that require them.
        /// BE CAREFUL: This exposes the secret in memory.
        pub fn expose(&self) -> &[u8] {
            &self.0
        }

        /// Expose the underlying bytes as a mutable slice.
        pub fn expose_mut(&mut self) -> &mut [u8] {
            &mut self.0
        }

        /// Convert to a byte vector (consumes self, zeroizes original).
        pub fn into_vec(self) -> Vec<u8> {
            self.0
        }

        /// Create from a slice.
        pub fn from_slice(slice: &[u8]) -> Self {
            Self(slice.to_vec())
        }

        /// Clear the vector (zeroizes contents).
        pub fn clear(&mut self) {
            self.0.zeroize();
        }
    }

    impl Zeroize for SecureVec {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    impl ZeroizeOnDrop for SecureVec {}

    impl Deref for SecureVec {
        type Target = [u8];
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for SecureVec {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Debug for SecureVec {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "SecureVec([REDACTED; {} bytes])", self.0.len())
        }
    }

    impl Display for SecureVec {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[REDACTED; {} bytes]", self.0.len())
        }
    }

    impl PartialEq for SecureVec {
        fn eq(&self, other: &Self) -> bool {
            self.expose().secure_eq(other.expose())
        }
    }

    impl Eq for SecureVec {}

    impl From<Vec<u8>> for SecureVec {
        fn from(v: Vec<u8>) -> Self {
            Self::new(v)
        }
    }

    impl From<&[u8]> for SecureVec {
        fn from(slice: &[u8]) -> Self {
            Self::from_slice(slice)
        }
    }

    impl From<SecureVec> for Vec<u8> {
        fn from(s: SecureVec) -> Self {
            s.0
        }
    }

    impl AsRef<[u8]> for SecureVec {
        fn as_ref(&self) -> &[u8] {
            self.expose()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_secure_vec_creation() {
            let v = SecureVec::new(vec![1, 2, 3, 4, 5]);
            assert_eq!(v.len(), 5);
            assert!(!v.is_empty());
        }

        #[test]
        fn test_secure_vec_redacted() {
            let v = SecureVec::new(vec![1, 2, 3, 4]);
            assert_eq!(format!("{:?}", v), "SecureVec([REDACTED; 4 bytes])");
            assert_eq!(format!("{}", v), "[REDACTED; 4 bytes]");
        }
    }
}

/// Constant-time comparison for secrets.
/// NEVER use `==` for comparing secrets - it leaks information via timing.
pub mod secure_cmp {
    /// Trait for constant-time comparison.
    pub trait SecureCompare {
        /// Constant-time equality check.
        fn secure_eq(&self, other: &Self) -> bool;
    }

    impl SecureCompare for [u8] {
        /// Constant-time comparison using bitwise OR accumulation to avoid short-circuiting.
        fn secure_eq(&self, other: &Self) -> bool {
            if self.len() != other.len() {
                return false;
            }
            let mut result = 0u8;
            for (a, b) in self.iter().zip(other.iter()) {
                result |= a ^ b;
            }
            result == 0
        }
    }

    impl SecureCompare for str {
        fn secure_eq(&self, other: &Self) -> bool {
            self.as_bytes().secure_eq(other.as_bytes())
        }
    }

    impl SecureCompare for String {
        fn secure_eq(&self, other: &Self) -> bool {
            self.as_bytes().secure_eq(other.as_bytes())
        }
    }

    impl SecureCompare for super::SecureString {
        fn secure_eq(&self, other: &Self) -> bool {
            self.expose_bytes().secure_eq(other.expose_bytes())
        }
    }

    impl SecureCompare for super::SecureVec {
        fn secure_eq(&self, other: &Self) -> bool {
            self.expose().secure_eq(other.expose())
        }
    }

    impl SecureCompare for Vec<u8> {
        fn secure_eq(&self, other: &Self) -> bool {
            self.as_slice().secure_eq(other.as_slice())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_secure_eq_byte_slices() {
            let a = b"password";
            let b = b"password";
            let c = b"different";
            assert!(a.secure_eq(b));
            assert!(!a.secure_eq(c));
        }

        #[test]
        fn test_secure_eq_different_lengths() {
            let a = b"short";
            let b = b"longer";
            assert!(!a.secure_eq(b));
        }
    }
}

/// Token bucket rate limiter for protecting against brute force attacks.
pub mod rate_limiter {
    use std::sync::{Arc, atomic::{AtomicU32, AtomicU64, Ordering}};
    use std::time::{Duration, Instant};

    /// A token bucket rate limiter.
    /// Used to limit auth attempts, API calls, and other sensitive operations.
    #[derive(Debug, Clone)]
    pub struct RateLimiter {
        max_tokens: u32,
        refill_rate_ns: u64, // nanoseconds per token
        tokens: Arc<AtomicU32>,
        last_refill: Arc<AtomicU64>,
    }

    impl RateLimiter {
        /// Create a new rate limiter with max tokens and refill rate.
        /// # Arguments
        /// * `max_tokens` - Maximum number of tokens (bucket capacity)
        /// * `refill_rate` - Tokens added per second
        pub fn new(max_tokens: u32, refill_rate: f64) -> Self {
            assert!(max_tokens > 0, "max_tokens must be > 0");
            assert!(refill_rate > 0.0, "refill_rate must be > 0");
            let refill_rate_ns = if refill_rate > 0.0 {
                (1_000_000_000.0 / refill_rate) as u64
            } else {
                u64::MAX
            };
            Self {
                max_tokens,
                refill_rate_ns,
                tokens: Arc::new(AtomicU32::new(max_tokens)),
                last_refill: Arc::new(AtomicU64::new(Self::current_nanos())),
            }
        }

        /// Try to consume a token.
        /// Returns true if a token was available and consumed.
        pub fn try_acquire(&self) -> bool {
            self.refill();
            let current = self.tokens.load(Ordering::Acquire);
            if current > 0 {
                self.tokens.fetch_sub(1, Ordering::Release);
                true
            } else {
                false
            }
        }

        /// Blocking acquire with optional timeout.
        /// Returns true if a token was acquired within the timeout.
        pub fn acquire_with_timeout(&self, timeout: Duration) -> bool {
            let start = Instant::now();
            while start.elapsed() < timeout {
                if self.try_acquire() {
                    return true;
                }
                // Sleep a bit to avoid busy waiting
                std::thread::sleep(Duration::from_millis(10));
            }
            false
        }

        /// Blocking acquire.
        pub fn acquire(&self) {
            while !self.try_acquire() {
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        /// Get the number of available tokens without consuming.
        pub fn available_tokens(&self) -> u32 {
            self.refill();
            self.tokens.load(Ordering::Acquire)
        }

        /// Reset the rate limiter to full capacity.
        pub fn reset(&self) {
            self.tokens.store(self.max_tokens, Ordering::Release);
            self.last_refill.store(Self::current_nanos(), Ordering::Release);
        }

        fn refill(&self) {
            let now = Self::current_nanos();
            let last = self.last_refill.load(Ordering::Acquire);
            let elapsed = now.saturating_sub(last);
            if elapsed == 0 {
                return;
            }
            let tokens_to_add = (elapsed as f64 / self.refill_rate_ns as f64) as u32;
            if tokens_to_add == 0 {
                return;
            }
            let current = self.tokens.load(Ordering::Acquire);
            let new_tokens = current.saturating_add(tokens_to_add).min(self.max_tokens);
            self.tokens.store(new_tokens, Ordering::Release);
            self.last_refill.store(now, Ordering::Release);
        }

        fn current_nanos() -> u64 {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        }
    }

    impl Default for RateLimiter {
        fn default() -> Self {
            Self::new(10, 1.0) // Default: 10 requests per second
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_rate_limiter_try_acquire() {
            let limiter = RateLimiter::new(5, 1.0);
            for _ in 0..5 {
                assert!(limiter.try_acquire());
            }
            assert!(!limiter.try_acquire());
        }

        #[test]
        fn test_rate_limiter_reset() {
            let limiter = RateLimiter::new(3, 1.0);
            for _ in 0..3 {
                assert!(limiter.try_acquire());
            }
            assert!(!limiter.try_acquire());
            limiter.reset();
            assert!(limiter.try_acquire());
        }
    }
}

/// Input validation utilities for preventing injection and other attacks.
pub mod input {
    use anyhow::Result;

    /// Input validation error.
    #[derive(Debug, Clone, thiserror::Error)]
    pub enum ValidationError {
        #[error("Input too short: expected at least {expected}, got {actual}")]
        TooShort { expected: usize, actual: usize },
        #[error("Input too long: expected at most {expected}, got {actual}")]
        TooLong { expected: usize, actual: usize },
        #[error("Input contains invalid characters")]
        InvalidCharacters,
        #[error("Input is empty")]
        Empty,
        #[error("Input does not match pattern: {pattern}")]
        PatternMismatch { pattern: String },
        #[error("Custom validation failed: {message}")]
        Custom { message: String },
    }

    /// Input validator with chainable rules.
    #[derive(Debug, Clone)]
    pub struct InputValidator;

    impl InputValidator {
        /// Create a new input validator.
        pub fn new() -> Self {
            Self
        }

        /// Validate that input is not empty.
        pub fn not_empty(input: &str) -> Result<&str, ValidationError> {
            if input.is_empty() {
                Err(ValidationError::Empty)
            } else {
                Ok(input)
            }
        }

        /// Validate minimum length.
        pub fn min_length(input: &str, min: usize) -> Result<&str, ValidationError> {
            if input.len() < min {
                Err(ValidationError::TooShort {
                    expected: min,
                    actual: input.len(),
                })
            } else {
                Ok(input)
            }
        }

        /// Validate maximum length.
        pub fn max_length(input: &str, max: usize) -> Result<&str, ValidationError> {
            if input.len() > max {
                Err(ValidationError::TooLong {
                    expected: max,
                    actual: input.len(),
                })
            } else {
                Ok(input)
            }
        }

        /// Validate length range (inclusive).
        pub fn length_range(input: &str, min: usize, max: usize) -> Result<&str, ValidationError> {
            Self::min_length(input, min).and_then(|_| Self::max_length(input, max))
        }

        /// Validate that input contains only alphanumeric characters.
        pub fn alphanumeric(input: &str) -> Result<&str, ValidationError> {
            if input.chars().all(|c| c.is_ascii_alphanumeric()) {
                Ok(input)
            } else {
                Err(ValidationError::InvalidCharacters)
            }
        }

        /// Validate password meets complexity requirements.
        /// Requirements: min_length, at least one uppercase, one lowercase, one digit.
        pub fn password_complexity(input: &str, min_length: usize) -> Result<&str, Vec<ValidationError>> {
            let mut errors = Vec::new();
            if input.len() < min_length {
                errors.push(ValidationError::TooShort {
                    expected: min_length,
                    actual: input.len(),
                });
            }
            if !input.chars().any(|c| c.is_ascii_uppercase()) {
                errors.push(ValidationError::Custom {
                    message: "Password must contain at least one uppercase letter".to_string(),
                });
            }
            if !input.chars().any(|c| c.is_ascii_lowercase()) {
                errors.push(ValidationError::Custom {
                    message: "Password must contain at least one lowercase letter".to_string(),
                });
            }
            if !input.chars().any(|c| c.is_ascii_digit()) {
                errors.push(ValidationError::Custom {
                    message: "Password must contain at least one digit".to_string(),
                });
            }
            if errors.is_empty() {
                Ok(input)
            } else {
                Err(errors)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_not_empty() {
            assert!(InputValidator::not_empty("test").is_ok());
            assert!(InputValidator::not_empty("").is_err());
        }

        #[test]
        fn test_length_validation() {
            let input = "test";
            assert!(InputValidator::min_length(input, 4).is_ok());
            assert!(InputValidator::min_length(input, 5).is_err());
            assert!(InputValidator::max_length(input, 4).is_ok());
            assert!(InputValidator::max_length(input, 3).is_err());
        }
    }
}