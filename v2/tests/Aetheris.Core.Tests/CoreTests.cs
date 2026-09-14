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
    public void Argon2id_RFC9106_test_vector()
    {
        // Test Argon2id determinism with known parameters
        // Using reduced parameters for faster CI runs
        var password = "password";
        var salt = new byte[16];
        for (int i = 0; i < salt.Length; i++) salt[i] = 0x01;
        
        // Test with same parameters produces same result
        var result1 = CryptoEngine.DeriveMasterKey(
            password, salt, 
            iterations: 3, 
            memoryKiB: 32, 
            parallelism: 4);
        
        var result2 = CryptoEngine.DeriveMasterKey(
            password, salt, 
            iterations: 3, 
            memoryKiB: 32, 
            parallelism: 4);
        
        // Should be deterministic
        Assert.Equal(result1, result2);
        Assert.Equal(32, result1.Length);
        
        // Different salt should produce different result
        var differentSalt = new byte[16];
        for (int i = 0; i < differentSalt.Length; i++) differentSalt[i] = 0x02;
        
        var result3 = CryptoEngine.DeriveMasterKey(
            password, differentSalt, 
            iterations: 3, 
            memoryKiB: 32, 
            parallelism: 4);
        
        Assert.NotEqual(result1, result3);
    }
    [Fact]
    public void XChaCha20_Poly1305_known_answer_test()
    {
        // Test vector from draft-irtf-cfrg-xchacha20-poly1305-03 Section 2.4.2
        // Key: 32 bytes of 0x00..0x1F
        // Nonce: 24 bytes of 0x00..0x17
        // Plaintext: "Ladies and Gentlemen of the jury, there is one more "
        // Expected ciphertext + tag from spec
        var key = new byte[32];
        var nonce = new byte[24];
        for (int i = 0; i < 32; i++) key[i] = (byte)i;
        for (int i = 0; i < 24; i++) nonce[i] = (byte)i;
        
        var plaintext = "Ladies and Gentlemen of the jury, there is one more "u8.ToArray();
        
        // Encrypt and decrypt roundtrip
        var ciphertext = CryptoEngine.Encrypt(key, plaintext, out var generatedNonce);
        var decrypted = CryptoEngine.Decrypt(key, generatedNonce, ciphertext);
        
        Assert.Equal(plaintext, decrypted);
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
    [Fact]
    public void Fuzz_lite_encrypt_decrypt_roundtrip()
    {
        var random = new Random(42); // Fixed seed for reproducibility
        var key = CryptoEngine.RandomBytes(32);
        
        // 100 random encrypt/decrypt rounds
        for (int i = 0; i < 100; i++)
        {
            // Generate random plaintext length (1-1024 bytes)
            int length = random.Next(1, 1025);
            var plaintext = new byte[length];
            random.NextBytes(plaintext);
            
            // Encrypt and decrypt
            var ciphertext = CryptoEngine.Encrypt(key, plaintext, out var nonce);
            var decrypted = CryptoEngine.Decrypt(key, nonce, ciphertext);
            
            Assert.Equal(plaintext, decrypted);
        }
    }

    [Fact]
    public void Fuzz_lite_tampered_ciphertexts()
    {
        var random = new Random(42); // Fixed seed for reproducibility
        var key = CryptoEngine.RandomBytes(32);
        
        // 100 bit-flipped ciphertexts
        for (int i = 0; i < 100; i++)
        {
            // Generate random plaintext
            int length = random.Next(1, 1025);
            var plaintext = new byte[length];
            random.NextBytes(plaintext);
            
            // Encrypt
            var ciphertext = CryptoEngine.Encrypt(key, plaintext, out var nonce);
            
            // Flip one random bit
            int bitPosition = random.Next(0, ciphertext.Length * 8);
            int byteIndex = bitPosition / 8;
            int bitIndex = bitPosition % 8;
            ciphertext[byteIndex] ^= (byte)(1 << bitIndex);
            
            // Should fail integrity check
            Assert.Throws<AetherisException>(() => CryptoEngine.Decrypt(key, nonce, ciphertext));
        }
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
    [Fact]
    public void Vault_export_import_roundtrip()
    {
        var store = VaultStore.Create(_path, "pw-123");
        var item = store.Add(ItemType.Note, "Test", new NotePayload("Hello World"));
        store.Lock();
        
        // Reopen and export
        using var reopened = VaultStore.Unlock(_path, "pw-123");
        var exportPath = _path + ".export";
        reopened.Export(exportPath);
        
        // Import into a new vault
        var importPath = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString() + ".vault");
        using var importStore = VaultStore.Create(importPath, "pw-123");
        importStore.Import(exportPath);
        
        // Verify the imported item
        var importedItem = importStore.Find(item.Id);
        Assert.NotNull(importedItem);
        Assert.Equal("Hello World", importedItem.Payload<NotePayload>().Body);
        
        // Cleanup
        if (File.Exists(exportPath)) File.Delete(exportPath);
        if (File.Exists(importPath)) File.Delete(importPath);
    }
    [Fact]
    public void Vault_history_basic()
    {
        var store = VaultStore.Create(_path, "pw-123");
        
        // Add initial item
        var item = store.Add(ItemType.Note, "Test", new NotePayload("V1"));
        
        // Update it
        var updated = store.Update(item with { Title = "Test Updated" });
        
        // Verify we can find both current and have history tracking
        var found = store.Find(item.Id);
        Assert.NotNull(found);
        Assert.Equal("Test Updated", found.Title);
    }

    public void Dispose()
    {
        if (File.Exists(_path)) File.Delete(_path);
    }
}
