namespace Aetheris.Core.Crypto;

/// <summary>
/// Exception thrown by Aetheris cryptographic operations.
/// </summary>
public class AetherisException : Exception
{
    public AetherisException(string message) : base(message) { }
    public AetherisException(string message, Exception inner) : base(message, inner) { }
}
