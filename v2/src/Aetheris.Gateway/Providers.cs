using System.Net.Http.Headers;
using System.Net.Http.Json;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace Aetheris.Gateway;

public sealed record ProviderResult(int Status, JsonNode? Body, string? Error, (long? In, long? Out) Usage);

/// <summary>Adapts the OpenAI-compatible wire format to a provider's API.</summary>
public interface ILlmProvider
{
    string Id { get; }
    bool NeedsKey { get; }
    IReadOnlyDictionary<string, string> KnownModels { get; }
    Task<ProviderResult> ChatAsync(HttpClient http, string model, ChatRequest request, string? apiKey, CancellationToken ct);
}

public abstract class ProviderBase : ILlmProvider
{
    public abstract string Id { get; }
    public virtual bool NeedsKey => true;
    public abstract IReadOnlyDictionary<string, string> KnownModels { get; }
    protected abstract Uri Endpoint { get; }
    protected abstract void Authorize(HttpRequestMessage message, string apiKey);
    protected virtual HttpRequestMessage BuildRequest(string model, ChatRequest request, string apiKey)
    {
        var body = new JsonObject
        {
            ["model"] = model,
            ["messages"] = JsonSerializer.SerializeToNode(request.Messages),
            ["max_tokens"] = request.MaxTokens,
            ["stream"] = false, // W3: SSE streaming
        };
        var message = new HttpRequestMessage(HttpMethod.Post, Endpoint)
        {
            Content = new StringContent(body.ToJsonString(), Encoding.UTF8, "application/json"),
        };
        Authorize(message, apiKey);
        return message;
    }
    protected virtual JsonNode? MapResponse(JsonNode? upstream) => upstream;

    public virtual async Task<ProviderResult> ChatAsync(HttpClient http, string model, ChatRequest request, string? apiKey, CancellationToken ct)
    {
        if (NeedsKey && string.IsNullOrEmpty(apiKey))
            return new ProviderResult(424, null, $"no vault key for provider '{Id}'", (null, null));

        using var message = BuildRequest(model, request, apiKey ?? "");
        var response = await http.SendAsync(message, ct);
        JsonNode? body = null;
        try { body = JsonNode.Parse(await response.Content.ReadAsStringAsync(ct)); } catch { /* non-json */ }

        var usage = ExtractUsage(body);
        var mapped = (int)response.StatusCode is >= 200 and < 300 ? MapResponse(body) : body;
        return new ProviderResult((int)response.StatusCode, mapped, null, usage);
    }

    protected virtual (long?, long?) ExtractUsage(JsonNode? body)
    {
        var usage = body?["usage"];
        return (usage?["prompt_tokens"]?.GetValue<long>(), usage?["completion_tokens"]?.GetValue<long>());
    }
}

public sealed class OpenAiProvider : ProviderBase
{
    public override string Id => "openai";
    public override IReadOnlyDictionary<string, string> KnownModels { get; } = new Dictionary<string, string>
    {
        ["gpt-4o"] = "flagship", ["gpt-4o-mini"] = "cheap-fast",
    };
    protected override Uri Endpoint => new("https://api.openai.com/v1/chat/completions");
    protected override void Authorize(HttpRequestMessage message, string apiKey) =>
        message.Headers.Authorization = new AuthenticationHeaderValue("Bearer", apiKey);
}

public sealed class AnthropicProvider : ProviderBase
{
    public override string Id => "anthropic";
    public override IReadOnlyDictionary<string, string> KnownModels { get; } = new Dictionary<string, string>
    {
        ["claude-opus-4-1"] = "flagship", ["claude-sonnet-4"] = "balanced",
    };
    protected override Uri Endpoint => new("https://api.anthropic.com/v1/messages");
    protected override void Authorize(HttpRequestMessage message, string apiKey)
    {
        message.Headers.Add("x-api-key", apiKey);
        message.Headers.Add("anthropic-version", "2023-06-01");
    }
    // OpenAI → Anthropic: hoist the system message into the dedicated field.
    protected override HttpRequestMessage BuildRequest(string model, ChatRequest request, string apiKey)
    {
        var system = string.Join("\n", request.Messages.Where(m => m.Role == "system").Select(m => m.Content));
        var rest = request.Messages.Where(m => m.Role != "system").ToList();
        var body = new JsonObject
        {
            ["model"] = model,
            ["max_tokens"] = request.MaxTokens ?? 1024,
            ["messages"] = JsonSerializer.SerializeToNode(rest),
        };
        if (system.Length > 0) body["system"] = system;
        var message = new HttpRequestMessage(HttpMethod.Post, Endpoint)
        {
            Content = new StringContent(body.ToJsonString(), Encoding.UTF8, "application/json"),
        };
        Authorize(message, apiKey);
        return message;
    }
    // Anthropic → OpenAI shape so clients only speak one protocol.
    protected override JsonNode? MapResponse(JsonNode? upstream)
    {
        if (upstream is null) return null;
        var text = upstream["content"]?[0]?["text"]?.GetValue<string>() ?? "";
        return new JsonObject
        {
            ["id"] = upstream["id"]?.GetValue<string>(),
            ["object"] = "chat.completion",
            ["model"] = upstream["model"]?.GetValue<string>(),
            ["choices"] = new JsonArray(new JsonObject
            {
                ["index"] = 0,
                ["message"] = new JsonObject { ["role"] = "assistant", ["content"] = text },
                ["finish_reason"] = "stop",
            }),
            ["usage"] = new JsonObject
            {
                ["prompt_tokens"] = upstream["usage"]?["input_tokens"]?.GetValue<long>() ?? 0,
                ["completion_tokens"] = upstream["usage"]?["output_tokens"]?.GetValue<long>() ?? 0,
            },
        };
    }
    protected override (long?, long?) ExtractUsage(JsonNode? body) =>
        (body?["usage"]?["input_tokens"]?.GetValue<long>(), body?["usage"]?["output_tokens"]?.GetValue<long>());
}

/// <summary>Local models via Ollama's OpenAI-compatible endpoint — no key required.</summary>
public sealed class OllamaProvider : OpenAiProvider
{
    public override string Id => "ollama";
    public override bool NeedsKey => false;
    public override IReadOnlyDictionary<string, string> KnownModels { get; } = new Dictionary<string, string>
    {
        ["llama3.1"] = "local-8b", ["qwen2.5"] = "local-alt",
    };
    protected override Uri Endpoint => new("http://127.0.0.1:11434/v1/chat/completions");
}
