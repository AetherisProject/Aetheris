# Aetheris — Current Status

> Single source of truth for *what is true today*. Dated entries only; older
> entries get replaced, not appended. Target design: [`docs/DESIGN.md`](docs/DESIGN.md).

## 2026-09-11 — Baseline established

**Phase: M0 (Foundation) — design & consolidation.** Earlier repo documents
claimed "100 % complete / ready for deployment"; those claims were **not
accurate** and the offending files have been removed. Verified state:

### What exists (verified by inspection)

- Cargo workspace with 5 members: `core` (243 LOC, stubs), `platforms/cli`
  (151 LOC), `platforms/desktop` (33 LOC Tauri shell), `platforms/web`
  (29 LOC WASM), `platforms/mobile/lib` (FFI shell crate).
- Root `src/` tree: ≈10,900 LOC across 52 modules — **not wired into any
  crate** (workspace has no root package), therefore uncompiled/untested.
  Catalogued as reference scaffolding (DESIGN §4, ADR-005).
- Good, current docs: `docs/architecture.md`, `docs/design-system.md`
  (tokens), `.github/PROJECT.md` (milestones), CI workflow definitions.
- Design tokens + brand: defined (`docs/design-system.md`).

### Known problems (must fix)

1. 🔴 **Insecure crypto in `core/src/crypto.rs`** — XOR keystream built from
   hand-rolled FNV-1a hash: no KDF, no AEAD, no integrity. Replacement is
   milestone M0 task **C-1** and blocks all feature work (DESIGN §5.4).
2. 🔴 **No working build verification** — sandbox lacks a Rust toolchain;
   compilation status must be proven in CI, not assumed.
3. 🟡 **No tests** in the workspace (benches/examples reference modules that
   don't exist as wired crates).
4. 🟡 **Doc debt cleaned** — ~25 contradictory status/root memos removed on
   2026-09-11; README + ROADMAP re-pointed here. Rule: no new status files
   at repo root; update this file.

### Next actions (M0, in order)

- [ ] C-1: real crypto stack per DESIGN §5 (Argon2id, XChaCha20-Poly1305,
      HKDF, optional ML-KEM) + test vectors
- [ ] Repo restructure per DESIGN §4 (`src/` → `legacy/src-reference/`)
- [ ] Core v0.1: `crypto` + `vault` (7 item types, sled) with ≥80 % coverage
- [ ] CLI v0.1: `init/unlock/add/get/ls` end-to-end, green in CI

### 2026-09-11 (later) — v2 track added

Decision pivot: for the personal goal (keys + passwords + LLM gateway on all
platforms, useful from the base up) the stack moved to **C#/.NET 8,
hub-and-spoke** (self-hosted hub + thin clients; PWA first). See **`v2/`**:
runnable .NET skeleton (`Aetheris.sln`), real click dummies (`v2/mockups/`),
and the agent orchestration plan (`v2/PLAN-FOR-AGENTS.md` + `v2/agents/`).
The v1 Rust DESIGN remains the long-term blueprint; v2 supersedes it for
near-term execution. Agent order: W1 core → W3 gateway ∥ W2 hub → W5 CLI →
W4 PWA → W6 desktop SSH (the commercial layer).

*Update this file whenever reality changes. Do not create new status docs.*
