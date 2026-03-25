using System;
using Authenticator.Models;
using CommunityToolkit.Mvvm.ComponentModel;
using Microsoft.VisualBasic;
using OtpNet;

namespace Authenticator.ViewModels;

public partial class AccountViewModel : ViewModelBase
{
    [ObservableProperty]
    private string? _id;

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
        Id = account.Id;
        if (string.IsNullOrWhiteSpace(Id))
        {
            Id = Guid.CreateVersion7().ToString();
        }
        AccountName = account.AccountName;
        Username = account.Username;
        Secret = account.Secret;
    }

    public Account GetAccount()
    {
        var account = new Account()
        {
            Id = this.Id,
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