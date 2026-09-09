use criterion::{black_box, criterion_group, criterion_main, Bencher};
use aetheris::crypto::CryptoEngine;

fn main() {
    criterion_group!(benches, bench_key_generation);
}

fn bench_key_generation(b: &mut Bencher) {
    b.iter(|| {
        let _engine = CryptoEngine::new();
    });
}