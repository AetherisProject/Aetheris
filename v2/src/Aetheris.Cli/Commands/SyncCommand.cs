using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Text.Json;
using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class SyncCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<ACTION>")]
        public string? Action { get; set; }
        
        [CommandOption("--hub <URL>")]
        public string HubUrl { get; set; } = "https://hub.aetheris.dev";
        
        [CommandOption("--token <TOKEN>")]
        public string? Token { get; set; }
        
        [CommandOption("--force")]
        public bool Force { get; set; }
    }
    
    public SyncCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var settings = (Settings)CommandSettings;
        
        if (string.IsNullOrEmpty(settings.Action))
        {
            AnsiConsole.MarkupLine("[red]Error: Action is required (push|pull|status)[/]");
            return 1;
        }
        
        var action = settings.Action.ToLower();
        if (action != "push" && action != "pull" && action != "status")
        {
            AnsiConsole.MarkupLine("[red]Error: Invalid action. Use push, pull, or status[/]");
            return 1;
        }
        
        var masterPassword = ReadMasterPassword(confirm: false);
        
        try
        {
            using var store = GetVaultStore(masterPassword);
            
            var token = settings.Token ?? Environment.GetEnvironmentVariable("AETHERIS_TOKEN");
            if (string.IsNullOrEmpty(token))
            {
                AnsiConsole.MarkupLine("[red]Error: Aetheris token is required. Use --token or set AETHERIS_TOKEN environment variable[/]");
                return 1;
            }
            
            using var httpClient = new HttpClient();
            httpClient.DefaultRequestHeaders.Authorization = new AuthenticationHeaderValue("Bearer", token);
            httpClient.DefaultRequestHeaders.Add("X-Aetheris-Token", token);
            
            switch (action)
            {
                case "push":
                    return await PushAsync(httpClient, store, settings);
                case "pull":
                    return await PullAsync(httpClient, store, settings);
                case "status":
                    return await StatusAsync(httpClient, store, settings);
                default:
                    return 1;
            }
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Sync failed: {ex.Message}[/]");
            return 1;
        }
    }
    
    private async Task<int> PushAsync(HttpClient httpClient, VaultStore store, Settings settings)
    {
        AnsiConsole.MarkupLine("[dim]Preparing sync push...[/]");
        
        // Create export for sync
        var tempExportPath = Path.GetTempFileName();
        try
        {
            store.Export(tempExportPath);
            
            var exportContent = await File.ReadAllTextAsync(tempExportPath);
            var content = new StringContent(exportContent, Encoding.UTF8, "application/json");
            
            var response = await httpClient.PostAsync($"{settings.HubUrl}/api/v1/sync/push", content);
            
            if (!response.IsSuccessStatusCode)
            {
                AnsiConsole.MarkupLine($"[red]Push failed: {response.StatusCode}[/]");
                var errorContent = await response.Content.ReadAsStringAsync();
                AnsiConsole.MarkupLine($"[red]{errorContent}[/]");
                return 1;
            }
            
            var result = await response.Content.ReadFromJsonAsync<JsonDocument>();
            var generation = result?.RootElement.GetProperty("generation").GetInt32() ?? store.Generation;
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    success = true,
                    action = "push",
                    generation,
                    itemsCount = store.Items.Count
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[green]✓ sync push successful[/]");
                AnsiConsole.MarkupLine($"[dim]generation: {generation}, items: {store.Items.Count}[/]");
            }
            
            return 0;
        }
        finally
        {
            if (File.Exists(tempExportPath))
            {
                File.Delete(tempExportPath);
            }
        }
    }
    
    private async Task<int> PullAsync(HttpClient httpClient, VaultStore store, Settings settings)
    {
        AnsiConsole.MarkupLine("[dim]Fetching sync data...[/]");
        
        var response = await httpClient.GetAsync($"{settings.HubUrl}/api/v1/sync/pull");
        
        if (!response.IsSuccessStatusCode)
        {
            AnsiConsole.MarkupLine($"[red]Pull failed: {response.StatusCode}[/]");
            var errorContent = await response.Content.ReadAsStringAsync();
            AnsiConsole.MarkupLine($"[red]{errorContent}[/]");
            return 1;
        }
        
        var tempImportPath = Path.GetTempFileName();
        try
        {
            var exportContent = await response.Content.ReadAsStringAsync();
            await File.WriteAllTextAsync(tempImportPath, exportContent);
            
            store.Import(tempImportPath);
            
            var result = await response.Content.ReadFromJsonAsync<JsonDocument>();
            var generation = result?.RootElement.GetProperty("generation").GetInt32() ?? store.Generation;
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    success = true,
                    action = "pull",
                    generation,
                    itemsCount = store.Items.Count
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[green]✓ sync pull successful[/]");
                AnsiConsole.MarkupLine($"[dim]generation: {generation}, items: {store.Items.Count}[/]");
            }
            
            return 0;
        }
        finally
        {
            if (File.Exists(tempImportPath))
            {
                File.Delete(tempImportPath);
            }
        }
    }
    
    private async Task<int> StatusAsync(HttpClient httpClient, VaultStore store, Settings settings)
    {
        AnsiConsole.MarkupLine("[dim]Checking sync status...[/]");
        
        var response = await httpClient.GetAsync($"{settings.HubUrl}/api/v1/sync/status");
        
        if (!response.IsSuccessStatusCode)
        {
            AnsiConsole.MarkupLine($"[red]Status check failed: {response.StatusCode}[/]");
            var errorContent = await response.Content.ReadAsStringAsync();
            AnsiConsole.MarkupLine($"[red]{errorContent}[/]");
            return 1;
        }
        
        var content = await response.Content.ReadAsStringAsync();
        
        if (CommandSettings.JsonOutput)
        {
            OutputJson(new
            {
                status = content
            });
        }
        else
        {
            try
            {
                var jsonDoc = JsonDocument.Parse(content);
                var localGen = store.Generation;
                var remoteGen = jsonDoc.RootElement.GetProperty("generation").GetInt32();
                var status = jsonDoc.RootElement.GetProperty("status").GetString() ?? "unknown";
                
                AnsiConsole.MarkupLine($"[bold]Sync Status:[/]");
                AnsiConsole.MarkupLine($"  Local generation:  {localGen}");
                AnsiConsole.MarkupLine($"  Remote generation: {remoteGen}");
                AnsiConsole.MarkupLine($"  Status:            {status}");
                
                if (localGen < remoteGen)
                {
                    AnsiConsole.MarkupLine("[yellow]⚠ Local vault is behind remote[/]");
                }
                else if (localGen > remoteGen)
                {
                    AnsiConsole.MarkupLine("[yellow]⚠ Local vault is ahead of remote[/]");
                }
                else
                {
                    AnsiConsole.MarkupLine("[green]✓ Vault is in sync[/]");
                }
            }
            catch
            {
                AnsiConsole.MarkupLine($"[dim]{content}[/]");
            }
        }
        
        return 0;
    }
}