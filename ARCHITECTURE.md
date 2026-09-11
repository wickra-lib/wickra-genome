# Architecture

Wickra Genome is a data-driven library that turns the market into a searchable
vector space.

## Workspace

| Crate          | Role |
|----------------|------|
| `wickra-genome-core`  | The library: the feature-vector builder, the vector space, and similarity / clustering / anomaly queries, exposed over the `command_json` boundary. |
| `wickra-genome`   | `wickra-genome`, the reference CLI. |
| `genome-bench` | Criterion benchmarks. |
| `bindings/*`   | The ten language bindings (Python, Node.js, WASM native; C, C++, C#, Go, Java, R over the C ABI hub). |

## The vector space

Each asset is embedded as a vector whose axes the spec names: `features.len()` is
the dimension, and each axis is one of the 497 indicators the shared registry
resolves by name, evaluated on the asset's candles at `O(1)` per bar. That is the
same registry the rest of the Wickra ecosystem uses, so a coordinate means exactly
what it means everywhere else. Over the cross-section of embedded assets,
`wickra-genome-core` answers three questions:

- **Similarity** — nearest neighbours to a query asset under a chosen metric.
- **Clustering** — a deterministic k-means partition of the market into regimes.
- **Anomaly** — assets whose vector is far from any cluster centroid.

## The command surface

The whole library is reachable as data over a JSON boundary
(`command_json` → response JSON), exposed through the C ABI and wrapped by every
binding, which forwards the command string verbatim. Because the engine lives once
in the Rust core, uses a fixed self-written PRNG (no `rand`), breaks every tie
deterministically, and rounds every serialised number onto a fixed grid, the
report is **byte-identical** across the CLI and all ten languages — the property
the golden corpus pins.
