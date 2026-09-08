//! Sync engine module for Aetheris.
pub mod backup;
pub mod blob;
pub mod client;
pub mod crdt;
pub mod offline;
pub mod realtime;

use anyhow::Result;

/// Sync client for multi-device synchronization.
pub struct SyncClient;

impl SyncClient {
    pub fn new() -> Self {
        Self
    }
    pub fn pull(&self) -> Result<()> {
        unimplemented!("pull")
    }
    pub fn push(&self) -> Result<()> {
        unimplemented!("push")
    }
}

impl Default for SyncClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sync_client_new() {
        let _c = SyncClient::new();
    }
}
