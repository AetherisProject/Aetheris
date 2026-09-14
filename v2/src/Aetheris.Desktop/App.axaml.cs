using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;
using Aetheris.Desktop.ViewModels;
using Aetheris.Desktop.Views;
using Microsoft.Extensions.DependencyInjection;
using Aetheris.Desktop.Services;

namespace Aetheris.Desktop;

public partial class App : Application
{
    private IServiceProvider? _serviceProvider;

    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
    }

    public override void OnFrameworkInitializationCompleted()
    {
        if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        {
            // Configure DI container
            var services = new ServiceCollection();
            ConfigureServices(services);
            _serviceProvider = services.BuildServiceProvider();

            // Set up main window with DI
            var mainWindow = new MainWindow
            {
                DataContext = _serviceProvider.GetRequiredService<MainViewModel>()
            };

            desktop.MainWindow = mainWindow;
        }

        base.OnFrameworkInitializationCompleted();
    }

    private void ConfigureServices(IServiceCollection services)
    {
        // Services
        services.AddSingleton<VaultService>();
        services.AddSingleton<SshService>();
        services.AddSingleton<SftpService>();
        services.AddSingleton<GatewayService>();
        services.AddSingleton<UpdateService>();

        // ViewModels
        services.AddSingleton<MainViewModel>();
        services.AddTransient<VaultViewModel>();
        services.AddTransient<SshViewModel>();
        services.AddTransient<SftpViewModel>();
        services.AddTransient<GatewayViewModel>();

        // Register factory for dialogs
        services.AddTransient<UnlockDialogViewModel>();
        services.AddTransient<AddItemDialogViewModel>();
        services.AddTransient<ConnectionDialogViewModel>();
    }

    public IServiceProvider? ServiceProvider => _serviceProvider;
}