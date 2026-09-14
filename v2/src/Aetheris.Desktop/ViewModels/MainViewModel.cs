using System;
using System.Collections.ObjectModel;
using System.Reactive;
using System.Threading.Tasks;
using Aetheris.Desktop.Services;
using Aetheris.Desktop.Views;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Interactivity;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.Extensions.DependencyInjection;

namespace Aetheris.Desktop.ViewModels;

public partial class MainViewModel : ViewModelBase
{
    private readonly VaultService _vaultService;
    private readonly SshService _sshService;
    private readonly SftpService _sftpService;
    private readonly GatewayService _gatewayService;
    private readonly IServiceProvider _serviceProvider;

    [ObservableProperty]
    private bool _isVaultUnlocked;

    [ObservableProperty]
    private int _selectedTabIndex;

    [ObservableProperty]
    private string _statusMessage = "Ready";

    [ObservableProperty]
    private string _connectionStatus = "Disconnected";

    [ObservableProperty]
    private string _gatewayStatus = "Stopped";

    public VaultViewModel VaultViewModel { get; }
    public SshViewModel SshViewModel { get; }
    public SftpViewModel SftpViewModel { get; }
    public GatewayViewModel GatewayViewModel { get; }

    public MainViewModel(
        VaultService vaultService,
        SshService sshService,
        SftpService sftpService,
        GatewayService gatewayService,
        VaultViewModel vaultViewModel,
        SshViewModel sshViewModel,
        SftpViewModel sftpViewModel,
        GatewayViewModel gatewayViewModel,
        IServiceProvider serviceProvider)
    {
        _vaultService = vaultService;
        _sshService = sshService;
        _sftpService = sftpService;
        _gatewayService = gatewayService;
        _serviceProvider = serviceProvider;

        VaultViewModel = vaultViewModel;
        SshViewModel = sshViewModel;
        SftpViewModel = sftpViewModel;
        GatewayViewModel = gatewayViewModel;

        // Initialize vault state
        IsVaultUnlocked = _vaultService.IsUnlocked;

        // Setup event handlers
        _vaultService.VaultUnlocked += OnVaultUnlocked;
        _vaultService.VaultLocked += OnVaultLocked;
        _sshService.ConnectionStatusChanged += OnConnectionStatusChanged;
        _gatewayService.StatusChanged += OnGatewayStatusChanged;

        // Setup view model dependencies
        VaultViewModel.Initialize(_vaultService);
        SshViewModel.Initialize(_sshService, _vaultService);
        SftpViewModel.Initialize(_sftpService, _vaultService);
        GatewayViewModel.Initialize(_gatewayService);
    }

    private void OnVaultUnlocked(object? sender, EventArgs e)
    {
        IsVaultUnlocked = true;
        StatusMessage = "Vault unlocked";
        SelectedTabIndex = 0; // Switch to Vault tab
    }

    private void OnVaultLocked(object? sender, EventArgs e)
    {
        IsVaultUnlocked = false;
        StatusMessage = "Vault locked";
        SelectedTabIndex = 0;
    }

    private void OnConnectionStatusChanged(object? sender, string status)
    {
        ConnectionStatus = status;
    }

    private void OnGatewayStatusChanged(object? sender, string status)
    {
        GatewayStatus = status;
    }

    [RelayCommand]
    private async Task LockVaultAsync()
    {
        await _vaultService.LockAsync();
        StatusMessage = "Vault locked";
    }

    public async Task ShowUnlockDialogAsync()
    {
        // This will be handled by the App class
        await Task.CompletedTask;
    }

    public async Task ShowAddItemDialogAsync()
    {
        // This will be handled by the App class
        await Task.CompletedTask;
    }

    public async Task ShowConnectionDialogAsync()
    {
        // This will be handled by the App class
        await Task.CompletedTask;
    }
}