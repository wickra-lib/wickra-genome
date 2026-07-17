# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the Rust workspace
  (`genome-core`, `genome-cli`, `genome-bench`) with the language-binding crates,
  and the `wickra-core` / `wickra-data` dependencies (the streaming indicators
  that build every asset's vector) plus the `wickra-exchange` git dependency (a
  live market feed, behind the `live` feature).
- `genome-core`: the market-genome vector engine. A data-driven `GenomeSpec`
  (feature axes, cross-section normalization, distance metric) turns each symbol
  into a live feature vector over the `wickra-core` streaming indicators, resolved
  by name through the `wickra-backtest-core` registry factory. Four queries run
  over that vector space — `vector`, `similar` (k nearest neighbors), `cluster`
  (deterministic seeded k-means++ / Lloyd) and `anomaly` (nearest-neighbor
  outlier scores) — behind the single `command_json` FFI boundary (`Genome`).
  Determinism is enforced end to end (`BTreeMap` ordering, serial key-order
  reductions, a portable `SplitMix64` PRNG, and fixed `1e-8` output rounding) so
  the batch and streaming paths and every language binding agree byte-for-byte.
- `wickra-genome` CLI over the core: `--spec` (JSON or TOML), `--data <dir>` of
  per-symbol `<SYMBOL>.csv` files or `--stdin` (a JSON dataset), `--op
  vector|similar|cluster|anomaly` with `--symbol` / `--k`, and `--format
  text|json`. The JSON output is the raw `command_json` response, byte-identical
  to what every language binding returns; the text output renders the vector
  axes, neighbor list, clusters or ranked anomaly scores.
- Ten language bindings over the `command_json` boundary: native Rust, Python
  (PyO3), Node.js (napi), WASM (wasm-bindgen), and — over a C ABI hub — C, C++,
  C#, Go, Java and R. Each forwards the command string verbatim, so every binding
  returns byte-identical answers.
- Golden corpus (`golden/`): a fixed six-symbol universe, five `GenomeSpec`
  envelopes and the blessed `command_json` responses for `vector` / `similar` /
  `cluster` / `anomaly`, reproducible byte-for-byte across the core, the CLI and
  every binding.
- Test, fuzz and bench surface: serde/validation conformance, a golden replay,
  streaming-equals-batch, proptest invariants, cargo-fuzz targets and criterion
  benchmarks; a Node cross-language golden pins the seeded-k-means result
  byte-for-byte across languages.
- A runnable example in every language (`examples/`) and eight CI workflows
  (`ci.yml`, CodeQL, Scorecard, zizmor, links, bench, sync-metadata, release).
- Documentation: `README`, `docs/{ARCHITECTURE,FEATURES,NORMALIZATION,METRICS,CLUSTERING,STREAMING,Cookbook}.md`,
  and measured `BENCHMARKS.md` figures.

[Unreleased]: https://github.com/wickra-lib/wickra-genome/commits/main
