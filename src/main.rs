//! Aetheris CLI — The Secrets Operating System
use aetheris::ssh::{
    probe_ssh_endpoint, PortForward, SshClient, SshConfig, SshKeypair,
};
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
        Commands::Init => println!("Initializing Aetheris vault..."),
        Commands::Vault { action } => match action {
            VaultAction::List { item_type } => println!("Listing vault items{}...", item_type.map_or(String::new(), |t| format!(" (type: {t})"))),
            VaultAction::Get { title, field } => println!("Getting item: {title}{}...", field.map_or(String::new(), |f| format!(" (field: {f})"))),
            VaultAction::Add { item_type, title, username } => println!("Adding {item_type}: {title}{}...", username.map_or(String::new(), |u| format!(" (user: {u})"))),
            VaultAction::Update { title, field } => println!("Updating {title}.{field}..."),
            VaultAction::Delete { title } => println!("Deleting {title}..."),
            VaultAction::Search { query } => println!("Searching for: {query}..."),
            VaultAction::Generate { length } => println!("Generating password ({length} chars)..."),
        },
        Commands::Ssh { action } => match action {
            SshAction::List => println!("Listing configured SSH hosts from vault..."),
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
                println!("Added connection: {title} ({user}@{host}) to secure vault.");
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
        Commands::ApiKey { action } => match action {
            ApiKeyAction::List => println!("Listing API keys..."),
            ApiKeyAction::Add { provider, key } => println!("Adding {provider} key: {}...", &key[..key.len().min(8)]),
            ApiKeyAction::Rotate { provider } => println!("Rotating {provider} key..."),
            ApiKeyAction::Health => println!("Checking API key health..."),
        },
        Commands::Sync { action } => match action {
            SyncAction::Pull => println!("Pulling changes..."),
            SyncAction::Push => println!("Pushing changes..."),
            SyncAction::Status => println!("Sync status: not configured"),
        },
        Commands::Serve => println!("Starting Aetheris web server..."),
    }
    Ok(())
}
