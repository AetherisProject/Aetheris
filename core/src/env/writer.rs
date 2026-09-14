//! Windows environment variable writer.

use std::path::PathBuf;
use serde::Serialize;
use crate::error::AetherisError;

#[derive(Debug, Clone, Serialize)]
pub struct WriteResult {
    pub name: String,
    pub scope: String,
    pub previous_value: Option<String>,
    pub action: WriteAction,
}

#[derive(Debug, Clone, Serialize)]
pub enum WriteAction {
    Created,
    Updated,
    Deleted,
    AlreadyPresent,
}

/// Writes or updates an environment variable in the registry.
#[cfg(windows)]
pub fn set_env_var(
    name: impl AsRef<str>,
    value: impl AsRef<str>,
    scope: impl AsRef<str>,
    requires_elevation: bool,
) -> Result<WriteResult, AetherisError> {
    use windows::Win32::System::Registry::{
        HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_WRITE, REG_SZ, RegOpenKeyExW,
        RegSetValueExW,
    };

    let name = name.as_ref();
    let value = value.as_ref();
    let scope = scope.as_ref();
    let previous = get_raw_env_var(name, scope)?;

    let (subkey, hkey): (&str, HKEY) = match scope {
        "user" | "User" => ("Environment", HKEY_CURRENT_USER),
        "system" | "System" => (
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
            HKEY_LOCAL_MACHINE,
        ),
        _ => return Err(AetherisError::VaultError(format!("Unknown scope: {scope}"))),
    };

    let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
    let mut hkey_out = HKEY::default();
    unsafe {
        RegOpenKeyExW(
            hkey,
            windows::core::PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_WRITE,
            &mut hkey_out,
        )
    }
    .ok()
    .map_err(|e| {
        if requires_elevation {
            AetherisError::VaultError(format!("Need UAC elevation: {e}"))
        } else {
            AetherisError::VaultError(format!("RegOpenKeyExW: {e}"))
        }
    })?;

    let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
    let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    // Convert UTF-16 to bytes for RegSetValueExW
    let mut value_bytes: Vec<u8> = Vec::with_capacity(value_wide.len() * 2);
    for &unit in &value_wide {
        value_bytes.extend_from_slice(&unit.to_le_bytes());
    }

    unsafe {
        RegSetValueExW(
            hkey_out,
            windows::core::PCWSTR(name_wide.as_ptr()),
            0,
            REG_SZ,
            Some(&value_bytes),
        )
    }
    .ok()
    .map_err(|e| AetherisError::VaultError(format!("RegSetValueExW: {e}")))?;

    broadcast_setting_change();

    Ok(WriteResult {
        name: name.to_string(),
        scope: scope.to_string(),
        previous_value: previous.clone(),
        action: if previous.is_some() { WriteAction::Updated } else { WriteAction::Created },
    })
}

/// Deletes an environment variable from the registry.
#[cfg(windows)]
pub fn delete_env_var(
    name: impl AsRef<str>,
    scope: impl AsRef<str>,
) -> Result<WriteResult, AetherisError> {
    use windows::Win32::System::Registry::{
        HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_WRITE, RegDeleteValueW, RegOpenKeyExW,
    };

    let name = name.as_ref();
    let scope = scope.as_ref();
    let previous = get_raw_env_var(name, scope)?;

    let (subkey, hkey): (&str, HKEY) = match scope {
        "user" | "User" => ("Environment", HKEY_CURRENT_USER),
        "system" | "System" => (
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
            HKEY_LOCAL_MACHINE,
        ),
        _ => return Err(AetherisError::VaultError(format!("Unknown scope: {scope}"))),
    };

    let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
    let mut hkey_out = HKEY::default();
    unsafe {
        RegOpenKeyExW(
            hkey,
            windows::core::PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_WRITE,
            &mut hkey_out,
        )
    }
    .ok()
    .map_err(|e| AetherisError::VaultError(format!("RegOpenKeyExW: {e}")))?;

    let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe { RegDeleteValueW(hkey_out, windows::core::PCWSTR(name_wide.as_ptr())) }
        .ok()
        .map_err(|e| AetherisError::VaultError(format!("RegDeleteValueW: {e}")))?;

    broadcast_setting_change();

    Ok(WriteResult {
        name: name.to_string(),
        scope: scope.to_string(),
        previous_value: previous.clone(),
        action: if previous.is_some() { WriteAction::Deleted } else { WriteAction::AlreadyPresent },
    })
}

/// Renames an env var (non-destructive: keep original under a backup name).
pub fn rename_env_var(
    name: impl AsRef<str>,
    new_name: impl AsRef<str>,
    scope: impl AsRef<str>,
) -> Result<WriteResult, AetherisError> {
    let name = name.as_ref();
    let new_name = new_name.as_ref();
    let scope = scope.as_ref();
    let value = get_raw_env_var(name, scope)?
        .ok_or_else(|| AetherisError::VaultError(format!("Var not found: {name}")))?;

    set_env_var(new_name, &value, scope, false)?;
    let backup_name = format!("{name}_AETH_BACKUP");
    set_env_var(&backup_name, &value, scope, false)?;

    Ok(WriteResult {
        name: new_name.to_string(),
        scope: scope.to_string(),
        previous_value: Some(value),
        action: WriteAction::Created,
    })
}

fn get_raw_env_var(name: &str, scope: &str) -> Result<Option<String>, AetherisError> {
    let vars = crate::env::reader::read_env_vars(match scope {
        "user" | "User" => crate::env::reader::EnvScope::User,
        "system" | "System" => crate::env::reader::EnvScope::System,
        _ => return Err(AetherisError::VaultError(format!("Unknown scope: {scope}"))),
    })?;
    Ok(vars.into_iter().find(|v| v.name == name).map(|v| v.value))
}

/// Broadcasts WM_SETTINGCHANGE so Explorer reloads environment.
#[cfg(windows)]
fn broadcast_setting_change() {
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };
    let env_str: Vec<u16> = "Environment".encode_utf16().chain(std::iter::once(0)).collect();
    let env_pcwstr = windows::core::PCWSTR(env_str.as_ptr());
    unsafe {
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(env_pcwstr.0 as isize),
            SMTO_ABORTIFHUNG,
            5000,
            None,
        );
    }
}

pub fn detect_dead_path_entries(scope: &str) -> Result<Vec<String>, AetherisError> {
    let path_var = get_raw_env_var("PATH", scope)?;
    let Some(path_var) = path_var else { return Ok(Vec::new()) };

    let dead: Vec<String> = path_var
        .split(';')
        .filter(|entry| {
            let entry = entry.trim();
            if entry.is_empty() {
                return false;
            }
            let p = PathBuf::from(entry);
            !p.exists()
        })
        .map(String::from)
        .collect();

    Ok(dead)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_env_var_creates_backup() {
        let _ = rename_env_var("NONEXISTENT", "NEW_NAME", "user");
    }
}
