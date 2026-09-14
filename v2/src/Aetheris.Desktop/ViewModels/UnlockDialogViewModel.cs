using System.Threading.Tasks;
using Aetheris.Desktop.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace Aetheris.Desktop.ViewModels;

public partial class UnlockDialogViewModel : ViewModelBase
{
    private readonly VaultService? _vaultService;

    [ObservableProperty]
    private string _password = string.Empty;

    [ObservableProperty]
    private string _statusMessage = string.Empty;

    [ObservableProperty]
    private bool _hasError;

    public UnlockDialogViewModel(VaultService vaultService)
    {
        _vaultService = vaultService;
    }

    [RelayCommand]
    private async Task UnlockAsync()
    {
        if (_vaultService == null) return;

        if (string.IsNullOrWhiteSpace(Password))
        {
            StatusMessage = "Password is required";
            HasError = true;
            return;
        }

        try
        {
            var success = await _vaultService.UnlockWithPasswordAsync(Password);
            if (!success)
            {
                StatusMessage = "Invalid password";
                HasError = true;
                return;
            }

            // Close dialog with success
            await CloseDialogAsync(true);
        }
        catch (Exception ex)
        {
            StatusMessage = ex.Message;
            HasError = true;
        }
    }

    [RelayCommand]
    private async Task CancelAsync()
    {
        await CloseDialogAsync(false);
    }

    private async Task CloseDialogAsync(bool result)
    {
        // This will be handled by the dialog's ShowDialog call
        await Task.CompletedTask;
    }
}