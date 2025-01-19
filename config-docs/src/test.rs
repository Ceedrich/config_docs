use config_docs::ConfigDocs;

use crate::ConfigDocumentation;

#[test]
fn it_works_simple() {
    #[derive(ConfigDocs)]
    #[allow(unused)]
    struct Config {
        /// The speed of the player
        speed: f64,
        /// The max health of the player
        max_health: u32,
    }

    assert_eq!(
        Config::config_docs(),
        ConfigDocumentation(&[
            config_docs::ConfigDocumentationPart::Line("speed", "The speed of the player"),
            config_docs::ConfigDocumentationPart::Line(
                "max_health",
                "The max health of the player"
            )
        ])
    )
}

#[test]
fn it_works_nested() {
    #[derive(ConfigDocs)]
    #[allow(unused)]
    struct Config {
        /// Holds the colors for your app
        #[nested]
        colors: ColorConfig,
        /// Holds the keybinds for your app
        #[nested]
        keybinds: KeybindConfig,
    }

    #[derive(ConfigDocs)]
    #[allow(unused)]
    struct ColorConfig {
        /// The foreground color for your app as a hex value
        fg: String,
        /// The background color for your app as a hex value
        bg: String,
    }

    #[derive(ConfigDocs)]
    #[allow(unused)]
    struct KeybindConfig {
        /// Show the help inside your app
        help: String,
        /// Quit your app
        quit: String,
    }

    assert_eq!(
        Config::config_docs(),
        ConfigDocumentation(&[
            config_docs::ConfigDocumentationPart::SubPart(
                "colors",
                &ConfigDocumentation(&[
                    config_docs::ConfigDocumentationPart::Line(
                        "fg",
                        "The foreground color for your app as a hex value"
                    ),
                    config_docs::ConfigDocumentationPart::Line(
                        "bg",
                        "The background color for your app as a hex value"
                    )
                ])
            ),
            config_docs::ConfigDocumentationPart::SubPart(
                "keybinds",
                &ConfigDocumentation(&[
                    config_docs::ConfigDocumentationPart::Line(
                        "help",
                        "Show the help inside your app"
                    ),
                    config_docs::ConfigDocumentationPart::Line("quit", "Quit your app")
                ])
            )
        ])
    )
}
