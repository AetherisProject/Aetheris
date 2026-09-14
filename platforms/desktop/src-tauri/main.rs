#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use aetheris_core::{
    env::scanner::scan_for_api_keys,
    env::reader::{read_all_env, read_env_vars, EnvScope},
    env::conflict::detect_user_system_conflicts,
    vault::store::VaultStore,
};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri::generate_handler;

static VAULT_PATH: Mutex<String> = Mutex::new(String::new());

fn get_vault_path() -> String {
    let path = VAULT_PATH.lock();
    if path.is_empty() {
        let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".into());
        format!("{}\\.aetheris\\vault", home)
    } else {
        path.clone()
    }
}

fn open_vault(password: &str) -> Result<VaultStore, String> {
    let path = get_vault_path();
    std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap_or(std::path::Path::new(".")))
        .map_err(|e| format!("create dir: {e}"))?;
    VaultStore::open(&path, password).map_err(|e| e.to_string())
}

#[tauri::command]
async fn env_scan(password: String) -> Result<Vec<KeyFinding>, String> {
    let _ = open_vault(&password);
    let vars = read_all_env().map_err(|e| e.to_string())?;
    let pairs: Vec<(String, String)> = vars.into_iter().map(|v| (v.name, v.value)).collect();
    let findings = scan_for_api_keys(&pairs).map_err(|e| e.to_string())?;
    Ok(findings.into_iter().map(KeyFinding::from).collect())
}

#[tauri::command]
async fn env_list() -> Result<Vec<EnvVarDto>, String> {
    let vars = read_all_env().map_err(|e| e.to_string())?;
    Ok(vars.into_iter().map(EnvVarDto::from).collect())
}

#[tauri::command]
async fn env_sync(password: String) -> Result<usize, String> {
    let store = open_vault(&password)?;
    let vars = read_all_env().map_err(|e| e.to_string())?;
    let pairs: Vec<(String, String)> = vars.iter().map(|v| (v.name.clone(), v.value.clone())).collect();
    let findings = scan_for_api_keys(&pairs).map_err(|e| e.to_string())?;
    let mut synced = 0;
    for finding in &findings {
        if let Some(var) = vars.iter().find(|v| v.name == finding.var_name) {
            store.insert(&var.name, &var.value).map_err(|e| e.to_string())?;
            synced += 1;
        }
    }
    Ok(synced)
}

#[tauri::command]
async fn env_conflicts() -> Result<Vec<ConflictDto>, String> {
    let user = read_env_vars(EnvScope::User).map_err(|e| e.to_string())?;
    let system = read_env_vars(EnvScope::System).map_err(|e| e.to_string())?;
    let conflicts = detect_user_system_conflicts(&user, &system).map_err(|e| e.to_string())?;
    Ok(conflicts.into_iter().map(ConflictDto::from).collect())
}

#[tauri::command]
async fn vault_list(password: String) -> Result<Vec<VaultItemDto>, String> {
    let store = open_vault(&password)?;
    let items = store.list().map_err(|e| e.to_string())?;
    Ok(items.into_iter().map(VaultItemDto::from).collect())
}

#[tauri::command]
async fn vault_add(password: String, name: String, value: String) -> Result<(), String> {
    let store = open_vault(&password)?;
    store.insert(&name, &value).map_err(|e| e.to_string())
}

#[tauri::command]
async fn vault_get(password: String, name: String) -> Result<VaultItemDto, String> {
    let store = open_vault(&password)?;
    let item = store.get(&name).map_err(|e| e.to_string())?;
    Ok(VaultItemDto::from(item))
}

#[tauri::command]
async fn vault_delete(password: String, name: String) -> Result<(), String> {
    let store = open_vault(&password)?;
    store.delete(&name).map_err(|e| e.to_string())
}

#[tauri::command]
async fn vault_snapshot(password: String, label: String) -> Result<SnapshotDto, String> {
    let store = open_vault(&password)?;
    let snap = store.snapshot(&label).map_err(|e| e.to_string())?;
    Ok(SnapshotDto::from(snap))
}

#[tauri::command]
async fn vault_snapshots(password: String) -> Result<Vec<SnapshotDto>, String> {
    let store = open_vault(&password)?;
    let snaps = store.list_snapshots().map_err(|e| e.to_string())?;
    Ok(snaps.into_iter().map(SnapshotDto::from).collect())
}

#[tauri::command]
async fn vault_rollback(password: String, snapshot_id: String) -> Result<(), String> {
    let store = open_vault(&password)?;
    store.rollback(&snapshot_id).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyFinding {
    var_name: String,
    provider: String,
    confidence: String,
    scope: String,
}

impl From<aetheris_core::env::scanner::ApiKeyDetection> for KeyFinding {
    fn from(d: aetheris_core::env::scanner::ApiKeyDetection) -> Self {
        Self {
            var_name: d.var_name,
            provider: d.provider,
            confidence: format!("{:?}", d.confidence),
            scope: d.scope,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EnvVarDto {
    name: String,
    value: String,
    scope: String,
}

impl From<aetheris_core::env::reader::EnvVar> for EnvVarDto {
    fn from(v: aetheris_core::env::reader::EnvVar) -> Self {
        Self { name: v.name, value: v.value, scope: v.scope }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConflictDto {
    name: String,
    conflict_type: String,
    description: String,
}

impl From<aetheris_core::env::conflict::Conflict> for ConflictDto {
    fn from(c: aetheris_core::env::conflict::Conflict) -> Self {
        Self { name: c.name, conflict_type: format!("{:?}", c.conflict_type), description: c.description }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VaultItemDto {
    name: String,
    value: String,
    scope: String,
    created_at: i64,
    updated_at: i64,
}

impl From<aetheris_core::vault::store::ApiKeyItem> for VaultItemDto {
    fn from(item: aetheris_core::vault::store::ApiKeyItem) -> Self {
        Self {
            name: item.name,
            value: item.value,
            scope: item.scope,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SnapshotDto {
    id: String,
    label: String,
    created_at: i64,
    item_count: usize,
}

impl From<aetheris_core::vault::store::Snapshot> for SnapshotDto {
    fn from(s: aetheris_core::vault::store::Snapshot) -> Self {
        Self { id: s.id, label: s.label, created_at: s.created_at, item_count: s.item_count }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            env_scan,
            env_list,
            env_sync,
            env_conflicts,
            vault_list,
            vault_add,
            vault_get,
            vault_delete,
            vault_snapshot,
            vault_snapshots,
            vault_rollback,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
