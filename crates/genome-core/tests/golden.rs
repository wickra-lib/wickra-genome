//! The golden invariant, from Rust: for every `golden/specs/<name>.json` run
//! against the shared `golden/data/` universe, each query's `command_json`
//! response is byte-for-byte equal to `golden/expected/<op>/<name>.json`. This is
//! exactly what the CLI's `--format json` and every language binding produce, so
//! this file is the reference the whole cross-language corpus is pinned to.
//!
//! A missing expected file is written (bless mode) so the corpus can be
//! regenerated after an intended change; a present file is asserted byte-for-byte.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use genome_core::{build, Candle, GenomeSpec};

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

/// Parse an OHLCV CSV (`ts,open,high,low,close,volume`, header skipped) — the
/// same layout the CLI's `--data` loader consumes.
fn parse_csv(content: &str) -> Vec<Candle> {
    let mut candles = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(str::trim).collect();
        let time = match cols[0].parse::<i64>() {
            Ok(t) => t,
            Err(_) if idx == 0 => continue,
            Err(e) => panic!("bad timestamp on line {}: {e}", idx + 1),
        };
        candles.push(Candle {
            time,
            open: cols[1].parse().unwrap(),
            high: cols[2].parse().unwrap(),
            low: cols[3].parse().unwrap(),
            close: cols[4].parse().unwrap(),
            volume: cols[5].parse().unwrap(),
        });
    }
    candles
}

/// Load the shared `golden/data/` universe (one `<SYMBOL>.csv` per symbol).
fn load_universe() -> BTreeMap<String, Vec<Candle>> {
    let dir = golden_dir().join("data");
    let mut data = BTreeMap::new();
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("csv") {
            continue;
        }
        let symbol = path.file_stem().unwrap().to_str().unwrap().to_owned();
        data.insert(symbol, parse_csv(&fs::read_to_string(&path).unwrap()));
    }
    data
}

/// The blessed op matrix per spec — `cluster5` is blessed at `k=5`, the rest at
/// `k=3` (mirrors `golden/README.md` and the bless command).
fn ops_for(stem: &str) -> Vec<(&'static str, String)> {
    let ck = if stem == "cluster5" { 5 } else { 3 };
    vec![
        ("vector", r#"{"cmd":"vector","symbol":"AAA"}"#.to_string()),
        (
            "similar",
            r#"{"cmd":"similar","symbol":"AAA","k":3}"#.to_string(),
        ),
        ("cluster", format!(r#"{{"cmd":"cluster","k":{ck}}}"#)),
        ("anomaly", r#"{"cmd":"anomaly"}"#.to_string()),
    ]
}

#[test]
fn every_spec_and_op_matches_its_expected_output() {
    let dir = golden_dir();
    let data = load_universe();

    let mut specs: Vec<_> = fs::read_dir(dir.join("specs"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    specs.sort();

    let mut checked = 0;
    for spec_path in specs {
        let stem = spec_path.file_stem().unwrap().to_str().unwrap().to_owned();
        let spec: GenomeSpec = serde_json::from_str(&fs::read_to_string(&spec_path).unwrap())
            .unwrap_or_else(|e| panic!("parse spec {stem}: {e}"));

        for (op, cmd) in ops_for(&stem) {
            let mut genome = build(&data, &spec).unwrap_or_else(|e| panic!("build {stem}: {e}"));
            let got = genome.command_json(&cmd);

            let expected_path = dir.join("expected").join(op).join(format!("{stem}.json"));
            if expected_path.exists() {
                let expected = fs::read_to_string(&expected_path).unwrap();
                assert_eq!(
                    got,
                    expected.trim_end(),
                    "golden mismatch for {op}/{stem}: the core output no longer matches the \
                     committed fixture (re-bless if the change is intended)"
                );
            } else {
                fs::create_dir_all(expected_path.parent().unwrap()).unwrap();
                fs::write(&expected_path, format!("{got}\n")).unwrap();
            }
            checked += 1;
        }
    }
    assert_eq!(
        checked, 20,
        "expected 5 specs x 4 ops = 20, checked {checked}"
    );
}
