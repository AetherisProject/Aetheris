# Architecture

## Overview

Aetheris is built around a Rust core (VaultEngine) that compiles to native code for desktop/mobile/CLI and WebAssembly for browsers. Every frontend binds to the same core, ensuring consistent behavior and security across all platforms.

## System Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        AETHERIS                                  │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    VaultEngine (Rust)                         ││
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      ││
│  │  │  Crypto  │ │   SSH    │ │   API    │ │  Vault   │      ││
│  │  │  Engine  │ │  Client  │ │   Key    │ │  Store   │      ││
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘      ││
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      ││
│  │  │   Sync   │ │   Auth   │ │ Proactive│ │  Web     │      ││
│  │  │  Client  │ │  System  │ │  Engine  │ │  Server  │      ││
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘      ││
│  └─────────────────────────────────────────────────────────────┘│
│         │              │              │              │          │
│    ┌────┴────┐   ┌─────┴────┐   ┌────┴────┐   ┌───┴────┐     │
│    │ Desktop │   │ Browser  │   │ Mobile  │   │  CLI   │     │
│    │  Tauri  │   │ Ext+WASM │   │ Flutter │   │  Rust  │     │
│    └─────────┘   └──────────┘   └─────────┘   └────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

## Core Modules

### Crypto Engine
All cryptographic operations: key derivation (Argon2id), encryption (XChaCha20-Poly1305), signing (Ed25519), key exchange (X25519), Shamir Secret Sharing, post-quantum hybrid mode, memory encryption.

### Vault Store
Encrypted local storage for all vault items. Supports CRUD, history, search, favorites, tags, and trash. All data is encrypted at rest.

### SSH Client
Pure Rust SSH client (russh) with no OpenSSH dependency. Supports password auth, key auth, agent forwarding, port forwarding, jump hosts, and SFTP.

### API Key Engine
Provider-aware API key management with 30+ providers. Supports auto-rotation, health monitoring, and env var injection.

### Sync Client
S3-compatible sync with end-to-end encryption. Supports CRDT merge, offline-first mode, real-time sync, and encrypted backups.

### Auth System
User accounts with JWT sessions, MFA (TOTP, WebAuthn), OAuth (Google, Apple, GitHub), and Shamir-based recovery.

### Proactive Engine
Auto-rotates API keys, auto-changes breached passwords, auto-monitors health, auto-syncs, auto-backs up, auto-cleans up.

### Web Server
HTTP API for browser extension and web app. Supports REST, WebSocket, CORS, rate limiting, and security headers.

### Browser Relay
Local WebSocket server that lets the browser extension access the desktop app's full power (live sync, SSH proxy).

### Admin Engine
User/team management, audit logs, policy enforcement, billing, and system health.

### i18n Engine
Fluent-based localization with 50+ languages, RTL support, and locale-aware formatting.

### Debug System
Structured logging, performance metrics, diagnostics, state inspection, and export tools.

### Design System
Design tokens, themes, component library, icon system, motion design, and accessibility.

## Data Flow

### Unlock Flow
1. User enters master password
2. Argon2id derives master key (t=3, m=64MB, p=4)
3. HKDF derives vault/sync/auth/backup/recovery keys
4. Vault is decrypted locally
5. Session key is generated for the session

### Sync Flow
1. Vault changes are encrypted with sync key
2. Encrypted blob is uploaded to S3-compatible storage
3. Other devices download the blob
4. Each device decrypts with its sync key
5. CRDT merge resolves any conflicts

### SSH Flow
1. User selects a connection from the vault
2. SSH key is decrypted from the vault
3. russh establishes connection
4. xterm.js renders the terminal in the UI
5. All session data is encrypted in transit

## Security Architecture

See [Security](security.html) for the full security model.

## Technology Stack

| Layer | Technology |
|-------|-----------|
| Core | Rust |
| Desktop | Tauri 2 (Rust + React) |
| Browser | WebExtension MV3 (React + WASM) |
| Mobile | Flutter (Dart + Rust FFI) |
| Web App | React (Rust HTTP API) |
| CLI | ratatui (Rust) |
| Sync | S3-compatible (aws-sdk-s3) |
| SSH | russh + russh-keys |
| Crypto | XChaCha20-Poly1305, Argon2id, Ed25519 |
| i18n | Fluent (Mozilla) |
