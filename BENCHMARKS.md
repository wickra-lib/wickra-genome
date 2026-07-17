# Benchmarks

The headline figure is **vectors per second** — the rate at which the engine
builds an asset's feature vector from its candles and folds it into the market
cross-section, plus the query latency of similarity search and clustering over
that cross-section.

Reproduce with:

```bash
cargo bench -p genome-bench --bench genome                       # parallel-capable build
cargo bench -p genome-bench --bench genome --no-default-features # single-threaded (WASM) path
```

## Measured (reference run)

Criterion, release build, over a fixed 64-bar universe; `build` throughput is
vectors (symbols) per second, queries are median wall-clock latency. Absolute
numbers depend on the host — treat them as an order of magnitude and reproduce
locally for your hardware.

### `build` throughput (vectors/second)

| Features | 100 symbols   | 1,000 symbols |
|----------|---------------|---------------|
| 5        | ~135 K vec/s  | ~169 K vec/s  |
| 50       | ~19 K vec/s   | ~34 K vec/s   |

Throughput is dominated by the streaming-indicator folds — more feature axes mean
more indicators per bar — while the cross-section reductions stay cheap. Building
a 1,000-symbol universe with a five-axis spec is a few milliseconds.

### Query latency (median)

| Query              | 5 features, 1,000 symbols | 50 features, 1,000 symbols |
|--------------------|---------------------------|----------------------------|
| `similar` (k=10)   | ~1.9 ms                   | ~19.7 ms                   |
| `cluster` (k=8)    | ~2.6 ms                   | ~35.0 ms                   |

`similar` is a single pass over the cross-section; `cluster` runs seeded k-means
to convergence, so its cost scales with the feature count and the iteration count.
Both are interactive at a thousand symbols.

The `parallel` feature (default) parallelizes the per-symbol fold with rayon; the
cross-section reductions and the seeded k-means always run serially in key order,
so the single-threaded (`--no-default-features`, WASM) build returns byte-identical
results at lower build throughput.
