# Golden corpus

The generate-once / replay-everywhere fixtures that pin Wickra Genome's
determinism: for every `specs/<name>.json` run against the shared `data/`
universe, each query's `command_json` response serializes **byte-for-byte** to
`expected/<op>/<name>.json` — in the Rust core, the CLI, and every one of the
ten language bindings.

> **Never edit `expected/**/*.json` by hand.** They are generated (blessed) from
> the core via the CLI. A hand edit desyncs the byte-golden and every downstream
> binding test fails.

## Layout

| Dir               | Contents                                                             |
|-------------------|---------------------------------------------------------------------|
| `data/`           | A fixed six-symbol universe: `AAA … FFF`, one `<SYMBOL>.csv` each.   |
| `specs/`          | Canonical `GenomeSpec` envelopes (features + normalize + metric + seed). |
| `expected/vector/`  | One blessed `vector` response per spec (feature vector of `AAA`).  |
| `expected/similar/` | One blessed `similar` response per spec (3 nearest neighbours of `AAA`). |
| `expected/cluster/` | One blessed `cluster` response per spec (seeded k-means).         |
| `expected/anomaly/` | One blessed `anomaly` response per spec (nearest-neighbour scores). |

Every spec runs against the **same** `data/` directory. `build` folds every
symbol file it finds, so all six symbols participate in every query regardless of
a spec's `symbols` list (that list is validated at construction, not used to
subset the universe).

## Data formula

Each symbol is a fixed closed-form OHLCV path over **48 hourly bars** starting at
epoch `1700000000` (step `3600`). For symbol index `s ∈ 0..6` (`AAA=0 … FFF=5`)
and bar `i ∈ 0..48`:

```
base  = 100 + 3·s
drift = 0.05 + 0.03·s   (+0.9 for CCC, the deliberate outlier)
amp   = 1 + (s mod 3)    (+4.0 for CCC)
phase = 0.7·s
mid_i   = base·(1 + drift·i/100) + amp·sin(0.5·i + phase)
open_i  = close_{i-1}   (open_0 = mid_0)
close_i = mid_i
high_i  = max(open_i, close_i) + 0.5
low_i   = min(open_i, close_i) − 0.5
volume_i = 1000 + 10·i
```

Values are rounded to four decimals. `CCC` carries the steepest drift and widest
amplitude, so it consistently leads the anomaly ranking; `AAA` and `BBB` are the
closest pair. The numbers are fixed, not sampled: the corpus is reproducible
bit-for-bit. The generator lives in the repository history; re-deriving the CSVs
must reproduce these files exactly.

## Determinism notes

- **Symbol order** is the `BTreeMap` key order (`AAA < BBB < … < FFF`); every
  reduction iterates in this order so `f64` rounding is identical across
  languages and thread counts.
- **Clustering** is seeded k-means (k-means++ seeding driven by the portable
  `SplitMix64` PRNG from the spec `seed`), so `cluster` is reproducible without a
  system RNG. `cluster5.json` carries five features and is blessed at `k=5`; the
  other specs are blessed at `k=3`.
- **Output rounding** is `round_to(1e-8)`; the parallel (native) and serial
  (wasm) paths agree byte-for-byte.

## Bless command

Regenerate every `expected/**/*.json` from the current core (run after any
intentional change to the feature pipeline, metric, or response shape, and review
the diff):

```bash
cargo build -p genome-cli --release
BIN=target/release/wickra-genome
for spec in dna cosine_dna minmax_dna cluster5 anomaly; do
  ck=3; [ "$spec" = "cluster5" ] && ck=5
  "$BIN" --spec golden/specs/$spec.json --data golden/data --op vector  --symbol AAA          --format json > golden/expected/vector/$spec.json
  "$BIN" --spec golden/specs/$spec.json --data golden/data --op similar --symbol AAA  --k 3    --format json > golden/expected/similar/$spec.json
  "$BIN" --spec golden/specs/$spec.json --data golden/data --op cluster --k $ck             --format json > golden/expected/cluster/$spec.json
  "$BIN" --spec golden/specs/$spec.json --data golden/data --op anomaly                     --format json > golden/expected/anomaly/$spec.json
done
```

## Cases

| Spec           | Features                          | Normalize | Metric   | Seed  | What it pins                                                  |
|----------------|-----------------------------------|-----------|----------|-------|--------------------------------------------------------------|
| `dna`          | Rsi(14), Roc(10), close           | z_score   | euclid   | 24333 | The worked example: `AAA`'s closest neighbour is `BBB`.      |
| `cosine_dna`   | Rsi(14), Roc(10), close           | z_score   | cosine   | 24333 | Cosine distance reorders neighbours vs the euclidean metric. |
| `minmax_dna`   | Rsi(14), Roc(10), close           | min_max   | euclid   | 24333 | Min–max normalization rescales the axes before distance.     |
| `cluster5`     | Rsi, Roc, Sma, Ema, close         | z_score   | euclid   | 777   | Five-feature seeded k-means at `k=5`.                        |
| `anomaly`      | Roc(10), Rsi(14), close           | z_score   | euclid   | 4242  | The outlier `CCC` leads the nearest-neighbour anomaly score. |
