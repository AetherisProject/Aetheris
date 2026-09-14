using System.IO;
using Aetheris.Gateway;

namespace Aetheris.Gateway.Services;

/// <summary>
/// Hot keystore service that watches the vault file for changes and re-unlocks on mtime change.
/// Only supports AETHERIS_GATEWAY_PASS from environment (not interactive).
/// </summary>
public sealed class VaultService : IKeyring, IDisposable
{
    private readonly string _vaultPath;
    private readonly string? _passphrase;
    private readonly ILogger _logger;
    private VaultStore? _vault;
    private FileSystemWatcher? _watcher;
    private readonly object _lock = new();
    private DateTime _lastMtime;
    private bool _disposed;

    public VaultService(string vaultPath, string? passphrase, ILogger logger)
    {
        _vaultPath = vaultPath;
        _passphrase = passphrase;
        _logger = logger;
        _lastMtime = File.Exists(vaultPath) ? File.GetLastWriteTimeUtc(vaultPath) : DateTime.MinValue;
    }

    public void StartWatching()
    {
        if (!File.Exists(_vaultPath))
        {
            _logger.LogWarning("Vault file {Path} does not exist, cannot watch for changes", _vaultPath);
            return;
        }

        try
        {
            _watcher = new FileSystemWatcher
            {
                Path = Path.GetDirectoryName(_vaultPath) ?? ".",
                Filter = Path.GetFileName(_vaultPath),
                NotifyFilter = NotifyFilters.LastWrite,
                EnableRaisingEvents = true
            };

            _watcher.Changed += OnVaultChanged;
            _logger.LogInformation("Watching vault file {Path} for changes", _vaultPath);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to start watching vault file {Path}", _vaultPath);
        }
    }

    private void OnVaultChanged(object sender, FileSystemEventArgs e)
    {
        try
        {
            // Debounce: wait a bit and check if the file is still being written
            Thread.Sleep(100);
            
            var currentMtime = File.GetLastWriteTimeUtc(_vaultPath);
            if (currentMtime == _lastMtime)
                return;

            _lastMtime = currentMtime;
            ReloadVault();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error handling vault file change");
        }
    }

    public void ReloadVault()
    {
        lock (_lock)
        {
            try
            {
                if (string.IsNullOrEmpty(_passphrase))
                {
                    _logger.LogWarning("AETHERIS_GATEWAY_PASS not set — cannot reload vault");
                    _vault?.Dispose();
                    _vault = null;
                    return;
                }

                _vault?.Dispose();
                _vault = VaultStore.Unlock(_vaultPath, _passphrase);
                _logger.LogInformation("Vault reloaded successfully");
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Failed to reload vault: {Message}", ex.Message);
                // Keep the old vault if reload fails
            }
        }
    }

    public string? GetKey(string provider)
    {
        lock (_lock)
        {
            if (_vault == null)
            {
                // Try to open if not yet loaded
                TryOpenInitial();
            }

            return _vault?.Items
                .Where(i => i.Type == ItemType.ApiKey)
                .Select(i => i.Payload<ApiKeyPayload>())
                .FirstOrDefault(k => k.Provider.Equals(provider, StringComparison.OrdinalIgnoreCase))
                ?.Key;
        }
    }

    private void TryOpenInitial()
    {
        if (string.IsNullOrEmpty(_passphrase))
        {
            _logger.LogWarning("AETHERIS_GATEWAY_PASS not set — vault stays locked");
            return;
        }

        if (!File.Exists(_vaultPath))
        {
            _logger.LogWarning("Vault file {Path} not found", _vaultPath);
            return;
        }

        try
        {
            _vault = VaultStore.Unlock(_vaultPath, _passphrase);
            _logger.LogInformation("Vault opened successfully");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to open vault: {Message}", ex.Message);
        }
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;

        _watcher?.Dispose();
        lock (_lock)
        {
            _vault?.Dispose();
        }

        GC.SuppressFinalize(this);
    }
}