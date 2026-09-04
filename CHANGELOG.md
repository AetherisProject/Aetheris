# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with Rust core (VaultEngine)
- CLI with `aeth` binary (vault, ssh, api-key, sync commands)
- Crypto module placeholder (Argon2id, XChaCha20-Poly1305)
- Vault module placeholder with unified VaultItem enum
- SSH client module placeholder (russh)
- API key engine module placeholder (30+ providers)
- Sync client module placeholder (S3-compatible, CRDT)
- Auth system module placeholder (JWT, MFA, OAuth)
- Proactive engine module placeholder (auto-rotate, auto-change, auto-monitor)
- Web server module placeholder (HTTP API, WebSocket)
- Browser relay module placeholder (local WebSocket server)
- Admin engine module placeholder (users, teams, audit)
- i18n engine module placeholder (Fluent, 50+ languages)
- Debug system module placeholder (tracing, metrics, diagnostics)
- Design system module placeholder (tokens, themes, components)
- Browser extension scaffold (Chrome, Firefox, Safari, Edge, Brave)
- Mobile app scaffold (Flutter)
- Web app scaffold (React)
- Documentation system with HTML generation from Markdown
- CI/CD workflows (CI, Release, Docs)
- Dependabot configuration for cargo, GitHub Actions, pip, npm
- Copilot instructions with memory-augmented context
- Memory files (decisions, instructions, preferences, quirks, security)
- Gitleaks configuration for secret scanning
- Editor configuration
- Environment configuration (.env.example)
- Runtime configuration (aetheris.toml)
- Build script (build.ps1)
- Shell integrations (cmd, PowerShell, bash)
- Skills documentation for AI agents
