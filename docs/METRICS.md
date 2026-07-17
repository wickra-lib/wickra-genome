# Metrics

A metric turns two [normalized](NORMALIZATION.md) vectors into a distance. It is
chosen per spec by `metric`, and it governs `similar`, `cluster` and `anomaly`
alike.

## `cosine` (default)

Cosine distance is `1 − cosine similarity`:

```
cos_sim(a, b) = (a · b) / (‖a‖ · ‖b‖)
distance      = 1 − cos_sim(a, b)
```

- Range `[0, 2]`; identical direction → `0`, opposite → `2`.
- A zero-norm vector (every axis `0`, e.g. a flat market under z-score) has an
  undefined direction; its distance is defined as `1`.

Cosine compares **shape** — the pattern of the axes — independent of magnitude,
so two assets moving the same way at different intensities land close together.

## `euclid`

Euclidean (L2) distance:

```
distance = sqrt( Σ_j (a_j − b_j)² )
```

Range `[0, ∞)`. Euclid compares **position** in the normalized space, so both the
pattern and the (normalized) magnitude matter.

## Choosing

- Use **`cosine`** to group assets by the *shape* of their microstructure
  signature, ignoring how far each has moved.
- Use **`euclid`** when the normalized magnitude is itself meaningful — e.g. when
  an axis is already a z-score and "two standard deviations out" should read as
  far.

Distances feed the queries directly: `similar` sorts neighbours by ascending
distance, `anomaly` scores each symbol by the distance to its nearest neighbour,
and `cluster` assigns each symbol to the nearest centroid.
