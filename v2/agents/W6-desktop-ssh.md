# W6 — Desktop app with SSH terminal (commercial layer)

- **Owns:** `src/Aetheris.Desktop/**`
- **Depends on:** W1, W4 (design language settled) · **Effort:** 2–3 weeks
- **Do NOT start before W1–W5 gates are green.** This is the sellable product.

## Mission

Avalonia 11 desktop app = the "Termius with a brain": vault view, embedded SSH
terminal with keys from the vault, SFTP pane, gateway dashboard. Spec:
`mockups/desktop.html`. Stack: Avalonia 11 + SSH.NET + Aetheris.Core (direct
reference — no FFI, no JS).

## Tasks

- [ ] 6.1 Scaffold: Avalonia MVVM app shell with the mockup's titlebar/tabs;
      reuse the design tokens （深 void `#090D16` etc.) as Avalonia resources.
- [ ] 6.2 Vault tab: bind to Core `VaultStore`; unlock screen; item list/detail;
      clipboard actions with auto-clear.
- [ ] 6.3 SSH tab: connection picker from `SshConnection` items; SSH.NET
      `PrivateKeyFile` from an in-memory stream of the **decrypted** key
      (never written to disk); terminal emulation via a maintained xterm
      control for Avalonia (evaluate: `AvaloniaTerminal`-style OSS controls —
      pick one, document choice).
- [ ] 6.4 SFTP tab: two-pane file browser over the same SSH session.
- [ ] 6.5 Gateway tab: consume W3 `/v1/gateway/*` endpoints.
- [ ] 6.6 Key lifecycle proof: disconnect → zeroized; app log/assert test that
      no private key bytes remain in any written file.
- [ ] 6.7 Packaging: signed installers (win/mac/linux) + auto-update
      (Velopack). Release workflow in `.github/workflows`.

## Acceptance gate

```bash
# docker run --rm -p 2222:22 linuxserver/openssh-server (test target)
# 1) add SshKey + SshConnection to vault → connect from the app → interactive shell works
# 2) lsof/handle check + log scan: private key never on disk in plaintext
# 3) close vault mid-session → session terminates, memory buffers zeroized
# 4) installers build in CI for all three OSes
```

## Not yours

Mobile SSH, session recording/sharing, team features — post-launch backlog.
