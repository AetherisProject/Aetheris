using System.Text.Json;

// Aetheris.Hub — zero-knowledge blob sync relay.
// The hub stores OPAQUE ENCRYPTED BLOBS with generation counters. It has no keys,
// performs no crypto, and cannot read vault contents. Single-tenant v1
// (one vault + a static access token). Multi-tenant + Ed25519 challenge
// sessions are W2 agent tasks (see agents/W2-hub.md).

var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

var accessToken = app.Configuration["AETHERIS_TOKEN"] ?? "dev-token";
var dataDir = Path.Combine(AppContext.BaseDirectory, "data");
Directory.CreateDirectory(dataDir);
var blobPath = Path.Combine(dataDir, "vault.blob.json");

// ---- auth gate (everything except /v1/health) ----
app.Use(async (ctx, next) =>
{
    if (ctx.Request.Path == "/v1/health") { await next(); return; }
    if (ctx.Request.Headers["X-Aetheris-Token"] != accessToken)
    {
        ctx.Response.StatusCode = StatusCodes.Status401Unauthorized;
        await ctx.Response.WriteAsJsonAsync(new { error = "unauthorized" });
        return;
    }
    await next();
});

app.MapGet("/v1/health", () => Results.Ok(new { ok = true, service = "aetheris-hub" }));

// ---- blob sync ----
app.MapGet("/v1/blob", () =>
{
    if (!File.Exists(blobPath))
        return Results.NotFound(new { error = "no_blob" });
    var dto = JsonSerializer.Deserialize<BlobDto>(File.ReadAllText(blobPath));
    return dto is null ? Results.NotFound(new { error = "no_blob" }) : Results.Ok(dto);
});

app.MapPut("/v1/blob", (BlobPut put) =>
{
    var current = 0;
    if (File.Exists(blobPath))
    {
        var dto = JsonSerializer.Deserialize<BlobDto>(File.ReadAllText(blobPath));
        current = dto?.Generation ?? 0;
    }
    if (put.Generation != current + 1)
        return Results.Conflict(new { error = "generation_conflict", expected = current + 1 });

    var stored = new BlobDto(put.Generation, put.Data, DateTimeOffset.UtcNow);
    var tmp = blobPath + ".tmp";
    File.WriteAllText(tmp, JsonSerializer.Serialize(stored));
    File.Move(tmp, blobPath, overwrite: true);
    return Results.Ok(new { generation = put.Generation });
});

app.Run("http://0.0.0.0:8080");

internal sealed record BlobDto(int Generation, string Data, DateTimeOffset UpdatedAt);
internal sealed record BlobPut(int Generation, string Data);
