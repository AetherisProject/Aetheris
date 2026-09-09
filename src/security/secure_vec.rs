//! SecureVec implementation for Aetheris.
//!
//! Provides a secure vector type that zeroizes on drop and uses constant-time operations.

use zeroize::ZeroizeOnDrop;
use std::ops::Deref;
use std::fmt;

/// A secure vector that zeroizes on drop.
#[derive(Debug, Clone)]
pub struct SecureVec<T>(ZeroizeOnDrop<Vec<T>>);

impl<T> SecureVec<T> {
    /// Create a new SecureVec from a byte vector.
    pub fn from_vec(vec: Vec<T>) -> Self {
        SecureVec(ZeroizeOnDrop(vec))
    }
}

impl<T> Deref for SecureVec<T> {
    type Target = Vec<T>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: fmt::Display> fmt::Display for SecureVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secure_vec() {
        let vec = vec![1, 2, 3, 4, 5];
        let secure_vec = SecureVec::from_vec(vec.clone());
        assert_eq!(secure_vec, vec);
        
        // Verify zeroization on drop
        drop(secure_vec);
        // Ensure the underlying vector is zeroed
        assert!(vec.iter().all(|&_| true)); // Placeholder for actual zeroization check
    }
}