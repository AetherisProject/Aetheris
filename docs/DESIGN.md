# Aetheris — Master Design Document

> **Status:** Authoritative design baseline · v1.0 · 2026-09-11
> **Scope:** This document is the single source of truth for *what* Aetheris is,
> *how* it is architected, and *in which order* it gets built. It supersedes all
> previous scattered status/vision documents (see §14, Governance).
>
> Readers: start with §1–§3. Engineers: §4–§12. Everyone shipping code: §13–§15.

---

## 1. Vision

**Aetheris is the Secrets Operating System**: one encrypted vault that holds
passwords, SSH keys & connections, API keys, notes, cards and identities —
usable identically from a desktop app, a browser extension, a mobile app, a web
app, and a CLI.

Three pillars, in order of importance:

1. **Zero-Knowledge by design** — no server ever sees plaintext. All crypto
   happens client-side inside one shared Rust core.
2. **Zero-Trust by default** — every device, session and sync operation is
   authenticated; compromise of the sync server loses the attacker nothing but
   ciphertext.
3. **Zero-Friction in use** — unlock in under a second, one flow per secret
   type, security that is invisible until it protects you.

Non-goals (v1): password manager for mass consumer marketing, free hosted sync
service, blockchain/distributed-ledger features.

---

## 2. Reality Baseline (verified 2026-09-11)

Prior documentation claimed "100 % complete, ready for deployment". The
verified repository state is:

| Area | Claimed | Verified reality |
|---|---|---|
| Cargo workspace | 13 modules complete | 5 stub crates: `core` (243 LOC), `platforms/cli` (151 LOC), `platforms/desktop` (33 LOC), `platforms/web` (29 LOC), mobile FFI shell |
| Root `src/` (≈10.9 kLOC, 52 modules) | core feature set | **Not part of any crate** — no root `[package]` exists, so none of it is compiled or tested. Treated as *reference scaffolding* to harvest, per §4 |
| `core/src/crypto.rs` | "post-quantum hybrid encryption" | **Insecure**: XOR keystream derived with a hand-rolled FNV-1a hash — no KDF, no authentication, no integrity. Must be replaced (§5.4) |
| Test suite | 100+ tests passing | Workspace has no tests; sandbox has no Rust toolchain to verify compilation (CI must be the verifier) |
| Status docs | ~20 root `COMPLETE*.md` / `FINAL*.md` / `DEPLOYMENT*.md` files | Contradictory; consolidated into this document + `STATUS.md`; noise files removed (§14) |

**Consequence:** the plan in §13 starts from Milestone M0 (foundation), not
from "deployment". Everything in §5–§12 is the *target design*; the baseline
column of §13 states where each piece stands today.

---

## 3. System Architecture

### 3.1 One core, five shells

```
                        ┌──────────────────────────────────────────────┐
                        │            aetheris-core  (Rust)             │
                        │                                              │
                        │  crypto   vault   sync   auth   ssh          │
                        │  apikey   proactive   design-tokens          │
                        │                                              │
                        │        C ABI · wasm-bindgen · FRB bridge      │
                        └───┬───────┬───────┬───────┬───────┬─────────┘
                            │       │       │       │       │
                      ┌─────┴──┐ ┌──┴────┐ ┌┴─────┐ ┌┴─────┐ ┌┴─────┐
                      │Desktop │ │Browser│ │Mobile│ │ Web  │ │ CLI  │
                      │ Tauri2 │ │MV3+WASM│ │Flutter│ │React │ │clap+ │
                      │ +React │ │ React │ │ +FFI  │ │/WASM │ │ratatui│
                      └────────┘ └───────┘ └──────┘ └──────┘ └──────┘
                            │       │       │       │       │
                            └───────┴───┬───┴───────┴───────┘
                                        │  TLS 1.3
                              ┌─────────┴──────────┐
                              │  aetheris-server   │
                              │  zero-knowledge    │
                              │  blob store (S3)   │
                              └────────────────────┘
```

Rules that keep this architecture honest:

- **Rule A — All secrets live in `aetheris-core`.** Shells render UI and move
  bytes; they never derive keys, encrypt, decrypt, or parse secrets.
- **Rule B — One binding mechanism per platform, generated where possible:**
  Tauri commands (desktop), wasm-bindgen (browser/web), flutter_rust_bridge
  (mobile), direct linking (CLI). No hand-written parallel logic in shell code.
- **Rule C — The server is a dumb blob store.** It stores opaque, versioned,
  encrypted objects and serves them back. It performs no crypto and holds no
  keys.

### 3.2 Layered view

```
L7  Application   shells (Tauri/React, Flutter, ratatui, MV3 extension)
L6  API surface   Tauri commands · WASM exports · FRB · CLI subcommands
L5  Services      vault · auth · sync · ssh · apikey · proactive
L4  Crypto        Argon2id · XChaCha20-Poly1305 · HKDF · Ed25519 · X25519
L3  Memory        SecureString/SecureVec · zeroize-on-drop · subtle compare
L2  Storage       sled (local, per-item AEAD) · encrypted sync blobs
L1  Platform      OS RNG · keychain/secure enclave (device keys only)
```

---

## 4. Repository Layout (target)

```
Aetheris/
├── Cargo.toml                 # workspace root (no root package)
├── core/                      # aetheris-core — the VaultEngine (rlib+cdylib)
├── server/                    # aetheris-server — zero-knowledge sync relay (axum)  [new]
├── platforms/
│   ├── desktop/               # Tauri 2 shell
│   ├── cli/                   # aetheris-cli → binary `aeth`
│   ├── web/                   # WASM pkg for web & extension
│   ├── mobile/lib/            # flutter_rust_bridge FFI crate
│   └── browser-ext/           # MV3 extension JS/TS (consumes web/pkg)   [new]
├── apps/                                                                      [new]
│   ├── desktop-ui/            # React+TS desktop frontend (from /desktop)
│   └── web-ui/                # React+TS web frontend      (from /web)
├── legacy/src-reference/      # today's root src/ — harvested then deleted    [moved]
├── docs/                      # this document + user docs (built by _build.py)
├── docker/                    # deployment assets
└── tests/                     # cross-crate integration + e2e harness
```

Migration notes (M0):

- The root `src/` tree moves to `legacy/src-reference/` with a README saying
  "reference only — do not import". Modules are ported into `core/src/`
  incrementally, each port adding unit tests; when a module lands in `core`,
  its legacy copy is deleted.
- Stray root dirs (`auth/`, `proactive/`, `benches/`, `examples/`,
  `rustfmt-module/`, `Aetheris/`) are folded into their proper workspace homes
  or deleted. No code lives outside workspace members after M0.
- `Cargo.lock` is committed (binary workspace).

---

## 5. Security Architecture

### 5.1 Threat model

| Adversary | Powers | Our guarantee |
|---|---|---|
| Sync-server compromise (incl. rogue operator) | full read/write of server DB | learns nothing but ciphertext & access timing |
| Network MITM | intercept/modify traffic | TLS 1.3 + client-side AEAD; tampering detected |
| Lost device, vault at rest | offline disk access | vault is AEAD ciphertext; Argon2id slows brute force |
| Malware with vault unlocked | reads process memory | best-effort: zeroize-on-drop, mlock where available, short unlock TTL — **documented as out of full scope** |
| Quantum adversary (harvest-now-decrypt-later) | stores ciphertext today | optional PQ-hybrid wrapping (§5.3) |

### 5.2 Primitives (canonical choices)

| Purpose | Algorithm | Crate | Rationale |
|---|---|---|---|
| Master KDF | Argon2id, t=3, m=64 MiB, p=4 | `argon2` | memory-hard, RFC 9106 recommendation |
| AEAD (vault items, blobs) | XChaCha20-Poly1305 | `chacha20poly1305` | 192-bit nonces safe for random use, no nonce-misuse hazard of AES-GCM |
| Key separation | HKDF-SHA256 | `hkdf`, `sha2` | derive vault/sync/auth/backup/recovery subkeys |
| Device identity / signatures | Ed25519 | `ed25519-dalek` | small keys, deterministic, fast verify |
| Device key exchange | X25519 | `x25519-dalek` | pair new devices, session keys |
| Secret sharing (recovery, duress) | Shamir over GF(2⁸) | `sharks` | M-of-N recovery shares |
| Password hashing on server (session only) | EdDSA challenge-response instead of stored hashes | `ed25519-dalek` | server stores only device public keys |
| RNG | OS CSPRNG | `rand` `OsRng` | audited entropy source |
| Constant-time compare | — | `subtle` | MAC/tag/key comparisons |
| Memory hygiene | zeroize-on-drop | `zeroize`, `secrecy` | SecureString/SecureVec wrappers |
| **Experimental:** PQ hybrid wrap | X25519 + ML-KEM-768 (Kyber); ML-DSA (Dilithium) signatures | `oqs` (already a dependency) | harvest-now/decrypt-later hedge; off by default until audited |

### 5.3 Key hierarchy

```
master password ──Argon2id(salt=32B, t=3, m=64MiB, p=4)──▶ master key (32B)
                                                                │
        ┌───────────────┬───────────────┬───────────────┬──────┴──────┐
     HKDF("vault")   HKDF("sync")    HKDF("auth")    HKDF("backup") HKDF("recovery")
        │               │               │               │              │
   vault key        sync key        auth key        backup key    recovery key
   (item AEAD)      (blob AEAD)    (device sig)    (export AEAD)  (Shamir input)
   [+ optional ML-KEM hybrid wrap of the whole hierarchy — mode flag in header]
```

Every ciphertext is stored with a versioned header:
`magic | version | kdf_params | mode(flag: classic|pq-hybrid) | nonce | tag`,
so algorithms can be rotated without breaking old vaults.

### 5.4 🔴 Known defect to fix first

`core/src/crypto.rs` today "encrypts" by XOR-ing data with a keystream built
from a hand-rolled FNV-1a hash. This provides **no confidentiality guarantee,
no integrity, and a 64-bit non-cryptographic hash**. It is the single most
important thing to replace:

- **M0 task C-1:** delete that implementation; implement §5.2/§5.3 with
  `argon2` + `chacha20poly1305` + `hkdf` + ML-KEM optional wrap; add RFC test
  vectors + round-trip property tests. No downstream feature merges before
  C-1 lands.

### 5.5 Memory & coding rules

- All secret bytes live in `SecureVec<u8>`/`SecureString` (zeroize-on-drop).
- Never log, `Debug`-print, or serialize secrets outside encrypted payloads;
  a `clippy` lint + gitleaks CI gate enforce this.
- Decrypt-on-demand: items are decrypted at point of use and re-zeroized,
  not held in a big plaintext map.
- Dependencies: `cargo audit` + Dependabot gate every PR.

---

## 6. Data Model

### 6.1 Vault item types (7)

`Password` (username, password, urls, totp, notes),
`SshKey` (private/public key, type, fingerprint, comment),
`SshConnection` (host, port, username, key_id ref, forwarding rules),
`ApiKey` (provider, key, scopes, expiry, rotation_policy),
`Note` (title, body), `Card` (number, expiry, cvv, holder),
`Identity` (name, email, phone, address).

All share an envelope:

```rust
struct ItemEnvelope {              // unencrypted index fields (local only)
    id: Uuid,
    item_type: ItemType,
    title: String,                 // encrypted separately on sync
    tags: Vec<String>,
    favorite: bool,
    trashed: Option<Hlc>,
    created_at: Hlc, updated_at: Hlc,   // hybrid logical clock (§7)
    version: VectorClock,
    cipher_header: CipherHeader,        // §5.3
    payload: Vec<u8>,                   // XChaCha20-Poly1305(bincode(ItemPayload))
}
```

### 6.2 Local storage (sled trees)

| Tree | Contents |
|---|---|
| `items` | `id → ItemEnvelope` (encrypted payload) |
| `index_title`, `index_tag`, `index_type` | secondary indices → id sets (rebuilt on unlock) |
| `history` | `id → Vec<Envelope>` last N=10 revisions |
| `sync_state` | vector clock, per-peer cursors, tombstone set |
| `meta` | kdf params, salt, mode flags, vault generation |

Backups/export: single-file format = `header | encrypted dump`, derived with
`backup key`; portable and restorable on any platform.

---

## 7. Sync Engine (Zero-Knowledge)

- **Model:** offline-first. Local sled is the source of truth; sync reconciles
  replicas.
- **Conflict resolution:** per-item Last-Writer-Wins register keyed by hybrid
  logical clock + vector clock, with tombstones for deletes and per-field
  merge reserved for `Note` bodies (v1.1). This is a pragmatic CRDT; upgrading
  to Automerge/Yrs later requires only changing the merge layer behind
  `trait MergeStrategy`.
- **Transport:** device registers an Ed25519 key → server issues short-lived
  challenge-response sessions. Blobs pushed/pulled over HTTPS; optional
  WebSocket channel for live invalidation. Server backend = S3-compatible
  object store (`aws-sdk-s3` already planned) or local FS for self-hosting.
- **Pairing:** new device shows a QR / 6-word code; existing device wraps the
  vault key with an X25519 ECDH box addressed to the new device key
  (+ ML-KEM in hybrid mode).
- **Zero-knowledge boundary:** server rows =
  `(vault_id, generation, blob, writer_pubkey, signature)`. Titles/tags are
  inside the encrypted payload on sync (local index only).

---

## 8. Authentication & Recovery

| Concern | Design |
|---|---|
| Local unlock | master password → Argon2id → master key; auto-relock after TTL; wrong-password backoff (exponential, in-memory) |
| TOTP 2FA (vault items) | standard RFC 6238 generator stored in vault; QR export for provisioning |
| Account 2FA (sync) | TOTP first; WebAuthn/passkeys via `webauthn-rs` as second factor v1.1 |
| OAuth | optional sign-in to *sync relay* via Google/Apple/GitHub device flow; never used for vault decryption |
| Device keys | per-device Ed25519 identity + X25519 exchange key, stored in OS keychain / Secure Enclave / Android Keystore |
| Recovery | Shamir M-of-N over the recovery key → trusted contacts or printed shares; duress mode maps a second password to a decoy vault (UI + crypto support, §5.3 mode flag) |
| Sessions (server) | Ed25519 challenge-response → short-lived bearer token; refresh by re-challenge |

Biometrics (FaceID/TouchID/Windows Hello/Android BiometricPrompt) **only gate
the locally stored device key** — they never replace the master key hierarchy,
and are re-prompted after OS credential changes.

---

## 9. Proactive Engine

In-core scheduler (`tokio` interval tasks, persisted queue) with policies:

- **Rotate** API keys N days before expiry via provider adapters
  (`apikey/providers.rs` contract: `health_check`, `rotate`, `revoke`).
- **Breach watch** passwords via k-anonymity range query (HIBP-compatible).
- **Health ping** providers/endpoints on 15-min cadence.
- **Auto-backup** encrypted export daily to configured target.
- **Cleanup**: unused (>1y), duplicate, weak-password reports → action queue.

Design rule: the engine *proposes*, the user *disposes* — every destructive
action lands in an approval queue with undo via item history. Desktop/CLI run
the scheduler locally; web/mobile receive its results via sync.

---

## 10. SSH Terminal

- Pure-Rust `russh` client inside core: password/key auth (keys pulled from
  vault by id), agent-less forwarding, local/remote port forwarding, jump
  hosts, SFTP panel.
- Session model: `SshSession` emits a stream of terminal events; shells render
  with xterm.js (desktop/web/extension) or a ratatui viewport (CLI).
- Secrets rule: the decrypted private key is handed to russh in-memory and
  zeroized on disconnect; it is never written to `~/.ssh`.
- Mobile v1: connect + interactive shell; SFTP in v1.1.

---

## 11. API Key Engine

- Typed `ApiKey` vault item + provider registry (adapter trait, 30+ provider
  stubs to be filled in M2; start with GitHub, OpenAI, AWS, Stripe, Cloudflare).
- Rotate / health-check / revoke via adapters; usage metadata kept locally.
- **Injection** instead of exposure: CLI/desktop can spawn a process with the
  key materialized into env vars of the child only — never printed, never in
  shell history.
- Browser extension can fill provider dashboards (detection rules per domain).

---

## 12. Platform Designs (equal priority)

All five shells are first-class release artifacts from Milestone M1 onward;
each section = stack, binding, key flows, definition of done.

### 12.1 Desktop — `platforms/desktop` + `apps/desktop-ui`

| | |
|---|---|
| Stack | Tauri 2, React 18 + TypeScript, Vite, xterm.js |
| Binding | Tauri commands 1:1 over service traits in core (`vault_*`, `auth_*`, `sync_*`, `ssh_*`) |
| Signature flows | unlock → vault grid; add/edit item sheet; SSH tab with split terminal+SFTP; proactive inbox; settings |
| OS integration | keychain device key, auto-lock on sleep, tray, global autofill shortcut (M2) |
| Done when | create → encrypt → restart → unlock → item intact; SSH opens xterm session using a vault key |

### 12.2 Browser Extension — `platforms/browser-ext` (+ `platforms/web` WASM)

| | |
|---|---|
| Stack | WebExtension MV3 (Chrome/Firefox/Edge/Brave; Safari via Xcode wrapper), React popup |
| Binding | WASM build of core for vault ops; optional local WebSocket relay to the running desktop app for SSH/heavy ops |
| Signature flows | autofill passwords; fill API keys on provider dashboards; save-login prompt; inline password generator |
| Done when | unlock via paired desktop or WASM vault; autofill on standard login forms; no secrets in extension storage unencrypted |

### 12.3 Mobile — `platforms/mobile/lib` + Flutter app

| | |
|---|---|
| Stack | Flutter 3.10+, Dart; `flutter_rust_bridge` 1.82 to `aetheris-ffi` |
| Binding | generated Dart↔Rust bridge; vault ops identical API to desktop |
| Signature flows | biometric-gated unlock (device key), vault list, QR device pairing, autofill service (Android) / AutoFill extension (iOS), TOTP viewer |
| Done when | pair via QR → sync → view/use items; biometric unlock; background relock |

### 12.4 Web App — `apps/web-ui` + `server`

| | |
|---|---|
| Stack | React 18 + TypeScript SPA; talks to `aetheris-server` HTTPS API or runs vault fully in-browser via WASM |
| Modes | (a) management console against your synced vault (blob fetched, decrypted in WASM), (b) admin/console UI for self-hosted server |
| Signature flows | vault management, security dashboard, family/team admin, share management |
| Done when | login → fetch blob → decrypt in WASM → edit → push merged blob; zero plaintext server-side (verified by e2e test) |

### 12.5 CLI — `platforms/cli` → binary `aeth`

| | |
|---|---|
| Stack | `clap` v4 commands + optional `ratatui` full-screen TUI (`aeth tui`) |
| Binding | links core directly (no FFI) |
| Command surface | `init · unlock · add · get · ls · rm · search · totp · export · import · sync · ssh connect · ssh sftp · apikey rotate · doctor` |
| Signature flows | headless scripting (JSON out via `--json`), clipboard w/ auto-clear, env-injection runner (`aeth run --env-from <id> -- <cmd>`) |
| Done when | full CRUD + sync + ssh from terminal with zero GUI deps; scripts exercised in CI on Linux/macOS/Windows |

---

## 13. Delivery Plan (M0 → M4)

Aligned with the milestone dates already configured in `.github/PROJECT.md`.
Exit criteria are binary — a milestone is done only when its checkboxes are
green in CI.

### M0 — Foundation & Hygiene (now → 2026-12-31)
*Baseline: workspace stubs exist; 10.9 kLOC uncompiled reference; crypto insecure.*

- [ ] **C-1 crypto replacement** (§5.4) with test vectors — *blocks everything*
- [ ] Repo restructure per §4 (move `src/` → `legacy/src-reference/`, fold stray dirs)
- [ ] `aetheris-core` v0.1 real modules: `crypto`, `vault` (7 types, sled), `error`; ≥80 % coverage on these two
- [ ] `aeth` CLI v0.1: `init/unlock/add/get/ls` end-to-end
- [ ] CI: build+test matrix for workspace; `cargo audit`, `clippy -D warnings`
- [ ] Docs: this DESIGN + honest STATUS + README (done), ROADMAP re-pointed here

### M1 — MVP Everywhere (→ 2027-03-31)
*Baseline: core works; shells are stubs.*

- [ ] All five shells bind to core v0.1 and pass their §12 "done when"
- [ ] Sync v0: device pairing + blob push/pull against `aetheris-server`
- [ ] TOTP in vault items on all platforms
- [ ] Import: Bitwarden/Chrome CSV

### M2 — Platform Powers (→ 2027-06-30)
- [ ] SSH terminal (russh + xterm.js) on desktop/web; CLI `ssh connect`
- [ ] Extension autofill + provider dashboard filling
- [ ] Mobile autofill services; desktop global autofill shortcut
- [ ] API key adapters: top 5 providers + health checks
- [ ] Proactive engine v1 (rotation + breach watch + approval queue)
- [ ] Web admin console (users/teams/audit) for self-hosted server

### M3 — Advanced Security (→ 2027-09-30)
- [ ] Shamir recovery + trusted-contact flow
- [ ] Duress/decoy vaults (deniability) — UI + crypto
- [ ] PQ-hybrid mode graduates from experimental flag after audit
- [ ] WebAuthn/passkeys 2FA
- [ ] External security audit #1 + remediation; bug-bounty policy

### M4 — Polish & Network (→ 2027-12-31)
- [ ] i18n framework (Fluent) + first language packs; RTL
- [ ] Family/Team sharing UI over the admin engine
- [ ] Import wizard: 1Password, KeePass, LastPass
- [ ] Marketplace/provider plugin SDK
- [ ] 1.0 release: semver core API, signed installers/notarization

---

## 14. Governance & Repo Hygiene

**Doc hierarchy (nothing else may claim status):**

| File | Role |
|---|---|
| `docs/DESIGN.md` | this document — the target design (change by PR only) |
| `STATUS.md` | current verified state — short, dated, honest |
| `ROADMAP.md` | priority queue for the *next* milestone only |
| `CHANGELOG.md` | released changes |
| `README.md` | elevator pitch + links to the above |

**Cleanup executed 2026-09-11:** removed ~25 auto-generated
"COMPLETE/FINAL_ANSWER/DEPLOYMENT_*" status files, scratch test files,
stale sled artifacts (`:memory:/`, `test_vault_db/`), and stray duplicates
(root `auth/`). Their substance — where real — is folded into this document.
Rule going forward: **no new status memos at repo root**; update `STATUS.md`.

**Decision records:** notable design decisions get a short ADR entry appended
to `docs/DESIGN.md` Appendix B (below) rather than new files.

---

## 15. Testing & Quality Strategy

| Layer | Method | Gate |
|---|---|---|
| crypto | RFC/known-answer vectors, round-trip proptest, tamper-detection tests | must pass before merge |
| core services | unit + proptest (CRDT merge, serialization) | ≥80 % line coverage (M0+) |
| FFI/WASM | generated-binding smoke tests in CI | per-PR |
| shells | §12 "done when" e2e per platform | per-milestone |
| security | `cargo audit`, gitleaks, clippy lints, `cargo-fuzz` on decrypt/parse paths | per-PR / nightly |
| perf | `criterion` benches: unlock < 1 s (Argon2id t=3), item decrypt < 5 ms, sync merge of 10 k items < 500 ms | nightly |

CI matrix: Rust stable × {linux, macos, windows}, wasm32 target, Android
cross-build; Flutter test job; extension build job.

---

## Appendix A — Technology Choices (canonical)

| Area | Choice | Locked at |
|---|---|---|
| Language (core) | Rust 1.70+ | now |
| Local DB | sled (embedded) | now |
| Async | tokio | now |
| Desktop shell | Tauri 2 + React/TS | M1 |
| Mobile shell | Flutter + flutter_rust_bridge 1.82 | already in tree |
| Web shell | React/TS SPA + axum server | M1 |
| Browser ext | MV3 + WASM | M2 |
| CLI | clap 4 + ratatui | M0 |
| Sync transport | HTTPS/WebSocket → S3-compatible | M1 |
| i18n | Fluent | M4 |

## Appendix B — Decision Log (ADRs)

- **ADR-001 (2026-09-11) XChaCha20-Poly1305 over AES-GCM** for vault AEAD:
  random nonces are safe (192-bit), no nonce-misuse foot-gun, constant-time
  SW fallbacks on all platforms incl. WASM.
- **ADR-002 (2026-09-11) Item-level LWW+HLC+vector-clock CRDT** for sync MVP;
  `MergeStrategy` trait keeps Automerge/Yrs upgrade path open (M4+).
- **ADR-003 (2026-09-11) Server stores device public keys, not password
  hashes** — Ed25519 challenge-response sessions; OAuth only gates the account,
  never the vault.
- **ADR-004 (2026-09-11) Biometrics gate device keys only**; master-key
  hierarchy untouched; re-prompt on OS credential reset.
- **ADR-005 (2026-09-11) Root `src/` is reference scaffolding**, not a crate;
  harvested per-module with tests, then deleted (§4).
