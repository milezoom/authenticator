using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Data.Core.Plugins;
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
            DisableAvaloniaDataAnnotationValidation();
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

    private void DisableAvaloniaDataAnnotationValidation()
    {
        // Get an array of plugins to remove
        var dataValidationPluginsToRemove =
            BindingPlugins.DataValidators.OfType<DataAnnotationsValidationPlugin>().ToArray();

        // remove each entry found
        foreach (var plugin in dataValidationPluginsToRemove)
        {
            BindingPlugins.DataValidators.Remove(plugin);
        }
    }

    private bool _canClose;
    private async void DesktopOnShutdownRequested(object? sender, ShutdownRequestedEventArgs e)
    {
        e.Cancel = !_canClose;

        if (!_canClose)
        {
            // var itemsToSave = _mainWindowViewModel.ToDoItems.Select(item => item.GetToDoItem());
            // await ToDoListFileService.SaveToFileAsync(itemsToSave);

            var accountsToSave = _mainWindowViewModel.Accounts.Select(item => item.GetAccount());
            await AccountFileService.SaveToFileAsync(accountsToSave);

            _canClose = true;
            if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                desktop.Shutdown();
            }
        }
    }

    private async Task InitMainViewModelAsync()
    {
        // var itemsLoaded = await ToDoListFileService.LoadItemFromFileAsync();
        // if (itemsLoaded is not null)
        // {
        //     foreach (var item in itemsLoaded)
        //     {
        //         _mainWindowViewModel.ToDoItems.Add(new ToDoItemViewModel(item));
        //     }
        // }

        var accountsLoaded = await AccountFileService.LoadItemFromFileAsync();
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