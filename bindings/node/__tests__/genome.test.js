"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Genome } = require("../index.js");

const SPEC = {
  features: [{ kind: "price", field: "close" }],
  symbols: ["AAA", "BBB", "CCC"],
  normalize: "z_score",
  metric: "euclid",
  seed: 24333,
};

const DATA = {
  AAA: [{ time: 0, open: 1, high: 1, low: 1, close: 1, volume: 0 }],
  BBB: [{ time: 0, open: 2, high: 2, low: 2, close: 2, volume: 0 }],
  CCC: [{ time: 0, open: 100, high: 100, low: 100, close: 100, volume: 0 }],
};

function built() {
  const g = new Genome(JSON.stringify(SPEC));
  const ok = JSON.parse(g.command(JSON.stringify({ cmd: "build", data: DATA })));
  assert.strictEqual(ok.ok, true);
  return g;
}

test("similar excludes self and finds the closest coin", () => {
  const res = JSON.parse(built().command(JSON.stringify({ cmd: "similar", symbol: "AAA", k: 2 })));
  const names = res.neighbors.map((n) => n.symbol);
  assert.ok(!names.includes("AAA"));
  assert.strictEqual(names[0], "BBB");
});

test("anomaly ranks the outlier first", () => {
  const res = JSON.parse(built().command(JSON.stringify({ cmd: "anomaly" })));
  assert.strictEqual(res.anomalies[0].symbol, "CCC");
});

test("deterministic across instances", () => {
  const cmd = JSON.stringify({ cmd: "cluster", k: 2 });
  assert.strictEqual(built().command(cmd), built().command(cmd));
});

test("version", () => {
  assert.strictEqual(typeof new Genome(JSON.stringify(SPEC)).version(), "string");
});
