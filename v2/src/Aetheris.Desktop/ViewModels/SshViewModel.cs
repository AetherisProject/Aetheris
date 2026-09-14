using System;
using System.Collections.ObjectModel;
using System.Linq;
using System.Threading.Tasks;
using Aetheris.Core;
using Aetheris.Desktop.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Renci.SshNet;

namespace Aetheris.Desktop.ViewModels;

public partial class SshViewModel : ViewModelBase
{
    private SshService? _sshService;
    private VaultService? _vaultService;

    [ObservableProperty]
    private bool _isBusy;

    [ObservableProperty]
    private bool _isConnected;

    [ObservableProperty]
    private string _connectionStatus = "Disconnected";

    [ObservableProperty]
    private string _terminalOutput = string.Empty;

    [ObservableProperty]
    private string _terminalInput = string.Empty;

    [ObservableProperty]
    private ObservableCollection<ConnectionViewModel> _savedConnections = new();

    [ObservableProperty]
    private ConnectionViewModel? _selectedConnection;

    [ObservableProperty]
    private string _commandHistory = string.Empty;

    public SshViewModel() { }

    public void Initialize(SshService sshService, VaultService vaultService)
    {
        _sshService = sshService;
        _vaultService = vaultService;

        _sshService.ConnectionStatusChanged += OnConnectionStatusChanged;
        _sshService.TerminalOutputReceived += OnTerminalOutputReceived;
        _sshService.ConnectionClosed += OnConnectionClosed;

        LoadSavedConnections();
    }

    private void OnConnectionStatusChanged(object? sender, string status)
    {
        ConnectionStatus = status;
        IsConnected = status == "Connected";
    }

    private void OnTerminalOutputReceived(object? sender, string output)
    {
        TerminalOutput += output;
    }

    private void OnConnectionClosed(object? sender, EventArgs e)
    {
        IsConnected = false;
        ConnectionStatus = "Disconnected";
        TerminalOutput += "\n[Connection closed]\n";
    }

    private void LoadSavedConnections()
    {
        if (_vaultService == null || !_vaultService.IsUnlocked) return;

        var connections = _vaultService.GetAllItems()
            .Where(item => item.Type == "ssh")
            .Select(item => new ConnectionViewModel(item));

        SavedConnections = new ObservableCollection<ConnectionViewModel>(connections);
    }

    [RelayCommand]
    private async Task ConnectAsync()
    {
        if (_sshService == null || SelectedConnection == null) return;

        IsBusy = true;
        try
        {
            var vaultItem = _vaultService?.GetItem(SelectedConnection.Id);
            if (vaultItem != null)
            {
                await _sshService.ConnectAsync(
                    vaultItem.Host,
                    vaultItem.Port,
                    vaultItem.Username,
                    vaultItem.Password);
            }
        }
        catch (Exception ex)
        {
            TerminalOutput += $"\n[Error: {ex.Message}]\n";
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task DisconnectAsync()
    {
        if (_sshService == null) return;

        await _sshService.DisconnectAsync();
        TerminalOutput += "\n[Disconnected]\n";
    }

    [RelayCommand]
    private async Task SendCommandAsync()
    {
        if (_sshService == null || string.IsNullOrWhiteSpace(TerminalInput)) return;

        var command = TerminalInput.Trim();
        TerminalInput = string.Empty;

        if (!string.IsNullOrEmpty(command))
        {
            await _sshService.SendCommandAsync(command);
            CommandHistory += command + "\n";
        }
    }

    [RelayCommand]
    private async Task NewConnectionAsync()
    {
        // This will be triggered from MainViewModel
        await Task.CompletedTask;
    }

    [RelayCommand]
    private async Task SaveConnectionAsync()
    {
        // TODO: Save current connection settings
        await Task.CompletedTask;
    }

    public void HandleKeyPress(string key)
    {
        if (key == "Enter")
        {
            SendCommandCommand.Execute(null);
        }
        else if (key == "Backspace")
        {
            if (TerminalInput.Length > 0)
                TerminalInput = TerminalInput[..^1];
        }
        else
        {
            TerminalInput += key;
        }
    }
}

public partial class ConnectionViewModel : ViewModelBase
{
    [ObservableProperty]
    private Guid _id;

    [ObservableProperty]
    private string _name = string.Empty;

    [ObservableProperty]
    private string _host = string.Empty;

    [ObservableProperty]
    private int _port = 22;

    [ObservableProperty]
    private string _username = string.Empty;

    public ConnectionViewModel() { }

    public ConnectionViewModel(VaultItem item)
    {
        Id = item.Id;
        Name = item.Name;
        Host = item.Host;
        Port = item.Port;
        Username = item.Username;
    }
}