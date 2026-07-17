<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-genome)
[![CI](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![CodeQL](https://github.com/wickra-lib/wickra-genome/actions/workflows/codeql.yml/badge.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/codeql.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-genome)
[![Deterministic across 10 languages](https://img.shields.io/badge/deterministic%20across-10%20languages-3b82f6)](#use-in-any-language)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

---

# Wickra Genome

**A vector database of the whole market: every asset as a live feature vector
over the streaming indicators. Similarity search, clustering and anomaly
detection over microstructure DNA.**

Every asset has a shape — the momentum, volatility, flow and microstructure
signature of how it is trading *right now*. Wickra Genome turns that shape into a
feature vector, one coordinate per `wickra-core` streaming indicator, and lets you
query the whole market by it: **find every asset behaving like X right now**,
cluster the market into regimes, or flag the assets whose DNA has gone anomalous.

Genome is one data-driven core, `genome-core`: it builds each asset's vector from
the same `wickra-core` indicators the rest of the ecosystem uses, then runs
similarity search, seeded k-means clustering and anomaly scoring over the
cross-section. The core is exposed as a **JSON-over-C-ABI data API**
(`command_json`) in **Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R**,
plus a reference CLI.

> **Status:** 0.1.0, unreleased. The vector engine, the reference CLI, all ten
> language bindings, the golden corpus, the test/fuzz/bench surface and CI are in
> place and green; the first tagged release publishes to the registries.

## Documentation

- [ARCHITECTURE](docs/ARCHITECTURE.md) — the engine, the `command_json` boundary and the determinism contract.
- [FEATURES](docs/FEATURES.md) — the feature axes and the `GenomeSpec` JSON.
- [NORMALIZATION](docs/NORMALIZATION.md) — the cross-section z-score and min–max formulas.
- [METRICS](docs/METRICS.md) — the cosine and euclidean distance metrics.
- [CLUSTERING](docs/CLUSTERING.md) — seeded k-means++ and the portable PRNG.
- [STREAMING](docs/STREAMING.md) — streaming (`feed`) equals batch (`build`).
- [Cookbook](docs/Cookbook.md) — worked recipes.

Full docs at [wickra.org](https://wickra.org).

## How it works

1. A [`GenomeSpec`](docs/FEATURES.md) fixes the feature axes (indicator outputs
   and price fields), the cross-section normalization and the distance metric.
2. Each symbol's candles fold, in O(1) per bar, into a raw vector — one value per
   feature — through the `wickra-core` streaming indicators.
3. The cross-section of raw vectors is [normalized](docs/NORMALIZATION.md), then
   four queries run over it: `vector` (one symbol's axes), `similar` (k nearest
   neighbours), `cluster` ([seeded k-means](docs/CLUSTERING.md)) and `anomaly`
   (nearest-neighbour outlier scores).

Determinism is enforced end to end — `BTreeMap` symbol ordering, serial
key-order reductions, a portable `SplitMix64` PRNG for k-means and fixed `1e-8`
output rounding — so the batch and [streaming](docs/STREAMING.md) paths and every
language binding return byte-identical answers.

## Quickstart

The CLI over a spec and a directory of per-symbol CSVs:

```bash
cargo run -p genome-cli -- \
  --spec examples/data/specs/dna.json \
  --data examples/data/universe \
  --op similar --symbol AAA --k 3
```

From Rust:

```rust
use genome_core::Genome;

let mut g = Genome::new(
    r#"{"features":[{"kind":"price","field":"close"}],
        "symbols":["AAA","BBB","CCC"],"metric":"euclid"}"#,
)?;
g.command_json(r#"{"cmd":"build","data":{
    "AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],
    "BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],
    "CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}}"#);
println!("{}", g.command_json(r#"{"cmd":"similar","symbol":"AAA","k":2}"#));
// {"neighbors":[{"symbol":"BBB",...},{"symbol":"CCC",...}]}
```

## GenomeSpec and the queries

A spec is a serde value, not code — the same JSON drives every language:

```json
{
  "features": [
    { "kind": "indicator", "name": "Rsi", "params": [14] },
    { "kind": "indicator", "name": "Roc", "params": [10] },
    { "kind": "price", "field": "close" }
  ],
  "symbols": ["AAA", "BBB", "CCC", "DDD"],
  "normalize": "z_score",
  "metric": "euclid",
  "seed": 24333
}
```

- **[Features](docs/FEATURES.md)** — each axis is a streaming-indicator output
  (`indicator`) or a raw OHLCV field (`price`), in the order they appear.
- **[Normalization](docs/NORMALIZATION.md)** — `z_score` or `min_max`, applied
  across the cross-section before any distance is taken.
- **[Metric](docs/METRICS.md)** — `cosine` or `euclid`.
- **Queries** — `vector`, `similar` (k nearest), `cluster`
  ([seeded k-means](docs/CLUSTERING.md), `k` clusters) and `anomaly`.

## Use in any language

The same handle + `command_json` + `version` surface ships for Rust, Python,
Node.js, WASM, and — over a C ABI hub — C, C++, C#, Go, Java and R. Each binding
forwards the command string verbatim, so the answer they return is identical.

```python
from wickra_genome import Genome
import json

g = Genome(json.dumps({
    "features": [{"kind": "price", "field": "close"}],
    "symbols": ["AAA", "BBB", "CCC"], "metric": "euclid",
}))
g.command(json.dumps({"cmd": "build", "data": data}))
print(g.command(json.dumps({"cmd": "similar", "symbol": "AAA", "k": 2})))
```

Runnable examples for all ten languages live in [`examples/`](examples/).

## Project layout

| Path                    | What                                                        |
|-------------------------|-------------------------------------------------------------|
| `crates/genome-core`    | The vector engine and the `command_json` boundary.          |
| `crates/genome-cli`     | The reference CLI (`wickra-genome`).                        |
| `crates/genome-bench`   | Criterion benchmarks.                                        |
| `bindings/`             | The ten language bindings (`python`, `node`, `wasm`, `c` + go/csharp/java/r). |
| `golden/`               | The generate-once / replay-everywhere byte-golden corpus.   |
| `examples/`             | A runnable example per language.                            |

## Building from source

```bash
cargo build
cargo test
```

## Requirements

- Rust 1.86+ (MSRV; the Node binding needs 1.88). Genome depends on `wickra-core`
  and `wickra-data` (crates.io) for the indicators and the candle reader, resolves
  indicators through `wickra-backtest-core`, and — behind the `live` feature —
  uses `wickra-exchange` (a git dependency) for a live market feed.

## Benchmarks

See [BENCHMARKS.md](BENCHMARKS.md). Reproduce with `cargo bench -p genome-bench`
(and `--no-default-features` for the single-threaded path).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Genome reads
recorded market data and specs only — no keys, no order placement.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Disclaimer

Wickra Genome is research and analytics software. Its similarity, clustering and
anomaly outputs are not investment advice, and nothing here is a recommendation
to trade. Use at your own risk.
