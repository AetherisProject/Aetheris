# Aetheris Project Preferences

## Code Style
- **Rust**: Use `clippy` warnings as errors (`-D warnings`).
- **Formatting**: Follow `rustfmt` defaults (edition 2021).
- **Error Handling**: Prefer `thiserror` for library errors, `anyhow` for application errors.

## Architecture
- **Core Library**: Single source of truth for vault, crypto, SSH, and sync.
- **Platforms**: Isolated entry points (Tauri, CLI, WASM, Browser Extension).
- **CI/CD**: Unified workflow for artifact uploads on `v*` tags.

## Dependencies
- **Core**: `oqs`, `zeroize`, `serde`, `tokio`, `tracing`, `thiserror`.
- **Platforms**: Exclude GTK dependencies from `platforms/web` to avoid build errors.

## Testing
- **Unit Tests**: Every public function has a doc test.
- **Integration Tests**: `tests/` directory for cross-module validation.
- **Property-Based**: Use `proptest` for crypto validation.

## Security
- **Master Password**: NEVER stored or logged.
- **Keys**: NEVER written to disk unencrypted.
- **Encryption**: Use authenticated encryption (AEAD).
- **Randomness**: `rand::rngs::OsRng` for cryptographic operations.

## Documentation
- **Rustdoc**: Every public API has comments.
- **User Docs**: `docs/` for Markdown, built to HTML.
- **Changelog**: Follow [Keep a Changelog](https://keepachangelog.com/).

## Release Workflow
- **Trigger**: On `v*` tags.
- **Artifacts**: Upload to GitHub Releases for all platforms.
- **Platforms**: Desktop, CLI, Web, Browser Extension.