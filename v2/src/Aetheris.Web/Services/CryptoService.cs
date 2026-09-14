using System.Text.Json;
using Microsoft.JSInterop;

namespace Aetheris.Web.Services;

/// <summary>
/// Browser-based cryptography service using libsodium.js via JS interop.
/// All crypto operations happen in the browser - server never sees plaintext.
/// </summary>
public sealed class CryptoService : IAsyncDisposable
{
    private readonly IJSRuntime _jsRuntime;
    private DotNetObjectReference<CryptoService>? _dotNetHelper;
    private bool _disposed;

    public CryptoService(IJSRuntime jsRuntime)
    {
        _jsRuntime = jsRuntime;
    }

    /// <summary>
    /// Initialize the crypto service and ensure libsodium.js is loaded.
    /// </summary>
    public async Task InitializeAsync()
    {
        _dotNetHelper = DotNetObjectReference.Create(this);
        await _jsRuntime.InvokeVoidAsync("import", "/js/argon2.js");
        await _jsRuntime.InvokeVoidAsync("import", "/js/crypto.js");
    }

    /// <summary>
    /// Derive a cryptographic key from a password using Argon2id.
    /// </summary>
    /// <param name="password">The master password</param>
    /// <param name="salt">Salt for key derivation (hex string)</param>
    /// <param name="keyLength">Length of the derived key in bytes (default: 32)</param>
    /// <returns>Hex-encoded derived key</returns>
    public async Task<string> DeriveKeyAsync(string password, string salt, int keyLength = 32)
    {
        return await _jsRuntime.InvokeAsync<string>(
            "AetherisCrypto.deriveKey",
            password,
            salt,
            keyLength
        );
    }

    /// <summary>
    /// Encrypt plaintext using XChaCha20-Poly1305.
    /// </summary>
    /// <param name="plaintext">The text to encrypt</param>
    /// <param name="key">The encryption key (hex string, 32 bytes)</param>
    /// <returns>Object containing ciphertext and nonce (both hex strings)</returns>
    public async Task<EncryptedData> EncryptAsync(string plaintext, string key)
    {
        var result = await _jsRuntime.InvokeAsync<JsonElement>(
            "AetherisCrypto.encrypt",
            plaintext,
            key
        );
        
        return new EncryptedData
        {
            Ciphertext = result.GetProperty("ciphertext").GetString()!,
            Nonce = result.GetProperty("nonce").GetString()!
        };
    }

    /// <summary>
    /// Decrypt ciphertext using XChaCha20-Poly1305.
    /// </summary>
    /// <param name="encryptedData">Object containing ciphertext and nonce</param>
    /// <param name="key">The decryption key (hex string, 32 bytes)</param>
    /// <returns>The decrypted plaintext</returns>
    public async Task<string> DecryptAsync(EncryptedData encryptedData, string key)
    {
        return await _jsRuntime.InvokeAsync<string>(
            "AetherisCrypto.decrypt",
            new { encryptedData.Ciphertext, encryptedData.Nonce },
            key
        );
    }

    /// <summary>
    /// Encrypt a JSON-serializable object.
    /// </summary>
    /// <param name="obj">The object to encrypt</param>
    /// <param name="key">The encryption key (hex string, 32 bytes)</param>
    /// <returns>Object containing ciphertext and nonce (both hex strings)</returns>
    public async Task<EncryptedData> EncryptObjectAsync(object obj, string key)
    {
        var result = await _jsRuntime.InvokeAsync<JsonElement>(
            "AetherisCrypto.encryptObject",
            obj,
            key
        );
        
        return new EncryptedData
        {
            Ciphertext = result.GetProperty("ciphertext").GetString()!,
            Nonce = result.GetProperty("nonce").GetString()!
        };
    }

    /// <summary>
    /// Decrypt to a JSON object.
    /// </summary>
    /// <param name="encryptedData">Object containing ciphertext and nonce</param>
    /// <param name="key">The decryption key (hex string, 32 bytes)</param>
    /// <returns>The decrypted object</returns>
    public async Task<T> DecryptObjectAsync<T>(EncryptedData encryptedData, string key)
    {
        var json = await _jsRuntime.InvokeAsync<string>(
            "AetherisCrypto.decryptObject",
            new { encryptedData.Ciphertext, encryptedData.Nonce },
            key
        );
        
        return JsonSerializer.Deserialize<T>(json)!;
    }

    /// <summary>
    /// Generate a random salt.
    /// </summary>
    /// <param name="length">Length in bytes (default: 16)</param>
    /// <returns>Hex-encoded salt</returns>
    public async Task<string> GenerateSaltAsync(int length = 16)
    {
        return await _jsRuntime.InvokeAsync<string>(
            "AetherisCrypto.generateSalt",
            length
        );
    }

    /// <summary>
    /// Generate a random encryption key.
    /// </summary>
    /// <returns>Hex-encoded key (32 bytes)</returns>
    public async Task<string> GenerateKeyAsync()
    {
        return await _jsRuntime.InvokeAsync<string>(
            "AetherisCrypto.generateKey"
        );
    }

    /// <summary>
    /// Hash a string using SHA-256.
    /// </summary>
    /// <param name="input">The string to hash</param>
    /// <returns>Hex-encoded hash</returns>
    public async Task<string> Sha256Async(string input)
    {
        return await _jsRuntime.InvokeAsync<string>(
            "AetherisCrypto.sha256",
            input
        );
    }

    public async ValueTask DisposeAsync()
    {
        if (!_disposed)
        {
            _disposed = true;
            _dotNetHelper?.Dispose();
            _dotNetHelper = null;
        }
    }
}

/// <summary>
/// Represents encrypted data with ciphertext and nonce.
/// </summary>
public sealed record EncryptedData
{
    public string Ciphertext { get; init; } = string.Empty;
    public string Nonce { get; init; } = string.Empty;
}
