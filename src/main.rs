//! Aetheris CLI — The Secrets Operating System
use aetheris::apikey::{ApiKeyManager, Provider};
use aetheris::crypto::{generate_master_key, EncryptionKey};
use aetheris::ssh::{
    probe_ssh_endpoint, PortForward, SshClient, SshConfig, SshKeypair,
};
use aetheris::sync::SyncClient;
use aetheris::vault::{PasswordItem, VaultItem, VaultStore};
use anyhow::Result;
use clap::Parser;
use std::time::Duration;
#[derive(Parser)]
#[command(name = "aeth", version, about = "Aetheris: The Secrets Operating System")]
struct Cli {
    #[arg(long, default_value = "aetheris.toml")]
    config: String,
    #[arg(long, short = 'd', global = true)]
    debug: bool,
    #[arg(long, global = true, default_value = "aetheris.log")]
    log_file: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Init,
    Vault {
        #[command(subcommand)]
        action: VaultAction,
    },
    Ssh {
        #[command(subcommand)]
        action: SshAction,
    },
    ApiKey {
        #[command(subcommand)]
        action: ApiKeyAction,
    },
    Sync {
        #[command(subcommand)]
        action: SyncAction,
    },
    Serve,
}

#[derive(clap::Subcommand)]
enum VaultAction {
    List { #[arg(long)] item_type: Option<String> },
    Get { title: String, #[arg(long)] field: Option<String> },
    Add { #[arg(long)] item_type: String, #[arg(long)] title: String, #[arg(long)] username: Option<String> },
    Update { title: String, #[arg(long)] field: String },
    Delete { title: String },
    Search { query: String },
    Generate { #[arg(long, default_value = "32")] length: usize },
}

#[derive(clap::Subcommand)]
enum SshAction {
    List,
    Connect {
        #[arg(long)]
        host: String,
        #[arg(long)]
        user: String,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        inject_env: Vec<String>,
    },
    Add {
        #[arg(long)]
        title: String,
        #[arg(long)]
        host: String,
        #[arg(long)]
        user: String,
    },
    Keygen {
        #[arg(long)]
        comment: Option<String>,
    },
    Probe {
        #[arg(long)]
        host: String,
        #[arg(long, default_value = "22")]
        port: u16,
    },
    Tunnel {
        #[arg(long)]
        local_port: u16,
        #[arg(long)]
        remote_host: String,
        #[arg(long)]
        remote_port: u16,
    },
}

#[derive(clap::Subcommand)]
enum ApiKeyAction {
    List,
    Add { #[arg(long)] provider: String, key: String },
    Rotate { provider: String },
    Health,
}

#[derive(clap::Subcommand)]
enum SyncAction {
    Pull,
    Push,
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(if cli.debug { "aetheris=debug" } else { "aetheris=info" })
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match cli.command {
        Commands::Init => {
            println!("Initializing Aetheris vault...");
            let (master_key_bytes, _salt) = generate_master_key(b"")?;
            let master_key = EncryptionKey(master_key_bytes);
            let mut store = VaultStore::new("aetheris_vault")?;
            store.initialize(master_key)?;
            println!("✔ Vault initialized at aetheris_vault/");
        }
        Commands::Vault { action } => {
            let mut store = VaultStore::new("aetheris_vault")?;
            match action {
                VaultAction::List { item_type } => {
                    let items = store.list()?;
                    let filtered: Vec<_> = if let Some(t) = item_type {
                        items.into_iter().filter(|i| i.item_type() == t).collect()
                    } else {
                        items
                    };
                    println!("Vault items ({}):", filtered.len());
                    for item in filtered {
                        println!("  - [{}] {}", item.item_type(), item.id());
                    }
                }
                VaultAction::Get { title, field } => {
                    let items = store.search(&title)?;
                    if let Some(item) = items.first() {
                        match field.as_deref() {
                            Some("password") => {
                                if let VaultItem::Password(p) = item {
                                    println!("Password: {}", p.password);
                                }
                            }
                            Some("username") => {
                                if let VaultItem::Password(p) = item {
                                    println!("Username: {}", p.username);
                                }
                            }
                            _ => println!("{:?}", item),
                        }
                    } else {
                        println!("Item not found: {title}");
                    }
                }
                VaultAction::Add { item_type, title, username } => {
                    let item = match item_type.as_str() {
                        "password" => {
                            let mut pwd = PasswordItem::new(title.clone(), username.unwrap_or_default());
                            pwd.password = rpassword::prompt_password("Enter password: ")?;
                            VaultItem::Password(pwd)
                        }
                        _ => return Err(anyhow::anyhow!("Unsupported item type: {item_type}")),
                    };
                    store.insert(item)?;
                    println!("✔ Added {item_type}: {title}");
                }
                VaultAction::Update { title, field } => {
                    let items = store.search(&title)?;
                    if let Some(item) = items.first() {
                        let mut updated = item.clone();
                        match &mut updated {
                            VaultItem::Password(p) => {
                                p.password = rpassword::prompt_password("Enter new password: ")?;
                            }
                            _ => return Err(anyhow::anyhow!("Unsupported field: {field}")),
                        }
                        store.update(updated)?;
                        println!("✔ Updated {title}.{field}");
                    }
                }
                VaultAction::Delete { title } => {
                    let items = store.search(&title)?;
                    if let Some(item) = items.first() {
                        store.delete(&item.id())?;
                        println!("✔ Deleted {title}");
                    }
                }
                VaultAction::Search { query } => {
                    let items = store.search(&query)?;
                    println!("Search results ({}):", items.len());
                    for item in items {
                        println!("  - [{}] {}", item.item_type(), item.id());
                    }
                }
                VaultAction::Generate { length } => {
                    let password: String = (0..length)
                        .map(|_| {
                            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*"
                                .chars()
                                .nth(rand::random::<usize>() % 76)
                                .unwrap()
                        })
                        .collect();
                    println!("Generated password ({length} chars): {password}");
                }
            }
        }
        Commands::Ssh { action } => match action {
            SshAction::List => {
                let store = VaultStore::new("aetheris_vault")?;
                let items = store.list()?;
                let ssh_items: Vec<_> = items
                    .into_iter()
                    .filter(|i| matches!(i, VaultItem::SshConnection(_)))
                    .collect();
                println!("SSH connections ({}):", ssh_items.len());
                for item in ssh_items {
                    println!("  - {}", item.id());
                }
            }
            SshAction::Connect { host, user, port, key, inject_env } => {
                let port_num = port.unwrap_or(22);
                println!("Connecting to {user}@{host}:{port_num}...");
                let config = SshConfig {
                    host: host.clone(),
                    port: port_num,
                    username: user.clone(),
                    private_key_path: key,
                    password: Some("env-or-vault".into()),
                    ..Default::default()
                };
                let client = SshClient::with_config(config);
                match client.connect().await {
                    Ok(mut session) => {
                        for item in inject_env {
                            if let Some((k, v)) = item.split_once('=') {
                                session.inject_secret_env(k, v);
                                println!("⚡ Injected secret environment variable '{k}' directly into memory");
                            }
                        }
                        println!("✔ Connected to {} (Session ID: {})", session.host, session.session_id);
                        println!("Session active. (Press Ctrl+C to terminate)");
                    }
                    Err(e) => eprintln!("Failed to connect: {e}"),
                }
            }
            SshAction::Add { title, host, user } => {
                let mut store = VaultStore::new("aetheris_vault")?;
                let conn = aetheris::vault::SshConnectionItem::new(title.clone(), host.clone(), user.clone());
                store.insert(VaultItem::SshConnection(conn))?;
                println!("✔ Added connection: {title} ({user}@{host}) to secure vault.");
            }
            SshAction::Keygen { comment } => {
                let keypair = SshKeypair::generate_ed25519(comment.as_deref());
                let pub_key = keypair.to_openssh_public_key()?;
                let fp = keypair.fingerprint_sha256();
                println!("✔ Generated Ed25519 SSH Keypair");
                println!("Fingerprint: {fp}");
                println!("Public Key:\n{pub_key}");
            }
            SshAction::Probe { host, port } => {
                println!("Probing TCP reachability and latency for {host}:{port}...");
                let report = probe_ssh_endpoint(&host, port, Duration::from_secs(5)).await;
                println!("Status: {:?}", report.status);
                if let Some(rtt) = report.round_trip_ms {
                    println!("Latency: {rtt}ms");
                }
                if let Some(err) = report.error_message {
                    println!("Error: {err}");
                }
            }
            SshAction::Tunnel { local_port, remote_host, remote_port } => {
                let mut tunnel = PortForward::local(local_port, remote_host.clone(), remote_port);
                tunnel.activate();
                println!("✔ Tunnel active: 127.0.0.1:{local_port} -> {remote_host}:{remote_port}");
            }
        },
        Commands::ApiKey { action } => {
            let mut manager = ApiKeyManager::new();
            match action {
                ApiKeyAction::List => {
                    let keys = manager.list_all_keys();
                    println!("API keys ({}):", keys.len());
                    for key in keys {
                        println!("  - [{}] {} (enabled: {})", key.id, key.provider.to_string(), key.enabled);
                    }
                }
                ApiKeyAction::Add { provider, key } => {
                    let provider_enum = match provider.to_lowercase().as_str() {
                        "openai" => Provider::Openai,
                        "anthropic" => Provider::Anthropic,
                        "google" => Provider::Google,
                        "aws" => Provider::Aws,
                        "github" => Provider::Github,
                        "gitlab" => Provider::Gitlab,
                        "azure" => Provider::Azure,
                        "nvidia" => Provider::Nvidia,
                        "huggingface" => Provider::Huggingface,
                        "mistral" => Provider::Mistral,
                        "openrouter" => Provider::Openrouter,
                        "groq" => Provider::Groq,
                        "cohere" => Provider::Cohere,
                        "stability" => Provider::Stability,
                        _ => Provider::Custom(provider.clone()),
                    };
                    let config = aetheris::apikey::ApiKeyConfig::new(provider_enum, key);
                    let id = manager.add_key(config)?;
                    println!("✔ Added {provider} key: {id}");
                }
                ApiKeyAction::Rotate { provider } => {
                    let provider_enum = match provider.to_lowercase().as_str() {
                        "openai" => Provider::Openai,
                        "anthropic" => Provider::Anthropic,
                        "google" => Provider::Google,
                        "aws" => Provider::Aws,
                        "github" => Provider::Github,
                        "gitlab" => Provider::Gitlab,
                        "azure" => Provider::Azure,
                        "nvidia" => Provider::Nvidia,
                        "huggingface" => Provider::Huggingface,
                        "mistral" => Provider::Mistral,
                        "openrouter" => Provider::Openrouter,
                        "groq" => Provider::Groq,
                        "cohere" => Provider::Cohere,
                        "stability" => Provider::Stability,
                        _ => Provider::Custom(provider.clone()),
                    };
                    let key_ids: Vec<_> = manager.list_keys_by_provider(&provider_enum)
                        .iter()
                        .map(|k| k.id)
                        .collect();
                    if let Some(key_id) = key_ids.first() {
                        let new_key = manager.rotate(key_id)?;
                        println!("✔ Rotated {provider} key to: {new_key}");
                    } else {
                        println!("No keys found for provider: {provider}");
                    }
                }
                ApiKeyAction::Health => {
                    let key_ids: Vec<_> = manager.list_all_keys()
                        .iter()
                        .map(|k| k.id)
                        .collect();
                    println!("API key health ({}):", key_ids.len());
                    for key_id in key_ids {
                        let health = manager.check_health(&key_id)?;
                        println!("  - [{}] {:?}", key_id, health);
                    }
                }
            }
        }
        Commands::Sync { action } => {
            let client = SyncClient::new();
            match action {
                SyncAction::Pull => {
                    client.pull()?;
                    println!("✔ Pulled changes from sync server");
                }
                SyncAction::Push => {
                    client.push()?;
                    println!("✔ Pushed changes to sync server");
                }
                SyncAction::Status => {
                    println!("Sync status: not configured");
                }
            }
        }
        Commands::Serve => {
            println!("Starting Aetheris web server...");
            // TODO: Implement web server startup
        }
    }
    Ok(())
}
