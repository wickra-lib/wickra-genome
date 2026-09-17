<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/ci.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-genome)
[![r-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/license.svg)](https://github.com/wickra-lib/wickra-genome#license)

# Wickra Genome — R

---

**Part of the [Wickra ecosystem](#ecosystem): — for R. `install.packages("wickragenome", repos = "https://wickra-lib.r-universe.dev")` — over the C ABI via `.Call`, prebuilt library fetched on install.**

R bindings for the Wickra Genome vector engine over its C ABI hub, via `.Call`. A
genome is built from a spec JSON and driven over a JSON boundary, so the
similarity, clustering and anomaly results are byte-identical to every other
Wickra Genome binding.

## Install

From r-universe:

```r
install.packages("wickragenome", repos = "https://wickra-lib.r-universe.dev")
```

The package's `configure` downloads the prebuilt C ABI library for this exact
version from the GitHub release and bundles it, so an ordinary install needs
nothing but a C toolchain (Rtools on Windows) for the thin `.Call` glue layer. To
build against a local checkout instead, point it at the header and library with
the environment variables below.

### Building from this repository (contributors)

The C ABI header and shared library are provided out-of-tree through two
environment variables (set by CI / the installer):

```bash
export WKGENOME_INC=/path/to/bindings/c/include   # the header dir
export WKGENOME_LIB=/path/to/target/release       # the library dir
R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

At run time the loader must find the shared library on `LD_LIBRARY_PATH`
(Linux), `DYLD_LIBRARY_PATH` (macOS) or `PATH` (Windows).

## Quick start

```r
library(wickragenome)

spec <- paste0(
  '{"features":[{"kind":"price","field":"close"}],',
  '"symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":24333}'
)
g <- wkgenome_new(spec)

data <- paste0(
  '{"AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],',
  '"BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],',
  '"CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}'
)
wkgenome_command(g, paste0('{"cmd":"build","data":', data, "}"))

cat(wkgenome_command(g, '{"cmd":"similar","symbol":"AAA","k":2}'), "\n")
cat(wkgenome_command(g, '{"cmd":"anomaly"}'), "\n")
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of R's native `.Call` interface over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-genome/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-genome>
- **Docs** (guides, spec reference, cookbook): <https://genome.wickra.org>
- **Runnable example:** [`examples/r/`](https://github.com/wickra-lib/wickra-genome/tree/main/examples/r)

Wickra Genome ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-genome/blob/main/SECURITY.md>.

## Disclaimer

Wickra Genome is research and analytics software. Its similarity, clustering and
anomaly outputs are not investment advice, and nothing here is a recommendation
to trade. Use at your own risk.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-MIT) at your option.
