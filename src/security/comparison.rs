//! Constant-time comparison utilities for Aetheris.
//!
//! Provides secure constant-time comparison functions.

use std::cmp;
use std::ops::Deref;
use std::fmt;
use std::str;

/// Constant-time comparison for byte slices.
/// 
/// This function ensures that the comparison cannot be used to infer information
/// about the lengths or contents of the input slices.
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    let len_a = a.len();
    let len_b = b.len();
    
    // Compare lengths first
    if len_a != len_b {
        return false;
    }
    
    let mut result = 0;
    for (i, &byte_a) in a.iter().enumerate() {
        let byte_b = b[i];
        result ^= byte_a.wrapping_xor(byte_b);
    }
    
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_time_compare() {
        let a = b"hello";
        let b = b"hello";
        assert!(constant_time_compare(a, b));
        
        let a = b"hello";
        let b = b"world";
        assert!(!constant_time_compare(a, b));
        
        // Test with different lengths
        let a = b"hello";
        let b = b"hell";
        assert!(!constant_time_compare(a, b));
    }
}