# Preferences

Style, tone, and design choices for this project.

## Code Style

- Rust edition 2021
- `rustfmt` defaults, 100 char line limit for Rust
- 2-space indent for TOML, JSON, YAML, HTML, CSS, JS, TS
- 4-space indent for Rust, PowerShell
- Trailing commas in multi-line structures
- Use `thiserror` for library errors, `anyhow` for apps
- Prefer `&str` over `String` in function params
- Use `Cow<str>` for flexible ownership
- Prefer `match` over `if let` for enums with >2 variants

## Naming

- Modules: `snake_case` (`crypto_engine`, `vault_store`)
- Types: `PascalCase` (`VaultItem`, `CryptoEngine`)
- Functions: `snake_case` (`derive_key`, `encrypt_item`)
- Constants: `SCREAMING_SNAKE_CASE` (`ARGON2_TIME_COST`)
- Features: `kebab-case` (`memory-encryption`, `post-quantum`)
- CLI commands: `kebab-case` (`aeth vault list`, `aeth ssh connect`)

## Documentation

- Keep a Changelog format in `CHANGELOG.md`
- Markdown wrapped at ~80 columns
- Code blocks have language annotations
- Links are descriptive, not "click here"
- Every doc page has a clear title and purpose

## Design (UI/UX)

- Dark theme is default
- Indigo primary (#6366f1)
- JetBrains Mono for monospace
- Smooth animations (150-300ms)
- WCAG AAA accessibility
- Responsive: mobile, tablet, desktop
- 50+ languages from day one
- RTL support for Arabic, Hebrew, Persian, Urdu

## Commit Messages

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: feat, fix, docs, style, refactor, test, chore, perf, ci, security
Scopes: crypto, vault, ssh, apikey, sync, auth, proactive, web, i18n, debug, design, cli, desktop, mobile, browser

## Testing

- Unit tests in `#[cfg(test)]` modules
- Integration tests in `tests/`
- Property-based tests for crypto (`proptest`)
- Doc tests for public APIs
- No network in tests
- Mock all external dependencies
- Coverage target: >80%

## Security

- Argon2id: t=3, m=64MB, p=4
- XChaCha20-Poly1305 for AEAD
- Ed25519 for signing
- X25519 for key exchange
- OsRng for all random values
- Zeroize on drop for all secrets
- Constant-time comparison for secrets
- Rate limiting on auth endpoints
