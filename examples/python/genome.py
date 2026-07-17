"""A runnable Python example: build a genome over a tiny three-symbol universe
and print the nearest neighbour and the biggest outlier.

    pip install wickra-genome
    python examples/python/genome.py

Every language example runs the same request and prints the same summary — that
is the cross-language guarantee.
"""

import json

from wickra_genome import Genome

# A price-close genome over three symbols: AAA and BBB sit close together, CCC is
# far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly ranking.
SPEC = json.dumps(
    {
        "features": [{"kind": "price", "field": "close"}],
        "symbols": ["AAA", "BBB", "CCC"],
        "normalize": "z_score",
        "metric": "euclid",
        "seed": 24333,
    }
)

DATA = {
    "AAA": [{"time": 0, "open": 1, "high": 1, "low": 1, "close": 1, "volume": 0}],
    "BBB": [{"time": 0, "open": 2, "high": 2, "low": 2, "close": 2, "volume": 0}],
    "CCC": [{"time": 0, "open": 100, "high": 100, "low": 100, "close": 100, "volume": 0}],
}


def main() -> None:
    genome = Genome(SPEC)
    genome.command(json.dumps({"cmd": "build", "data": DATA}))

    version = json.loads(genome.command(json.dumps({"cmd": "version"})))
    similar = json.loads(genome.command(json.dumps({"cmd": "similar", "symbol": "AAA", "k": 2})))
    anomaly = json.loads(genome.command(json.dumps({"cmd": "anomaly"})))

    print(f"wickra-genome {version['version']}")
    print(f"AAA nearest: {similar['neighbors'][0]['symbol']}")
    print(f"top anomaly: {anomaly['anomalies'][0]['symbol']}")


if __name__ == "__main__":
    main()
