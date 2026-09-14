using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Aetheris.Core;

namespace Aetheris.Desktop.Services;

public class VaultService
{
    private readonly Vault _vault;
    private bool _isUnlocked = false;

    public event EventHandler? VaultUnlocked;
    public event EventHandler? VaultLocked;
    public event EventHandler<VaultItem>? ItemAdded;
    public event EventHandler<VaultItem>? ItemUpdated;
    public event EventHandler<Guid>? ItemRemoved;

    public bool IsUnlocked => _isUnlocked;

    public VaultService()
    {
        // Initialize vault - this would be configured with actual vault path
        _vault = new Vault("aetheris.vault");
    }

    public async Task<bool> UnlockWithPasswordAsync(string password)
    {
        try
        {
            await _vault.UnlockAsync(password);
            _isUnlocked = true;
            VaultUnlocked?.Invoke(this, EventArgs.Empty);
            return true;
        }
        catch
        {
            return false;
        }
    }

    public async Task UnlockAsync()
    {
        // This would use the master key from secure storage
        await _vault.UnlockAsync();
        _isUnlocked = true;
        VaultUnlocked?.Invoke(this, EventArgs.Empty);
    }

    public async Task LockAsync()
    {
        await _vault.LockAsync();
        _isUnlocked = false;
        VaultLocked?.Invoke(this, EventArgs.Empty);
    }

    public async Task AddItemAsync(VaultItem item)
    {
        if (!_isUnlocked) throw new InvalidOperationException("Vault is locked");

        await _vault.AddItemAsync(item);
        ItemAdded?.Invoke(this, item);
    }

    public async Task UpdateItemAsync(VaultItem item)
    {
        if (!_isUnlocked) throw new InvalidOperationException("Vault is locked");

        await _vault.UpdateItemAsync(item);
        ItemUpdated?.Invoke(this, item);
    }

    public async Task RemoveItemAsync(Guid id)
    {
        if (!_isUnlocked) throw new InvalidOperationException("Vault is locked");

        await _vault.RemoveItemAsync(id);
        ItemRemoved?.Invoke(this, id);
    }

    public VaultItem GetItem(Guid id)
    {
        if (!_isUnlocked) throw new InvalidOperationException("Vault is locked");

        return _vault.GetItem(id);
    }

    public IReadOnlyList<VaultItem> GetAllItems()
    {
        if (!_isUnlocked) throw new InvalidOperationException("Vault is locked");

        return _vault.GetAllItems();
    }

    public async Task ChangeMasterPasswordAsync(string oldPassword, string newPassword)
    {
        if (!_isUnlocked) throw new InvalidOperationException("Vault is locked");

        await _vault.ChangeMasterPasswordAsync(oldPassword, newPassword);
    }

    // Zeroize keys on disconnect/lock
    public void ZeroizeKeys()
    {
        // This would zeroize all in-memory keys
        // Implementation depends on Aetheris.Core
        _vault.Zeroize();
    }
}