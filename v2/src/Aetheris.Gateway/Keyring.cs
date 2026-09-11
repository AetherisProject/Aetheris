using Aetheris.Core.Vault;

namespace Aetheris.Gateway;

/// <summary>Resolves provider API keys at request time from the encrypted vault.</summary>
public interface IKeyring
{
    string? GetKey(string provider);
}

/// <summary>Unlocks a VaultStore (same file the CLI manages) with AETHERIS_GATEWAY_PASS.</summary>
public sealed class VaultKeyring : IKeyring, IDisposable
{
    private readonly VaultStore _vault;

    private VaultKeyring(VaultStore vault) => _vault = vault;

    public static VaultKeyring? TryOpen(string path, string? passphrase, ILogger logger)
    {
        if (!File.Exists(path))
        {
            logger.LogWarning("keystore {Path} missing — gateway runs without key-backed providers", path);
            return null;
        }
        if (string.IsNullOrEmpty(passphrase))
        {
            logger.LogWarning("AETHERIS_GATEWAY_PASS not set — keystore stays locked");
            return null;
        }
        try
        {
            return new VaultKeyring(VaultStore.Unlock(path, passphrase));
        }
        catch (Exception ex)
        {
            logger.LogError("keystore unlock failed: {Message} (no key material logged)", ex.Message);
            return null;
        }
    }

    public string? GetKey(string provider) => _vault.Items
        .Where(i => i.Type == ItemType.ApiKey)
        .Select(i => i.Payload<ApiKeyPayload>())
        .FirstOrDefault(k => k.Provider.Equals(provider, StringComparison.OrdinalIgnoreCase))
        ?.Key;

    public void Dispose() => _vault.Dispose();
}

/// <summary>Empty keyring: local providers (ollama) still work; hosted ones return 424.</summary>
public sealed class EmptyKeyring : IKeyring
{
    public string? GetKey(string provider) => null;
}
