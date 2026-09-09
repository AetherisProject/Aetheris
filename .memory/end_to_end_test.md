# End-to-End Testing for Aetheris

## Overview
This document outlines the end-to-end testing workflow for Aetheris, ensuring all features work together seamlessly.

## Setup
### Initialize Vault Store
```rust
use aetheris::vault::store::VaultStore;
use aetheris::crypto::CryptoEngine;

let vault_store = VaultStore::new(".vault_test_endtoend");
let crypto_engine = CryptoEngine::new()?;
let master_key = crypto_engine.generate_key()?;
vault_store.initialize(master_key)?;
```

### Initialize Session Store
```rust
use aetheris::vault::store::VaultStore;

let session_store = VaultStore::new(".session_store_test");
session_store.initialize(master_key)?;
```

## Workflow
### Insert Password Item
```rust
use aetheris::vault::item::PasswordItem;
use uuid::Uuid;

let password_item = PasswordItem {
    id: Uuid::new_v4(),
    title: "Test Password".to_string(),
    password: "test_password".to_string(),
    notes: "Test notes".to_string(),
};

vault_store.insert(password_item)?;
```

### Insert API Key
```rust
use aetheris::vault::item::ApiKeyItem;
use aetheris::auth::oauth::Provider;
use aetheris::proactive::policies::PolicyType;

let api_key_item = ApiKeyItem {
    id: Uuid::new_v4(),
    title: "Test API Key".to_string(),
    provider: Provider::Github,
    api_key: "test_api_key".to_string(),
    api_secret: None,
    scopes: vec!["read"],
    rotation_strategy: PolicyType::Manual,
    last_used: None,
    usage_count: 0,
    max_usage: None,
    notes: "Test notes".to_string(),
    disabled: false,
    tags: vec!["test"],
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    expires_at: None,
};

vault_store.insert(api_key_item)?;
```

### Create and Validate Session
```rust
use aetheris::auth::oauth::{OAuth2Client, Provider, TokenResponse};
use aetheris::auth::session::Session;

let token_response = TokenResponse {
    access_token: "test_token".to_string(),
    token_type: "Bearer".to_string(),
    expires_in: 3600,
    scope: "read write".to_string(),
    refresh_token: None,
};

let oauth2_client = OAuth2Client::new(Provider::GitHub, vault_store.clone(), session_store.clone());
let session = oauth2_client.create_session(&token_response).await?;

let session_valid = oauth2_client.validate_session(session.id).await?;
assert!(session_valid);
```

### Test Proactive Monitoring
```rust
use aetheris::proactive::monitor::MonitorConfig;
use aetheris::proactive::policies::Policy;

let monitor_config = MonitorConfig::new(vault_store.clone());
let policy = Policy {
    id: Uuid::new_v4(),
    type_: PolicyType::SuspiciousActivity,
    name: "Test Policy".to_string(),
    description: "Test description".to_string(),
    enabled: true,
    threshold: 10,
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
};

monitor_config.add_policy(policy)?;
```

### Sync Data
```rust
vault_store.sync_data(Uuid::new_v4())?;
```

## Cleanup
```rust
os.remove(".vault_test_endtoend");
os.remove(".session_store_test");
```

## Conclusion
This workflow ensures that all features of Aetheris work together seamlessly, validating the end-to-end functionality.

### Next Steps
- Run automated tests for each feature to ensure robustness.
- Validate security compliance and performance.
- Gather user feedback for further improvements.