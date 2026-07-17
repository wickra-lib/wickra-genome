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

[Unreleased]: https://github.com/wickra-lib/wickra-genome/commits/main
