# PLAN-FOR-AGENTS — how the v2 base gets built to product grade

> Audience: AI coding agents and the human orchestrating them. One agent takes
> ONE workstream card. Cards own directories → no merge conflicts. Every card
> ends at an **acceptance gate** (commands + expected results). If a gate is
> red, the workstream is not done — no exceptions.

## North star

**Keys, passwords and an LLM gateway that work on all of the owner's devices —
base useful in weeks, commercial SSH desktop later.** Innovation = the gateway
(key injection from an encrypted vault) + one PWA everywhere + `aeth run` env
injection. Everything else supports that.

## Architecture contract (don't re-litigate inside a card)

```
        thin clients (PWA / CLI / desktop-later)
              │            │               │
              ▼            ▼               ▼
        Aetheris.Hub (blob sync)   Aetheris.Gateway (LLM proxy)
              │            │               │
              └────────────┴──── reads ────┘
                                   │
                    encrypted vault file = Aetheris.Core
                  (Argon2id → XChaCha20-Poly1305, per-item)
```

- Only `Aetheris.Core` derives keys or en/decrypts secrets.
- Hub stores ciphertext blobs with generation counters — **zero-knowledge**.
- Gateway is the declared **trusted component**: it sees keys, runs only on
  infrastructure the owner controls (localhost/VPS), binds 127.0.0.1 by default.
- All code logging flows through redaction paths — never log a key, ever.

## Workstreams

| W | Name | Owns (exclusively) | Depends on | Card |
|---|---|---|---|---|
| W0 | Repo & CI hygiene | `v2/.github` workflow, scripts | — | (in this file, below) |
| W1 | Core crypto & vault | `src/Aetheris.Core`, `tests/` | W0 | `agents/W1-core.md` |
| W2 | Hub sync server | `src/Aetheris.Hub` | W1 | `agents/W2-hub.md` |
| W3 | LLM gateway | `src/Aetheris.Gateway` | W1 | `agents/W3-gateway.md` |
| W4 | All-platform PWA client | `src/Aetheris.Web` | W1, W2, W3 | `agents/W4-web-pwa.md` |
| W5 | CLI | `src/Aetheris.Cli` | W1 | `agents/W5-cli.md` |
| W6 | Desktop SSH app (commercial) | `src/Aetheris.Desktop` | W1, W4 | `agents/W6-desktop-ssh.md` |
| W7 | Market: monetization & launch | positioning/pricing/launch artifacts | W1–W3 | `agents/W7-market.md` |

**Run order (solo + agents):** W1 → (W3 ∥ W2) → W5 → W4 → W6.
W3 and W2 are independent; W3 first = the personal "wow" moment sooner.

## W0 (30 minutes, do first, human)

- [ ] `dotnet --version` ≥ 8.0.x on the dev machine.
- [ ] `cd v2 && dotnet restore && dotnet build && dotnet test` — all green from
      this skeleton. If restore fails on a package version, bump to newest 2.x/8.x.
- [ ] CI job (extend `.github/workflows`): `dotnet test v2/` on ubuntu+windows.

## Global agent rules

1. **Ownership**: edit only files in your workstream's directories.
2. **Proof**: every card lists acceptance commands — run them, paste outputs.
3. **Secrets**: no plaintext keys in code, tests, logs, fixtures, or commits.
   Test fixtures use `sk-test-fake-…`. gitleaks must stay green.
4. **No new status memos**: report by updating the card's checkboxes + PR body.
5. **Small PRs**: one card = one PR, base `main`.
6. **Visuals**: PWA/desktop screens must match `mockups/*.html` tokens
   (deep-void `#090D16`, aether-indigo `#6366F1`, quantum-violet `#A855F7`,
   cyber-cyan `#06B6D4`, secure-emerald `#10B981`).

## The verification harness (every gate uses these)

```bash
dotnet test                                     # crypto/vault correctness
curl :8080/v1/health                            # hub alive
curl :7474/v1/models -H "X-Aetheris-Gateway-Token: dev"   # gateway alive
aeth ls --vault keys.vault                      # CLI reads vault
# e2e smoke: add apikey → gateway chat call → budget decrements → log redacted
```

## Risk register (top 5)

| Risk | Mitigation |
|---|---|
| Crypto implemented wrong | W1 gate = round-trip + tamper + cross-check vectors; never hand-roll beyond BouncyCastle usage in `CryptoEngine` |
| Gateway leaks a key into logs | W3 gate = log redaction test (assert key string absent from all outputs) |
| Scope creep (mobile native, CRDT, PQ…) | park everything to backlog; cards end at their gate |
| BouncyCastle/version drift | pin versions; W0 fixes restore first |
| Solo burnout | base = 4 workstreams; each ≤ 1 week; ship each gate |

## Path to first revenue (after W1–W4 are green)

1. Self-hostable OSS core (MIT) ⇒ GitHub stars, trust.
2. Hosted hub (you run Hub+Gateway for users) at ~€4/mo — LemonSqueezy checkout.
3. Desktop SSH app (W6) as the paid "Pro" front — Termius's turf, with a wedge
   it doesn't have: vault-injected SSH keys *and* an LLM gateway.
4. Launch surfaces: HN ("Show HN: local LLM gateway that keeps keys out of
   .env"), r/selfhosted, r/netsec, dev.to write-up of the gateway design.
