<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/ci.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-genome)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/pypi.svg)](https://pypi.org/project/wickra-genome/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/license.svg)](https://github.com/wickra-lib/wickra-genome#license)

# Wickra Genome — Python

---

**Part of the [Wickra ecosystem](#ecosystem): — for Python. `pip install wickra-genome` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for the Wickra Genome vector engine, built with PyO3 and
maturin. A `Genome` handle is driven over a JSON boundary, so the same commands
yield the byte-identical similarity, clustering and anomaly results as every
other Wickra Genome binding.

## Install

```bash
pip install wickra-genome
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

### Building from this repository (contributors)

```bash
maturin develop --release
```

## Quick start

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

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-genome/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-genome>
- **Docs** (guides, spec reference, cookbook): <https://genome.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-genome/tree/main/examples/python)

Wickra Genome ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-genome/blob/main/SECURITY.md>.

## Disclaimer

Wickra Genome is research and analytics software. Its similarity, clustering and
anomaly outputs are not investment advice, and nothing here is a recommendation
to trade. Use at your own risk.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-MIT) at your option.
