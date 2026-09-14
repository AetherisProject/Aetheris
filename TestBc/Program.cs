using System;
using System.Security.Cryptography;

// .NET 8 does NOT have Argon2id or XChaCha20-Poly1305 built-in
// We need to use BouncyCastle for both

// Check BouncyCastle for Argon2
Console.WriteLine("Searching for Argon2 in BouncyCastle:");
var bcAssembly = typeof(Org.BouncyCastle.Crypto.Modes.ChaCha20Poly1305).Assembly;
bool hasArgon2 = false;
foreach (var type in bcAssembly.GetExportedTypes()) {
    if (type.Namespace != null && type.Namespace.Contains("BouncyCastle")) {
        if (type.Name.Contains("Argon") || type.Name.Contains("argon")) {
            Console.WriteLine(type.FullName);
            hasArgon2 = true;
        }
    }
}
if (!hasArgon2) Console.WriteLine("No Argon2 found in BouncyCastle 2.4.0");

// Check for XChaCha
Console.WriteLine("\nSearching for XChaCha in BouncyCastle:");
bool hasXChaCha = false;
foreach (var type in bcAssembly.GetExportedTypes()) {
    if (type.Namespace != null && type.Namespace.Contains("BouncyCastle")) {
        if (type.Name.Contains("XChaCha") || type.Name.Contains("xchacha")) {
            Console.WriteLine(type.FullName);
            hasXChaCha = true;
        }
    }
}
if (!hasXChaCha) Console.WriteLine("No XChaCha found in BouncyCastle 2.4.0");
