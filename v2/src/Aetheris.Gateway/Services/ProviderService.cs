using System.Text.Json;

namespace Aetheris.Gateway.Services;

/// <summary>
/// Provider catalog with benchmark endpoints and capabilities.
/// Ported from KEY-BITCHER design.
/// </summary>
public sealed class ProviderService
{
    private readonly Dictionary<string, ProviderConfig> _providers;

    public ProviderService()
    {
        _providers = new Dictionary<string, ProviderConfig>(StringComparer.OrdinalIgnoreCase)
        {
            ["openai"] = new ProviderConfig
            {
                Id = "openai",
                Name = "OpenAI",
                BaseUrl = "https://api.openai.com/v1",
                ChatEndpoint = "/chat/completions",
                SupportsStreaming = true,
                SupportsVision = true,
                SupportsFunctionCalling = true,
                DefaultModel = "gpt-4o-mini",
                Models = new Dictionary<string, ProviderModel>
                {
                    ["gpt-4o"] = new ProviderModel { Name = "GPT-4o", Type = "chat", ContextLength = 128000, MaxOutput = 4096 },
                    ["gpt-4o-mini"] = new ProviderModel { Name = "GPT-4o Mini", Type = "chat", ContextLength = 128000, MaxOutput = 16384 },
                    ["gpt-4"] = new ProviderModel { Name = "GPT-4", Type = "chat", ContextLength = 128000, MaxOutput = 4096 },
                    ["gpt-3.5-turbo"] = new ProviderModel { Name = "GPT-3.5 Turbo", Type = "chat", ContextLength = 16384, MaxOutput = 4096 },
                },
                BenchmarkScores = new ProviderBenchmarks
                {
                    Reasoning = 95,
                    Coding = 92,
                    Creativity = 90,
                    Speed = 85,
                    CostEfficiency = 70
                }
            },
            
            ["anthropic"] = new ProviderConfig
            {
                Id = "anthropic",
                Name = "Anthropic",
                BaseUrl = "https://api.anthropic.com/v1",
                ChatEndpoint = "/messages",
                SupportsStreaming = true,
                SupportsVision = true,
                SupportsFunctionCalling = true,
                DefaultModel = "claude-sonnet-4",
                Models = new Dictionary<string, ProviderModel>
                {
                    ["claude-opus-4-1"] = new ProviderModel { Name = "Claude Opus 4.1", Type = "chat", ContextLength = 200000, MaxOutput = 64000 },
                    ["claude-sonnet-4"] = new ProviderModel { Name = "Claude Sonnet 4", Type = "chat", ContextLength = 200000, MaxOutput = 4096 },
                    ["claude-3-haiku"] = new ProviderModel { Name = "Claude 3 Haiku", Type = "chat", ContextLength = 200000, MaxOutput = 4096 },
                },
                BenchmarkScores = new ProviderBenchmarks
                {
                    Reasoning = 98,
                    Coding = 95,
                    Creativity = 93,
                    Speed = 80,
                    CostEfficiency = 85
                }
            },
            
            ["ollama"] = new ProviderConfig
            {
                Id = "ollama",
                Name = "Ollama",
                BaseUrl = "http://127.0.0.1:11434/v1",
                ChatEndpoint = "/chat/completions",
                SupportsStreaming = true,
                SupportsVision = false,
                SupportsFunctionCalling = false,
                DefaultModel = "llama3.1",
                NeedsKey = false,
                Models = new Dictionary<string, ProviderModel>
                {
                    ["llama3.1"] = new ProviderModel { Name = "Llama 3.1", Type = "chat", ContextLength = 128000, MaxOutput = 8192 },
                    ["qwen2.5"] = new ProviderModel { Name = "Qwen 2.5", Type = "chat", ContextLength = 32000, MaxOutput = 8192 },
                    ["mistral"] = new ProviderModel { Name = "Mistral", Type = "chat", ContextLength = 32000, MaxOutput = 8192 },
                },
                BenchmarkScores = new ProviderBenchmarks
                {
                    Reasoning = 75,
                    Coding = 70,
                    Creativity = 78,
                    Speed = 95,
                    CostEfficiency = 100
                }
            },
            
            ["mistral"] = new ProviderConfig
            {
                Id = "mistral",
                Name = "Mistral AI",
                BaseUrl = "https://api.mistral.ai/v1",
                ChatEndpoint = "/chat/completions",
                SupportsStreaming = true,
                SupportsVision = true,
                SupportsFunctionCalling = true,
                DefaultModel = "mistral-large",
                Models = new Dictionary<string, ProviderModel>
                {
                    ["mistral-large"] = new ProviderModel { Name = "Mistral Large", Type = "chat", ContextLength = 128000, MaxOutput = 8192 },
                    ["mistral-small"] = new ProviderModel { Name = "Mistral Small", Type = "chat", ContextLength = 32000, MaxOutput = 4096 },
                    ["mistral-tiny"] = new ProviderModel { Name = "Mistral Tiny", Type = "chat", ContextLength = 32000, MaxOutput = 4096 },
                },
                BenchmarkScores = new ProviderBenchmarks
                {
                    Reasoning = 88,
                    Coding = 85,
                    Creativity = 82,
                    Speed = 88,
                    CostEfficiency = 90
                }
            },
            
            ["google"] = new ProviderConfig
            {
                Id = "google",
                Name = "Google",
                BaseUrl = "https://generativelanguage.googleapis.com/v1beta",
                ChatEndpoint = "/models/{model}:generateContent",
                SupportsStreaming = true,
                SupportsVision = true,
                SupportsFunctionCalling = false,
                DefaultModel = "gemini-1.5-flash",
                Models = new Dictionary<string, ProviderModel>
                {
                    ["gemini-1.5-pro"] = new ProviderModel { Name = "Gemini 1.5 Pro", Type = "chat", ContextLength = 1048576, MaxOutput = 8192 },
                    ["gemini-1.5-flash"] = new ProviderModel { Name = "Gemini 1.5 Flash", Type = "chat", ContextLength = 1048576, MaxOutput = 8192 },
                    ["gemini-1.0-pro"] = new ProviderModel { Name = "Gemini 1.0 Pro", Type = "chat", ContextLength = 32000, MaxOutput = 8192 },
                },
                BenchmarkScores = new ProviderBenchmarks
                {
                    Reasoning = 92,
                    Coding = 88,
                    Creativity = 90,
                    Speed = 92,
                    CostEfficiency = 80
                }
            },
            
            ["openrouter"] = new ProviderConfig
            {
                Id = "openrouter",
                Name = "OpenRouter",
                BaseUrl = "https://openrouter.ai/api/v1",
                ChatEndpoint = "/chat/completions",
                SupportsStreaming = true,
                SupportsVision = true,
                SupportsFunctionCalling = true,
                DefaultModel = "openai/gpt-4o-mini",
                Models = new Dictionary<string, ProviderModel>
                {
                    ["openai/gpt-4o"] = new ProviderModel { Name = "OpenAI GPT-4o", Type = "chat", ContextLength = 128000, MaxOutput = 4096 },
                    ["openai/gpt-4o-mini"] = new ProviderModel { Name = "OpenAI GPT-4o Mini", Type = "chat", ContextLength = 128000, MaxOutput = 16384 },
                    ["anthropic/claude-opus-4-1"] = new ProviderModel { Name = "Anthropic Claude Opus 4.1", Type = "chat", ContextLength = 200000, MaxOutput = 64000 },
                },
                BenchmarkScores = new ProviderBenchmarks
                {
                    Reasoning = 95,
                    Coding = 92,
                    Creativity = 90,
                    Speed = 85,
                    CostEfficiency = 75
                }
            },
            
            ["nvidia"] = new ProviderConfig
            {
                Id = "nvidia",
                Name = "NVIDIA",
                BaseUrl = "https://integrate.api.nvidia.com/v1",
                ChatEndpoint = "/chat/completions",
                SupportsStreaming = true,
                SupportsVision = true,
                SupportsFunctionCalling = false,
                DefaultModel = "mistralai/mistral-large",
                Models = new Dictionary<string, ProviderModel>
                {
                    ["mistralai/mistral-large"] = new ProviderModel { Name = "Mistral Large", Type = "chat", ContextLength = 128000, MaxOutput = 8192 },
                    ["mistralai/mistral-small"] = new ProviderModel { Name = "Mistral Small", Type = "chat", ContextLength = 32000, MaxOutput = 4096 },
                    ["meta/llama3-70b"] = new ProviderModel { Name = "Llama 3 70B", Type = "chat", ContextLength = 128000, MaxOutput = 8192 },
                },
                BenchmarkScores = new ProviderBenchmarks
                {
                    Reasoning = 85,
                    Coding = 82,
                    Creativity = 80,
                    Speed = 88,
                    CostEfficiency = 85
                }
            }
        };
    }

    public IReadOnlyDictionary<string, ProviderConfig> Providers => _providers;

    public ProviderConfig? GetProvider(string id) => _providers.TryGetValue(id, out var provider) ? provider : null;

    public IReadOnlyDictionary<string, ProviderModel> GetModels(string providerId)
    {
        if (_providers.TryGetValue(providerId, out var provider))
            return provider.Models;
        return new Dictionary<string, ProviderModel>();
    }

    public ProviderModel? GetModel(string providerId, string modelId)
    {
        if (_providers.TryGetValue(providerId, out var provider))
            return provider.Models.TryGetValue(modelId, out var model) ? model : null;
        return null;
    }

    public object GetCatalog() => _providers.ToDictionary(
        kv => kv.Key,
        kv => new
        {
            id = kv.Value.Id,
            name = kv.Value.Name,
            default_model = kv.Value.DefaultModel,
            supports_streaming = kv.Value.SupportsStreaming,
            supports_vision = kv.Value.SupportsVision,
            supports_functions = kv.Value.SupportsFunctionCalling,
            needs_key = kv.Value.NeedsKey,
            models = kv.Value.Models.ToDictionary(
                m => m.Key,
                m => new { name = m.Value.Name, type = m.Value.Type, context_length = m.Value.ContextLength, max_output = m.Value.MaxOutput }
            ),
            benchmarks = kv.Value.BenchmarkScores
        }
    );
}

public sealed class ProviderConfig
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string BaseUrl { get; set; } = "";
    public string ChatEndpoint { get; set; } = "";
    public bool SupportsStreaming { get; set; } = true;
    public bool SupportsVision { get; set; } = false;
    public bool SupportsFunctionCalling { get; set; } = false;
    public string DefaultModel { get; set; } = "";
    public bool NeedsKey { get; set; } = true;
    public Dictionary<string, ProviderModel> Models { get; set; } = new();
    public ProviderBenchmarks BenchmarkScores { get; set; } = new();
}

public sealed class ProviderModel
{
    public string Name { get; set; } = "";
    public string Type { get; set; } = "chat";
    public int ContextLength { get; set; } = 0;
    public int MaxOutput { get; set; } = 0;
}

public sealed class ProviderBenchmarks
{
    public int Reasoning { get; set; } = 0;
    public int Coding { get; set; } = 0;
    public int Creativity { get; set; } = 0;
    public int Speed { get; set; } = 0;
    public int CostEfficiency { get; set; } = 0;
}