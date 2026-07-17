//! The three read queries over the normalized vector space: `vector` (a single
//! symbol's axes), `similar` (k nearest neighbors) and `anomaly` (each symbol's
//! nearest-neighbor distance). All distances are rounded to `1e-8`; ties break
//! by symbol key so the ordering is fully deterministic.

use crate::error::{Error, Result};
use crate::metric::distance;
use crate::normalize::normalize;
use crate::spec::GenomeSpec;
use crate::types::{Anomaly, Neighbor, Vector};
use crate::universe::Universe;

/// The fixed output precision: every `f64` in a response is snapped to this grid.
const PRECISION: f64 = 1e-8;

/// Round `x` to the fixed output grid (`1e-8`), so the CLI and every binding emit
/// byte-identical JSON. `-0.0` is normalized to `0.0`.
#[must_use]
pub(crate) fn round_to(x: f64) -> f64 {
    let r = (x / PRECISION).round() * PRECISION;
    if r == 0.0 {
        0.0
    } else {
        r
    }
}

/// The self-describing feature vector for one symbol, in spec axis order.
///
/// A ready symbol reports its raw (un-normalized) axis values; a symbol that is
/// unknown or not ready reports `ready: false` with every axis `None`. An
/// unknown symbol (not in the universe at all) is an [`Error::UnknownSymbol`].
pub(crate) fn vector(u: &Universe, spec: &GenomeSpec, symbol: &str) -> Result<Vector> {
    let keys: Vec<String> = spec
        .features
        .iter()
        .map(super::feature::Feature::key)
        .collect();
    let dim = keys.len();
    let state = u
        .symbols
        .get(symbol)
        .ok_or_else(|| Error::UnknownSymbol(symbol.to_string()))?;
    match state.raw_vector(spec) {
        Some(raw) => Ok(Vector {
            symbol: symbol.to_string(),
            dim,
            values: raw.into_iter().map(|x| Some(round_to(x))).collect(),
            keys,
            ready: true,
        }),
        None => Ok(Vector {
            symbol: symbol.to_string(),
            dim,
            values: vec![None; dim],
            keys,
            ready: false,
        }),
    }
}

/// The `k` nearest neighbors of `symbol` in the normalized space, ordered by
/// `(distance asc, symbol key asc)`. `symbol` itself is excluded; `k` is capped
/// at the number of other ready symbols. Errors if `symbol` is not a ready
/// member of the universe.
pub(crate) fn similar(
    u: &Universe,
    spec: &GenomeSpec,
    symbol: &str,
    k: usize,
) -> Result<Vec<Neighbor>> {
    let rows = normalize(&u.ready(spec), spec.normalize);
    let query = rows
        .iter()
        .find(|(s, _)| s == symbol)
        .ok_or_else(|| Error::UnknownSymbol(symbol.to_string()))?
        .1
        .clone();

    let mut pairs: Vec<Neighbor> = rows
        .iter()
        .filter(|(s, _)| s != symbol)
        .map(|(s, v)| Neighbor {
            symbol: s.clone(),
            distance: round_to(distance(&query, v, spec.metric)),
        })
        .collect();
    pairs.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.symbol.cmp(&b.symbol))
    });
    pairs.truncate(k);
    Ok(pairs)
}

/// Each ready symbol's anomaly score: the distance to its nearest neighbor,
/// ordered by `(score desc, symbol key asc)` — the biggest outlier first. With a
/// single ready symbol every score is `0`.
///
/// Returns [`Result`] for symmetry with the other queries even though it cannot
/// fail today.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn anomaly(u: &Universe, spec: &GenomeSpec) -> Result<Vec<Anomaly>> {
    let rows = normalize(&u.ready(spec), spec.normalize);
    let mut out: Vec<Anomaly> = Vec::with_capacity(rows.len());
    for (i, (sym, v)) in rows.iter().enumerate() {
        let mut nearest = f64::INFINITY;
        for (j, (_, w)) in rows.iter().enumerate() {
            if i != j {
                let d = distance(v, w, spec.metric);
                if d < nearest {
                    nearest = d;
                }
            }
        }
        let score = if nearest.is_finite() { nearest } else { 0.0 };
        out.push(Anomaly {
            symbol: sym.clone(),
            score: round_to(score),
        });
    }
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.symbol.cmp(&b.symbol))
    });
    Ok(out)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;
    use crate::feature::{Feature, PriceField};
    use crate::spec::{Metric, Normalize};
    use wickra_backtest_core::Candle;

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
            symbols: vec!["AAA".into(), "BBB".into(), "CCC".into()],
            normalize: Normalize::ZScore,
            metric: Metric::Euclid,
            seed: 0,
            timeframe: None,
        }
    }

    fn universe(closes: &[(&str, f64)]) -> Universe {
        let spec = spec();
        let mut u = Universe::new();
        for (sym, c) in closes {
            u.fold(sym, &candle(*c), &spec).unwrap();
        }
        u
    }

    #[test]
    fn round_to_snaps_and_kills_negative_zero() {
        // Snaps to the 1e-8 grid; the tiny extra digits are dropped.
        assert!((round_to(1.234_567_894_9) - 1.234_567_89).abs() < 1e-12);
        assert!((round_to(2.000_000_004) - 2.0).abs() < 1e-12);
        assert_eq!(round_to(-0.0), 0.0);
        assert!(round_to(-0.0).is_sign_positive());
    }

    #[test]
    fn vector_unknown_symbol_errors() {
        let u = universe(&[("AAA", 1.0)]);
        assert!(matches!(
            vector(&u, &spec(), "ZZZ"),
            Err(Error::UnknownSymbol(_))
        ));
    }

    #[test]
    fn similar_excludes_self_and_orders_by_distance_then_key() {
        let u = universe(&[("AAA", 1.0), ("BBB", 2.0), ("CCC", 100.0)]);
        let n = similar(&u, &spec(), "AAA", 5).unwrap();
        assert_eq!(n.len(), 2); // capped at other-ready count
        assert_eq!(n[0].symbol, "BBB"); // closest
        assert_eq!(n[1].symbol, "CCC");
    }

    #[test]
    fn anomaly_biggest_outlier_first() {
        let u = universe(&[("AAA", 1.0), ("BBB", 2.0), ("CCC", 100.0)]);
        let a = anomaly(&u, &spec()).unwrap();
        assert_eq!(a[0].symbol, "CCC");
    }
}
