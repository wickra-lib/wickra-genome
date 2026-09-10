# wickra-genome WASM examples

Browser demos for the `wickra-genome-wasm` binding.

The WASM build carries the whole vector engine with `--no-default-features`: no
rayon, so the per-symbol fold is sequential rather than parallel — and
byte-for-byte identical to the parallel one, which is what the golden fixtures
pin down. A spec is data, not code, so the spec bytes on this page are the same
ones `examples/node/genome.js` sends.

## Build

The module ships as a `wasm-pack` `--target web` bundle. Build it once from the
repository root:

```bash
wasm-pack build bindings/wasm --target web --release
```

That writes `bindings/wasm/pkg/` with the `.wasm` binary, the JS loader and the
type declarations the page imports.

## Run

The page loads its module over `http://`, not `file://`, because ES module
imports and `WebAssembly.instantiateStreaming` both need a real origin. Serve the
repository root:

```bash
python -m http.server 8000
```

Then open `http://localhost:8000/examples/wasm/genome.html`.

## Pages

| Page | What it does |
|------|--------------|
| `genome.html` | Builds a genome over the shared three-symbol universe (`AAA` at 1, `BBB` at 2, `CCC` at 100), then renders `AAA`'s nearest neighbours and the anomaly ranking, plus the raw JSON. The page counterpart of `examples/node/genome.js`. |

## See also

- [examples/README.md](../README.md) — the same query in every other language.
- [bindings/wasm/README.md](../../bindings/wasm/README.md) — the binding itself.
