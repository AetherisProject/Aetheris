using System.Security.Cryptography;
using System.Text;
using Aetheris.Hub.Models;
using Aetheris.Hub.Services;

var builder = WebApplication.CreateBuilder(args);

// Configuration
builder.Services.AddSingleton<IConfiguration>(builder.Configuration);
var accessToken = builder.Configuration["AETHERIS_TOKEN"] ?? "dev-token";

// Services
var dataDir = Path.Combine(AppContext.BaseDirectory, "data");
Directory.CreateDirectory(dataDir);

builder.Services.AddSingleton<IDeviceStore>(new FileDeviceStore(dataDir));
builder.Services.AddSingleton<IBlobStore>(new FileBlobStore(Path.Combine(dataDir, "blobs"), maxGenerations: 50));

// Controllers
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// Security headers middleware
app.Use(async (ctx, next) =>
{
    ctx.Response.Headers.Append("X-Content-Type-Options", "nosniff");
    ctx.Response.Headers.Append("X-Frame-Options", "DENY");
    ctx.Response.Headers.Append("X-XSS-Protection", "0");
    ctx.Response.Headers.Append("Content-Security-Policy", "default-src 'none'");
    await next();
});

// Auth gate (everything except /v1/health and /v1/devices)
app.Use(async (ctx, next) =>
{
    var path = ctx.Request.Path.Value?.ToLowerInvariant();

    if (path?.StartsWith("/v1/health") == true ||
        path?.StartsWith("/v1/meta") == true ||
        path?.StartsWith("/v1/devices") == true)
    {
        await next();
        return;
    }

    if (ctx.Request.Headers["X-Aetheris-Token"] != accessToken)
    {
        ctx.Response.StatusCode = StatusCodes.Status401Unauthorized;
        await ctx.Response.WriteAsJsonAsync(new { error = "unauthorized" });
        return;
    }

    await next();
});

// Health endpoint
app.MapGet("/v1/health", () => Results.Ok(new { ok = true, service = "aetheris-hub", version = "2.0.0" }));

// API routes
app.MapControllers();

app.Run("http://0.0.0.0:8080");
