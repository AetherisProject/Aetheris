using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Aetheris.Core;
using Aetheris.Desktop.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace Aetheris.Desktop.ViewModels;

public partial class AddItemDialogViewModel : ViewModelBase
{
    private readonly VaultService? _vaultService;

    [ObservableProperty]
    private string _name = string.Empty;

    [ObservableProperty]
    private string _username = string.Empty;

    [ObservableProperty]
    private string _password = string.Empty;

    [ObservableProperty]
    private string _host = string.Empty;

    [ObservableProperty]
    private int _port = 22;

    [ObservableProperty]
    private string _selectedType = "ssh";

    public List<string> ItemTypes { get; } = new() { "ssh", "password", "note", "credit-card" };

    public AddItemDialogViewModel(VaultService vaultService)
    {
        _vaultService = vaultService;
    }

    [RelayCommand]
    private async Task SaveAsync()
    {
        if (_vaultService == null) return;

        if (string.IsNullOrWhiteSpace(Name))
        {
            // TODO: Show error
            return;
        }

        var item = new VaultItem
        {
            Id = Guid.NewGuid(),
            Name = Name,
            Type = SelectedType,
            Username = Username,
            Password = Password,
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