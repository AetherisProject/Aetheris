using System.Diagnostics;
using Aetheris.Gateway;
using Aetheris.Gateway.Services;

// ============================================================
//  Aetheris.Gateway — the wedge product.
//  One OpenAI-compatible endpoint for all your tools.
//  Provider keys come from the ENCRYPTED VAULT at request time —
//  never from .env files, shell history, or client code.
//  Binds 127.0.0.1 by default (trusted component, owner-run).
// ============================================================

var builder = WebApplication.CreateBuilder(args);

// Configuration
var config = new GatewayConfig();
builder.Configuration.GetSection("Gateway").Bind(config);
builder.Services.AddSingleton(config);

// Core services
builder.Services.AddHttpClient();

// Register new services (3.2, 3.3, 3.4, 3.7)
builder.Services.AddSingleton<ProviderService>();
builder.Services.AddSingleton<BudgetService>(sp =>
    new BudgetService(Path.Combine("data", "budgets.json"), config.BudgetTokensPerDay));
builder.Services.AddSingleton<HealthCheckService>();

// Register VaultService as IKeyring (3.2 - hot keystore)
builder.Services.AddSingleton<IKeyring, VaultService>(sp =>
    new VaultService(
        config.KeyStorePath,
        Environment.GetEnvironmentVariable("AETHERIS_GATEWAY_PASS"),
        sp.GetRequiredService<ILogger<VaultService>>()));

// Register providers
var providers = new Dictionary<string, ILlmProvider>(StringComparer.OrdinalIgnoreCase)
{
    ["openai"] = new OpenAiProvider(),
    ["anthropic"] = new AnthropicProvider(),
    ["ollama"] = new OllamaProvider(),
};
builder.Services.AddSingleton(providers);

// Register request log
builder.Services.AddSingleton<GatewayLog>();

// Register controllers (3.1, 3.6)
builder.Services.AddControllers();

var app = builder.Build();
var logger = app.Logger;

// Start hot keystore watching (3.2)
var vaultService = app.Services.GetRequiredService<IKeyring>() as VaultService;
vaultService?.StartWatching();

// Initialize and start health check service (3.3)
var healthCheckService = app.Services.GetRequiredService<HealthCheckService>();
var httpFactory = app.Services.GetRequiredService<IHttpClientFactory>();
var keyring = app.Services.GetRequiredService<IKeyring>();
healthCheckService.Initialize(providers, keyring, httpFactory, logger);
healthCheckService.Start();

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

// ---- endpoints ----
// Health endpoint with per-provider status (3.3)
app.MapGet("/v1/gateway/health", () => healthCheckService.GetStatus());

// Model catalog: aliases + provider models (3.6)
var providerService = app.Services.GetRequiredService<ProviderService>();
app.MapGet("/v1/models", () => Results.Ok(providerService.GetCatalog()));

// ---- Register controllers (replaces inline implementations) ----
app.MapControllers();

// ---- operator surfaces (redacted) ----
var budgetService = app.Services.GetRequiredService<BudgetService>();
var requestLog = app.Services.GetRequiredService<GatewayLog>();

app.MapGet("/v1/gateway/budgets", () => Results.Ok(budgetService.Snapshot()));
app.MapGet("/v1/gateway/logs", () => Results.Ok(requestLog.Entries.OrderByDescending(e => e.At)));

app.Run(config.Listen);