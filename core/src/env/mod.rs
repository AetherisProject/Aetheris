//! Environment variable management for Windows.
//!
//! Reads, writes, monitors, and scans Windows environment variables with
//! focus on API key detection and vault synchronization.

pub mod reader;
pub mod writer;
pub mod monitor;
pub mod scanner;
pub mod snapshot;
pub mod conflict;

// Re-exports
pub use reader::{read_env_vars, read_user_env, read_system_env, read_all_env, read_env_map, EnvScope, EnvVar};
pub use writer::{set_env_var, delete_env_var, rename_env_var, detect_dead_path_entries, WriteResult, WriteAction};
pub use monitor::EnvMonitor;
pub use scanner::{scan_for_api_keys, ApiKeyDetection};
pub use snapshot::{create_snapshot, diff_snapshots, Snapshot as EnvSnapshot};
pub use conflict::{detect_conflicts, Conflict};
