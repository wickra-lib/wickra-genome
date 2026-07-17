# Wickra Genome — R

R bindings for the Wickra Genome vector engine over its C ABI hub, via `.Call`. A
genome is built from a spec JSON and driven over a JSON boundary, so the
similarity, clustering and anomaly results are byte-identical to every other
Wickra Genome binding.

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/wickra-lib/wickra-genome#license)
[![.Call](https://img.shields.io/badge/bindings-.Call-3b82f6)](https://cran.r-project.org/doc/manuals/r-release/R-exts.html)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

## Build & test

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

## Usage

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

## License

Dual-licensed under either of Apache-2.0 or MIT at your option.
