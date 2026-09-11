# Feeds

Most indicators need only a candle. A large part of the registry needs something
else as well — a reference series, a derivatives tick, an order-book snapshot,
the trades that printed in the bar, or the market cross-section. This page is the
whole feed model: which indicator needs what, and how you supply it when building
in batch and when feeding bar by bar.

Related: [FEATURES.md](FEATURES.md) names a feature axis and its parameters;
[STREAMING.md](STREAMING.md) covers feeding against building in batch.

## The seven feed families

`wickra_genome_core::feed_kind(name)` reports which family an indicator belongs to.

| Family | What the indicator is given besides the bar | Example |
| --- | --- | --- |
| `Candle` | nothing | `Sma`, `Rsi`, `Atr` |
| `Pair` | the reference series' close at the same bar | `Beta`, `PearsonCorrelation`, `Cointegration` |
| `Derivatives` | a derivatives tick — funding, open interest, mark, index | `FundingRate` |
| `OrderBook` | an order-book snapshot | `Microprice` |
| `Trades` | the trades that printed within the bar | `CumulativeVolumeDelta` |
| `TradeQuote` | trades quoted against the book mid — needs **both** | `EffectiveSpread` |
| `CrossSection` | the market panel for that bar | `AdvanceDecline` |

A spec whose axis needs a feed the caller cannot supply is **refused**, naming
the indicator and the feed:

```
Microprice needs the books feed, which this build does not supply
```

That matters more here than a `NaN` column would elsewhere. An indicator whose
feed is absent resolves, ticks and returns nothing — every bar, without
complaint — so the axis is `None` for ever. A symbol with any `None` axis is
never *ready*, and the ready universe is what similarity search, clustering and
anomaly scoring iterate. One dead axis therefore takes the whole symbol out:
`similar()` reports a symbol plainly in the universe as unknown, `cluster()`
returns nothing and `anomaly()` returns nothing, with no error naming the cause.
Refusing the spec removes that failure mode entirely.

## Batch: parallel arrays beside the candles

In a batch build each symbol is either a bare candle array or an object carrying
its side feeds. Both forms may appear in the same dataset.

```jsonc
{
  "AAA": [ {"time": 1, "open": 10, "high": 11, "low": 9, "close": 10.5, "volume": 100} ],

  "BBB": {
    "candles":   [ /* the bars */ ],
    "reference": [ /* candles of the reference series, 1:1 with `candles` */ ],
    "derivs":    [ /* one DerivativesTick per bar */ ],
    "books":     [ /* one OrderBook per bar */ ],
    "trades":    [ [ /* the TradePrints of bar 0 */ ], [ /* bar 1 */ ] ],
    "sections":  [ /* one CrossSection per bar */ ]
  }
}
```

Every side array is optional and, when present, must be the same length as
`candles`. A mismatch is an error naming the feed and both lengths, rather than a
build that quietly reads past the end:

```
BBB: reference feed length 10 does not match 120 candles
```

`trades` is an array **of arrays**: a bar can carry any number of prints,
including none.

This is the shape `wickra-backtest`'s `RunRequest` uses, field for field, so a
dataset assembled for one is accepted by the other.

## Streaming: feeds on the bar

A `feed` command may carry the bar's feeds alongside its candle, in the per-bar
shape `wickra_backtest_core::StepFeeds` defines — reused verbatim, so the JSON
field names match across the ecosystem:

```jsonc
{
  "cmd": "feed",
  "symbol": "AAA",
  "candle": {"time": 1, "open": 10, "high": 11, "low": 9, "close": 10.5, "volume": 100},
  "feeds": {
    "reference":     50.25,          // the reference close at this bar
    "deriv":         { /* … */ },
    "orderbook":     { /* … */ },
    "trades":        [ /* … */ ],
    "cross_section": { /* … */ }
  }
}
```

The bar's feeds are checked against the spec before the fold, so a bar that omits
a feed the spec needs is refused by name rather than folded into an axis that can
never become ready.

`feeds` is optional. A `feed` without it is the candle-only case, which is what
every existing caller sends.

## Which feeds a spec needs

Only the axes decide. A spec of `Sma`, `Rsi` and a price field needs nothing
beyond candles and builds from a directory of CSV files. Add one `Microprice`
axis and the same dataset is refused until the books arrive — the CLI's `--data`
directory carries OHLCV only, so a fed build reads its dataset from `--stdin` in
the shape above.

## Warmup is unchanged

A feed does not shorten warmup. `RollingCorrelation(20)` still needs 20 bars of
both series before it produces anything, and a symbol stays unready until every
axis has a value. The feed changes whether the indicator can ever produce a
value, not when.
