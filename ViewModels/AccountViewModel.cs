using System;
using Authenticator.Models;
using CommunityToolkit.Mvvm.ComponentModel;
using OtpNet;

namespace Authenticator.ViewModels;

public partial class AccountViewModel : ViewModelBase
{
    [ObservableProperty]
    private string? _accountName;

    [ObservableProperty]
    private string? _username;

    [ObservableProperty]
    private string? _secret;

    [ObservableProperty]
    private string? _code;

    public AccountViewModel()
    {
        // empty
    }

    public AccountViewModel(Account account)
    {
        AccountName = account.AccountName;
        Username = account.Username;
        Secret = account.Secret;
    }

    public Account GetAccount()
    {
        var account = new Account()
        {
            AccountName = this.AccountName,
            Username = this.Username,
            Secret = this.Secret,
        };

        return account;
    }

    /// <summary>
    /// Set totp code to field code.
    /// </summary>
    /// <returns>Remaining seconds before code need to update.</returns>
    public int SetCode()
    {
        if (!string.IsNullOrWhiteSpace(this.Secret))
        {
            var secretKey = Base32Encoding.ToBytes(this.Secret);
            var totp = new Totp(secretKey);
            Code = totp.ComputeTotp();
            // Console.WriteLine($"TOTP Updated | {AccountName} | {Code}");
            return totp.RemainingSeconds();
        }
        return 0;
    }
}