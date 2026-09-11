using System.Text.Json;
using Aetheris.Core.Crypto;

namespace Aetheris.Core.Vault;

/// <summary>
/// Encrypted file vault. On-disk format (JSON):
/// { version, kdf:{iterations,memoryKiB,parallelism,salt}, generation,
///   items:[{ id, type, blob:{ nonce, data } }] }
/// <b>Everything beyond id/type is inside the XChaCha20-Poly1305 envelope</b> —
/// titles, tags and payloads never touch disk in plaintext.
/// </summary>
public sealed class VaultStore : IDisposable
{
    private const int FileVersion = 1;

    private readonly string _path;
    private byte[]? _vaultKey;
    private KdfDto _kdf = new();
    private List<VaultItem> _items = new();

    public int Generation { get; private set; }
    public bool IsUnlocked => _vaultKey is not null;

    private VaultStore(string path) => _path = path;

    // ---------- lifecycle ----------

    public static VaultStore Create(string path, string masterPassword)
    {
        if (File.Exists(path))
            throw new AetherisException($"Vault already exists: {path}");

        var store = new VaultStore(path)
        {
            _kdf = new KdfDto
            {
                Iterations = KdfParams.Iterations,
                MemoryKiB = KdfParams.MemoryKiB,
                Parallelism = KdfParams.Parallelism,
                Salt = Convert.ToBase64String(CryptoEngine.RandomBytes(KdfParams.SaltLength)),
            },
            Generation = 1,
        };
        store.UnlockWith(masterPassword);
        store.Save();
        return store;
    }

    public static VaultStore Unlock(string path, string masterPassword)
    {
        if (!File.Exists(path))
            throw new AetherisException($"Vault not found: {path}");

        var store = new VaultStore(path);
        var file = JsonSerializer.Deserialize<VaultFileDto>(File.ReadAllText(path), VaultJson.Options)
            ?? throw new AetherisException("Vault file is corrupt or empty.");
        if (file.Version != FileVersion)
            throw new AetherisException($"Unsupported vault version {file.Version}.");

        store._kdf = file.Kdf;
        store.Generation = file.Generation;
        store.UnlockWith(masterPassword);

        // Wrong password surfaces HERE as an integrity failure (AEAD), not as a bad read later.
        store._items = file.Items
            .Select(encrypted => store.DecryptItem(encrypted, masterKeyHint: null))
            .ToList();
        return store;
    }

    private void UnlockWith(string masterPassword)
    {
        var masterKey = CryptoEngine.DeriveMasterKey(
            masterPassword,
            Convert.FromBase64String(_kdf.Salt),
            _kdf.Iterations, _kdf.MemoryKiB, _kdf.Parallelism);
        _vaultKey = KeyHierarchy.Derive(masterKey, SubKey.Vault);
        CryptoEngine.Wipe(masterKey);
    }

    // ---------- operations (unlock required) ----------

    public IReadOnlyList<VaultItem> Items
    {
        get { RequireUnlocked(); return _items; }
    }

    public VaultItem Add<T>(ItemType type, string title, T payload, IEnumerable<string>? tags = null)
    {
        RequireUnlocked();
        var item = VaultItemFactory.Create(type, title, payload, tags);
        _items.Add(item);
        Touch();
        return item;
    }

    public VaultItem Update(VaultItem updated)
    {
        RequireUnlocked();
        var index = _items.FindIndex(i => i.Id == updated.Id);
        if (index < 0) throw new AetherisException($"Item {updated.Id} not found.");
        _items[index] = updated with { UpdatedAt = DateTimeOffset.UtcNow };
        Touch();
        return _items[index];
    }

    public bool Remove(string id)
    {
        RequireUnlocked();
        var removed = _items.RemoveAll(i => i.Id == id) > 0;
        if (removed) Touch();
        return removed;
    }

    public VaultItem? Find(string idOrTitle)
    {
        RequireUnlocked();
        return _items.FirstOrDefault(i =>
            i.Id.Equals(idOrTitle, StringComparison.OrdinalIgnoreCase) ||
            i.Title.Equals(idOrTitle, StringComparison.OrdinalIgnoreCase));
    }

    /// <summary>Decrypt-on-demand read of one item's payload; caller must not log the result.</summary>
    public T ReadPayload<T>(string idOrTitle) =>
        (Find(idOrTitle) ?? throw new AetherisException($"Item '{idOrTitle}' not found."))
        .Payload<T>();

    // ---------- persistence ----------

    public void Save()
    {
        RequireUnlocked();
        var file = new VaultFileDto
        {
            Version = FileVersion,
            Kdf = _kdf,
            Generation = Generation,
            Items = _items.Select(EncryptItem).ToList(),
        };
        var tmp = _path + ".tmp";
        File.WriteAllText(tmp, JsonSerializer.Serialize(file, VaultJson.Options));
        File.Move(tmp, _path, overwrite: true); // atomic-ish replace
    }

    public void Lock()
    {
        if (_vaultKey is not null) CryptoEngine.Wipe(_vaultKey);
        _vaultKey = null;
        _items = new List<VaultItem>();
    }

    public void Dispose() => Lock();

    // ---------- internals ----------

    private StoredItemDto EncryptItem(VaultItem item)
    {
        var plain = JsonSerializer.SerializeToUtf8Bytes(item, VaultJson.Options);
        var data = CryptoEngine.Encrypt(_vaultKey!, plain, out var nonce);
        CryptoEngine.Wipe(plain);
        return new StoredItemDto
        {
            Id = item.Id,
            Type = item.Type.ToString(),
            Blob = new EncBlobDto
            {
                Mode = "xchacha20poly1305",
                Nonce = Convert.ToBase64String(nonce),
                Data = Convert.ToBase64String(data),
            },
        };
    }

    private VaultItem DecryptItem(StoredItemDto stored, object? masterKeyHint)
    {
        var plain = CryptoEngine.Decrypt(
            _vaultKey!,
            Convert.FromBase64String(stored.Blob.Nonce),
            Convert.FromBase64String(stored.Blob.Data));
        try
        {
            return JsonSerializer.Deserialize<VaultItem>(plain, VaultJson.Options)
                ?? throw new AetherisException("Corrupt item in vault.");
        }
        finally
        {
            CryptoEngine.Wipe(plain);
        }
    }

    private void Touch()
    {
        Generation++;
        Save();
    }

    private void RequireUnlocked()
    {
        if (_vaultKey is null) throw new AetherisException("Vault is locked.");
    }

    // ---------- on-disk DTOs ----------

    private sealed class VaultFileDto
    {
        public int Version { get; set; }
        public KdfDto Kdf { get; set; } = new();
        public int Generation { get; set; }
        public List<StoredItemDto> Items { get; set; } = new();
    }

    private sealed class KdfDto
    {
        public int Iterations { get; set; }
        public int MemoryKiB { get; set; }
        public int Parallelism { get; set; }
        public string Salt { get; set; } = "";
    }

    private sealed class StoredItemDto
    {
        public string Id { get; set; } = "";
        public string Type { get; set; } = "";
        public EncBlobDto Blob { get; set; } = new();
    }

    private sealed class EncBlobDto
    {
        public string Mode { get; set; } = "xchacha20poly1305";
        public string Nonce { get; set; } = "";
        public string Data { get; set; } = "";
    }
}
