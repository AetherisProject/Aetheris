namespace Aetheris.Hub.Models;

public sealed record BlobDto(
    int Generation,
    string Data,
    DateTimeOffset UpdatedAt
);

public sealed record BlobPut(
    int Generation,
    string Data,
    int ExpectedGeneration
);

public sealed record BlobGetResponse(
    int Generation,
    string Data,
    DateTimeOffset UpdatedAt
);

public sealed record BlobMetadataResponse(
    int LatestGeneration,
    int[] AvailableGenerations,
    DateTimeOffset UpdatedAt
);
