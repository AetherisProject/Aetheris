# Aetheris

**The Secrets Operating System**

A unified SSH terminal, password vault, API key manager, and zero-knowledge sync platform — all in one tool. Published under **[aether.merlin-tribukait.com](https://aether.merlin-tribukait.com)**.

> 🎨 **New:** Explore the official **[Aetheris Brand Identity & Web Design Portal](brand.html)** and **[Design System Specifications](design-system.html)**.

## Why Aetheris?

Developers juggle dozens of secrets: passwords, SSH keys, API keys, tokens. They end up duplicated across devices, expire without notice, and live in disconnected tools. Aetheris gives you **one encrypted vault** that syncs everywhere and **maintains itself**.

| Tool | SSH Terminal | Password Vault | API Key Manager | Browser | Mobile | Zero-Knowledge |
|------|:---:|:---:|:---:|:---:|:---:|:---:|
| **Termius** | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ |
| **1Password** | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |
| **Bitwarden** | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |
| **Aetheris** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Key Features

- **SSH Terminal** — Termius-grade terminal with xterm.js, vault-integrated keys, snippets, port forwarding, SFTP
- **Password Vault** — Zero-knowledge encryption, breach monitoring, auto-change, TOTP built-in
- **API Key Manager** — 30+ providers, auto-rotation, health monitoring, auto-injection
- **Browser Extension** — Autofill passwords AND API keys, provider dashboard detection
- **Mobile** — Native iOS/Android with biometric unlock, autofill services, QR sync
- **Web App** — Full vault management, security dashboard, admin interface
- **Proactive Engine** — Auto-rotates, auto-changes, auto-monitors, auto-syncs, auto-backs up

## Security

- **Zero-knowledge** — The server sees only encrypted blobs
- **Shamir Secret Sharing** — M-of-N master key recovery
- **Plausible deniability** — Hidden volumes, duress mode, decoy entries
- **Post-quantum** — Optional CRYSTALS-Kyber/Dilithium hybrid
- **Memory encryption** — Keys encrypted even in RAM
- **Multi-device consensus** — N-device approval for sensitive ops
- **Social recovery** — Trusted contacts can help you recover

## Quick Start

```bash
# Install from source
git clone https://github.com/merlin-tribukait/Aetheris.git
cd Aetheris
cargo build --release

# Run the CLI
./target/release/aeth --help

# Initialize your vault
./target/release/aeth vault init

# Add a password
./target/release/aeth vault add --type password --title "GitHub"

# Connect via SSH
./target/release/aeth ssh connect --host example.com --user admin
```

## Platforms

- **Desktop** — Windows, macOS, Linux (Tauri + React + xterm.js)
- **Browser** — Chrome, Firefox, Safari, Edge, Brave (WebExtension + WASM)
- **Mobile** — iOS, Android (Flutter + Rust FFI)
- **Web** — Any browser (React + Rust API)
- **CLI** — Any platform (ratatui)

## Architecture

Aetheris is built around a Rust core (VaultEngine) that compiles to native code for desktop/mobile/CLI and WASM for browsers. Every frontend binds to the same core, ensuring consistent behavior and security everywhere.

```
VaultEngine (Rust)
    ├── Desktop (Tauri + React)
    ├── Browser (WebExtension + WASM)
    ├── Mobile (Flutter + FFI)
    ├── Web (React + HTTP API)
    └── CLI (ratatui)
```

## License

MIT License — see [LICENSE](../LICENSE) for details. Published under [aether.merlin-tribukait.com](https://aether.merlin-tribukait.com).
