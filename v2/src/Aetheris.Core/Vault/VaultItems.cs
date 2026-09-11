using System.Text.Json;
using System.Text.Json.Serialization;

namespace Aetheris.Core.Vault;

public enum ItemType
{
    Password,
    ApiKey,
    Note,
    Card,
    Identity,
    SshKey,
    SshConnection,
}

// ---- Payloads (always travel inside the encrypted envelope, never plaintext at rest) ----

public sealed record PasswordPayload(
    string Username,
    string Password,
    List<string> Urls,
    string? Totp,
    string? Notes);

public sealed record ApiKeyPayload(
    string Provider,
    string Key,
    List<string> Scopes,
    DateTimeOffset? ExpiresAt,
    string? RotationPolicy);

public sealed record NotePayload(string Body);

public sealed record CardPayload(string Number, string Expiry, string Cvv, string Holder);

public sealed record IdentityPayload(string Name, string Email, string? Phone, string? Address);

public sealed record SshKeyPayload(string PrivateKey, string? PublicKey, string? Fingerprint, string? Comment);

public sealed record SshConnectionPayload(string Host, int Port, string Username, string? KeyId);

// ---- Item (what gets encrypted as one envelope) ----

public sealed record VaultItem(
    string Id,
    ItemType Type,
    string Title,
    List<string> Tags,
    bool Favorite,
    DateTimeOffset CreatedAt,
    DateTimeOffset UpdatedAt,
    string PayloadJson)
{
    public T Payload<T>() => JsonSerializer.Deserialize<T>(PayloadJson, VaultJson.Options)
        ?? throw new InvalidOperationException("Corrupt payload.");

    public VaultItem WithPayload<T>(T payload) => this with
    {
        PayloadJson = JsonSerializer.Serialize(payload, VaultJson.Options),
        UpdatedAt = DateTimeOffset.UtcNow,
    };
}

public static class VaultJson
{
    public static readonly JsonSerializerOptions Options = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    };
}

public static class VaultItemFactory
{
    public static VaultItem Create<T>(ItemType type, string title, T payload, IEnumerable<string>? tags = null)
    {
        var now = DateTimeOffset.UtcNow;
        return new VaultItem(
            Id: Guid.NewGuid().ToString("N"),
            Type: type,
            Title: title,
            Tags: tags?.ToList() ?? new List<string>(),
            Favorite: false,
            CreatedAt: now,
            UpdatedAt: now,
            PayloadJson: JsonSerializer.Serialize(payload, VaultJson.Options));
    }
}
