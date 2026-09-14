namespace Aetheris.Hub.Services;

public interface IDeviceStore
{
    Task<Models.Device?> GetDeviceAsync(string deviceId);
    Task StoreDeviceAsync(Models.Device device);
    Task<bool> DeviceExistsAsync(string deviceId);
}
