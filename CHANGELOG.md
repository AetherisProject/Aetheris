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

### Changed
- Fixed `src/*/mod.rs` module stubs with proper structure and content
- Created submodule stub files for all planned module hierarchy
- Fixed `lib.rs` re-exports to match actual module layout
- Added `clap_complete` feature to Cargo.toml for shell completion
- Fixed `.gitignore` to not exclude `.gitleaks.toml`
- Updated test files to reference actual module items
- Added bash shell wrapper (`aetheris.sh`)
- Fixed AGENTS.md YAML frontmatter syntax

### Fixed
- Removed self-referencing `pub mod X;` lines from module stubs
- Removed literal `\n` escape sequences from all module files
- Fixed `Cargo.lock` missing warning by adding `clap` complete feature
- Removed empty `cargo_launcher.log` from repo

### Added
- Per-platform CI workflows (`web.yml`, `desktop.yml`, `browser.yml`, `mobile.yml`)
- Multi-platform `.gitignore` (`**/node_modules/`, `**/dist/`, build outputs)
- Build scripts: `desktop/test_build.sh`, `browser/build.sh`, `web/test.sh`, `mobile/build.sh`
- `desktop/index.html`, `desktop/src/main.tsx`, `mobile/lib/main.dart`
- `web/vite.config.ts`; tracked `web/package-lock.json`
- `.github/workflows/changelog-update.yml` (auto-updatable CI log)

### Fixed
- Untracked build artifacts removed; `.gitignore` covers all subprojects
- `Desktop CI`: added entry (`index.html` + `main.tsx`) — was missing
- `Web CI`: valid `vite.config.ts` — was stub
- `Mobile CI`: `main.dart` + fixed `flutter` command syntax
- `Browser CI`: `zip` install; bundle script executable
- All 4 new platform CI runs green on `f7ba81c`
- Remote switched to SSH; `workflow` token scope noted
