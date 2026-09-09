#![allow(dead_code)]
use aetheris_core::{vault::VaultItem, crypto::CryptoEngine, ssh::SshClient};
use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "aetheris")]
#[command(about = "Aetheris CLI - Unified Secrets Management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
struct Commands {
    #[command(about = "Manage vault items")]
    Vault { action: VaultAction },
    
    #[command(about = "Manage SSH connections")]
    Ssh { action: SshAction },
    
    #[command(about = "Test crypto operations")]
    Crypto { action: CryptoAction },
}

#[derive(Subcommand)]
struct VaultAction {
    #[arg(short, long)]
    add: Option<String>, // name
    #[arg(short, long)]
    data: Option<String>, // data
    #[command(about = "List vault items")]
    list: bool,
}

#[derive(Subcommand)]
struct SshAction {
    #[arg(short, long)]
    connect: bool,
    #[arg(short, long)]
    host: String,
    #[arg(short, long, default_value = "22")]
    port: u16,
}

#[derive(Subcommand)]
struct CryptoAction {
    #[command(about = "Generate keypair")]
    generate: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Vault { action: VaultAction { add, data: _, list: _ } } => {
            if let Some(name) = add {
                let data = data.unwrap_or_else(|| "empty".to_string());
                let item = VaultItem::new(name, data.into_bytes(), vec![]);
                println!("Added vault item: {:?}", item);
            }
        }
        Commands::Ssh { action: SshAction { connect, host, port } } => {
            if connect {
                let client = SshClient::new(host, port);
                match client.connect() {
                    Ok(_) => println!("Connected to {}:{}", host, port),
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
            }
        }
        Commands::Crypto { action: CryptoAction { generate } } => {
            if generate {
                let engine = CryptoEngine::new().expect("Failed to initialize CryptoEngine");
                let (pk, sk) = engine.generate_kyber_keypair().expect("Failed to generate keypair");
                println!("Public Key: {:?}", pk);
                println!("Secret Key: {:?}", sk);
            }
        }
    }
    
    Ok(())
}