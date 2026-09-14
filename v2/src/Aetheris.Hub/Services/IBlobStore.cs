namespace Aetheris.Hub.Services;

public interface IBlobStore
{
    Task<int> GetLatestGenerationAsync();
    Task<int[]> GetAvailableGenerationsAsync(int maxCount = 50);
    Task<Models.BlobDto?> GetBlobAsync(int generation);
    Task StoreBlobAsync(Models.BlobDto blob);
    Task<bool> ExistsAsync(int generation);
}
