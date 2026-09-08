//! UI / UX smoke tests — browser plugin popup + options
use std::path::Path;

#[test]
fn popup_has_dark_theme_and_interactive_elements() {
    let html = std::fs::read_to_string("browser/chrome/popup.html").expect("readable");
    assert!(html.contains("Aetheris"));
    assert!(html.contains("Vault Status"));
    assert!(html.contains("btn") || html.contains("button"));
    assert!(html.contains("#090D16") || html.contains("background:var(--bg)"));
}

#[test]
fn options_has_form_elements() {
    let html = std::fs::read_to_string("browser/chrome/options.html").expect("readable");
    assert!(html.contains("relay-url"));
    assert!(html.contains("sync-mode"));
    assert!(html.contains("saveOptions"));
}

#[test]
fn manifest_has_action_and_background() {
    let raw = std::fs::read_to_string("browser/chrome/manifest.json").expect("readable");
    assert!(raw.contains("action"));
    assert!(raw.contains("background"));
    assert!(raw.contains("options_page"));
}
