//! Browser relay module for Aetheris.
pub mod server;
pub mod protocol;
pub mod auth;
pub mod ssh_proxy;

pub struct BrowserRelay;

impl BrowserRelay {
    pub fn new() -> Self { Self }
    pub fn start(&self) { unimplemented!("start") }
}

impl Default for BrowserRelay {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_browser_relay_new() { let _r = BrowserRelay::new(); }
}
