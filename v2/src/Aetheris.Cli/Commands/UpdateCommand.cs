using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class UpdateCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<ID|TITLE>")]
        public string? IdOrTitle { get; set; }
        
        [CommandOption("--title <TITLE>")]
        [CommandOption("-t <TITLE>")]
        public string? Title { get; set; }
        
        [CommandOption("--tag <TAG>")]
        public string[]? Tags { get; set; }
        
        [CommandOption("--add-tag <TAG>")]
        public string[]? AddTags { get; set; }
        
        [CommandOption("--remove-tag <TAG>")]
        public string[]? RemoveTags { get; set; }
        
        [CommandOption("--favorite")]
        [CommandOption("-f")]
        public bool? Favorite { get; set; }
        
        [CommandOption("--user <USERNAME>")]
        public string? Username { get; set; }
        
        [CommandOption("--password <PASSWORD>")]
        public string? Password { get; set; }
        
        [CommandOption("--provider <PROVIDER>")]
        public string? Provider { get; set; }
        
        [CommandOption("--key <KEY>")]
        public string? Key { get; set; }
        
        [CommandOption("--body <BODY>")]
        public string? Body { get; set; }
        
        [CommandOption("--number <NUMBER>")]
        public string? Number { get; set; }
        
        [CommandOption("--expiry <EXPIRY>")]
        public string? Expiry { get; set; }
        
        [CommandOption("--cvv <CVV>")]
        public string? Cvv { get; set; }
        
        [CommandOption("--holder <HOLDER>")]
        public string? Holder { get; set; }
        
        [CommandOption("--name <NAME>")]
        public string? Name { get; set; }
        
        [CommandOption("--email <EMAIL>")]
        public string? Email { get; set; }
        
        [CommandOption("--phone <PHONE>")]
        public string? Phone { get; set; }
        
        [CommandOption("--address <ADDRESS>")]
        public string? Address { get; set; }
        
        [CommandOption("--host <HOST>")]
        public string? Host { get; set; }
        
        [CommandOption("--port <PORT>")]
        public int? Port { get; set; }
        
        [CommandOption("--private-key <PRIVATE_KEY>")]
        public string? PrivateKey { get; set; }
        
        [CommandOption("--public-key <PUBLIC_KEY>")]
        public string? PublicKey { get; set; }
        
        [CommandOption("--fingerprint <FINGERPRINT>")]
        public string? Fingerprint { get; set; }
        
        [CommandOption("--comment <COMMENT>")]
        public string? Comment { get; set; }
    }
    
    public UpdateCommand(Settings settings) : base(settings) { }
    
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
            
            // Update basic properties
            var updatedTitle = settings.Title ?? item.Title;
            var updatedTags = UpdateTags(item.Tags, settings.Tags, settings.AddTags, settings.RemoveTags);
            var updatedFavorite = settings.Favorite ?? item.Favorite;
            
            // Update payload based on item type
            var updatedPayloadJson = UpdatePayload(item, settings);
            
            var updatedItem = item with
            {
                Title = updatedTitle,
                Tags = updatedTags,
                Favorite = updatedFavorite,
                PayloadJson = updatedPayloadJson,
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
                    type = updatedItem.Type.ToString(),
                    updatedAt = updatedItem.UpdatedAt
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[green]✓ updated {SharedUtilities.FormatItemType(updatedItem.Type)} '{updatedItem.Title}' ({updatedItem.Id[..8]})[/]");
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to update item: {ex.Message}[/]");
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
    
    private static string UpdatePayload(VaultItem item, Settings settings)
    {
        return item.Type switch
        {
            ItemType.Password => UpdatePasswordPayload(item, settings),
            ItemType.ApiKey => UpdateApiKeyPayload(item, settings),
            ItemType.Note => UpdateNotePayload(item, settings),
            ItemType.Card => UpdateCardPayload(item, settings),
            ItemType.Identity => UpdateIdentityPayload(item, settings),
            ItemType.SshKey => UpdateSshKeyPayload(item, settings),
            ItemType.SshConnection => UpdateSshConnectionPayload(item, settings),
            _ => item.PayloadJson
        };
    }
    
    private static string UpdatePasswordPayload(VaultItem item, Settings settings)
    {
        var payload = item.Payload<PasswordPayload>();
        var updated = new PasswordPayload(
            settings.Username ?? payload.Username,
            settings.Password ?? payload.Password,
            payload.Urls,
            settings.Totp ?? payload.Totp,
            settings.Notes ?? payload.Notes
        );
        return System.Text.Json.JsonSerializer.Serialize(updated, VaultJson.Options);
    }
    
    private static string UpdateApiKeyPayload(VaultItem item, Settings settings)
    {
        var payload = item.Payload<ApiKeyPayload>();
        var updated = new ApiKeyPayload(
            settings.Provider ?? payload.Provider,
            settings.Key ?? payload.Key,
            payload.Scopes,
            payload.ExpiresAt,
            payload.RotationPolicy
        );
        return System.Text.Json.JsonSerializer.Serialize(updated, VaultJson.Options);
    }
    
    private static string UpdateNotePayload(VaultItem item, Settings settings)
    {
        var payload = item.Payload<NotePayload>();
        var updated = new NotePayload(settings.Body ?? payload.Body);
        return System.Text.Json.JsonSerializer.Serialize(updated, VaultJson.Options);
    }
    
    private static string UpdateCardPayload(VaultItem item, Settings settings)
    {
        var payload = item.Payload<CardPayload>();
        var updated = new CardPayload(
            settings.Number ?? payload.Number,
            settings.Expiry ?? payload.Expiry,
            settings.Cvv ?? payload.Cvv,
            settings.Holder ?? payload.Holder
        );
        return System.Text.Json.JsonSerializer.Serialize(updated, VaultJson.Options);
    }
    
    private static string UpdateIdentityPayload(VaultItem item, Settings settings)
    {
        var payload = item.Payload<IdentityPayload>();
        var updated = new IdentityPayload(
            settings.Name ?? payload.Name,
            settings.Email ?? payload.Email,
            settings.Phone ?? payload.Phone,
            settings.Address ?? payload.Address
        );
        return System.Text.Json.JsonSerializer.Serialize(updated, VaultJson.Options);
    }
    
    private static string UpdateSshKeyPayload(VaultItem item, Settings settings)
    {
        var payload = item.Payload<SshKeyPayload>();
        var updated = new SshKeyPayload(
            settings.PrivateKey ?? payload.PrivateKey,
            settings.PublicKey ?? payload.PublicKey,
            settings.Fingerprint ?? payload.Fingerprint,
            settings.Comment ?? payload.Comment
        );
        return System.Text.Json.JsonSerializer.Serialize(updated, VaultJson.Options);
    }
    
    private static string UpdateSshConnectionPayload(VaultItem item, Settings settings)
    {
        var payload = item.Payload<SshConnectionPayload>();
        var updated = new SshConnectionPayload(
            settings.Host ?? payload.Host,
            settings.Port ?? payload.Port,
            settings.Username ?? payload.Username,
            settings.PrivateKey ?? payload.KeyId
        );
        return System.Text.Json.JsonSerializer.Serialize(updated, VaultJson.Options);
    }
}