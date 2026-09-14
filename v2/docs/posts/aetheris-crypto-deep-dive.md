# I built a local LLM gateway that keeps keys out of .env (C#/.NET 8, OSS)

> **Status:** Draft for dev.to / HN
> **Author:** Aetheris Team
> **Date:** September 2026
> **Tags:** #csharp #dotnet #llm #security #opensource

---

## The Problem: API Keys Everywhere

I have a problem. Actually, we all have this problem.

Every time I spin up a new LLM project, I end up with API keys scattered across:
- `.env` files in multiple repositories
- Docker compose files with `OPENAI_API_KEY=sk-...` 
- CI/CD secrets that need constant rotation
- Local development configs that accidentally get committed

And the worst part? These keys have access to my credit card. One leaked key can mean thousands in unexpected charges.

I'm not alone. The r/selfhosted and r/LocalLLaMA subreddits are full of posts like:
- "How do you manage your API keys?"
- "My key got leaked, what do I do?"
- "Is there a better way than .env files?"

There had to be a better way.

---

## The Solution: Aetheris

So I built **Aetheris** - a local LLM gateway with an encrypted vault that keeps your keys secure and accessible across all your devices.

### Core Principles

1. **Your keys, your infrastructure** - Everything runs locally or on your own servers
2. **Zero knowledge** - Even the sync service can't read your keys
3. **Open source** - MIT licensed, auditable, extensible
4. **Multi-platform** - Works on Windows, Linux, macOS, and even mobile via PWA

### The Stack

- **Language:** C#/.NET 8 (because I love the ecosystem and performance)
- **Crypto:** BouncyCastle for Argon2id + XChaCha20-Poly1305
- **Frontend:** PWA with HTML/CSS/JS (installable on any platform)
- **Desktop:** Avalonia for cross-platform native apps
- **Sync:** Zero-knowledge architecture with hosted hub

---

## How It Works

### 1. The Vault

At the heart of Aetheris is an encrypted vault. When you run `aeth init`, you create a new vault with a master password:

```bash
# Initialize a new vault
aeth init
# Enter master password: ********
# Confirm master password: ********
```

Your master password is used to derive an encryption key using **Argon2id** - a memory-hard key derivation function that's resistant to GPU/ASIC attacks.

### 2. Adding Keys

Store your API keys securely:

```bash
# Add an OpenAI key
aeth add openai-key
# Enter value: sk-...
# Confirm value: sk-...

# Add an Anthropic key  
aeth add anthropic-key
# Enter value: sk-ant-...
```

Each key is encrypted with **XChaCha20-Poly1305** - a modern, secure authenticated encryption scheme. The encrypted data is stored in a local file that's useless without your master password.

### 3. The Gateway

Start the local gateway:

```bash
aeth gateway
# Gateway listening on http://localhost:8080
```

The gateway acts as a proxy between your applications and LLM providers. When you make a request:

```bash
curl http://localhost:8080/v1/chat/completions \
  -H "Authorization: Bearer openai-key" \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4", "messages": [{"role": "user", "content": "Hello"}]}'
```

The gateway:
1. Validates your request
2. Looks up the encrypted key for "openai-key"
3. Decrypts it using your session
4. Forwards the request to OpenAI
5. Returns the response

Your actual API key never leaves the gateway process.

### 4. Using with Your Apps

The magic happens with `aeth run`:

```bash
# Run any command with keys injected
aeth run -- curl http://localhost:8080/v1/chat/completions \
  -H "Authorization: Bearer openai-key" \
  -d '{"model": "gpt-4", "messages": [{"role": "user", "content": "Hello"}]}'
```

Or configure your applications to use the gateway directly.

---

## Security Deep Dive

### Why Not Just Use .env?

The `.env` approach has several problems:

1. **Accidental commits** - Even with `.gitignore`, it happens
2. **No encryption** - Keys are stored in plaintext
3. **No access control** - Anyone with file access has all keys
4. **No audit trail** - Can't track who used what and when

### Aetheris Security Model

```
┌─────────────────────────────────────────────────────────┐
│                    Your Applications                         │
└─────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────┐
│                    Aetheris Gateway                          │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  Request comes in: http://localhost:8080/v1/...      │  │
│  │  - Validates authentication                         │  │
│  │  - Looks up key reference                            │  │
│  │  - Retrieves encrypted key from vault                │  │
│  │  - Decrypts key using session                        │  │
│  │  - Forwards request to provider with real key        │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────┐
│                    LLM Provider (OpenAI, etc.)               │
└─────────────────────────────────────────────────────────┘
```

### Cryptographic Primitives

#### Key Derivation: Argon2id

```csharp
// Simplified key derivation
var salt = GenerateRandomSalt(16);
var derivedKey = Argon2id.Hash(
    password: masterPassword,
    salt: salt,
    iterations: 3,
    memorySize: 65536, // 64MB
    parallelism: 4,
    hashLength: 32
);
```

Parameters chosen to be resistant to both GPU and side-channel attacks while remaining usable on most hardware.

#### Encryption: XChaCha20-Poly1305

```csharp
// Encrypt a key
var nonce = GenerateRandomNonce(24);
var ciphertext = XChaCha20Poly1305.Encrypt(
    plaintext: apiKey,
    key: derivedKey,
    nonce: nonce
);
// Store: nonce + ciphertext + tag
```

XChaCha20-Poly1305 provides:
- 256-bit security level
- Authenticated encryption (confidentiality + integrity)
- Resistance to nonce reuse (unlike AES-GCM)
- Fast on modern CPUs

### Threat Model

Aetheris protects against:
- ✅ Local file access (keys are encrypted)
- ✅ Accidental commits (vault file is binary, not text)
- ✅ Memory scraping (keys are zeroed after use)
- ✅ Network interception (all communication is local)

Aetheris does NOT protect against:
- ❌ Keyloggers on your machine
- ❌ Physical access to unlocked device
- ❌ Malware with root access
- ❌ Compromised LLM provider

---

## Budget Tracking

One of the most requested features - cost control:

```bash
# Set a budget for OpenAI
aeth budget set openai-key --monthly 100

# Check current usage
aeth budget check openai-key
# OpenAI: $45.23 / $100.00 (45.23%)

# Get alerts at 80%
aeth budget alert openai-key --threshold 80
```

The gateway tracks every request and calculates costs based on provider pricing. You get real-time feedback on your spending.

---

## Multi-Device Sync (Coming Soon)

The Sync tier (€5/month or €49/year) adds:

1. **Hosted Hub** - A service that syncs your vault across devices
2. **Zero-Knowledge** - Your keys are encrypted before leaving your device
3. **End-to-End Encryption** - Only you can decrypt your data
4. **Automatic Backups** - Never lose your keys

### How Sync Works

```
Device A                          Hub                          Device B
   │                              │                              │
   │  Encrypted Vault            │                              │
   │──────────────────────────>│                              │
   │  (only encrypted data)      │                              │
   │                            │                              │
   │                            │──────────────────────────>│
   │                            │  Encrypted Vault            │
   │                            │                              │
```

The hub never sees your plaintext keys. It only stores and forwards encrypted blobs.

---

## Desktop Pro (Coming Soon)

For power users who want a native experience:

- **SSH Terminal** - Full terminal with vault-key authentication
- **SFTP Support** - Secure file transfer using your encrypted credentials
- **Avalonia UI** - Native look and feel on Windows, macOS, Linux
- **Lifetime License** - €89 one-time payment

---

## Getting Started

### Prerequisites

- .NET 8 SDK
- Git

### Installation

```bash
# Clone the repo
git clone https://github.com/Aetheris/Aetheris.git
cd Aetheris

# Build
dotnet build

# Install CLI
dotnet tool install --global --from-path ./src/Aetheris.Cli

# Initialize
aeth init
```

### Quick Demo

```bash
# Add a test key
aeth add test-key --value "test-value"

# Start gateway
aeth gateway &

# Test it
curl http://localhost:8080/api/keys/test-key \
  -H "Authorization: Bearer test-key"
```

---

## Roadmap

| Feature | Status | ETA |
|---------|--------|-----|
| Core Gateway | ✅ Done | Now |
| CLI Tool | ✅ Done | Now |
| PWA Client | ✅ Done | Now |
| Budget Tracking | ✅ Done | Now |
| Sync Tier | 🚧 Building | Q4 2026 |
| Desktop Pro | 🚧 Building | Q4 2026 |
| Compliance Tier | 📋 Planned | 2027 |

---

## Contributing

Aetheris is open source under the MIT license. Contributions are welcome!

- **Code:** Check out the [GitHub repo](https://github.com/Aetheris/Aetheris)
- **Issues:** Report bugs or request features
- **Discussions:** Join the conversation
- **Stars:** ⭐ if you find it useful

---

## Alternatives

| Tool | Self-Hosted | Encryption | Multi-Device | Open Source |
|------|-------------|------------|--------------|--------------|
| Aetheris | ✅ Yes | ✅ Yes | ✅ (Sync) | ✅ Yes |
| Doppler | ❌ No | ✅ Yes | ✅ Yes | ❌ No |
| Infisical | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| Vault by HashiCorp | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |
| AWS Secrets Manager | ❌ No | ✅ Yes | ✅ Yes | ❌ No |

Aetheris fills a unique niche: **self-hosted, encrypted, multi-device, open source, with LLM-specific features**.

---

## Conclusion

I built Aetheris because I was tired of the security risks and inconvenience of managing LLM API keys. The response from the community has been incredible, and I'm excited to see where this goes.

**Your LLM keys never live in .env again.**

Try it out: [https://github.com/Aetheris/Aetheris](https://github.com/Aetheris/Aetheris)

Join the waitlist for Sync tier: [https://aetheris.dev/#waitlist](https://aetheris.dev/#waitlist)

---

> **Discussion:** What's your biggest pain point with API key management? Let me know in the comments!

> **Follow:** [@AetherisDev](https://twitter.com/AetherisDev) for updates

> **License:** MIT
