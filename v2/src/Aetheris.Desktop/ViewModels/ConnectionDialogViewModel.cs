using System;
using System.Threading.Tasks;
using Aetheris.Core;
using Aetheris.Desktop.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace Aetheris.Desktop.ViewModels;

public partial class ConnectionDialogViewModel : ViewModelBase
{
    private readonly VaultService? _vaultService;

    [ObservableProperty]
    private string _name = string.Empty;

    [ObservableProperty]
    private string _host = string.Empty;

    [ObservableProperty]
    private int _port = 22;

    [ObservableProperty]
    private string _username = string.Empty;

    public ConnectionDialogViewModel(VaultService vaultService)
    {
        _vaultService = vaultService;
    }

    [RelayCommand]
    private async Task SaveAsync()
    {
        if (_vaultService == null) return;

        if (string.IsNullOrWhiteSpace(Name) || string.IsNullOrWhiteSpace(Host))
        {
            // TODO: Show error
            return;
        }

        var item = new VaultItem
        {
            Id = Guid.NewGuid(),
            Name = Name,
            Type = "ssh",
            Username = Username,
            Host = Host,
            Port = Port,
            CreatedAt = DateTimeOffset.UtcNow,
            UpdatedAt = DateTimeOffset.UtcNow
        };

        await _vaultService.AddItemAsync(item);
        await CloseDialogAsync(true);
    }

    [RelayCommand]
    private async Task CancelAsync()
    {
        await CloseDialogAsync(false);
    }

    private async Task CloseDialogAsync(bool result)
    {
        await Task.CompletedTask;
    }
}