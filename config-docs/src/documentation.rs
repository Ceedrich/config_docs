pub type ConfigDocumentation = &'static [ConfigDocumentationPart];

#[derive(Debug, PartialEq)]
pub enum ConfigDocumentationPart {
    Line(&'static str, &'static str),
    SubPart(&'static str, ConfigDocumentation),
}

/// Trait that allows for documentation printing
pub trait ConfigDocs {
    const DOCS: ConfigDocumentation;
    /// Returns the fields in your struct with their assiciated documentation
    fn config_docs() -> ConfigDocumentation {
        Self::DOCS
    }
}
