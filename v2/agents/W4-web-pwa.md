# W4 — All-platform PWA client

- **Owns:** `src/Aetheris.Web/**`, `mockups/*` (visual deltas only)
- **Depends on:** W1, W2, W3 · **Effort:** 5–8 days · **Biggest card — split by screen if two agents work it**

## Mission

The client that IS "all platforms": one Blazor WASM PWA, installable on
desktop and mobile. Screens exist as click dummies (`mockups/web.html`,
`mockups/mobile.html`) — production Razor must match those tokens/layouts.

## Tasks

- [ ] 4.1 Real unlock: master password → Argon2id **in the browser**
      (libsodium.js via JS interop for XChaCha20-Poly1305; .NET WASM Argon2
      acceptable at reduced params with a perf note). Never send the password
      anywhere.
- [ ] 4.2 Vault screens: grid, search, add/edit sheets, per-type payload forms
      (Password/ApiKey/Note/Card/Identity), TOTP display (RFC 6238 in JS).
- [ ] 4.3 Sync: GET/PUT encrypted blob against Hub (decrypt → edit →
      re-encrypt → PUT with expectedGeneration; handle 409 = pull-merge-push).
      Offline queue in localStorage (encrypted), flush on reconnect.
- [ ] 4.4 Gateway dashboard page: budgets bars, alias table, redacted log
      list, health badges (calls `/v1/gateway/*` from config).
- [ ] 4.5 PWA hardening: service worker v2 (offline app shell), install
      prompt, auto-lock timer (5 min default) + lock on tab hidden.
- [ ] 4.6 Mobile layout pass: bottom tab bar per `mockups/mobile.html`,
      safe-area insets, touch targets ≥44px.

## Acceptance gate

```bash
dotnet run --project src/Aetheris.Web
# 1) Lighthouse PWA installable on desktop + phone emulator
# 2) e2e: unlock → add password → push → NEW INCOGNITO WINDOW → unlock → item present
# 3) devtools network tab: every request body is base64 ciphertext; no plaintext leaves the page
# 4) kill the hub for 1 min → edits queue → hub back → generation advances, no data loss
```

## Not yours

Native biometrics (wrap later via native shell), browser extension autofill,
the desktop app (W6).
