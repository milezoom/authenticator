using System.Collections.ObjectModel;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using AuthAvalonia.Helper;

namespace AuthAvalonia.ViewModels;

public partial class MainWindowViewModel : ViewModelBase
{
    public ObservableCollection<AccountViewModel> Accounts { get; } = new ObservableCollection<AccountViewModel>();

    [ObservableProperty]
    [NotifyCanExecuteChangedFor(nameof(AddAccountCommand))]
    private string? _newAccountName;

    [ObservableProperty]
    [NotifyCanExecuteChangedFor(nameof(AddAccountCommand))]
    private string? _newUsername;

    [ObservableProperty]
    [NotifyCanExecuteChangedFor(nameof(AddAccountCommand))]
    private string? _newSecret;

    private bool CanStoreAccount() =>
        !string.IsNullOrWhiteSpace(NewAccountName) &&
        !string.IsNullOrWhiteSpace(NewUsername) &&
        !string.IsNullOrWhiteSpace(NewSecret);

    [RelayCommand(CanExecute = nameof(CanStoreAccount))]
    private void AddAccount()
    {
        var account = new AccountViewModel()
        {
            AccountName = NewAccountName,
            Username = NewUsername,
            Secret = NewSecret,
        };
        account.SetCode();
        Accounts.Add(account);

        NewAccountName = null;
        NewUsername = null;
        NewSecret = null;
    }

    [RelayCommand]
    private void RemoveAccount(AccountViewModel account)
    {
        Accounts.Remove(account);
    }

    [RelayCommand]
    private static void CopyCode(AccountViewModel account)
    {
        _ = ClipboardHelper.SetTextAsync(account.Code!);
    }
}
