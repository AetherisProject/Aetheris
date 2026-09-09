use thiserror::Error;

#[derive(Error, Debug)]
pub enum AetherisError {
    #[error("Vault error: {0}")]
    VaultError(String),
    
    #[error("Crypto error: {0}")]
    CryptoError(String),
    
    #[error("SSH error: {0}")]
    SshError(String),
    
    #[error("Sync error: {0}")]
    SyncError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<bincode::Error> for AetherisError {
    fn from(err: bincode::Error) -> Self {
        AetherisError::SerializationError(err.to_string())
    }
}
