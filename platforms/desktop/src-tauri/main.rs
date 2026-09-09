#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use aetheris_core::{vault::VaultItem, crypto::CryptoEngine, ssh::SshClient};
use tauri::generate_handler;

#[tauri::command]
async fn add_vault_item(name: String, data: String) -> Result<VaultItem, String> {
    let item = VaultItem::new(name, data.into_bytes(), vec![]);
    Ok(item)
}

#[tauri::command]
async fn generate_keypair() -> Result<(Vec<u8>, Vec<u8>), String> {
    let engine = CryptoEngine::new()?;
    Ok(engine.generate_kyber_keypair()?)
}

#[tauri::command]
async fn connect_ssh(host: String, port: u16) -> Result<String, String> {
    let mut client = SshClient::new(host.clone(), port);
    client.connect().map_err(|e| e.to_string())?;
    Ok(format!("Connected to {}:{}", host, port))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            add_vault_item,
            generate_keypair,
            connect_ssh,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}