#![no_main]
//! Fuzz the build + query pipeline: arbitrary bytes drive a three-symbol
//! universe of close prices under a fixed spec; `build` then every query runs
//! over it. No input may panic — indicators and the cross-section reductions must
//! stay finite over any price path.

use std::collections::BTreeMap;

use genome_core::{build, Candle, GenomeSpec};
use libfuzzer_sys::fuzz_target;

const SPEC: &str = r#"{"features":[{"kind":"price","field":"close"},
    {"kind":"indicator","name":"Rsi","params":[14]}],
    "symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":7}"#;

fn candle(time: i64, prev: f64, close: f64) -> Candle {
    Candle {
        time,
        open: prev,
        high: prev.max(close) + 0.5,
        low: prev.min(close) - 0.5,
        close,
        volume: 1000.0,
    }
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }
    let spec: GenomeSpec = serde_json::from_str(SPEC).unwrap();

    // Round-robin the bytes across the three symbols as bounded close prices.
    let mut universe: BTreeMap<String, Vec<Candle>> = BTreeMap::new();
    let symbols = ["AAA", "BBB", "CCC"];
    let mut prev = [100.0f64; 3];
    let mut time = [0i64; 3];
    for (i, &b) in data.iter().enumerate() {
        let s = i % 3;
        let close = 50.0 + f64::from(b); // 50..=305, always positive
        let c = candle(time[s], prev[s], close);
        universe
            .entry(symbols[s].to_string())
            .or_default()
            .push(c);
        prev[s] = close;
        time[s] += 3600;
    }

    let Ok(mut genome) = build(&universe, &spec) else {
        return;
    };
    let _ = genome.command_json(r#"{"cmd":"vector","symbol":"AAA"}"#);
    let _ = genome.command_json(r#"{"cmd":"similar","symbol":"AAA","k":2}"#);
    let _ = genome.command_json(r#"{"cmd":"anomaly"}"#);
});
