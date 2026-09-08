FFI wire: rust ffi bridge defined
pub fn auth_register(email: String, pw: String) -> String { format!("register: {}", email) }
pub fn auth_login(email: String, pw: String) -> String { format!("login: {}", email) }
