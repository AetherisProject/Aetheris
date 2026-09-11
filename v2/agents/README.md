# agents/ — workstream cards

One card = one agent (or one focused human session). Read
`../PLAN-FOR-AGENTS.md` first; it defines ownership, order and the global rules.

## Card protocol

1. Take the lowest-numbered card whose dependencies are all ✅.
2. Edit **only** the directories your card owns.
3. Implement tasks in order; keep `dotnet build` green at every commit.
4. Run your card's acceptance gate; paste the full outputs into the PR body.
5. Tick the card's checkboxes in the PR. No checklist = no merge.

## Order & dependency graph

```
W0 (setup, in PLAN) → W1 core → ┌─ W2 hub ────────────┐
                                ├─ W3 gateway ─→ W4 PWA ├→ W6 desktop (commercial)
                                └─ W5 cli ────────────┘
```

| Card | Owns | Gate headline |
|---|---|---|
| [W1-core.md](W1-core.md) | `src/Aetheris.Core`, `tests/` | `dotnet test` green incl. AEAD tamper + no-plaintext-on-disk |
| [W2-hub.md](W2-hub.md) | `src/Aetheris.Hub` | curl round-trip: PUT blob → GET same blob; conflicts 409 |
| [W3-gateway.md](W3-gateway.md) | `src/Aetheris.Gateway` | chat call via vault key; budget blocks; logs contain **no** key |
| [W4-web-pwa.md](W4-web-pwa.md) | `src/Aetheris.Web` | installable PWA; E2E: unlock→edit→push→other client reads |
| [W5-cli.md](W5-cli.md) | `src/Aetheris.Cli` | scripted run of `mockups/cli.html` steps against real vault |
| [W6-desktop-ssh.md](W6-desktop-ssh.md) | `src/Aetheris.Desktop` | SSH into a container using a vault key; zeroize proven |

## Prompt template (paste to any AI agent)

```
You are agent <W#> on project Aetheris v2 (C#/.NET 8, hub-and-spoke secrets OS).
Read v2/PLAN-FOR-AGENTS.md and v2/agents/<card>.md. Work only in your owned
directories. Global rules: all crypto stays in Aetheris.Core; never log or
commit secrets; keep builds green; implement tasks in order; finish by running
the acceptance gate and report its outputs. Ask before deviating from the card.
```
