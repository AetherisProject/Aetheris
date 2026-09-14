using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class InitCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandOption("--force")]
        public bool Force { get; set; }
    }
    
    public InitCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var vaultPath = GetVaultPath();
        var settings = (Settings)CommandSettings;
        
        if (File.Exists(vaultPath) && !settings.Force)
        {
            AnsiConsole.MarkupLine($"[red]Vault already exists: {vaultPath}[/]");
            return 1;
        }
        
        if (File.Exists(vaultPath) && settings.Force)
        {
            AnsiConsole.MarkupLine($"[yellow]Warning: Overwriting existing vault at {vaultPath}[/]");
        }
        
        var masterPassword = ReadMasterPassword(confirm: true);
        
        try
        {
            using var store = VaultStore.Create(vaultPath, masterPassword);
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    success = true,
                    vaultPath,
                    generation = store.Generation
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[green]✓ vault created: {vaultPath}[/]");
                AnsiConsole.MarkupLine($"[dim]generation: {store.Generation}[/]");
            }
            
            return 0;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to create vault: {ex.Message}[/]");
            return 1;
        }
    }
}