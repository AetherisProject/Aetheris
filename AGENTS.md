# AGENTS.md: Blueprint for the Absolute Omni-Agent (Aetheris Engine)

## 1. Core Vision & Target Matrix
Build a unified, ultra-low-overhead terminal client (`aeth`) and native visual application wrapper that bridges the gap between **Termius-grade UI simplicity** and **Moshi-grade performance engineering**.

It eliminates the "re-setup" nightmare by serving as an automatic deployment hub, client-side zero-knowledge vault, and hardware-aware AI routing engine.

### Cross-Platform Matrix
*   **Operating Systems:** Linux (Native ELF), Windows (PowerShell/Native PE), macOS (Darwin Universal), iOS, Android.
*   **Browsers:** Chrome, Firefox, Safari, Edge, Brave (via an automated WebAuthn/MCP extension loop).
*   **Terminal Interface:** Pure `ratatui` (Rust) or `bubbletea` (Go) for terminal modes; hardware-accelerated Canvas for mobile/browser wrappers.

---

## 2. Engineering Architecture & Feature Specifications

### Module A: The Omni-Vault (Password, SSH, and API Key Storage)
*   **Cryptographic Primitive:** Client-side zero-knowledge architecture using `XChaCha20-Poly1305` and `Argon2id` for local data derivation. Master passwords never leave the CPU registers in plaintext.
*   **Automated Lifecycle Management:**
    *   **Auto-Rotation:** Implements programmatic background API key rotation loops for platforms like NVIDIA NGC, OpenAI, Hugging Face, AWS, and Google Cloud.
    *   **Dynamic Injection:** When launching an SSH terminal session, the client temporarily maps environment variables directly into the process memory loop, keeping secrets safe from disk logging or `history` buffers.
    *   **Browser Autofill:** Interacts via a secure WebSockets loop to a unified browser extension framework.

### Module B: The Headless Infrastructure Engineer & Script Vault
*   **The Problem It Solves:** Re-setting up AI engines (like llama.cpp) manually on fresh machines is tedious and slow.
*   **The Feature:**
    *   **One-Click Bootstrap:** Stores custom scripts (e.g., your optimized PowerShell and Bash AI setup setups) inside an internal vault.
    *   **Zero-Interaction Deploy:** When establishing an SSH connection to a fresh system (WSL, bare-metal Linux, or a remote server), it inspects the destination architecture and automatically triggers the setup process natively.

### Module C: Integrated Local AI Benchmarker
*   **Auto-Probing:** Measures available VRAM capacity and computes execution paths natively via CUDA/Vulkan.
*   **Live Metrics Evaluation:** Runs localized testing layers (`llama-bench`) upon instance setup to determine token throughput (Tokens per Second). It optimizes model routing parameters automatically (e.g., forcing a 3B model if VRAM is less than 6GB to prevent system lag).

---

## 3. Usage

Run with: `cargo run`

---
## 4. Progress Tracking (Auto-Updated)

**State file:** `.memory/progress.json`
**Generated file:** `CONTEXT.md`
**Update script:** `scripts/update-context.sh`

Run `bash scripts/update-context.sh` to regenerate CONTEXT.md from `.memory/progress.json`.

**Current Phase:** Rust Design System Export (Phase 1/6) — In Progress
**Platform Plan:** CLI (ratatui) → Desktop (Tauri 2 + React) → Mobile (Flutter) → Web (React) → Browser Extension (WebExtension MV3 + WASM)
**Source of truth:** `src/design/` (Rust design module)
**Design docs:** `docs/design-system.html` / `docs/design-system.md` / `docs/styles.css`
