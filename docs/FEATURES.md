# Features

A feature is one axis of every symbol's vector. The `features` array in a
[`GenomeSpec`](../README.md#genomespec-and-the-queries) fixes the axes and their
order; that order is the order of the coordinates in every vector.

## Feature kinds

A feature is tagged by `kind`:

```json
{ "kind": "indicator", "name": "Rsi", "params": [14] }
{ "kind": "indicator", "name": "Macd", "params": [12, 26, 9], "field": "hist" }
{ "kind": "price", "field": "close" }
```

- **`indicator`** — a `wickra-core` streaming indicator, resolved by `name` and
  `params` through the `wickra-backtest-core` registry factory. The optional
  `field` selects a named sub-output on a multi-output indicator (e.g. `"hist"` on
  MACD); omit it for the indicator's primary value.
- **`price`** — a raw OHLCV field read from the current candle: `open`, `high`,
  `low`, `close` or `volume`.

An unknown indicator `name` is a build error, surfaced in band.

## Feature keys

Every axis has a canonical, self-describing key, returned by the `vector` query:

| Feature                                              | Key                 |
|------------------------------------------------------|---------------------|
| `{"kind":"price","field":"close"}`                   | `price.close`       |
| `{"kind":"indicator","name":"Rsi","params":[14]}`    | `Rsi(14)`           |
| `{"kind":"indicator","name":"Macd",...,"field":"hist"}` | `Macd(12,26,9).hist` |

Parameters render with `{}`, so `14.0` becomes `14` and `2.5` stays `2.5`.

## Warmup and readiness

Each indicator has its own warmup. A symbol is *ready* only once every axis is
past warmup and finite; the cross-section queries (`similar`, `cluster`,
`anomaly`) operate on the ready symbols. A `vector` for a not-yet-ready symbol
reports `ready: false` with `null` on the unfilled axes.

## The full spec

```json
{
  "features": [ ... ],
  "symbols": ["AAA", "BBB", "CCC"],
  "normalize": "z_score",
  "metric": "euclid",
  "seed": 24333,
  "timeframe": "1h"
}
```

`features` and `symbols` must be non-empty (a validation error otherwise).
`normalize` defaults to `z_score`, `metric` to `cosine`, `seed` to a fixed
default, and `timeframe` is advisory metadata.
