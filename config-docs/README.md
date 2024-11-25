# Config Docs

This crate adds a trait with a derive macro to create a documentation for your
struct in code. This is useful for things like configuration objects that should
be documented in your app.

## Disclaimer

This project came into place for a small project I'm working on and is in no
means perfect. If you have any suggestions, please consider contributing on
[Github](https://github.com/Ceedrich/config_docs)

## Usage Example

```
use config_docs::ConfigDocs;

#[derive(ConfigDocs)]
struct Config {
    /// Holds the colors for your app
    colors: ColorConfig,
    /// Holds the keybinds for your app
    keybinds: KeybindConfig,
}

#[derive(ConfigDocs)]
struct ColorConfig {
    /// The foreground color for your app as a hex value
    fg: String,
    /// The background color for your app as a hex value
    bg: String,
}

#[derive(ConfigDocs)]
struct KeybindConfig {
    /// Show the help inside your app
    help: String,
    /// Quit your app
    quit: String
}

assert_eq!(Config::config_docs(), &[
    ("colors", "Holds the colors for your app"),
    ("fg", "The foreground color for your app as a hex value"),
    ("bg", "The background color for your app as a hex value"),
    ("keybinds", "Holds the keybinds for your app"),
    ("help", "Show the help inside your app"),
    ("quit", "Quit your app")
])
```
