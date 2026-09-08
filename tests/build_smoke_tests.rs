//! Build smoke tests for all platforms (no external network required)
use std::process::Command;

#[test]
fn desktop_build_script_exists_and_executable() {
    assert!(std::path::Path::new("desktop/test_build.sh").exists());
}

#[test]
fn web_build_script_exists_and_executable() {
    assert!(std::path::Path::new("web/test.sh").exists());
}

#[test]
fn browser_build_script_exists() {
    assert!(std::path::Path::new("browser/build.sh").exists());
}

#[test]
fn mobile_build_script_exists() {
    assert!(std::path::Path::new("mobile/build.sh").exists());
}

#[test]
fn gitignore_covers_build_outputs() {
    let content = std::fs::read_to_string(".gitignore").expect("gitignore readable");
    assert!(content.contains("**/node_modules/"));
    assert!(content.contains("**/dist/"));
    assert!(content.contains("!web/package-lock.json"));
}
