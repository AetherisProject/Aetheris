# Aetheris

> **Aetheris** — the Secrets Operating System: SSH terminal, password vault, API key manager, and zero-knowledge sync in one tool. One Rust core powers five shells: desktop (Tauri + React), browser extension (MV3 + WASM), mobile (Flutter), web (React), and CLI (`aeth`).

## 📌 Status

**Early development — Milestone M0 (Foundation & Hygiene).** The target
architecture and delivery plan are defined in the
**[Master Design Document](docs/DESIGN.md)**; the verified, honest current
state is tracked in **[STATUS.md](STATUS.md)**.

The plan: `M0 Foundation → M1 MVP Everywhere → M2 Platform Powers → M3 Advanced Security → M4 Polish (1.0)` — see DESIGN §13.

## ✨ Target Feature Set

- 🗄️ **Vault** — passwords, SSH keys & connections, API keys, notes, cards, identities; per-item XChaCha20-Poly1305 encryption; Argon2id key derivation
- 🔄 **Zero-knowledge sync** — offline-first, CRDT-based conflict resolution; the server only ever stores ciphertext
- 🔑 **API key engine** — provider adapters, auto-rotation, health monitoring, env injection without exposure
- 💻 **SSH terminal** — pure-Rust russh client, vault-integrated keys, SFTP, port forwarding
- 🛡️ **Proactive engine** — breach watch, rotation reminders, approval-queue automation
- 🔐 **Security** — Ed25519/X25519 device keys, TOTP, Shamir M-of-N recovery, duress mode, optional post-quantum hybrid mode

## 📋 Documentation

| Document | Purpose |
|---|---|
| **[docs/DESIGN.md](docs/DESIGN.md)** | 🧭 Master design & delivery plan (start here) |
| [STATUS.md](STATUS.md) | Current verified state |
| [ROADMAP.md](ROADMAP.md) | Near-term priority queue |
| [docs/](docs/index.md) | User & platform documentation set |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |

## 🚀 Build & Run

Prerequisites: Rust 1.70+, Node 18+, Flutter 3.10+ (mobile only).

```bash
git clone https://github.com/merlin-tribukait/Aetheris.git
cd Aetheris

# Core + CLI + desktop + web workspace
cargo build --workspace

# Run the CLI
cargo run -p aetheris-cli -- --help
```

## 🧪 Testing

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo audit
```

CI enforces build/test/lint/audit on every PR (see `.github/workflows/`).

## 🔒 Security Rules

- Master password is never stored or logged; all secrets are zeroized on drop
- All encryption/decryption happens in `aetheris-core` — never in UI shells
- The sync server is zero-knowledge: opaque, versioned, encrypted blobs only
- Report vulnerabilities per [SECURITY.md](SECURITY.md)

## 📜 License

Aetheris is licensed under the [MIT License](LICENSE).
