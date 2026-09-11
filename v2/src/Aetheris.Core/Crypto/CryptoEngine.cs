using System.Security.Cryptography;
using System.Text;
using Org.BouncyCastle.Crypto;
using Org.BouncyCastle.Crypto.Engines;
using Org.BouncyCastle.Crypto.Generators;
using Org.BouncyCastle.Crypto.Modes;
using Org.BouncyCastle.Crypto.Parameters;

namespace Aetheris.Core.Crypto;

public sealed class AetherisException : Exception
{
    public AetherisException(string message) : base(message) { }
    public AetherisException(string message, Exception inner) : base(message, inner) { }
}

/// <summary>Canonical KDF parameters (DESIGN §5.2). Tune down only for WASM/mobile via explicit override.</summary>
public static class KdfParams
{
    public const int Iterations = 3;
    public const int MemoryKiB = 65536; // 64 MiB
    public const int Parallelism = 4;
    public const int SaltLength = 32;
    public const int KeyLength = 32;
}

/// <summary>
/// The ONLY place cryptography happens. Argon2id KDF, HKDF-SHA256 key separation,
/// XChaCha20-Poly1305 AEAD. No hand-rolled primitives beyond composition.
/// </summary>
public static class CryptoEngine
{
    private const int NonceLength = 24; // XChaCha20: 192-bit random nonces are safe

    public static byte[] RandomBytes(int length)
    {
        var bytes = new byte[length];
        RandomNumberGenerator.Fill(bytes);
        return bytes;
    }

    /// <summary>master password → master key (RFC 9106 recommended profile).</summary>
    public static byte[] DeriveMasterKey(
        string password,
        byte[] salt,
        int iterations = KdfParams.Iterations,
        int memoryKiB = KdfParams.MemoryKiB,
        int parallelism = KdfParams.Parallelism)
    {
        var passwordBytes = Encoding.UTF8.GetBytes(password.Normalize(NormalizationForm.FormKC));
        try
        {
            var parameters = new Argon2Parameters.Builder(Argon2Parameters.Argon2id)
                .WithIterations(iterations)
                .WithMemoryAsKB(memoryKiB)
                .WithParallelism(parallelism)
                .WithSalt(salt)
                .Build();

            var generator = new Argon2BytesGenerator();
            generator.Init(parameters);

            var key = new byte[KdfParams.KeyLength];
            generator.GenerateBytes(passwordBytes, key, 0, key.Length);
            return key;
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
        using (var extract = new HMACSHA256(new byte[32])) // salt = zeros (IKM is already a KDF output)
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

    /// <summary>XChaCha20-Poly1305 encrypt. Output = ciphertext || 16-byte Poly1305 tag.</summary>
    public static byte[] Encrypt(byte[] key, byte[] plaintext, out byte[] nonce)
    {
        nonce = RandomBytes(NonceLength);
        var cipher = new ChaCha20Poly1305(new XChaCha7539Engine());
        cipher.Init(true, new AeadParameters(new KeyParameter(key), 128, nonce));
        var output = new byte[cipher.GetOutputSize(plaintext.Length)];
        var offset = cipher.ProcessBytes(plaintext, 0, plaintext.Length, output, 0);
        cipher.DoFinal(output, offset);
        return output;
    }

    /// <summary>Decrypt and VERIFY integrity. Any tampering throws AetherisException.</summary>
    public static byte[] Decrypt(byte[] key, byte[] nonce, byte[] ciphertext)
    {
        var cipher = new ChaCha20Poly1305(new XChaCha7539Engine());
        cipher.Init(false, new AeadParameters(new KeyParameter(key), 128, nonce));
        var output = new byte[cipher.GetOutputSize(ciphertext.Length)];
        try
        {
            var offset = cipher.ProcessBytes(ciphertext, 0, ciphertext.Length, output, 0);
            cipher.DoFinal(output, offset);
            return output;
        }
        catch (InvalidCipherTextException ex)
        {
            throw new AetherisException("Integrity check failed (wrong key or tampered data).", ex);
        }
    }

    public static void Wipe(byte[] secret) => CryptographicOperations.ZeroMemory(secret);

    public static bool FixedTimeEquals(byte[] a, byte[] b) =>
        a.Length == b.Length && CryptographicOperations.FixedTimeEquals(a, b);
}
