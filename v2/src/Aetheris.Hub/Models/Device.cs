namespace Aetheris.Hub.Models;

public sealed record Device(
    string Id,
    string Name,
    string PublicKeyBase64,
    DateTimeOffset CreatedAt
);

public sealed record DeviceRegistrationRequest(
    string Name,
    string PublicKeyBase64,
    string SignatureBase64
);

public sealed record DeviceRegistrationResponse(
    string DeviceId,
    string Name,
    DateTimeOffset CreatedAt
);

public sealed record DeviceAuthRequest(
    string DeviceId,
    string SignatureBase64,
    string ChallengeNonceBase64
);
