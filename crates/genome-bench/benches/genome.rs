#![allow(clippy::cast_precision_loss)]
//! Criterion benchmarks for the Wickra Genome vector engine.
//!
//! The default build measures the parallel-capable `build`; `--no-default-features`
//! measures the single-threaded path (what WASM uses). Each case constructs a
//! universe of `N` symbols over a fixed 64-bar path with `F` feature axes, then
//! times `build`, `similar` and `cluster`. Build throughput is reported in
//! symbols/second.

use std::collections::BTreeMap;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use genome_core::{build, Candle, GenomeSpec};

const BARS: usize = 64;

/// A deterministic `bars`-long OHLCV path for symbol index `idx`.
fn candles(idx: usize) -> Vec<Candle> {
    let base = 100.0 + (idx % 50) as f64;
    let mut out = Vec::with_capacity(BARS);
    let mut prev = base;
    for i in 0..BARS {
        let close = base + ((i as f64 * 0.3) + idx as f64 * 0.1).sin() * 3.0;
        out.push(Candle {
            time: i64::try_from(i).unwrap() * 3600,
            open: prev,
            high: prev.max(close) + 0.5,
            low: prev.min(close) - 0.5,
            close,
            volume: 1000.0 + i as f64,
        });
        prev = close;
    }
    out
}

/// A universe of `n` symbols (`S000000..`).
fn universe(n: usize) -> BTreeMap<String, Vec<Candle>> {
    (0..n).map(|i| (format!("S{i:06}"), candles(i))).collect()
}

/// A spec with `n_features` axes, cycling four indicator families with varying
/// periods so every axis is distinct, over `n` symbols.
fn spec(n: usize, n_features: usize) -> GenomeSpec {
    let mut feats = vec![r#"{"kind":"price","field":"close"}"#.to_string()];
    let kinds = ["Rsi", "Roc", "Sma", "Ema"];
    for i in 1..n_features {
        let kind = kinds[i % kinds.len()];
        let period = 5 + (i % 20);
        feats.push(format!(
            r#"{{"kind":"indicator","name":"{kind}","params":[{period}]}}"#
        ));
    }
    let symbols: Vec<String> = (0..n).map(|i| format!("S{i:06}")).collect();
    let json = format!(
        r#"{{"features":[{}],"symbols":{},"normalize":"z_score","metric":"euclid","seed":42}}"#,
        feats.join(","),
        serde_json::to_string(&symbols).unwrap()
    );
    serde_json::from_str(&json).unwrap()
}

fn bench_genome(c: &mut Criterion) {
    let sizes = [100usize, 1_000];
    let feature_counts = [5usize, 50];

    let mut build_group = c.benchmark_group("genome_build");
    for &n in &sizes {
        let data = universe(n);
        build_group.throughput(Throughput::Elements(n as u64));
        for &f in &feature_counts {
            let s = spec(n, f);
            build_group.bench_with_input(BenchmarkId::new(format!("f{f}"), n), &n, |b, _| {
                b.iter(|| build(&data, &s).unwrap());
            });
        }
    }
    build_group.finish();

    let mut query_group = c.benchmark_group("genome_query");
    for &n in &sizes {
        let data = universe(n);
        for &f in &feature_counts {
            let s = spec(n, f);
            let genome = build(&data, &s).unwrap();
            query_group.bench_with_input(
                BenchmarkId::new(format!("similar_f{f}"), n),
                &n,
                |b, _| b.iter(|| genome.similar("S000000", 10).unwrap()),
            );
            query_group.bench_with_input(
                BenchmarkId::new(format!("cluster_f{f}"), n),
                &n,
                |b, _| b.iter(|| genome.cluster(8)),
            );
        }
    }
    query_group.finish();
}

criterion_group!(benches, bench_genome);
criterion_main!(benches);
