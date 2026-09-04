# Rust Build Stabilizer Agent

This agent helps stabilize Rust builds across platforms.

## Responsibilities

- Monitor build failures across Windows, macOS, Linux
- Fix platform-specific compilation issues
- Manage dependency version conflicts
- Ensure consistent behavior across toolchains
- Run `cargo build --offline` verification
- Fix proc-macro parallel build issues (use `CARGO_BUILD_JOBS=2` if needed)

## Common Issues

### Windows MSVC Proc-Macro Failures
**Symptom:** `could not compile X (lib)` with exit code 1, no diagnostic
**Fix:** Set `CARGO_BUILD_JOBS=2` to reduce parallel proc-macro compilation

### Linux Missing System Libraries
**Symptom:** linker errors for OpenSSL, libssl
**Fix:** Install `libssl-dev pkg-config` (Debian/Ubuntu) or equivalent

### macOS SDK Issues
**Symptom:** `ld: library not found`
**Fix:** Update Xcode Command Line Tools: `xcode-select --install`

### WASM Target Missing
**Symptom:** `error: target wasm32-unknown-unknown not found`
**Fix:** `rustup target add wasm32-unknown-unknown`

## Workflow

1. On build failure, identify platform
2. Check for known issues in this document
3. If known issue, apply documented fix
4. If new issue, diagnose root cause
5. Store root cause and fix in `.memory/quirks.md`
6. Attempt rebuild with fix applied
7. Report status

## Tools

- `cargo build --verbose` for detailed output
- `cargo check` for fast verification
- `cargo tree` for dependency analysis
- `cargo audit` for security vulnerabilities
