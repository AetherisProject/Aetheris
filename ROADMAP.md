# Aetheris Roadmap

> **Scope:** near-term priority queue only. The full phased plan (M0 → M4, with
> dates and exit criteria) lives in **[docs/DESIGN.md §13](docs/DESIGN.md)**.
> Current verified state: **[STATUS.md](STATUS.md)**.

## Now: Milestone M0 — Foundation & Hygiene (→ 2026-12-31)

1. **C-1 — Replace insecure crypto** in `core/src/crypto.rs` with the canonical
   stack (Argon2id · XChaCha20-Poly1305 · HKDF-SHA256; optional ML-KEM hybrid).
   Add known-answer + round-trip property tests. *Blocks everything else.*
2. **Repo restructure** — move root `src/` (≈10.9 kLOC uncompiled reference)
   to `legacy/src-reference/`; fold stray root dirs into workspace members.
3. **Core v0.1** — real `crypto` + `vault` modules (7 item types, sled store,
   per-item AEAD), ≥80 % line coverage.
4. **CLI v0.1** — `aeth init/unlock/add/get/ls` end-to-end against the vault.
5. **CI proof** — workspace build+test matrix (linux/macOS/windows + wasm32),
   `cargo audit`, `clippy -D warnings`, gitleaks.

## Next: Milestone M1 — MVP Everywhere (→ 2027-03-31)

- All five shells (desktop, browser ext, mobile, web, CLI) bind to core v0.1
  and pass their "done when" checks (DESIGN §12)
- Sync v0: device pairing + encrypted blob push/pull via `aetheris-server`
- TOTP support in vault items on all platforms
- Import: Bitwarden / Chrome CSV

## Later (summary — details & dates in DESIGN §13)

| Milestone | Theme | Target |
|---|---|---|
| M2 | SSH terminal, autofill everywhere, API-key adapters, proactive engine, admin console | 2027-06-30 |
| M3 | Shamir recovery, duress vaults, PQ-hybrid graduation, WebAuthn, external audit | 2027-09-30 |
| M4 | i18n (Fluent), family/team sharing, import wizards, plugin SDK, **1.0 release** | 2027-12-31 |

## Contribution notes

- Feature requests and bugs: GitHub Issues (labels: `crypto`, `vault`, `sync`,
  `ssh`, `ui`, `docs`, `security`).
- PRs must follow the rules in [AGENTS.md](AGENTS.md) and pass CI.
- Priorities are re-evaluated at each milestone boundary — propose changes via
  PR against this file.
