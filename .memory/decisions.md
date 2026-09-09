# Aetheris CI/CD & Release Decisions

## Core Library
- **Dependencies**: Removed duplicate `anyhow` and excluded GTK dependencies from `platforms/web`.
- **Structure**: `core/src/` modules are modular and re-export key types for platform integration.

## Platforms
- **Desktop**: Tauri for cross-platform compatibility.
- **CLI**: `clap` for command-line subcommands.
- **Web**: WASM bindings for lightweight crypto/vault operations.
- **Browser Extension**: Manifest v3 for Chrome/Firefox compatibility.

## CI/CD
- **Unified Workflow**: `release.yml` triggers builds on `v*` tags and uploads artifacts to GitHub Releases.
- **Tagging**: `v0.0.1` pushed and workflow verified.

## Verification
- **Workspace Check**: Core Rust modules compile successfully (excluding GTK dependencies).
- **GitHub Actions**: Workflows are ready for artifact uploads.

## Next Steps
1. Push `v0.0.1` to GitHub.
2. Test all platforms locally.
3. Deploy verified artifacts to GitHub Releases.

## Preferences
- **Rust-only WASM**: Excluded GTK dependencies from `platforms/web` to avoid build errors.
- **Modular Design**: Core library abstracts platform-specific logic.
- **Automated Releases**: Unified workflow for cross-platform artifacts.