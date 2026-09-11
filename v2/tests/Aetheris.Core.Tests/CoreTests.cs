using Aetheris.Core.Crypto;
using Aetheris.Core.Vault;
using Xunit;

namespace Aetheris.Core.Tests;

public class CryptoTests
{
    private static byte[] FastKey(string password, byte[] salt) =>
        // reduced Argon2 params so CI stays fast; production defaults stay in KdfParams
        CryptoEngine.DeriveMasterKey(password, salt, iterations: 2, memoryKiB: 4096, parallelism: 1);

    [Fact]
    public void Argon2_deterministic_and_salt_sensitive()
    {
        var salt = CryptoEngine.RandomBytes(32);
        var k1 = FastKey("correct horse", salt);
        var k2 = FastKey("correct horse", salt);
        var k3 = FastKey("correct horse", CryptoEngine.RandomBytes(32));
        Assert.Equal(k1, k2);
        Assert.NotEqual(k1, k3);
        Assert.Equal(32, k1.Length);
    }

    [Fact]
    public void Encrypt_decrypt_roundtrip_random_payload()
    {
        var key = CryptoEngine.RandomBytes(32);
        var data = CryptoEngine.RandomBytes(1024);
        var ct = CryptoEngine.Encrypt(key, data, out var nonce);
        Assert.Equal(24, nonce.Length);
        Assert.Equal(data, CryptoEngine.Decrypt(key, nonce, ct));
    }

    [Fact]
    public void Tampered_ciphertext_fails_integrity()
    {
        var key = CryptoEngine.RandomBytes(32);
        var ct = CryptoEngine.Encrypt(key, "secret"u8.ToArray(), out var nonce);
        ct[0] ^= 0x01; // flip one bit
        Assert.Throws<AetherisException>(() => CryptoEngine.Decrypt(key, nonce, ct));
    }

    [Fact]
    public void Subkeys_are_domain_separated()
    {
        var master = CryptoEngine.RandomBytes(32);
        Assert.NotEqual(
            KeyHierarchy.Derive(master, SubKey.Vault),
            KeyHierarchy.Derive(master, SubKey.Sync));
    }
}

public class VaultStoreTests : IDisposable
{
    private readonly string _path = Path.Combine(Path.GetTempPath(), $"aeth-test-{Guid.NewGuid():N}.vault");

    [Fact]
    public void Full_vault_roundtrip_with_password_items()
    {
        var store = VaultStore.Create(_path, "pw-123");
        var added = store.Add(ItemType.Password, "GitHub",
            new PasswordPayload("me", "hunter2", new List<string> { "https://github.com" }, null, null));

        store.Lock();
        Assert.False(store.IsUnlocked);

        using var reopened = VaultStore.Unlock(_path, "pw-123");
        var item = reopenend_find(reopened, "GitHub");
        Assert.Equal(added.Id, item.Id);
        Assert.Equal("hunter2", item.Payload<PasswordPayload>().Password);
        Assert.Contains("api key", addAndRead(reopened));
    }

    private static VaultItem reopenend_find(VaultStore s, string title) =>
        s.Find(title) ?? throw new Xunit.Sdk.XunitException("item missing");

    private static IEnumerable<string> addAndRead(VaultStore s)
    {
        var item = s.Add(ItemType.ApiKey, "OpenAI",
            new ApiKeyPayload("openai", "sk-test-fake-0000", new List<string> { "chat" }, null, null));
        yield return "api key " + item.Payload<ApiKeyPayload>().Provider;
        s.Remove(item.Id);
    }

    [Fact]
    public void Wrong_password_fails_unlock_via_aead()
    {
        var store = VaultStore.Create(_path, "right");
        store.Add(ItemType.Note, "n", new NotePayload("body"));
        store.Lock();
        Assert.Throws<AetherisException>(() => VaultStore.Unlock(_path, "wrong"));
    }

    [Fact]
    public void Vault_file_contains_no_plaintext()
    {
        var store = VaultStore.Create(_path, "pw-123");
        store.Add(ItemType.ApiKey, "Stripe",
            new ApiKeyPayload("stripe", "sk-test-fake-9999", new List<string>(), null, null));
        store.Lock();

        var raw = File.ReadAllText(_path);
        Assert.DoesNotContain("sk-test-fake-9999", raw);
        Assert.DoesNotContain("Stripe", raw); // titles are inside the envelope too
    }

    public void Dispose()
    {
        if (File.Exists(_path)) File.Delete(_path);
    }
}
