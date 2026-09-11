# AGENTS.md — Working Rules for Contributors & Agents

## Canonical documents (read these, trust only these)

1. [`docs/DESIGN.md`](docs/DESIGN.md) — target architecture & delivery plan (change by PR only)
2. [`STATUS.md`](STATUS.md) — current verified state (dated, honest)
3. [`ROADMAP.md`](ROADMAP.md) — next-milestone priority queue

**Do not create new status/summary/completion files** (no `COMPLETE*.md`,
`FINAL_*.md`, `DEPLOYMENT_*.md` memos). Update `STATUS.md` instead. Notable
decisions go into DESIGN Appendix B as ADR entries.

## Where the code really lives

- Compiled workspace members: `core/`, `platforms/{cli,desktop,web,mobile/lib}`.
- Root `src/`: **reference scaffolding, not part of any crate** — migrate
  modules into `core/src/` with tests, then delete from `src/` (DESIGN §4).
- Don't add dead code: every module must be reachable from a workspace crate
  and covered by tests (core coverage target ≥80 %).

## Security invariants (non-negotiable)

- All crypto in `aetheris-core` only; shells never derive keys or handle
  plaintext secrets. Algorithms per DESIGN §5.2 — no hand-rolled crypto.
  (Known defect: legacy FNV keystream in `core/src/crypto.rs` — M0 task C-1.)
- Secrets use `SecureString`/`SecureVec`, zeroized on drop; never printed,
  logged, or serialized in plaintext.
- Sync server is zero-knowledge: ciphertext blobs only.
- No secrets in git — gitleaks + `cargo audit` gate every PR.

## Build & verify

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

Prefer proof over claims: a feature is "done" only when CI proves it.
