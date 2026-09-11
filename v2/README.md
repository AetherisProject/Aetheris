# Aetheris v2 — Hub-and-Spoke Edition (.NET 8 / C#)

> **This folder is the real starting base.** It implements the v2 architecture:
> **one self-hosted hub + thin clients on every platform**, chosen for fastest
> time-to-value. The v1 Rust design (`docs/DESIGN.md`) remains the longer-term
> blueprint; the domain model, crypto choices and milestones translate 1:1.

## Why this shape

- **One codebase, every platform**: ASP.NET hub + Blazor PWA (desktop/mobile/tablet
  browsers, installable) + CLI + later Avalonia desktop (native SSH).
- **The LLM gateway is the wedge**: one local/self-hosted endpoint that injects
  your provider keys from the encrypted vault. Keys never live in `.env`,
  shell history, or source code again.
- **Base-first**: passwords + API keys + gateway are useful from week one;
  the native SSH desktop app is the commercial layer on top.

## Solution map

```
v2/
├── Aetheris.sln
├── src/
│   ├── Aetheris.Core/      # crypto (Argon2id + XChaCha20-Poly1305 via BouncyCastle),
│   │                       # key hierarchy, 7 vault item types, encrypted file vault
│   ├── Aetheris.Hub/       # ASP.NET: zero-knowledge blob sync (server sees ciphertext only)
│   ├── Aetheris.Gateway/   # ASP.NET: OpenAI-compatible LLM gateway, key injection,
│   │                       # model aliases + fallback chains, token budgets, redacted logs
│   ├── Aetheris.Web/       # Blazor WASM PWA — the all-platform client
│   ├── Aetheris.Cli/       # `aeth` console: init/add/get/ls + `aeth run --env ...`
│   └── Aetheris.Desktop/   # (plan only, W6) Avalonia + SSH.NET terminal
├── tests/Aetheris.Core.Tests/   # xunit: crypto vectors + vault round-trips
├── mockups/                # REAL CLICK DUMMIES — open mockups/index.html in a browser
├── agents/                 # per-workstream task cards for AI agents / solo work
└── PLAN-FOR-AGENTS.md      # orchestration plan: order, ownership, acceptance gates
```

## See it in 10 seconds (no SDK needed)

Open **`v2/mockups/index.html`** in any browser — click through the Web/PWA,
Mobile, Desktop (with SSH terminal) and CLI prototypes. These are the visual
spec every agent card references.

## Run the base (needs .NET 8 SDK)

```bash
cd v2
dotnet restore && dotnet build
dotnet test                                   # crypto + vault proof

# 1) create your encrypted vault + keys
dotnet run --project src/Aetheris.Cli -- init --vault keys.vault
AETHERIS_PASS=… dotnet run --project src/Aetheris.Cli -- add apikey \
    --vault keys.vault --provider openai --title "OpenAI prod"

# 2) start the LLM gateway (keys injected from the vault)
AETHERIS_GATEWAY_PASS=… dotnet run --project src/Aetheris.Gateway
# → OpenAI-compatible endpoint on http://127.0.0.1:7474/v1

# 3) point any tool at it
curl http://127.0.0.1:7474/v1/chat/completions \
  -H "X-Aetheris-Gateway-Token: dev" -H "Content-Type: application/json" \
  -d '{"model":"gpt","messages":[{"role":"user","content":"hello"}]}'

# 4) hub (sync) + web client
AETHERIS_TOKEN=dev-token dotnet run --project src/Aetheris.Hub      # :8080
dotnet run --project src/Aetheris.Web                               # PWA
```

## Status

Baseline skeleton: Core crypto + file vault are implemented and tested; Hub,
Gateway, Web, CLI are working baselines that the **agent workstreams W1–W6**
harden to product grade. Nothing here claims to be "done" — `agents/` says
exactly what remains, with acceptance gates.
