# Terminal

Aetheris Terminal is a Termius-grade SSH client built with xterm.js and the Rust russh library. It features vault-integrated connections, AI-powered command suggestions, session recording, collaborative sessions, and more.

## Features

### Vault-Integrated Connections
SSH connections are vault items — encrypted, synced, and autofillable. No `~/.ssh/config` needed. Keys never touch disk unencrypted.

### xterm.js Rendering
GPU-accelerated terminal with 256-color support, themes, split panes, search, and clickable links.

### AI-Powered Suggestions
Context-aware command suggestions based on current directory, history, and server type. Runs entirely locally — no cloud, no data leaves your device.

### Session Management
- Tabbed sessions
- Split panes (vertical, horizontal, grid)
- Session recording and playback
- Collaborative sessions (pair programming)
- Session resurrection after crash/disconnect

### Smart Snippets
Variable snippets like `ssh {{user}}@{{host}} -p {{port}}`. Contextual snippets per server type. Team snippet library. Snippet marketplace.

### Integrated SFTP
Visual file browser alongside the terminal. Drag-and-drop upload/download. Directory sync. Built-in editor.

### Port Forwarding
Visual port map showing all forwards at a glance. One-click forwarding. Multi-hop forwarding. Health indicators.

### Connection Health
Real-time latency monitoring, uptime tracking, resource monitoring (CPU, RAM, disk). Alerts on issues.

## Quick Start

```bash
# Connect to a server
aeth ssh connect --host example.com --user admin

# Connect with a specific key
aeth ssh connect --host example.com --user admin --key ~/.ssh/id_ed25519

# List saved connections
aeth ssh list

# Start a collaborative session
aeth ssh share --session-id abc123
```

## Key Bindings

| Key | Action |
|-----|--------|
| `Ctrl+Shift+T` | New tab |
| `Ctrl+Shift+D` | Split vertically |
| `Ctrl+Shift+W` | Close tab/pane |
| `Ctrl+Shift+F` | Search |
| `Ctrl+Shift+C` | Copy |
| `Ctrl+Shift+V` | Paste |
| `Ctrl+Shift+P` | Command palette |
| `Ctrl+Shift+,` | Settings |

## Themes

50+ built-in themes including popular terminal themes (Dracula, Nord, Solarized, Gruvbox, etc.). Per-connection themes. Theme marketplace for community themes.
