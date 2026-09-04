# Getting Started

## Quick Start

```bash
# Clone the repository
git clone https://github.com/merlin-tribukait/Aetheris.git
cd Aetheris

# Build the project
cargo build --release

# Run the CLI
./target/release/aeth --help

# Initialize your vault
./target/release/aeth vault init

# Add your first password
./target/release/aeth vault add --type password --title "GitHub" --username "you@example.com"

# List your vault items
./target/release/aeth vault list

# Connect via SSH
./target/release/aeth ssh connect --host example.com --user admin
```

## Installation

### From Source

Requirements:
- Rust 1.70+ (install via [rustup](https://rustup.rs))
- For desktop: Node.js 18+ (for React frontend)
- For mobile: Flutter SDK 3.0+

```bash
# Build CLI only
cargo build --release

# Build with all features
cargo build --release --features full

# Build desktop (requires Node.js)
cd src-tauri && npm install && npm run build

# Build mobile (requires Flutter)
cd flutter && flutter build
```

### From Package Manager

```bash
# Homebrew (macOS/Linux)
brew install aetheris

# Cargo
cargo install aetheris

# Windows (winget)
winget install Aetheris
```

## Configuration

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

Edit `.env` with your settings. See [Configuration](configuration.html) for all options.

## First Steps

1. **Initialize your vault** — Set a strong master password
2. **Add your first item** — Password, SSH key, or API key
3. **Set up sync** — Configure S3/Backblaze for cloud sync
4. **Install the browser extension** — For password and API key autofill
5. **Download mobile app** — For on-the-go access

## Next Steps

- Read the [Terminal](terminal.html) guide for SSH connections
- Read the [Vault](vault.html) guide for password management
- Read the [API Keys](api-keys.html) guide for key rotation
- Read the [Sync](sync.html) guide for multi-device sync
- Read the [Security](security.html) guide for the security model
