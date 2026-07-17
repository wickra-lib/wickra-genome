//! Per-symbol state: the indicators a spec needs, folded one candle at a time,
//! plus the current candle for price-field features. Produces the symbol's raw
//! feature vector at the latest bar.

use crate::feature::{Feature, PriceField};
use crate::indicator_set::IndicatorSet;
use crate::spec::GenomeSpec;
use wickra_backtest_core::Candle;

/// The rolling state of one symbol: its indicator set, a bar counter, a readiness
/// flag (past the largest warmup) and the current candle.
pub(crate) struct SymbolState {
    inds: IndicatorSet,
    warmup: usize,
    bars: usize,
    cur: Option<Candle>,
}

impl SymbolState {
    /// Build the state for a spec: register every indicator referenced by a
    /// feature. Errors if the registry does not know an indicator.
    pub(crate) fn new(spec: &GenomeSpec) -> crate::error::Result<Self> {
        let mut inds = IndicatorSet::new();
        for feature in &spec.features {
            inds.required(feature)?;
        }
        let warmup = inds.max_warmup();
        Ok(Self {
            inds,
            warmup,
            bars: 0,
            cur: None,
        })
    }

    /// Fold one candle in O(1): tick every indicator and shift the candle window.
    pub(crate) fn fold(&mut self, candle: &Candle) {
        self.inds.update(candle);
        self.cur = Some(*candle);
        self.bars += 1;
    }

    /// Whether the symbol has folded at least `warmup` bars (and at least one).
    pub(crate) fn is_ready(&self) -> bool {
        self.bars >= self.warmup && self.bars > 0
    }

    /// The current value of a single feature: a price field from the current
    /// candle, or an indicator output by its canonical key.
    pub(crate) fn feature_cur(&self, feature: &Feature) -> Option<f64> {
        match feature {
            Feature::Price { field } => self.cur.as_ref().map(|c| price_field(c, *field)),
            Feature::Indicator { .. } => self.inds.cur(&feature.key()),
        }
    }

    /// The raw feature vector at the latest bar, in `features` order. Returns
    /// `None` (the symbol is not ready) if it is inside warmup or any axis is
    /// missing, `NaN` or infinite — a non-finite axis never reaches an output.
    pub(crate) fn raw_vector(&self, spec: &GenomeSpec) -> Option<Vec<f64>> {
        if !self.is_ready() {
            return None;
        }
        let mut out = Vec::with_capacity(spec.features.len());
        for feature in &spec.features {
            let value = self.feature_cur(feature)?;
            if !value.is_finite() {
                return None;
            }
            out.push(value);
        }
        Some(out)
    }
}

/// Read a price field from a candle.
fn price_field(candle: &Candle, field: PriceField) -> f64 {
    match field {
        PriceField::Open => candle.open,
        PriceField::High => candle.high,
        PriceField::Low => candle.low,
        PriceField::Close => candle.close,
        PriceField::Volume => candle.volume,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{Metric, Normalize};

    fn candle(close: f64) -> Candle {
        Candle {
            time: 0,
            open: close,
            high: close + 1.0,
            low: close - 1.0,
            close,
            volume: 100.0,
        }
    }

    fn spec() -> GenomeSpec {
        GenomeSpec {
            features: vec![
                Feature::Indicator {
                    name: "Sma".into(),
                    params: vec![3.0],
                    field: None,
                },
                Feature::Price {
                    field: PriceField::Close,
                },
            ],
            symbols: vec!["A".into()],
            normalize: Normalize::ZScore,
            metric: Metric::Cosine,
            seed: 0,
            timeframe: None,
        }
    }

    #[test]
    fn not_ready_inside_warmup() {
        let spec = spec();
        let mut state = SymbolState::new(&spec).unwrap();
        state.fold(&candle(1.0));
        assert!(!state.is_ready());
        assert!(state.raw_vector(&spec).is_none());
    }

    #[test]
    fn raw_vector_in_feature_order() {
        let spec = spec();
        let mut state = SymbolState::new(&spec).unwrap();
        for c in [1.0, 2.0, 3.0] {
            state.fold(&candle(c));
        }
        assert!(state.is_ready());
        let v = state.raw_vector(&spec).unwrap();
        assert_eq!(v, vec![2.0, 3.0]); // Sma(3) = (1+2+3)/3, close = 3
    }
}
