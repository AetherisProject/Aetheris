using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public abstract class BaseCommand : AsyncCommand
{
    public class Settings : CommandSettings
    {
        [CommandOption("--vault <PATH>")]
        [CommandOption("-v <PATH>")]
        public string? VaultPath { get; set; }
        
        [CommandOption("--json")]
        public bool JsonOutput { get; set; }
        
        [CommandOption("--reveal")]
        public bool RevealSecrets { get; set; }
    }
    
    protected Settings CommandSettings { get; set; }
    
    protected BaseCommand(Settings settings)
    {
        CommandSettings = settings;
    }
    
    protected string GetVaultPath()
    {
        return SharedUtilities.GetVaultPath(CommandSettings.VaultPath);
    }
    
    protected string ReadMasterPassword(bool confirm)
    {
        return SharedUtilities.ReadMasterPassword(confirm);
    }
    
    protected string ReadSecret(string label)
    {
        return SharedUtilities.ReadSecret(label);
    }
    
    protected VaultStore GetVaultStore(string masterPassword)
    {
        return SharedUtilities.GetVaultStore(GetVaultPath(), masterPassword);
    }
    
    protected void OutputJson(object data)
    {
        if (CommandSettings.JsonOutput)
        {
            var json = System.Text.Json.JsonSerializer.Serialize(data, new System.Text.Json.JsonSerializerOptions
            {
                WriteIndented = true,
                PropertyNamingPolicy = System.Text.Json.JsonNamingPolicy.CamelCase
            });
            
            if (!CommandSettings.RevealSecrets)
            {
                json = SharedUtilities.RedactSecrets(json);
            }
            
            Console.WriteLine(json);
        }
    }
}