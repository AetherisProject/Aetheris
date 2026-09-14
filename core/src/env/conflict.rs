//! Conflict detection for environment variables.
//!
//! Detects: user-vs-system shadow, PATH entry override, duplicate definitions.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::env::reader::EnvVar;
use crate::error::AetherisError;

/// A detected conflict between environment variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub name: String,
    pub conflict_type: ConflictType,
    pub scope_a: String,
    pub scope_b: String,
    pub value_a: String,
    pub value_b: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictType {
    UserShadowsSystem,
    PathDuplicate,
    PathOverride,
}

/// Detects user vs system env var conflicts.
///
/// When a variable exists in both user and system scope, the user value wins
/// but this can cause confusion. This function flags such cases.
///
/// # Example
///
/// ```
/// use aetheris_core::env::conflict::detect_user_system_conflicts;
/// use aetheris_core::env::reader::EnvVar;
/// let user = vec![EnvVar { name: "MY_VAR".to_string(), value: "user-value".to_string(), scope: "user".to_string() }];
/// let system = vec![EnvVar { name: "MY_VAR".to_string(), value: "sys-value".to_string(), scope: "system".to_string() }];
/// let conflicts = detect_user_system_conflicts(&user, &system).unwrap();
/// assert_eq!(conflicts.len(), 1);
/// assert!(matches!(conflicts[0].conflict_type, aetheris_core::env::conflict::ConflictType::UserShadowsSystem));
/// ```
pub fn detect_user_system_conflicts(
    user_vars: &[EnvVar],
    system_vars: &[EnvVar],
) -> Result<Vec<Conflict>, AetherisError> {
    let mut conflicts = Vec::new();
    let system_map: HashMap<String, &EnvVar> = system_vars.iter().map(|v| (v.name.clone(), v)).collect();

    for user_var in user_vars {
        if let Some(sys_var) = system_map.get(&user_var.name) {
            if user_var.value != sys_var.value {
                conflicts.push(Conflict {
                    name: user_var.name.clone(),
                    conflict_type: ConflictType::UserShadowsSystem,
                    scope_a: "user".to_string(),
                    scope_b: "system".to_string(),
                    value_a: user_var.value.clone(),
                    value_b: sys_var.value.clone(),
                    description: format!(
                        "User '{}' ({}) shadows system value ({})",
                        user_var.name, user_var.value, sys_var.value
                    ),
                });
            }
        }
    }

    Ok(conflicts)
}

/// Detects duplicate entries in PATH.
pub fn detect_path_duplicates(scope: &str) -> Result<Vec<String>, AetherisError> {
    let vars = crate::env::reader::read_env_vars(match scope {
        "user" | "User" => crate::env::reader::EnvScope::User,
        "system" | "System" => crate::env::reader::EnvScope::System,
        _ => return Err(AetherisError::VaultError(format!("Unknown scope: {scope}"))),
    })?;

    let path_var = vars.into_iter().find(|v| v.name == "PATH");
    let Some(path_var) = path_var else { return Ok(Vec::new()) };

    let entries: Vec<&str> = path_var.value.split(';').filter(|e| !e.is_empty()).collect();
    let mut seen = HashMap::new();
    let mut duplicates = Vec::new();

    for entry in &entries {
        let normalized = entry.trim().to_lowercase();
        if seen.contains_key(&normalized) {
            duplicates.push(entry.to_string());
        } else {
            seen.insert(normalized, ());
        }
    }

    Ok(duplicates)
}

/// Detects PATH entries that point to nonexistent directories.
pub fn detect_path_overrides(scope: &str) -> Result<Vec<String>, AetherisError> {
    crate::env::writer::detect_dead_path_entries(scope)
}

/// Runs all conflict detection checks.
pub fn detect_conflicts() -> Result<Vec<Conflict>, AetherisError> {
    let user_vars = crate::env::reader::read_user_env()?;
    let system_vars = crate::env::reader::read_system_env()?;
    let mut conflicts = detect_user_system_conflicts(&user_vars, &system_vars)?;

    // Check PATH duplicates.
    for scope in &["user", "system"] {
        let dupes = detect_path_duplicates(scope)?;
        for dup in dupes {
            conflicts.push(Conflict {
                name: "PATH".to_string(),
                conflict_type: ConflictType::PathDuplicate,
                scope_a: scope.to_string(),
                scope_b: scope.to_string(),
                value_a: dup.clone(),
                value_b: String::new(),
                description: format!("Duplicate PATH entry in {scope}: {dup}"),
            });
        }
    }

    Ok(conflicts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_shadows_system_detected() {
        let user = vec![EnvVar {
            name: "FOO".into(),
            value: "user".into(),
            scope: "user".into(),
        }];
        let system = vec![EnvVar {
            name: "FOO".into(),
            value: "system".into(),
            scope: "system".into(),
        }];
        let conflicts = detect_user_system_conflicts(&user, &system).unwrap();
        assert_eq!(conflicts.len(), 1);
        assert!(matches!(conflicts[0].conflict_type, ConflictType::UserShadowsSystem));
    }

    #[test]
    fn identical_values_no_conflict() {
        let user = vec![EnvVar {
            name: "FOO".into(),
            value: "same".into(),
            scope: "user".into(),
        }];
        let system = vec![EnvVar {
            name: "FOO".into(),
            value: "same".into(),
            scope: "system".into(),
        }];
        let conflicts = detect_user_system_conflicts(&user, &system).unwrap();
        assert!(conflicts.is_empty());
    }

    #[test]
    fn no_overlap_no_conflict() {
        let user = vec![EnvVar {
            name: "A".into(),
            value: "1".into(),
            scope: "user".into(),
        }];
        let system = vec![EnvVar {
            name: "B".into(),
            value: "2".into(),
            scope: "system".into(),
        }];
        let conflicts = detect_user_system_conflicts(&user, &system).unwrap();
        assert!(conflicts.is_empty());
    }
}
