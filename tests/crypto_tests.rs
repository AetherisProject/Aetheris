//! Cryptography tests for Aetheris.

use aetheris::crypto::CryptoEngine;

#[test]
fn test_crypto_engine_new() {
    let _engine = CryptoEngine::new();
}

#[test]
fn test_crypto_engine_default() {
    let _engine: CryptoEngine = Default::default();
}

