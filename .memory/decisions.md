# Decisions

Architectural commitments made in this project.

## 2026-09-05 — Hybrid Cross-Platform Architecture

We chose a hybrid architecture: Rust core with per-platform frontends.

- Rust core compiled to native (desktop/mobile/CLI) and WASM (browser)
- Desktop: Tauri 2 (Rust + React) — tiny binaries, native feel
- Browser: WebExtension MV3 (React + WASM) — same React components as desktop
- Mobile: Flutter (Dart + Rust FFI) — native mobile UX
- Web App: React (shared components)
- CLI: ratatui (Rust)

Why not pure web everywhere: SSH needs raw TCP, impossible in browser. Mobile needs native integrations (biometrics, autofill) that Capacitor can't deliver well.

Why not pure native: Too many UI codebases. Sharing React between desktop + browser is high-value reuse.

## 2026-09-05 — Hybrid Auth Model

Cloud-hosted by default, self-hosted option for enterprises.

- Cloud: Aetheris runs the auth servers, users create accounts
- Self-hosted: Enterprises run their own auth server
- Local mode: No accounts, purely local vault with S3 sync

Why: Most users want zero-config. Enterprises want data sovereignty.

## 2026-09-05 — Maximum Security Model

All security features enabled:

- Shamir Secret Sharing (M-of-N master key recovery)
- Plausible deniability (hidden volumes, duress mode, decoy entries)
- Post-quantum cryptography (CRYSTALS-Kyber/Dilithium hybrid)
- Memory encryption (in-RAM key encryption)
- Multi-device consensus (N-device approval for sensitive ops)
- Social recovery (trusted contacts)

Why: If we're building the most secure secrets manager, we go all the way.

## 2026-09-05 — Hybrid Browser Extension

WASM for offline viewing of cached decrypted items. Desktop app proxy for live sync and SSH.

- Extension has WASM-compiled Rust crypto — decrypts locally
- Works offline without desktop app running
- When desktop app is present: live sync, SSH terminal proxy, API key rotation
- WebSocket connection to localhost, authenticated with random token

Why: Browser extensions that require a desktop app feel broken when the app isn't running. Our extension is always useful, supercharged when desktop is present.

## 2026-09-05 — Proactive Engine as Core Feature

The "real manager" that keeps everything up-to-date:

- Auto-rotate API keys before expiry
- Auto-change passwords after breach detection
- Auto-monitor provider health
- Auto-sync across all devices
- Auto-backup on schedule
- Auto-cleanup unused/duplicate items
- Auto-update the tool itself
- Auto-remind and auto-alert

Why: A secrets manager shouldn't be passive. It should maintain your secrets so you never have to think about them.

## 2026-09-05 — Build Job Limit

Set `CARGO_BUILD_JOBS=2` to avoid proc-macro DLL collision on Windows MSVC (rustc 1.98.1).

Why: Parallel proc-macro compilation fails nondeterministically on this toolchain. Limiting jobs fixes it.
