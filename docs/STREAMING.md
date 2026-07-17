# Streaming vs batch

Genome ingests a universe two ways, and they are equivalent.

- **Batch** — `build` folds a whole `{symbol: [candles]}` map at once. This is
  what the CLI's `--data` directory and most examples use.
- **Streaming** — `feed` folds one candle into one symbol at a time (or
  `feed_batch` for a symbol's history). This is the live path: as each new bar
  arrives, feed it and re-query.

## The equivalence

Feeding a universe candle-by-candle produces a genome whose every query response
is **byte-identical** to one built in a single `build` over the same data. The
reason is structural: the streaming indicators are O(1) folds that carry all their
state forward, and the cross-section queries read only each symbol's *final*
state — not the order in which bars arrived. So the ingestion path cannot change
the answer.

This is pinned by the `streaming_eq_batch` test: for `vector`, `similar`,
`cluster` and `anomaly`, the streamed genome and the batch genome return the same
string.

```rust
use genome_core::{build, Genome};

// batch
let mut batch = build(&data, &spec)?;

// streaming — the same data, one candle at a time
let mut streamed = Genome::with_spec(spec);
for (symbol, candles) in &data {
    for candle in candles {
        streamed.feed(symbol, candle)?;
    }
}

assert_eq!(
    batch.command_json(r#"{"cmd":"anomaly"}"#),
    streamed.command_json(r#"{"cmd":"anomaly"}"#),
);
```

## Live use

Behind the `live` feature the engine can consume a `wickra-exchange` market feed
directly: feed each incoming candle, then re-run `similar` / `cluster` / `anomaly`
to track how the market's DNA is shifting in real time. The determinism contract
still holds — the same feed replayed yields the same answers.
