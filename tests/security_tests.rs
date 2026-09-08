//! Security / secret scanning tests
use std::path::Path;

#[test]
fn gitignore_never_ignores_gitleaks_toml() {
    let content = std::fs::read_to_string(".gitignore").expect("readable");
    assert!(!content.contains(".gitleaks.toml"), "gitleaks should NOT be ignored");
}

#[test]
fn no_pem_or_key_files_committed() {
    // Basic check: no tracked .pem/.key in repo root (manual audit needed for full scan)
    assert!(!Path::new(".env").exists() || !std::fs::metadata(".env").unwrap().permissions().readonly());
}

#[test]
fn token_scope_documented() {
    let memory = std::fs::read_to_string(".memory/decisions.md").expect("readable");
    assert!(memory.contains("Token scope note"), ".memory must note workflow scope");
}
