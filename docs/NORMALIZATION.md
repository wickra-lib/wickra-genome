# Normalization

Raw feature values live on wildly different scales — an RSI is `0..100`, a price
is hundreds, a rate of change is a small signed fraction. Taking a distance over
raw axes would let the largest-scale axis dominate. So every axis is normalized
**across the cross-section** — over the ready symbols, per column — before any
distance is taken.

Normalization is chosen per spec by `normalize`.

## `z_score` (default)

For each axis, over the ready symbols:

```
mean_j = (1/n) · Σ_i x_ij
std_j  = sqrt( (1/n) · Σ_i (x_ij − mean_j)²  )
z_ij   = (x_ij − mean_j) / std_j
```

A zero-variance axis (every symbol equal) yields `0` for that axis rather than a
division by zero.

## `min_max`

For each axis, over the ready symbols:

```
min_j = min_i x_ij
max_j = max_i x_ij
m_ij  = (x_ij − min_j) / (max_j − min_j)
```

mapping each axis into `[0, 1]`. A zero-range axis (`max == min`) yields `0`.

## Order and determinism

The reductions run serially over the symbols in `BTreeMap` key order, so the
mean, variance, min and max are summed in a fixed order and the `f64` result is
identical across languages and thread counts. Variance is computed as
`Σx²/n − mean²`; because of floating-point cancellation, downstream assertions use
a `1e-9` tolerance, not `1e-12`.

The normalized matrix is what the [metric](METRICS.md) and
[clustering](CLUSTERING.md) then operate on.
