use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aetheris::crypto::CryptoEngine;

fn encrypt_benchmark(c: &mut Criterion) {
    let engine = CryptoEngine::new().unwrap();
    let plaintext = vec![0u8; 1024];
    
    c.bench_function("encrypt_1kb", |b| {
        b.iter(|| {
            let _ = engine.encrypt_memory(black_box(&plaintext), black_box(&plaintext)).unwrap();
        })
    });
}

fn decrypt_benchmark(c: &mut Criterion) {
    let engine = CryptoEngine::new().unwrap();
    let plaintext = vec![0u8; 1024];
    let (iv, ciphertext) = engine.encrypt_memory(&plaintext, &plaintext).unwrap();
    
    c.bench_function("decrypt_1kb", |b| {
        b.iter(|| {
            let _ = engine.decrypt_memory(black_box(&ciphertext), black_box(&iv)).unwrap();
        })
    });
}

criterion_group!(benches, encrypt_benchmark, decrypt_benchmark);
criterion_main!(benches);
