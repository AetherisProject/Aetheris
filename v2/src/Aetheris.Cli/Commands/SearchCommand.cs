using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class SearchCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<QUERY>")]
        public string? Query { get; set; }
        
        [CommandOption("--type <TYPE>")]
        [CommandOption("-t <TYPE>")]
        public string? TypeFilter { get; set; }
        
        [CommandOption("--tag <TAG>")]
        public string? TagFilter { get; set; }
    }
    
    public SearchCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var settings = (Settings)CommandSettings;
        
        if (string.IsNullOrEmpty(settings.Query))
        {
            AnsiConsole.MarkupLine("[red]Error: Search query is required[/]");
            return 1;
        }
        
        var masterPassword = ReadMasterPassword(confirm: false);
        
        try
        {
            using var store = GetVaultStore(masterPassword);
            
            var query = settings.Query.ToLower();
            var items = store.Items;
            
            // Apply type filter
            if (!string.IsNullOrEmpty(settings.TypeFilter))
            {
                if (Enum.TryParse<ItemType>(settings.TypeFilter, true, out var typeFilter))
                {
                    items = items.Where(i => i.Type == typeFilter).ToList();
                }
            }
            
            // Apply tag filter
            if (!string.IsNullOrEmpty(settings.TagFilter))
            {
                items = items.Where(i => i.Tags.Contains(settings.TagFilter, StringComparer.OrdinalIgnoreCase)).ToList();
            }
            
            // Search in title, id, tags, and payload (for some types)
            var results = items.Where(i => 
                i.Title.ToLower().Contains(query) ||
                i.Id.ToLower().Contains(query) ||
                i.Tags.Any(t => t.ToLower().Contains(query)) ||
                SearchInPayload(i, query)).ToList();
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    query = settings.Query,
                    results = results.Select(i => new
                    {
                        id = i.Id,
                        type = i.Type.ToString(),
                        title = i.Title,
                        tags = i.Tags,
                        matchScore = CalculateMatchScore(i, query)
                    }),
                    count = results.Count
                });
            }
            else
            {
                if (results.Count == 0)
                {
                    AnsiConsole.MarkupLine($"[dim]No results found for '{settings.Query}'[/]");
                }
                else
                {
                    AnsiConsole.MarkupLine($"[bold]Search results for '{settings.Query}':[/]");
                    AnsiConsole.WriteLine();
                    
                    var table = new Table();
                    table.AddColumn("ID");
                    table.AddColumn("Type");
                    table.AddColumn("Title");
                    table.AddColumn("Tags");
                    table.AddColumn("Score");
                    
                    foreach (var item in results.OrderByDescending(i => CalculateMatchScore(i, query)))
                    {
                        var score = CalculateMatchScore(item, query);
                        table.AddRow(
                            item.Id[..8],
                            SharedUtilities.FormatItemType(item.Type),
                            item.Title,
                            string.Join(", ", item.Tags),
                            score > 0 ? $"[green]{score}%[/]" : "0%");
                    }
                    
                    AnsiConsole.Write(table);
                    AnsiConsole.MarkupLine($"[dim]— {results.Count} result(s)[/]");
                }
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to search: {ex.Message}[/]");
            return 1;
        }
    }
    
    private static bool SearchInPayload(VaultItem item, string query)
    {
        try
        {
            // For some item types, we can search in the payload
            return item.Type switch
            {
                ItemType.Note => item.Payload<NotePayload>().Body.ToLower().Contains(query),
                ItemType.Password => 
                    item.Payload<PasswordPayload>().Username.ToLower().Contains(query) ||
                    (item.Payload<PasswordPayload>().Notes?.ToLower().Contains(query) ?? false),
                ItemType.ApiKey => 
                    item.Payload<ApiKeyPayload>().Provider.ToLower().Contains(query),
                ItemType.Identity => 
                    item.Payload<IdentityPayload>().Name.ToLower().Contains(query) ||
                    item.Payload<IdentityPayload>().Email.ToLower().Contains(query),
                _ => false
            };
        }
        catch
        {
            return false;
        }
    }
    
    private static int CalculateMatchScore(VaultItem item, string query)
    {
        var score = 0;
        
        // Title match
        if (item.Title.ToLower().Contains(query))
        {
            score += 50;
            if (item.Title.ToLower().StartsWith(query))
                score += 30;
            if (item.Title.ToLower() == query)
                score += 20;
        }
        
        // ID match
        if (item.Id.ToLower().Contains(query))
        {
            score += 40;
            if (item.Id.ToLower().StartsWith(query))
                score += 25;
        }
        
        // Tag match
        if (item.Tags.Any(t => t.ToLower().Contains(query)))
        {
            score += 30;
        }
        
        // Payload match
        if (SearchInPayload(item, query))
        {
            score += 20;
        }
        
        return Math.Min(score, 100);
    }
}