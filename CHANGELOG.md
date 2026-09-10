# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **The headline indicator count was the catalogue figure, not the reachable
  one.** 514 is what the `wickra-core` catalogue ships; what a spec can actually
  name is what the shared registry in `wickra-backtest-core` resolves, and that
  `build` match has 497 arms. The two are different sets — bar builders emit
  bars rather than a value per bar, and a handful of indicators the registry
  does not carry yet — so every 514 promised names a spec would be refused for.

- **The vector was described as fixed at 514 dimensions.** `GenomeSpec::features`
  is what fixes it — `features.len()` is the dimension, chosen per spec — and the
  axes are drawn from the 497 names the registry resolves. `ARCHITECTURE.md`,
  `ROADMAP.md`, the core crate description and its module docs all said otherwise.

- **`CITATION.cff` described the wrong project.** The abstract and the keyword
  list were the feature store's, down to "a data-driven FeatureSpec ...
  materialized as a feature matrix" for a vector database. `CITATION.cff` is
  what GitHub's citation box and Zenodo quote back at a reader as the project's
  own words, so it is the one file where a wrong description is the project
  saying it.

- **The Ecosystem section repeated two claims their own repositories had already
  corrected**: DARWIN at "millions of backtests per second" across "the
  514-indicator space", where its benchmark says hundreds of thousands over the
  registry, and GENOME as "a 514-dim live vector", where the dimension is
  whatever the spec's feature list names.

- **Indicators that read a side feed produced nothing, silently, and took the
  whole symbol with them.** `IndicatorSet::update` hardcoded the reference
  series, derivatives tick, order book, trades and cross-section to absent, so
  an indicator needing any of them ticked and returned nothing every bar. A
  symbol with any `None` axis is never ready, and the ready universe is what
  similarity search, clustering and anomaly scoring iterate — so one dead axis
  emptied it. Measured before the fix, a spec of `Microprice` plus the close
  gave `similar("AAA") = Err("unknown symbol: AAA")` for a symbol plainly in
  the universe, `cluster()` = 0 and `anomaly()` = 0, with nothing naming the
  cause.

### Added

- **Side feeds.** `SymbolSeries` carries the batch form (parallel arrays, one
  entry per candle, mirroring `wickra-backtest`'s `RunRequest`) and a `feed`
  carries the per-bar form (`StepFeeds`, reused verbatim). The bare candle
  array still works, so every existing spec, fixture and binding payload is
  unchanged.

- **A feed check that refuses rather than answers.** `GenomeSpec::check_feeds`
  rejects a spec whose axis needs a feed the caller cannot supply, naming the
  indicator and the feed, on the batch build and on each streamed bar alike; a
  feed whose length differs from the candle count is an error rather than a
  fold that runs out part-way.

- **Streaming-equals-batch tests in every binding.** Python, Node, Go, Java,
  C#, R, WASM and C each drive both paths through the JSON boundary and compare
  all four queries. Every other test in every language only ever sent
  `{"cmd":"build"}`, so `feed` crossed those boundaries untested.

- **A golden test for the C binding**, which had none, holding all twenty
  spec × query comparisons byte-identical, plus `golden/data.json` so any
  binding can load the corpus without a CSV parser of its own.

- The blueprint scaffold: `LICENSES/`, `docs/README.md`, `docs/FEEDS.md`, the
  five long-form issue templates, the CodeQL config, the actionlint and
  CodSpeed workflows, the five check scripts, a C++ hull, licence copies in
  every published crate and npm package, a WASM example, and dependabot
  coverage for the fuzz workspace and the Go and Java examples.

- CI gains `osv`, `links`, `binding-surface`, `semver`, `fuzz-smoke`,
  `examples` and `python-wheel-container-smoke`; the release pipeline gains the
  `gate` and `guard` jobs, provenance over the nupkg, jar and C ABI archives, a
  Maven artifact on the release page, and a Go mirror that builds before it
  publishes.

### Changed

- **The family pins move to the published releases.** `wickra-backtest-core`
  comes from crates.io at 0.1.4 rather than a git rev 130 commits behind,
  `wickra-exchange` at 0.1.3 rather than a git rev, and `wickra-core` /
  `wickra-data` rise from 0.9 to 1.0, so the tree carries one set of indicator
  types rather than two that share none.

- `CMAKE_CXX_STANDARD` moves from 14 to 17, which the C++ hull requires and
  nothing compiled it to find out.

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
