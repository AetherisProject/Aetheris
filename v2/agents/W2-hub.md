# W2 — Hub sync server

- **Owns:** `src/Aetheris.Hub/**`
- **Depends on:** W1 (envelope format only) · **Effort:** 2–4 days

## Mission

Zero-knowledge sync relay: opaque versioned blobs, conflict-safe generations,
device sessions. The hub must be boring on purpose — **it cannot read
anything it stores**, so keep it that way.

## Tasks

- [ ] 2.1 Replace `AETHERIS_TOKEN` static auth: device registration flow —
      `POST /v1/devices` (name + Ed25519 pubkey) then request-signed auth
      (X-Device / X-Signature headers; verify with BouncyCastle Ed25519).
- [ ] 2.2 Blob endpoints versioned per vault-generation: GET latest, PUT with
      `expectedGeneration`; store last N=50 generations for rollback.
- [ ] 2.3 Add `GET /v1/health`, `GET /v1/meta` (generation, size, updated).
- [ ] 2.4 Storage backends behind `IBlobStore`: file (default) + S3-compatible
      (W2 stretch; config-selected).
- [ ] 2.5 Rate limiting + security headers (aspnetcore middleware); tokens/
      pubkeys are the only secrets, and they must be hashed at rest.

## Acceptance gate

```bash
AETHERIS_TOKEN=dev dotnet run --project src/Aetheris.Hub &
curl -s :8080/v1/health                                  # {"ok":true,...}
curl -s -X PUT :8080/v1/blob -H 'X-Aetheris-Token: dev' \
     -H 'Content-Type: application/json' -d '{"generation":1,"data":"AAAA"}'
curl -s :8080/v1/blob -H 'X-Aetheris-Token: dev'         # → generation 1, data AAAA
# same PUT again → 409 generation_conflict
# wrong token → 401
```

## Not yours

Client-side sync logic (W4/W5), CRDT anything, multi-user sharing UI.
