using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using System.Threading.Tasks;
using Authenticator.Models;

namespace Authenticator.Services;

public static class AccountFileService
{
    private static readonly string _jsonFileName = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
        "Avalonia.Authenticator", "AccountList.json"
    );

    public static async Task SaveToFileAsync(IEnumerable<Account> itemsToSave)
    {
        Directory.CreateDirectory(Path.GetDirectoryName(_jsonFileName)!);
        using (var fs = File.Create(_jsonFileName))
        {
            await JsonSerializer.SerializeAsync(fs, itemsToSave);
        }
    }

    public static async Task<IEnumerable<Account>?> LoadItemFromFileAsync()
    {
        try
        {
            using (var fs = File.OpenRead(_jsonFileName))
            {
                return await JsonSerializer.DeserializeAsync<IEnumerable<Account>>(fs);
            }
        }
        catch (Exception e) when (e is FileNotFoundException || e is DirectoryNotFoundException)
        {
            return null;
        }
    }
}