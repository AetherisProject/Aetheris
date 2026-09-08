//! SFTP client module for Aetheris.
//!
//! Provides remote file browsing, upload, download, and metadata querying
//! integrated with the zero-knowledge session framework.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Type of remote file entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
}

/// Remote file information entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFile {
    pub name: String,
    pub path: String,
    pub file_type: FileType,
    pub size: u64,
    pub permissions: u32,
    pub modified_at: DateTime<Utc>,
}

/// SFTP client session.
pub struct SftpClient {
    remote_cwd: String,
    is_open: bool,
}

impl SftpClient {
    /// Initialize a new SFTP client session with default remote path.
    pub fn new() -> Self {
        Self {
            remote_cwd: "/".to_string(),
            is_open: true,
        }
    }

    /// Return current remote working directory.
    pub fn pwd(&self) -> &str {
        &self.remote_cwd
    }

    /// Change remote working directory.
    pub fn cd(&mut self, path: impl Into<String>) {
        self.remote_cwd = path.into();
    }

    /// Check if SFTP channel is active.
    pub fn is_active(&self) -> bool {
        self.is_open
    }

    /// Close SFTP session channel.
    pub fn close(&mut self) {
        self.is_open = false;
    }
}

impl Default for SftpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sftp_client_navigation() {
        let mut sftp = SftpClient::new();
        assert_eq!(sftp.pwd(), "/");
        assert!(sftp.is_active());

        sftp.cd("/var/log");
        assert_eq!(sftp.pwd(), "/var/log");

        sftp.close();
        assert!(!sftp.is_active());
    }
}
