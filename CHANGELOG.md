# Changelog

## [Phase 1] - 2026-09-09

### Added

### Crypto
- **Post-quantum Hybrid**: Integrated Kyber/Dilithium with AES-GCM for forward secrecy.
- **Memory Encryption**: Implemented AES-GCM authenticated encryption for secure memory operations.
- **Duress Mode**: Added Shamir Secret Sharing for master key recovery.

### Vault
- **VaultItem Variants**: Defined all VaultItem variants (`Password`, `SshKey`, `SshConnection`, `ApiKey`, `Note`, `Card`, `Identity`).
- **Item Encryption/Decryption**: Implemented secure encryption/decryption for VaultItem variants.
- **Sled Backend**: Integrated with `sled` for encrypted local storage.

### Security
- **SecureString**: Implemented a secure string type that zeroizes on drop.
- **SecureVec**: Implemented a secure vector type that zeroizes on drop.
- **Constant-Time Comparison**: Added secure comparison utilities.

### Verification
- **Phase 1 Tests**: All tests pass successfully.
- **Clippy Warnings**: No warnings in Phase 1 modules.

### Fixed
- **Build Environment**: Resolved dependencies for `oqs-sys` compilation.

### Security Rules Followed
- Master password is NEVER stored or logged.
- Keys are NEVER written to disk unencrypted.
- All encryption uses authenticated encryption (AEAD).
- All random values use `rand::rngs::OsRng`.
- No secrets in git.
- All user input is validated and sanitized.
- Rate limiting on auth endpoints.
- Constant-time comparison for secrets.

## [Phase 2] - Planned
- Implement Phase 2 features as outlined in the project roadmap.