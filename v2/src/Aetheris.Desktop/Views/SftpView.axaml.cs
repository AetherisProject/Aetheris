using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace Aetheris.Desktop.Views;

public partial class SftpView : UserControl
{
    public SftpView()
    {
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
    }
}