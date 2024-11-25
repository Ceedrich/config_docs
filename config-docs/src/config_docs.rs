/// Trait that allows for documentation printing
pub trait ConfigDocs {
    /// Returns the fields in your struct with their assiciated documentation
    fn config_docs() -> &'static [(&'static str, &'static str)];
}

macro_rules! default_impl_for_types {
    ($($t:ty),*) => {
        $(
            impl ConfigDocs for $t {
                fn config_docs() -> &'static [(&'static str, &'static str)] {
                    &[]
                }
            }
        )*
    };
}

// Implement MyTrait for all integers and floating-point types
default_impl_for_types!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, String, &str
);

#[cfg(test)]
mod test {
    use super::*;
    use config_docs_macros::ConfigDocs;

    #[test]
    fn it_parses_structs() {
        #[allow(unused)]
        #[derive(ConfigDocs)]
        struct ThemeConfig {
            /// Foreground color used in the text as a hex color
            ///
            /// # Example:
            /// `fg = "#7f00ff"`
            fg: String,
            /// Background color used in the text as a hex color
            ///
            /// # Example:
            /// `bg = "#7f00ff"`
            bg: String,
        }

        assert_eq!(
            ThemeConfig::config_docs(),
            &[
                ("fg", "Foreground color used in the text as a hex color"),
                ("bg", "Background color used in the text as a hex color"),
            ]
        )
    }

    #[test]
    fn it_parses_nested_structs() {
        #[derive(ConfigDocs)]
        #[allow(unused)]
        struct Config {
            /// Sets the colors used throughout the app
            pub theme: ThemeConfig,
            /// Sets the keybinds used throughout the app
            pub keybinds: KeybindsConfig,
        }
        #[derive(ConfigDocs)]
        #[allow(unused)]
        struct ThemeConfig {
            /// Foreground color used in the text as a hex color
            ///
            /// # Example:
            /// `fg = "#7f00ff"`
            fg: String,
            /// Background color used in the text as a hex color
            ///
            /// # Example:
            /// `bg = "#7f00ff"`
            bg: String,
        }
        #[derive(ConfigDocs)]
        #[allow(unused)]
        struct KeybindsConfig {
            /// show help
            help: String,
            /// quit app
            quit: String,
        }

        assert_eq!(
            Config::config_docs(),
            &[
                ("theme", "Sets the colors used throughout the app"),
                ("fg", "Foreground color used in the text as a hex color"),
                ("bg", "Background color used in the text as a hex color"),
                ("keybinds", "Sets the keybinds used throughout the app"),
                ("help", "show help"),
                ("quit", "quit app")
            ]
        )
    }
}
