//! The CLI-facing config file wrapper: a spec loaded from a JSON or TOML file.

use crate::error::{Error, Result};
use crate::spec::GenomeSpec;
use serde::{Deserialize, Serialize};

/// A config document: just the genome spec, loadable from JSON or TOML so the CLI
/// can accept either extension.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    /// The genome spec.
    pub spec: GenomeSpec,
}

impl Config {
    /// Parse a config from JSON. Accepts either a bare spec object or a
    /// `{ "spec": { ... } }` wrapper.
    ///
    /// # Errors
    /// Parse or validation errors.
    pub fn from_json(s: &str) -> Result<Self> {
        if let Ok(cfg) = serde_json::from_str::<Config>(s) {
            cfg.spec.validate()?;
            return Ok(cfg);
        }
        let spec = GenomeSpec::from_json(s)?;
        Ok(Config { spec })
    }

    /// Parse a config from TOML. Accepts either a bare spec table or a
    /// `[spec]` wrapper.
    ///
    /// # Errors
    /// Parse or validation errors.
    pub fn from_toml(s: &str) -> Result<Self> {
        if let Ok(cfg) = toml::from_str::<Config>(s) {
            cfg.spec.validate()?;
            return Ok(cfg);
        }
        let spec = GenomeSpec::from_toml(s)?;
        Ok(Config { spec })
    }

    /// Load a config from a file, picking JSON or TOML by extension.
    ///
    /// # Errors
    /// [`Error::Data`] on read failure, then parse/validation errors.
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|e| Error::Data(e.to_string()))?;
        if path.extension().is_some_and(|e| e == "toml") {
            Self::from_toml(&text)
        } else {
            Self::from_json(&text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_spec_json_loads() {
        let s = r#"{"features":[{"kind":"price","field":"close"}],"symbols":["A"]}"#;
        let cfg = Config::from_json(s).unwrap();
        assert_eq!(cfg.spec.symbols, vec!["A"]);
    }

    #[test]
    fn wrapped_spec_json_loads() {
        let s = r#"{"spec":{"features":[{"kind":"price","field":"close"}],"symbols":["A"]}}"#;
        let cfg = Config::from_json(s).unwrap();
        assert_eq!(cfg.spec.symbols, vec!["A"]);
    }
}
