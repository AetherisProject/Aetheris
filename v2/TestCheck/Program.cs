using System.Reflection;
using System.Security.Cryptography;

// Check ChaCha20Poly1305 API
var key = new byte[32];
var cipher = new ChaCha20Poly1305(key);

Console.WriteLine("ChaCha20Poly1305 methods:");
foreach (var method in typeof(ChaCha20Poly1305).GetMethods(BindingFlags.Public | BindingFlags.Instance)) {
    Console.WriteLine(method.Name + " -> params: " + string.Join(", ", method.GetParameters().Select(p => p.ParameterType.Name)));
}
