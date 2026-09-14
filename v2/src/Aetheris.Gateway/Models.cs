using System.Text.Json.Serialization;

namespace Aetheris.Gateway;

// ---- OpenAI-compatible wire models (subset; extend in W3) ----

public sealed record ChatMessage(
    [property: JsonPropertyName("role")] string Role,
    [property: JsonPropertyName("content")] string Content);

public sealed record ChatRequest(
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("messages")] List<ChatMessage> Messages,
    [property: JsonPropertyName("max_tokens")] int? MaxTokens,
    [property: JsonPropertyName("stream")] bool? Stream);


// ---- gateway configuration ----

public sealed class GatewayConfig
{
    public string Listen { get; set; } = "http://127.0.0.1:7474";
    public string AccessToken { get; set; } = "dev";
    public string KeyStorePath { get; set; } = "keys.vault";
    public Dictionary<string, AliasConfig> Aliases { get; set; } = new();
    public Dictionary<string, long> BudgetTokensPerDay { get; set; } = new();
}

public sealed class AliasConfig
{
    public string Provider { get; set; } = "openai";
    public string Model { get; set; } = "";
    public string? FallbackTo { get; set; }
}

// ---- redacted runtime log ----

public sealed record GatewayLogEntry(
    DateTimeOffset At,
    string Alias,
    string Provider,
    int Status,
    long LatencyMs,
    long? PromptTokens,
    long? CompletionTokens);
