# VaultEngine Development Skill

Guides development of the Aetheris VaultEngine core.

## Focus
- Cryptographic implementations
- Vault item models
- Storage layer
- Security rules

## Rules
- Read .memory/security.md before any crypto code
- All secrets use Zeroize
- Constant-time comparison for secrets
- No unwrap() in production
- Property-based tests for crypto
