//! Web server module for Aetheris.
pub mod api;
pub mod graphql;
pub mod middleware;
pub mod static_files;
pub mod websocket;

pub struct WebServer;

impl WebServer {
    pub fn new() -> Self {
        Self
    }
    pub fn bind(&self, _addr: &str) {
        unimplemented!("bind")
    }
}

impl Default for WebServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_web_server_new() {
        let _s = WebServer::new();
    }
}
pub fn serve_http() -> () {}
