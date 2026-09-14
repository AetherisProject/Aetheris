using System.Diagnostics;
using System.Text.Json.Nodes;
using Aetheris.Gateway;
namespace Aetheris.Gateway.Services;

/// <summary>
/// Performs periodic health checks on all providers using cheap models.
/// Exposes health status via GET /v1/gateway/health
/// </summary>
public sealed class HealthCheckService : IDisposable
{
    private Dictionary<string, ILlmProvider> _providers = new();
    private IKeyring _keyring = null!;
    private IHttpClientFactory _httpFactory = null!;
    private ILogger _logger = null!;
    private readonly Dictionary<string, ProviderHealthStatus> _status = new();
    private readonly object _statusLock = new();
    private Timer? _timer;
    private bool _disposed;

    // Cheap models for health checks per provider
    private static readonly Dictionary<string, string> CheapModels = new(StringComparer.OrdinalIgnoreCase)
    {
        ["openai"] = "gpt-4o-mini",
        ["anthropic"] = "claude-sonnet-4",
        ["ollama"] = "llama3.1",
        ["mistral"] = "mistral-tiny",
        ["google"] = "gemini-1.5-flash",
        ["openrouter"] = "openai/gpt-4o-mini",
        ["nvidia"] = "mistralai/mistral-tiny"
    };

    public void Initialize(
        Dictionary<string, ILlmProvider> providers,
        IKeyring keyring,
        IHttpClientFactory httpFactory,
        ILogger logger)
    {
        _providers = providers;
        _keyring = keyring;
        _httpFactory = httpFactory;
        _logger = logger;

        // Initialize all providers as unknown
        foreach (var provider in _providers.Keys)
        {
            _status[provider] = new ProviderHealthStatus
            {
                Provider = provider,
                Healthy = false,
                LastCheck = null,
                Error = "Not checked yet"
            };
        }
    }

    public void Start(int intervalMinutes = 15)
    {
        _timer = new Timer(_ => CheckAllProvidersAsync(), null, TimeSpan.Zero, TimeSpan.FromMinutes(intervalMinutes));
        _logger.LogInformation("Health check service started with {Interval} minute interval", intervalMinutes);
    }

    private async void CheckAllProvidersAsync()
    {
        try
        {
            await CheckAllProviders();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error during health check cycle");
        }
    }

    private async Task CheckAllProviders()
    {
        var http = _httpFactory.CreateClient();
        var tasks = new List<Task>();

        foreach (var (providerId, provider) in _providers)
        {
            tasks.Add(Task.Run(async () =>
            {
                var sw = Stopwatch.StartNew();
                try
                {
                    var apiKey = _keyring.GetKey(providerId);
                    if (provider.NeedsKey && string.IsNullOrEmpty(apiKey))
                    {
                        UpdateStatus(providerId, false, sw.ElapsedMilliseconds, "No API key configured");
                        return;
                    }

                    if (!CheapModels.TryGetValue(providerId, out var model))
                    {
                        UpdateStatus(providerId, false, sw.ElapsedMilliseconds, "No cheap model configured for health check");
                        return;
                    }

                    // Create a minimal health check request
                    var request = new ChatRequest
                    {
                        Model = model,
                        Messages = new List<ChatMessage> { new("user", "ping") },
                        MaxTokens = 1,
                        Stream = false
                    };

                    var result = await provider.ChatAsync(http, model, request, apiKey, CancellationToken.None);
                    
                    if (result.Status >= 200 && result.Status < 300)
                    {
                        UpdateStatus(providerId, true, sw.ElapsedMilliseconds, null);
                    }
                    else
                    {
                        UpdateStatus(providerId, false, sw.ElapsedMilliseconds, 
                            $"HTTP {result.Status}: {result.Error ?? "Unknown error"}");
                    }
                }
                catch (Exception ex)
                {
                    UpdateStatus(providerId, false, sw.ElapsedMilliseconds, ex.Message);
                }
            }));
        }

        await Task.WhenAll(tasks);
    }

    private void UpdateStatus(string providerId, bool healthy, long latencyMs, string? error)
    {
        lock (_statusLock)
        {
            _status[providerId] = new ProviderHealthStatus
            {
                Provider = providerId,
                Healthy = healthy,
                LatencyMs = latencyMs,
                LastCheck = DateTimeOffset.UtcNow,
                Error = error
            };
        }
    }

    public GatewayHealthStatus GetStatus()
    {
        lock (_statusLock)
        {
            var providerStatuses = new Dictionary<string, ProviderHealthStatus>();
            foreach (var (key, status) in _status)
            {
                providerStatuses[key] = status;
            }
            
            return new GatewayHealthStatus
            {
                Ok = _status.Values.All(s => s.Healthy),
                Service = "aetheris-gateway",
                Providers = providerStatuses
            };
        }
    }

    public ProviderHealthStatus? GetProviderStatus(string providerId)
    {
        lock (_statusLock)
        {
            return _status.TryGetValue(providerId, out var status) ? status : null;
        }
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;

        _timer?.Dispose();
        GC.SuppressFinalize(this);
    }
}