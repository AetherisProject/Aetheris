<!-- hacklm-memory:start -->
## Memory-Augmented Context

Read memory files on-demand — not all at once.

| File | When to read |
|------|-------------|
| [.memory/instructions.md](.memory/instructions.md) | How to behave |
| [.memory/quirks.md](.memory/quirks.md) | When something breaks unexpectedly |
| [.memory/preferences.md](.memory/preferences.md) | Style/design/naming |
| [.memory/decisions.md](.memory/decisions.md) | Architectural changes |
| [.memory/security.md](.memory/security.md) | **ALWAYS — before any code change** |

### Memory Tools

Call `queryMemory` before answering anything about architecture, conventions, or style.

Call `storeMemory` (with a kebab-case `slug`) when:
1. User states a preference or rule → store as Instruction or Preference **before** acting
2. User corrects you → store the correction
3. A command or build fails → store root cause and fix
4. After completing any implementation task → store each architectural decision, convention, or pattern applied that is not already in memory. Do this **before ending the turn**.

Same slug = update, not duplicate.

### Writing Style for Memory Entries

Hemingway style. Short sentences. No jargon. No filler. Be blunt.
Bad: "The system employs an asynchronous locking mechanism to serialise concurrent write operations."
Good: "Use a lock before writing. One write at a time."

### Categories
| Category | Use for |
|----------|---------|
| Instruction | How to behave |
| Quirk | Project-specific weirdness |
| Preference | Style/design/naming |
| Decision | Architectural commitments |
| Security | Rules that must NEVER be broken |

<!-- hacklm-memory:end -->

---

# Aetheris Copilot Instructions

## Project Overview

Aetheris is a unified secrets management platform: SSH terminal, password vault, API key manager, and zero-knowledge sync system. It runs on desktop (Win/Mac/Linux), mobile (iOS/Android), browser (Chrome/Firefox/Safari/Edge/Brave), web, and CLI.

## Tech Stack

- **Core:** Rust (crypto, vault, SSH, sync, auth, proactive engine, web server, i18n, debug, design)
- **Desktop:** Tauri 2 (Rust + React/TypeScript)
- **Browser:** WebExtension MV3 (React + WASM)
- **Mobile:** Flutter (Dart + Rust FFI)
- **Web App:** React (shared components with desktop + extension)
- **CLI:** ratatui (Rust)
- **Sync:** S3-compatible (Backblaze B2, R2, MinIO)

## Code Style

- Follow `rustfmt` defaults (edition 2021)
- Clippy warnings are errors (`-D warnings`)
- No `unwrap()` in production code — use `?` or proper error handling
- Use `thiserror` for library errors, `anyhow` for application errors
- Prefer `tracing::` over `println!` or `eprintln!`
- All sensitive data must use `Zeroize` on drop
- All crypto operations must be constant-time where possible
- All user-facing strings must go through i18n (`fluent`)

## Architecture Rules

- The Rust core (`src/`) is the single source of truth
- No frontend-specific logic in the core
- Features are additive — don't break existing APIs
- Use feature flags for platform-specific code
- All vault items go through the `VaultItem` enum
- All sync operations go through the `SyncClient`
- All crypto operations go through the `CryptoEngine`

## Testing

- Every public function has a doc test
- Every module has unit tests (`#[cfg(test)]`)
- Integration tests in `tests/`
- Property-based tests with `proptest` for crypto
- Mock all external dependencies in tests
- No tests should require network access

## Security Rules (NEVER break)

- Master password is NEVER stored or logged
- Keys are NEVER written to disk unencrypted
- All encryption uses authenticated encryption (AEAD)
- All random values use `rand::rngs::OsRng`
- No secrets in git — `.gitleaks.toml` enforces this
- All user input is validated and sanitized
- Rate limiting on auth endpoints
- Constant-time comparison for secrets

## Documentation

- Every public API has rustdoc comments
- User-facing docs are in `docs/` as Markdown, built to HTML
- Changelog follows Keep a Changelog format
- Security policy in `SECURITY.md`

## Commit Style

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: feat, fix, docs, style, refactor, test, chore, perf, ci, security
Scopes: crypto, vault, ssh, apikey, sync, auth, proactive, web, i18n, debug, design, cli, desktop, mobile, browser

Example:
```
feat(vault): add Shamir Secret Sharing for master key recovery

Implements M-of-N Shamir Secret Sharing using the `shares` crate.
Threshold is configurable via `crypto.shamir_threshold`.

Closes #42
```
