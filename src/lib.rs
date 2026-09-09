#![feature(proc_macro)]
use anyhow::{Context, Result};
use std::env;
use std::process;
mod screenshot;
#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--screenshot".to_string()) {
        if args.contains(&"--web".to_string()) {
            screenshot::capture_web_screenshot().await?;
        } else {
            screenshot::capture_desktop_screenshot()?;
        }
        println!("Screenshot captured successfully!");
    }
    Ok(())
}
pub mod admin;
pub mod apikey;
pub mod auth;
pub mod browser;
pub mod crypto;
pub mod debug;
pub mod design;
pub mod i18n;
pub mod password;
pub mod proactive;
pub mod security;
pub mod ssh;
pub mod sync;
pub mod tauri;
pub mod vault;
pub mod web;
pub use crypto::CryptoEngine;
pub use design::{DesignSystem, Device, Icon, Layout, ResponsiveDesign, Theme};
pub use security::{SecureCompare, SecureString, SecureVec};
pub use vault::store::VaultStore;
pub use tauri::{add_vault_item, remove_vault_item, update_vault_item, search_vault_items, create_api_key, delete_api_key, connect_ssh, disconnect_ssh, sync_vault, backup_vault};
pub use nlu::NluEngine;
pub use security::{SecureCompare, SecureString, SecureVec};
pub use vault::store::VaultStore;

pub use nlu::NluEngine;