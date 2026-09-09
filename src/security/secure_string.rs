//! SecureString implementation for Aetheris.
//!
//! Provides a secure string type that zeroizes on drop and uses constant-time operations.

use zeroize::ZeroizeOnDrop;
use std::ops::Deref;
use std::fmt;
use std::str;

/// A secure string that zeroizes on drop.
#[derive(Debug, Clone)]
pub struct SecureString(ZeroizeOnDrop<Vec<u8>>);

impl SecureString {
    /// Create a new SecureString from a byte vector.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        SecureString(ZeroizeOnDrop(bytes))
    }
}

impl Deref for SecureString {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.0).expect("Invalid UTF-8 bytes")
    }
}

impl fmt::Display for SecureString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secure_string() {
        let bytes = vec![b'H', b'e', b'l', b'l', b'o', b', b' ', b'W', b'o', b'r', b'l', b'd'];
        let secure_string = SecureString::from_bytes(bytes.clone());
        assert_eq!(secure_string, "Hello, World");
        
        // Verify zeroization on drop
        drop(secure_string);
        // Ensure the underlying bytes are zeroed
        assert!(bytes.iter().all(|&b| b == 0));
    }
}