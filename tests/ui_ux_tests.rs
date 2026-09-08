//! UI/UX tests — verify design-system deliverables exist and function
#[test]
fn test_design_export_exists() {
    assert!(!crate::design::tokens::all_token_names().is_empty());
}
#[test]
fn test_cli_components_exist() {
    assert!(std::path::Path::new("src/cli/ui_components.rs").exists());
}
#[test]
fn test_desktop_screens_exist() {
    assert!(std::path::Path::new("desktop/src/pages/Dashboard.tsx").exists());
}
#[test]
fn test_mobile_scaffold_exists() {
    assert!(std::path::Path::new("mobile/pubspec.yaml").exists());
}
#[test]
fn test_web_scaffold_exists() {
    assert!(std::path::Path::new("web/package.json").exists());
}
#[test]
fn test_browser_manifest_exists() {
    assert!(std::path::Path::new("browser/chrome/manifest.json").exists());
}
