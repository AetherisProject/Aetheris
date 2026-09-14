using System;
using System.IO;
using System.Text;
using System.Threading.Tasks;
using Renci.SshNet;
using Renci.SshNet.Common;

namespace Aetheris.Desktop.Services;

public class SshService
{
    private SshClient? _sshClient;
    private ShellStream? _shellStream;
    private StreamWriter? _streamWriter;
    private StreamReader? _streamReader;
    private bool _isConnected = false;
    private string? _currentHost;
    private int _currentPort;
    private string? _currentUsername;
    private byte[]? _privateKeyData;

    public event EventHandler<string>? ConnectionStatusChanged;
    public event EventHandler<string>? TerminalOutputReceived;
    public event EventHandler? ConnectionClosed;

    public bool IsConnected => _isConnected;

    public async Task ConnectAsync(string host, int port, string username, string password)
    {
        await DisconnectAsync();

        try
        {
            var connectionInfo = new ConnectionInfo(host, port, username, 
                new PasswordAuthenticationMethod(username, password));

            _sshClient = new SshClient(connectionInfo);
            _currentHost = host;
            _currentPort = port;
            _currentUsername = username;

            await Task.Run(() => _sshClient.Connect());
            
            _shellStream = _sshClient.CreateShellStream("xterm", 80, 24, 800, 600, 1024);
            _streamWriter = new StreamWriter(_shellStream);
            _streamReader = new StreamReader(_shellStream);

            _isConnected = true;
            ConnectionStatusChanged?.Invoke(this, "Connected");

            // Start reading output
            _ = ReadOutputAsync();
        }
        catch (Exception ex)
        {
            ConnectionStatusChanged?.Invoke(this, $"Error: {ex.Message}");
            throw;
        }
    }

    public async Task ConnectWithKeyAsync(string host, int port, string username, byte[] privateKeyData, string? passphrase = null)
    {
        await DisconnectAsync();

        try
        {
            var privateKeyFile = new MemoryStream(privateKeyData);
            var privateKey = new PrivateKeyFile(privateKeyFile);
            
            if (!string.IsNullOrEmpty(passphrase))
            {
                privateKey = new PrivateKeyFile(privateKeyFile, passphrase);
            }

            var authMethod = new PrivateKeyAuthenticationMethod(username, privateKey);
            var connectionInfo = new ConnectionInfo(host, port, username, authMethod);

            _sshClient = new SshClient(connectionInfo);
            _currentHost = host;
            _currentPort = port;
            _currentUsername = username;
            _privateKeyData = privateKeyData;

            await Task.Run(() => _sshClient.Connect());
            
            _shellStream = _sshClient.CreateShellStream("xterm", 80, 24, 800, 600, 1024);
            _streamWriter = new StreamWriter(_shellStream);
            _streamReader = new StreamReader(_shellStream);

            _isConnected = true;
            ConnectionStatusChanged?.Invoke(this, "Connected");

            // Start reading output
            _ = ReadOutputAsync();
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
            _streamWriter?.Close();
            _streamReader?.Close();
            _shellStream?.Close();
            
            if (_sshClient != null)
            {
                await Task.Run(() => _sshClient.Disconnect());
                _sshClient.Dispose();
                _sshClient = null;
            }

            // Zeroize the private key data
            ZeroizeKeyData();

            _isConnected = false;
            ConnectionStatusChanged?.Invoke(this, "Disconnected");
            ConnectionClosed?.Invoke(this, EventArgs.Empty);
        }
        catch { }
    }

    public async Task SendCommandAsync(string command)
    {
        if (!_isConnected || _streamWriter == null) return;

        try
        {
            await _streamWriter.WriteLineAsync(command);
            await _streamWriter.FlushAsync();
        }
        catch (Exception ex)
        {
            TerminalOutputReceived?.Invoke(this, $"\n[Error: {ex.Message}]\n");
        }
    }

    private async Task ReadOutputAsync()
    {
        if (_shellStream == null || _streamReader == null) return;

        var buffer = new byte[1024];
        try
        {
            while (_isConnected && !_shellStream.DataAvailable)
            {
                await Task.Delay(100);
            }

            while (_isConnected)
            {
                if (_shellStream.DataAvailable)
                {
                    var bytesRead = await _shellStream.ReadAsync(buffer, 0, buffer.Length);
                    if (bytesRead > 0)
                    {
                        var output = Encoding.UTF8.GetString(buffer, 0, bytesRead);
                        TerminalOutputReceived?.Invoke(this, output);
                    }
                }
                else
                {
                    await Task.Delay(100);
                }
            }
        }
        catch (Exception ex) when (ex is IOException || ex is SshException)
        {
            // Connection closed
            TerminalOutputReceived?.Invoke(this, $"\n[Connection closed: {ex.Message}]\n");
        }
        catch { }
    }

    private void ZeroizeKeyData()
    {
        if (_privateKeyData != null)
        {
            Array.Clear(_privateKeyData, 0, _privateKeyData.Length);
            _privateKeyData = null;
        }
    }

    // Memory dump proof - this would be used to verify keys are zeroized
    public bool HasKeysInMemory()
    {
        return _privateKeyData != null && _privateKeyData.Length > 0;
    }

    public string? GetCurrentConnectionInfo()
    {
        if (!_isConnected) return null;
        return $"{_currentUsername}@{_currentHost}:{_currentPort}";
    }
}