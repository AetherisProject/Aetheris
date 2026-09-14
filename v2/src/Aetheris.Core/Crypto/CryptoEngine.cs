using System.Security.Cryptography;
using System.Text;
using System.Buffers;
using Org.BouncyCastle.Crypto.Digests;
using Org.BouncyCastle.Crypto.Generators;
using Org.BouncyCastle.Crypto.Parameters;
using Org.BouncyCastle.Crypto.Modes;
namespace Aetheris.Core.Crypto;
//
// Copyright (c) Aetheris Authors.
// Licensed under the MIT License.
//

/// <summary>
/// Canonical KDF parameters (DESIGN §5.2). Argon2id parameters tuned for 2026-era hardware.
/// </summary>
public static class KdfParams
{
    // Argon2id parameters following RFC 9106 recommendations and OWASP guidance
    // Memory: 64 MiB (65536 KiB) - reasonable for desktop/server; tune down for mobile/WASM
    // Iterations: 3 - moderate work factor
    // Parallelism: 4 - utilize multi-core
    // Salt length: 32 bytes (256 bits)
    // Output key length: 32 bytes (256 bits)
    public const int DegreeOfParallelism = 4;
    public const int MemoryKiB = 65536; // 64 MiB
    public const int Iterations = 3;
    public const int SaltLength = 32;
    public const int KeyLength = 32;
}

/// <summary>
/// The ONLY place cryptography happens. Argon2id KDF, HKDF-SHA256 key separation,
/// XChaCha20-Poly1305 AEAD. No hand-rolled primitives beyond composition.
/// </summary>
public static class CryptoEngine
{
    private const int NonceLength = 24; // XChaCha20-Poly1305: 192-bit random nonces

    public static byte[] RandomBytes(int length)
    {
        var bytes = new byte[length];
        RandomNumberGenerator.Fill(bytes);
        return bytes;
    }

    /// <summary>master password → master key using Argon2id.</summary>
    public static byte[] DeriveMasterKey(
        string password,
        byte[] salt,
        int iterations = KdfParams.Iterations,
        int memoryKiB = KdfParams.MemoryKiB,
        int parallelism = KdfParams.DegreeOfParallelism)
    {
        var passwordBytes = Encoding.UTF8.GetBytes(password.Normalize(System.Text.NormalizationForm.FormKC));
        try
        {
            var generator = new Argon2BytesGenerator();
            var parameters = new Argon2Parameters.Builder(Argon2Parameters.Argon2id)
                .WithSalt(salt)
                .WithParallelism(parallelism)
                .WithMemoryAsKB(memoryKiB)
                .WithIterations(iterations)
                .Build();
            
            generator.Init(parameters);
            var output = new byte[KdfParams.KeyLength];
            generator.GenerateBytes(passwordBytes, output);
            return output;
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
    /// <summary>XChaCha20-Poly1305 encrypt. Output = nonce || ciphertext || 16-byte Poly1305 tag.</summary>
    public static byte[] Encrypt(byte[] key, byte[] plaintext, out byte[] nonce)
    {
        nonce = RandomBytes(NonceLength);
        var ciphertext = new byte[plaintext.Length];
        var tag = new byte[16];
        
        var xchacha = new XChaCha20Poly1305();
        xchacha.Init(true, new AeadParameters(new KeyParameter(key), 128, nonce, null));
        
        var len = xchacha.ProcessBytes(plaintext, 0, plaintext.Length, ciphertext, 0);
        xchacha.DoFinal(tag, 0);
        
        // Return result: nonce prepended to ciphertext+tag
        var result = new byte[NonceLength + ciphertext.Length + tag.Length];
        Buffer.BlockCopy(nonce, 0, result, 0, NonceLength);
        Buffer.BlockCopy(ciphertext, 0, result, NonceLength, ciphertext.Length);
        Buffer.BlockCopy(tag, 0, result, NonceLength + ciphertext.Length, tag.Length);
        return result;
    }
    /// <summary>Decrypt and VERIFY integrity. Any tampering throws AetherisException.</summary>
    public static byte[] Decrypt(byte[] key, byte[] nonce, byte[] ciphertextAndTag)
    {
        // ciphertextAndTag = ciphertext || tag (total: data + 16)
        if (ciphertextAndTag.Length < 16)
            throw new AetherisException("Ciphertext too short for authentication tag.");
        
        var ciphertext = new byte[ciphertextAndTag.Length - 16];
        var tag = new byte[16];
        Buffer.BlockCopy(ciphertextAndTag, 0, ciphertext, 0, ciphertext.Length);
        Buffer.BlockCopy(ciphertextAndTag, ciphertext.Length, tag, 0, tag.Length);
        var xchacha = new XChaCha20Poly1305();
        xchacha.Init(false, new AeadParameters(new KeyParameter(key), 128, nonce, null));
        
        try
        {
            var plaintext = new byte[ciphertext.Length];
            var len = xchacha.ProcessBytes(ciphertext, 0, ciphertext.Length, plaintext, 0);
            xchacha.DoFinal(plaintext, len);
            return plaintext;
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

