// A runnable Node.js example: build a genome over a tiny three-symbol universe
// and print the nearest neighbour and the biggest outlier.
//
//   npm install
//   node examples/node/genome.js
//
// Every language example runs the same request and prints the same summary —
// that is the cross-language guarantee.
"use strict";

const { Genome } = require("wickra-genome");

// A price-close genome over three symbols: AAA and BBB sit close together, CCC is
// far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly ranking.
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

const genome = new Genome(SPEC);
genome.command(JSON.stringify({ cmd: "build", data: DATA }));

const version = JSON.parse(genome.command(JSON.stringify({ cmd: "version" })));
const similar = JSON.parse(
  genome.command(JSON.stringify({ cmd: "similar", symbol: "AAA", k: 2 })),
);
const anomaly = JSON.parse(genome.command(JSON.stringify({ cmd: "anomaly" })));

console.log(`wickra-genome ${version.version}`);
console.log(`AAA nearest: ${similar.neighbors[0].symbol}`);
console.log(`top anomaly: ${anomaly.anomalies[0].symbol}`);
