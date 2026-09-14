//! Environment variable reader.
//!
//! Reads user and system environment variables. On Windows, this reads from
//! `AETH_USER_*` and `AETH_SYS_*` prefixed environment variables.
//! Direct Windows registry integration can be added via the `windows` crate.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::AetherisError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvScope {
    User,
    System,
}

impl std::fmt::Display for EnvScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvScope::User => write!(f, "user"),
            EnvScope::System => write!(f, "system"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
    pub scope: String,
}

/// Reads environment variables from `AETH_USER_*` / `AETH_SYS_*` env vars.
/// On non-Windows, this is the primary mechanism.
/// On Windows, registry integration via the `windows` crate can be layered in.
pub fn read_env_vars(scope: EnvScope) -> Result<Vec<EnvVar>, AetherisError> {
    let prefix = match scope {
        EnvScope::User => "AETH_USER_",
        EnvScope::System => "AETH_SYS_",
    };
    let vars: Vec<EnvVar> = std::env::vars()
        .filter(|(k, _)| k.starts_with(prefix))
        .map(|(k, v)| EnvVar {
            name: k.trim_start_matches(prefix).to_string(),
            value: v,
            scope: scope.to_string(),
        })
        .collect();
    Ok(vars)
}

pub fn read_user_env() -> Result<Vec<EnvVar>, AetherisError> {
    read_env_vars(EnvScope::User)
}

pub fn read_system_env() -> Result<Vec<EnvVar>, AetherisError> {
    read_env_vars(EnvScope::System)
}

pub fn read_all_env() -> Result<Vec<EnvVar>, AetherisError> {
    let mut user = read_user_env()?;
    let system = read_system_env()?;
    user.extend(system);
    Ok(user)
}

pub fn read_env_map() -> Result<HashMap<String, String>, AetherisError> {
    let vars = read_all_env()?;
    Ok(vars.into_iter().map(|v| (v.name, v.value)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_env_vars_fallback() {
        std::env::set_var("AETH_USER_TEST_VAR", "hello");
        let vars = read_env_vars(EnvScope::User).unwrap();
        assert!(vars.iter().any(|v| v.name == "TEST_VAR" && v.value == "hello"));
        std::env::remove_var("AETH_USER_TEST_VAR");
    }

    #[test]
    fn read_user_env_ok() {
        let _ = read_user_env();
    }

    #[test]
    fn read_env_map_ok() {
        let _ = read_env_map();
    }
}
