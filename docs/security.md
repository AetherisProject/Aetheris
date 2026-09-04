# Security Model

Aetheris uses a zero-knowledge architecture: the server never sees your plaintext data or master password. All encryption and decryption happens client-side.

## Security Layers

### Layer 1: Something You Know
- **Master password** — Argon2id-derived key (never stored, never transmitted)
- **PIN** — Quick unlock, derived from master key

### Layer 2: Something You Have
- **Hardware key** — YubiKey 5, FIDO2, WebAuthn
- **Trusted device** — Device-specific certificate
- **Recovery codes** — Generated at setup, stored offline

### Layer 3: Something You Are
- **Biometric** — Face ID, Touch ID, Windows Hello, Android Biometric
- **Behavioral** — Typing pattern (optional, privacy-safe)

### Layer 4: Someone You Trust
- **Social recovery** — N-of-M trusted contacts must approve recovery
- **Organization recovery** — Admin recovery for enterprise teams

### Layer 5: Time
- **Time-locked recovery** — Configurable delay before recovery completes
- **Rate limiting** — Exponential backoff, anti-brute force

### Layer 6: Plausible Deniability
- **Hidden volumes** — Hidden vault within your vault
- **Duress password** — Silently unlocks decoy vault
- **Decoy entries** — Fake entries that look real

## Key Derivation

```
Master Password + Hardware Challenge
        │
        ▼
    Argon2id (t=3, m=64MB, p=4)
        │
        ▼
    Master Key (32 bytes, never leaves secure memory)
        │
        ├──► Vault Key (HKDF-SHA256, context="vault")
        ├──► Sync Key (HKDF-SHA256, context="sync")
        ├──► Auth Key (HKDF-SHA256, context="auth")
        ├──► Backup Key (HKDF-SHA256, context="backup")
        └──► Recovery Key (HKDF-SHA256, context="recovery")
```

## Encryption

| Algorithm | Use | Key Size |
|-----------|-----|----------|
| XChaCha20-Poly1305 | AEAD encryption | 256-bit |
| Argon2id | Key derivation | — |
| Ed25519 | Signing | 256-bit |
| X25519 | Key exchange | 256-bit |
| HKDF-SHA256 | Key derivation | 256-bit |
| HMAC-SHA256 | Integrity | 256-bit |
| CRYSTALS-Kyber-1024 | Post-quantum KEM | 1024-byte |
| CRYSTALS-Dilithium-5 | Post-quantum signing | — |

## Shamir Secret Sharing

Split the master key into N shares (threshold M). Any M shares can reconstruct the master key. Fewer than M shares reveal nothing.

Example: 5 shares, threshold 3. Distribute to spouse, parent, best friend, lawyer, safe deposit box. Any 3 can recover.

## Plausible Deniability

```
Outer Vault (decoy)
├── Normal-looking passwords
├── Normal-looking API keys
└── Hidden Volume (indistinguishable from random padding)
    ├── Real passwords
    ├── Real API keys
    └── Real SSH keys

Duress Password → Unlocks Outer Vault only
Real Password → Unlocks both
```

## Multi-Device Consensus

Sensitive operations (master password change, vault export, recovery) require approval from N trusted devices. This prevents a single compromised device from being catastrophic.

## Security Comparison

| Feature | 1Password | Bitwarden | Aetheris |
|---------|:---------:|:---------:|:--------:|
| Zero-knowledge | ✅ | ✅ | ✅ |
| Hardware key | ✅ | ✅ | ✅ |
| Biometric | ✅ | ✅ | ✅ |
| TOTP built-in | ✅ | ✅ | ✅ |
| Breach monitoring | ✅ | ✅ | ✅ |
| Auto-password change | ❌ | ❌ | ✅ |
| Shamir sharing | ❌ | ❌ | ✅ |
| Plausible deniability | ❌ | ❌ | ✅ |
| Duress mode | ❌ | ❌ | ✅ |
| Post-quantum | ❌ | ❌ | ✅ |
| Memory encryption | ❌ | ❌ | ✅ |
| Multi-device consensus | ❌ | ❌ | ✅ |
| Social recovery | ❌ | ❌ | ✅ |

## Reporting Vulnerabilities

See [SECURITY.md](../SECURITY.md) for our vulnerability disclosure policy.
