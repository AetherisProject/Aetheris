use criterion::{black_box, criterion_group, criterion_main, Bencher};
use aetheris::vault::VaultItem;
use aetheris::crypto::CryptoEngine;

fn main() {
    criterion_group!(benches, bench_vault_operations);
}

fn bench_vault_operations(b: &mut Bencher) {
    let engine = CryptoEngine::new();
    let vault = VaultItem::new(engine);
    
    // Simulate adding and retrieving items
    let test_item = b.black_box("test_item");
    
    b.iter(|| {
        // Add an item to the vault
        vault.add_item(test_item.clone(), b.black_box(&[0u8; 1024])).unwrap();
        
        // Retrieve the item from the vault
        let retrieved_item = vault.retrieve_item(test_item.clone()).unwrap();
        
        // Ensure the retrieved item matches the original
        assert_eq!(retrieved_item, test_item);
    });
}