using System;
using System.Collections.Generic;
using System.IO;
using System.Threading.Tasks;
using Renci.SshNet;
using Renci.SshNet.Sftp;

namespace Aetheris.Desktop.Services;

public class SftpService
{
    private SftpClient? _sftpClient;
    private bool _isConnected = false;
    private string? _currentHost;
    private int _currentPort;
    private string? _currentUsername;

    public event EventHandler<string>? ConnectionStatusChanged;
    public event EventHandler<FileSystemItem[]>? DirectoryListed;
    public event EventHandler<string>? FileTransferred;
    public event EventHandler? ConnectionClosed;

    public bool IsConnected => _isConnected;

    public async Task ConnectAsync(string host = "", int port = 22, string username = "", string password = "")
    {
        await DisconnectAsync();

        try
        {
            // If no connection info provided, try to use SSH service's connection
            if (string.IsNullOrEmpty(host))
            {
                ConnectionStatusChanged?.Invoke(this, "Error: No connection information");
                return;
            }

            var connectionInfo = new ConnectionInfo(host, port, username, 
                new PasswordAuthenticationMethod(username, password));

            _sftpClient = new SftpClient(connectionInfo);
            _currentHost = host;
            _currentPort = port;
            _currentUsername = username;

            await Task.Run(() => _sftpClient.Connect());
            
            _isConnected = true;
            ConnectionStatusChanged?.Invoke(this, "Connected");
        }
        catch (Exception ex)
        {
            ConnectionStatusChanged?.Invoke(this, $"Error: {ex.Message}");
            throw;
        }
    }

    public async Task DisconnectAsync()
    {
        if (!_isConnected) return;

        try
        {
            if (_sftpClient != null)
            {
                await Task.Run(() => _sftpClient.Disconnect());
                _sftpClient.Dispose();
                _sftpClient = null;
            }

            _isConnected = false;
            ConnectionStatusChanged?.Invoke(this, "Disconnected");
            ConnectionClosed?.Invoke(this, EventArgs.Empty);
        }
        catch { }
    }

    public async Task ListDirectoryAsync(string path)
    {
        if (!_isConnected || _sftpClient == null) return;

        try
        {
            var files = await Task.Run(() => _sftpClient.ListDirectory(path));
            
            var items = new List<FileSystemItem>();
            foreach (var file in files)
            {
                items.Add(new FileSystemItem
                {
                    Name = file.Name,
                    Path = file.FullName,
                    IsDirectory = file.IsDirectory,
                    Size = file.Length,
                    ModifiedAt = file.LastWriteTime
                });
            }

            DirectoryListed?.Invoke(this, items.ToArray());
        }
        catch (Exception ex)
        {
            FileTransferred?.Invoke(this, $"Error listing directory: {ex.Message}");
        }
    }

    public async Task UploadAsync(string localPath, string remotePath)
    {
        if (!_isConnected || _sftpClient == null) return;

        try
        {
            var fileName = Path.GetFileName(localPath);
            var remoteFilePath = Path.Combine(remotePath, fileName).Replace("\\", "/");

            using (var fileStream = File.OpenRead(localPath))
            {
                await Task.Run(() => _sftpClient.UploadFile(fileStream, remoteFilePath));
            }

            FileTransferred?.Invoke(this, $"Uploaded: {fileName}");
        }
        catch (Exception ex)
        {
            FileTransferred?.Invoke(this, $"Upload failed: {ex.Message}");
        }
    }

    public async Task DownloadAsync(string remotePath, string localPath)
    {
        if (!_isConnected || _sftpClient == null) return;

        try
        {
            var fileName = Path.GetFileName(remotePath);
            var localFilePath = Path.Combine(localPath, fileName);

            using (var fileStream = File.Create(localFilePath))
            {
                await Task.Run(() => _sftpClient.DownloadFile(remotePath, fileStream));
            }

            FileTransferred?.Invoke(this, $"Downloaded: {fileName}");
        }
        catch (Exception ex)
        {
            FileTransferred?.Invoke(this, $"Download failed: {ex.Message}");
        }
    }

    public async Task DeleteAsync(string remotePath)
    {
        if (!_isConnected || _sftpClient == null) return;

        try
        {
            await Task.Run(() => _sftpClient.Delete(remotePath));
            FileTransferred?.Invoke(this, $"Deleted: {remotePath}");
        }
        catch (Exception ex)
        {
            FileTransferred?.Invoke(this, $"Delete failed: {ex.Message}");
        }
    }

    public async Task CreateDirectoryAsync(string remotePath)
    {
        if (!_isConnected || _sftpClient == null) return;

        try
        {
            await Task.Run(() => _sftpClient.CreateDirectory(remotePath));
            FileTransferred?.Invoke(this, $"Created directory: {remotePath}");
        }
        catch (Exception ex)
        {
            FileTransferred?.Invoke(this, $"Create directory failed: {ex.Message}");
        }
    }

    public async Task<bool> FileExistsAsync(string remotePath)
    {
        if (!_isConnected || _sftpClient == null) return false;

        return await Task.Run(() => _sftpClient.Exists(remotePath));
    }

    public async Task<long> GetFileSizeAsync(string remotePath)
    {
        if (!_isConnected || _sftpClient == null) return 0;

        return await Task.Run(() => _sftpClient.GetAttributes(remotePath).Size);
    }
}