using System;
using System.Collections.ObjectModel;
using System.Threading.Tasks;
using Aetheris.Desktop.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace Aetheris.Desktop.ViewModels;

public partial class GatewayViewModel : ViewModelBase
{
    private GatewayService? _gatewayService;

    [ObservableProperty]
    private bool _isBusy;

    [ObservableProperty]
    private bool _isRunning;

    [ObservableProperty]
    private string _status = "Stopped";

    [ObservableProperty]
    private string _healthStatus = "Unknown";

    [ObservableProperty]
    private string _healthColor = "Gray";

    [ObservableProperty]
    private string _logs = string.Empty;

    [ObservableProperty]
    private string _budgetInfo = "Loading...";

    [ObservableProperty]
    private ObservableCollection<BudgetItemViewModel> _budgetItems = new();

    [ObservableProperty]
    private int _port = 8080;

    [ObservableProperty]
    private string _host = "localhost";

    public GatewayViewModel() { }

    public void Initialize(GatewayService gatewayService)
    {
        _gatewayService = gatewayService;

        _gatewayService.StatusChanged += OnStatusChanged;
        _gatewayService.LogReceived += OnLogReceived;
        _gatewayService.HealthUpdated += OnHealthUpdated;
        _gatewayService.BudgetUpdated += OnBudgetUpdated;

        // Load initial state
        LoadInitialState();
    }

    private void OnStatusChanged(object? sender, string status)
    {
        Status = status;
        IsRunning = status == "Running";
    }

    private void OnLogReceived(object? sender, string log)
    {
        Logs += log + "\n";
    }

    private void OnHealthUpdated(object? sender, string health)
    {
        HealthStatus = health;
        HealthColor = health switch
        {
            "Healthy" => "Green",
            "Degraded" => "Yellow",
            "Unhealthy" => "Red",
            _ => "Gray"
        };
    }

    private void OnBudgetUpdated(object? sender, BudgetInfo budget)
    {
        BudgetInfo = $"Budget: {budget.Used}/{budget.Limit} tokens, {budget.PercentUsed:P0} used";
        BudgetItems = new ObservableCollection<BudgetItemViewModel>(
            budget.Items.Select(item => new BudgetItemViewModel(item)));
    }

    private void LoadInitialState()
    {
        if (_gatewayService == null) return;

        IsBusy = true;
        try
        {
            _gatewayService.RefreshStatus();
            _gatewayService.RefreshHealth();
            _gatewayService.RefreshBudget();
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task StartGatewayAsync()
    {
        if (_gatewayService == null) return;

        IsBusy = true;
        try
        {
            await _gatewayService.StartAsync(Host, Port);
        }
        catch (Exception ex)
        {
            Logs += $"[Error starting gateway: {ex.Message}]\n";
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task StopGatewayAsync()
    {
        if (_gatewayService == null) return;

        IsBusy = true;
        try
        {
            await _gatewayService.StopAsync();
        }
        catch (Exception ex)
        {
            Logs += $"[Error stopping gateway: {ex.Message}]\n";
        }
        finally
        {
            IsBusy = false;
        }
    }

    [RelayCommand]
    private async Task RestartGatewayAsync()
    {
        if (_gatewayService == null) return;

        await StopGatewayAsync();
        await Task.Delay(1000);
        await StartGatewayAsync();
    }

    [RelayCommand]
    private async Task RefreshStatusAsync()
    {
        if (_gatewayService == null) return;

        await _gatewayService.RefreshStatus();
        await _gatewayService.RefreshHealth();
        await _gatewayService.RefreshBudget();
    }

    [RelayCommand]
    private async Task ClearLogsAsync()
    {
        Logs = string.Empty;
    }

    [RelayCommand]
    private async Task OpenGatewayUrlAsync()
    {
        // TODO: Open browser to gateway URL
        await Task.CompletedTask;
    }
}

public partial class BudgetItemViewModel : ViewModelBase
{
    [ObservableProperty]
    private string _name = string.Empty;

    [ObservableProperty]
    private long _usedTokens;

    [ObservableProperty]
    private long _limitTokens;

    [ObservableProperty]
    private double _percentUsed;

    public BudgetItemViewModel() { }

    public BudgetItemViewModel(BudgetItem item)
    {
        Name = item.Name;
        UsedTokens = item.UsedTokens;
        LimitTokens = item.LimitTokens;
        PercentUsed = item.PercentUsed;
    }
}

public class BudgetInfo
{
    public long Used { get; set; }
    public long Limit { get; set; }
    public double PercentUsed => Limit > 0 ? (double)Used / Limit * 100 : 0;
    public BudgetItem[] Items { get; set; } = Array.Empty<BudgetItem>();
}

public class BudgetItem
{
    public string Name { get; set; } = string.Empty;
    public long UsedTokens { get; set; }
    public long LimitTokens { get; set; }
    public double PercentUsed => LimitTokens > 0 ? (double)UsedTokens / LimitTokens * 100 : 0;
}