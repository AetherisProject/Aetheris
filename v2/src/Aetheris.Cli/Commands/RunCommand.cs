using System.Diagnostics;
using Spectre.Console;
using Spectre.Console.Cli;
using Aetheris.Core.Vault;

namespace Aetheris.Cli.Commands;

public class RunCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandOption("--env <ENV>")]
        [CommandOption("-e <ENV>")]
        public string[]? EnvVariables { get; set; }
        
        [CommandOption("--workdir <DIR>")]
        [CommandOption("-w <DIR>")]
        public string? WorkingDirectory { get; set; }
        
        [CommandOption("--shell")]
        public bool UseShell { get; set; } = true;
        
        [CommandArgument(0, "<COMMAND>")]
        public string? Command { get; set; }
        
        [CommandArgument(1, "<ARGS>")]
        public string[]? Args { get; set; }
    }
    
    public RunCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var settings = (Settings)CommandSettings;
        
        if (settings.EnvVariables == null || settings.EnvVariables.Length == 0)
        {
            AnsiConsole.MarkupLine("[red]Error: At least one --env variable is required[/]");
            return 1;
        }
        
        if (string.IsNullOrEmpty(settings.Command))
        {
            AnsiConsole.MarkupLine("[red]Error: Command is required[/]");
            return 1;
        }
        
        var masterPassword = ReadMasterPassword(confirm: false);
        
        try
        {
            using var store = GetVaultStore(masterPassword);
            
            var envDict = new Dictionary<string, string>();
            
            foreach (var envSpec in settings.EnvVariables)
            {
                var eq = envSpec.IndexOf('=');
                if (eq < 1)
                {
                    AnsiConsole.MarkupLine($"[red]Invalid env format: {envSpec}. Use NAME=<id|title>[/]");
                    return 1;
                }
                
                var envName = envSpec[..eq];
                var itemRef = envSpec[(eq + 1)..];
                
                var item = store.Find(itemRef);
                if (item == null)
                {
                    AnsiConsole.MarkupLine($"[red]Item '{itemRef}' not found[/]");
                    return 1;
                }
                
                string value = item.Type switch
                {
                    ItemType.ApiKey => item.Payload<ApiKeyPayload>().Key,
                    ItemType.Password => item.Payload<PasswordPayload>().Password,
                    ItemType.Note => item.Payload<NotePayload>().Body,
                    ItemType.Card => item.Payload<CardPayload>().Number,
                    ItemType.Identity => item.Payload<IdentityPayload>().Email,
                    ItemType.SshKey => item.Payload<SshKeyPayload>().PrivateKey,
                    ItemType.SshConnection => $"{item.Payload<SshConnectionPayload>().Username}@{item.Payload<SshConnectionPayload>().Host}",
                    _ => item.PayloadJson
                };
                
                envDict[envName] = value;
            }
            
            // Build command line
            var commandLine = new List<string> { settings.Command };
            if (settings.Args != null)
            {
                commandLine.AddRange(settings.Args);
            }
            
            var psi = new ProcessStartInfo
            {
                FileName = settings.UseShell ? "/bin/sh" : settings.Command,
                Arguments = settings.UseShell ? $"-c '{string.Join(" ", commandLine.Select(a => a.Replace("'", "'\\''")))}'" : string.Join(" ", settings.Args ?? Array.Empty<string>()),
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                WorkingDirectory = settings.WorkingDirectory ?? Directory.GetCurrentDirectory()
            };
            
            // Add environment variables
            foreach (var (key, value) in envDict)
            {
                psi.Environment[key] = value;
            }
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    command = settings.Command,
                    args = settings.Args,
                    envCount = envDict.Count,
                    workingDirectory = psi.WorkingDirectory
                });
            }
            else
            {
                AnsiConsole.MarkupLine($"[dim]— aeth: injecting {envDict.Count} environment variable(s) into child process[/]");
                foreach (var (key, _) in envDict)
                {
                    AnsiConsole.MarkupLine($"[dim]  {key}=<from vault>[/]");
                }
            }
            
            using var process = Process.Start(psi);
            if (process == null)
            {
                AnsiConsole.MarkupLine("[red]Failed to start process[/]");
                return 1;
            }
            
            // Read output
            var output = await process.StandardOutput.ReadToEndAsync();
            var error = await process.StandardError.ReadToEndAsync();
            
            if (!string.IsNullOrEmpty(output))
            {
                Console.Write(output);
            }
            
            if (!string.IsNullOrEmpty(error))
            {
                Console.Error.Write(error);
            }
            
            await process.WaitForExitAsync();
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    exitCode = process.ExitCode
                });
            }
            
            return process.ExitCode;
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Failed to run command: {ex.Message}[/]");
            return 1;
        }
    }
}