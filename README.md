```rust
struct Config {
    /// Specifies colors used in the application
    theme: ThemeConfig,
    /// Specifies the keybinds used in the application
    keybinds: KeybindConfig,
}

struct KeybindConfig {
    /// Keybind to press to show help
    help: char,
    /// Keybind to press to quit the app
    quit: char,
}

struct ThemeConfig {
    /// Foreground color used in the text as a hex color
    ///
    /// Example:
    /// `fg = "#ff7fff"`
    fg: String,
    /// Background color used in the text as a hex color
    ///
    /// Example:
    /// `bg = "#00ff7f"`
    bg: String
}
```
