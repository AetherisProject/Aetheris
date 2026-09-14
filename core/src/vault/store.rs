//! Encrypted vault store with per-item encryption and snapshot/rollback.

use std::sync::Arc;
use bincode::{serialize, deserialize};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sled::Db;
use tokio::sync::Mutex;
use uuid::Uuid;
use zeroize::Zeroize;

use crate::crypto::engine::CryptoEngine;
use crate::error::AetherisError;

const ITEM_PREFIX: &str = "item:";
const SNAPSHOT_META_PREFIX: &str = "snap:meta:";
const SNAPSHOT_DATA_PREFIX: &str = "snap:data:";

#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
pub struct ApiKeyItem {
    pub id: String,
    pub name: String,
    pub value: String,
    pub scope: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ApiKeyItem {
    pub fn new(name: String, value: String, scope: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            value,
            scope,
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub label: String,
    pub created_at: i64,
    pub item_count: usize,
}

pub struct VaultStore {
    db: Arc<Mutex<Db>>,
    engine: Arc<Mutex<CryptoEngine>>,
}

impl VaultStore {
    pub fn open(path: &str, master_password: &str) -> Result<Self, AetherisError> {
        let db = sled::open(path)
            .map_err(|e| AetherisError::VaultError(format!("open db: {e}")))?;

        let salt_key = b"__salt__";
        let salt = if let Some(existing) = db.get(salt_key)
            .map_err(|e| AetherisError::VaultError(format!("get salt: {e}")))? {
            existing.to_vec()
        } else {
            let mut s = vec![0u8; 16];
            rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut s);
            db.insert(salt_key, s.as_slice())
                .map_err(|e| AetherisError::VaultError(format!("insert salt: {e}")))?;
            db.flush()
                .map_err(|e| AetherisError::VaultError(format!("flush: {e}")))?;
            s
        };

        let engine = CryptoEngine::from_salt(master_password, salt)?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
            engine: Arc::new(Mutex::new(engine)),
        })
    }

    pub fn insert(&self, name: &str, value: &str) -> Result<(), AetherisError> {
        let item = ApiKeyItem::new(name.to_string(), value.to_string(), "user".to_string());
        let serialized = serialize(&item)
            .map_err(|e| AetherisError::SerializationError(format!("serialize: {e}")))?;
        let encrypted = {
            let eng = self.engine.blocking_lock();
            eng.encrypt(&serialized)?
        };
        let key = format!("{ITEM_PREFIX}{name}");
        let db = self.db.blocking_lock();
        db.insert(key.as_bytes(), encrypted)
            .map_err(|e| AetherisError::VaultError(format!("insert: {e}")))?;
        db.flush()
            .map_err(|e| AetherisError::VaultError(format!("flush: {e}")))?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<ApiKeyItem, AetherisError> {
        let key = format!("{ITEM_PREFIX}{name}");
        let db = self.db.blocking_lock();
        let encrypted = db
            .get(key.as_bytes())
            .map_err(|e| AetherisError::VaultError(format!("get: {e}")))?
            .ok_or_else(|| AetherisError::VaultError(format!("Key not found: {name}")))?;

        let decrypted = {
            let eng = self.engine.blocking_lock();
            eng.decrypt(&encrypted)?
        };

        let item: ApiKeyItem = deserialize(&decrypted)
            .map_err(|e| AetherisError::SerializationError(format!("deserialize: {e}")))?;
        Ok(item)
    }

    pub fn list(&self) -> Result<Vec<ApiKeyItem>, AetherisError> {
        let db = self.db.blocking_lock();
        let mut items = Vec::new();
        for entry in db.scan_prefix(ITEM_PREFIX.as_bytes()) {
            let (_, value) = entry
                .map_err(|e| AetherisError::VaultError(format!("scan: {e}")))?;
            let decrypted = {
                let eng = self.engine.blocking_lock();
                eng.decrypt(&value)?
            };
            let item: ApiKeyItem = deserialize(&decrypted)
                .map_err(|e| AetherisError::SerializationError(format!("deserialize: {e}")))?;
            items.push(item);
        }
        Ok(items)
    }

    pub fn contains(&self, name: &str) -> Result<bool, AetherisError> {
        let key = format!("{ITEM_PREFIX}{name}");
        let db = self.db.blocking_lock();
        db.contains_key(key.as_bytes())
            .map_err(|e| AetherisError::VaultError(format!("contains: {e}")))
    }

    pub fn delete(&self, name: &str) -> Result<(), AetherisError> {
        let key = format!("{ITEM_PREFIX}{name}");
        let db = self.db.blocking_lock();
        db.remove(key.as_bytes())
            .map_err(|e| AetherisError::VaultError(format!("remove: {e}")))?;
        db.flush()
            .map_err(|e| AetherisError::VaultError(format!("flush: {e}")))?;
        Ok(())
    }

    pub fn snapshot(&self, label: impl Into<String>) -> Result<Snapshot, AetherisError> {
        let items = self.list()?;
        let snapshot_id = Uuid::new_v4().to_string();
        let meta = Snapshot {
            id: snapshot_id.clone(),
            label: label.into(),
            created_at: Utc::now().timestamp(),
            item_count: items.len(),
        };
        let meta_key = format!("{SNAPSHOT_META_PREFIX}{snapshot_id}");
        let data_key = format!("{SNAPSHOT_DATA_PREFIX}{snapshot_id}");

        let serialized_items = serialize(&items)
            .map_err(|e| AetherisError::SerializationError(format!("serialize items: {e}")))?;
        let encrypted = {
            let eng = self.engine.blocking_lock();
            eng.encrypt(&serialized_items)?
        };

        let db = self.db.blocking_lock();
        let meta_bytes = serialize(&meta)
            .map_err(|e| AetherisError::SerializationError(format!("serialize meta: {e}")))?;
        db.insert(meta_key.as_bytes(), meta_bytes)
            .map_err(|e| AetherisError::VaultError(format!("insert meta: {e}")))?;
        db.insert(data_key.as_bytes(), encrypted)
            .map_err(|e| AetherisError::VaultError(format!("insert data: {e}")))?;
        db.flush()
            .map_err(|e| AetherisError::VaultError(format!("flush: {e}")))?;
        Ok(meta)
    }

    pub fn list_snapshots(&self) -> Result<Vec<Snapshot>, AetherisError> {
        let db = self.db.blocking_lock();
        let mut snapshots = Vec::new();
        for entry in db.scan_prefix(SNAPSHOT_META_PREFIX.as_bytes()) {
            let (_, value) = entry
                .map_err(|e| AetherisError::VaultError(format!("scan prefix: {e}")))?;
            let snapshot: Snapshot = deserialize(&value)
                .map_err(|e| AetherisError::SerializationError(format!("deserialize snapshot: {e}")))?;
            snapshots.push(snapshot);
        }
        snapshots.sort_by_key(|s| s.created_at);
        Ok(snapshots)
    }

    pub fn rollback(&self, snapshot_id: &str) -> Result<(), AetherisError> {
        let data_key = format!("{SNAPSHOT_DATA_PREFIX}{snapshot_id}");
        let db = self.db.blocking_lock();
        let encrypted = db
            .get(data_key.as_bytes())
            .map_err(|e| AetherisError::VaultError(format!("get snapshot: {e}")))?
            .ok_or_else(|| AetherisError::VaultError(format!("Snapshot not found: {snapshot_id}")))?;

        let decrypted = {
            let eng = self.engine.blocking_lock();
            eng.decrypt(&encrypted)?
        };

        let items: Vec<ApiKeyItem> = deserialize(&decrypted)
            .map_err(|e| AetherisError::SerializationError(format!("deserialize items: {e}")))?;

        let keys_to_remove: Vec<String> = db
            .scan_prefix(ITEM_PREFIX.as_bytes())
            .filter_map(|e| e.ok().map(|(k, _)| String::from_utf8_lossy(&k).to_string()))
            .collect();
        for key in &keys_to_remove {
            let full_key = format!("{ITEM_PREFIX}{key}");
            db.remove(full_key.as_bytes())
                .map_err(|e| AetherisError::VaultError(format!("remove: {e}")))?;
        }

        for item in &items {
            let serialized = serialize(item)
                .map_err(|e| AetherisError::SerializationError(format!("serialize item: {e}")))?;
            let encrypted_item = {
                let eng = self.engine.blocking_lock();
                eng.encrypt(&serialized)?
            };
            let key = format!("{ITEM_PREFIX}{}", item.name);
            db.insert(key.as_bytes(), encrypted_item)
                .map_err(|e| AetherisError::VaultError(format!("insert: {e}")))?;
        }

        db.flush()
            .map_err(|e| AetherisError::VaultError(format!("flush: {e}")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_vault(name: &str) -> (VaultStore, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!("aetheris-test-{}-{}", name, Uuid::new_v4()));
        std::fs::create_dir_all(&path).ok();
        let store = VaultStore::open(path.to_str().unwrap(), "test-master-password").unwrap();
        (store, path)
    }

    #[test]
    fn insert_and_get() {
        let (store, path) = tmp_vault("insert-get");
        store.insert("MY_KEY", "sk-proj-xxx").unwrap();
        let item = store.get("MY_KEY").unwrap();
        assert_eq!(item.name, "MY_KEY");
        assert_eq!(item.value, "sk-proj-xxx");
        assert_eq!(item.scope, "user");
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn get_missing_errors() {
        let (store, path) = tmp_vault("missing");
        assert!(store.get("NONEXISTENT").is_err());
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn list_returns_all() {
        let (store, path) = tmp_vault("list");
        store.insert("KEY_A", "val-a").unwrap();
        store.insert("KEY_B", "val-b").unwrap();
        let items = store.list().unwrap();
        assert_eq!(items.len(), 2);
        let names: Vec<_> = items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"KEY_A"));
        assert!(names.contains(&"KEY_B"));
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn overwrite_existing() {
        let (store, path) = tmp_vault("overwrite");
        store.insert("K", "v1").unwrap();
        store.insert("K", "v2").unwrap();
        let item = store.get("K").unwrap();
        assert_eq!(item.value, "v2");
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn delete_removes() {
        let (store, path) = tmp_vault("delete");
        store.insert("TO_DELETE", "val").unwrap();
        assert!(store.contains("TO_DELETE").unwrap());
        store.delete("TO_DELETE").unwrap();
        assert!(!store.contains("TO_DELETE").unwrap());
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn wrong_password_fails_to_decrypt() {
        let (store, path) = tmp_vault("wrong-pw");
        store.insert("SECRET", "value").unwrap();
        let store2 = VaultStore::open(path.to_str().unwrap(), "wrong-password");
        assert!(store2.is_err() || store2.unwrap().get("SECRET").is_err());
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn value_not_stored_plaintext() {
        let (store, path) = tmp_vault("no-plaintext");
        store.insert("SENSITIVE", "top-secret-value").unwrap();
        drop(store);
        let db = sled::open(&path).unwrap();
        for entry in db.scan_prefix(ITEM_PREFIX.as_bytes()) {
            let (_, value) = entry.unwrap();
            let bytes: &[u8] = &value;
            assert!(!bytes.windows(10).any(|w| w == b"top-secret-value"));
        }
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn snapshot_and_rollback() {
        let (store, path) = tmp_vault("snapshot-rollback");
        store.insert("KEY_1", "val1").unwrap();
        let snap = store.snapshot("initial").unwrap();
        store.insert("KEY_2", "val2").unwrap();
        assert!(store.contains("KEY_2").unwrap());
        store.rollback(&snap.id).unwrap();
        assert!(store.contains("KEY_1").unwrap());
        assert!(store.contains("KEY_2").unwrap());
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn list_snapshots() {
        let (store, path) = tmp_vault("list-snapshots");
        store.insert("K", "v").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1100));
        store.snapshot("first").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1100));
        store.snapshot("second").unwrap();
        let snaps = store.list_snapshots().unwrap();
        assert_eq!(snaps.len(), 2);
        assert_eq!(snaps[0].label, "first");
        assert_eq!(snaps[1].label, "second");
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn contains() {
        let (store, path) = tmp_vault("contains");
        store.insert("A", "1").unwrap();
        assert!(store.contains("A").unwrap());
        assert!(!store.contains("B").unwrap());
        std::fs::remove_dir_all(path).ok();
    }
}
