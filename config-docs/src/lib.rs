//! # Config Docs
//!
//! This crate adds a trait with a derive macro to create a documentation for your struct in code.
//! This is useful for things like configuration objects that should be documented in your app.
//!
//!
//! ## Disclaimer
//! This project came into place for a small project I'm working on and is in no means perfect. If
//! you have any suggestions, please consider contributing on [Github](https://github.com/Ceedrich/config_docs)
//!
//! ## Usage Example
//! ```
//! use config_docs::ConfigDocs;
//!
//! #[derive(ConfigDocs)]
//! struct Config {
//!     /// Holds the colors for your app
//!     #[nested]
//!     colors: ColorConfig,
//!     /// Holds the keybinds for your app
//!     #[nested]
//!     keybinds: KeybindConfig,
//! }
//!
//! #[derive(ConfigDocs)]
//! struct ColorConfig {
//!     /// The foreground color for your app as a hex value
//!     fg: String,
//!     /// The background color for your app as a hex value
//!     bg: String,
//! }
//!
//! #[derive(ConfigDocs)]
//! struct KeybindConfig {
//!     /// Show the help inside your app
//!     help: String,
//!     /// Quit your app
//!     quit: String
//! }
//!
//! // Print it directly
//! println!("{}", Config::config_docs());
//!
//! // or get it as a list of key-value pairs to display yourself
//! let kv: &[(&str, &str)] = Config::config_docs().as_tuples();
//! ```
//!

mod documentation;
extern crate self as config_docs;

#[cfg(test)]
mod test;

pub use documentation::*;
pub use config_docs_macros::*;
