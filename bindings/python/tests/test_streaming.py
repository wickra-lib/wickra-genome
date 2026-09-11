"""Streaming equals batch, driven through the JSON command boundary.

``wickra-genome-core`` proves this in Rust
(``crates/genome-core/tests/streaming_eq_batch.rs``), but that says nothing about
the boundary each language actually crosses. A binding reaches the core through
``command``, and every other test here only ever sends ``{"cmd":"build"}`` — so
``feed`` was exercised in no language at all. A binding that mis-serialised a
candle on the feed path, or dropped a field from the envelope, had no test to
fail.

Batch: one ``build`` over the whole dataset.
Streaming: ``feed`` each candle of each symbol in turn.
Both are then asked the same query, and both return the core's compact JSON
verbatim, so byte equality is the check.
"""

import json

from wickra_genome import Genome

SPEC = json.dumps(
    {
        "features": [
            {"kind": "indicator", "name": "Sma", "params": [3]},
            {"kind": "price", "field": "close"},
        ],
        "symbols": ["AAA", "BBB"],
        "normalize": "z_score",
        "metric": "euclid",
        "seed": 24333,
    }
)


def _candle(time: int, close: float) -> dict:
    return {
        "time": time,
        "open": close,
        "high": close,
        "low": close,
        "close": close,
        "volume": 1.0,
    }


# A rising ramp per symbol so Sma(3) is well defined, and so a candle applied
# twice would produce a different mean — which is what tells the two apart.
DATA = {
    "AAA": [_candle(i + 1, c) for i, c in enumerate((10.0, 20.0, 30.0))],
    "BBB": [_candle(i + 1, c) for i, c in enumerate((40.0, 50.0, 60.0))],
}


def _feed_all(genome: Genome) -> None:
    for symbol in sorted(DATA):
        for candle in DATA[symbol]:
            genome.command(
                json.dumps({"cmd": "feed", "symbol": symbol, "candle": candle})
            )


def test_streaming_equals_batch() -> None:
    batch = Genome(SPEC)
    batch.command(json.dumps({"cmd": "build", "data": DATA}))

    streaming = Genome(SPEC)
    _feed_all(streaming)

    for query in (
        {"cmd": "vector", "symbol": "AAA"},
        {"cmd": "similar", "symbol": "AAA", "k": 1},
        {"cmd": "cluster", "k": 2},
        {"cmd": "anomaly"},
    ):
        cmd = json.dumps(query)
        assert streaming.command(cmd) == batch.command(cmd), (
            f"streaming != batch for {query['cmd']}"
        )


def test_the_batch_vector_is_the_mean_of_three_bars() -> None:
    """A candle applied twice would give a different Sma(3); this pins which."""
    genome = Genome(SPEC)
    genome.command(json.dumps({"cmd": "build", "data": DATA}))
    vector = json.loads(genome.command(json.dumps({"cmd": "vector", "symbol": "AAA"})))
    assert vector["values"][0] == 20.0, "Sma(3) over 10, 20, 30 is 20"


def test_reset_returns_to_the_pre_feed_state() -> None:
    genome = Genome(SPEC)
    empty = genome.command(json.dumps({"cmd": "cluster", "k": 2}))

    _feed_all(genome)
    assert genome.command(json.dumps({"cmd": "cluster", "k": 2})) != empty, (
        "feeding the whole universe changed nothing"
    )

    genome.command(json.dumps({"cmd": "reset"}))
    assert genome.command(json.dumps({"cmd": "cluster", "k": 2})) == empty, (
        "reset did not return the genome to its pre-feed state"
    )
