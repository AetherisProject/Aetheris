//! Aetheris — The Secrets Operating System
//!
//! A unified SSH terminal, password vault, API key manager, and zero-knowledge
//! sync platform. This library provides the core VaultEngine that powers all
//! Aetheris frontends.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod admin;
pub mod apikey;
pub mod auth;
pub mod browser;
pub mod crypto;
pub mod debug;
pub mod design;
pub mod i18n;
pub mod proactive;
pub mod security;
pub mod ssh;
pub mod sync;
pub mod vault;
pub mod web;

pub use crypto::CryptoEngine;
pub use security::{SecureCompare, SecureString, SecureVec};
pub use vault::store::VaultStore;
