# W1 — Core crypto & vault

- **Owns:** `src/Aetheris.Core/**`, `tests/Aetheris.Core.Tests/**`
- **Depends on:** W0 · **Effort:** 2–3 days · **Blocks:** everything

## Mission

The only code that ever derives keys or en/decrypts secrets. Argon2id →
HKDF subkeys → XChaCha20-Poly1305 envelopes; encrypted file vault with
decrypt-on-demand. Baseline exists and is close — your job is to harden it.

## Tasks

- [ ] 1.1 Run baseline: `dotnet test` — expect all green already; fix any
      BouncyCastle API drift (pin `BouncyCastle.Cryptography` to newest 2.x).
- [ ] 1.2 Add known-answer cross-checks: verify our Argon2id output matches a
      published RFC 9106 test vector (low-param variant) and add an
      XChaCha20-Poly1305 vector from the draft-irtf spec appendix.
- [ ] 1.3 `VaultStore`: add item history (keep last 10 envelopes per id in a
      `history` list inside the file) + `Restore(id, version)`.
- [ ] 1.4 Add export/backup: `Export(path)` writes the whole vault re-encrypted
      under the **backup** subkey; `Import(path)` round-trips. Test both.
- [ ] 1.5 Fuzz-lite: 100 random encrypt/decrypt rounds with random sizes,
      assert no exceptions and exact round-trip; 100 bit-flipped ciphertexts,
      assert 100 rejections.

## Acceptance gate

```bash
dotnet test tests/Aetheris.Core.Tests   # all green
# and the eyeball check: the vault file on disk contains zero secrets:
grep -c "sk-" /dev/null; strings aetheris.vault | grep -E "sk-|password|http" && echo FAIL
```

## Not yours

Hub/Gateway/Web/CLI behavior, new item types, Shamir/recovery, PQ mode.
