# API Keys

The API Key Manager handles all your API keys with provider-aware rotation, health monitoring, and auto-injection.

## Supported Providers

- OpenAI, Anthropic, Google, AWS, GitHub, GitLab, Azure
- NVIDIA, HuggingFace, Mistral, OpenRouter, Groq, Cohere, Stability
- And 15+ more — with a plugin system for custom providers

## Features

### Auto-Rotation
Rotate API keys before expiry. Provider-specific rotation logic. One-click manual rotation when auto isn't possible.

### Health Monitoring
Periodic ping to detect expired or revoked keys. Alerts on failures.

### Auto-Injection
Map keys to environment variables, config files, or API calls.

### Rotation Calendar
View upcoming rotations at a glance. Configurable policies per provider.

## Usage

```bash
# Add an API key
aeth apikey add --provider openai --key sk-xxx

# Rotate a key
aeth apikey rotate --provider openai

# Check health
aeth apikey health

# List keys
aeth apikey list
```

## Provider Configuration

Each provider defines:
- Key format (regex validation)
- Rotation API endpoint
- Health check endpoint
- Injection targets
- Scopes and permissions
