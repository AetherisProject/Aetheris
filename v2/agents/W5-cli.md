# W5 — CLI (`aeth`)

- **Owns:** `src/Aetheris.Cli/**`
- **Depends on:** W1 · **Effort:** 2–3 days

## Mission

The scripted, hackable surface. Baseline dispatcher exists; make it the
tool from `mockups/cli.html`: Spectre-quality output, all item types,
clipboard with auto-clear, JSON mode, sync push/pull, `aeth run` polished.

## Tasks

- [ ] 5.1 Migrate to `Spectre.Console.Cli` (commands + help + tables), keep
      the binary name `aeth` and the current flags working.
- [ ] 5.2 Full CRUD for all 7 item types + `search`, `edit <id>`, tags,
      `--json` output mode (secrets redacted unless `--reveal`).
- [ ] 5.3 `get <id> --copy`: clipboard copy with 45s auto-clear
      (TextCopy or P/Invoke helper; Linux/X11+Wayland note).
- [ ] 5.4 `sync push|pull|status` against Hub (`X-Aetheris-Token` env),
      encryption stays client-side (Core).
- [ ] 5.5 `aeth run --env NAME=<ref> …` multi-var support, `--vault` on every
      command, exit codes mirror the child's.
- [ ] 5.6 `aeth gateway models|budgets|logs|health` hitting W3's API.
- [ ] 5.7 `.env` compatibility bridge (ported from KEY-BITCHER's
      `sync-secrets`): `aeth env-export --to .env [--only prefix1,prefix2]
      [--secure]` materializes vault API keys into an `.env` file with 0600
      perms, an auto-generated restore warning header, and a `secure` command
      that verifies ownership/permissions. Marked in docs as the *legacy*
      escape hatch — gateway injection (W3) and `aeth run` are the defaults.

## Acceptance gate

```bash
# every step printed in mockups/cli.html executes for real:
aeth init --vault t.vault && aeth add apikey --vault t.vault --provider openai --title k
aeth ls --vault t.vault
aeth run --vault t.vault --env K=k -- sh -c 'test -n "$K" && echo injected'
# → prints "injected", never prints the key itself
AETHERIS_TOKEN=dev aeth sync push --vault t.vault  # → generation 1 at the hub
```

## Not yours

TUI (`aeth tui` with full-screen layout) — backlog after W6.
