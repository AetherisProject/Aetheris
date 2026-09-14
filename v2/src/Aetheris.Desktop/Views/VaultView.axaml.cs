using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace Aetheris.Desktop.Views;

public partial class VaultView : UserControl
{
    public VaultView()
    {
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
    }
}