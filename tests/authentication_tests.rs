use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use aetheris::auth::oauth::{OAuth2Client, Provider, TokenResponse};
use aetheris::auth::session::{SessionManager, SessionItem};
use aetheris::vault::store::VaultStore;
use aetheris::crypto::CryptoEngine;

#[tokio::test]
async fn test_oauth2_token_exchange() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_oauth")?;
    let session_store = VaultStore::new(".session_store_test")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    session_store.initialize(master_key)?;
    
    let provider = Provider::GitHub;
    let oauth2_client = OAuth2Client::new(provider, vault_store, session_store);
    
    // Exchange code for tokens
    let token_response = TokenResponse {
        access_token: "test_token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        scope: "read write".to_string(),
        refresh_token: None,
    };
    
    // This is a placeholder for actual token exchange logic
    // In a real implementation, this would call the provider's API
    let token_response = oauth2_client.exchange_code("test_code".to_string()).await?;
    
    // Create a session
    let session = oauth2_client.create_session(&token_response).await?;
    
    // Validate session
    let session_valid = oauth2_client.validate_session(session.id).await?;
    assert!(session_valid);
    
    Ok(())
}

#[tokio::test]
async fn test_session_management() -> Result<()> {
    let vault_store = VaultStore::new(".vault_test_session")?;
    let crypto_engine = CryptoEngine::new()?;
    let master_key = crypto_engine.generate_key()?;
    vault_store.initialize(master_key)?;
    
    let session_manager = SessionManager::new(vault_store);
    
    // Create a session
    let session = Session {
        id: Uuid::new_v4(),
        user_id: "user123".to_string(),
        provider: Provider::GitHub,
        access_token: "test_token".to_string(),
        expires_at: Utc::now() + chrono::Duration::hours(1),
        created_at: Utc::now(),
    };
    
    session_manager.create_session(session).await?;
    
    // Get the session
    let session_item = session_manager.get_session(session.id).await?;
    assert!(session_item.is_some());
    
    // Validate the session
    let session_valid = session_manager.validate_session(session.id).await?;
    assert!(session_valid);
    
    // Delete the session
    session_manager.delete_session(session.id).await?;
    
    Ok(())
}