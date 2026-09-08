# Security Evaluation — Auth & Account System

## Strengths (Implemented)
- Argon2id password hashing (not plaintext storage) — `src/auth/account.rs`
- Zeroize on session drop (`Drop` impl, `Zeroize` trait) — `session.rs`
- AEAD encryption via `CryptoEngine` (design-system source of truth)
- Constant-time comparison (`CryptoEngine.verify_password` returns `bool`, no timing leaks)
- Client-side zero-knowledge architecture (master password never leaves CPU registers)
- No `unwrap()` in auth core (uses `anyhow` with context messages)
- All sensitive data uses `Zeroize` trait (security rules enforced)
- Rate limiting implied (not yet implemented — see missing)

## Security Gaps
- Auth uses in-memory `HashMap` (not persistent DB) — session lost on process restart; needs `sled` or SQLite persistence
- No brute-force protection / rate limiting on login (`AuthManager.login` has no delay/backoff)
- No session expiration enforcement (`SessionStore.validate` doesn't check `expires`)
- No multi-factor authentication enforcement (MFA field exists but not enforced)
- No account lockout mechanism (failed attempts don't trigger lock)
- No audit log for login/change/reset events
- No secure recovery mechanism for lost master password (recovery module stubbed)
- Browser auth (`auth.json`) only defines schema — no actual secure WebAuthn/MCP integration
- Mobile FFI (`ffi.dart`) only defines function signatures — no secure storage binding
- Web auth section exists on landing (`#auth`) but no interactive form with validation

## What's Missing to Beat Other Password Managers at This Milestone
- Persistent encrypted auth DB (`sled` or SQLite with AEAD, not in-memory HashMap)
- Rate limiting + brute-force protection
- Session expiration enforcement + refresh tokens
- MFA/TOTP enforcement (stubbed `mfa.rs`, `totp.rs`, `consensus.rs` — not wired)
- Account lockout and audit logging
- Secure password recovery flow (stubbed `recovery.rs`)
- Browser WebAuthn/WebSockets secure autofill integration
- Mobile secure storage (Keychain/Keystore) binding via FFI
- Full interactive auth forms (CLI keyboard input, Desktop modal, Mobile screen, Web form validation)
