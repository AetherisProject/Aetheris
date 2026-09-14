using System.Net;
using System.Net.Http.Json;
using System.Text.Json;

namespace Aetheris.Web.Services;

/// <summary>
/// Client for communicating with the Aetheris Hub (sync server).
/// All data sent to/received from the hub is encrypted - the hub only stores ciphertext.
/// </summary>
public sealed class HubClient : IAsyncDisposable
{
    private readonly HttpClient _httpClient;
    private readonly IConfiguration _configuration;
    private bool _disposed;

    /// <summary>
    /// The base URL of the hub.
    /// </summary>
    public string HubUrl { get; }

    /// <summary>
    /// The current vault generation.
    /// </summary>
    public long CurrentGeneration { get; private set; }

    /// <summary>
    /// Whether the hub is currently reachable.
    /// </summary>
    public bool IsHubReachable { get; private set; } = true;

    public HubClient(HttpClient httpClient, IConfiguration configuration)
    {
        _httpClient = httpClient;
        _configuration = configuration;
        HubUrl = configuration["HubUrl"] ?? "http://127.0.0.1:8080";
        // Remove trailing slash if present
        if (HubUrl.EndsWith('/'))
        {
            HubUrl = HubUrl[..^1];
        }
    }

    /// <summary>
    /// Fetch the encrypted vault blob from the hub.
    /// </summary>
    /// <param name="vaultKey">The vault encryption key (hex string)</param>
    /// <param name="cryptoService">Crypto service for decryption</param>
    /// <returns>Decrypted vault data and generation</returns>
    public async Task<VaultBlob> FetchVaultAsync(string vaultKey, CryptoService cryptoService)
    {
        try
        {
            IsHubReachable = true;
            var response = await _httpClient.GetAsync($"{HubUrl}/v1/vault");
            
            if (response.StatusCode == HttpStatusCode.NotFound)
            {
                // No vault exists yet - return empty
                return new VaultBlob { Generation = 0, Items = [] };
            }
            
            response.EnsureSuccessStatusCode();
            
            var blob = await response.Content.ReadFromJsonAsync<HubVaultBlob>();
            if (blob == null)
            {
                throw new InvalidOperationException("Invalid vault blob from hub");
            }
            
            CurrentGeneration = blob.Generation;
            
            // Decrypt the vault data
            var decrypted = await cryptoService.DecryptObjectAsync<VaultData>(
                new EncryptedData { Ciphertext = blob.Ciphertext, Nonce = blob.Nonce },
                vaultKey
            );
            
            return new VaultBlob
            {
                Generation = blob.Generation,
                Items = decrypted.Items,
                Metadata = decrypted.Metadata
            };
        }
        catch (HttpRequestException ex) when (ex.StatusCode == HttpStatusCode.NotFound)
        {
            return new VaultBlob { Generation = 0, Items = [] };
        }
        catch (Exception)
        {
            IsHubReachable = false;
            throw;
        }
    }

    /// <summary>
    /// Push the encrypted vault blob to the hub.
    /// </summary>
    /// <param name="vaultBlob">The vault data to push</param>
    /// <param name="vaultKey">The vault encryption key (hex string)</param>
    /// <param name="cryptoService">Crypto service for encryption</param>
    /// <returns>The new generation from the hub</returns>
    public async Task<long> PushVaultAsync(VaultBlob vaultBlob, string vaultKey, CryptoService cryptoService)
    {
        try
        {
            IsHubReachable = true;
            
            // Encrypt the vault data
            var vaultData = new VaultData
            {
                Items = vaultBlob.Items,
                Metadata = vaultBlob.Metadata
            };
            
            var encrypted = await cryptoService.EncryptObjectAsync(vaultData, vaultKey);
            
            var hubBlob = new HubVaultBlob
            {
                Generation = vaultBlob.Generation,
                Ciphertext = encrypted.Ciphertext,
                Nonce = encrypted.Nonce,
                ExpectedGeneration = vaultBlob.Generation - 1
            };
            
            var response = await _httpClient.PutAsJsonAsync($"{HubUrl}/v1/vault", hubBlob);
            
            if (response.StatusCode == HttpStatusCode.Conflict)
            {
                // Generation mismatch - need to pull-merge-push
                throw new GenerationConflictException("Generation conflict - vault was modified elsewhere");
            }
            
            response.EnsureSuccessStatusCode();
            
            var result = await response.Content.ReadFromJsonAsync<PushResult>();
            CurrentGeneration = result.NewGeneration;
            
            return result.NewGeneration;
        }
        catch (Exception)
        {
            IsHubReachable = false;
            throw;
        }
    }

    /// <summary>
    /// Pull the latest vault, merge local changes, and push back.
    /// </summary>
    /// <param name="localBlob">The local vault data</param>
    /// <param name="vaultKey">The vault encryption key</param>
    /// <param name="cryptoService">Crypto service for encryption/decryption</param>
    /// <returns>The merged vault blob with new generation</returns>
    public async Task<VaultBlob> PullMergePushAsync(VaultBlob localBlob, string vaultKey, CryptoService cryptoService)
    {
        // Fetch latest from hub
        var remoteBlob = await FetchVaultAsync(vaultKey, cryptoService);
        
        // Simple merge: prefer local changes for now (more sophisticated merge in future)
        // For this implementation, we'll do a simple last-write-wins per item
        var mergedItems = new List<VaultItem>();
        var localById = localBlob.Items.ToDictionary(i => i.Id);
        var remoteById = remoteBlob.Items.ToDictionary(i => i.Id);
        
        // Start with all remote items
        foreach (var item in remoteBlob.Items)
        {
            mergedItems.Add(item with { });
        }
        
        // Overlay local changes
        foreach (var item in localBlob.Items)
        {
            if (localById.TryGetValue(item.Id, out var localItem))
            {
                // Local version wins
                if (mergedItems.All(i => i.Id != item.Id))
                {
                    mergedItems.Add(item);
                }
                else
                {
                    var index = mergedItems.FindIndex(i => i.Id == item.Id);
                    if (index >= 0)
                    {
                        mergedItems[index] = item;
                    }
                }
            }
        }
        
        // Create merged blob
        var mergedBlob = new VaultBlob
        {
            Generation = remoteBlob.Generation,
            Items = mergedItems,
            Metadata = localBlob.Metadata ?? remoteBlob.Metadata
        };
        
        // Push merged version
        var newGeneration = await PushVaultAsync(mergedBlob, vaultKey, cryptoService);
        
        return mergedBlob with { Generation = newGeneration };
    }

    /// <summary>
    /// Check if the hub is reachable.
    /// </summary>
    public async Task<bool> CheckHubHealthAsync()
    {
        try
        {
            var response = await _httpClient.GetAsync($"{HubUrl}/v1/health");
            IsHubReachable = response.IsSuccessStatusCode;
            return IsHubReachable;
        }
        catch
        {
            IsHubReachable = false;
            return false;
        }
    }

    public async ValueTask DisposeAsync()
    {
        if (!_disposed)
        {
            _disposed = true;
            _httpClient.Dispose();
        }
    }
}

/// <summary>
/// Exception thrown when there's a generation conflict.
/// </summary>
public sealed class GenerationConflictException : Exception
{
    public GenerationConflictException(string message) : base(message) { }
}

/// <summary>
/// The encrypted vault blob as stored on the hub.
/// </summary>
public sealed record HubVaultBlob
{
    public long Generation { get; init; }
    public string Ciphertext { get; init; } = string.Empty;
    public string Nonce { get; init; } = string.Empty;
    public long ExpectedGeneration { get; init; }
}

/// <summary>
/// The decrypted vault blob.
/// </summary>
public sealed record VaultBlob
{
    public long Generation { get; init; }
    public List<VaultItem> Items { get; init; } = [];
    public VaultMetadata? Metadata { get; init; }
}

/// <summary>
/// Vault metadata.
/// </summary>
public sealed record VaultMetadata
{
    public string Name { get; init; } = string.Empty;
    public DateTime LastSync { get; init; }
    public string? DeviceId { get; init; }
}

/// <summary>
/// A vault item (encrypted on the hub, decrypted in the browser).
/// </summary>
public sealed record VaultItem
{
    public string Id { get; init; } = string.Empty;
    public string Type { get; init; } = string.Empty; // Password, ApiKey, Note, Card, Identity
    public string Title { get; init; } = string.Empty;
    public string? Username { get; init; }
    public string? Password { get; init; }
    public string? Url { get; init; }
    public string? Notes { get; init; }
    public Dictionary<string, string>? CustomFields { get; init; }
    public DateTime CreatedAt { get; init; }
    public DateTime UpdatedAt { get; init; }
    public string? TotpSecret { get; init; }
    public string? Icon { get; init; }
    public List<string>? Tags { get; init; }
}

/// <summary>
/// Internal data structure for encryption.
/// </summary>
public sealed record VaultData
{
    public List<VaultItem> Items { get; init; } = [];
    public VaultMetadata? Metadata { get; init; }
}

/// <summary>
/// Result of a push operation.
/// </summary>
public sealed record PushResult
{
    public long NewGeneration { get; init; }
}
