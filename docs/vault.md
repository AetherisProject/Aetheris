# Vault

The Aetheris Vault is a zero-knowledge encrypted store for all your secrets: passwords, SSH keys, API keys, notes, cards, and identities.

## Item Types

### Password
Website credentials with username, password, URL, TOTP, and custom fields. Breach-monitored, strength-rated, auto-changeable.

### SSH Key
Private key stored encrypted. Never written to disk unencrypted. Supports Ed25519, RSA, ECDSA. Passphrase-protected.

### SSH Connection
Connection configuration (host, port, username, auth method, key reference, jump host, port forwards). Health-monitored.

### API Key
Provider-aware key with rotation policy, health monitoring, and env var injection. Supports 30+ providers.

### Note
Encrypted text note with title and body.

### Card
Credit/debit card with number, expiry, CVV, and cardholder name.

### Identity
Personal information (name, address, phone, email) for autofill forms.

## Security

- All items encrypted individually with XChaCha20-Poly1305
- Each item has a unique nonce
- Items are never written to disk unencrypted
- Clipboard is cleared after configurable timeout
- Auto-lock after inactivity

## Operations

```bash
# Add an item
aeth vault add --type password --title "GitHub" --username "you@example.com"

# Get an item
aeth vault get --title "GitHub" --field password

# List items
aeth vault list --type password

# Update an item
aeth vault update --title "GitHub" --field password

# Delete an item
aeth vault delete --title "GitHub"

# Search
aeth vault search --query "github"
```

## Password Generator

Built-in password generator with configurable length and character sets.

```bash
# Generate a 32-character password
aeth vault generate --length 32

# Generate with custom symbols
aeth vault generate --length 32 --symbols "!@#$%"
```

## Security Dashboard

View breached, weak, reused, and old passwords at a glance. One-click fixes.
