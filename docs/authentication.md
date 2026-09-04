# Authentication

## Modes

- **Local** — No accounts, vault lives only on your device
- **Cloud** — Aetheris-hosted auth servers (default)
- **Self-hosted** — Enterprise runs their own auth server

## Methods

- Email + password
- OAuth (Google, Apple, GitHub, GitLab)
- SSO / SAML (enterprise)
- WebAuthn / FIDO2 (hardware keys)

## MFA

- TOTP (RFC 6238)
- WebAuthn / FIDO2 (YubiKey)
- Recovery codes (generated at setup)
- Social recovery (trusted contacts)

## Password Flows

- Change password (verify current → set new → re-encrypt vault)
- Forgot password (email + TOTP/recovery → reset)
- Master password rotation (re-derive all keys)

## Sessions

- JWT with 24h expiry
- Refresh tokens with rotation
- Session revocation per device
- Multi-device session management