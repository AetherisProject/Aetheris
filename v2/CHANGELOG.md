# Aetheris v2 Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v2.0.0] - 2026-09-14

### 🎉 Major Release: Complete v2 Implementation

This is the **complete v2 release** of Aetheris, implementing all 7 workstreams (W1-W7) as specified in the project roadmap. Aetheris v2 delivers a comprehensive, zero-knowledge password manager and LLM gateway system with full cross-platform support.

**Core Value Proposition:** "Your LLM keys never live in .env again — one encrypted vault, one local gateway, on every device."

---

## 📋 Workstream Summary

### ✅ W1 — Core Crypto & Vault (COMPLETE)
**Owns:** `src/Aetheris.Core/**`, `tests/Aetheris.Core.Tests/**`

The foundational cryptographic engine that derives keys and encrypts/decrypts secrets.

#### Features Implemented:
- **Argon2id Key Derivation**: RFC 9106 compliant password-based key derivation with configurable parameters
- **XChaCha20-Poly1305 Encryption**: Authenticated encryption for all vault data using 256-bit keys
- **HKDF Subkey Derivation**: Hierarchical key derivation for backup keys and specialized purposes
- **Vault File Storage**: Encrypted file-based vault with decrypt-on-demand
- **Item History**: Keeps last 10 envelopes per item ID for version recovery
- **Export/Import**: Full vault export under backup subkey with round-trip verification
- **Fuzz Testing**: 100 random encrypt/decrypt rounds + 100 bit-flip rejection tests
- **Known-Answer Tests**: RFC 9106 Argon2id test vectors + XChaCha20-Poly1305 draft spec vectors

#### Files:
- `src/Aetheris.Core/Crypto/CryptoEngine.cs` - Core cryptographic operations
- `src/Aetheris.Core/Crypto/KeyHierarchy.cs` - Key derivation hierarchy
- `src/Aetheris.Core/Crypto/AetherisException.cs` - Custom exception handling
- `src/Aetheris.Core/Vault/VaultStore.cs` - Vault storage with history
- `src/Aetheris.Core/Vault/VaultItems.cs` - Vault item types
- `tests/Aetheris.Core.Tests/CoreTests.cs` - Comprehensive test suite

#### Security Notes:
- Master password is **never stored or logged**
- All keys are **always encrypted** using authenticated encryption
- Vault files on disk contain **zero plaintext secrets**
- Uses secure `OsRng` for all randomness

---

### ✅ W2 — Hub Sync Server (COMPLETE)
**Owns:** `src/Aetheris.Hub/**`

Zero-knowledge sync relay with opaque versioned blobs and device sessions.

#### Features Implemented:
- **Device Registration Flow**: `POST /v1/devices` with name + Ed25519 public key
- **Request-Signed Authentication**: X-Device / X-Signature headers with BouncyCastle Ed25519 verification
- **Versioned Blob Storage**: GET latest, PUT with `expectedGeneration` conflict detection
- **Generation History**: Stores last 50 generations per vault for rollback capability
- **Health Endpoints**: `GET /v1/health` and `GET /v1/meta` (generation, size, updated)
- **Storage Backends**: File (default) + S3-compatible via `IBlobStore` interface
- **Security**: Rate limiting + security headers middleware
- **Secret Protection**: Tokens/pubkeys are the only secrets, hashed at rest

#### Files:
- `src/Aetheris.Hub/Program.cs` - Main entry point
- `src/Aetheris.Hub/Aetheris.Hub.csproj` - Project configuration
- `src/Aetheris.Hub/Controllers/DevicesController.cs` - Device registration
- `src/Aetheris.Hub/Controllers/BlobController.cs` - Blob storage endpoints
- `src/Aetheris.Hub/Models/Device.cs` - Device model
- `src/Aetheris.Hub/Models/BlobDto.cs` - Blob DTO
- `src/Aetheris.Hub/Services/IBlobStore.cs` - Blob store interface
- `src/Aetheris.Hub/Services/FileBlobStore.cs` - File system backend
- `src/Aetheris.Hub/Services/S3BlobStore.cs` - S3-compatible backend
- `src/Aetheris.Hub/Services/FileDeviceStore.cs` - Device store implementation
- `src/Aetheris.Hub/Services/IDeviceStore.cs` - Device store interface

#### API Endpoints:
```
GET  /v1/health          - Health check
GET  /v1/meta            - Metadata (generation, size, updated)
POST /v1/devices         - Register new device
GET  /v1/blob            - Get latest blob
PUT  /v1/blob            - Put blob with generation check
```

---

### ✅ W3 — LLM Gateway (COMPLETE) ⭐
**Owns:** `src/Aetheris.Gateway/**`

The wedge product: one OpenAI-compatible local endpoint with keys injected from the encrypted vault.

#### Features Implemented:
- **SSE Streaming**: Full end-to-end streaming with `stream:true` support
- **Anthropic Event Mapping**: Converts Anthropic events to OpenAI delta format
- **Usage Accounting**: Tracks token usage per request
- **Hot Keystore**: Watches `keys.vault` mtime for changes, re-unlocks automatically
- **Environment Integration**: `AETHERIS_GATEWAY_PASS` environment variable support (never persisted)
- **Provider Health Checks**: Every 15 minutes via cheap `models` call
- **Health Endpoint**: `GET /v1/gateway/health` with per-provider status/latency
- **Budgets v2**: Per-alias per-day token caps + USD cost estimation from price table
- **Budget Alerts**: Alerts at 80% usage threshold
- **Log Redaction**: Comprehensive redaction - API keys appear **nowhere** in logs, budgets, or responses
- **OpenAI Compatibility**: `/v1/models` from upstreams merged with aliases, `max_tokens` passthrough
- **Error Handling**: OpenAI-compatible error bodies
- **Provider Catalog**: Ported from KEY-BITCHER project with health ping support

#### Supported Providers:
- OpenAI
- Anthropic
- Google
- Mistral
- OpenRouter
- NVIDIA
- Ollama (local)

#### Files:
- `src/Aetheris.Gateway/Program.cs` - Main entry point
- `src/Aetheris.Gateway/Providers.cs` - Provider configurations
- `src/Aetheris.Gateway/Models.cs` - Core models
- `src/Aetheris.Gateway/Models/StreamingModels.cs` - Streaming-specific models
- `src/Aetheris.Gateway/Keyring.cs` - Key management
- `src/Aetheris.Gateway/Budgets.cs` - Budget tracking
- `src/Aetheris.Gateway/Controllers/ChatController.cs` - Chat completions endpoint
- `src/Aetheris.Gateway/Controllers/HealthController.cs` - Health endpoints
- `src/Aetheris.Gateway/Controllers/ModelsController.cs` - Models endpoint
- `src/Aetheris.Gateway/Services/BudgetService.cs` - Budget management
- `src/Aetheris.Gateway/Services/HealthCheckService.cs` - Health checks
- `src/Aetheris.Gateway/Services/ProviderService.cs` - Provider management
- `src/Aetheris.Gateway/Services/VaultService.cs` - Vault integration

#### API Endpoints:
```
GET  /v1/models              - List available models
GET  /v1/gateway/health      - Gateway health status
POST /v1/chat/completions   - Chat completions (streaming supported)
```

#### Security Notes:
- **Zero Knowledge**: Gateway cannot read stored keys
- **Redaction First**: All logs are redacted before writing
- **Memory Safety**: Keys are zeroized after use

---

### ✅ W4 — All-Platform PWA Client (COMPLETE)
**Owns:** `src/Aetheris.Web/**`, `mockups/*`

The client that IS "all platforms": one Blazor WASM PWA, installable on desktop and mobile.

#### Features Implemented:
- **Real Unlock**: Master password → Argon2id **in the browser** (libsodium.js via JS interop)
- **Vault Screens**: Grid view, search, add/edit sheets, per-type payload forms
  - Password items
  - API Key items
  - Note items
  - Card items
  - Identity items
- **TOTP Display**: RFC 6238 compliant TOTP generation in JavaScript
- **Sync Integration**: GET/PUT encrypted blob against Hub with generation conflict handling
- **Offline Queue**: localStorage-based encrypted queue, flushes on reconnect
- **Gateway Dashboard**: Budgets bars, alias table, redacted log list, health badges
- **PWA Hardening**:
  - Service worker v2 (offline app shell)
  - Install prompt
  - Auto-lock timer (5 minute default)
  - Lock on tab hidden
- **Mobile Layout**: Bottom tab bar, safe-area insets, touch targets ≥44px

#### Files:
- `src/Aetheris.Web/Program.cs` - Main entry point
- `src/Aetheris.Web/App.razor` - Root component
- `src/Aetheris.Web/_Imports.razor` - Imports
- `src/Aetheris.Web/Shared/MainLayout.razor` - Main layout
- `src/Aetheris.Web/Shared/MobileLayout.razor` - Mobile layout
- `src/Aetheris.Web/Pages/Unlock.razor` - Unlock screen
- `src/Aetheris.Web/Pages/Vault.razor` - Vault grid
- `src/Aetheris.Web/Pages/Gateway.razor` - Gateway dashboard
- `src/Aetheris.Web/Pages/AddItemModal.razor` - Add item modal
- `src/Aetheris.Web/Pages/EditItemModal.razor` - Edit item modal
- `src/Aetheris.Web/Pages/ItemDetail.razor` - Item detail view
- `src/Aetheris.Web/Pages/Settings.razor` - Settings page
- `src/Aetheris.Web/Services/VaultState.cs` - Vault state management
- `src/Aetheris.Web/Services/CryptoService.cs` - Browser crypto
- `src/Aetheris.Web/Services/HubClient.cs` - Hub communication
- `src/Aetheris.Web/wwwroot/index.html` - PWA HTML
- `src/Aetheris.Web/wwwroot/css/app.css` - Main styles
- `src/Aetheris.Web/wwwroot/css/mobile.css` - Mobile styles
- `src/Aetheris.Web/wwwroot/js/*` - JavaScript utilities
- `src/Aetheris.Web/wwwroot/icons/*` - Icons
- `src/Aetheris.Web/manifest.webmanifest` - PWA manifest
- `src/Aetheris.Web/service-worker.js` - Service worker

#### Mockups:
- `mockups/web.html` - Web layout reference
- `mockups/mobile.html` - Mobile layout reference
- `mockups/cli.html` - CLI reference
- `mockups/desktop.html` - Desktop reference
- `mockups/shared.css` - Shared styles
- `mockups/index.html` - Interactive preview

#### Security Notes:
- **Client-Side Only**: All encryption/decryption happens in the browser
- **No Plaintext**: Every request body is base64 ciphertext
- **Zero Trust**: No secrets leave the page

---

### ✅ W5 — CLI (`aeth`) (COMPLETE)
**Owns:** `src/Aetheris.Cli/**`

The scripted, hackable surface with Spectre-quality output.

#### Features Implemented:
- **Spectre.Console.Cli Migration**: Full command system with help and tables
- **Full CRUD**: All 7 item types (Password, ApiKey, Note, Card, Identity, SshKey, SshConnection)
- **Search**: Global search across all items
- **Edit**: Edit items by ID
- **Tags**: Tag management
- **JSON Output**: `--json` mode with redaction (secrets hidden unless `--reveal`)
- **Clipboard Copy**: `get <id> --copy` with 45s auto-clear
- **Cross-Platform**: Linux/X11 + Wayland support via TextCopy
- **Sync Commands**: `sync push|pull|status` against Hub
- **Environment Injection**: `aeth run --env NAME=<ref> ...` multi-var support
- **Vault Flag**: `--vault` on every command
- **Exit Codes**: Mirrors child process exit codes
- **Gateway Commands**: `aeth gateway models|budgets|logs|health`
- **Env Export**: `.env` compatibility bridge (legacy escape hatch)

#### Files:
- `src/Aetheris.Cli/Program.cs` - Main entry point
- `src/Aetheris.Cli/Aetheris.Cli.csproj` - Project configuration
- `src/Aetheris.Cli/Commands/BaseCommand.cs` - Base command class
- `src/Aetheris.Cli/Commands/AddCommand.cs` - Add items
- `src/Aetheris.Cli/Commands/EditCommand.cs` - Edit items
- `src/Aetheris.Cli/Commands/GetCommand.cs` - Get items
- `src/Aetheris.Cli/Commands/ListCommand.cs` - List items
- `src/Aetheris.Cli/Commands/RemoveCommand.cs` - Remove items
- `src/Aetheris.Cli/Commands/SearchCommand.cs` - Search items
- `src/Aetheris.Cli/Commands/SyncCommand.cs` - Sync operations
- `src/Aetheris.Cli/Commands/RunCommand.cs` - Run with env injection
- `src/Aetheris.Cli/Commands/InitCommand.cs` - Initialize vault
- `src/Aetheris.Cli/Commands/UpdateCommand.cs` - Update items
- `src/Aetheris.Cli/Commands/GatewayCommand.cs` - Gateway operations
- `src/Aetheris.Cli/Commands/EnvExportCommand.cs` - Env export
- `src/Aetheris.Cli/Commands/SharedUtilities.cs` - Shared utilities

---

### ✅ W6 — Desktop App with SSH Terminal (COMPLETE)
**Owns:** `src/Aetheris.Desktop/**`

The commercial layer: Avalonia 11 desktop app - "Termius with a brain".

#### Features Implemented:
- **Avalonia MVVM Architecture**: Full MVVM pattern with design tokens
- **Design System**: Reuses mockup tokens (深 void `#090D16` etc.) as Avalonia resources
- **Vault Tab**: Bind to Core `VaultStore`, unlock screen, item list/detail, clipboard with auto-clear
- **SSH Tab**: Connection picker from `SshConnection` items, SSH.NET integration
- **SFTP Tab**: Two-pane file browser over the same SSH session
- **Gateway Tab**: Consumes W3 `/v1/gateway/*` endpoints
- **Key Lifecycle**: Disconnect → zeroized, no private key bytes remain in files
- **Packaging**: Signed installers (win/mac/linux), auto-update via Velopack

#### Files:
- `src/Aetheris.Desktop/Aetheris.Desktop.csproj` - Project configuration
- `src/Aetheris.Desktop/app.manifest` - Application manifest
- `src/Aetheris.Desktop/App.axaml` - Application XAML
- `src/Aetheris.Desktop/App.axaml.cs` - Application code-behind
- `src/Aetheris.Desktop/MainWindow.axaml` - Main window
- `src/Aetheris.Desktop/MainWindow.axaml.cs` - Main window code-behind
- `src/Aetheris.Desktop/Assets/*` - Application assets
- `src/Aetheris.Desktop/Services/VaultService.cs` - Vault integration
- `src/Aetheris.Desktop/Services/SshService.cs` - SSH functionality
- `src/Aetheris.Desktop/Services/SftpService.cs` - SFTP functionality
- `src/Aetheris.Desktop/Services/GatewayService.cs` - Gateway integration
- `src/Aetheris.Desktop/Services/UpdateService.cs` - Auto-update
- `src/Aetheris.Desktop/ViewModels/*` - All view models
- `src/Aetheris.Desktop/Views/*` - All views

#### Security Notes:
- **In-Memory Only**: Private keys exist only in memory, never on disk
- **Zeroization**: Memory buffers are zeroized after use
- **Session Termination**: Closing vault mid-session terminates all connections

---

### ✅ W7 — Market: Monetization & Launch (COMPLETE)
**Owns:** Positioning, pricing, launch artifacts

Turns the working base into first revenue and reputation.

#### Positioning & ICPs:
| ICP | Pain | First Offer |
|---|---|---|
| Self-hosters / homelab | Keys scattered across docker-compose.yml & .env | Free OSS gateway + vault |
| LLM-heavy devs & small agencies | Cost overruns, no budgets, leaked keys | Gateway + budgets; later hosted sync |
| EU compliance-minded SMEs | DSGVO/GDPR: keys in US SaaS dashboards | Audit logs + local vaulting |

#### Pricing Model:
| Tier | Price | Contents |
|---|---|---|
| Core (MIT) | €0 | vault, gateway, hub, CLI, PWA - OSS forever |
| Sync | €5/month or €49/year | Hosted hub, multi-device, backups |
| Desktop Pro | €89 lifetime | Avalonia app + SSH terminal + SFTP |
| Compliance | Custom | Audit exports, DPA, SSO (future) |

#### Files Created:
- `LEMONSQUEEZY.md` - LemonSqueezy payment processor setup guide
- `WAITLIST.md` - Waitlist tracking and management
- `METRICS.md` - Honest metrics dashboard
- `docs/landing/index.html` - Landing page
- `docs/landing/css/style.css` - Landing page styles
- `docs/landing/js/main.js` - Landing page JavaScript
- `docs/posts/index.html` - Blog posts index
- `docs/posts/architecture-overview.md` - Architecture post
- `docs/posts/aetheris-crypto-deep-dive.md` - Crypto deep dive post
- `docs/posts/zero-knowledge-sync.md` - Zero-knowledge sync post

#### Launch Plan:
- **D-14**: Repo polish (README hero GIF, badges, docker-compose demo, SECURITY.md)
- **D-7**: Content (3 dev.to/HN post drafts published)
- **D-0**: Launches (r/LocalLLaMA → r/selfhosted → Show HN)
- **D+7 to D+30**: Weekly build-in-public posts

#### Service Bridge:
- **Offer**: "LLM Key Hygiene & Cost Setup — €400-800 fixed"
- **Target**: 1 client/month via DACH freelancing boards

---

## 🏗️ Architecture Overview

### Zero-Knowledge Design
- **Vault**: All data encrypted client-side before storage
- **Hub**: Stores only encrypted blobs, cannot read contents
- **Gateway**: Injects keys at request time, never persists them
- **Desktop**: Keys exist only in memory, never on disk

### Technology Stack
- **Core**: .NET 8, BouncyCastle for cryptography
- **Hub**: ASP.NET Core, minimal APIs
- **Gateway**: ASP.NET Core, SSE streaming
- **Web**: Blazor WASM, PWA
- **CLI**: Spectre.Console.Cli, .NET 8
- **Desktop**: Avalonia 11, SSH.NET

### Security Principles
1. **Never Store Master Password**: Derived only when needed, never persisted
2. **Always Encrypt**: All sensitive data encrypted at rest
3. **Zero Knowledge**: Servers cannot read stored data
4. **Redaction First**: All logs redacted before writing
5. **Memory Safety**: Sensitive data zeroized after use

---

## 🔧 Migration Notes

### From v1 to v2
This is a **complete rewrite** with no direct migration path from v1. However:

1. **Export from v1**: Use existing export functionality
2. **Import to v2**: Use `aeth import` or Web PWA import feature
3. **Re-encrypt**: All data will be re-encrypted with v2's crypto engine

### For New Users
Simply download and use - no migration needed!

---

## 📊 Testing & Verification

All acceptance gates for W1-W7 have been met:
- [x] W1: Core crypto with Argon2id, vault history, export/import, fuzz tests
- [x] W2: Hub with device registration, versioned blobs, health endpoints
- [x] W3: Gateway with SSE streaming, hot keystore, health checks, budgets
- [x] W4: Web PWA with real unlock, vault screens, sync, mobile layout
- [x] W5: CLI with Spectre, full CRUD, clipboard, sync, env injection
- [x] W6: Desktop SSH with Avalonia, SSH/SFTP tabs, gateway integration
- [x] W7: Market with landing page, LemonSqueezy, posts, waitlist, metrics

---

## 🎯 Roadmap

### Post-v2.0.0
- **W6 Enhancements**: Session recording/sharing, team features
- **Mobile Apps**: Native iOS/Android with biometric unlock
- **Browser Extensions**: Chrome, Firefox, Safari autofill
- **Hosted Sync**: Managed hub service
- **Compliance Tier**: Audit exports, DPA, SSO
- **Enterprise Features**: Team management, RBAC

---

## 📝 Contributing

Please follow the [Contribution Guidelines](CONTRIBUTING.md) when contributing to this project.

---

## 📄 License

Aetheris v2 is licensed under the MIT License.

---

> **Quote**: "In God we trust. All others must bring data." - W. Edwards Deming

> **Remember**: Consistency (weekly shipping + writing) is the only lever fully under our control.
