# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **The napi bump split a crate in two and the build stopped.**
  `napi-derive-backend` 6.1.3 pulls `convert_case` 0.12 while `napi-derive`
  3.6.3 still uses 0.11, and two versions of a crate are two unrelated types --
  so `napi-derive` itself failed to compile, taking the node binding, the clippy
  job and every `cargo build --workspace` row down with it. Held at 6.1.2, which
  is what the screener runs.

- **`cargo-deny` was set to warn about duplicated crates, so it noted that split
  and moved on.** It is an error now. Only four duplicates exist across this
  workspace and each is a crate part-way through a major release reached through
  two ecosystems; they are skipped by name with the reason recorded, so a fifth
  still fails. Verified by putting 6.1.3 back and watching the check fail on
  `convert_case` before the compiler ever ran.

- **`actionlint` failed on five shell constructs the screener had already
  fixed.** `a && b || c` is not if-then-else -- when the publish succeeded but
  the echo failed, the fallback branch ran and reported "already published";
  `local pkg=$(basename …)` and `export PATH="$(cygpath …)"` hide the command's
  exit status behind `local`/`export`; and an asset count taken from `ls` breaks
  on a filename containing a newline. The runner-label config the linter needs
  for `windows-11-arm` was missing too.

- **A yanked crate was in the lockfile.** `wnaf` 0.14.0, reached through `p256`
  -> `wickra-exchange-core`, was yanked from crates.io; 0.14.1 is not.

- **Three steps of the `examples` job ran a file that is not here.** Python,
  Node.js and R each invoked `examples/<lang>/scan.*` -- the screener's file
  name, left over from the port -- so they died on a missing file before
  reaching any assertion.

- **Every language step asserted `"symbol":"BBB"`**, a line from the screener's
  scan report that no example here prints. Each step now matches a string its
  own example emits, read off the format string rather than guessed: the
  assertions were checked against a real run of each example.

- **The Rust example dropped the response of the command that loads its data.**
  `command_json` is `#[must_use]`; `build` is what fills the genome, and a
  refusal arrives in-band as `{"ok":false,...}`. Ignoring it would have left the
  queries below running against an empty genome. It is checked now.

- **The Rust example's lockfile pinned the pre-migration engine.** It still held
  `wickra-core` 0.9.9 and `wickra-backtest` 0.1.0 while the workspace declares
  1.0 and 0.1.4, so cargo silently repaired the lock on every build and the
  example was the one reach measured against a different engine than the rest.

- **The C++ example now goes through the C++ hull.** It called the C functions
  directly and rebuilt the two-call length protocol by hand -- the very thing
  `wickra_genome.hpp` exists to remove -- which left the shipped C++ surface
  built by nothing. Verified by running both: the C and C++ examples print
  byte-identical output.

- **Every C++ hull used the include guard `WICKRA_SCREENER_HPP`.** The C headers
  beside them are guarded correctly; only the `.hpp` files shared one name, so
  including two of the family's headers in the same translation unit dropped the
  second silently. Proven by compiling a file that includes two of them and
  names a class from each: `'Env' is not a member of 'wickra'`. All seven now
  compile standalone and together.

- **The hull's usage example could not run.** It showed a spec shaped
  `{"universe":[...]}` and `{"cmd":"scan"}`, the screener's, which this core
  rejects twice over. It now shows this repository's own spec fields and one of
  its own commands.

- **The release notes named the wrong package.** They told a reader
  `install.packages("wickrafeaturestore")` from r-universe, where this package
  is `wickragenome`. These notes go out with the GitHub release: a reader
  following them installs a different library.

- **The issue and pull-request templates asked for a `ScanSpec`**, a type this
  repository does not have, so a contributor was asked to attach something that
  does not exist. `GOVERNANCE.md`, `SUPPORT.md` and `CONTRIBUTING.md` carried
  the same substitution, along with the screener's "condition schema" for a core
  that has no conditions.

- **The R `configure` scripts still defined `wkscreen_download`**, the last
  trace of the screener's prefix — the CI-visible half of which already had to
  be fixed once.

- **The Java example's `exec-maven-plugin` was a version behind the family.**
  Dependabot opened the 3.5.0 -> 3.6.3 bump in the four sibling repositories
  that carry the same example pom and not in this one, so it would have stayed
  on 3.5.0 until the next cycle noticed. Verified by running the example on
  3.6.3.

- **Six SHA-pinned actions sat on two lines across the family**, and two of the
  splits were inside this repository. `actions/setup-node` is pinned at the same
  commit everywhere, but some call sites annotated it `# v6.4.0`; GitHub's tag
  list says that commit is **v7.0.0** and v6.4.0 is a different one. Dependabot
  reads that comment to decide what to bump, so a wrong one misdirects the tool
  meant to keep the pin current. `Swatinem/rust-cache` ran at two commits at
  once, the older behind a floating `# v2`. Every pin now matches what the
  sibling repositories run, each target checked against the upstream tag list.

- **The CI Java example step compiled a file that is not there.** The `examples`
  job was ported from the screener, whose Java example is a single
  `examples/java/Scan.java` built with `javac`. This repository ships a Maven
  project instead, so the step compiled a missing file and then asserted on
  output the example never prints. It now builds the binding into the local
  repository and runs the example through `mvn exec:exec`, the way the example's
  own javadoc documents -- verified by running it.

- **The `examples` job installed a lockfile that is not here.** It names
  `.github/requirements/ci-dev-py3.txt`, and so does
  `scripts/update-lockfiles.sh`, but the directory held a single `ci-dev.txt`
  that nothing referenced. The split is not cosmetic: the Python matrix includes
  3.9, and that single lock pinned `pytest==9.1.1` and `iniconfig==2.3.0`, both
  of which declare requires-python >= 3.10.

- **The `python` job installed unpinned.** `pip install maturin pytest` is a
  fetch of whatever the index serves that minute -- the exact thing the locked
  file exists to prevent. It now installs the hash-locked row for its
  interpreter, and the advisory the 3.9 pin sits inside is recorded with its
  reason in `osv-scanner.toml`.

- **Dependabot watched directories that do not exist**, so it reported nothing
  and the silence read as calm. `nuget` pointed at `WickraCompile.Tests`, a
  project name from another repository; `pip` did not cover
  `/.github/requirements` and `npm` did not cover `/examples/node`.

- **The workspace's own core was pinned as a range.** `genome-core` was named
  six times as `version = "0.1"` -- a caret range -- and the root manifest
  carried no `[workspace.dependencies]` entry for it at all. A published
  `genome-cli` 0.1.0 would have accepted `genome-core` 0.1.99, a crate resolving
  against a core it was never built against, in a workspace whose whole point is
  that the pieces move together. It also hid the line from `bump_version.py` and
  `check_version_sync.py`, both of which look for the exact version.

- **`release.yml` overwrote the binding READMEs before packing.** Three steps
  copied the root README over `bindings/python/README.md` (wheel and sdist) and
  `bindings/node/README.md`. They date from when the bindings had no README of
  their own; they do now, one per registry, and `check_readme_links.py` exists
  to keep their links absolute because a relative link is dead on PyPI and npm.
  The copy threw that away and shipped the root README, whose links are relative
  by design. The remaining relative links in the C, C#, Go and WASM READMEs are
  absolute now.

- **The Python wheel would have shipped without its licence texts.**
  `bindings/python/` carried neither `LICENSE-MIT` nor `LICENSE-APACHE`, so
  maturin had nothing to include, while every crate and the release archive
  carry both.

- **`SECURITY.md` named a support policy for releases that do not exist yet.**
  It promised fixes for "the latest `0.x` release line" where there is no
  released line; it now says plainly that nothing is published and names `0.1.0`
  as the first version that will be.

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
