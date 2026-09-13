# Quirks

## CI Platform Behaviors

### Tags-only workflow phantom runs
GitHub registers 0-job `failure` runs for workflows with only tag triggers on every branch push. File-content-independent. Not fixable via YAML edits.

**Affected:** `release.yml`, `screenshot_standalone.yml`
**Seen since:** 2026-09-09, pre-exists `dtolnay` migration

### coverage-summary artifact download hard-fails
When `test-coverage` job falls back to plain `cargo test` (no XML upload), the summary job's download step aborted the whole pipeline. Use `continue-on-error: true`.

## Windows Git Workarounds

### `:memory:` path invalid on NTFS
`git apply` and `git checkout` cannot create files with `:` in paths. Split per-file patch, apply via temp index, or use `git update-index --force-remove`.

## Build Environment

### Noble GTK package names
ubuntu-latest is 24.04. `libwebkit2gtk-4.0-dev` / `libsoup-2.4-dev` don't exist. Use:
- `libwebkit2gtk-4.1-dev`
- `libsoup-3.0-dev`
- `libglib2.0-dev`

## Rust Lint Issues

### `-D warnings` with benign cargo messages
Workspace profile warning triggers `-D warnings` and aborts builds. Let `cargo clippy` exit code propagate directly through `tee` (pipefail).
