using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class RemoveCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<ID|TITLE>")]
        public string? IdOrTitle { get; set; }
        
        [CommandOption("--force")]
        [CommandOption("-f")]
        public bool Force { get; set; }
    }
    
    public RemoveCommand(Settings settings) : base(settings) { }
    
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
            
            if (!settings.Force)
            {
                AnsiConsole.MarkupLine($"[yellow]Are you sure you want to remove '{item.Title}' ({item.Id[..8]})?[/]");
                AnsiConsole.Markup("Type 'yes' to confirm: ");
                var confirmation = Console.ReadLine();
                if (confirmation?.ToLower() != "yes")
                {
                    AnsiConsole.MarkupLine("[dim]Cancelled[/]");
                    return 0;
                }
            }
            
            var removed = store.Remove(item.Id);
            
            if (!removed)
            {
                AnsiConsole.MarkupLine($"[red]Failed to remove item '{settings.IdOrTitle}'[/]");
                return 1;
            }
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    success = true,
                    removedId = item.Id,
                    title = item.Title
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[green]✓ removed {SharedUtilities.FormatItemType(item.Type)} '{item.Title}' ({item.Id[..8]})[/]");
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to remove item: {ex.Message}[/]");
            return 1;
        }
    }
}