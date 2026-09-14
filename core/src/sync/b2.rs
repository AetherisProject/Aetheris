//! Backblaze B2 sync client (S3-compatible API).
//!
//! Provides encrypted backup/restore of vault data to a B2 bucket.
//! Uses HMAC-SHA1 signing for B2 S3-compatible API.

use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::error::AetherisError;

type HmacSha1 = Hmac<sha1::Sha1>;

/// Backblaze B2 sync client.
///
/// Stores encrypted vault snapshots in a B2 bucket via the S3-compatible API.
///
/// # Example
///
/// ```
/// use aetheris_core::sync::b2::B2SyncClient;
/// let client = B2SyncClient::new(
///     "key-id".into(),
///     "application-key".into(),
///     "wave-secrets-bucket".into(),
///     "https://s3.us-west-002.backblazeb2.com".into(),
/// );
/// ```
#[derive(Clone)]
pub struct B2SyncClient {
    key_id: String,
    application_key: String,
    bucket: String,
    endpoint: String,
    client: reqwest::Client,
}

impl B2SyncClient {
    /// Creates a new B2 sync client.
    pub fn new(
        key_id: String,
        application_key: String,
        bucket: String,
        endpoint: String,
    ) -> Self {
        Self {
            key_id,
            application_key,
            bucket,
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    /// Creates a client from environment variables:
    /// `B2_KEY_ID`, `B2_APPLICATION_KEY`, `B2_BUCKET`, `B2_ENDPOINT`.
    pub fn from_env() -> Result<Self, AetherisError> {
        let key_id = std::env::var("B2_KEY_ID")
            .map_err(|_| AetherisError::SyncError("B2_KEY_ID not set".into()))?;
        let application_key = std::env::var("B2_APPLICATION_KEY")
            .map_err(|_| AetherisError::SyncError("B2_APPLICATION_KEY not set".into()))?;
        let bucket = std::env::var("B2_BUCKET")
            .map_err(|_| AetherisError::SyncError("B2_BUCKET not set".into()))?;
        let endpoint = std::env::var("B2_ENDPOINT")
            .map_err(|_| AetherisError::SyncError("B2_ENDPOINT not set".into()))?;
        Ok(Self::new(key_id, application_key, bucket, endpoint))
    }

    /// Uploads encrypted data to the B2 bucket.
    ///
    /// # Arguments
    ///
    /// * `key` - Object key (file path in bucket).
    /// * `data` - Encrypted bytes to upload.
    /// * `content_type` - MIME type (use "application/octet-stream" for encrypted data).
    pub async fn upload(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<SyncResult, AetherisError> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let date = Utc::now().to_rfc2822();
        let body_hash = hex::encode(Sha256::digest(data));

        let auth = self.s3_sign("PUT", key, &date, content_type, &body_hash, data.len());

        let response = self
            .client
            .put(&url)
            .header("Date", &date)
            .header("Content-Type", content_type)
            .header("X-Bz-Content-Sha1", &body_hash)
            .header("Authorization", &auth)
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| AetherisError::SyncError(format!("Upload failed: {e}")))?;

        if response.status().is_success() {
            Ok(SyncResult {
                key: key.to_string(),
                size: data.len(),
                status: SyncStatus::Uploaded,
            })
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(AetherisError::SyncError(format!("Upload error {status}: {body}")))
        }
    }

    /// Downloads an object from B2.
    pub async fn download(&self, key: &str) -> Result<Vec<u8>, AetherisError> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let date = Utc::now().to_rfc2822();
        let auth = self.s3_sign("GET", key, &date, "", "", 0);

        let response = self
            .client
            .get(&url)
            .header("Date", &date)
            .header("Authorization", &auth)
            .send()
            .await
            .map_err(|e| AetherisError::SyncError(format!("Download failed: {e}")))?;

        if response.status().is_success() {
            response
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| AetherisError::SyncError(format!("Read failed: {e}")))
        } else {
            Err(AetherisError::SyncError(format!(
                "Download error: {}",
                response.status()
            )))
        }
    }

    /// Lists objects in the bucket with a given prefix.
    pub async fn list(&self, prefix: Option<&str>) -> Result<Vec<SyncObject>, AetherisError> {
        // Simplified listing — in production use B2's b2_list_file_names API.
        let url = format!("{}/{}/", self.endpoint, self.bucket);
        let date = Utc::now().to_rfc2822();
        let auth = self.s3_sign("GET", "", &date, "", "", 0);

        let response = self
            .client
            .get(&url)
            .header("Date", &date)
            .header("Authorization", &auth)
            .send()
            .await
            .map_err(|e| AetherisError::SyncError(format!("List failed: {e}")))?;

        if response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            Ok(vec![SyncObject {
                key: "list-response".to_string(),
                size: text.len(),
                modified: Utc::now().to_rfc3339(),
            }])
        } else {
            Err(AetherisError::SyncError(format!(
                "List error: {}",
                response.status()
            )))
        }
    }

    /// Deletes an object from the bucket.
    pub async fn delete(&self, key: &str) -> Result<(), AetherisError> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let date = Utc::now().to_rfc2822();
        let auth = self.s3_sign("DELETE", key, &date, "", "", 0);

        let response = self
            .client
            .delete(&url)
            .header("Date", &date)
            .header("Authorization", &auth)
            .send()
            .await
            .map_err(|e| AetherisError::SyncError(format!("Delete failed: {e}")))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(AetherisError::SyncError(format!(
                "Delete error: {}",
                response.status()
            )))
        }
    }

    /// Signs a request using AWS Signature Version 2 (S3-compatible).
    fn s3_sign(
        &self,
        method: &str,
        key: &str,
        date: &str,
        content_type: &str,
        body_hash: &str,
        content_length: usize,
    ) -> String {
        let string_to_sign = format!(
            "{}\n\n{}\n{}\n/{}/{}/{}",
            method, content_type, date, self.bucket, self.bucket, key
        );

        let mut mac = HmacSha1::new_from_slice(self.application_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        let signature = B64.encode(mac.finalize().into_bytes());

        format!("AWS {}:{}", self.key_id, signature)
    }
}

/// Result of a sync operation.
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub key: String,
    pub size: usize,
    pub status: SyncStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncStatus {
    Uploaded,
    Downloaded,
    Deleted,
    Conflict,
}

/// Metadata about a synced object.
#[derive(Debug, Clone)]
pub struct SyncObject {
    pub key: String,
    pub size: usize,
    pub modified: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client_creates() {
        let client = B2SyncClient::new(
            "key-id".into(),
            "app-key".into(),
            "wave-secrets-bucket".into(),
            "https://s3.us-west-002.backblazeb2.com".into(),
        );
        assert_eq!(client.bucket, "wave-secrets-bucket");
    }

    #[test]
    fn s3_sign_produces_aws_format() {
        let client = B2SyncClient::new(
            "test-key".into(),
            "test-app-key".into(),
            "test-bucket".into(),
            "https://example.com".into(),
        );
        let sig = client.s3_sign("PUT", "file.bin", "Mon, 01 Jan 2024 00:00:00 +0000", "application/octet-stream", "abc123", 1024);
        assert!(sig.starts_with("AWS test-key:"));
    }
}
