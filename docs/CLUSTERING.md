# Clustering

The `cluster` query partitions the ready cross-section into `k` clusters with
**seeded k-means**. The whole procedure is deterministic — the same universe, spec
and `seed` always produce the identical clustering, in every language — because it
draws its randomness from a portable PRNG, never a system RNG.

## The algorithm

1. **k-means++ seeding.** The first centroid is a symbol chosen by the PRNG; each
   further centroid is chosen with probability proportional to its squared
   distance to the nearest already-chosen centroid (the D² rule). This spreads
   the initial centroids and avoids the degenerate starts plain random seeding can
   hit.
2. **Lloyd iterations.** Assign each symbol to its nearest centroid under the spec
   [metric](METRICS.md); recompute each centroid as the mean of its members;
   repeat until assignments stop changing or `MAX_ITERS` (100) is reached.
3. **Result.** Each cluster reports its centroid (one value per axis) and its
   member symbols, sorted ascending by key. `k` is clamped to the ready count.

## The portable PRNG

Randomness comes from a self-contained `SplitMix64` seeded by the spec `seed`:

```
state ← seed
next():
  state ← state + 0x9E3779B97F4A7C15
  z ← state
  z ← (z XOR (z >> 30)) · 0xBF58476D1CE4E5B9
  z ← (z XOR (z >> 27)) · 0x94D049BB133111EB
  return z XOR (z >> 31)
```

The first output for `seed = 0` is pinned to `0xE220A8397B1DCDAF` by a unit test.
Because the PRNG is part of the Rust core and every binding forwards the
`command_json` string verbatim, **only the Rust core ever draws random numbers** —
there is no per-language RNG to diverge. That is why the `cluster` response is
byte-identical across all ten languages: it is the strongest proof in the golden
corpus.

## Determinism contract

- Same `(universe, spec, seed)` ⇒ identical clusters (asserted by proptest).
- Members partition exactly the ready universe (`Σ |cluster_i| = ready count`).
- The parallel (native) and single-threaded (WASM) builds agree — the fold
  parallelizes, but the clustering reduction runs serially in key order.
