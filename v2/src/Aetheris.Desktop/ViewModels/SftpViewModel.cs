using System;
using System.Collections.ObjectModel;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Aetheris.Core;
using Aetheris.Desktop.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace Aetheris.Desktop.ViewModels;

public partial class SftpViewModel : ViewModelBase
{
    private SftpService? _sftpService;
    private VaultService? _vaultService;

    [ObservableProperty]
    private bool _isBusy;

    [ObservableProperty]
    private bool _isConnected;

    [ObservableProperty]
    private string _connectionStatus = "Disconnected";

    [ObservableProperty]
    private string _currentPath = "/";

    [ObservableProperty]
    private ObservableCollection<FileSystemItemViewModel> _remoteItems = new();

    [ObservableProperty]
    private ObservableCollection<FileSystemItemViewModel> _localItems = new();

    [ObservableProperty]
    private FileSystemItemViewModel? _selectedRemoteItem;

    [ObservableProperty]
    private FileSystemItemViewModel? _selectedLocalItem;

    [ObservableProperty]
    private string _localPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), "Downloads");

    [ObservableProperty]
    private string _transferStatus = "Ready";

    public SftpViewModel() { }

    public void Initialize(SftpService sftpService, VaultService vaultService)
    {
        _sftpService = sftpService;
        _vaultService = vaultService;

        _sftpService.ConnectionStatusChanged += OnConnectionStatusChanged;
        _sftpService.DirectoryListed += OnDirectoryListed;
        _sftpService.FileTransferred += OnFileTransferred;
        _sftpService.ConnectionClosed += OnConnectionClosed;

        LoadLocalFiles();
    }

    private void OnConnectionStatusChanged(object? sender, string status)
    {
        ConnectionStatus = status;
        IsConnected = status == "Connected";
    }

    private void OnDirectoryListed(object? sender, FileSystemItem[] items)
    {
        RemoteItems = new ObservableCollection<FileSystemItemViewModel>(
            items.Select(item => new FileSystemItemViewModel(item)));
    }

    private void OnFileTransferred(object? sender, string message)
    {
        TransferStatus = message;
    }

    private void OnConnectionClosed(object? sender, EventArgs e)
    {
        IsConnected = false;
        ConnectionStatus = "Disconnected";
        RemoteItems.Clear();
    }

    private void LoadLocalFiles()
    {
        if (!Directory.Exists(LocalPath)) return;

        try
        {
            var files = Directory.GetFileSystemEntries(LocalPath)
                .Select(path => new FileSystemItem
                {
                    Name = Path.GetFileName(path),
                    Path = path,
                    IsDirectory = Directory.Exists(path),
                    Size = File.Exists(path) ? new FileInfo(path).Length : 0,
                    ModifiedAt = File.Exists(path) ? File.GetLastWriteTime(path) : DateTime.MinValue
                });

            LocalItems = new ObservableCollection<FileSystemItemViewModel>(
                files.Select(item => new FileSystemItemViewModel(item)));
        }
        catch { }
    }

    [RelayCommand]
    private async Task ConnectAsync()
    {
        if (_sftpService == null) return;

        IsBusy = true;
        try
        {
            // Use the same connection as SSH if available
            await _sftpService.ConnectAsync();
            await ListRemoteDirectoryAsync();
        }
        catch (Exception ex)
        {
            TransferStatus = $"Error: {ex.Message}";
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task DisconnectAsync()
    {
        if (_sftpService == null) return;

        await _sftpService.DisconnectAsync();
    }

    [RelayCommand]
    private async Task ListRemoteDirectoryAsync()
    {
        if (_sftpService == null || !IsConnected) return;

        await _sftpService.ListDirectoryAsync(CurrentPath);
    }

    [RelayCommand]
    private async Task ChangeRemoteDirectoryAsync()
    {
        if (SelectedRemoteItem == null || !SelectedRemoteItem.IsDirectory) return;

        CurrentPath = Path.Combine(CurrentPath, SelectedRemoteItem.Name).Replace("\\", "/");
        await ListRemoteDirectoryAsync();
    }

    [RelayCommand]
    private async Task ChangeLocalDirectoryAsync()
    {
        if (SelectedLocalItem == null || !SelectedLocalItem.IsDirectory) return;

        LocalPath = SelectedLocalItem.Path;
        LoadLocalFiles();
    }

    [RelayCommand]
    private async Task UploadAsync()
    {
        if (_sftpService == null || SelectedLocalItem == null || !IsConnected) return;

        IsBusy = true;
        try
        {
            await _sftpService.UploadAsync(SelectedLocalItem.Path, CurrentPath);
            await ListRemoteDirectoryAsync();
        }
        catch (Exception ex)
        {
            TransferStatus = $"Upload failed: {ex.Message}";
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task DownloadAsync()
    {
        if (_sftpService == null || SelectedRemoteItem == null || !IsConnected) return;

        IsBusy = true;
        try
        {
            await _sftpService.DownloadAsync(SelectedRemoteItem.Path, LocalPath);
            LoadLocalFiles();
        }
        catch (Exception ex)
        {
            TransferStatus = $"Download failed: {ex.Message}";
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task DeleteRemoteAsync()
    {
        if (_sftpService == null || SelectedRemoteItem == null || !IsConnected) return;

        if (await ShowDeleteConfirmationAsync())
        {
            await _sftpService.DeleteAsync(SelectedRemoteItem.Path);
            await ListRemoteDirectoryAsync();
        }
    }

    [RelayCommand]
    private async Task RefreshAsync()
    {
        if (IsConnected)
        {
            await ListRemoteDirectoryAsync();
        }
        LoadLocalFiles();
    }

    private async Task<bool> ShowDeleteConfirmationAsync()
    {
        // TODO: Show confirmation dialog
        return true;
    }
}

public partial class FileSystemItemViewModel : ViewModelBase
{
    [ObservableProperty]
    private string _name = string.Empty;

    [ObservableProperty]
    private string _path = string.Empty;

    [ObservableProperty]
    private bool _isDirectory;

    [ObservableProperty]
    private long _size;

    [ObservableProperty]
    private DateTime _modifiedAt;

    public FileSystemItemViewModel() { }

    public FileSystemItemViewModel(FileSystemItem item)
    {
        Name = item.Name;
        Path = item.Path;
        IsDirectory = item.IsDirectory;
        Size = item.Size;
        ModifiedAt = item.ModifiedAt;
    }
}

public class FileSystemItem
{
    public string Name { get; set; } = string.Empty;
    public string Path { get; set; } = string.Empty;
    public bool IsDirectory { get; set; }
    public long Size { get; set; }
    public DateTime ModifiedAt { get; set; }
}