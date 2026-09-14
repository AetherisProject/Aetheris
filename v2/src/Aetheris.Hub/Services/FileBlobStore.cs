using System.Text.Json;

namespace Aetheris.Hub.Services;

public sealed class FileBlobStore : IBlobStore
{
    private readonly string _basePath;
    private readonly int _maxGenerations;

    public FileBlobStore(string basePath, int maxGenerations = 50)
    {
        _basePath = basePath ?? throw new ArgumentNullException(nameof(basePath));
        _maxGenerations = maxGenerations;
        Directory.CreateDirectory(_basePath);
    }

    public async Task<int> GetLatestGenerationAsync()
    {
        var files = Directory.GetFiles(_basePath, "*.blob.json")
            .Select(Path.GetFileNameWithoutExtension)
            .Where(name => int.TryParse(name, out _))
            .Select(name => int.Parse(name!))
            .OrderByDescending(g => g)
            .ToArray();

        return files.Length > 0 ? files[0] : 0;
    }

    public async Task<int[]> GetAvailableGenerationsAsync(int maxCount = 50)
    {
        var files = Directory.GetFiles(_basePath, "*.blob.json")
            .Select(Path.GetFileNameWithoutExtension)
            .Where(name => int.TryParse(name, out _))
            .Select(name => int.Parse(name!))
            .OrderByDescending(g => g)
            .Take(maxCount)
            .ToArray();

        return files;
    }

    public async Task<Models.BlobDto?> GetBlobAsync(int generation)
    {
        var path = Path.Combine(_basePath, $"{generation}.blob.json");
        if (!File.Exists(path))
            return null;

        var json = await File.ReadAllTextAsync(path);
        return JsonSerializer.Deserialize<Models.BlobDto>(json);
    }

    public async Task StoreBlobAsync(Models.BlobDto blob)
    {
        var path = Path.Combine(_basePath, $"{blob.Generation}.blob.json");
        var tmpPath = path + ".tmp";

        var json = JsonSerializer.Serialize(blob);
        await File.WriteAllTextAsync(tmpPath, json);
        File.Move(tmpPath, path, overwrite: true);

        // Clean up old generations beyond maxGenerations
        var files = Directory.GetFiles(_basePath, "*.blob.json")
            .Select(Path.GetFileNameWithoutExtension)
            .Where(name => int.TryParse(name, out _))
            .Select(name => int.Parse(name!))
            .OrderBy(g => g)
            .ToArray();

        if (files.Length > _maxGenerations)
        {
            foreach (var oldGen in files.Take(files.Length - _maxGenerations))
            {
                var oldPath = Path.Combine(_basePath, $"{oldGen}.blob.json");
                try { File.Delete(oldPath); } catch { /* ignore */ }
            }
        }
    }

    public async Task<bool> ExistsAsync(int generation)
    {
        var path = Path.Combine(_basePath, $"{generation}.blob.json");
        return File.Exists(path);
    }
}
