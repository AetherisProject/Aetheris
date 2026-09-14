using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace Aetheris.Desktop.Views;

public partial class AddItemDialog : Window
{
    public AddItemDialog()
    {
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
    }
}