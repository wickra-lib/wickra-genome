#![no_main]
//! Fuzz the seeded k-means surface: a fixed universe is clustered with a
//! fuzz-derived `k` and `seed`. No `k` may panic (including `k = 0` and `k`
//! larger than the universe), and a fixed seed is deterministic across two calls.

use std::collections::BTreeMap;

use genome_core::{build, Candle, GenomeSpec};
use libfuzzer_sys::fuzz_target;

fn universe() -> BTreeMap<String, Vec<Candle>> {
    let mut data = BTreeMap::new();
    for (idx, sym) in ["AAA", "BBB", "CCC", "DDD", "EEE"].iter().enumerate() {
        let base = 100.0 + idx as f64 * 4.0;
        let mut candles = Vec::new();
        let mut prev = base;
        for i in 0..24 {
            let close = base + (i as f64 * 0.3 + idx as f64).sin() * 2.0;
            candles.push(Candle {
                time: i64::from(i) * 3600,
                open: prev,
                high: prev.max(close) + 0.5,
                low: prev.min(close) - 0.5,
                close,
                volume: 1000.0,
            });
            prev = close;
        }
        data.insert((*sym).to_string(), candles);
    }
    data
}

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    let k = usize::from(data[0]); // 0..=255, deliberately spanning past the universe
    let seed = u64::from(data.get(1).copied().unwrap_or(0));

    let spec: GenomeSpec = serde_json::from_str(&format!(
        r#"{{"features":[{{"kind":"price","field":"close"}},
            {{"kind":"indicator","name":"Roc","params":[5]}}],
            "symbols":["AAA","BBB","CCC","DDD","EEE"],
            "normalize":"z_score","metric":"euclid","seed":{seed}}}"#
    ))
    .unwrap();

    let genome = build(&universe(), &spec).expect("build a fixed finite universe");
    let a = genome.cluster(k);
    let b = genome.cluster(k);
    assert_eq!(a, b, "seeded k-means must be deterministic for a fixed seed");
});
