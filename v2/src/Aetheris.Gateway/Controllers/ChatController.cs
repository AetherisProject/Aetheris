using Microsoft.AspNetCore.Mvc;
using System.Diagnostics;
using System.Text.Json;
using Aetheris.Gateway;
using Aetheris.Gateway.Services;

namespace Aetheris.Gateway.Controllers;

[ApiController]
[Route("v1")]
public sealed class ChatController : ControllerBase
{
    private readonly Dictionary<string, ILlmProvider> _providers;
    private readonly IKeyring _keyring;
    private readonly IHttpClientFactory _httpFactory;
    private readonly BudgetService _budgetService;
    private readonly GatewayConfig _config;
    private readonly GatewayLog _requestLog;
    private readonly ILogger<ChatController> _logger;

    public ChatController(
        Dictionary<string, ILlmProvider> providers,
        IKeyring keyring,
        IHttpClientFactory httpFactory,
        BudgetService budgetService,
        GatewayConfig config,
        GatewayLog requestLog,
        ILogger<ChatController> logger)
    {
        _providers = providers;
        _keyring = keyring;
        _httpFactory = httpFactory;
        _budgetService = budgetService;
        _config = config;
        _requestLog = requestLog;
        _logger = logger;
    }

    [HttpPost("chat/completions")]
    public async Task<IActionResult> CreateChatCompletion([FromBody] ChatRequest request)
    {
        var alias = request.Model;
        if (!_config.Aliases.TryGetValue(alias, out var target))
            return NotFound(new { error = $"unknown alias '{alias}'", hint = "GET /v1/models" });

        var http = _httpFactory.CreateClient();
        var sw = Stopwatch.StartNew();
        var visited = new HashSet<string>();

        // Resilience: walk the fallback chain (a → b → …), each hop once.
        while (true)
        {
            if (!visited.Add(alias))
                return Problem("fallback loop detected in alias chain");
            if (!_providers.TryGetValue(target.Provider, out var provider))
                return Problem($"alias '{alias}' points at unknown provider '{target.Provider}'");

            // Cheap estimate for the budget gate; real usage is reported from the response.
            var estimated = request.Messages.Sum(m => m.Content.Length / 4) + (request.MaxTokens ?? 512);
            var budgetCheck = _budgetService.Allow(alias, estimated);
            
            if (!budgetCheck.Allowed)
            {
                _requestLog.Add(new GatewayLogEntry(DateTimeOffset.UtcNow, alias, provider.Id, 402, sw.ElapsedMilliseconds, null, null));
                return StatusCode(402, new { error = "budget_exceeded", alias, reset = "midnight UTC" });
            }

            // Handle streaming (3.1) - SSE streaming now implemented
            if (request.Stream == true)
            {
                return await StreamChatCompletion(provider, target.Model, request, alias, sw, http);
            }

            var result = await provider.ChatAsync(http, target.Model, request, _keyring.GetKey(provider.Id), HttpContext.RequestAborted);
            _requestLog.Add(new GatewayLogEntry(DateTimeOffset.UtcNow, alias, provider.Id, result.Status, sw.ElapsedMilliseconds, result.Usage.In, result.Usage.Out));

            if (result.Usage.In is not null || result.Usage.Out is not null)
                _budgetService.Report(alias, result.Usage.In ?? 0, result.Usage.Out ?? 0);

            if (result.Status is >= 200 and < 300)
                return Ok(result.Body);

            // Server-side failure with a configured fallback → hop to the next alias.
            if (result.Status >= 500 && target.FallbackTo is not null && _config.Aliases.TryGetValue(target.FallbackTo, out var next))
            {
                _logger.LogWarning("alias {Alias} failed ({Status}) → falling back to {Fallback}", alias, result.Status, target.FallbackTo);
                alias = target.FallbackTo;
                target = next;
                continue;
            }

            if (!string.IsNullOrEmpty(result.Error))
                return StatusCode(424, new { error = result.Error, alias });
            return StatusCode(result.Status, result.Body);
        }
    }

    private async Task<IActionResult> StreamChatCompletion(
        ILlmProvider provider, 
        string model, 
        ChatRequest request, 
        string alias, 
        Stopwatch sw,
        HttpClient http)
    {
        var apiKey = _keyring.GetKey(provider.Id);
        if (provider.NeedsKey && string.IsNullOrEmpty(apiKey))
        {
            return StatusCode(424, new { error = $"no vault key for provider '{provider.Id}'" });
        }

        // Set SSE headers
        Response.Headers.Add("Content-Type", "text/event-stream");
        Response.Headers.Add("Cache-Control", "no-cache");
        Response.Headers.Add("Connection", "keep-alive");

        var stream = provider.StreamChatAsync(http, model, request, apiKey, HttpContext.RequestAborted);
        
        try
        {
            await foreach (var chunk in stream)
            {
                if (HttpContext.RequestAborted.IsCancellationRequested)
                    break;
                    
                try
                {
                    // Convert JsonNode to string
                    var jsonString = chunk.ToJsonString();
                    await Response.WriteAsync($"data: {jsonString}\n\n");
                    await Response.Body.FlushAsync();
                }
                catch (Exception ex)
                {
                    _logger.LogError(ex, "Error writing SSE chunk");
                    break;
                }
            }

            // Send completion marker
            await Response.WriteAsync("data: [DONE]\n\n");
            await Response.Body.FlushAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Streaming error for alias {Alias}", alias);
            // Try to send error as SSE
            await Response.WriteAsync($"data: {{\"error\": \"{ex.Message.Replace("\"", "\\\"")}\"}}\n\n");
            await Response.WriteAsync("data: [DONE]\n\n");
        }

        return new EmptyResult();
    }
}