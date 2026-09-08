Tauri command: invoke vault
Tauri Vault wire: VaultStore exposed via invoke
// Tauri auth commands exposed to desktop
pub fn invoke_register(email: &str, pw: &str) -> String { format!("register: {}", email) }
pub fn invoke_login(email: &str, pw: &str) -> String { format!("login: {}", email) }
