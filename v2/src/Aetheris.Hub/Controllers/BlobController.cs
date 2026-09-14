using Aetheris.Hub.Models;
using Aetheris.Hub.Services;

namespace Aetheris.Hub.Controllers;

[ApiController]
[Route("[controller]")]
public sealed class BlobController : ControllerBase
{
    private readonly IBlobStore _blobStore;
    private readonly ILogger<BlobController> _logger;

    public BlobController(IBlobStore blobStore, ILogger<BlobController> logger)
    {
        _blobStore = blobStore ?? throw new ArgumentNullException(nameof(blobStore));
        _logger = logger;
    }

    [HttpGet("v1/blob")]
    public async Task<IActionResult> GetLatestBlob()
    {
        try
        {
            var latestGen = await _blobStore.GetLatestGenerationAsync();
            if (latestGen <= 0)
                return NotFound(new { error = "no_blob" });

            var blob = await _blobStore.GetBlobAsync(latestGen);
            if (blob == null)
                return NotFound(new { error = "blob_not_found" });

            return Ok(new BlobGetResponse(
                Generation: blob.Generation,
                Data: blob.Data,
                UpdatedAt: blob.UpdatedAt
            ));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get latest blob");
            return StatusCode(500, new { error = "internal_error" });
        }
    }

    [HttpGet("v1/blob/{generation:int}")]
    public async Task<IActionResult> GetBlobByGeneration(int generation)
    {
        try
        {
            var blob = await _blobStore.GetBlobAsync(generation);
            if (blob == null)
                return NotFound(new { error = "blob_not_found" });

            return Ok(new BlobGetResponse(
                Generation: blob.Generation,
                Data: blob.Data,
                UpdatedAt: blob.UpdatedAt
            ));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get blob generation {Generation}", generation);
            return StatusCode(500, new { error = "internal_error" });
        }
    }

    [HttpGet("v1/meta")]
    public async Task<IActionResult> GetMetadata()
    {
        try
        {
            var latestGen = await _blobStore.GetLatestGenerationAsync();
            var availableGens = await _blobStore.GetAvailableGenerationsAsync(50);

            return Ok(new BlobMetadataResponse(
                LatestGeneration: latestGen,
                AvailableGenerations: availableGens,
                UpdatedAt: DateTimeOffset.UtcNow
            ));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get metadata");
            return StatusCode(500, new { error = "internal_error" });
        }
    }

    [HttpPut("v1/blob")]
    public async Task<IActionResult> StoreBlob([FromBody] BlobPut request)
    {
        try
        {
            if (request == null)
                return BadRequest(new { error = "invalid_request" });

            if (string.IsNullOrWhiteSpace(request.Data))
                return BadRequest(new { error = "data_required" });

            // Get current latest generation
            var currentGen = await _blobStore.GetLatestGenerationAsync();

            // Validate expected generation
            if (request.ExpectedGeneration.HasValue)
            {
                if (request.ExpectedGeneration.Value != currentGen + 1)
                {
                    return Conflict(new {
                        error = "generation_conflict",
                        expected = currentGen + 1,
                        actual = request.ExpectedGeneration.Value
                    });
                }
            }
            else if (request.Generation != currentGen + 1)
            {
                return Conflict(new {
                    error = "generation_conflict",
                    expected = currentGen + 1,
                    actual = request.Generation
                });
            }

            // Use provided generation or auto-increment
            var generation = request.Generation > 0 ? request.Generation : currentGen + 1;

            // Store the blob
            var blob = new BlobDto(
                Generation: generation,
                Data: request.Data,
                UpdatedAt: DateTimeOffset.UtcNow
            );

            await _blobStore.StoreBlobAsync(blob);

            return Ok(new { generation = blob.Generation });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to store blob");
            return StatusCode(500, new { error = "internal_error" });
        }
    }
}
