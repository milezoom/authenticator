using System;
using System.Collections.Generic;
using Avalonia.Controls;
using Avalonia.Controls.Templates;
using Authenticator.ViewModels;
using Authenticator.Views;

namespace Authenticator;

public class ViewLocator : IDataTemplate
{
    private static readonly Dictionary<Type, Func<Control>> ViewFactories = new()
    {
        [typeof(MainWindowViewModel)] = () => new MainWindow(),
    };

    public Control? Build(object? param)
    {
        if (param is null)
            return null;

        if (ViewFactories.TryGetValue(param.GetType(), out var factory))
            return factory();

        return new TextBlock { Text = "Not Found: " + param.GetType().Name };
    }

    public bool Match(object? data)
    {
        return data is ViewModelBase;
    }
}
