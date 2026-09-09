# Instructions

How to behave in this project.

## Core Principles

1. **Security first.** Never compromise on security. Never store secrets in plaintext. Never log sensitive data. Never skip input validation.

2. **Zero-knowledge by design.** The server never sees plaintext. The master password never leaves the device. All encryption happens client-side.

3. **Test everything.** Every public function has tests. Crypto gets property-based tests. No test requires network.

4. **Document everything.** Public APIs get rustdoc. User docs get Markdown. Changelog gets every change.

5. **No shortcuts in production code.** No `unwrap()`. No `expect()` without justification. No `todo!()` in shipped code.

## Workflow

1. Read `.memory/security.md` before ANY code change.
2. Read relevant module docs before modifying.
3. Write tests for new code BEFORE or ALONGSIDE the code.
4. Run `cargo fmt`, `cargo clippy`, `cargo test` before committing.
5. Store architectural decisions in `.memory/decisions.md`.
6. Store project conventions in `.memory/preferences.md`.
7. Store bug fixes and quirks in `.memory/quirks.md`.

## Git Workflow

- Branch from `main`: `feat/`, `fix/`, `docs/`, `refactor/`, `security/`
- PRs require: passing CI, code review, updated docs
- Squash merge to `main`
- Tags for releases: `v*.*.*`

## Code Review Checklist

- [ ] Security: no plaintext secrets, no unsafe crypto
- [ ] Tests: new code covered, all tests pass
- [ ] Docs: public APIs documented, changelog updated
- [ ] Style: `cargo fmt` clean, `cargo clippy` clean
- [ ] Cross-platform: builds on Windows, macOS, Linux
- [ ] i18n: user-facing strings use `fluent`
- [ ] Error handling: no `unwrap()` in production paths
- [ ] Logging: appropriate `tracing` spans and events
