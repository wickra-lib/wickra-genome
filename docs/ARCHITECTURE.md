# Architecture

Wickra Genome is one Rust core, `genome-core`, exposed everywhere through a single
JSON string boundary. Every language binding is a thin shim over the same core, so
there is exactly one implementation of the maths and one source of determinism.

## The pipeline

```
candles ──fold──▶ per-symbol raw vector ──cross-section──▶ normalize ──▶ query
  (O(1)/bar)         (one value / feature)      (z-score / min-max)   (metric)
```

1. **Spec.** A [`GenomeSpec`](FEATURES.md) fixes the feature axes, the
   cross-section normalization and the distance metric. It is a serde value, not
   Rust code, so the same JSON drives every language.
2. **Fold.** Each symbol's candles fold, one bar at a time, into a raw vector —
   one value per feature — through the `wickra-core` streaming indicators,
   resolved by name through the `wickra-backtest-core` registry. Folding is O(1)
   per bar; a symbol is *ready* once every axis is past warmup and finite.
3. **Cross-section.** The ready symbols form a matrix (symbols × features). It is
   [normalized](NORMALIZATION.md) per axis, then the queries run over it under the
   chosen [metric](METRICS.md).

## The `command_json` boundary

`Genome` owns a spec and a universe and dispatches one JSON command to one JSON
response — `Genome::command_json(&str) -> String`. Commands:

| Command       | Effect                                                       |
|---------------|--------------------------------------------------------------|
| `set_spec`    | Replace the spec, clearing the universe.                     |
| `feed`        | Fold one candle into a symbol (streaming).                   |
| `feed_batch`  | Fold a symbol's candle history.                             |
| `build`       | Fold a whole `{symbol: [candles]}` universe at once (batch). |
| `vector`      | One symbol's self-describing feature vector.                 |
| `similar`     | The `k` nearest neighbours of a symbol.                      |
| `cluster`     | The seeded k-means clustering into `k` clusters.             |
| `anomaly`     | Each symbol's nearest-neighbour outlier score.               |
| `reset`       | Clear the universe, keep the spec.                          |
| `version`     | The crate version.                                          |

A domain error comes back **in band** as an error object; a malformed command
never panics. Every binding forwards the command string verbatim and returns the
response verbatim, so all ten languages produce byte-identical output.

## The determinism contract

Byte-for-byte reproducibility, across languages and thread counts, rests on:

- **`BTreeMap` symbol ordering.** The universe is a `BTreeMap<String, _>`; every
  reduction iterates in ascending key order, so floating-point rounding is fixed.
- **Serial reductions.** Folding symbols parallelizes (feature `parallel`, rayon),
  but the cross-section reductions always run serially in key order — the
  parallel and single-threaded (WASM) builds agree.
- **A portable PRNG.** [k-means](CLUSTERING.md) is seeded by a self-contained
  `SplitMix64` driven by the spec `seed`, not a system RNG.
- **Fixed output rounding.** Every `f64` in a response is rounded to `1e-8` before
  serialization, so text output is stable.

This is the same contract the golden corpus (`golden/`) pins: for each spec and
op, the `command_json` response is asserted byte-for-byte from the Rust core, the
CLI and every binding.
