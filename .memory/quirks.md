# Quirks

Project-specific weirdness — the non-obvious stuff.

## Windows Build: Proc-Macro DLL Collision

**Symptom:** `could not compile X (lib)` with exit code 1, no diagnostic. Different crate each run (syn, tracing-attributes, windows-sys, crypto-bigint, icu_locale_core).

**Cause:** rustc 1.98.1 MSVC toolchain has nondeterministic failures when compiling proc-macro crates in parallel. Proc-macros compile to DLLs, and parallel DLL emission collides.

**Fix:** Set `CARGO_BUILD_JOBS=2` to limit parallelism. This makes builds reliable.

**CI:** All CI configs use `CARGO_BUILD_JOBS=2`.

## Cargo Build on Windows: Output Truncation

**Symptom:** `cargo check` or `cargo build` output gets truncated or shows `[stderr]` with "Compiling" progress lines.

**Cause:** PowerShell captures the "Compiling" progress lines (which go to stderr in json format) and the NativeCommandError output. Long builds hit the 30s tool timeout before finishing.

**Fix:** For long builds, run in background with `Start-Process` and redirect to a file. Poll the file for completion.

## Offline Build: No Cargo.lock

**Symptom:** `cargo build --offline` fails with "no matching package named X found".

**Cause:** No `Cargo.lock` exists, and the offline cache doesn't have all dependencies pre-cached.

**Fix:** First build with network (or set up vendored deps), then `Cargo.lock` exists for offline builds.

## Tauri Build: WebView2 on Windows

**Symptom:** Tauri app fails to launch on clean Windows installs.

**Cause:** WebView2 runtime not installed.

**Fix:** Bundle the WebView2 installer or require it as prerequisite.

## Android Build: NDK Version Mismatch

**Symptom:** Rust build for Android fails with NDK errors.

**Cause:** Android NDK version doesn't match what the Rust toolchain expects.

**Fix:** Use `cargo-ndk` and specify NDK version. Set `ANDROID_NDK_HOME`.

## iOS Build: Code Signing

**Symptom:** Build succeeds but app won't install on device.

**Cause:** Missing provisioning profile or signing certificate.

**Fix:** Configure Xcode signing, or use `cargo-lipo` for simulator builds.

## WASM Build: Memory Limits

**Symptom:** WASM build runs out of memory during optimization.

**Cause:** `wasm-opt` uses too much memory.

**Fix:** Set `WASM_OPT=0` to disable optimization, or increase Node memory: `NODE_OPTIONS=--max-old-space-size=4096`.

## Russh: Key Format Compatibility

**Symptom:** SSH connection fails with "key format not supported".

**Cause:** OpenSSH changed key format between versions.

**Fix:** Use `russh-keys` 0.43+ which supports both old and new formats. Convert keys with `ssh-keygen` if needed.

## Russh: Host Key Verification

**Symptom:** First connection to a host fails with "host key verification failed".

**Cause:** Host key not in known_hosts.

**Fix:** Implement host key verification prompt. Store accepted host keys in vault (encrypted).

## Argon2: Performance on Low-End Devices

**Symptom:** Vault unlock takes >5 seconds on Raspberry Pi or old laptops.

**Cause:** Argon2id with m=64MB is too expensive for low-end hardware.

**Fix:** Detect available memory and adjust cost parameters dynamically. Allow user to configure.
