#![allow(dead_code)]
use wasm_bindgen::prelude::*;
use aetheris_core::{vault::VaultItem, crypto::CryptoEngine};

#[wasm_bindgen]
pub struct AetherisWeb {
    engine: CryptoEngine,
}

#[wasm_bindgen]
impl AetherisWeb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: CryptoEngine::new().expect("Failed to initialize CryptoEngine"),
        }
    }
    
    #[wasm_bindgen]
    pub fn generate_keypair(&self) -> Result<Vec<u8>, JsValue> {
        let (pk, _) = self.engine.generate_kyber_keypair()?;
        Ok(pk)
    }
    
    #[wasm_bindgen]
    pub fn create_vault_item(&self, name: String, data: &[u8]) -> Result<JsValue, JsValue> {
        let item = VaultItem::new(name, data.to_vec(), vec![]);
        Ok(serde_wasm_bindgen::to_value(&item).unwrap())
    }
}