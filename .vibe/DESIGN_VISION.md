# Aetheris - Complete Design Vision & Architecture

## 🎯 Executive Vision
**Secrets Operating System** - Unified, zero-knowledge platform for all secrets management.

**Mission**: Eliminate the "re-setup" nightmare with automatic deployment, client-side zero-knowledge vault, and hardware-aware AI routing.

---

## 🏗️ Architecture Overview

### Core Philosophy: Three Pillars
1. **Zero-Knowledge by Design**: Server never sees plaintext
2. **Zero-Trust by Default**: Every request authenticated
3. **Zero-Compromise on Security**: Military-grade encryption

---

## 🔐 Security Architecture

### Defense in Depth Layers
```
Layer 7: Application (Rate Limiting, Input Validation, Session Mgmt)
Layer 6: Transport (TLS 1.3, Security Headers, CORS)
Layer 5: Data (End-to-End Encryption, CRDT Merge, Version Vectors)
Layer 4: Cryptographic (XChaCha20-Poly1305, Argon2id, Ed25519, X25519)
Layer 3: Memory (Zeroize on Drop, Secure Types, Constant-time Comparison)
Layer 2: Vault (Individual Item Encryption, HMAC, Sled Database)
Layer 1: Foundation (OsRng, No Plaintext, No Log Secrets, Dependency Audit)
```

### Current Implementation Status
```
✅ Security Module: FULLY IMPLEMENTED (508 lines)
   ├─ SecureString, SecureVec (Zeroize types)
   ├─ SecureCompare (Constant-time trait)
   ├─ RateLimiter (Token bucket)
   └─ InputValidator (Validation utilities)

🔄 Crypto Module: IN PROGRESS
   ├─ Argon2id KDF (t=3, m=64MB, p=4)
   ├─ XChaCha20-Poly1305 AEAD
   └─ HMAC-SHA256, Ed25519, X25519

🔄 Vault Module: IN PROGRESS
   ├─ All 7 VaultItem variants defined
   └─ Methods and encryption integration

⏳ Remaining Modules: 10/13 pending
```

---

## 🗃️ Data Model Summary

### Vault Item Types
- **PasswordItem**: username, password, urls, totp, notes, tags
- **SshKeyItem**: private_key, public_key, key_type, fingerprint
- **SshConnectionItem**: host, port, username, key_id
- **ApiKeyItem**: provider, key, scopes, expiry, rotation_policy
- **NoteItem**: title, body, tags
- **CardItem**: number, expiry, cvv, name, tags
- **IdentityItem**: name, email, phone, address, tags

---

## 🎨 UI/UX Design Language

### Design Principles
- **Security is Invisible**: Users never think about security
- **Zero Friction**: One-click setup, automatic configuration
- **Power When Needed**: Simple for beginners, powerful for experts
- **Consistency**: Same experience across all platforms
- **Trust Through Transparency**: Open source, audit friendly

### Color Scheme
- Primary: `#6366f1` (Indigo)
- Success: `#10B981` (Green)
- Warning: `#F59E0B` (Amber)
- Error: `#EF4444` (Red)
- Background: `#0F172A` (Dark Navy)

---

## 🚀 Implementation Priority

### Phase 1: Foundation (Week 1-2) - CURRENT
1. ✅ Security Module - **COMPLETED**
2. 🔄 Crypto Module - **IN PROGRESS**
3. 🔄 Vault Module - **IN PROGRESS**

### Phase 2: Core Features (Week 3-5)
1. Authentication (JWT, TOTP, WebAuthn, OAuth)
2. SSH Module (russh client, terminal, SFTP)
3. API Key Management (rotation, health checks)
4. Sync Engine (CRDT, offline queue, S3 backup)

### Phase 3: Advanced (Week 6-8)
1. Proactive Engine (auto-rotation, monitoring)
2. Browser Extension (WASM, WebSocket)
3. Web Server (Axum, GraphQL, REST)

### Phase 4: Platforms (Week 9-10)
1. Tauri Desktop App
2. Flutter Mobile Apps
3. Web App

---

## 💡 Unique Features Vision

### Proactive Engine
- **Auto-Rotate**: API keys rotated 7 days before expiry
- **Auto-Change**: Passwords changed immediately on breach detection
- **Auto-Monitor**: Provider health checked every 15 minutes
- **Auto-Sync**: Changes pushed to all devices automatically
- **Auto-Backup**: Daily encrypted backups to S3
- **Auto-Cleanup**: Remove unused/duplicate items
- **Auto-Alert**: Notifications for security events

### Plausible Deniability
- **Hidden Vault**: Accessible only with main password + duress password
- **Decoy Entries**: Fake items that appear real
- **Duress Mode**: Shows only decoy vault when forced
- **Shamir Secrets**: M-of-N sharing for hidden vault recovery

### Social Recovery
- **Trusted Contacts**: Designate contacts for account recovery
- **Recovery Shares**: Encrypted shares sent to contacts
- **M-of-N Required**: Any M trusted contacts can help recover
- **Time-Limited**: Shares expire after 7 days
- **Revocable**: User can revoke recovery access at any time

---

## 🔧 Technical Stack

### Per Platform
| Platform | Frontend | Backend | Target |
|----------|----------|---------|--------|
| Desktop | Tauri 2 + React | Rust Core | Windows, macOS, Linux |
| Browser | React + WASM | Rust WASM | Chrome, Firefox, Safari, Edge, Brave |
| Mobile | Flutter (Dart) | Rust FFI | iOS, Android |
| Web | React (TS) | Rust + Axum | All browsers |
| CLI | ratatui | Rust Binary | All platforms |

### Core Dependencies
- **Crypto**: chacha20poly1305, argon2, x25519-dalek, ed25519-dalek
- **Database**: sled (embedded), aws-sdk-s3 (cloud sync)
- **Network**: reqwest, tokio, websocket
- **UI**: ratatui, crossterm, arboard, keyring
- **Security**: zeroize, secrecy, hmac, hkdf

---

## 🎯 Current Milestone Status

### ✅ COMPLETED (Today)
- Project analysis and documentation complete
- Security module **FULLY implemented and compiling**
- 508 lines of secure code with comprehensive types
- Vault item types 70% complete
- Coordination infrastructure established

### 🔄 IN PROGRESS (Today)
- Crypto module: KDF and Cipher implementation
- Vault module: Methods and encryption integration
- Base functionality integration testing

### 🎯 GOAL FOR TODAY
**First working end-to-end test**:
```
1. ✅ Security types working
2. 🔄 Crypto encrypt/decrypt working
3. 🔄 Vault item encryption/decryption working
4. 🟡 Test: Create vault item → encrypt → store → retrieve → decrypt
```

---

## 📊 Success Metrics

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Security Module | 100% | 100% | ✅ DONE |
| Lines of Code | 8000+ | ~800+ | 🟡 ~10% |
| Modules Complete | 13 | 1 | 🔴 7.7% |
| Test Coverage | >80% | ~5% | 🔴 Needs Work |
| Compilation | ✅ Working | ✅ Working | ✅ OK |

---

**Document**: Comprehensive Design Vision  
**Version**: 1.0  
**Status**: 🟢 ACTIVE - Major Progress Today  
**Next**: Complete Phase 1 implementation and testing