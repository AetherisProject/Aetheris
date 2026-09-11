# W3 — LLM gateway ⭐ (the wedge product)

- **Owns:** `src/Aetheris.Gateway/**`
- **Depends on:** W1 (vault read) · **Effort:** 3–5 days

## Mission

The reason people install Aetheris: one OpenAI-compatible local endpoint,
keys injected from the encrypted vault at request time, model aliases with
fallback chains, token budgets, redaction-first logs. Baseline proxies
OpenAI/Anthropic/Ollama non-streaming — harden it to daily-driver grade.

## Tasks

- [ ] 3.1 **SSE streaming** end-to-end (`stream:true`): forward upstream SSE,
      mapping Anthropic events → OpenAI deltas; keep usage accounting.
- [ ] 3.2 Hot keystore: watch `keys.vault` mtime → re-unlock on change
      (AETHERIS_GATEWAY_PASS in env only; never persisted). + tests.
- [ ] 3.3 Provider health checks every 15 min (cheap `models` call); expose
      `GET /v1/gateway/health` with per-provider status/latency.
- [ ] 3.4 Budgets v2: per-alias per-day token caps (exists) + USD cost
      estimation from a small price table; alert at 80%.
- [ ] 3.5 Log redaction test: run a chat call, then assert the literal API
      key string appears **nowhere** in logs, budgets file, or responses.
- [ ] 3.6 OpenAI-compat completeness: `/v1/models` from upstreams merged with
      aliases; `max_tokens` passthrough; error bodies shaped like OpenAI's.

## Acceptance gate

```bash
# with keys.vault containing an openai item + AETHERIS_GATEWAY_PASS set:
curl -s 127.0.0.1:7474/v1/chat/completions -H 'X-Aetheris-Gateway-Token: dev' \
  -H 'Content-Type: application/json' \
  -d '{"model":"gpt","messages":[{"role":"user","content":"say hi"}]}' | jq .   # 200
# set BudgetTokensPerDay.gpt=50 → next call → HTTP 402 budget_exceeded
# grep the log directory for the literal key → 0 matches, or the card FAILS
```

## Not yours

The web dashboard (W4 consumes your API), key rotation (proactive engine,
later), hosted/multi-tenant mode (never for v2 — local/self-hosted only).
