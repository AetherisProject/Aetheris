using Org.BouncyCastle.Crypto;
using Org.BouncyCastle.Crypto.Generators;

// Test what's actually in BouncyCastle
var types = typeof(Org.BouncyCastle.Crypto.Ciphers.AESEngine).Assembly.GetTypes();
foreach (var t in types) {
    if (t.Name.Contains("Argon") || t.Name.Contains("Argon2")) {
        Console.WriteLine(t.FullName);
    }
}
Console.WriteLine("---");
foreach (var t in types) {
    if (t.Name.Contains("BytesGenerator")) {
        Console.WriteLine(t.FullName);
    }
}
