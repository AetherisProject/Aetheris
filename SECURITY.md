# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in Aetheris, please report it responsibly.

**DO NOT** open a public GitHub issue for security vulnerabilities.

Instead, email: **security@aetheris.dev**

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will acknowledge your report within 48 hours and provide a detailed response within 7 days.

## Security Features

- **Zero-knowledge architecture** — Server never sees plaintext
- **Shamir Secret Sharing** — M-of-N master key recovery
- **Plausible deniability** — Hidden volumes, duress mode, decoy entries
- **Post-quantum cryptography** — CRYSTALS-Kyber/Dilithium hybrid
- **Memory encryption** — Keys encrypted in RAM
- **Multi-device consensus** — N-device approval for sensitive ops
- **Social recovery** — Trusted contacts for recovery

## Supported Versions

| Version | Supported |
|---------|:---------:|
| 0.1.x   | ✅        |

## Security Audit

We run `cargo audit` on every CI build and use `gitleaks` for secret scanning.
All dependencies are kept up to date via Dependabot.
