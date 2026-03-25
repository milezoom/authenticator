using System;
using System.Collections.Generic;
using System.IO;
using System.Threading.Tasks;
using Authenticator.Models;
using Microsoft.Data.Sqlite;

namespace Authenticator.Services;

public static class AccountFileService
{
    private static readonly string _sqliteFileName = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
        "Avalonia.Authenticator", "AccountList.db"
    );

    public static async Task CheckOrCreateTableInDBAsync()
    {
        using var connection = new SqliteConnection($"Data Source={_sqliteFileName}");
        await connection.OpenAsync();

        using var command = connection.CreateCommand();
        command.CommandText = "select name from sqlite_master where type='table' and name = 'account';";
        var result = await command.ExecuteScalarAsync();

        if (result == null)
        {
            command.CommandText = "create table account (id text primary key, account_name text, username text, secret text);";
            await command.ExecuteNonQueryAsync();
        }

        await connection.CloseAsync();
    }

    public static async Task<IEnumerable<Account>?> LoadFromDBAsync()
    {
        await CheckOrCreateTableInDBAsync();
        var accounts = new List<Account>();

        using var connection = new SqliteConnection($"Data Source={_sqliteFileName}");
        await connection.OpenAsync();

        using var command = connection.CreateCommand();
        command.CommandText = "select id, account_name, username, secret from account;";

        using var reader = await command.ExecuteReaderAsync();
        while (await reader.ReadAsync())
        {
            string id = reader.GetString(0);
            string accountName = reader.GetString(1);
            string username = reader.GetString(2);
            string secret = reader.GetString(3);
            var acc = new Account()
            {
                Id = id,
                AccountName = accountName,
                Username = username,
                Secret = secret
            };
            accounts.Add(acc);
        }

        await connection.CloseAsync();
        return accounts;
    }

    public static async Task StoreToDBAsync(IEnumerable<Account> itemsToSave)
    {
        await CheckOrCreateTableInDBAsync();

        using var connection = new SqliteConnection($"Data Source={_sqliteFileName}");
        await connection.OpenAsync();

        foreach (var item in itemsToSave)
        {
            using var command = connection.CreateCommand();
            command.CommandText = """
                insert into account(id, account_name, username, secret)
                values ($id, $account_name, $username, $secret)
                on conflict(id) do update set
                    account_name = excluded.account_name,
                    username = excluded.username,
                    secret = excluded.secret;
                """;
            if (string.IsNullOrWhiteSpace(item.Id))
            {
                item.Id = Guid.CreateVersion7().ToString();
            }
            command.Parameters.AddWithValue("$id", item.Id);
            command.Parameters.AddWithValue("$account_name", item.AccountName);
            command.Parameters.AddWithValue("$username", item.Username);
            command.Parameters.AddWithValue("$secret", item.Secret);
            await command.ExecuteNonQueryAsync();
        }

        await connection.CloseAsync();
    }

}