# Documentation

These pages are the guides that live beside the code, because they describe how
this repository behaves and have to change in the same commit the behaviour does.

| Page | What it answers |
|------|-----------------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | The engine, the `command_json` boundary and the determinism contract |
| [FEATURES.md](FEATURES.md) | The feature axes and the shape of a `GenomeSpec` |
| [FEEDS.md](FEEDS.md) | The side feeds: which indicator needs what, and why a spec is refused rather than answered with an axis that can never be ready |
| [NORMALIZATION.md](NORMALIZATION.md) | Z-score and min-max across the ready universe, per axis |
| [METRICS.md](METRICS.md) | Euclidean and cosine distance, and what each is good for |
| [CLUSTERING.md](CLUSTERING.md) | The seeded k-means, and what makes it reproducible |
| [STREAMING.md](STREAMING.md) | Feeding bar by bar against building in batch, and where the two are identical |
| [Cookbook.md](Cookbook.md) | Worked queries |

The API reference for each language is generated from the source rather than
committed here — `cargo doc` for Rust, the `.d.ts` beside the Node binding, the
docstrings in the Python module, the C header. Keeping a second copy in this
repository would drift from the code that generates it, and a reader opening
`docs/` would have no way to tell which copy was current.

The indicator library the genome resolves names through documents itself at
<https://docs.wickra.org>.

What stays here is what a generator cannot produce: the meaning of a field, the
reason a case is refused rather than answered, and the worked examples.

Elsewhere in the repository:

- [`../ARCHITECTURE.md`](../ARCHITECTURE.md) — the crate and binding layout
- [`../BENCHMARKS.md`](../BENCHMARKS.md) — what is measured and how
- [`../golden/README.md`](../golden/README.md) — the cross-language corpus and how to regenerate it
- [`../CONTRIBUTING.md`](../CONTRIBUTING.md) — how to build, test and propose a change
- [`../THREAT_MODEL.md`](../THREAT_MODEL.md) — what the genome does and does not touch
