using System.Text.Json;

namespace Aetheris.Hub.Services;

public sealed class FileDeviceStore : IDeviceStore
{
    private readonly string _devicesPath;

    public FileDeviceStore(string basePath)
    {
        _devicesPath = Path.Combine(basePath, "devices");
        Directory.CreateDirectory(_devicesPath);
    }

    public async Task<Models.Device?> GetDeviceAsync(string deviceId)
    {
        var path = Path.Combine(_devicesPath, $"{deviceId}.json");
        if (!File.Exists(path))
            return null;

        var json = await File.ReadAllTextAsync(path);
        return JsonSerializer.Deserialize<Models.Device>(json);
    }

    public async Task StoreDeviceAsync(Models.Device device)
    {
        var path = Path.Combine(_devicesPath, $"{device.Id}.json");
        var tmpPath = path + ".tmp";

        var json = JsonSerializer.Serialize(device, new JsonSerializerOptions { WriteIndented = true });
        await File.WriteAllTextAsync(tmpPath, json);
        File.Move(tmpPath, path, overwrite: true);
    }

    public async Task<bool> DeviceExistsAsync(string deviceId)
    {
        var path = Path.Combine(_devicesPath, $"{deviceId}.json");
        return File.Exists(path);
    }
}
