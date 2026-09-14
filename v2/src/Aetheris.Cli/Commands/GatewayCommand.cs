using System.Net.Http;
using System.Net.Http.Headers;
using System.Text.Json;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Aetheris.Cli.Commands;

public class GatewayCommand : BaseCommand
{
    public class Settings : BaseCommand.Settings
    {
        [CommandArgument(0, "<ACTION>")]
        public string? Action { get; set; }
        
        [CommandOption("--gateway <URL>")]
        public string GatewayUrl { get; set; } = "http://127.0.0.1:7474";
        
        [CommandOption("--token <TOKEN>")]
        public string? Token { get; set; }
        
        [CommandOption("--model <MODEL>")]
        public string? Model { get; set; }
        
        [CommandOption("--limit <LIMIT>")]
        public int? Limit { get; set; }
    }
    
    public GatewayCommand(Settings settings) : base(settings) { }
    
    public override async Task<int> ExecuteAsync(CommandContext context)
    {
        var settings = (Settings)CommandSettings;
        
        if (string.IsNullOrEmpty(settings.Action))
        {
            AnsiConsole.MarkupLine("[red]Error: Action is required (models|budgets|logs|health)[/]");
            return 1;
        }
        
        var action = settings.Action.ToLower();
        
        try
        {
            using var httpClient = new HttpClient();
            
            var token = settings.Token ?? Environment.GetEnvironmentVariable("AETHERIS_GATEWAY_TOKEN");
            if (!string.IsNullOrEmpty(token))
            {
                httpClient.DefaultRequestHeaders.Authorization = new AuthenticationHeaderValue("Bearer", token);
            }
            
            string url = $"{settings.GatewayUrl}/v1/{action}";
            
            if (action == "models" && !string.IsNullOrEmpty(settings.Model))
            {
                url += $"/{settings.Model}";
            }
            else if (action == "budgets" && settings.Limit.HasValue)
            {
                url += $"?limit={settings.Limit}";
            }
            
            var response = await httpClient.GetAsync(url);
            
            if (!response.IsSuccessStatusCode)
            {
                AnsiConsole.MarkupLine($"[red]Request failed: {response.StatusCode}[/]");
                var errorContent = await response.Content.ReadAsStringAsync();
                AnsiConsole.MarkupLine($"[red]{errorContent}[/]");
                return 1;
            }
            
            var content = await response.Content.ReadAsStringAsync();
            
            if (CommandSettings.JsonOutput)
            {
                OutputJson(new
                {
                    action,
                    data = JsonDocument.Parse(content)
                });
                return 0;
            }
            
            switch (action)
            {
                case "models":
                    return await DisplayModelsAsync(content);
                case "budgets":
                    return await DisplayBudgetsAsync(content);
                case "logs":
                    return await DisplayLogsAsync(content);
                case "health":
                    return await DisplayHealthAsync(content);
                default:
                    AnsiConsole.MarkupLine($"[red]Unknown action: {action}[/]");
                    return 1;
            }
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Gateway request failed: {ex.Message}[/]");
            return 1;
        }
    }
    
    private async Task<int> DisplayModelsAsync(string content)
    {
        try
        {
            var jsonDoc = JsonDocument.Parse(content);
            var models = jsonDoc.RootElement.GetProperty("models").EnumerateArray();
            
            AnsiConsole.MarkupLine("[bold]Available Models:[/]");
            AnsiConsole.WriteLine();
            
            var table = new Table();
            table.AddColumn("Name");
            table.AddColumn("Provider");
            table.AddColumn("Type");
            table.AddColumn("Status");
            
            foreach (var model in models)
            {
                var name = model.GetProperty("name").GetString() ?? "unknown";
                var provider = model.GetProperty("provider").GetString() ?? "unknown";
                var type = model.GetProperty("type").GetString() ?? "unknown";
                var status = model.GetProperty("status").GetString() ?? "unknown";
                
                var statusColor = status switch
                {
                    "available" => "[green]",
                    "busy" => "[yellow]",
                    "unavailable" => "[red]",
                    _ => "[dim]"
                };
                
                table.AddRow(name, provider, type, $"{statusColor}{status}[/]");
            }
            
            AnsiConsole.Write(table);
            return 0;
        }
        catch
        {
            AnsiConsole.MarkupLine($"[dim]{content}[/]");
            return 0;
        }
    }
    
    private async Task<int> DisplayBudgetsAsync(string content)
    {
        try
        {
            var jsonDoc = JsonDocument.Parse(content);
            var budgets = jsonDoc.RootElement.GetProperty("budgets").EnumerateArray();
            
            AnsiConsole.MarkupLine("[bold]Budget Usage:[/]");
            AnsiConsole.WriteLine();
            
            var table = new Table();
            table.AddColumn("Model");
            table.AddColumn("Used");
            table.AddColumn("Limit");
            table.AddColumn("Percentage");
            table.AddColumn("Bar");
            
            foreach (var budget in budgets)
            {
                var model = budget.GetProperty("model").GetString() ?? "unknown";
                var used = budget.GetProperty("used").GetInt64();
                var limit = budget.GetProperty("limit").GetInt64();
                var percentage = limit > 0 ? (int)((double)used / limit * 100) : 0;
                
                var bar = CreateProgressBar(percentage, 10);
                var percentageColor = percentage switch
                {
                    < 50 => "[green]",
                    < 80 => "[yellow]",
                    _ => "[red]"
                };
                
                table.AddRow(
                    model,
                    used.ToString("N0"),
                    limit > 0 ? limit.ToString("N0") : "unlimited",
                    $"{percentageColor}{percentage}%[/]",
                    bar);
            }
            
            AnsiConsole.Write(table);
            return 0;
        }
        catch
        {
            AnsiConsole.MarkupLine($"[dim]{content}[/]");
            return 0;
        }
    }
    
    private async Task<int> DisplayLogsAsync(string content)
    {
        try
        {
            var jsonDoc = JsonDocument.Parse(content);
            var logs = jsonDoc.RootElement.GetProperty("logs").EnumerateArray();
            
            AnsiConsole.MarkupLine("[bold]Gateway Logs:[/]");
            AnsiConsole.WriteLine();
            
            var table = new Table();
            table.AddColumn("Timestamp");
            table.AddColumn("Level");
            table.AddColumn("Message");
            
            foreach (var log in logs)
            {
                var timestamp = log.GetProperty("timestamp").GetString() ?? "unknown";
                var level = log.GetProperty("level").GetString() ?? "info";
                var message = log.GetProperty("message").GetString() ?? "";
                
                var levelColor = level switch
                {
                    "error" => "[red]",
                    "warn" => "[yellow]",
                    "info" => "[blue]",
                    "debug" => "[dim]",
                    _ => "[white]"
                };
                
                table.AddRow(timestamp, $"{levelColor}{level}[/]", message);
            }
            
            AnsiConsole.Write(table);
            return 0;
        }
        catch
        {
            AnsiConsole.MarkupLine($"[dim]{content}[/]");
            return 0;
        }
    }
    
    private async Task<int> DisplayHealthAsync(string content)
    {
        try
        {
            var jsonDoc = JsonDocument.Parse(content);
            var status = jsonDoc.RootElement.GetProperty("status").GetString() ?? "unknown";
            var version = jsonDoc.RootElement.GetProperty("version").GetString() ?? "unknown";
            var uptime = jsonDoc.RootElement.GetProperty("uptime").GetString() ?? "unknown";
            
            AnsiConsole.MarkupLine("[bold]Gateway Health:[/]");
            AnsiConsole.WriteLine();
            
            var statusColor = status switch
            {
                "healthy" => "[green]",
                "degraded" => "[yellow]",
                "unhealthy" => "[red]",
                _ => "[dim]"
            };
            
            AnsiConsole.MarkupLine($"  Status:   {statusColor}{status}[/]");
            AnsiConsole.MarkupLine($"  Version:  {version}");
            AnsiConsole.MarkupLine($"  Uptime:   {uptime}");
            
            return 0;
        }
        catch
        {
            AnsiConsole.MarkupLine($"[dim]{content}[/]");
            return 0;
        }
    }
    
    private static string CreateProgressBar(int percentage, int width)
    {
        var filled = (int)(percentage / 100.0 * width);
        var empty = width - filled;
        return $"[green]{new string('█', filled)}[/][dim]{new string('░', empty)}[/]";
    }
}