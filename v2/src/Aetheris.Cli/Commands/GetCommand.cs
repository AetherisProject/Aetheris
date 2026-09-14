using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class GetCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<ID|TITLE>")]
        public string? IdOrTitle { get; set; }
        
        [CommandOption("--show")]
        public bool ShowPayload { get; set; }
        
        [CommandOption("--copy")]
        public bool CopyToClipboard { get; set; }
        
        [CommandOption("--field <FIELD>")]
        public string? Field { get; set; }
    }
    
    public GetCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var settings = (Settings)CommandSettings;
        
        if (string.IsNullOrEmpty(settings.IdOrTitle))
        {
            AnsiConsole.MarkupLine("[red]Error: ID or title is required[/]");
            return 1;
        }
        
        var masterPassword = ReadMasterPassword(confirm: false);
        
        try
        {
            using var store = GetVaultStore(masterPassword);
            
            var item = store.Find(settings.IdOrTitle);
            if (item == null)
            {
                AnsiConsole.MarkupLine($"[red]Item '{settings.IdOrTitle}' not found[/]");
                return 1;
            }
            
            if (CommandSettings.JsonOutput)
            {
                var json = item.PayloadJson;
                if (!CommandSettings.RevealSecrets)
                {
                    json = SharedUtilities.RedactSecrets(json);
                }
                Console.WriteLine(json);
                return 0;
            }
            
            // Display basic info
            AnsiConsole.MarkupLine($"{SharedUtilities.FormatItemType(item.Type)} '[bold]{item.Title}[/]'");
            AnsiConsole.MarkupLine($"[dim]ID: {item.Id}[/]");
            AnsiConsole.MarkupLine($"[dim]Type: {item.Type}[/]");
            AnsiConsole.MarkupLine($"[dim]Tags: {string.Join(", ", item.Tags)}[/]");
            AnsiConsole.MarkupLine($"[dim]Created: {item.CreatedAt:yyyy-MM-dd HH:mm:ss}[/]");
            AnsiConsole.MarkupLine($"[dim]Updated: {item.UpdatedAt:yyyy-MM-dd HH:mm:ss}[/]");
            
            if (settings.ShowPayload || settings.CopyToClipboard || !string.IsNullOrEmpty(settings.Field))
            {
                string payloadToShow = settings.Field != null ? ExtractField(item, settings.Field) : item.PayloadJson;
                
                if (settings.CopyToClipboard)
                {
                    await SharedUtilities.CopyToClipboardWithAutoClear(payloadToShow);
                }
                else
                {
                    AnsiConsole.MarkupLine("[bold]Payload:[/]");
                    AnsiConsole.WriteLine(payloadToShow);
                }
            }
            else
            {
                AnsiConsole.MarkupLine("[dim](payload hidden — use --show, --copy, or --field)[/]");
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to get item: {ex.Message}[/]");
            return 1;
        }
    }
    
    private static string ExtractField(VaultItem item, string field)
    {
        try
        {
            var jsonDoc = System.Text.Json.JsonDocument.Parse(item.PayloadJson);
            if (jsonDoc.RootElement.TryGetProperty(field, out var value))
            {
                return value.ToString();
            }
            return "Field not found: " + field;
        }
        catch
        {
            return "Failed to extract field: " + field;
        }
    }
}