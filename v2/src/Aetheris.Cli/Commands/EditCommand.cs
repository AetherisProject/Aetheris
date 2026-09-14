using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class EditCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<ID|TITLE>")]
        public string? IdOrTitle { get; set; }
        
        [CommandOption("--title <TITLE>")]
        public string? Title { get; set; }
        
        [CommandOption("--tag <TAG>")]
        public string[]? Tags { get; set; }
        
        [CommandOption("--add-tag <TAG>")]
        public string[]? AddTags { get; set; }
        
        [CommandOption("--remove-tag <TAG>")]
        public string[]? RemoveTags { get; set; }
        
        [CommandOption("--favorite")]
        public bool? Favorite { get; set; }
    }
    
    public EditCommand(Settings settings) : base(settings) { }
    
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
            
            // For now, edit just updates metadata. For full payload editing, users should use update command
            var updatedTitle = settings.Title ?? item.Title;
            var updatedTags = UpdateTags(item.Tags, settings.Tags, settings.AddTags, settings.RemoveTags);
            var updatedFavorite = settings.Favorite ?? item.Favorite;
            
            var updatedItem = item with
            {
                Title = updatedTitle,
                Tags = updatedTags,
                Favorite = updatedFavorite,
                UpdatedAt = DateTimeOffset.UtcNow
            };
            
            store.Update(updatedItem);
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    success = true,
                    id = updatedItem.Id,
                    title = updatedItem.Title,
                    tags = updatedItem.Tags,
                    favorite = updatedItem.Favorite
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[green]✓ edited {SharedUtilities.FormatItemType(updatedItem.Type)} '{updatedItem.Title}' ({updatedItem.Id[..8]})[/]");
                AnsiConsole.MarkupLine($"[dim]Tags: {string.Join(", ", updatedItem.Tags)}[/]");
                AnsiConsole.MarkupLine($"[dim]Favorite: {updatedItem.Favorite}[/]");
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to edit item: {ex.Message}[/]");
            return 1;
        }
    }
    
    private static List<string> UpdateTags(List<string> currentTags, string[]? newTags, string[]? addTags, string[]? removeTags)
    {
        var tags = new List<string>(currentTags);
        
        if (newTags != null)
        {
            tags = newTags.ToList();
        }
        else
        {
            if (addTags != null)
            {
                foreach (var tag in addTags)
                {
                    if (!tags.Contains(tag, StringComparer.OrdinalIgnoreCase))
                    {
                        tags.Add(tag);
                    }
                }
            }
            
            if (removeTags != null)
            {
                foreach (var tag in removeTags)
                {
                    tags.RemoveAll(t => t.Equals(tag, StringComparison.OrdinalIgnoreCase));
                }
            }
        }
        
        return tags;
    }
}