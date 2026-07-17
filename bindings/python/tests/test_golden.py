"""Determinism: the same commands yield the byte-identical response string.

The full cross-language golden (asserting the response equals a blessed
golden/expected file) lands with the golden corpus in P-GEN-4; here we pin the
core invariant that the results are byte-reproducible from the inputs, which
every binding must preserve by forwarding the command string verbatim.
"""

import json

from wickra_genome import Genome

SPEC = {
    "features": [{"kind": "price", "field": "close"}],
    "symbols": ["AAA", "BBB", "CCC"],
    "normalize": "z_score",
    "metric": "euclid",
    "seed": 24333,
}

DATA = {
    "AAA": [{"time": 0, "open": 1, "high": 1, "low": 1, "close": 1, "volume": 0}],
    "BBB": [{"time": 0, "open": 2, "high": 2, "low": 2, "close": 2, "volume": 0}],
    "CCC": [{"time": 0, "open": 100, "high": 100, "low": 100, "close": 100, "volume": 0}],
}


def _run(cmd: dict) -> str:
    g = Genome(json.dumps(SPEC))
    g.command(json.dumps({"cmd": "build", "data": DATA}))
    return g.command(json.dumps(cmd))


def test_similar_is_reproducible() -> None:
    cmd = {"cmd": "similar", "symbol": "AAA", "k": 2}
    assert _run(cmd) == _run(cmd)


def test_anomaly_field_order_is_symbol_first() -> None:
    out = _run({"cmd": "anomaly"})
    assert out.startswith('{"anomalies":[{"symbol":')
