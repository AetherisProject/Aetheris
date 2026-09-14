using System;
using System.Threading.Tasks;
using Velopack;
using Velopack.Sources;

namespace Aetheris.Desktop.Services;

public class UpdateService
{
    private const string UpdateUrl = "https://updates.aetheris.com/updates";
    private UpdateManager? _updateManager;

    public event EventHandler<string>? UpdateStatusChanged;
    public event EventHandler<Version>? UpdateAvailable;
    public event EventHandler<double>? DownloadProgress;

    public bool IsChecking { get; private set; }
    public bool IsDownloading { get; private set; }
    public Version? LatestVersion { get; private set; }

    public async Task CheckForUpdatesAsync()
    {
        if (IsChecking) return;

        IsChecking = true;
        UpdateStatusChanged?.Invoke(this, "Checking for updates...");

        try
        {
            _updateManager = new UpdateManager(new Uri(UpdateUrl));
            var result = await _updateManager.CheckForUpdatesAsync();

            if (result.IsUpdateAvailable)
            {
                LatestVersion = result.LatestVersion;
                UpdateAvailable?.Invoke(this, LatestVersion);
                UpdateStatusChanged?.Invoke(this, $"Update available: {LatestVersion}");
            }
            else
            {
                UpdateStatusChanged?.Invoke(this, "No updates available");
            }
        }
        catch (Exception ex)
        {
            UpdateStatusChanged?.Invoke(this, $"Error checking updates: {ex.Message}");
        }
        finally
        {
            IsChecking = false;
        }
    }

    public async Task DownloadUpdateAsync()
    {
        if (_updateManager == null || !LatestVersion.HasValue) return;

        IsDownloading = true;
        UpdateStatusChanged?.Invoke(this, "Downloading update...");

        try
        {
            var progress = new Progress<double>(value => 
                DownloadProgress?.Invoke(this, value));

            await _updateManager.DownloadUpdatesAsync(progress);
            UpdateStatusChanged?.Invoke(this, "Update downloaded");
        }
        catch (Exception ex)
        {
            UpdateStatusChanged?.Invoke(this, $"Error downloading update: {ex.Message}");
        }
        finally
        {
            IsDownloading = false;
        }
    }

    public async Task InstallUpdateAsync()
    {
        if (_updateManager == null) return;

        UpdateStatusChanged?.Invoke(this, "Installing update...");

        try
        {
            await _updateManager.ApplyUpdatesAndRestartAsync();
        }
        catch (Exception ex)
        {
            UpdateStatusChanged?.Invoke(this, $"Error installing update: {ex.Message}");
        }
    }

    public async Task<Version> GetCurrentVersionAsync()
    {
        if (_updateManager == null)
        {
            _updateManager = new UpdateManager(new Uri(UpdateUrl));
        }

        return await _updateManager.GetCurrentVersionAsync();
    }

    public string GetReleaseNotes()
    {
        if (_updateManager == null) return string.Empty;
        return _updateManager.ReleaseNotes ?? string.Empty;
    }
}