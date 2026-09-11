<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-genome)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/ci.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codeql.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-genome)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/release.svg)](https://github.com/wickra-lib/wickra-genome/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/crates.svg)](https://crates.io/crates/wickra-genome)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/pypi.svg)](https://pypi.org/project/wickra-genome/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/npm.svg)](https://www.npmjs.com/package/wickra-genome)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/nuget.svg)](https://www.nuget.org/packages/Wickra.Genome)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-genome)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-genome-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-genome)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/provenance.svg)](https://github.com/wickra-lib/wickra-genome/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/docs.svg)](https://wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/verified.svg)](golden/)

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

Genome is one data-driven core, `wickra-genome-core`: it builds each asset's vector from
the same `wickra-core` indicators the rest of the ecosystem uses, then runs
similarity search, seeded k-means clustering and anomaly scoring over the
cross-section. The core is exposed as a **JSON-over-C-ABI data API**
(`command_json`) in **Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R**,
plus a reference CLI.

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the same
> data-driven core and ten-language binding surface also power
> [wickra-backtest](https://github.com/wickra-lib/wickra-backtest),
> [wickra-proof](https://github.com/wickra-lib/wickra-proof),
> [wickra-verify](https://github.com/wickra-lib/wickra-verify) and 20 more — see
> [the full list](https://github.com/wickra-lib).

> **Status:** 0.1.0, unreleased. The vector engine, the reference CLI, all ten
> language bindings, the golden corpus, the test/fuzz/bench surface and CI are in
> place and green; the first tagged release publishes to the registries.

- **Batch** — `build(data, spec)` folds every symbol's whole history into a vector.
- **Streaming** — `feed(symbol, candle)`, O(1) per tick, over the state so far.
- **Side feeds** — an axis whose indicator reads a reference series, a derivatives tick, an order book, the bar's trades or the market cross-section gets it, or the spec is refused by name.

```rust
use wickra_genome_core::{build, GenomeSpec, SymbolInput};
use std::collections::BTreeMap;

let spec = GenomeSpec::from_json(r#"{
    "symbols":  ["AAA", "BBB"],
    "features": [{"kind": "indicator", "name": "Rsi", "params": [14]},
                 {"kind": "price", "field": "close"}],
    "normalize": "z_score", "metric": "euclid"
}"#)?;

let mut data: BTreeMap<String, SymbolInput> = BTreeMap::new();
data.insert("AAA".into(), candles.into());

let genome = build(&data, &spec)?;
println!("{:?}", genome.similar("AAA", 5)?);
```

## Documentation

- [ARCHITECTURE](docs/ARCHITECTURE.md) — the engine, the `command_json` boundary and the determinism contract.
- [FEATURES](docs/FEATURES.md) — the feature axes and the `GenomeSpec` JSON.
- [FEEDS](docs/FEEDS.md) — the side feeds an indicator reads beyond the candle, and why a spec is refused rather than answered with an axis that can never be ready.
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
cargo run -p wickra-genome -- \
  --spec examples/data/specs/dna.json \
  --data examples/data/universe \
  --op similar --symbol AAA --k 3
```

From Rust:

```rust
use wickra_genome_core::Genome;

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

## Building everything from source

```bash
cargo build --workspace --all-features                 # Rust core + CLI + C ABI
(cd bindings/python && maturin develop --release)      # Python
(cd bindings/node   && npm ci && npm run build)        # Node
(cd bindings/wasm   && wasm-pack build --target web)   # WASM
(cd bindings/csharp && dotnet build)                   # C#
(cd bindings/go     && go build ./...)                 # Go
(cd bindings/java   && mvn -q package)                 # Java
R CMD INSTALL bindings/r                               # R
```

Each binding builds against the C ABI hub in `bindings/c`, so build that first —
`cargo build -p wickra-genome-c` — and put the resulting library on the loader
path.

## Testing

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

Every binding runs the same golden corpus from [`golden/`](golden/) and must
produce the identical bytes; that corpus is the cross-language contract, not a
per-language approximation. `python scripts/check_binding_surface.py` asserts
the ten surfaces stayed in step.

## Requirements

- **Rust 1.86+** — the workspace MSRV; the Node binding needs **Rust 1.88**.
- **Python 3.9+** — the Python binding.
- **Node 22+** — the Node binding.
- **Go 1.23+** — the Go binding.
- **Java 22+** — the Java binding.
- **R 2.10+** — the R package.

Genome depends on `wickra-core` and `wickra-data` for the indicators and the
candle reader, resolves indicators through `wickra-backtest-core`, and — behind
the `live` feature — uses `wickra-exchange` for a live market feed. All four come
from crates.io.

## Benchmarks

See [BENCHMARKS.md](BENCHMARKS.md). Reproduce with `cargo bench -p genome-bench`
(and `--no-default-features` for the single-threaded path).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Genome reads
recorded market data and specs only — no keys, no order placement.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-radar**](https://github.com/wickra-lib/wickra-radar) — perp-universe alert radar: OI delta, funding flip, book imbalance, liquidation clusters, OI/price divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- [**wickra-benchmark**](https://github.com/wickra-lib/wickra-benchmark) — reproducible, golden-verified benchmark suite — recompute any (strategy, dataset, report) in ten languages and confirm it byte-for-byte
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-darwin**](https://github.com/wickra-lib/wickra-darwin) — evolutionary strategy search at hundreds of thousands of backtests per second, mutating and crossing JSON specs across the whole indicator registry
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 497 O(1) streaming indicators
- [**wickra-timemachine**](https://github.com/wickra-lib/wickra-timemachine) — scrub the whole market like a video — every symbol, full order book, rewound to any moment via deterministic re-fold
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-compile**](https://github.com/wickra-lib/wickra-compile) — compile a strategy spec into a standalone deployable: a WASM module, a self-contained binary, or a `no_std` artifact
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

The screener's own guides live in [`docs/`](docs/) beside the code; its site,
with the in-browser demo and the benchmark figures, is at
[screener.wickra.org](https://screener.wickra.org). The indicator library's
reference is at [docs.wickra.org](https://docs.wickra.org) and the org landing
page at [wickra.org](https://wickra.org).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Disclaimer

Wickra Genome is research and analytics software. Its similarity, clustering and
anomaly outputs are not investment advice, and nothing here is a recommendation
to trade. Use at your own risk.

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-genome">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-genome/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-genome/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-genome star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/star-history.svg">
</p>
