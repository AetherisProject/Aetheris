//! Git-style environment variable snapshots.
//!
//! Captures a point-in-time state of all environment variables, stores it
//! in a diff-like structure, and can compute diffs between two snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::env::reader::EnvVar;
use crate::error::AetherisError;

/// A point-in-time snapshot of environment variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub label: String,
    pub created_at: i64,
    pub vars: HashMap<String, EnvVarEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVarEntry {
    pub name: String,
    pub value: String,
    pub scope: String,
}

/// A single change between two snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEntry {
    pub name: String,
    pub scope: String,
    pub change: ChangeType,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

/// Creates a snapshot from a list of env vars.
pub fn create_snapshot(
    id: impl Into<String>,
    label: impl Into<String>,
    vars: &[EnvVar],
    created_at: i64,
) -> Snapshot {
    let mut map = HashMap::new();
    for var in vars {
        map.insert(
            var.name.clone(),
            EnvVarEntry {
                name: var.name.clone(),
                value: var.value.clone(),
                scope: var.scope.clone(),
            },
        );
    }

    Snapshot {
        id: id.into(),
        label: label.into(),
        created_at,
        vars: map,
    }
}

/// Computes the diff between an old and new snapshot.
///
/// Returns a vector of [`DiffEntry`] describing every change.
///
/// # Example
///
/// ```
/// use aetheris_core::env::snapshot::{create_snapshot, diff_snapshots};
/// use aetheris_core::env::reader::EnvVar;
/// let old = create_snapshot("old".to_string(), "before".to_string(), &[], 0);
/// let mut new = create_snapshot("new".to_string(), "after".to_string(), &[], 0);
/// new.vars.insert("NEW_VAR".to_string(), aetheris_core::env::snapshot::EnvVarEntry {
///     name: "NEW_VAR".to_string(), value: "v".to_string(), scope: "user".to_string(),
/// });
/// let diff = diff_snapshots(&old, &new);
/// assert_eq!(diff.len(), 1);
/// assert!(matches!(diff[0].change, aetheris_core::env::snapshot::ChangeType::Added));
/// ```
pub fn diff_snapshots(old: &Snapshot, new: &Snapshot) -> Vec<DiffEntry> {
    let mut diffs = Vec::new();
    let all_keys: std::collections::HashSet<_> = old
        .vars
        .keys()
        .chain(new.vars.keys())
        .cloned()
        .collect();

    for key in all_keys {
        match (old.vars.get(&key), new.vars.get(&key)) {
            (Some(old_entry), Some(new_entry)) => {
                if old_entry.value != new_entry.value {
                    diffs.push(DiffEntry {
                        name: key.clone(),
                        scope: new_entry.scope.clone(),
                        change: ChangeType::Modified,
                        old_value: Some(old_entry.value.clone()),
                        new_value: Some(new_entry.value.clone()),
                    });
                }
            }
            (None, Some(new_entry)) => {
                diffs.push(DiffEntry {
                    name: key.clone(),
                    scope: new_entry.scope.clone(),
                    change: ChangeType::Added,
                    old_value: None,
                    new_value: Some(new_entry.value.clone()),
                });
            }
            (Some(old_entry), None) => {
                diffs.push(DiffEntry {
                    name: key.clone(),
                    scope: old_entry.scope.clone(),
                    change: ChangeType::Removed,
                    old_value: Some(old_entry.value.clone()),
                    new_value: None,
                });
            }
            (None, None) => {}
        }
    }

    diffs.sort_by(|a, b| a.name.cmp(&b.name));
    diffs
}

/// Applies a diff to a snapshot (forward apply).
pub fn apply_diff(snapshot: &Snapshot, diff: &[DiffEntry]) -> Snapshot {
    let mut result = snapshot.clone();
    for entry in diff {
        match entry.change {
            ChangeType::Added | ChangeType::Modified => {
                result.vars.insert(
                    entry.name.clone(),
                    EnvVarEntry {
                        name: entry.name.clone(),
                        value: entry.new_value.clone().unwrap_or_default(),
                        scope: entry.scope.clone(),
                    },
                );
            }
            ChangeType::Removed => {
                result.vars.remove(&entry.name);
            }
        }
    }
    result
}

/// Reverse-applies a diff (undo).
pub fn revert_diff(snapshot: &Snapshot, diff: &[DiffEntry]) -> Snapshot {
    let mut result = snapshot.clone();
    for entry in diff {
        match entry.change {
            ChangeType::Added => {
                result.vars.remove(&entry.name);
            }
            ChangeType::Modified => {
                if let Some(old) = &entry.old_value {
                    result.vars.insert(
                        entry.name.clone(),
                        EnvVarEntry {
                            name: entry.name.clone(),
                            value: old.clone(),
                            scope: entry.scope.clone(),
                        },
                    );
                }
            }
            ChangeType::Removed => {
                if let Some(old) = &entry.old_value {
                    result.vars.insert(
                        entry.name.clone(),
                        EnvVarEntry {
                            name: entry.name.clone(),
                            value: old.clone(),
                            scope: entry.scope.clone(),
                        },
                    );
                }
            }
        }
    }
    result
}

/// Returns the total number of changes between two snapshots.
pub fn change_count(old: &Snapshot, new: &Snapshot) -> usize {
    diff_snapshots(old, new).len()
}

/// Returns only changes of a specific type.
pub fn filter_changes(diffs: &[DiffEntry], change_type: &ChangeType) -> Vec<DiffEntry> {
    diffs
        .iter()
        .filter(|d| &d.change == change_type)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_snapshot<const N: usize>(vars: &[(&str, &str); N]) -> Snapshot {
        let env_vars: Vec<EnvVar> = vars
            .iter()
            .map(|(n, v)| EnvVar {
                name: n.to_string(),
                value: v.to_string(),
                scope: "user".to_string(),
            })
            .collect();
        create_snapshot("test", "test", &env_vars, 0)
    }

    #[test]
    fn diff_added_var() {
        let old = make_snapshot(&[("A", "1")]);
        let new = make_snapshot(&[("A", "1"), ("B", "2")]);
        let diff = diff_snapshots(&old, &new);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].name, "B");
        assert!(matches!(diff[0].change, ChangeType::Added));
    }

    #[test]
    fn diff_removed_var() {
        let old = make_snapshot(&[("A", "1"), ("B", "2")]);
        let new = make_snapshot(&[("A", "1")]);
        let diff = diff_snapshots(&old, &new);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].name, "B");
        assert!(matches!(diff[0].change, ChangeType::Removed));
    }

    #[test]
    fn diff_modified_var() {
        let old = make_snapshot(&[("A", "1")]);
        let new = make_snapshot(&[("A", "2")]);
        let diff = diff_snapshots(&old, &new);
        assert_eq!(diff.len(), 1);
        assert!(matches!(diff[0].change, ChangeType::Modified));
        assert_eq!(diff[0].old_value.as_deref(), Some("1"));
        assert_eq!(diff[0].new_value.as_deref(), Some("2"));
    }

    #[test]
    fn diff_no_changes() {
        let old = make_snapshot(&[("A", "1")]);
        let new = make_snapshot(&[("A", "1")]);
        let diff = diff_snapshots(&old, &new);
        assert!(diff.is_empty());
    }

    #[test]
    fn apply_diff_creates() {
        let snap = make_snapshot(&[]);
        let diff = vec![DiffEntry {
            name: "NEW".into(),
            scope: "user".into(),
            change: ChangeType::Added,
            old_value: None,
            new_value: Some("val".into()),
        }];
        let result = apply_diff(&snap, &diff);
        assert!(result.vars.contains_key("NEW"));
        assert_eq!(result.vars.get("NEW").unwrap().value, "val");
    }

    #[test]
    fn revert_diff_removes_added() {
        let snap = make_snapshot(&[]);
        let diff = vec![DiffEntry {
            name: "NEW".into(),
            scope: "user".into(),
            change: ChangeType::Added,
            old_value: None,
            new_value: Some("val".into()),
        }];
        let applied = apply_diff(&snap, &diff);
        let reverted = revert_diff(&applied, &diff);
        assert!(!reverted.vars.contains_key("NEW"));
    }

    #[test]
    fn change_count_correct() {
        let old = make_snapshot(&[("A", "1")]);
        let new = make_snapshot(&[("A", "2"), ("B", "3"), ("C", "4")]);
        assert_eq!(change_count(&old, &new), 3);
    }
}
