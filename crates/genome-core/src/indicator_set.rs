//! Resolves the indicators a spec references and folds candles through them.
//!
//! Indicators are resolved by name and parameters from the `wickra-core`
//! registry — reused through the `wickra-backtest-core` factory, the only
//! name -> indicator resolver in the ecosystem. Each resolved indicator is an
//! object-safe `EvalIndicator`, driven with a [`BarInput`] carrying the candle
//! and whatever side feeds the caller supplied. Unlike the screener, the genome
//! keeps no previous-bar values: a vector is a snapshot of the latest bar, with
//! no crossover semantics.
//!
//! [`feed_kind`] answers which feed a name consumes, which is what lets
//! [`crate::spec::GenomeSpec::check_feeds`] refuse a spec whose axis could only
//! ever be `None` -- and one such axis takes the whole symbol out of the ready
//! universe, so the refusal matters more here than a dead column would.

use crate::error::{Error, Result};
use crate::feature::Feature;
use crate::feeds::{BarFeeds, FeedKind};
use std::collections::BTreeMap;
use wickra_backtest_core::registry::{build, feed_of, BarInput};
use wickra_backtest_core::spec::Feed;
use wickra_backtest_core::{Candle, EvalIndicator};

/// The pairwise indicators, which read the reference series' close alongside the
/// bar close.
///
/// This list exists because `registry::feed_of` cannot express the family:
/// upstream classifies every pairwise indicator as `Feed::Kline`, since the
/// candle is indeed one of its two inputs. The `every_pairwise_name_behaves_like_one`
/// test probes every name here against the live registry and fails if the list
/// ever drifts from the set that actually needs a reference, so a registry that
/// grows a new pairwise indicator cannot slip past silently.
const PAIRWISE: [&str; 24] = [
    "Alpha",
    "Beta",
    "BetaNeutralSpread",
    "Cointegration",
    "DistanceSsd",
    "GrangerCausality",
    "HasbrouckInformationShare",
    "InformationRatio",
    "KalmanHedgeRatio",
    "KendallTau",
    "LeadLagCrossCorrelation",
    "OuHalfLife",
    "PairSpreadZScore",
    "PairwiseBeta",
    "PearsonCorrelation",
    "RelativeStrengthAB",
    "RollingCorrelation",
    "RollingCovariance",
    "SpearmanCorrelation",
    "SpreadAr1Coefficient",
    "SpreadBollingerBands",
    "SpreadHurst",
    "TreynorRatio",
    "VarianceRatio",
];

/// Which feed an indicator consumes, or `None` if the registry does not know it.
///
/// Wraps `registry::feed_of` and refines its `Kline` answer with the pairwise
/// list, so a caller can tell "candle is enough" from "needs a reference".
#[must_use]
pub fn feed_kind(name: &str) -> Option<FeedKind> {
    let feed = feed_of(name)?;
    Some(match feed {
        Feed::Kline if PAIRWISE.contains(&name) => FeedKind::Pair,
        Feed::Kline => FeedKind::Candle,
        Feed::Trade => FeedKind::Trades,
        Feed::Orderbook => FeedKind::OrderBook,
        Feed::TradeQuote => FeedKind::TradeQuote,
        Feed::Derivatives => FeedKind::Derivatives,
        Feed::CrossSection => FeedKind::CrossSection,
    })
}

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
    pub(crate) fn update(&mut self, candle: &Candle, feeds: BarFeeds<'_>) {
        let bar = BarInput {
            candle,
            reference: feeds.reference,
            deriv: feeds.deriv,
            orderbook: feeds.orderbook,
            trades: feeds.trades,
            cross_section: feeds.cross_section,
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
            set.update(&candle(c), BarFeeds::default());
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
