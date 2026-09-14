using System.Text.Json;
using Microsoft.JSInterop;
namespace Aetheris.Web.Services;

/// <summary>
/// Client-side vault state with real end-to-end encryption.
/// All data is encrypted in the browser using XChaCha20-Poly1305 with a key derived
/// from the master password via Argon2id. The Hub only ever stores ciphertext.
/// </summary>
public sealed class VaultState : IAsyncDisposable
{
    private readonly CryptoService _cryptoService;
    private readonly HubClient _hubClient;
    private readonly IJSRuntime _jsRuntime;
    private readonly IConfiguration _configuration;
    
    private string? _masterPassword;
    private string? _vaultKey;
    private string? _salt;
    private bool _disposed;

    /// <summary>
    /// Whether the vault is currently unlocked.
    /// </summary>
    public bool IsUnlocked { get; private set; }

    /// <summary>
    /// The current vault generation.
    /// </summary>
    public long CurrentGeneration { get; private set; }

    /// <summary>
    /// All vault items (decrypted).
    /// </summary>
    public IReadOnlyList<VaultItem> Items => _items;
    
    private readonly List<VaultItem> _items = [];
    
    /// <summary>
    /// Filtered items for search.
    /// </summary>
    public IReadOnlyList<VaultItem> FilteredItems { get; private set; } = [];

    /// <summary>
    /// Whether the hub is reachable.
    /// </summary>
    public bool IsHubReachable => _hubClient.IsHubReachable;

    /// <summary>
    /// Whether there are unsaved changes.
    /// </summary>
    public bool HasUnsavedChanges { get; private set; }

    /// <summary>
    /// The hub URL.
    /// </summary>
    public string HubUrl => _hubClient.HubUrl;

    public VaultState(CryptoService cryptoService, HubClient hubClient, IJSRuntime jsRuntime, IConfiguration configuration)
    {
        _cryptoService = cryptoService;
        _hubClient = hubClient;
        _jsRuntime = jsRuntime;
        _configuration = configuration;
    }

    /// <summary>
    /// Initialize the vault state.
    /// </summary>
    public async Task InitializeAsync()
    {
        await _cryptoService.InitializeAsync();
        
        // Try to load saved salt from localStorage
        try
        {
            var savedSalt = await _jsRuntime.InvokeAsync<string>("localStorage.getItem", "aetheris:salt");
            if (!string.IsNullOrEmpty(savedSalt))
            {
                _salt = savedSalt;
            }
        }
        catch
        {
            // localStorage not available or error
        }
        
        // Check hub health
        _ = CheckHubHealthAsync();
    }

    /// <summary>
    /// Check if the hub is reachable.
    /// </summary>
    public async Task CheckHubHealthAsync()
    {
        await _hubClient.CheckHubHealthAsync();
    }

    /// <summary>
    /// Unlock the vault with the master password.
    /// </summary>
    /// <param name="password">The master password</param>
    /// <returns>True if unlock was successful, false otherwise</returns>
    public async Task<bool> UnlockAsync(string password)
    {
        if (IsUnlocked)
        {
            return true;
        }

        try
        {
            _masterPassword = password;
            
            // If no salt exists, this is a new vault - create one
            if (string.IsNullOrEmpty(_salt))
            {
                _salt = await _cryptoService.GenerateSaltAsync();
                await SaveSaltAsync();
            }
            
            // Derive the vault key from the password and salt
            _vaultKey = await _cryptoService.DeriveKeyAsync(password, _salt, 32);
            
            // Try to fetch the vault from the hub
            try
            {
                var vaultBlob = await _hubClient.FetchVaultAsync(_vaultKey, _cryptoService);
                CurrentGeneration = vaultBlob.Generation;
                _items.Clear();
                _items.AddRange(vaultBlob.Items);
                FilteredItems = _items;
            }
            catch (Exception ex) when (ex is not GenerationConflictException)
            {
                // Hub might not be reachable or vault doesn't exist yet
                // Start with empty vault
                CurrentGeneration = 0;
                _items.Clear();
                FilteredItems = _items;
            }
            
            IsUnlocked = true;
            HasUnsavedChanges = false;
            
            // Start auto-lock timer
            StartAutoLockTimer();
            
            // Setup tab visibility listener for auto-lock
            await SetupTabVisibilityListenerAsync();
            
            return true;
        }
        catch (Exception)
        {
            // Clear sensitive data on failure
            _masterPassword = null;
            _vaultKey = null;
            IsUnlocked = false;
            
            return false;
        }
    }

    /// <summary>
    /// Lock the vault.
    /// </summary>
    public void Lock()
    {
        // Clear sensitive data
        _masterPassword = null;
        _vaultKey = null;
        IsUnlocked = false;
        FilteredItems = [];
        
        // Stop auto-lock timer
        StopAutoLockTimer();
    }

    /// <summary>
    /// Mark the vault as unlocked (for demo purposes - use UnlockAsync for real unlock).
    /// </summary>
    [Obsolete("Use UnlockAsync for real unlock with crypto")]
    public void MarkUnlocked() => IsUnlocked = true;

    /// <summary>
    /// Add a new item to the vault.
    /// </summary>
    /// <param name="item">The item to add</param>
    public void AddItem(VaultItem item)
    {
        if (!IsUnlocked)
        {
            throw new InvalidOperationException("Vault is locked");
        }
        
        // Generate ID if not set
        if (string.IsNullOrEmpty(item.Id))
        {
            item = item with { Id = Guid.NewGuid().ToString() };
        }
        
        // Set timestamps
        var now = DateTime.UtcNow;
        item = item with { CreatedAt = now, UpdatedAt = now };
        
        _items.Add(item);
        FilteredItems = _items; // Reset filter
        HasUnsavedChanges = true;
    }

    /// <summary>
    /// Update an existing item.
    /// </summary>
    /// <param name="item">The updated item</param>
    public void UpdateItem(VaultItem item)
    {
        if (!IsUnlocked)
        {
            throw new InvalidOperationException("Vault is locked");
        }
        
        var existingIndex = _items.FindIndex(i => i.Id == item.Id);
        if (existingIndex >= 0)
        {
            item = item with { UpdatedAt = DateTime.UtcNow };
            _items[existingIndex] = item;
            FilteredItems = _items; // Reset filter
            HasUnsavedChanges = true;
        }
    }

    /// <summary>
    /// Delete an item from the vault.
    /// </summary>
    /// <param name="itemId">The ID of the item to delete</param>
    public void DeleteItem(string itemId)
    {
        if (!IsUnlocked)
        {
            throw new InvalidOperationException("Vault is locked");
        }
        
        var index = _items.FindIndex(i => i.Id == itemId);
        if (index >= 0)
        {
            _items.RemoveAt(index);
            FilteredItems = _items; // Reset filter
            HasUnsavedChanges = true;
        }
    }

    /// <summary>
    /// Filter items by search query.
    /// </summary>
    /// <param name="query">The search query</param>
    public void FilterItems(string query)
    {
        if (string.IsNullOrWhiteSpace(query))
        {
            FilteredItems = _items;
        }
        else
        {
            var lowerQuery = query.ToLowerInvariant();
            FilteredItems = _items.Where(i => 
                i.Title.Contains(lowerQuery, StringComparison.OrdinalIgnoreCase) ||
                (i.Username != null && i.Username.Contains(lowerQuery, StringComparison.OrdinalIgnoreCase)) ||
                (i.Url != null && i.Url.Contains(lowerQuery, StringComparison.OrdinalIgnoreCase)) ||
                (i.Notes != null && i.Notes.Contains(lowerQuery, StringComparison.OrdinalIgnoreCase)) ||
                (i.Tags != null && i.Tags.Any(t => t.Contains(lowerQuery, StringComparison.OrdinalIgnoreCase)))
            ).ToList();
        }
    }

    /// <summary>
    /// Push changes to the hub.
    /// </summary>
    public async Task<bool> PushAsync()
    {
        if (!IsUnlocked || _vaultKey == null)
        {
            return false;
        }
        
        try
        {
            var vaultBlob = new VaultBlob
            {
                Generation = CurrentGeneration + 1,
                Items = _items,
                Metadata = new VaultMetadata
                {
                    Name = "Main Vault",
                    LastSync = DateTime.UtcNow,
                    DeviceId = await GetDeviceIdAsync()
                }
            };
            
            var newGeneration = await _hubClient.PushVaultAsync(vaultBlob, _vaultKey, _cryptoService);
            CurrentGeneration = newGeneration;
            HasUnsavedChanges = false;
            return true;
        }
        catch (GenerationConflictException)
        {
            // Try pull-merge-push
            var merged = await _hubClient.PullMergePushAsync(
                new VaultBlob { Generation = CurrentGeneration, Items = _items },
                _vaultKey,
                _cryptoService
            );
            CurrentGeneration = merged.Generation;
            _items.Clear();
            _items.AddRange(merged.Items);
            FilteredItems = _items;
            HasUnsavedChanges = false;
            return true;
        }
        catch
        {
            return false;
        }
    }

    /// <summary>
    /// Pull latest changes from the hub.
    /// </summary>
    public async Task<bool> PullAsync()
    {
        if (!IsUnlocked || _vaultKey == null)
        {
            return false;
        }
        
        try
        {
            var vaultBlob = await _hubClient.FetchVaultAsync(_vaultKey, _cryptoService);
            CurrentGeneration = vaultBlob.Generation;
            _items.Clear();
            _items.AddRange(vaultBlob.Items);
            FilteredItems = _items;
            HasUnsavedChanges = false;
            return true;
        }
        catch
        {
            return false;
        }
    }

    /// <summary>
    /// Sync with hub (pull-merge-push).
    /// </summary>
    public async Task<bool> SyncAsync()
    {
        if (!IsUnlocked || _vaultKey == null)
        {
            return false;
        }
        
        try
        {
            var localBlob = new VaultBlob { Generation = CurrentGeneration, Items = _items };
            var merged = await _hubClient.PullMergePushAsync(localBlob, _vaultKey, _cryptoService);
            CurrentGeneration = merged.Generation;
            _items.Clear();
            _items.AddRange(merged.Items);
            FilteredItems = _items;
            HasUnsavedChanges = false;
            return true;
        }
        catch
        {
            return false;
        }
    }

    /// <summary>
    /// Get TOTP code for an item (if it has a TOTP secret).
    /// </summary>
    /// <param name="itemId">The item ID</param>
    /// <returns>The current TOTP code or null</returns>
    public async Task<string?> GetTotpCodeAsync(string itemId)
    {
        if (!IsUnlocked)
        {
            return null;
        }
        
        var item = _items.FirstOrDefault(i => i.Id == itemId);
        if (item?.TotpSecret == null)
        {
            return null;
        }
        
        // Use JS interop for TOTP calculation
        try
        {
            return await _jsRuntime.InvokeAsync<string>(
                "AetherisTotp.generate",
                item.TotpSecret
            );
        }
        catch
        {
            return null;
        }
    }

    /// <summary>
    /// Copy text to clipboard.
    /// </summary>
    /// <param name="text">The text to copy</param>
    public async Task CopyToClipboardAsync(string text)
    {
        try
        {
            await _jsRuntime.InvokeVoidAsync("navigator.clipboard.writeText", text);
        }
        catch
        {
            // Fallback for older browsers
            await _jsRuntime.InvokeVoidAsync("AetherisClipboard.copy", text);
        }
    }

    private async Task SaveSaltAsync()
    {
        if (_salt != null)
        {
            try
            {
                await _jsRuntime.InvokeVoidAsync("localStorage.setItem", "aetheris:salt", _salt);
            }
            catch
            {
                // localStorage not available
            }
        }
    }

    private async Task<string?> GetDeviceIdAsync()
    {
        try
        {
            var deviceId = await _jsRuntime.InvokeAsync<string>("localStorage.getItem", "aetheris:deviceId");
            if (string.IsNullOrEmpty(deviceId))
            {
                deviceId = Guid.NewGuid().ToString();
                await _jsRuntime.InvokeVoidAsync("localStorage.setItem", "aetheris:deviceId", deviceId);
            }
            return deviceId;
        }
        catch
        {
            return null;
        }
    }

    private System.Timers.Timer? _autoLockTimer;
    private const int AutoLockMinutes = 5; // 5 minute default

    private void StartAutoLockTimer()
    {
        StopAutoLockTimer();
        
        _autoLockTimer = new System.Timers.Timer(AutoLockMinutes * 60 * 1000);
        _autoLockTimer.Elapsed += (sender, e) => Lock();
        _autoLockTimer.AutoReset = false;
        _autoLockTimer.Start();
    }

    private void StopAutoLockTimer()
    {
        if (_autoLockTimer != null)
        {
            _autoLockTimer.Stop();
            _autoLockTimer.Elapsed -= (sender, e) => Lock();
            _autoLockTimer.Dispose();
            _autoLockTimer = null;
        }
    }

    private async Task SetupTabVisibilityListenerAsync()
    {
        try
        {
            await _jsRuntime.InvokeVoidAsync("setupTabVisibilityListener");
        }
        catch
        {
            // Browser doesn't support visibility API
        }
    }

    public async ValueTask DisposeAsync()
    {
        if (!_disposed)
        {
            _disposed = true;
            StopAutoLockTimer();
            Lock();
        }
    }
}

