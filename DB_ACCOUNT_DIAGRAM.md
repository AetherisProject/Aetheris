## Aetheris Account System — Relation Diagram

```
App (CLI/Desktop/Mobile/Web/Browser)
  │
  ▼
AuthManager (src/auth/mod.rs)
  ├── AccountStore (HashMap in-memory → needs sled DB)
  │      └── UserAccount (id, email, password_hash[Argon2id], last_login, mfa_enabled)
  ├── SessionStore (HashMap in-memory → needs DB + expiration enforce)
  │      └── SessionToken (token, user_id, created, expires) [Drop=Zeroize]
  ├── CryptoEngine (AEAD + Argon2id + constant-time compare)
  └── Recovery/MFA/TOTP (stubbed — not enforced)
          │
          ▼
VaultStore (sled DB at path) — encrypted blobs per item.id
  │
  ▼
VaultItem (PasswordItem/SSHKey/APIKey etc.) — encrypted via AEAD
  │
  ▼
SyncClient (S3-compatible: Backblaze B2, R2, MinIO)
  │
  ▼
CryptoEngine (key derivation from master password)
```

Key flows:
- Register: AuthManager.register → AccountStore → Argon2id hash → session token → VaultStore init
- Login: AuthManager.login → verify_hash(constant-time) → SessionStore.create → token returned → VaultStore access
- Change password: verify old → new Argon2id hash → session invalidated (manual)
- Reset: Admin/init only (stubbed recovery.rs) — needs secure token-based flow
