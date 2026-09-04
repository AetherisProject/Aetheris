# Sync

Aetheris Sync keeps your vault up-to-date across all your devices with end-to-end encryption.

## How It Works

1. Vault changes are encrypted locally
2. Encrypted blob uploaded to S3-compatible storage
3. Other devices download the blob
4. Each device decrypts with its sync key
5. CRDT merge resolves conflicts

## Supported Backends

- **AWS S3** — Standard, IA, One Zone-IA
- **Backblaze B2** — S3-compatible API
- **Cloudflare R2** — Zero egress fees
- **MinIO** — Self-hosted S3
- **Wasabi** — Hot cloud storage

## Configuration

```toml
[sync]
enabled = true
provider = "s3"
bucket = "aetheris-vault"
region = "us-east-1"
object_key = "vault/blob.enc"
interval_seconds = 300
e2e_encrypted = true
offline_first = true
```

## Conflict Resolution

Uses CRDT (Conflict-free Replicated Data Type) merge. Last-writer-wins per field. Full history preserved for manual resolution if needed.

## Offline-First

All changes queued locally when offline. Automatically replayed when connection is restored. No data loss.

## Real-Time Sync

WebSocket-based push notifications for instant sync across devices. Changes appear on other devices within seconds.
