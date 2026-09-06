//! Vault item types and definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The unified vault item type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VaultItem {
    Password(PasswordItem),
    SshKey(SshKeyItem),
    SshConnection(SshConnectionItem),
    ApiKey(ApiKeyItem),
    Note(NoteItem),
    Card(CardItem),
    Identity(IdentityItem),
}

/// Password item for website credentials.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PasswordItem {
    pub id: Uuid,
    pub title: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub totp_secret: Option<String>,
    pub notes: String,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// SSH key item with encrypted private key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SshKeyItem {
    pub id: Uuid,
    pub title: String,
    pub private_key: String,
    pub public_key: String,
    pub key_type: SshKeyType,
    pub fingerprint: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// SSH connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SshConnectionItem {
    pub id: Uuid,
    pub title: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub key_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub last_connected: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// API key item with provider and rotation policy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiKeyItem {
    pub id: Uuid,
    pub title: String,
    pub provider: crate::apikey::Provider,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub scopes: Vec<String>,
    pub rotation_strategy: crate::apikey::RotationStrategy,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
    pub max_usage: Option<u64>,
    pub notes: String,
    pub disabled: bool,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Encrypted note item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteItem {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Credit/debit card item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardItem {
    pub id: Uuid,
    pub title: String,
    pub number: String,
    pub expiry: String,
    pub cvv: String,
    pub name: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Identity item for autofill forms.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdentityItem {
    pub id: Uuid,
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// SSH key type enumeration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SshKeyType {
    Ed25519,
    Rsa,
    Ecdsa,
}

impl VaultItem {
    pub fn id(&self) -> &Uuid {
        match self {
            VaultItem::Password(i) => &i.id,
            VaultItem::SshKey(i) => &i.id,
            VaultItem::SshConnection(i) => &i.id,
            VaultItem::ApiKey(i) => &i.id,
            VaultItem::Note(i) => &i.id,
            VaultItem::Card(i) => &i.id,
            VaultItem::Identity(i) => &i.id,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            VaultItem::Password(i) => &i.title,
            VaultItem::SshKey(i) => &i.title,
            VaultItem::SshConnection(i) => &i.title,
            VaultItem::ApiKey(i) => &i.title,
            VaultItem::Note(i) => &i.title,
            VaultItem::Card(i) => &i.title,
            VaultItem::Identity(i) => &i.title,
        }
    }

    pub fn item_type(&self) -> &str {
        match self {
            VaultItem::Password(_) => "password",
            VaultItem::SshKey(_) => "ssh_key",
            VaultItem::SshConnection(_) => "ssh_connection",
            VaultItem::ApiKey(_) => "api_key",
            VaultItem::Note(_) => "note",
            VaultItem::Card(_) => "card",
            VaultItem::Identity(_) => "identity",
        }
    }
}

impl PasswordItem {
    pub fn new(title: String, username: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            username,
            password: String::new(),
            urls: Vec::new(),
            totp_secret: None,
            notes: String::new(),
            tags: Vec::new(),
            favorite: false,
            created_at: now,
            updated_at: now,
            expires_at: None,
        }
    }
}
