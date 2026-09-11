namespace Aetheris.Core.Crypto;

public enum SubKey
{
    Vault,
    Sync,
    Auth,
    Backup,
    Recovery,
}

/// <summary>
/// master key → domain-separated subkeys (DESIGN §5.3).
/// vault = item AEAD · sync = blob AEAD · auth = device sessions ·
/// backup = export AEAD · recovery = Shamir input (W-later).
/// </summary>
public static class KeyHierarchy
{
    public static byte[] Derive(byte[] masterKey, SubKey subKey) =>
        CryptoEngine.Hkdf(masterKey, $"aetheris/{subKey.ToString().ToLowerInvariant()}/v1");
}
