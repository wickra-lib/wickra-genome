# Cookbook

Worked recipes over the `command_json` boundary. Every snippet is the same
sequence — construct a `Genome` from a spec, `build` a universe, then query — and
returns the byte-identical result in every language.

## Find every asset behaving like X right now

```json
{"cmd":"similar","symbol":"BTC","k":5}
```

Returns the five nearest neighbours of `BTC` by ascending distance under the spec
[metric](METRICS.md). Use `cosine` to match the *shape* of the microstructure
signature regardless of magnitude, `euclid` to match position in the normalized
space.

## Cluster the market into regimes

```json
{"cmd":"cluster","k":5}
```

Partitions the ready universe into five [seeded k-means](CLUSTERING.md) clusters —
each with a centroid and its member symbols. Fix `seed` in the spec so the regimes
are reproducible across runs and languages.

## Flag the anomalies

```json
{"cmd":"anomaly"}
```

Scores every ready symbol by the distance to its nearest neighbour and returns
them biggest-outlier-first. A symbol whose DNA has drifted away from the rest of
the market leads the list.

## Inspect one symbol's axes

```json
{"cmd":"vector","symbol":"BTC"}
```

Returns the self-describing vector — the value on each [feature](FEATURES.md) axis
with its canonical key — and whether the symbol is past warmup.

## From the CLI

```bash
wickra-genome --spec spec.json --data ./universe --op similar --symbol BTC --k 5
wickra-genome --spec spec.json --data ./universe --op cluster --k 5 --format json
wickra-genome --spec spec.json --data ./universe --op anomaly
```

`--data` is a directory of `<SYMBOL>.csv` OHLCV files; `--format` is `text`
(default) or `json` (the raw `command_json` response, byte-identical to every
binding).

## Track the market live

Behind the `live` feature, feed each incoming candle and re-query — see
[STREAMING](STREAMING.md). The [determinism contract](ARCHITECTURE.md#the-determinism-contract)
means the same feed replayed yields the same answers.
