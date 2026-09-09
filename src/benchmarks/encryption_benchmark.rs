use criterion::{black_box, criterion_group, criterion_main, Bencher};
use aetheris::crypto::{CryptoEngine, KyberPublicKey, KyberSecretKey};

fn main() {
    criterion_group!(benches, bench_encryption_decryption);
}

fn bench_encryption_decryption(b: &mut Bencher) {
    let engine = CryptoEngine::new();
    let public_key = KyberPublicKey(engine.generate_kyber_keypair().public_key);
    let secret_key = KyberSecretKey(engine.generate_kyber_keypair().secret_key);
    
    let data = b.black_box(&[0u8; 1024]);
    
    b.iter(|| {
        let _encrypted = engine.hybrid_encrypt(data, &public_key).unwrap();
        let _decrypted = engine.hybrid_decrypt(&_encrypted.0, &_encrypted.1, &secret_key, data).unwrap();
    });
}