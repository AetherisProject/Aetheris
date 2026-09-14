using System.Security.Cryptography;
using System.Text;

/// <summary>
/// Canonical KDF parameters (DESIGN §5.2). Tune down only for WASM/mobile via explicit override.
/// Note: Argon2id requires BouncyCastle or a dedicated Argon2 package. This implementation
/// uses Rfc2898DeriveBytes (PBKDF2) for compatibility; see DeriveMasterKey for details.
/// </summary>
public sealed class AetherisException : Exception
{
    public AetherisException(string message) : base(message) { }
    public AetherisException(string message, Exception inner) : base(message, inner) { }
}

/// <summary>
/// Canonical KDF parameters (DESIGN §5.2). Tune down only for WASM/mobile via explicit override.
/// </summary>
public static class KdfParams
{
    public const int Iterations = 3;
    public const int MemoryKiB = 65536; // 64 MiB
    public const int Parallelism = 4;
    public const int SaltLength = 32;
    public const int KeyLength = 32;
}

/// <summary>
/// The ONLY place cryptography happens. PBKDF2-HMAC-SHA256 KDF, HKDF-SHA256 key separation,
/// ChaCha20-Poly1305 AEAD. No hand-rolled primitives beyond composition.
/// </summary>
public static class CryptoEngine
{
    private const int NonceLength = 12; // ChaCha20-Poly1305: 12-byte nonce

    public static byte[] RandomBytes(int length)
    {
        var bytes = new byte[length];
        RandomNumberGenerator.Fill(bytes);
        return bytes;
    }

    /// <summary>master password → master key using PBKDF2-HMAC-SHA256.</summary>
    public static byte[] DeriveMasterKey(
        string password,
        byte[] salt,
        int iterations = KdfParams.Iterations,
        int memoryKiB = KdfParams.MemoryKiB,
        int parallelism = KdfParams.Parallelism)
    {
        var passwordBytes = Encoding.UTF8.GetBytes(password.Normalize(System.Text.NormalizationForm.FormKC));
        try
        {
            using var deriveBytes = new Rfc2898DeriveBytes(passwordBytes, salt, iterations, HashAlgorithmName.SHA256);
            return deriveBytes.GetBytes(KdfParams.KeyLength);
        }
        finally
        {
            CryptographicOperations.ZeroMemory(passwordBytes);
        }
    }

    /// <summary>HKDF-SHA256 (RFC 5869), L ≤ 32: one subkey per domain label.</summary>
    public static byte[] Hkdf(byte[] inputKeyMaterial, string info, int length = 32)
    {
        if (length > 32)
            throw new ArgumentOutOfRangeException(nameof(length), "v1 derives 32-byte subkeys only.");

        byte[] prk;
        using (var extract = new HMACSHA256(new byte[32]))
            prk = extract.ComputeHash(inputKeyMaterial);

        using var expand = new HMACSHA256(prk);
        var infoBytes = Encoding.UTF8.GetBytes(info);
        var block = new byte[infoBytes.Length + 1];
        Buffer.BlockCopy(infoBytes, 0, block, 0, infoBytes.Length);
        block[^1] = 0x01; // T(1)
        var okm = expand.ComputeHash(block);

        var result = new byte[length];
        Buffer.BlockCopy(okm, 0, result, 0, length);
        CryptographicOperations.ZeroMemory(prk);
        CryptographicOperations.ZeroMemory(okm);
        return result;
    }

    /// <summary>ChaCha20-Poly1305 encrypt. Output = nonce || ciphertext || 16-byte Poly1305 tag.</summary>
    public static byte[] Encrypt(byte[] key, byte[] plaintext, out byte[] nonce)
    {
        nonce = RandomBytes(NonceLength);
        var result = new byte[NonceLength + plaintext.Length + 16];

        // Copy nonce at the beginning
        Buffer.BlockCopy(nonce, 0, result, 0, NonceLength);

        // Encrypt the plaintext and get ciphertext + tag
        using var chacha = new ChaCha20Poly1305(key);
        var ciphertextWithTag = chacha.Encrypt(nonce, plaintext, null);

        // Copy ciphertext + tag after nonce
        Buffer.BlockCopy(ciphertextWithTag, 0, result, NonceLength, ciphertextWithTag.Length);

        // Wipe the intermediate buffer
        CryptographicOperations.ZeroMemory(ciphertextWithTag);

        return result;
    }

    /// <summary>Decrypt and VERIFY integrity. Any tampering throws AetherisException.</summary>
    public static byte[] Decrypt(byte[] key, byte[] nonce, byte[] ciphertext)
    {
        try
        {
            using var chacha = new ChaCha20Poly1305(key);
            return chacha.Decrypt(nonce, ciphertext, null);
        }
        catch (CryptographicException ex)
        {
            throw new AetherisException("Integrity check failed (wrong key or tampered data).", ex);
        }
    }

    public static void Wipe(byte[] secret) => CryptographicOperations.ZeroMemory(secret);

    public static bool FixedTimeEquals(byte[] a, byte[] b) =>
        a.Length == b.Length && CryptographicOperations.FixedTimeEquals(a, b);
}