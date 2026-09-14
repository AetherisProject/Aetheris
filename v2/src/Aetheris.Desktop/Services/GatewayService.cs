using System;
using System.Diagnostics;
using System.Net.Http;
using System.Threading.Tasks;

namespace Aetheris.Desktop.Services;

public class GatewayService
{
    private Process? _gatewayProcess;
    private HttpClient? _httpClient;
    private bool _isRunning = false;
    private string _gatewayUrl = "http://localhost:8080";

    public event EventHandler<string>? StatusChanged;
    public event EventHandler<string>? LogReceived;
    public event EventHandler<string>? HealthUpdated;
    public event EventHandler<BudgetInfo>? BudgetUpdated;

    public bool IsRunning => _isRunning;

    public GatewayService()
    {
        _httpClient = new HttpClient();
    }

    public async Task StartAsync(string host, int port)
    {
        if (_isRunning) return;

        try
        {
            // This would start the Aetheris.Gateway process
            var processInfo = new ProcessStartInfo
            {
                FileName = "dotnet",
                Arguments = "run --project src/Aetheris.Gateway --urls http://localhost:8080",
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                CreateNoWindow = true
            };

            _gatewayProcess = new Process { StartInfo = processInfo };
            _gatewayProcess.OutputDataReceived += (sender, e) => 
                LogReceived?.Invoke(this, e.Data ?? string.Empty);
            _gatewayProcess.ErrorDataReceived += (sender, e) => 
                LogReceived?.Invoke(this, $"[ERROR] {e.Data}");

            _gatewayProcess.Start();
            _gatewayProcess.BeginOutputReadLine();
            _gatewayProcess.BeginErrorReadLine();

            _gatewayUrl = $"http://{host}:{port}";
            _isRunning = true;
            StatusChanged?.Invoke(this, "Running");

            // Start monitoring health
            _ = MonitorHealthAsync();
        }
        catch (Exception ex)
        {
            StatusChanged?.Invoke(this, $"Error: {ex.Message}");
            throw;
        }
    }

    public async Task StopAsync()
    {
        if (!_isRunning) return;

        try
        {
            if (_gatewayProcess != null && !_gatewayProcess.HasExited)
            {
                _gatewayProcess.Kill();
                await _gatewayProcess.WaitForExitAsync();
                _gatewayProcess.Dispose();
                _gatewayProcess = null;
            }

            _isRunning = false;
            StatusChanged?.Invoke(this, "Stopped");
        }
        catch (Exception ex)
        {
            StatusChanged?.Invoke(this, $"Error stopping: {ex.Message}");
        }
    }

    public async Task RefreshStatus()
    {
        if (_gatewayProcess == null || _gatewayProcess.HasExited)
        {
            _isRunning = false;
            StatusChanged?.Invoke(this, "Stopped");
        }
        else
        {
            StatusChanged?.Invoke(this, "Running");
        }
    }

    public async Task RefreshHealth()
    {
        if (_httpClient == null) return;

        try
        {
            var response = await _httpClient.GetAsync($"{_gatewayUrl}/health");
            if (response.IsSuccessStatusCode)
            {
                var health = await response.Content.ReadAsStringAsync();
                HealthUpdated?.Invoke(this, health);
            }
        }
        catch { }
    }

    public async Task RefreshBudget()
    {
        if (_httpClient == null) return;

        try
        {
            var response = await _httpClient.GetAsync($"{_gatewayUrl}/api/budget");
            if (response.IsSuccessStatusCode)
            {
                // Parse budget info from response
                var budget = new BudgetInfo();
                BudgetUpdated?.Invoke(this, budget);
            }
        }
        catch { }
    }

    private async Task MonitorHealthAsync()
    {
        while (_isRunning)
        {
            await RefreshHealth();
            await Task.Delay(10000); // Check every 10 seconds
        }
    }

    public string GetGatewayUrl() => _gatewayUrl;
}