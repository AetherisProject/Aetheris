using System.Diagnostics;
using System.Text.Json;
using Aetheris.Gateway;

// ============================================================
//  Aetheris.Gateway — the wedge product.
//  One OpenAI-compatible endpoint for all your tools.
//  Provider keys come from the ENCRYPTED VAULT at request time —
//  never from .env files, shell history, or client code.
//  Binds 127.0.0.1 by default (trusted component, owner-run).
// ============================================================

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddHttpClient();
var app = builder.Build();
var logger = app.Logger;

var config = new GatewayConfig();
app.Configuration.GetSection("Gateway").Bind(config);

// ---- services ----
using var keyringVault = VaultKeyring.TryOpen(config.KeyStorePath,
    Environment.GetEnvironmentVariable("AETHERIS_GATEWAY_PASS"), logger);
IKeyring keyring = keyringVault ?? new EmptyKeyring();

var providers = new Dictionary<string, ILlmProvider>(StringComparer.OrdinalIgnoreCase)
{
    ["openai"] = new OpenAiProvider(),
    ["anthropic"] = new AnthropicProvider(),
    ["ollama"] = new OllamaProvider(),
};
var budgets = new BudgetTracker(Path.Combine("data", "budgets.json"), config.BudgetTokensPerDay);
var requestLog = new GatewayLog();

// ---- access gate ----
app.Use(async (ctx, next) =>
{
    if (ctx.Request.Path.StartsWithSegments("/v1/gateway/health")) { await next(); return; }
    if (ctx.Request.Headers["X-Aetheris-Gateway-Token"] != config.AccessToken)
    {
        ctx.Response.StatusCode = StatusCodes.Status401Unauthorized;
        await ctx.Response.WriteAsJsonAsync(new { error = "unauthorized" });
        return;
    }
    await next();
});

app.MapGet("/v1/gateway/health", () => Results.Ok(new { ok = true, service = "aetheris-gateway" }));

// ---- model catalog: aliases + provider models ----
app.MapGet("/v1/models", () => Results.Ok(new
{
    aliases = config.Aliases.Select(a => new
    {
        id = a.Key,
        provider = a.Value.Provider,
        model = a.Value.Model,
        fallback = a.Value.FallbackTo,
    }),
    providers = providers.Select(p => new { id = p.Key, models = p.Value.KnownModels.Keys }),
}));

// ---- chat completions (OpenAI-compatible), with fallback chain ----
app.MapPost("/v1/chat/completions", async (ChatRequest request, HttpContext ctx, IHttpClientFactory httpFactory) =>
{
    if (request.Stream == true)
        return Results.BadRequest(new { error = "streaming lands in W3 — send stream:false for now" });

    var alias = request.Model;
    if (!config.Aliases.TryGetValue(alias, out var target))
        return Results.NotFound(new { error = $"unknown alias '{alias}'", hint = "GET /v1/models" });

    var http = httpFactory.CreateClient();
    var sw = Stopwatch.StartNew();
    var visited = new HashSet<string>();

    // Resilience: walk the fallback chain (a → b → …), each hop once.
    while (true)
    {
        if (!visited.Add(alias))
            return Results.Problem("fallback loop detected in alias chain");
        if (!providers.TryGetValue(target.Provider, out var provider))
            return Results.Problem($"alias '{alias}' points at unknown provider '{target.Provider}'");

        // Cheap estimate for the budget gate; real usage is reported from the response.
        var estimated = request.Messages.Sum(m => m.Content.Length / 4) + (request.MaxTokens ?? 512);
        if (!budgets.Allow(alias, estimated))
        {
            requestLog.Add(new GatewayLogEntry(DateTimeOffset.UtcNow, alias, provider.Id, 402, sw.ElapsedMilliseconds, null, null));
            return Results.Json(new { error = $"budget_exceeded", alias, reset = "midnight UTC" }, statusCode: 402);
        }

        var result = await provider.ChatAsync(http, target.Model, request, keyring.GetKey(provider.Id), ctx.RequestAborted);
        requestLog.Add(new GatewayLogEntry(DateTimeOffset.UtcNow, alias, provider.Id, result.Status, sw.ElapsedMilliseconds, result.Usage.In, result.Usage.Out));

        if (result.Usage.In is not null || result.Usage.Out is not null)
            budgets.Report(alias, (result.Usage.In ?? 0) + (result.Usage.Out ?? 0));

        if (result.Status is >= 200 and < 300)
            return Results.Json(result.Body);

        // Server-side failure with a configured fallback → hop to the next alias.
        if (result.Status >= 500 && target.FallbackTo is not null && config.Aliases.TryGetValue(target.FallbackTo, out var next))
        {
            logger.LogWarning("alias {Alias} failed ({Status}) → falling back to {Fallback}", alias, result.Status, target.FallbackTo);
            alias = target.FallbackTo;
            target = next;
            continue;
        }

        if (!string.IsNullOrEmpty(result.Error))
            return Results.Json(new { error = result.Error, alias }, statusCode: 424);
        return Results.Json(result.Body, statusCode: result.Status);
    }
});

// ---- operator surfaces (redacted) ----
app.MapGet("/v1/gateway/budgets", () => Results.Ok(budgets.Snapshot()));
app.MapGet("/v1/gateway/logs", () => Results.Ok(requestLog.Entries.OrderByDescending(e => e.At)));

app.Run(config.Listen);
