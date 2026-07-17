//! One axis of the genome vector: a streaming-indicator output or a price field.

use serde::{Deserialize, Serialize};

/// A single feature — one dimension of every symbol's vector. `Indicator`
/// resolves a `wickra-core` indicator by name and parameters; `Price` reads a
/// raw OHLCV field. The order features appear in a [`crate::GenomeSpec`] is the
/// order of the axes in the resulting vector.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Feature {
    /// A streaming indicator output, resolved from the registry by `name` and
    /// `params`. `field` selects a named sub-output on multi-output indicators;
    /// `None` uses the indicator's primary value.
    Indicator {
        /// Registry indicator name (e.g. `"Rsi"`).
        name: String,
        /// Indicator parameters in registry order (e.g. `[14.0]`).
        #[serde(default)]
        params: Vec<f64>,
        /// Optional named sub-output (e.g. `"hist"` on MACD).
        #[serde(default)]
        field: Option<String>,
    },
    /// A raw price field read directly from the current candle.
    Price {
        /// Which OHLCV field.
        field: PriceField,
    },
}

/// The raw OHLCV price fields a [`Feature::Price`] can select.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PriceField {
    /// Candle open.
    Open,
    /// Candle high.
    High,
    /// Candle low.
    Low,
    /// Candle close.
    Close,
    /// Candle volume.
    Volume,
}

impl PriceField {
    /// The canonical lowercase name used inside a feature key.
    fn as_str(self) -> &'static str {
        match self {
            PriceField::Open => "open",
            PriceField::High => "high",
            PriceField::Low => "low",
            PriceField::Close => "close",
            PriceField::Volume => "volume",
        }
    }
}

impl Feature {
    /// The canonical, self-describing key for this feature axis:
    /// `price.<field>` for a price field, `<name>(<p,p>)` for an indicator, and
    /// `<name>(<p,p>).<field>` when a named sub-output is selected. Parameters
    /// render with `{}` (so `14.0` becomes `14`, `2.5` stays `2.5`).
    #[must_use]
    pub fn key(&self) -> String {
        match self {
            Feature::Price { field } => format!("price.{}", field.as_str()),
            Feature::Indicator {
                name,
                params,
                field,
            } => {
                let params = params
                    .iter()
                    .map(|p| format!("{p}"))
                    .collect::<Vec<_>>()
                    .join(",");
                let base = format!("{name}({params})");
                match field {
                    Some(f) => format!("{base}.{f}"),
                    None => base,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_key_is_prefixed() {
        assert_eq!(
            Feature::Price {
                field: PriceField::Close
            }
            .key(),
            "price.close"
        );
    }

    #[test]
    fn indicator_key_renders_params() {
        assert_eq!(
            Feature::Indicator {
                name: "Rsi".into(),
                params: vec![14.0],
                field: None,
            }
            .key(),
            "Rsi(14)"
        );
    }

    #[test]
    fn indicator_key_with_field_and_multiple_params() {
        assert_eq!(
            Feature::Indicator {
                name: "Macd".into(),
                params: vec![12.0, 26.0, 9.0],
                field: Some("hist".into()),
            }
            .key(),
            "Macd(12,26,9).hist"
        );
    }

    #[test]
    fn paramless_indicator_key_has_empty_parens() {
        assert_eq!(
            Feature::Indicator {
                name: "AnchoredRsi".into(),
                params: vec![],
                field: None,
            }
            .key(),
            "AnchoredRsi()"
        );
    }
}
