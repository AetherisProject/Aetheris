//! Icon system for Aetheris UI.
//! Provides icon definitions matching the cyber-heraldic brand identity.

use serde::{Deserialize, Serialize};

/// Icon size scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IconSize {
    Xs = 16,   // 16px
    Sm = 20,   // 20px
    Md = 24,   // 24px
    Lg = 32,   // 32px
    Xl = 48,   // 48px
}

impl IconSize {
    pub fn as_px(self) -> u32 { self as u32 }
}

/// Icon names for Aetheris
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Icon {
    // Core icons
    Shield,
    Key,
    Lock,
    Unlock,
    Vault,
    
    // SSH icons
    Terminal,
    Server,
    Network,
    Tunnel,
    
    // API key icons
    Api,
    Cloud,
    Sync,
    Refresh,
    
    // Navigation
    Home,
    Search,
    Settings,
    Menu,
    Close,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronDown,
    
    // Actions
    Add,
    Edit,
    Delete,
    Copy,
    Paste,
    Download,
    Upload,
    
    // Status
    Check,
    Warning,
    Error,
    Info,
    
    // Brand
    Logo,
    Crest,
    Crown,
}

impl Icon {
    pub fn name(&self) -> &str {
        match self {
            Icon::Shield => "shield",
            Icon::Key => "key",
            Icon::Lock => "lock",
            Icon::Unlock => "unlock",
            Icon::Vault => "vault",
            Icon::Terminal => "terminal",
            Icon::Server => "server",
            Icon::Network => "network",
            Icon::Tunnel => "tunnel",
            Icon::Api => "api",
            Icon::Cloud => "cloud",
            Icon::Sync => "sync",
            Icon::Refresh => "refresh",
            Icon::Home => "home",
            Icon::Search => "search",
            Icon::Settings => "settings",
            Icon::Menu => "menu",
            Icon::Close => "close",
            Icon::ChevronLeft => "chevron-left",
            Icon::ChevronRight => "chevron-right",
            Icon::ChevronUp => "chevron-up",
            Icon::ChevronDown => "chevron-down",
            Icon::Add => "add",
            Icon::Edit => "edit",
            Icon::Delete => "delete",
            Icon::Copy => "copy",
            Icon::Paste => "paste",
            Icon::Download => "download",
            Icon::Upload => "upload",
            Icon::Check => "check",
            Icon::Warning => "warning",
            Icon::Error => "error",
            Icon::Info => "info",
            Icon::Logo => "logo",
            Icon::Crest => "crest",
            Icon::Crown => "crown",
        }
    }
    
    pub fn svg_path(&self) -> &str {
        // Simplified SVG paths for demo purposes
        match self {
            Icon::Shield => "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z",
            Icon::Key => "M21 10h-8.35A5.99 5.99 0 0 0 7 6c-3.31 0-6 2.69-6 6s2.69 6 6 6a5.99 5.99 0 0 0 5.65-4H17v4h4v-4h2v-4h-2z M7 15c-1.65 0-3-1.35-3-3s1.35-3 3-3 3 1.35 3 3-1.35 3-3 3z",
            Icon::Lock => "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z",
            Icon::Unlock => "M12 17c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm6-9h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zM8.9 6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2H8.9V6zM18 20H6V10h12v10z",
            Icon::Vault => "M20 6h-3V4c0-1.1-.9-2-2-2H9c-1.1 0-2 .9-2 2v2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zM9 4h6v2H9V4zm11 16H4V8h16v12z",
            Icon::Terminal => "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM4 18V6h16v12H4zm8-9h6v2h-6V9zm-4 0l4 4-4 4V9z",
            Icon::Server => "M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1zM4 3h16c.55 0 1 .45 1 1v6c0 .55-.45 1-1 1H4c-.55 0-1-.45-1-1V4c0-.55.45-1 1-1z",
            Icon::Network => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z",
            Icon::Tunnel => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-5-9h10v2H7z",
            Icon::Api => "M9 11.75c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zm6 0c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm0-14c-3.31 0-6 2.69-6 6 0 2.22 1.21 4.15 3 5.19l1-1.74c-1.19-.7-2-1.97-2-3.45 0-2.21 1.79-4 4-4s4 1.79 4 4c0 1.48-.81 2.75-2 3.45l1 1.74c1.79-1.04 3-2.97 3-5.19 0-3.31-2.69-6-6-6z",
            Icon::Cloud => "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z",
            Icon::Sync => "M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z",
            Icon::Refresh => "M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z",
            Icon::Home => "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z",
            Icon::Search => "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            Icon::Settings => "M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.24-.47.47l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z",
            Icon::Menu => "M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z",
            Icon::Close => "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            Icon::ChevronLeft => "M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z",
            Icon::ChevronRight => "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
            Icon::ChevronUp => "M7.41 15.41L12 10.83l4.59 4.58L18 14l-6-6-6 6z",
            Icon::ChevronDown => "M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6z",
            Icon::Add => "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z",
            Icon::Edit => "M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z",
            Icon::Delete => "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z",
            Icon::Copy => "M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z",
            Icon::Paste => "M19 2h-4.18C14.4.84 13.3 0 12 0c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm7 18H5V4h2v3h10V4h2v16z",
            Icon::Download => "M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z",
            Icon::Upload => "M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z",
            Icon::Check => "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z",
            Icon::Warning => "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
            Icon::Error => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z",
            Icon::Info => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z",
            Icon::Logo => "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5",
            Icon::Crest => "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z",
            Icon::Crown => "M5 16L3 5l5.5 5L12 4l3.5 6L21 5l-2 11h-14z",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_icon_names() {
        assert_eq!(Icon::Shield.name(), "shield");
        assert_eq!(Icon::Key.name(), "key");
    }
    
    #[test]
    fn test_icon_sizes() {
        assert_eq!(IconSize::Sm.as_px(), 20);
        assert_eq!(IconSize::Md.as_px(), 24);
    }
}