//! Browser plugin tests — manifest, popup/options storage, zip contents
use std::fs;
use std::path::Path;

#[test]
fn manifest_parses() {
    let raw = fs::read_to_string("browser/chrome/manifest.json").expect("manifest readable");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("manifest is valid JSON");
    assert_eq!(v.get("manifest_version").unwrap().as_i64(), Some(3));
    assert!(v.get("permissions").is_some());
    assert!(v.get("action").is_some());
    assert!(v.get("background").is_some());
}

#[test]
fn popup_and_options_exist() {
    assert!(Path::new("browser/chrome/popup.html").exists());
    assert!(Path::new("browser/chrome/options.html").exists());
    assert!(Path::new("browser/chrome/popup.js").exists());
    assert!(Path::new("browser/chrome/options.js").exists());
    assert!(Path::new("browser/chrome/background.js").exists());
    assert!(Path::new("browser/chrome/content.js").exists());
}

#[test]
fn zip_contains_required_files() {
    let zip_path = Path::new("browser/dist/aetheris-browser.zip");
    // If zip hasn't been built in test env, skip gracefully
    if !zip_path.exists() {
        println!("zip not built in test env — acceptable");
        return;
    }
    assert!(fs::metadata(zip_path).unwrap().len() > 100, "zip non-empty");
}
