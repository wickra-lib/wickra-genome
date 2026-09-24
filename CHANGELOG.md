# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4] - 2026-09-24

A follow-up release: the market vector database and its bindings are unchanged.
It pins wickra-exchange 0.1.8, the release that makes exchange's R package build
on r-universe's WebAssembly target and its release pipeline re-runnable.

### Changed

- **Built on wickra-exchange 0.1.8.** The exact pin on `wickra-exchange` moves
  from =0.1.7 to =0.1.8, and every tracked lockfile follows. Nothing in
  exchange's Rust API changed between the two; 0.1.8 fixes its R package's
  WebAssembly build and its Maven Central step.

## [0.1.3] - 2026-09-23

A maintenance release: the market vector database and its bindings are
unchanged. It publishes the refreshed dependency tree and toolchain pins.

### Added

- **The Node binding reports which artifact it loaded.** The loader generated
  by `@napi-rs/cli` 3.10.4 exports `__napiBindingTarget` -- `'native'` for the
  native addon, otherwise the WASI flavor it resolved -- and follows a
  `NAPI_RS_NATIVE_LIBRARY_PATH` override to a WASI loader instead of
  misreporting it as native. Typed in `index.d.ts`.

### Changed

- **Built on wickra-core 1.0.6.** The lock takes the indicator core's latest
  release; the `1.0` requirement already admitted it.
- **The family pins follow the owners' releases.** `wickra-backtest-core` =0.1.7
  -> =0.1.8, `wickra-exchange` =0.1.6 -> =0.1.7 -- the exact pins this
  repository keeps on its siblings move to the versions those repositories
  release in the same train, and every tracked lockfile follows.
- **Third-party dependencies refreshed.** `Cargo.lock` takes 108 crates to their
  newest versions compatible with the Rust floor (the lock now resolves
  MSRV-aware, see below), run across the family in one pass so every repository
  resolves the same day's versions. The refresh itself changes no manifest.
- **The lockfile resolves for the Rust floor.** `.cargo/config.toml` sets
  `incompatible-rust-versions = "fallback"`, so `cargo update` takes the newest
  version the workspace's `rust-version` can build rather than the newest
  release -- the setting compile, copilot and shazam already carried, now
  family-wide. Without it, a routine refresh elsewhere in the family raised the
  icu crates to 2.3.0, which declares Rust 1.88, above a 1.86 floor. Re-resolved
  under it, the lock steps back to the newest versions the floor can build for
  `icu_collections`, `icu_locale_core`, `icu_normalizer`, `icu_normalizer_data`,
  `icu_properties`, `icu_properties_data`, `icu_provider`, `wasip2`,
  `wit-bindgen`.
- **`@napi-rs/cli` 3.10.4** for the Node binding, the family's line.
- **uv 0.12.18** for the lockfile bootstrap in `scripts/update-lockfiles.sh`,
  with all four platform checksums moved together.
- **The README's static badges are served by the organization** rather than
  hot-linked from shields.io, so they no longer break when shields is down.

### Fixed

- **The Go install line names the published module.** The badge and `go get`
  in `bindings/go/README.md` pointed at the in-repo path
  `github.com/wickra-lib/wickra-genome/bindings/go`, which the Go proxy never
  serves; they now name the mirror `github.com/wickra-lib/wickra-genome-go`, as the
  other bindings' READMEs do.

## [0.1.2] - 2026-09-18

### Fixed

- **The Java binding loads the library it ships.** The jar carries the native
  library under `native/<os>-<arch>/` -- the release pipeline stages every
  platform there -- but the loader only ever looked at `-Dnative.lib.dir` and
  the working directory, so a Maven Central consumer got a jar it could not
  load without pointing the JVM at a library it had to build itself. The loader
  now resolves in wickra's order: `-Dnative.lib.dir` when set, the bundled copy
  extracted to a temporary file, every `target/release` or `target/debug` up
  the tree from the working directory and the class's own location, then the
  bare name.

### Changed

- **Family pins follow the owners' releases:** wickra-backtest-core =0.1.6 -> =0.1.7, wickra-exchange =0.1.5 -> =0.1.6. No code of this repository changes; the engine it links is the one those releases ship.
- **Every README follows wickra's shape.** A cross-repo scan compared the
  heading skeleton of each README against wickra's and this repository's
  differed throughout. The root README opens as wickra's does (banner, badges,
  the one-liner, the live-demo and ecosystem lines, no separate H1), the
  License section carries wickra's wording and its `### Contribution` clause,
  and the shared sections run in wickra's order. Each binding README is
  `Install`, `Quick start`, `Benchmark`, `Documentation`, `Security`,
  `Disclaimer`, `License` with the product's own surface and protocol notes
  as subsections; the registry pages that render them now say how to report a
  vulnerability and under which licence the package ships.
  `examples/README.md` lists every language the way wickra's does, with the
  commands the CI examples job runs; the per-language example READMEs,
  `fuzz/README.md` and the `## Editing the docs` section of
  `docs/README.md` exist as they do in wickra.

### Changed

- **wickra-backtest-core 0.1.6 and wickra-exchange 0.1.5.** The pins move to the releases the family is on; the lock follows.
  A cross-repo scan lined the 24 wickra-lib repositories up, and the rest is
  what this one spelled differently: the fuzz job runs the family's pinned
  `nightly-2026-07-01` rather than a rolling nightly, and the example job's
  `dotnet-version` reads `8.0.x`.

### Changed

- **wickra-backtest-core 0.1.6 and wickra-exchange 0.1.5.** The pins move to the releases the family is on; the lock follows.
  A cross-repo scan lined the 24 wickra-lib repositories up, and the rest is
  what this one spelled differently: the fuzz job runs the family's pinned
  `nightly-2026-07-01` rather than a rolling nightly, and the example job's
  `dotnet-version` reads `8.0.x`.

### Changed

- **uv 0.12.15 for the lockfile script.** `scripts/update-lockfiles.sh`
  bootstraps 0.12.15 (was 0.12.13); the pin and all four release
  checksums move together, taken from the release's `.sha256` files.

## [0.1.1] - 2026-09-14

### Fixed

- **The R package installs on macOS and Windows.** r-universe built the
  first release on every platform and failed on ten of thirteen: the package
  object linked the C ABI library but nothing bundled it, so macOS could not
  load `@rpath/libwickra_*.dylib`, and `Makevars.win` still expected the
  header and library through environment variables that r-universe never
  sets, so the Windows link found no symbols at all. The package is in the
  family's form now: `configure` / `configure.win` stage the library into
  `src/`, `install.libs.R` bundles it beside the package object (the DLL under
  its `_abi` name, the dylib and the `.so`), `Makevars.win` links the import
  library `configure.win` builds, a shipped `tests/smoke.R` runs inside the
  tarball, and `DESCRIPTION` states the R floor.

## [0.1.0] - 2026-09-14

### Fixed

- **CI is green again.** The R test matched `"values":[20,` against an
  envelope that prints floats with their fraction (`[20.0,`), so the R job
  failed on every platform while the binding was right. The `kmeans` fuzz
  target did not compile: `build()` takes `SymbolInput` per symbol and the
  target still passed bare candle vectors. `CONTRIBUTING.md` linked a
  `docs/LABELS.md` that does not exist. The napi glue
  (`bindings/node/index.js`) was stale against the locked CLI, so the
  in-sync check failed on every Node job; it is regenerated. The Examples
  job ran `cargo run -p wickra-genome-example` against a crate that is not a
  workspace member, pointed `dotnet run` at a project directory that does
  not exist (`Genome` is the project), and the Node and C# examples depended
  on the npm and NuGet packages, which are not published yet: both examples
  now reference the binding in this checkout, which also un-breaks CodeQL's
  C# autobuild. osv-scanner runs with `--no-resolve`, since the Java
  example's dependency on the unpublished org.wickra artefact cannot be
  resolved from Maven Central until the release exists.
- **The Maven Central publish is idempotent, and waits as long as Central
  takes.** A sibling's first release deployed successfully and still went red:
  Central published after the plugin's default 30-minute wait had expired,
  and a rerun could only fail on the duplicate. The release workflow now skips
  a version already on Central, the plugin waits up to two hours
  (`waitMaxTime`), and the job has the budget for it.
- **The engine pins are exact** (`wickra-backtest = "=0.1.4"`, and the
  exchange client where it is used), as the released siblings pin them, so a
  newer patch on one side cannot leave two copies of the engine in one graph.
- zizmor's `self-repository` note is a documented policy (`.github/zizmor.yml`)
  rather than an open alert per workflow; uv 0.12.13 for the lockfile script;
  the C# test packages are the family's (Microsoft.NET.Test.Sdk 18.9.0,
  xunit.runner.visualstudio 4.0.0).
- **The Python 3.9 CI row runs without pytest.** pytest 9.x requires 3.10,
  so that row could only pin 8.4.2, below the fix for GHSA-6w46-j5rx-g56g
  with no backport. The 3.9 lock carries maturin only, and the row runs the
  same test modules through `bindings/python/tests/run_without_pytest.py`
  (plain functions, plain asserts); 3.10 and up run them under pytest as
  before.
- **The R package builds for WebAssembly on r-universe.** `configure`
  refused the wasm target outright, which would have left the `wasm-release`
  job red on every build. The r-universe wasm image ships cargo and
  emscripten, so `configure` now builds the C ABI staticlib from the release
  tag's source for `wasm32-unknown-emscripten` right there and links it into
  the package object, the way the released siblings do.
- **The exported R functions are documented.** `wkgenome_new`, `wkgenome_command`
  and `wkgenome_version` carried roxygen comments but no generated `man/` pages,
  which `R CMD check` reports as a WARNING on every platform.
- **The two published crates carried names the release could not upload.**
  `genome-core` and `genome-cli` are outside the org's crates.io token scope,
  which creates new crates under the `wickra-` prefix only; `cargo publish` on
  either name returns 403 at upload while `--dry-run` passes, and because the
  publish jobs run in parallel the release would have landed on PyPI, npm,
  NuGet, Maven Central and the Go mirror without ever reaching crates.io.
  `genome-cli` is also taken -- 0.2.1 belongs to an unrelated project -- and
  `release.yml` already published `-p wickra-genome`, a package that did not
  exist. The core is now `wickra-genome-core` and the CLI crate
  `wickra-genome`, matching the binary it ships and the shape of every
  released sibling. Directories keep their names; only the packages and the
  `wickra_genome_core` path moved. The same audit ran across the family (xray
  paid for this with its first tag).

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

- **The workspace's own core was pinned as a range.** `wickra-genome-core` was named
  six times as `version = "0.1"` -- a caret range -- and the root manifest
  carried no `[workspace.dependencies]` entry for it at all. A published
  `wickra-genome` 0.1.0 would have accepted `wickra-genome-core` 0.1.99, a crate resolving
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
  (`wickra-genome-core`, `wickra-genome`, `genome-bench`) with the language-binding crates,
  and the `wickra-core` / `wickra-data` dependencies (the streaming indicators
  that build every asset's vector) plus the `wickra-exchange` git dependency (a
  live market feed, behind the `live` feature).
- `wickra-genome-core`: the market-genome vector engine. A data-driven `GenomeSpec`
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

[Unreleased]: https://github.com/wickra-lib/wickra-genome/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/wickra-lib/wickra-genome/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/wickra-lib/wickra-genome/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/wickra-lib/wickra-genome/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/wickra-lib/wickra-genome/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/wickra-lib/wickra-genome/releases/tag/v0.1.0
