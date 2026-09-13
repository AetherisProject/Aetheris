use clap::{Command, Arg, ArgAction};
use aetheris_core::{VaultItem, CryptoEngine, SshClient};

fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut hex = String::new();
    for b in bytes {
        write!(&mut hex, "{:02x}", b).expect("write to string");
    }
    hex
}

fn from_hex_or_raw(input: &str) -> Vec<u8> {
    let hex: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if hex.len().is_multiple_of(2) && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap_or(0))
            .collect()
    } else {
        input.as_bytes().to_vec()
    }
}

fn print_bytes(prefix: &str, bytes: &[u8]) {
    println!("{}: {}", prefix, to_hex(bytes));
}

fn generate_keypair() -> Result<(), String> {
    let engine = CryptoEngine::new()?;
    let (pk, sk) = engine.generate_kyber_keypair()?;
    print_bytes("public_key", &pk);
    print_bytes("secret_key", &sk);
    Ok(())
}

fn encrypt(data: &str, public_key: &str) -> Result<(), String> {
    let engine = CryptoEngine::new()?;
    let key = from_hex_or_raw(public_key);
    let ciphertext = engine.hybrid_encrypt(data.as_bytes(), &key)?;
    print_bytes("ciphertext", &ciphertext);
    Ok(())
}

fn decrypt(ciphertext: &str, secret_key: &str) -> Result<(), String> {
    let engine = CryptoEngine::new()?;
    let ct = from_hex_or_raw(ciphertext);
    let key = from_hex_or_raw(secret_key);
    let plaintext = engine.hybrid_decrypt(&ct, &key)?;
    println!("plaintext: {}", String::from_utf8_lossy(&plaintext));
    Ok(())
}

fn vault_add(name: &str, data: &str, tags: &[&str]) -> Result<(), String> {
    let item = VaultItem::new(name.to_string(), data.as_bytes().to_vec(), tags.iter().map(|t| t.to_string()).collect());
    print_bytes("serialized", &item.serialize()?);
    Ok(())
}

fn vault_get(serialized: &str) -> Result<(), String> {
    let bytes = from_hex_or_raw(serialized);
    let item = VaultItem::deserialize(&bytes)?;
    println!("id: {}", item.id);
    println!("name: {}", item.name);
    println!("data: {}", String::from_utf8_lossy(&item.data));
    println!("tags: {:?}", item.tags);
    println!("created_at: {}", item.created_at);
    println!("updated_at: {}", item.updated_at);
    Ok(())
}

fn ssh_execute(host: &str, port: u16, command: &str) -> Result<(), String> {
    let mut client = SshClient::new(host.to_string(), port);
    client.spawn_channel()?;
    println!("{}", client.execute(command)?);
    Ok(())
}

fn main() -> Result<(), String> {
    let matches = Command::new("aetheris-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Aetheris secrets management CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate-keypair")
                .about("Generate a Kyber keypair")
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt data with a public key")
                .arg(Arg::new("data").required(true).index(1))
                .arg(Arg::new("public_key").required(true).index(2))
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt ciphertext with a secret key")
                .arg(Arg::new("ciphertext").required(true).index(1))
                .arg(Arg::new("secret_key").required(true).index(2))
        )
        .subcommand(
            Command::new("vault-add")
                .about("Create a new vault item")
                .arg(Arg::new("name").required(true).index(1))
                .arg(Arg::new("data").required(true).index(2))
                .arg(Arg::new("tags").num_args(0..).action(ArgAction::Append))
        )
        .subcommand(
            Command::new("vault-get")
                .about("Deserialize a serialized vault item")
                .arg(Arg::new("serialized").required(true).index(1))
        )
        .subcommand(
            Command::new("ssh")
                .about("Connect to an SSH host and run a command")
                .arg(Arg::new("host").required(true).index(1))
                .arg(Arg::new("port").required(true).index(2))
                .arg(Arg::new("command").required(true).index(3))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate-keypair", _)) => generate_keypair(),
        Some(("encrypt", sub)) => {
            let data = sub.get_one::<String>("data").expect("data required");
            let key = sub.get_one::<String>("public_key").expect("public key required");
            encrypt(data, key)
        }
        Some(("decrypt", sub)) => {
            let ct = sub.get_one::<String>("ciphertext").expect("ciphertext required");
            let key = sub.get_one::<String>("secret_key").expect("secret key required");
            decrypt(ct, key)
        }
        Some(("vault-add", sub)) => {
            let name = sub.get_one::<String>("name").expect("name required");
            let data = sub.get_one::<String>("data").expect("data required");
            let tags: Vec<&str> = sub.get_many::<String>("tags").map(|v| v.map(|s| s.as_str()).collect()).unwrap_or_default();
            vault_add(name, data, &tags)
        }
        Some(("vault-get", sub)) => {
            let serialized = sub.get_one::<String>("serialized").expect("serialized required");
            vault_get(serialized)
        }
        Some(("ssh", sub)) => {
            let host = sub.get_one::<String>("host").expect("host required");
            let port: u16 = sub.get_one::<String>("port").expect("port required").parse().map_err(|e| format!("invalid port: {}", e))?;
            let command = sub.get_one::<String>("command").expect("command required");
            ssh_execute(host, port, command)
        }
        _ => Err("no subcommand provided".to_string()),
    }
}