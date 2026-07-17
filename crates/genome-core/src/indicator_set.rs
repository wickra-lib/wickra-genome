//! Resolves the indicators a spec references and folds candles through them.
//!
//! Indicators are resolved by name and parameters from the `wickra-core`
//! registry — reused through the `wickra-backtest-core` factory, the only
//! name -> indicator resolver in the ecosystem. Each resolved indicator is an
//! object-safe `EvalIndicator`; the genome drives it with a candle-only
//! [`BarInput`] (no reference series, derivatives, order book or trades). Unlike
//! the screener, the genome keeps no previous-bar values: a vector is a snapshot
//! of the latest bar, with no crossover semantics.

use crate::error::{Error, Result};
use crate::feature::Feature;
use std::collections::BTreeMap;
use wickra_backtest_core::registry::{build, BarInput};
use wickra_backtest_core::{Candle, EvalIndicator};

/// One resolved indicator plus its canonical base key (`<name>(<p,p>)`).
struct Entry {
    key: String,
    indicator: Box<dyn EvalIndicator>,
}

/// The set of indicators a spec needs, folded one candle at a time. Each
/// `update` records the primary value under the indicator's base key and every
/// named sub-output under `<base>.<field>`.
pub(crate) struct IndicatorSet {
    items: Vec<Entry>,
    cur: BTreeMap<String, f64>,
}

impl IndicatorSet {
    /// An empty set.
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            cur: BTreeMap::new(),
        }
    }

    /// Register the indicator a feature needs (price fields need none).
    /// Idempotent per base key. Errors if the registry does not know the
    /// indicator or rejects its parameters.
    pub(crate) fn required(&mut self, feature: &Feature) -> Result<()> {
        if let Feature::Indicator { name, params, .. } = feature {
            let key = base_key(name, params);
            if self.items.iter().all(|e| e.key != key) {
                let indicator = build(name, params)
                    .map_err(|e| Error::UnknownIndicator(format!("{name}: {e}")))?;
                self.items.push(Entry { key, indicator });
            }
        }
        Ok(())
    }

    /// Fold one candle: every indicator ticks and records its primary value and
    /// named fields into `cur`.
    pub(crate) fn update(&mut self, candle: &Candle) {
        let bar = BarInput {
            candle,
            reference: None,
            deriv: None,
            orderbook: None,
            trades: &[],
            cross_section: None,
        };
        for entry in &mut self.items {
            if let Some(value) = entry.indicator.update(&bar) {
                self.cur.insert(entry.key.clone(), value);
                for (field, field_value) in entry.indicator.fields() {
                    self.cur
                        .insert(format!("{}.{field}", entry.key), field_value);
                }
            }
        }
    }

    /// The current value for a canonical feature key, if computed this bar.
    pub(crate) fn cur(&self, key: &str) -> Option<f64> {
        self.cur.get(key).copied()
    }

    /// The largest warmup period across all registered indicators.
    pub(crate) fn max_warmup(&self) -> usize {
        self.items
            .iter()
            .map(|e| e.indicator.warmup())
            .max()
            .unwrap_or(0)
    }
}

/// Canonical base key for an indicator, without any field suffix:
/// `<name>(<p,p,...>)`. Matches [`Feature::key`] for a field-less indicator.
fn base_key(name: &str, params: &[f64]) -> String {
    Feature::Indicator {
        name: name.to_string(),
        params: params.to_vec(),
        field: None,
    }
    .key()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn resolves_and_folds_an_sma() {
        let mut set = IndicatorSet::new();
        set.required(&Feature::Indicator {
            name: "Sma".into(),
            params: vec![3.0],
            field: None,
        })
        .unwrap();
        assert!(set.max_warmup() > 0);

        for c in [1.0, 2.0, 3.0, 4.0, 5.0] {
            set.update(&candle(c));
        }
        assert_eq!(set.cur("Sma(3)"), Some(4.0));
    }

    #[test]
    fn unknown_indicator_errors() {
        let mut set = IndicatorSet::new();
        assert!(matches!(
            set.required(&Feature::Indicator {
                name: "NotAnIndicator".into(),
                params: vec![],
                field: None,
            }),
            Err(Error::UnknownIndicator(_))
        ));
    }
}
