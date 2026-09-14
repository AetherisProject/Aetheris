using System.Text.Json.Serialization;

namespace Aetheris.Gateway;

// ---- Streaming models for OpenAI-compatible SSE ----

public sealed record ChatCompletionChunk(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("created")] long Created,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("choices")] List<ChatCompletionChoice> Choices);

public sealed record ChatCompletionChoice(
    [property: JsonPropertyName("index")] int Index,
    [property: JsonPropertyName("delta")] ChatMessage Delta,
    [property: JsonPropertyName("finish_reason")] string? FinishReason);

// ---- Provider health models ----

public sealed record ProviderHealthStatus(
    [property: JsonPropertyName("provider")] string Provider,
    [property: JsonPropertyName("healthy")] bool Healthy,
    [property: JsonPropertyName("latency_ms")] long? LatencyMs,
    [property: JsonPropertyName("last_check")] DateTimeOffset? LastCheck,
    [property: JsonPropertyName("error")] string? Error);

public sealed record GatewayHealthStatus(
    [property: JsonPropertyName("ok")] bool Ok,
    [property: JsonPropertyName("service")] string Service,
    [property: JsonPropertyName("providers")] Dictionary<string, ProviderHealthStatus> Providers);

// ---- Budget models ----

public sealed record BudgetConfig(
    [property: JsonPropertyName("tokens_per_day")] long TokensPerDay,
    [property: JsonPropertyName("usd_per_token")] decimal? UsdPerToken,
    [property: JsonPropertyName("alert_at_percent")] int AlertAtPercent = 80);

public sealed record BudgetStatus(
    [property: JsonPropertyName("alias")] string Alias,
    [property: JsonPropertyName("cap_per_day")] long CapPerDay,
    [property: JsonPropertyName("spent_today")] long SpentToday,
    [property: JsonPropertyName("usd_estimated")] decimal? UsdEstimated,
    [property: JsonPropertyName("percent_used")] double PercentUsed,
    [property: JsonPropertyName("alert")] bool Alert);

// ---- Price table for cost estimation ----
public sealed record TokenPrice(
    [property: JsonPropertyName("provider")] string Provider,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("prompt_token_usd")] decimal PromptTokenUsd,
    [property: JsonPropertyName("completion_token_usd")] decimal CompletionTokenUsd);