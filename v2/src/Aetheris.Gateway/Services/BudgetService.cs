using System.Collections.Concurrent;
using System.Text.Json;
using Aetheris.Gateway;
namespace Aetheris.Gateway.Services;

/// <summary>
/// Enhanced budget tracking with per-alias per-day token caps and USD cost estimation.
/// </summary>
public sealed class BudgetService : IDisposable
{
    private sealed record DaySpend(string Day, long Spent);
    private readonly string _path;
    private readonly Dictionary<string, BudgetConfig> _configs;
    private readonly ConcurrentDictionary<string, DaySpend> _spend = new();
    private readonly ConcurrentDictionary<string, long> _tokenCounts = new();
    private readonly object _persistLock = new();
    private bool _disposed;

    // Price table for cost estimation (USD per token)
    private static readonly Dictionary<string, TokenPrice> DefaultPrices = new(StringComparer.OrdinalIgnoreCase)
    {
        // OpenAI prices
        ["openai:gpt-4o"] = new TokenPrice("openai", "gpt-4o", 0.000015m, 0.00006m),
        ["openai:gpt-4o-mini"] = new TokenPrice("openai", "gpt-4o-mini", 0.000003m, 0.000012m),
        ["openai:gpt-4"] = new TokenPrice("openai", "gpt-4", 0.00003m, 0.00006m),
        
        // Anthropic prices
        ["anthropic:claude-opus-4-1"] = new TokenPrice("anthropic", "claude-opus-4-1", 0.000015m, 0.000075m),
        ["anthropic:claude-sonnet-4"] = new TokenPrice("anthropic", "claude-sonnet-4", 0.000003m, 0.000015m),
        
        // Local/Ollama - free
        ["ollama:llama3.1"] = new TokenPrice("ollama", "llama3.1", 0m, 0m),
        ["ollama:qwen2.5"] = new TokenPrice("ollama", "qwen2.5", 0m, 0m),
    };

    private readonly Dictionary<string, TokenPrice> _prices;

    public BudgetService(string path, Dictionary<string, long> tokensPerDay, Dictionary<string, TokenPrice>? customPrices = null)
    {
        _path = path;
        _prices = customPrices ?? new Dictionary<string, TokenPrice>(DefaultPrices, StringComparer.OrdinalIgnoreCase);
        
        // Convert tokens per day to budget configs
        _configs = tokensPerDay.ToDictionary(
            kv => kv.Key,
            kv => new BudgetConfig { TokensPerDay = kv.Value, AlertAtPercent = 80 }
        );
        
        // Load persisted spend data
        if (File.Exists(path))
        {
            try
            {
                var persisted = JsonSerializer.Deserialize<Dictionary<string, DaySpend>>(File.ReadAllText(path));
                if (persisted is not null)
                    foreach (var (k, v) in persisted) _spend[k] = v;
            }
            catch { /* corrupt file, start fresh */ }
        }
    }

    private static string Today => DateTimeOffset.UtcNow.ToString("yyyy-MM-dd");

    /// <summary>Check if the alias may proceed with the estimated tokens.</summary>
    public BudgetCheckResult Allow(string alias, long estimatedTokens)
    {
        if (!_configs.TryGetValue(alias, out var config) || config.TokensPerDay == 0)
            return new BudgetCheckResult(true, 0, null);
        
        var spend = _spend.GetOrAdd(alias, _ => new DaySpend(Today, 0));
        if (spend.Day != Today) spend = new DaySpend(Today, 0);
        
        var totalSpent = spend.Spent + estimatedTokens;
        var percentUsed = (double)totalSpent / config.TokensPerDay * 100;
        var alert = percentUsed >= config.AlertAtPercent;
        
        return new BudgetCheckResult(
            totalSpent <= config.TokensPerDay,
            percentUsed,
            alert ? "budget_alert" : null
        );
    }

    /// <summary>Report actual token usage for an alias.</summary>
    public void Report(string alias, long promptTokens, long completionTokens)
    {
        var totalTokens = promptTokens + completionTokens;
        _tokenCounts.AddOrUpdate(alias, totalTokens, (_, old) => old + totalTokens);
        
        var spend = _spend.GetOrAdd(alias, _ => new DaySpend(Today, 0));
        spend = new DaySpend(Today, spend.Spent + totalTokens);
        _spend[alias] = spend;
        
        Persist();
    }

    /// <summary>Get USD cost estimate for token usage.</summary>
    public decimal EstimateUsdCost(string alias, long promptTokens, long completionTokens)
    {
        if (!_configs.TryGetValue(alias, out var config))
            return 0m;
        
        // Find the price for this alias
        var price = FindPriceForAlias(alias);
        if (price == null)
            return 0m;
        
        return promptTokens * price.PromptTokenUsd + completionTokens * price.CompletionTokenUsd;
    }

    private TokenPrice? FindPriceForAlias(string alias)
    {
        if (_configs.TryGetValue(alias, out var config))
        {
            // Try to find exact match
            if (_prices.TryGetValue($"{alias}", out var price))
                return price;
            
            // Try provider:model format
            if (config.Provider != null && config.Model != null)
            {
                var key = $"{config.Provider}:{config.Model}";
                if (_prices.TryGetValue(key, out price))
                    return price;
            }
        }
        
        return null;
    }

    public object Snapshot() => _configs.ToDictionary(
        kv => kv.Key,
        kv => new
        {
            capPerDay = kv.Value.TokensPerDay,
            spentToday = _spend.TryGetValue(kv.Key, out var s) && s.Day == Today ? s.Spent : 0,
            usdEstimated = EstimateUsdCost(kv.Key, 
                _spend.TryGetValue(kv.Key, out var spend) && spend.Day == Today ? spend.Spent : 0, 0),
            percentUsed = kv.Value.TokensPerDay > 0 ? 
                (double)(_spend.TryGetValue(kv.Key, out var s) && s.Day == Today ? s.Spent : 0) / kv.Value.TokensPerDay * 100 : 0,
            alert = kv.Value.TokensPerDay > 0 && 
                   (_spend.TryGetValue(kv.Key, out var s2) && s2.Day == Today ? s2.Spent : 0) / (double)kv.Value.TokensPerDay * 100 >= kv.Value.AlertAtPercent
        });

    private void Persist()
    {
        if (_disposed) return;
        
        lock (_persistLock)
        {
            try
            {
                Directory.CreateDirectory(Path.GetDirectoryName(_path) ?? ".");
                File.WriteAllText(_path, JsonSerializer.Serialize(_spend));
            }
            catch { /* budgets are best-effort */ }
        }
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        Persist();
        GC.SuppressFinalize(this);
    }
}

public sealed record BudgetCheckResult(bool Allowed, double PercentUsed, string? Alert);

public sealed record BudgetConfig
{
    public long TokensPerDay { get; set; } = 0;
    public int AlertAtPercent { get; set; } = 80;
    public string? Provider { get; set; }
    public string? Model { get; set; }
}

public sealed record TokenPrice
{
    public string Provider { get; set; } = "";
    public string Model { get; set; } = "";
    public decimal PromptTokenUsd { get; set; }
    public decimal CompletionTokenUsd { get; set; }
    
    public TokenPrice(string provider, string model, decimal promptTokenUsd, decimal completionTokenUsd)
    {
        Provider = provider;
        Model = model;
        PromptTokenUsd = promptTokenUsd;
        CompletionTokenUsd = completionTokenUsd;
    }
}