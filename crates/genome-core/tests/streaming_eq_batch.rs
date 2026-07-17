//! Streaming equals batch: feeding a universe candle-by-candle through `feed`
//! yields a genome whose every query response is byte-identical to one built in a
//! single `build` call over the same data. The cross-section reductions read only
//! the final per-symbol state, so the two ingestion paths must agree exactly.

use std::collections::BTreeMap;

use genome_core::{build, Candle, Genome, GenomeSpec};

fn spec() -> GenomeSpec {
    serde_json::from_str(
        r#"{"features":[{"kind":"price","field":"close"},
            {"kind":"indicator","name":"Rsi","params":[14]},
            {"kind":"indicator","name":"Roc","params":[10]}],
            "symbols":["AAA","BBB","CCC","DDD"],
            "normalize":"z_score","metric":"euclid","seed":99}"#,
    )
    .unwrap()
}

/// A deterministic four-symbol universe, 30 bars each, distinct paths.
fn universe() -> BTreeMap<String, Vec<Candle>> {
    let mut data = BTreeMap::new();
    for (idx, sym) in ["AAA", "BBB", "CCC", "DDD"].iter().enumerate() {
        let base = 100.0 + idx as f64 * 5.0;
        let slope = 0.1 + idx as f64 * 0.05;
        let mut candles = Vec::new();
        let mut prev = base;
        for i in 0..30 {
            let osc = (f64::from(i) * 0.4).sin();
            let close = base * (1.0 + slope * f64::from(i) / 100.0) + osc;
            candles.push(Candle {
                time: i64::from(i) * 3600,
                open: prev,
                high: prev.max(close) + 0.5,
                low: prev.min(close) - 0.5,
                close,
                volume: 1000.0 + f64::from(i) * 10.0,
            });
            prev = close;
        }
        data.insert((*sym).to_string(), candles);
    }
    data
}

fn assert_same(op: &str, cmd: &str, batch: &mut Genome, streamed: &mut Genome) {
    assert_eq!(
        batch.command_json(cmd),
        streamed.command_json(cmd),
        "streaming and batch disagree on {op}"
    );
}

#[test]
fn feed_matches_build_for_every_op() {
    let spec = spec();
    let data = universe();

    // Batch path: one build over the whole universe.
    let mut batch = build(&data, &spec).unwrap();

    // Streaming path: an empty genome fed candle-by-candle, per symbol.
    let mut streamed = Genome::with_spec(spec);
    for (sym, candles) in &data {
        for candle in candles {
            streamed.feed(sym, candle).unwrap();
        }
    }

    assert_same(
        "vector",
        r#"{"cmd":"vector","symbol":"AAA"}"#,
        &mut batch,
        &mut streamed,
    );
    assert_same(
        "similar",
        r#"{"cmd":"similar","symbol":"AAA","k":3}"#,
        &mut batch,
        &mut streamed,
    );
    assert_same(
        "cluster",
        r#"{"cmd":"cluster","k":2}"#,
        &mut batch,
        &mut streamed,
    );
    assert_same("anomaly", r#"{"cmd":"anomaly"}"#, &mut batch, &mut streamed);
}
