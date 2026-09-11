namespace Aetheris.Web.Services;

/// <summary>
/// Client-side vault state. CURRENTLY DEMO DATA — W4 wires real E2E crypto:
/// blobs are fetched from Hub and decrypted IN THE BROWSER (WebCrypto/libsodium
/// via JS interop); the Hub must never see plaintext (see agents/W4-web-pwa.md).
/// </summary>
public sealed class VaultState
{
    public bool IsUnlocked { get; private set; }

    public void MarkUnlocked() => IsUnlocked = true;
    public void Lock() => IsUnlocked = false;

    public IReadOnlyList<DemoItem> Items { get; } = new List<DemoItem>
    {
        new("Password", "GitHub", "me@example.com", "…encrypted"),
        new("ApiKey", "OpenAI production", "openai", "sk-…(via gateway)"),
        new("ApiKey", "Anthropic", "anthropic", "sk-…(via gateway)"),
        new("Note", "Recovery codes", "—", "…encrypted"),
    };
}

public sealed record DemoItem(string Type, string Title, string Meta, string Secret);
