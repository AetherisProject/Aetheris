using System.Text.Json;
using Amazon.S3;
using Amazon.S3.Model;

namespace Aetheris.Hub.Services;

public sealed class S3BlobStore : IBlobStore
{
    private readonly IAmazonS3 _s3Client;
    private readonly string _bucketName;
    private readonly string _keyPrefix;
    private readonly int _maxGenerations;

    public S3BlobStore(IAmazonS3 s3Client, string bucketName, string keyPrefix = "blobs/", int maxGenerations = 50)
    {
        _s3Client = s3Client ?? throw new ArgumentNullException(nameof(s3Client));
        _bucketName = bucketName ?? throw new ArgumentNullException(nameof(bucketName));
        _keyPrefix = keyPrefix ?? throw new ArgumentNullException(nameof(keyPrefix));
        _maxGenerations = maxGenerations;
    }

    private string GetKey(int generation) => $"{_keyPrefix}{generation}.blob.json";

    public async Task<int> GetLatestGenerationAsync()
    {
        var request = new ListObjectsV2Request
        {
            BucketName = _bucketName,
            Prefix = _keyPrefix,
            Delimiter = "/"
        };

        var response = await _s3Client.ListObjectsV2Async(request);
        var generations = response.S3Objects
            .Select(o => Path.GetFileNameWithoutExtension(o.Key))
            .Where(name => int.TryParse(name, out _))
            .Select(name => int.Parse(name!))
            .OrderByDescending(g => g)
            .ToArray();

        return generations.Length > 0 ? generations[0] : 0;
    }

    public async Task<int[]> GetAvailableGenerationsAsync(int maxCount = 50)
    {
        var request = new ListObjectsV2Request
        {
            BucketName = _bucketName,
            Prefix = _keyPrefix,
            Delimiter = "/"
        };

        var response = await _s3Client.ListObjectsV2Async(request);
        var generations = response.S3Objects
            .Select(o => Path.GetFileNameWithoutExtension(o.Key))
            .Where(name => int.TryParse(name, out _))
            .Select(name => int.Parse(name!))
            .OrderByDescending(g => g)
            .Take(maxCount)
            .ToArray();

        return generations;
    }

    public async Task<Models.BlobDto?> GetBlobAsync(int generation)
    {
        var key = GetKey(generation);
        try
        {
            var response = await _s3Client.GetObjectAsync(_bucketName, key);
            using var reader = new StreamReader(response.ResponseStream);
            var json = await reader.ReadToEndAsync();
            return JsonSerializer.Deserialize<Models.BlobDto>(json);
        }
        catch (AmazonS3Exception ex) when (ex.StatusCode == System.Net.HttpStatusCode.NotFound)
        {
            return null;
        }
    }

    public async Task StoreBlobAsync(Models.BlobDto blob)
    {
        var key = GetKey(blob.Generation);
        var json = JsonSerializer.Serialize(blob);
        var content = new StringContent(json);

        await _s3Client.PutObjectAsync(new PutObjectRequest
        {
            BucketName = _bucketName,
            Key = key,
            ContentBody = json
        });

        // Clean up old generations beyond maxGenerations
        var request = new ListObjectsV2Request
        {
            BucketName = _bucketName,
            Prefix = _keyPrefix,
            Delimiter = "/"
        };

        var response = await _s3Client.ListObjectsV2Async(request);
        var generations = response.S3Objects
            .Select(o => Path.GetFileNameWithoutExtension(o.Key))
            .Where(name => int.TryParse(name, out _))
            .Select(name => int.Parse(name!))
            .OrderBy(g => g)
            .ToArray();

        if (generations.Length > _maxGenerations)
        {
            foreach (var oldGen in generations.Take(generations.Length - _maxGenerations))
            {
                var oldKey = GetKey(oldGen);
                try { await _s3Client.DeleteObjectAsync(_bucketName, oldKey); } catch { /* ignore */ }
            }
        }
    }

    public async Task<bool> ExistsAsync(int generation)
    {
        var key = GetKey(generation);
        try
        {
            var response = await _s3Client.GetObjectAsync(_bucketName, key);
            return response != null;
        }
        catch (AmazonS3Exception ex) when (ex.StatusCode == System.Net.HttpStatusCode.NotFound)
        {
            return false;
        }
    }
}
