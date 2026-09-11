# Aetheris.Desktop — native desktop & SSH terminal (W6)

**Status: intentionally empty.** This is the commercial layer, built AFTER the
base (W1–W5) is green. Do not scaffold it before its card gate.

## What it will be

- **Avalonia UI 11** desktop app (single C# codebase → Windows/macOS/Linux),
  reusing `Aetheris.Core` directly (no FFI, no JS bridge).
- **SSH terminal**: SSH.NET client + an embedded xterm-equivalent rendering
  pane; vault keys are decrypted in memory, handed to SSH.NET, zeroized after.
- **LLM gateway pane**: embeds the same budget/log surfaces as the PWA.
- Auto-updates (Squirrel/Velopack), signed installers.

## Design spec

Click dummy: `../../mockups/desktop.html`. Task card: `../../agents/W6-desktop-ssh.md`.

## Why Avalonia (locked decision)

Tauri/Electron would reintroduce a second UI language; MAUI is weak on Linux.
Avalonia keeps the whole product in C# — one language, one mental model,
fastest path for a solo builder.
