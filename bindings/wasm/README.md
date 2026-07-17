# Wickra Genome — WASM

WebAssembly bindings for the Wickra Genome vector engine, compiled from Rust with
[wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A `Genome` is built
from a spec JSON and driven by command JSONs over a JSON boundary, so a browser
front-end runs against the exact same core as every other Wickra Genome binding.

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/wickra-lib/wickra-genome#license)
[![wasm-bindgen](https://img.shields.io/badge/bindings-wasm--bindgen-3b82f6)](https://wasm-bindgen.github.io/wasm-bindgen/)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

## Build

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Usage

```js
import init, { Genome } from "./pkg/wickra_genome_wasm.js";

await init();

const spec = JSON.stringify({
  features: [{ kind: "price", field: "close" }],
  symbols: ["AAA", "BBB", "CCC"],
  normalize: "z_score",
  metric: "euclid",
  seed: 24333,
});
const g = new Genome(spec);

const data = {
  AAA: [{ time: 0, open: 1, high: 1, low: 1, close: 1, volume: 0 }],
  BBB: [{ time: 0, open: 2, high: 2, low: 2, close: 2, volume: 0 }],
  CCC: [{ time: 0, open: 100, high: 100, low: 100, close: 100, volume: 0 }],
};
g.command(JSON.stringify({ cmd: "build", data }));

console.log(g.command(JSON.stringify({ cmd: "similar", symbol: "AAA", k: 2 })));
```

The engine runs single-threaded in the browser, and because every cross-section
reduction runs serially in key order, the result is byte-identical to the native
parallel run — the exact cross-language golden check.

## License

Dual-licensed under either of Apache-2.0 or MIT at your option.
