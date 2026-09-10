"use strict";

// Streaming equals batch, driven through the JSON command boundary.
//
// genome-core proves this in Rust, but that says nothing about the boundary this
// binding crosses. Every other test here only ever sends {"cmd":"build"}, so
// `feed` was exercised in no language at all: a binding that mis-serialised a
// candle on the feed path had no test to fail.
//
// The ramp is chosen so a candle applied twice would change Sma(3), which is
// what tells the two paths apart — a spec over the raw close reads the same
// however many times a bar arrived, which is how this class of bug stays
// invisible.

const { test } = require("node:test");
const assert = require("node:assert");
const { Genome } = require("../index.js");

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

test("streaming equals batch", () => {
  const batch = new Genome(SPEC);
  batch.command(JSON.stringify({ cmd: "build", data: DATA }));

  const streaming = new Genome(SPEC);
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

test("the batch vector is the mean of three bars", () => {
  const genome = new Genome(SPEC);
  genome.command(JSON.stringify({ cmd: "build", data: DATA }));
  const vector = JSON.parse(
    genome.command(JSON.stringify({ cmd: "vector", symbol: "AAA" })),
  );
  assert.strictEqual(vector.values[0], 20.0, "Sma(3) over 10, 20, 30 is 20");
});

test("reset returns the genome to its pre-feed state", () => {
  const genome = new Genome(SPEC);
  const query = JSON.stringify({ cmd: "cluster", k: 2 });
  const empty = genome.command(query);

  feedAll(genome);
  assert.notStrictEqual(
    genome.command(query),
    empty,
    "feeding the whole universe changed nothing",
  );

  genome.command(JSON.stringify({ cmd: "reset" }));
  assert.strictEqual(
    genome.command(query),
    empty,
    "reset did not return the genome to its pre-feed state",
  );
});
