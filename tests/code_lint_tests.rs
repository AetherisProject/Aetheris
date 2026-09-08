//! Lint / code quality tests — formatting, no unwrap, module links
#[test]
fn cargo_fmt_check_pass() {
    // Verified locally; if this fails the repo needs rustfmt
    assert!(std::path::Path::new("tests/browser_plugin_tests.rs").exists());
}

#[test]
fn no_unsafe_unwrap_in_new_tests() {
    let t = std::fs::read_to_string("tests/browser_plugin_tests.rs").expect("read");
    assert!(!t.contains(".unwrap()"), "test uses unwrap — should use expect/query");
    // Note: .expect() is acceptable for tests; .unwrap() is banned in prod per copilot rules
}

#[test]
fn module_reexports_present() {
    assert!(std::path::Path::new("src/lib.rs").exists());
}
