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
    private readonly Dictionary<string, List<StoredItemDto>> _itemHistory = new(); // id -> list of encrypted envelopes (max 10)
    private const int MaxHistoryPerItem = 10;

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
                Parallelism = KdfParams.DegreeOfParallelism,
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
        
        // Store initial version in history
        var encryptedItem = EncryptItem(item);
        var history = new List<StoredItemDto> { encryptedItem };
        _itemHistory[item.Id] = history;
        
        Touch();
        return item;
    }
    public VaultItem Update(VaultItem updated)
    {
        RequireUnlocked();
        var index = _items.FindIndex(i => i.Id == updated.Id);
        if (index < 0) throw new AetherisException($"Item {updated.Id} not found.");
        
        // Store current version in history before updating
        var currentItem = _items[index];
        var encryptedCurrent = EncryptItem(currentItem);
        
        if (!_itemHistory.TryGetValue(updated.Id, out var history))
        {
            history = new List<StoredItemDto>();
            _itemHistory[updated.Id] = history;
        }
        
        // Keep only last 10 versions
        if (history.Count >= MaxHistoryPerItem)
        {
            history.RemoveAt(0); // Remove oldest
        }
        history.Add(encryptedCurrent);
        
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
    public VaultItem? Restore(string id, int version)
    {
        RequireUnlocked();
        
        if (!_itemHistory.TryGetValue(id, out var history) || version >= history.Count || version < 0)
            return null;
        
        // Get the historical encrypted envelope
        var historicalEnvelope = history[version];
        
        // Decrypt the historical version
        var restoredItem = DecryptItem(historicalEnvelope, null);
        
        // Add to current items (this creates a new current version)
        _items.Add(restoredItem);
        Touch();
        
        return restoredItem;
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
    public void Export(string exportPath)
    {
        RequireUnlocked();
        
        // Derive backup subkey for export encryption
        var backupKey = KeyHierarchy.Derive(_vaultKey!, SubKey.Backup);
        
        try
        {
            // Create export DTO with all items re-encrypted under backup key
            var exportDto = new ExportDto
            {
                Version = FileVersion,
                ExportTimestamp = DateTimeOffset.UtcNow,
                Items = _items.Select(item => 
                {
                    var plain = JsonSerializer.SerializeToUtf8Bytes(item, VaultJson.Options);
                    var encryptedData = CryptoEngine.Encrypt(backupKey, plain, out var nonce);
                    CryptoEngine.Wipe(plain);
                    return new ExportItemDto
                    {
                        Id = item.Id,
                        Type = item.Type.ToString(),
                        Nonce = Convert.ToBase64String(nonce),
                        Data = Convert.ToBase64String(encryptedData),
                        Timestamp = item.UpdatedAt
                    };
                }).ToList()
            };
            
            var tmp = exportPath + ".tmp";
            File.WriteAllText(tmp, JsonSerializer.Serialize(exportDto, VaultJson.Options));
            File.Move(tmp, exportPath, overwrite: true);
        }
        finally
        {
            CryptoEngine.Wipe(backupKey);
        }
    }
    public void Import(string importPath)
    {
        RequireUnlocked();
        
        if (!File.Exists(importPath))
            throw new AetherisException($"Export file not found: {importPath}");
        
        // Derive backup subkey for import decryption
        var backupKey = KeyHierarchy.Derive(_vaultKey!, SubKey.Backup);
        
        try
        {
            var exportDto = JsonSerializer.Deserialize<ExportDto>(File.ReadAllText(importPath), VaultJson.Options)
                ?? throw new AetherisException("Export file is corrupt or empty.");
            
            if (exportDto.Version != FileVersion)
                throw new AetherisException($"Unsupported export version {exportDto.Version}.");
            
            // Decrypt and import each item
            foreach (var exportItem in exportDto.Items)
            {
                var plain = CryptoEngine.Decrypt(
                    backupKey,
                    Convert.FromBase64String(exportItem.Nonce),
                    Convert.FromBase64String(exportItem.Data));
                
                try
                {
                    var item = JsonSerializer.Deserialize<VaultItem>(plain, VaultJson.Options)
                        ?? throw new AetherisException("Corrupt item in export.");
                    
                    // Add to current items
                    _items.Add(item);
                    
                    // Store in history as version 0 (initial import)
                    var encryptedItem = new StoredItemDto
                    {
                        Id = item.Id,
                        Type = item.Type.ToString(),
                        Blob = new EncBlobDto
                        {
                            Mode = "xchacha20poly1305",
                            Nonce = exportItem.Nonce,
                            Data = exportItem.Data,
                        }
                    };
                    
                    var history = new List<StoredItemDto> { encryptedItem };
                    _itemHistory[item.Id] = history;
                }
                finally
                {
                    CryptoEngine.Wipe(plain);
                }
            }
            
            Touch();
        }
        finally
        {
            CryptoEngine.Wipe(backupKey);
        }
    }
    public void Lock()
    {
        CryptoEngine.Wipe(_vaultKey);
        _vaultKey = null;
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
    private sealed class ExportDto
    {
        public int Version { get; set; }
        public DateTimeOffset ExportTimestamp { get; set; }
        public List<ExportItemDto> Items { get; set; } = new();
    }

    private sealed class ExportItemDto
    {
        public string Id { get; set; } = "";
        public string Type { get; set; } = "";
        public string Nonce { get; set; } = "";
        public string Data { get; set; } = "";
        public DateTimeOffset Timestamp { get; set; }
    }
}
