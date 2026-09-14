using System.Diagnostics;
using Aetheris.Core.Vault;
using Spectre.Console;
using TextCopy;

namespace Aetheris.Cli.Commands;

public static class SharedUtilities
{
    private static readonly Clipboard _clipboard = new();
    private static readonly Dictionary<string, VaultStore> _vaultCache = new();
    
    public static string GetVaultPath(string? vaultPath, string defaultPath = "aetheris.vault")
    {
        return vaultPath ?? defaultPath;
    }
    
    public static VaultStore GetVaultStore(string vaultPath, string masterPassword)
    {
        var cacheKey = vaultPath;
        if (_vaultCache.TryGetValue(cacheKey, out var store))
        {
            return store;
        }
        
        if (File.Exists(vaultPath))
        {
            store = VaultStore.Unlock(vaultPath, masterPassword);
        }
        else
        {
            store = VaultStore.Create(vaultPath, masterPassword);
        }
        
        _vaultCache[cacheKey] = store;
        return store;
    }
    
    public static string ReadMasterPassword(bool confirm)
    {
        var existing = Environment.GetEnvironmentVariable("AETHERIS_PASS");
        if (!string.IsNullOrEmpty(existing)) return existing;
        
        var first = ReadSecret("master password");
        if (!confirm) return first;
        
        var second = ReadSecret("confirm master password");
        if (first != second) throw new InvalidOperationException("passwords do not match");
        return first;
    }
    
    public static string ReadSecret(string label)
    {
        AnsiConsole.Write($"\n{label}: ");
        var sb = new System.Text.StringBuilder();
        while (true)
        {
            var key = Console.ReadKey(intercept: true);
            if (key.Key == ConsoleKey.Enter) break;
            if (key.Key == ConsoleKey.Backspace && sb.Length > 0) sb.Length--;
            else if (!char.IsControl(key.KeyChar)) sb.Append(key.KeyChar);
        }
        AnsiConsole.WriteLine();
        var value = sb.ToString();
        if (value.Length == 0) throw new InvalidOperationException($"{label} must not be empty");
        return value;
    }
    
    public static async Task CopyToClipboardWithAutoClear(string text, int clearAfterSeconds = 45)
    {
        try
        {
            await _clipboard.SetTextAsync(text);
            AnsiConsole.MarkupLine("[green]✓ Copied to clipboard[/]");
            AnsiConsole.MarkupLine($"[dim]Auto-clearing in {clearAfterSeconds}s...[/]");
            
            // Start auto-clear task
            _ = Task.Run(async () =>
            {
                await Task.Delay(clearAfterSeconds * 1000);
                try
                {
                    await _clipboard.SetTextAsync("");
                    AnsiConsole.MarkupLine("[dim]Clipboard cleared[/]");
                }
                catch { /* Ignore clipboard clear failures */ }
            });
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to copy to clipboard: {ex.Message}[/]");
        }
    }
    
    public static string RedactSecrets(string json, bool reveal = false)
    {
        if (reveal) return json;
        
        // Simple redaction for common secret fields
        var fieldsToRedact = new[] { "Password", "Key", "Secret", "Token", "ApiKey", "PrivateKey" };
        
        foreach (var field in fieldsToRedact)
        {
            json = System.Text.RegularExpressions.Regex.Replace(
                json, 
                $@"\"{field}\":\s*\"[^\"]*\"",
                $@"\"{field}\":\"[REDACTED]\"");
        }
        
        return json;
    }
    
    public static string FormatItemType(ItemType type)
    {
        return type switch
        {
            ItemType.Password => "Password",
            ItemType.ApiKey => "ApiKey",
            ItemType.Note => "Note",
            ItemType.Card => "Card",
            ItemType.Identity => "Identity",
            ItemType.SshKey => "SSH Key",
            ItemType.SshConnection => "SSH Connection",
            _ => type.ToString()
        };
    }
    
    public static void ClearVaultCache()
    {
        foreach (var store in _vaultCache.Values)
        {
            store.Dispose();
        }
        _vaultCache.Clear();
    }
}