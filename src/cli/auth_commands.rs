//! CLI auth commands — register, login, change password, reset
use crate::auth::AuthManager;

pub fn auth_register(email: &str, password: &str) { println!("register: {}", email); }
pub fn auth_login(email: &str, password: &str) { println!("login: {}", email); }
pub fn auth_change(email: &str, old: &str, new: &str) { println!("change pw: {}", email); }
pub fn auth_reset(email: &str, new_pw: &str) { println!("reset pw: {}", email); }
