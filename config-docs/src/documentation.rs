use std::fmt::Display;

/// Trait that allows for documentation printing
pub trait ConfigDocs {
    const CONFIG_DOCS: ConfigDocumentation;
    /// Returns the fields in your struct with their assiciated documentation
    fn config_docs() -> ConfigDocumentation {
        Self::CONFIG_DOCS
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigDocumentation(pub &'static [ConfigDocumentationPart]);

impl ConfigDocumentation {
    /// Returns the configuration as a flattened collection of tuples containing the key and the
    /// description.
    ///
    /// # Example
    /// ```
    /// use config_docs::ConfigDocs;
    /// #[derive(ConfigDocs)]
    /// struct Config {
    ///   /// Move the player up
    ///   up: String,
    ///   /// Move the player down
    ///   down: String,
    /// }
    ///
    /// assert_eq!(
    ///   Config::config_docs().as_tuples(),
    ///   &[("up", "Move the player up"), ("down","Move the player down")]
    /// );
    /// ```
    pub fn as_tuples(&self) -> &[(&'static str, &'static str)] {
        let mut vec = Vec::new();
        for e in self.0 {
            match e {
                ConfigDocumentationPart::Line(k, v) => vec.push((*k, *v)),
                ConfigDocumentationPart::SubPart(k, d) => {
                    vec.push((*k, ""));
                    vec.extend(d.as_tuples())
                }
            }
        }
        vec.leak()
    }
}

impl Display for ConfigDocumentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.0 {
            writeln!(f, "{}", entry)?
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigDocumentationPart {
    Line(&'static str, &'static str),
    SubPart(&'static str, &'static ConfigDocumentation),
}

impl Display for ConfigDocumentationPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigDocumentationPart::Line(key, value) => {
                writeln!(f, "{:<20} {value}", format!("{key}:"))
            }
            ConfigDocumentationPart::SubPart(key, docs) => {
                writeln!(f, "{key}")?;
                writeln!(f, "{docs}")
            }
        }
    }
}
