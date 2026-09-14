using System;
using System.Collections.ObjectModel;
using System.Linq;
using System.Threading.Tasks;
using Aetheris.Core;
using Aetheris.Desktop.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace Aetheris.Desktop.ViewModels;

public partial class VaultViewModel : ViewModelBase
{
    private VaultService? _vaultService;

    [ObservableProperty]
    private bool _isBusy;

    [ObservableProperty]
    private string _searchText = string.Empty;

    [ObservableProperty]
    private ObservableCollection<VaultItemViewModel> _items = new();

    [ObservableProperty]
    private VaultItemViewModel? _selectedItem;

    public VaultViewModel()
    {
        // Will be initialized later with service
    }

    public void Initialize(VaultService vaultService)
    {
        _vaultService = vaultService;
        LoadItems();
        
        _vaultService.VaultUnlocked += OnVaultUnlocked;
        _vaultService.VaultLocked += OnVaultLocked;
        _vaultService.ItemAdded += OnItemAdded;
        _vaultService.ItemUpdated += OnItemUpdated;
        _vaultService.ItemRemoved += OnItemRemoved;
    }

    private void OnVaultUnlocked(object? sender, EventArgs e)
    {
        LoadItems();
    }

    private void OnVaultLocked(object? sender, EventArgs e)
    {
        Items.Clear();
    }

    private void OnItemAdded(object? sender, VaultItem item)
    {
        Items.Add(new VaultItemViewModel(item));
    }

    private void OnItemUpdated(object? sender, VaultItem item)
    {
        var existing = Items.FirstOrDefault(i => i.Id == item.Id);
        if (existing != null)
        {
            existing.UpdateFrom(item);
        }
    }

    private void OnItemRemoved(object? sender, Guid id)
    {
        var existing = Items.FirstOrDefault(i => i.Id == id);
        if (existing != null)
        {
            Items.Remove(existing);
        }
    }

    private void LoadItems()
    {
        if (_vaultService == null || !_vaultService.IsUnlocked) return;

        IsBusy = true;
        try
        {
            var vaultItems = _vaultService.GetAllItems();
            Items = new ObservableCollection<VaultItemViewModel>(
                vaultItems.Select(item => new VaultItemViewModel(item)));
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task AddItemAsync()
    {
        if (_vaultService == null) return;

        // This will be triggered from MainViewModel
        await Task.CompletedTask;
    }

    [RelayCommand]
    private async Task EditItemAsync()
    {
        if (SelectedItem == null || _vaultService == null) return;

        // TODO: Show edit dialog
        await Task.CompletedTask;
    }

    [RelayCommand]
    private async Task DeleteItemAsync()
    {
        if (SelectedItem == null || _vaultService == null) return;

        if (await ShowDeleteConfirmationAsync())
        {
            await _vaultService.RemoveItemAsync(SelectedItem.Id);
        }
    }

    [RelayCommand]
    private void CopyPassword()
    {
        if (SelectedItem == null) return;

        // TODO: Copy to clipboard
        // Clipboard.SetTextAsync(SelectedItem.Password);
    }

    private async Task<bool> ShowDeleteConfirmationAsync()
    {
        // TODO: Show confirmation dialog
        return true;
    }

    public void FilterItems(string searchText)
    {
        SearchText = searchText;
        // TODO: Implement filtering
    }
}

public partial class VaultItemViewModel : ViewModelBase
{
    [ObservableProperty]
    private Guid _id;

    [ObservableProperty]
    private string _name = string.Empty;

    [ObservableProperty]
    private string _type = string.Empty;

    [ObservableProperty]
    private string _username = string.Empty;

    [ObservableProperty]
    private string _host = string.Empty;

    [ObservableProperty]
    private int _port;

    [ObservableProperty]
    private DateTimeOffset _createdAt;

    [ObservableProperty]
    private DateTimeOffset _updatedAt;

    public VaultItemViewModel() { }

    public VaultItemViewModel(VaultItem item)
    {
        UpdateFrom(item);
    }

    public void UpdateFrom(VaultItem item)
    {
        Id = item.Id;
        Name = item.Name;
        Type = item.Type;
        Username = item.Username;
        Host = item.Host;
        Port = item.Port;
        CreatedAt = item.CreatedAt;
        UpdatedAt = item.UpdatedAt;
    }
}