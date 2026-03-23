using Avalonia;
using Avalonia.Input.Platform;
using Avalonia.Controls.ApplicationLifetimes;
using System;
using System.Threading.Tasks;

namespace Authenticator.Helper;

public static class ClipboardHelper
{
    public static IClipboard GetClipboard()
    {
        if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime { MainWindow: { } window })
        {
            return window.Clipboard!;
        }

        throw new InvalidOperationException("Cannot access clipboard: No valid TopLevel or Window found.");
    }

    public static async Task SetTextAsync(string text)
    {
        await GetClipboard().SetTextAsync(text);
    }
}