using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Aetheris.Desktop.ViewModels;

namespace Aetheris.Desktop.Views;

public partial class SshView : UserControl
{
    public SshView()
    {
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
    }

    private void OnTerminalKeyDown(object sender, KeyEventArgs e)
    {
        if (DataContext is SshViewModel viewModel)
        {
            if (e.Key == Key.Enter)
            {
                viewModel.SendCommandCommand.Execute(null);
                e.Handled = true;
            }
            else if (e.Key == Key.Back)
            {
                viewModel.HandleKeyPress("Backspace");
                e.Handled = true;
            }
        }
    }
}