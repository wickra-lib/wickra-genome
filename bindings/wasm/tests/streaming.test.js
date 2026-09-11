"use strict";

// Streaming equals batch, through the WASM boundary.
//
// wickra-genome-core proves this in Rust, and the golden test here proves the WASM
// build reproduces a *batch* run byte-identically. Neither says anything about
// `feed`: every other check only ever sends {"cmd":"build"}, so the streaming
// path crossed this boundary untested.
//
// The ramp is chosen so a candle applied twice would change Sma(3), which is
// what tells the two paths apart — a spec over the raw close reads the same
// however many times a bar arrived, which is how this class of bug stays
// invisible.
//
// Skips cleanly when `pkg/` has not been built yet
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
  features: [
    { kind: "indicator", name: "Sma", params: [3] },
    { kind: "price", field: "close" },
  ],
  symbols: ["AAA", "BBB"],
  normalize: "z_score",
  metric: "euclid",
  seed: 24333,
});

const candle = (time, close) => ({
  time,
  open: close,
  high: close,
  low: close,
  close,
  volume: 1.0,
});

const DATA = {
  AAA: [10.0, 20.0, 30.0].map((c, i) => candle(i + 1, c)),
  BBB: [40.0, 50.0, 60.0].map((c, i) => candle(i + 1, c)),
};

function feedAll(genome) {
  for (const symbol of Object.keys(DATA).sort()) {
    for (const bar of DATA[symbol]) {
      genome.command(JSON.stringify({ cmd: "feed", symbol, candle: bar }));
    }
  }
}

test("streaming equals batch through WASM", { skip: wasm === null }, () => {
  const batch = new wasm.Genome(SPEC);
  batch.command(JSON.stringify({ cmd: "build", data: DATA }));

  const streaming = new wasm.Genome(SPEC);
  feedAll(streaming);

  const queries = [
    { cmd: "vector", symbol: "AAA" },
    { cmd: "similar", symbol: "AAA", k: 1 },
    { cmd: "cluster", k: 2 },
    { cmd: "anomaly" },
  ];
  for (const query of queries) {
    const cmd = JSON.stringify(query);
    assert.strictEqual(
      streaming.command(cmd),
      batch.command(cmd),
      `streaming != batch for ${query.cmd}`,
    );
  }
});

test("the batch vector is the mean of three bars", { skip: wasm === null }, () => {
  const genome = new wasm.Genome(SPEC);
  genome.command(JSON.stringify({ cmd: "build", data: DATA }));
  const vector = JSON.parse(
    genome.command(JSON.stringify({ cmd: "vector", symbol: "AAA" })),
  );
  assert.strictEqual(vector.values[0], 20.0, "Sma(3) over 10, 20, 30 is 20");
});
