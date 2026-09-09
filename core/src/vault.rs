#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;
use bincode::{serialize, deserialize};
use zeroize::Zeroize;

#[derive(Debug, Serialize, Deserialize, Zeroize)]
pub struct VaultItem {
    pub id: String,
    pub name: String,
    pub data: Vec<u8>,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl VaultItem {
    pub fn new(name: String, data: Vec<u8>, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            data,
            tags,
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        }
    }
    
    pub fn update(&mut self, name: String, new_data: Vec<u8>, new_tags: Vec<String>) {
        self.name = name;
        self.data = new_data;
        self.tags = new_tags;
        self.updated_at = Utc::now().timestamp();
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        serialize(self).map_err(|e| e.to_string())
    }
    
    pub fn deserialize(data: &[u8]) -> Result<VaultItem, String> {
        deserialize(data).map_err(|e| e.to_string())
    }
}