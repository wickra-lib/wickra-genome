//! Property tests for the vector engine: over random universes the engine never
//! panics, a vector's dimension equals the feature count, `similar` returns at
//! most `k` neighbours (never the query itself) in non-decreasing distance,
//! `anomaly` scores are non-increasing, and seeded k-means is deterministic with
//! its members partitioning exactly the ready universe.

use std::collections::BTreeMap;

use genome_core::{build, Candle, GenomeSpec};
use proptest::prelude::*;

/// A spec over price-close and RSI(14), for `n` symbols with a fixed seed.
fn spec_for(symbols: &[String], seed: u64) -> GenomeSpec {
    let syms = serde_json::to_string(symbols).unwrap();
    serde_json::from_str(&format!(
        r#"{{"features":[{{"kind":"price","field":"close"}},
            {{"kind":"indicator","name":"Rsi","params":[14]}}],
            "symbols":{syms},"normalize":"z_score","metric":"euclid","seed":{seed}}}"#
    ))
    .unwrap()
}

/// A random universe: `n` symbols (`S0..Sn`), each a `bars`-long positive walk
/// driven by the per-symbol/per-bar step list.
fn universe(steps: &[Vec<f64>]) -> (Vec<String>, BTreeMap<String, Vec<Candle>>) {
    let mut symbols = Vec::new();
    let mut data = BTreeMap::new();
    for (idx, walk) in steps.iter().enumerate() {
        let sym = format!("S{idx:02}");
        symbols.push(sym.clone());
        let mut price = 100.0 + idx as f64;
        let mut candles = Vec::new();
        for (i, step) in walk.iter().enumerate() {
            let next = (price + step).max(1.0);
            candles.push(Candle {
                time: i64::try_from(i).unwrap() * 3600,
                open: price,
                high: price.max(next) + 0.5,
                low: price.min(next) - 0.5,
                close: next,
                volume: 1000.0,
            });
            price = next;
        }
        data.insert(sym, candles);
    }
    (symbols, data)
}

fn ready_count(genome: &mut genome_core::Genome, symbols: &[String]) -> usize {
    symbols
        .iter()
        .filter(|s| genome.vector(s).is_ok_and(|v| v.ready))
        .count()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]

    #[test]
    fn engine_holds_its_invariants(
        walks in prop::collection::vec(
            prop::collection::vec(-3.0f64..3.0, 16..24),
            2..6,
        ),
        seed in 0u64..10_000,
        k in 1usize..5,
    ) {
        let (symbols, data) = universe(&walks);
        let spec = spec_for(&symbols, seed);

        let mut genome = build(&data, &spec).expect("build must not fail on finite data");

        // vector.dim == feature count, for every symbol.
        for sym in &symbols {
            let v = genome.vector(sym).expect("vector");
            prop_assert_eq!(v.dim, 2);
            prop_assert_eq!(v.keys.len(), 2);
            prop_assert_eq!(v.values.len(), 2);
        }

        let ready = ready_count(&mut genome, &symbols);

        // similar: <= k, never self, distances non-decreasing.
        let query = &symbols[0];
        let neighbours = genome.similar(query, k).expect("similar");
        prop_assert!(neighbours.len() <= k, "at most k neighbours");
        for w in neighbours.windows(2) {
            prop_assert!(w[1].distance >= w[0].distance - 1e-9, "distances non-decreasing");
        }
        for n in &neighbours {
            prop_assert_ne!(&n.symbol, query, "a symbol is never its own neighbour");
        }

        // anomaly: scores non-increasing (biggest outlier first).
        let anomalies = genome.anomaly().expect("anomaly");
        prop_assert_eq!(anomalies.len(), ready, "one anomaly score per ready symbol");
        for w in anomalies.windows(2) {
            prop_assert!(w[0].score >= w[1].score - 1e-9, "anomaly scores non-increasing");
        }

        // k-means: deterministic for a fixed seed; members partition the ready set.
        let clusters_a = genome.cluster(k);
        let clusters_b = genome.cluster(k);
        prop_assert_eq!(&clusters_a, &clusters_b, "same seed -> identical clusters");
        let members: usize = clusters_a.iter().map(|c| c.members.len()).sum();
        prop_assert_eq!(members, ready, "cluster members partition the ready universe");
    }
}
