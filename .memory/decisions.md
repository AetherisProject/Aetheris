# Decision: CLI Auto-Integration

## Context
The terminal must auto-integrate with vaults and SSH clients seamlessly.

## Decision
- **Use mock data** for CLI testing to avoid external dependencies.
- **Simulate decryption** with mock logic.
- **Test SSH client** with mocks.

## Implementation
- CLI auto-fetches vault items and decrypts API keys.
- SSH client mocks simulate connections and commands.

## Verification
- CLI logic verified with mock data.
- SSH client mocks tested successfully.

## Next Steps
- Fix CLI build setup for real deployment.
- Test real-world scenarios with actual vaults and SSH.
## Decision: Rust CI Toolchain (2026-09-13)

**Context:** `actions-rs/{rust,toolchain}@v1` org is archived and unmaintained.

**Decision:** Use `dtolnay/rust-toolchain@stable` — reads toolchain from ref, `targets:` (plural) input for cross-compilation.

**Also:**
- `actions/{upload,download}-artifact` bumped to v4
- Noble package names for tauri-in-workspace GTK builds
- FFI functions marked `unsafe` with `# Safety` doc sections for clippy

**Verification:** All real workflows green on `4933698`.
