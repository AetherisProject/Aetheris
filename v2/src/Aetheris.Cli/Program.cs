using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Cli.Commands;

// aeth — Aetheris CLI (W5: Spectre.Console.Cli, clipboard, JSON mode, sync, gateway)
// Usage examples from mockups/cli.html:
//   aeth init --vault keys.vault
//   aeth add apikey --vault keys.vault --provider openai --title "OpenAI prod"
//   aeth ls --vault keys.vault
//   aeth get <id> --show --vault keys.vault
//   aeth get <id> --copy --vault keys.vault
//   aeth run --vault keys.vault --env OPENAI_API_KEY=<id> -- sh -c 'test -n "$OPENAI_API_KEY" && echo injected'
//   AETHERIS_TOKEN=dev aeth sync push --vault keys.vault
//   aeth gateway models
//   aeth gateway budgets
//   aeth env-export --to .env --only openai,anthropic

var app = new CommandApp();

app.Configure(config =>
{
    // Register all commands with their aliases
    config.AddCommand<InitCommand>("init");
    config.AddCommand<AddCommand>("add");
    config.AddCommand<ListCommand>("ls", "list");
    config.AddCommand<GetCommand>("get", "show");
    config.AddCommand<UpdateCommand>("update", "up", "modify");
    config.AddCommand<RemoveCommand>("remove", "rm", "del", "delete");
    config.AddCommand<SearchCommand>("search", "find", "query");
    config.AddCommand<EditCommand>("edit", "ed");
    config.AddCommand<RunCommand>("run", "exec", "execute");
    config.AddCommand<SyncCommand>("sync");
    config.AddCommand<GatewayCommand>("gateway", "gw");
    config.AddCommand<EnvExportCommand>("env-export", "export", "env");
    
    config.SetApplicationName("aeth");
    config.SetApplicationVersion("2.0.0");
    config.CaseSensitivity(CaseSensitivity.None);
    
    // Custom exception handling
    config.AddExceptionHandler<Exception>(ex =>
    {
        AnsiConsole.MarkupLine($"[red]Error: {ex.Message}[/]");
        return 1;
    });
    
    // Configure help
    config.UseDefaultHelpParser();
});

// Handle Ctrl+C gracefully
Console.CancelKeyPress += (sender, e) =>
{
    AnsiConsole.MarkupLine("\n[yellow]Operation cancelled[/]");
    SharedUtilities.ClearVaultCache();
    Environment.Exit(130); // Standard exit code for Ctrl+C
};

try
{
    return await app.RunAsync(args);
}
catch (Exception ex)
{
    AnsiConsole.MarkupLine($"[red]Fatal error: {ex.Message}[/]");
    return 1;
}
finally
{
    SharedUtilities.ClearVaultCache();
}