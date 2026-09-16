# Wickra Genome examples — R

Runnable R examples for the [Wickra Genome R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-genome-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/genome.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `genome.R` | A runnable R example: build a genome over a tiny three-symbol universe and print the nearest neighbour and the biggest outlier. |
