# Final Aetheris Decisions

## Core Library
- **Dependencies**: Minimal dependencies for core functionality.
- **VaultItem**: Serialization/deserialization with UUID generation.
- **CryptoEngine**: Kyber keypair generation + hybrid encryption/decryption.
- **SshClient**: SSH client with channel support.
- **SyncClient**: Thread-safe sync state management.

## Platforms
- **Desktop**: Tauri backend with `add_vault_item`, `generate_keypair`, `connect_ssh`.
- **CLI**: Subcommands for vault, SSH, and crypto operations.
- **Web**: WASM bindings for vault and crypto.
- **Browser Extension**: Functional key generation.

## CI/CD
- **Unified Workflow**: `release.yml` triggers builds on `v*` tags.
- **Tagging**: `v0.0.1` pushed and workflow verified.

## Testing
- **Core Modules**: All compile successfully without regressions.
- **Platforms**: Ready for local testing.

## Preferences
- **Rust-only WASM**: Excluded GTK dependencies from `platforms/web`.
- **Modular Design**: Core library abstracts platform-specific logic.
- **Automated Releases**: Unified workflow for cross-platform artifacts.