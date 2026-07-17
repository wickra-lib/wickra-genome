# Benchmarks

The headline figure is **vectors per second** — the rate at which the engine
builds an asset's 514-dimensional feature vector from its candles and folds it
into the market cross-section, plus the query throughput of similarity search and
clustering over that cross-section.

Reproduce with:

```bash
cargo bench -p genome-bench
```

_Placeholder — replaced with measured figures during the test-and-bench phase._
