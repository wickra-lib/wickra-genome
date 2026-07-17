//! The cross-section of all symbols: a `BTreeMap` from symbol to its rolling
//! state, plus the extraction of the ready rows in symbol-key order.

use crate::error::Result;
use crate::spec::GenomeSpec;
use crate::symbol_state::SymbolState;
use std::collections::BTreeMap;
use wickra_backtest_core::Candle;

/// The universe of symbols and their rolling states. A `BTreeMap` keeps symbols
/// in a deterministic key order — every reduction downstream iterates in this
/// order so `f64` rounding is identical across languages and thread counts.
pub(crate) struct Universe {
    pub(crate) symbols: BTreeMap<String, SymbolState>,
}

impl Universe {
    /// An empty universe.
    pub(crate) fn new() -> Self {
        Self {
            symbols: BTreeMap::new(),
        }
    }

    /// Ensure a symbol has a state (creating it from the spec if absent). Errors
    /// if the spec references an indicator the registry does not know.
    pub(crate) fn ensure(&mut self, symbol: &str, spec: &GenomeSpec) -> Result<()> {
        if !self.symbols.contains_key(symbol) {
            self.symbols
                .insert(symbol.to_string(), SymbolState::new(spec)?);
        }
        Ok(())
    }

    /// Fold one candle into a symbol's state, creating the state if needed.
    pub(crate) fn fold(&mut self, symbol: &str, candle: &Candle, spec: &GenomeSpec) -> Result<()> {
        self.ensure(symbol, spec)?;
        if let Some(state) = self.symbols.get_mut(symbol) {
            state.fold(candle);
        }
        Ok(())
    }

    /// The raw feature vectors of every ready symbol, in symbol-key order.
    pub(crate) fn ready(&self, spec: &GenomeSpec) -> Vec<(String, Vec<f64>)> {
        self.symbols
            .iter()
            .filter_map(|(sym, state)| state.raw_vector(spec).map(|v| (sym.clone(), v)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{Feature, PriceField};
    use crate::spec::{Metric, Normalize};

    fn candle(close: f64) -> Candle {
        Candle {
            time: 0,
            open: close,
            high: close,
            low: close,
            close,
            volume: 0.0,
        }
    }

    fn spec() -> GenomeSpec {
        GenomeSpec {
            features: vec![Feature::Price {
                field: PriceField::Close,
            }],
            symbols: vec!["A".into(), "B".into()],
            normalize: Normalize::ZScore,
            metric: Metric::Cosine,
            seed: 0,
            timeframe: None,
        }
    }

    #[test]
    fn ready_rows_in_key_order() {
        let spec = spec();
        let mut u = Universe::new();
        // Insert B first, then A; the BTreeMap must still return A before B.
        u.fold("B", &candle(2.0), &spec).unwrap();
        u.fold("A", &candle(1.0), &spec).unwrap();
        let rows = u.ready(&spec);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "A");
        assert_eq!(rows[1].0, "B");
    }
}
