using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class ListCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandOption("--type <TYPE>")]
        [CommandOption("-t <TYPE>")]
        public string? TypeFilter { get; set; }
        
        [CommandOption("--tag <TAG>")]
        public string? TagFilter { get; set; }
        
        [CommandOption("--favorite")]
        [CommandOption("-f")]
        public bool FavoriteOnly { get; set; }
        
        [CommandOption("--search <QUERY>")]
        [CommandOption("-s <QUERY>")]
        public string? SearchQuery { get; set; }
    }
    
    public ListCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var settings = (Settings)CommandSettings;
        var masterPassword = ReadMasterPassword(confirm: false);
        
        try
        {
            using var store = GetVaultStore(masterPassword);
            
            var items = store.Items;
            
            // Apply filters
            if (!string.IsNullOrEmpty(settings.TypeFilter))
            {
                if (Enum.TryParse<ItemType>(settings.TypeFilter, true, out var typeFilter))
                {
                    items = items.Where(i => i.Type == typeFilter).ToList();
                }
            }
            
            if (!string.IsNullOrEmpty(settings.TagFilter))
            {
                items = items.Where(i => i.Tags.Contains(settings.TagFilter, StringComparer.OrdinalIgnoreCase)).ToList();
            }
            
            if (settings.FavoriteOnly)
            {
                items = items.Where(i => i.Favorite).ToList();
            }
            
            if (!string.IsNullOrEmpty(settings.SearchQuery))
            {
                var query = settings.SearchQuery.ToLower();
                items = items.Where(i => 
                    i.Title.ToLower().Contains(query) ||
                    i.Id.ToLower().Contains(query) ||
                    string.Join(",", i.Tags).ToLower().Contains(query)).ToList();
            }
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    items = items.Select(i => new
                    {
                        id = i.Id,
                        type = i.Type.ToString(),
                        title = i.Title,
                        tags = i.Tags,
                        favorite = i.Favorite,
                        createdAt = i.CreatedAt,
                        updatedAt = i.UpdatedAt
                    }),
                    count = items.Count,
                    generation = store.Generation
                });
            }
            else
            {
                if (items.Count == 0)
                {
                    AnsiConsole.MarkupLine("[dim]No items found[/]");
                }
                else
                {
                    var table = new Table();
                    table.AddColumn("ID");
                    table.AddColumn("Type");
                    table.AddColumn("Title");
                    table.AddColumn("Tags");
                    table.AddColumn("Favorite");
                    
                    foreach (var item in items)
                    {
                        table.AddRow(
                            item.Id[..8],
                            SharedUtilities.FormatItemType(item.Type),
                            item.Title,
                            string.Join(", ", item.Tags),
                            item.Favorite ? "[yellow]★[/]" : "");
                    }
                    
                    AnsiConsole.Write(table);
                    AnsiConsole.MarkupLine($"[dim]— {items.Count} item(s), generation {store.Generation}[/]");
                }
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to list items: {ex.Message}[/]");
            return 1;
        }
    }
}