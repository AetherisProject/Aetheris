using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class AddCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<TYPE>")]
        public string? Type { get; set; }
        
        [CommandOption("--title <TITLE>")]
        [CommandOption("-t <TITLE>")]
        public string? Title { get; set; }
        
        [CommandOption("--user <USERNAME>")]
        [CommandOption("-u <USERNAME>")]
        public string? Username { get; set; }
        
        [CommandOption("--provider <PROVIDER>")]
        [CommandOption("-p <PROVIDER>")]
        public string? Provider { get; set; }
        
        [CommandOption("--url <URL>")]
        public string? Url { get; set; }
        
        [CommandOption("--totp <TOTP>")]
        public string? Totp { get; set; }
        
        [CommandOption("--notes <NOTES>")]
        public string? Notes { get; set; }
        
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
        public int Port { get; set; } = 22;
        
        [CommandOption("--private-key <PRIVATE_KEY>")]
        public string? PrivateKey { get; set; }
        
        [CommandOption("--public-key <PUBLIC_KEY>")]
        public string? PublicKey { get; set; }
        
        [CommandOption("--fingerprint <FINGERPRINT>")]
        public string? Fingerprint { get; set; }
        
        [CommandOption("--comment <COMMENT>")]
        public string? Comment { get; set; }
        
        [CommandOption("--tag <TAG>")]
        public string[]? Tags { get; set; }
    }
    
    public AddCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var settings = (Settings)CommandSettings;
        
        if (string.IsNullOrEmpty(settings.Type))
        {
            AnsiConsole.MarkupLine("[red]Error: Item type is required[/]");
            return 1;
        }
        
        if (string.IsNullOrEmpty(settings.Title))
        {
            AnsiConsole.MarkupLine("[red]Error: Title is required[/]");
            return 1;
        }
        
        var masterPassword = ReadMasterPassword(confirm: false);
        
        try
        {
            using var store = GetVaultStore(masterPassword);
            
            var itemType = ParseItemType(settings.Type);
            var tags = settings.Tags?.ToList() ?? new List<string>();
            
            VaultItem item = itemType switch
            {
                ItemType.Password => store.Add(ItemType.Password, settings.Title, new PasswordPayload(
                    settings.Username ?? "",
                    ReadSecret("password"),
                    settings.Url != null ? new List<string> { settings.Url } : new List<string>(),
                    settings.Totp,
                    settings.Notes)),
                
                ItemType.ApiKey => store.Add(ItemType.ApiKey, settings.Title, new ApiKeyPayload(
                    settings.Provider ?? "",
                    ReadSecret("api key"),
                    new List<string>(),
                    null,
                    null)),
                
                ItemType.Note => store.Add(ItemType.Note, settings.Title, new NotePayload(
                    ReadSecret("note"))),
                
                ItemType.Card => store.Add(ItemType.Card, settings.Title, new CardPayload(
                    settings.Number ?? "",
                    settings.Expiry ?? "",
                    settings.Cvv ?? "",
                    settings.Holder ?? "")),
                
                ItemType.Identity => store.Add(ItemType.Identity, settings.Title, new IdentityPayload(
                    settings.Name ?? "",
                    settings.Email ?? "",
                    settings.Phone,
                    settings.Address)),
                
                ItemType.SshKey => store.Add(ItemType.SshKey, settings.Title, new SshKeyPayload(
                    settings.PrivateKey ?? "",
                    settings.PublicKey,
                    settings.Fingerprint,
                    settings.Comment)),
                
                ItemType.SshConnection => store.Add(ItemType.SshConnection, settings.Title, new SshConnectionPayload(
                    settings.Host ?? "",
                    settings.Port,
                    settings.Username ?? "",
                    settings.PrivateKey)),
                
                _ => throw new ArgumentException("Unknown item type: " + settings.Type)
            };
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    success = true,
                    id = item.Id,
                    type = item.Type.ToString(),
                    title = item.Title
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[green]✓ added {SharedUtilities.FormatItemType(item.Type)} '{item.Title}' ({item.Id[..8]})[/]");
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to add item: {ex.Message}[/]");
            return 1;
        }
    }
    
    private static ItemType ParseItemType(string type)
    {
        return type.ToLower() switch
        {
            "password" or "pass" => ItemType.Password,
            "apikey" or "api-key" or "api" => ItemType.ApiKey,
            "note" or "text" => ItemType.Note,
            "card" or "credit-card" => ItemType.Card,
            "identity" or "id" => ItemType.Identity,
            "sshkey" or "ssh-key" or "sshkey" => ItemType.SshKey,
            "sshconnection" or "ssh-connection" or "sshconn" => ItemType.SshConnection,
            _ => throw new ArgumentException("Unknown item type: " + type)
        };
    }
    
    private string ReadSecret(string label)
    {
        return SharedUtilities.ReadSecret(label);
    }
}