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

**A vector database of the whole market: every asset as a 514-dim live vector
over 514 O(1) streaming indicators. Similarity search, clustering and anomaly
detection over microstructure DNA.**

Every asset has a shape — the momentum, volatility, flow and microstructure
signature of how it is trading *right now*. Wickra Genome turns that shape into a
514-dimensional vector, one coordinate per `wickra-core` streaming indicator, and
lets you query the whole market by it: **find every coin behaving like X right
now**, cluster the market into regimes, or flag the assets whose DNA has gone
anomalous.

Genome is one data-driven core, `genome-core`: it builds each asset's vector from
the same `wickra-core` indicators the rest of the ecosystem uses, then runs
similarity search, k-means clustering and anomaly scoring over the cross-section.
The core is exposed as a **JSON-over-C-ABI data API** (`command_json`) in **Rust,
Python, Node.js, WASM, C, C++, C#, Go, Java and R**, plus a reference CLI.

> **Status:** early scaffold (0.1.0, unreleased). The repository skeleton,
> workspace and governance are in place; the vector engine, the CLI, the ten
> bindings, the golden corpus and CI land in the phases that follow.

## Use in any language

The same handle + `command_json` + `version` surface ships for Rust, Python,
Node.js, WASM, and — over a C ABI hub — C, C++, C#, Go, Java and R. Each binding
forwards the command string verbatim, so the report they return is identical.

## Building from source

```bash
cargo build
cargo test
```

## Requirements

- Rust 1.86+ (MSRV). Genome depends on `wickra-core` and `wickra-data`
  (crates.io) for the indicators and the candle reader, and, behind the `live`
  feature, `wickra-exchange` (a git dependency) for a live market feed.

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
