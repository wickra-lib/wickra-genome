"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// reproduces the market genome byte-identically to the native run — the serial,
// single-threaded reductions in the browser sandbox match the native parallel
// run exactly. Skips cleanly when `pkg/` has not been built yet
// (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "wickra_genome_wasm.js"));
} catch {
  wasm = null;
}

const SPEC = JSON.stringify({
  features: [{ kind: "price", field: "close" }],
  symbols: ["AAA", "BBB", "CCC"],
  normalize: "z_score",
  metric: "euclid",
  seed: 24333,
});

const DATA = {
  AAA: [{ time: 0, open: 1, high: 1, low: 1, close: 1, volume: 0 }],
  BBB: [{ time: 0, open: 2, high: 2, low: 2, close: 2, volume: 0 }],
  CCC: [{ time: 0, open: 100, high: 100, low: 100, close: 100, volume: 0 }],
};

function built() {
  const g = new wasm.Genome(SPEC);
  g.command(JSON.stringify({ cmd: "build", data: DATA }));
  return g;
}

test("wasm build present or skipped", (t) => {
  if (!wasm) t.skip("run `wasm-pack build --target nodejs` first");
});

if (wasm) {
  test("wasm similar excludes self and finds the closest coin", () => {
    const res = JSON.parse(built().command(JSON.stringify({ cmd: "similar", symbol: "AAA", k: 2 })));
    const names = res.neighbors.map((n) => n.symbol);
    assert.ok(!names.includes("AAA"));
    assert.strictEqual(names[0], "BBB");
  });

  test("wasm anomaly ranks the outlier first", () => {
    const res = JSON.parse(built().command(JSON.stringify({ cmd: "anomaly" })));
    assert.strictEqual(res.anomalies[0].symbol, "CCC");
  });

  test("wasm run is byte-identical across instances", () => {
    const cmd = JSON.stringify({ cmd: "cluster", k: 2 });
    assert.strictEqual(built().command(cmd), built().command(cmd));
  });

  test("wasm version matches the module export", () => {
    assert.strictEqual(new wasm.Genome(SPEC).version(), wasm.version());
  });

  test("wasm throws on an invalid spec", () => {
    assert.throws(() => new wasm.Genome("{ not valid json"));
  });
}
