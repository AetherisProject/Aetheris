using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace Aetheris.Desktop.Views;

public partial class GatewayView : UserControl
{
    public GatewayView()
    {
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
    }
}