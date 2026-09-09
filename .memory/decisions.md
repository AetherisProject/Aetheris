# Decision: CLI Auto-Integration

## Context
The terminal must auto-integrate with vaults and SSH clients seamlessly.

## Decision
- **Use mock data** for CLI testing to avoid external dependencies.
- **Simulate decryption** with mock logic.
- **Test SSH client** with mocks.

## Implementation
- CLI auto-fetches vault items and decrypts API keys.
- SSH client mocks simulate connections and commands.

## Verification
- CLI logic verified with mock data.
- SSH client mocks tested successfully.

## Next Steps
- Fix CLI build setup for real deployment.
- Test real-world scenarios with actual vaults and SSH.