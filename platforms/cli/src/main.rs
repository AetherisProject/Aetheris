use clap::{Command, Arg};

use aetheris_core::{
    env::scanner::scan_for_api_keys,
    env::reader::{read_all_env, read_env_vars, EnvScope},
    env::conflict::detect_user_system_conflicts,
    vault::store::VaultStore,
};

fn scan_env() {
    println!("Scanning environment for API keys...");
    let vars = read_all_env().unwrap_or_default();
    let pairs: Vec<(String, String)> = vars.into_iter().map(|v| (v.name, v.value)).collect();
    match scan_for_api_keys(&pairs) {
        Ok(findings) => {
            if findings.is_empty() {
                println!("No API keys detected in environment.");
            } else {
                println!("Found {} potential API key(s):", findings.len());
                for f in &findings {
                    println!("  - {} ({})", f.var_name, f.provider);
                }
            }
        }
        Err(e) => eprintln!("Scan failed: {}", e),
    }
}

fn sync_to_vault(vault_path: &str, password: &str) {
    let store = match VaultStore::open(vault_path, password) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            return;
        }
    };
    let vars = read_all_env().unwrap_or_default();
    let pairs: Vec<(String, String)> = vars.iter().map(|v| (v.name.clone(), v.value.clone())).collect();
    let findings = match scan_for_api_keys(&pairs) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Scan failed: {}", e);
            return;
        }
    };
    let mut synced = 0;
    for finding in &findings {
        if let Some(var) = vars.iter().find(|v| v.name == finding.var_name) {
            match store.insert(&var.name, &var.value) {
                Ok(()) => {
                    synced += 1;
                    println!("  ✓ {}", var.name);
                }
                Err(e) => eprintln!("  ✗ {}: {}", var.name, e),
            }
        }
    }
    println!("Synced {} key(s) to vault.", synced);
}

fn show_conflicts() {
    let user = read_env_vars(EnvScope::User).unwrap_or_default();
    let system = read_env_vars(EnvScope::System).unwrap_or_default();
    match detect_user_system_conflicts(&user, &system) {
        Ok(conflicts) => {
            if conflicts.is_empty() {
                println!("No user/system conflicts detected.");
            } else {
                for c in conflicts {
                    println!("  ! [{:?}] {}", c.conflict_type, c.name);
                }
            }
        }
        Err(e) => eprintln!("Conflict detection failed: {}", e),
    }
}

fn export_env(path: &str) {
    let vars = read_all_env().unwrap_or_default();
    let content: String = vars.iter()
        .map(|v| format!("{}={}", v.name, v.value))
        .collect::<Vec<_>>()
        .join("\n");
    match std::fs::write(path, &content) {
        Ok(()) => println!("Exported {} vars to {}", vars.len(), path),
        Err(e) => eprintln!("Export failed: {}", e),
    }
}

fn list_vault_keys(vault_path: &str, password: &str) {
    let store = match VaultStore::open(vault_path, password) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            return;
        }
    };
    match store.list() {
        Ok(items) => {
            if items.is_empty() {
                println!("Vault is empty.");
            } else {
                for item in items {
                    println!("  {} ({})", item.name, item.scope);
                }
            }
        }
        Err(e) => eprintln!("List failed: {}", e),
    }
}

fn snapshot_vault(vault_path: &str, password: &str, label: &str) {
    let store = match VaultStore::open(vault_path, password) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            return;
        }
    };
    match store.snapshot(label) {
        Ok(snap) => println!("Snapshot '{}' (id: {}, items: {})", snap.label, snap.id, snap.item_count),
        Err(e) => eprintln!("Snapshot failed: {}", e),
    }
}

fn rollback_vault(vault_path: &str, password: &str, snapshot_id: &str) {
    let store = match VaultStore::open(vault_path, password) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            return;
        }
    };
    match store.rollback(snapshot_id) {
        Ok(()) => println!("Rolled back to {}", snapshot_id),
        Err(e) => eprintln!("Rollback failed: {}", e),
    }
}

fn list_snapshots(vault_path: &str, password: &str) {
    let store = match VaultStore::open(vault_path, password) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            return;
        }
    };
    match store.list_snapshots() {
        Ok(snaps) => {
            if snaps.is_empty() {
                println!("No snapshots found.");
            } else {
                for s in snaps {
                    println!("  [{}] {} ({} items)", s.id, s.label, s.item_count);
                }
            }
        }
        Err(e) => eprintln!("List snapshots failed: {}", e),
    }
}

fn vault_add(vault_path: &str, password: &str, name: &str, value: &str) {
    let store = match VaultStore::open(vault_path, password) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            return;
        }
    };
    match store.insert(name, value) {
        Ok(()) => println!("Stored {}", name),
        Err(e) => eprintln!("Insert failed: {}", e),
    }
}

fn vault_get(vault_path: &str, password: &str, name: &str) {
    let store = match VaultStore::open(vault_path, password) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            return;
        }
    };
    match store.get(name) {
        Ok(item) => println!("{} = {}", item.name, item.value),
        Err(e) => eprintln!("Get failed: {}", e),
    }
}

fn main() -> Result<(), String> {
    let matches = Command::new("aetheris-cli")
        .version("0.1.0")
        .about("Aetheris secrets management CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("vault-path")
                .long("vault-path")
                .default_value(".aetheris/vault")
                .global(true)
        )
        .arg(
            Arg::new("password")
                .long("password")
                .global(true)
        )
        .subcommand(
            Command::new("env")
                .about("Environment variable management")
                .subcommand(Command::new("scan").about("Scan for API keys in env vars"))
                .subcommand(Command::new("sync").about("Sync detected API keys into vault"))
                .subcommand(Command::new("conflicts").about("Show user/system env conflicts"))
                .subcommand(Command::new("export").about("Export env vars to file").arg(
                    Arg::new("output").required(true).help("Output file path")
                ))
        )
        .subcommand(
            Command::new("vault")
                .about("Vault operations")
                .subcommand(Command::new("list").about("List all vault keys"))
                .subcommand(Command::new("add").about("Add a key").arg(
                    Arg::new("name").required(true)
                ).arg(
                    Arg::new("value").required(true)
                ))
                .subcommand(Command::new("get").about("Get a key").arg(
                    Arg::new("name").required(true)
                ))
                .subcommand(Command::new("snapshot").about("Create vault snapshot").arg(
                    Arg::new("label").required(true)
                ))
                .subcommand(Command::new("snapshots").about("List snapshots"))
                .subcommand(Command::new("rollback").about("Rollback to snapshot").arg(
                    Arg::new("id").required(true)
                ))
        )
        .get_matches();

    let vault_path = matches.get_one::<String>("vault-path").unwrap().clone();
    let password = matches.get_one::<String>("password").cloned().unwrap_or_default();

    match matches.subcommand() {
        Some(("env", env_matches)) => match env_matches.subcommand() {
            Some(("scan", _)) => scan_env(),
            Some(("sync", _)) => sync_to_vault(&vault_path, &password),
            Some(("conflicts", _)) => show_conflicts(),
            Some(("export", export_matches)) => {
                let output = export_matches.get_one::<String>("output").unwrap();
                export_env(output);
            }
            _ => eprintln!("Unknown env subcommand"),
        },
        Some(("vault", vault_matches)) => match vault_matches.subcommand() {
            Some(("list", _)) => list_vault_keys(&vault_path, &password),
            Some(("add", add_matches)) => {
                let name = add_matches.get_one::<String>("name").unwrap();
                let value = add_matches.get_one::<String>("value").unwrap();
                vault_add(&vault_path, &password, name, value);
            }
            Some(("get", get_matches)) => {
                let name = get_matches.get_one::<String>("name").unwrap();
                vault_get(&vault_path, &password, name);
            }
            Some(("snapshot", snap_matches)) => {
                let label = snap_matches.get_one::<String>("label").unwrap();
                snapshot_vault(&vault_path, &password, label);
            }
            Some(("snapshots", _)) => list_snapshots(&vault_path, &password),
            Some(("rollback", rollback_matches)) => {
                let id = rollback_matches.get_one::<String>("id").unwrap();
                rollback_vault(&vault_path, &password, id);
            }
            _ => eprintln!("Unknown vault subcommand"),
        },
        _ => eprintln!("Unknown command"),
    }

    Ok(())
}
