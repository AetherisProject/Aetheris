# Contributing to Aetheris

Thank you for your interest in contributing! This document outlines the process.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_NAME/Aetheris.git`
3. Install Rust 1.70+: [rustup.rs](https://rustup.rs)
4. Build: `cargo build --features full`
5. Run tests: `cargo test --all-features`

## Development Setup

### Prerequisites

- Rust 1.70+ (stable or nightly)
- For desktop: Node.js 18+
- For mobile: Flutter SDK 3.0+
- For docs: Python 3.12+

### Build

```powershell
# PowerShell (recommended on Windows)
.\build.ps1 -Profile release -Clippy -Test -Fmt

# Or with cargo
cargo build --release --features full
```

### Code Style

- `cargo fmt` before committing
- `cargo clippy -- -D warnings` must pass
- No `unwrap()` in production code
- All public APIs documented
- Tests for new code

## Pull Request Process

1. Create a branch: `git checkout -b feat/my-feature`
2. Make your changes
3. Add tests
4. Run `.\build.ps1 -All`
5. Commit with conventional commits: `feat(scope): description`
6. Push and open a PR
7. Wait for CI to pass
8. Address review comments
9. Squash merge by maintainer

## Commit Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: feat, fix, docs, style, refactor, test, chore, perf, ci, security
Scopes: crypto, vault, ssh, apikey, sync, auth, proactive, web, i18n, debug, design, cli, desktop, mobile, browser

## Code of Conduct

Be respectful. Be constructive. Be helpful.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
