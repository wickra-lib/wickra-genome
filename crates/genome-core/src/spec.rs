//! The data-driven genome specification: the feature axes, the symbol universe,
//! the cross-section normalization and the distance metric.

use crate::error::{Error, Result};
use crate::feature::Feature;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use wickra_backtest_core::registry::build;

/// Cross-section normalization applied per feature axis over the ready universe.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Normalize {
    /// Population z-score: `(x - mean) / std_pop`; a constant axis maps to `0`.
    #[default]
    ZScore,
    /// Min-max to `[0, 1]`: `(x - min) / (max - min)`; a constant axis maps to `0`.
    MinMax,
}

/// The distance metric between two normalized vectors.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Metric {
    /// Cosine distance `1 - cos_sim`; a zero-norm vector has distance `1`.
    #[default]
    Cosine,
    /// Euclidean distance.
    Euclid,
}

/// The default k-means seed (`0x5EED`), used when a spec omits `seed`.
#[must_use]
pub fn default_seed() -> u64 {
    0x5EED
}

/// A complete genome specification. `features` fixes both the dimension and the
/// axis order of every symbol vector; `symbols` fixes the universe.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GenomeSpec {
    /// The feature axes, in order. `features.len()` is the vector dimension.
    pub features: Vec<Feature>,
    /// The universe of symbols.
    pub symbols: Vec<String>,
    /// Cross-section normalization (default z-score).
    #[serde(default)]
    pub normalize: Normalize,
    /// Distance metric (default cosine).
    #[serde(default)]
    pub metric: Metric,
    /// Seed for the deterministic k-means initialization.
    #[serde(default = "default_seed")]
    pub seed: u64,
    /// Optional timeframe hint (carried through, not interpreted by the core).
    #[serde(default)]
    pub timeframe: Option<String>,
}

impl GenomeSpec {
    /// Parse a spec from JSON and validate it.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed JSON and [`Error::BadSpec`] /
    /// [`Error::UnknownIndicator`] when validation fails.
    pub fn from_json(s: &str) -> Result<Self> {
        let spec: GenomeSpec = serde_json::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Parse a spec from TOML and validate it.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed TOML and validation errors as above.
    pub fn from_toml(s: &str) -> Result<Self> {
        let spec: GenomeSpec = toml::from_str(s).map_err(|e| Error::Parse(e.to_string()))?;
        spec.validate()?;
        Ok(spec)
    }

    /// Validate structural invariants and that every referenced indicator
    /// resolves in the registry with its parameters.
    ///
    /// # Errors
    /// - [`Error::BadSpec`] if `features` or `symbols` is empty, or a feature key
    ///   or symbol is duplicated.
    /// - [`Error::UnknownIndicator`] if an indicator name or its parameters are
    ///   rejected by the registry.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.features.is_empty() {
            return Err(Error::BadSpec("features must not be empty".into()));
        }
        if self.symbols.is_empty() {
            return Err(Error::BadSpec("symbols must not be empty".into()));
        }
        let mut keys = BTreeSet::new();
        for feat in &self.features {
            if !keys.insert(feat.key()) {
                return Err(Error::BadSpec(format!("duplicate feature: {}", feat.key())));
            }
            if let Feature::Indicator { name, params, .. } = feat {
                build(name, params).map_err(|e| Error::UnknownIndicator(format!("{name}: {e}")))?;
            }
        }
        let mut syms = BTreeSet::new();
        for sym in &self.symbols {
            if !syms.insert(sym.clone()) {
                return Err(Error::BadSpec(format!("duplicate symbol: {sym}")));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::PriceField;

    fn valid_json() -> &'static str {
        r#"{"features":[{"kind":"indicator","name":"Rsi","params":[14]},
            {"kind":"price","field":"close"}],
            "symbols":["AAA","BBB"]}"#
    }

    #[test]
    fn parses_and_defaults() {
        let spec = GenomeSpec::from_json(valid_json()).unwrap();
        assert_eq!(spec.features.len(), 2);
        assert_eq!(spec.normalize, Normalize::ZScore);
        assert_eq!(spec.metric, Metric::Cosine);
        assert_eq!(spec.seed, default_seed());
    }

    #[test]
    fn empty_features_rejected() {
        let spec = GenomeSpec {
            features: vec![],
            symbols: vec!["A".into()],
            normalize: Normalize::ZScore,
            metric: Metric::Cosine,
            seed: 0,
            timeframe: None,
        };
        assert!(matches!(spec.validate(), Err(Error::BadSpec(_))));
    }

    #[test]
    fn duplicate_feature_rejected() {
        let feat = Feature::Price {
            field: PriceField::Close,
        };
        let spec = GenomeSpec {
            features: vec![feat.clone(), feat],
            symbols: vec!["A".into()],
            normalize: Normalize::ZScore,
            metric: Metric::Cosine,
            seed: 0,
            timeframe: None,
        };
        assert!(matches!(spec.validate(), Err(Error::BadSpec(_))));
    }

    #[test]
    fn duplicate_symbol_rejected() {
        let spec = GenomeSpec {
            features: vec![Feature::Price {
                field: PriceField::Close,
            }],
            symbols: vec!["A".into(), "A".into()],
            normalize: Normalize::ZScore,
            metric: Metric::Cosine,
            seed: 0,
            timeframe: None,
        };
        assert!(matches!(spec.validate(), Err(Error::BadSpec(_))));
    }

    #[test]
    fn unknown_indicator_rejected() {
        let spec = GenomeSpec {
            features: vec![Feature::Indicator {
                name: "NotAnIndicator".into(),
                params: vec![],
                field: None,
            }],
            symbols: vec!["A".into()],
            normalize: Normalize::ZScore,
            metric: Metric::Cosine,
            seed: 0,
            timeframe: None,
        };
        assert!(matches!(spec.validate(), Err(Error::UnknownIndicator(_))));
    }

    #[test]
    fn metric_and_normalize_serialize_snake_case() {
        assert_eq!(
            serde_json::to_string(&Metric::Euclid).unwrap(),
            "\"euclid\""
        );
        assert_eq!(
            serde_json::to_string(&Normalize::ZScore).unwrap(),
            "\"z_score\""
        );
    }
}
