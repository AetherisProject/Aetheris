# Security

Rules that must NEVER be broken. Always read this file before any code change.

## Absolute Rules

### 1. Master Password Is Never Stored
The master password MUST NEVER be written to disk, logged, transmitted, or stored in memory longer than necessary. It exists only in CPU registers during key derivation, then is immediately zeroized.

### 2. Keys Never Touch Disk Unencrypted
Private SSH keys, API keys, and passwords MUST NEVER be written to disk in plaintext. They are always encrypted with the vault encryption key before any I/O.

### 3. All Encryption Is Authenticated
Only authenticated encryption (AEAD) is used. XChaCha20-Poly1305 is the standard. No unauthenticated modes (CBC, CTR without MAC).

### 4. Constant-Time Comparison
All secret comparisons (passwords, keys, tokens) MUST use constant-time comparison. Never use `==` for secrets.

### 5. Secure Random Only
All random values (nonces, salts, keys, IVs) MUST use `rand::rngs::OsRng` (or `getrandom`). Never use `rand::thread_rng()` for security-critical randomness.

### 6. Zeroize on Drop
All types holding secrets MUST implement `Zeroize` and `ZeroizeOnDrop`. This includes: master key, derived keys, plaintext passwords, private keys, API keys.

### 7. No Secrets in Logs
NEVER log sensitive data. Use `tracing::debug!` with redacted values. Use the `Secret<T>` wrapper type that redacts Debug output.

### 8. Rate Limiting
All authentication endpoints MUST have rate limiting. Exponential backoff after failures. Lockout after N failures.

### 9. Input Validation
All user input is validated and sanitized. Length limits, character set validation, format validation.

### 10. Dependency Security
All dependencies are audited (`cargo audit`). No known vulnerabilities. Dependabot keeps them updated.

## Implementation Rules

### Crypto Engine
- Argon2id: t=3, m=64MB, p=4 (minimum)
- XChaCha20-Poly1305: 256-bit key, 192-bit nonce
- Ed25519: for signatures (or CRYSTALS-Dilithium-5 in post-quantum mode)
- X25519: for key exchange (or CRYSTALS-Kyber-1024 in post-quantum mode)
- HKDF-SHA256: for key derivation from master key

### Vault Store
- All items encrypted individually with unique nonces
- Vault blob encrypted with sync encryption key
- HMAC-SHA256 for blob integrity verification
- CRDT merge with conflict detection

### Sync Client
- End-to-end encryption: server sees only encrypted blobs
- Blob signed with authentication key
- Version vector for conflict detection
- Offline queue encrypted at rest

### Auth System
- JWT with short expiry (24h default)
- Refresh tokens with rotation
- TOTP for MFA (RFC 6238)
- WebAuthn/FIDO2 for hardware key MFA
- Rate limiting: 5 attempts per 15 minutes per IP
- Account lockout after 10 failed attempts

### Web Server
- CORS: explicit origin list, no wildcards in production
- Rate limiting: 100 requests per minute per user
- HTTPS only (in production)
- Security headers: HSTS, CSP, X-Frame-Options, X-Content-Type-Options
- No sensitive data in URLs (use POST body)

### Browser Extension
- WASM crypto: same Rust code, compiled to WebAssembly
- Content scripts: minimal permissions, no eval
- Storage: only encrypted data in chrome.storage
- Messaging: validated origin, authenticated

### Mobile
- Biometric: platform-native (Face ID, Touch ID, Windows Hello, Android Biometric)
- Keychain: iOS Keychain, Android Keystore, Windows Credential Manager
- No secrets in app logs
- Jailbreak/root detection (optional)

## Incident Response

If a security issue is discovered:
1. Stop work immediately
2. Document the issue in `.memory/security.md`
3. Fix the issue before any other work
4. Update SECURITY.md with disclosure process
5. Notify affected users if data may be compromised
