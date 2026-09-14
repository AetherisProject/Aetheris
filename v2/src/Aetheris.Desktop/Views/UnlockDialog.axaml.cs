using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace Aetheris.Desktop.Views;

public partial class UnlockDialog : Window
{
    public UnlockDialog()
    {
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
    }
}