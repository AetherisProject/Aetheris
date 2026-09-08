# Aetheris

**The Secrets Operating System**

[![CI](https://github.com/merlin-tribukait/Aetheris/actions/workflows/ci.yml/badge.svg)](https://github.com/merlin-tribukait/Aetheris/actions/workflows/ci.yml)
[![Docs](https://github.com/merlin-tribukait/Aetheris/actions/workflows/docs.yml/badge.svg)](https://merlin-tribukait.github.io/Aetheris/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://rust-lang.org)

A unified SSH terminal, password vault, API key manager, and zero-knowledge sync platform — all in one tool. Better than Termius + 1Password combined.

## Why Aetheris?

| Tool | SSH Terminal | Password Vault | API Key Manager | Browser | Mobile | Zero-Knowledge |
|------|:---:|:---:|:---:|:---:|:---:|:---:|
| **Termius** | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ |
| **1Password** | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |
| **Bitwarden** | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |
| **Aetheris** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Features

- **SSH Terminal** — xterm.js rendering, vault-integrated keys, snippets, SFTP, port forwarding, AI suggestions
- **Password Vault** — Zero-knowledge encryption, breach monitoring, auto-change, TOTP, Shamir sharing
- **API Key Manager** — 30+ providers, auto-rotation, health monitoring, injection
- **Browser Extension** — Password + API key autofill, provider dashboard detection
- **Mobile** — iOS/Android, biometric unlock, autofill, QR sync
- **Proactive Engine** — Auto-rotate, auto-change, auto-monitor, auto-sync

## Security

- **Zero-knowledge** — Server sees only encrypted blobs
- **Shamir Secret Sharing** — M-of-N master key recovery
- **Plausible deniability** — Hidden volumes, duress mode
- **Post-quantum** — CRYSTALS-Kyber/Dilithium hybrid
- **Memory encryption** — Keys encrypted in RAM

## Quick Start

```bash
git clone https://github.com/merlin-tribukait/Aetheris.git
cd Aetheris
cargo build --release
./target/release/aeth --help
./target/release/aeth vault init
```

## Documentation

Full documentation is available at [merlin-tribukait.github.io/Aetheris](https://merlin-tribukait.github.io/Aetheris/).

## Development

```powershell
# Build
.\build.ps1 -Profile release -Clippy -Test -Fmt

# Or with cargo
cargo build --release --features full
cargo test --all-features
cargo clippy --all-features -- -D warnings
```

## License

[MIT](LICENSE)
[![CI](https://github.com/merlin-tribukait/Aetheris/actions/workflows/ci.yml/badge.svg)](https://github.com/merlin-tribukait/Aetheris/actions/workflows/ci.yml)
