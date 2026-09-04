//! Aetheris CLI — The Secrets Operating System
use anyhow::Result;
use clap::Parser;

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
    Connect { #[arg(long)] host: String, #[arg(long)] user: String, #[arg(long)] port: Option<u16>, #[arg(long)] key: Option<String> },
    Add { #[arg(long)] title: String, #[arg(long)] host: String, #[arg(long)] user: String },
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
            SshAction::List => println!("Listing SSH connections..."),
            SshAction::Connect { host, user, port, key: _ } => println!("Connecting to {user}@{host}{}...", port.map_or(String::new(), |p| format!(":{p}"))),
            SshAction::Add { title, host, user } => println!("Adding connection: {title} ({user}@{host})..."),
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

