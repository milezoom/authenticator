using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using System.Linq;
using Avalonia.Markup.Xaml;
using Authenticator.ViewModels;
using Authenticator.Views;
using Authenticator.Services;
using System.Threading.Tasks;
using System.ComponentModel;
using System.Threading;

namespace Authenticator;

public partial class App : Application
{
    private readonly MainWindowViewModel _mainWindowViewModel = new MainWindowViewModel();

    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
    }

    public override async void OnFrameworkInitializationCompleted()
    {
        if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        {

            desktop.MainWindow = new MainWindow
            {
                DataContext = _mainWindowViewModel,
            };
            desktop.ShutdownRequested += DesktopOnShutdownRequested;
        }

        base.OnFrameworkInitializationCompleted();

        await InitMainViewModelAsync();
        RunAccountCodeWorker();
    }

    private bool _canClose;
    private async void DesktopOnShutdownRequested(object? sender, ShutdownRequestedEventArgs e)
    {
        e.Cancel = !_canClose;

        if (!_canClose)
        {
            var accountsToSave = _mainWindowViewModel.Accounts.Select(item => item.GetAccount());
            await AccountFileService.StoreToDBAsync(accountsToSave);

            _canClose = true;
            if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                desktop.Shutdown();
            }
        }
    }

    private async Task InitMainViewModelAsync()
    {
        var accountsLoaded = await AccountFileService.LoadFromDBAsync();
        if (accountsLoaded is not null)
        {
            foreach (var item in accountsLoaded)
            {
                var account = new AccountViewModel(item);
                _mainWindowViewModel.Accounts.Add(account);
            }
        }
    }

    private void RunAccountCodeWorker()
    {
        var worker = new BackgroundWorker();
        var duration = 0;
        worker.DoWork += (sender, e) =>
        {
            while (!worker.CancellationPending)
            {
                foreach (var item in _mainWindowViewModel.Accounts)
                {
                    var currDuration = item.SetCode();
                    if (currDuration > duration)
                    {
                        duration = currDuration;
                    }
                }
                Thread.Sleep(duration * 1000);
            }
        };
        worker.RunWorkerAsync();
    }
}