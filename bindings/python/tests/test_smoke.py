"""Smoke test: build a genome, query similarity / clustering / anomaly."""

import json

from wickra_genome import Genome, __version__

SPEC = {
    "features": [{"kind": "price", "field": "close"}],
    "symbols": ["AAA", "BBB", "CCC"],
    "normalize": "z_score",
    "metric": "euclid",
    "seed": 24333,
}


def _candles(closes: list[float]) -> list[dict]:
    return [
        {"time": i, "open": c, "high": c, "low": c, "close": c, "volume": 0}
        for i, c in enumerate(closes)
    ]


DATA = {
    "AAA": _candles([1.0]),
    "BBB": _candles([2.0]),
    "CCC": _candles([100.0]),
}


def _built() -> Genome:
    g = Genome(json.dumps(SPEC))
    ok = json.loads(g.command(json.dumps({"cmd": "build", "data": DATA})))
    assert ok["ok"] is True
    return g


def test_similar_excludes_self() -> None:
    g = _built()
    res = json.loads(g.command(json.dumps({"cmd": "similar", "symbol": "AAA", "k": 2})))
    names = [n["symbol"] for n in res["neighbors"]]
    assert "AAA" not in names
    assert names[0] == "BBB"  # the closest coin


def test_anomaly_ranks_the_outlier_first() -> None:
    g = _built()
    res = json.loads(g.command(json.dumps({"cmd": "anomaly"})))
    assert res["anomalies"][0]["symbol"] == "CCC"


def test_cluster_partitions_the_universe() -> None:
    g = _built()
    res = json.loads(g.command(json.dumps({"cmd": "cluster", "k": 2})))
    members = sorted(sorted(c["members"]) for c in res["clusters"])
    assert sum(len(m) for m in members) == 3


def test_deterministic_across_instances() -> None:
    a = _built().command(json.dumps({"cmd": "cluster", "k": 2}))
    b = _built().command(json.dumps({"cmd": "cluster", "k": 2}))
    assert a == b


def test_version() -> None:
    assert Genome.version() == __version__
