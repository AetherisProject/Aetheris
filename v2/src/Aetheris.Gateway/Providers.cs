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
    IAsyncEnumerable<JsonNode> StreamChatAsync(HttpClient http, string model, ChatRequest request, string? apiKey, CancellationToken ct);
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

    public virtual async IAsyncEnumerable<JsonNode> StreamChatAsync(HttpClient http, string model, ChatRequest request, string? apiKey, [EnumeratorCancellation] CancellationToken ct)
    {
        if (NeedsKey && string.IsNullOrEmpty(apiKey))
        {
            yield break;
        }

        using var message = BuildStreamRequest(model, request, apiKey ?? "");
        var response = await http.SendAsync(message, ct);
        
        if (!response.IsSuccessStatusCode)
        {
            yield break;
        }

        using var stream = await response.Content.ReadAsStreamAsync(ct);
        using var reader = new StreamReader(stream);
        
        while (!reader.EndOfStream && !ct.IsCancellationRequested)
        {
            var line = await reader.ReadLineAsync(ct);
            if (string.IsNullOrWhiteSpace(line))
                continue;
            
            if (line.StartsWith("data: "))
            {
                var data = line[6..].Trim();
                if (data == "[DONE]")
                    break;
                    
                try
                {
                    var json = JsonNode.Parse(data);
                    if (json is not null)
                    {
                        var mapped = MapStreamResponse(json);
                        if (mapped is not null)
                            yield return mapped;
                    }
                }
                catch { /* parse error, skip */ }
            }
        }
    }

    protected virtual HttpRequestMessage BuildStreamRequest(string model, ChatRequest request, string apiKey)
    {
        var body = new JsonObject
        {
            ["model"] = model,
            ["messages"] = JsonSerializer.SerializeToNode(request.Messages),
            ["max_tokens"] = request.MaxTokens,
            ["stream"] = true,
        };
        var message = new HttpRequestMessage(HttpMethod.Post, Endpoint)
        {
            Content = new StringContent(body.ToJsonString(), Encoding.UTF8, "application/json"),
        };
        Authorize(message, apiKey);
        return message;
    }

    protected virtual JsonNode? MapStreamResponse(JsonNode? upstream) => upstream;
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
    
    protected override HttpRequestMessage BuildStreamRequest(string model, ChatRequest request, string apiKey)
    {
        var system = string.Join("\n", request.Messages.Where(m => m.Role == "system").Select(m => m.Content));
        var rest = request.Messages.Where(m => m.Role != "system").ToList();
        var body = new JsonObject
        {
            ["model"] = model,
            ["max_tokens"] = request.MaxTokens ?? 1024,
            ["messages"] = JsonSerializer.SerializeToNode(rest),
            ["stream"] = true,
        };
        if (system.Length > 0) body["system"] = system;
        var message = new HttpRequestMessage(HttpMethod.Post, Endpoint)
        {
            Content = new StringContent(body.ToJsonString(), Encoding.UTF8, "application/json"),
        };
        Authorize(message, apiKey);
        return message;
    }

    protected override JsonNode? MapStreamResponse(JsonNode? upstream)
    {
        if (upstream is null) return null;
        
        // Anthropic streaming format: { type: "message_start" | "content_block_start" | "content_block_delta" | "message_delta" | "message_stop" }
        var type = upstream["type"]?.GetValue<string>();
        
        if (type == "message_start")
        {
            // First chunk with metadata
            return new JsonObject
            {
                ["id"] = upstream["message"]?["id"]?.GetValue<string>() ?? Guid.NewGuid().ToString(),
                ["object"] = "chat.completion.chunk",
                ["created"] = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
                ["model"] = upstream["model"]?.GetValue<string>() ?? "",
                ["choices"] = new JsonArray(new JsonObject
                {
                    ["index"] = 0,
                    ["delta"] = new JsonObject { ["role"] = "assistant", ["content"] = "" },
                    ["finish_reason"] = null,
                })
            };
        }
        else if (type == "content_block_delta")
        {
            // Content chunks
            var text = upstream["delta"]?["text"]?.GetValue<string>() ?? "";
            return new JsonObject
            {
                ["id"] = Guid.NewGuid().ToString(),
                ["object"] = "chat.completion.chunk",
                ["created"] = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
                ["model"] = "",
                ["choices"] = new JsonArray(new JsonObject
                {
                    ["index"] = 0,
                    ["delta"] = new JsonObject { ["role"] = null, ["content"] = text },
                    ["finish_reason"] = null,
                })
            };
        }
        else if (type == "message_delta")
        {
            // Final metadata with stop reason
            var finishReason = upstream["delta"]?["stop_reason"]?.GetValue<string>() ?? "stop";
            return new JsonObject
            {
                ["id"] = Guid.NewGuid().ToString(),
                ["object"] = "chat.completion.chunk",
                ["created"] = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
                ["model"] = "",
                ["choices"] = new JsonArray(new JsonObject
                {
                    ["index"] = 0,
                    ["delta"] = new JsonObject { ["role"] = null, ["content"] = "" },
                    ["finish_reason"] = finishReason,
                })
            };
        }
        else if (type == "message_stop")
        {
            // Stream end - return null to signal completion
            return null;
        }
        
        return upstream;
    }
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
