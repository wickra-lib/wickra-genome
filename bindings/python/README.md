# Wickra Genome — Python

Python bindings for the Wickra Genome vector engine, built with PyO3 and
maturin. A `Genome` handle is driven over a JSON boundary, so the same commands
yield the byte-identical similarity, clustering and anomaly results as every
other Wickra Genome binding.

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/wickra-lib/wickra-genome#license)
[![PyO3](https://img.shields.io/badge/bindings-PyO3-3b82f6)](https://pyo3.rs)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

## Install

```bash
pip install wickra-genome
```

## Build from source

```bash
maturin develop --release
```

## Usage

```python
import json
from wickra_genome import Genome

spec = {
    "features": [{"kind": "price", "field": "close"}],
    "symbols": ["AAA", "BBB", "CCC"],
    "normalize": "z_score",
    "metric": "euclid",
    "seed": 24333,
}
g = Genome(json.dumps(spec))

data = {
    "AAA": [{"time": 0, "open": 1, "high": 1, "low": 1, "close": 1, "volume": 0}],
    "BBB": [{"time": 0, "open": 2, "high": 2, "low": 2, "close": 2, "volume": 0}],
    "CCC": [{"time": 0, "open": 100, "high": 100, "low": 100, "close": 100, "volume": 0}],
}
g.command(json.dumps({"cmd": "build", "data": data}))

print(g.command(json.dumps({"cmd": "similar", "symbol": "AAA", "k": 2})))
print(g.command(json.dumps({"cmd": "anomaly"})))
```

The command protocol (`build`, `feed`, `vector`, `similar`, `cluster`,
`anomaly`, `version`) is identical across every binding; only the Rust core
computes, so a fixed seed gives the byte-identical clustering everywhere.

## License

Dual-licensed under either of Apache-2.0 or MIT at your option.
