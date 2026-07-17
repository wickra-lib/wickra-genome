"use strict";

// Determinism plus the cross-language golden. The first tests pin that the same
// commands yield a byte-identical response string (every binding must preserve
// this by forwarding the command string verbatim). When the shared golden corpus
// is present, the Node binding is verified against every blessed fixture
// byte-for-byte — this is the cross-language proof that seeded k-means and the
// whole vector pipeline agree across all ten languages.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Genome } = require("../index.js");

const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");

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

function run(cmd) {
  const g = new Genome(JSON.stringify(SPEC));
  g.command(JSON.stringify({ cmd: "build", data: DATA }));
  return g.command(JSON.stringify(cmd));
}

test("the same commands yield the byte-identical response string", () => {
  const cmd = { cmd: "similar", symbol: "AAA", k: 2 };
  assert.strictEqual(run(cmd), run(cmd));
});

test("anomaly field order is symbol-first", () => {
  assert.ok(run({ cmd: "anomaly" }).startsWith('{"anomalies":[{"symbol":'));
});

// Parse an OHLCV CSV (`ts,open,high,low,close,volume`, header skipped) into the
// candle-object array the `build` command consumes.
function parseCsv(text) {
  const candles = [];
  text.split(/\r?\n/).forEach((line, idx) => {
    const row = line.trim();
    if (row === "") return;
    const c = row.split(",").map((x) => x.trim());
    if (!/^-?\d+$/.test(c[0])) {
      if (idx === 0) return; // header
      throw new Error(`bad timestamp: ${c[0]}`);
    }
    candles.push({
      time: Number(c[0]),
      open: Number(c[1]),
      high: Number(c[2]),
      low: Number(c[3]),
      close: Number(c[4]),
      volume: Number(c[5]),
    });
  });
  return candles;
}

// Load the shared `golden/data/` universe (one `<SYMBOL>.csv` per symbol).
function loadUniverse() {
  const dir = path.join(GOLDEN, "data");
  const data = {};
  for (const file of fs.readdirSync(dir)) {
    if (!file.endsWith(".csv")) continue;
    const sym = path.basename(file, ".csv");
    data[sym] = parseCsv(fs.readFileSync(path.join(dir, file), "utf8"));
  }
  return data;
}

// The blessed op matrix per spec (mirrors golden/README.md): `cluster5` clusters
// at k=5, the rest at k=3; `vector`/`similar` query `AAA`.
function opsFor(name) {
  const ck = name === "cluster5" ? 5 : 3;
  return [
    ["vector", { cmd: "vector", symbol: "AAA" }],
    ["similar", { cmd: "similar", symbol: "AAA", k: 3 }],
    ["cluster", { cmd: "cluster", k: ck }],
    ["anomaly", { cmd: "anomaly" }],
  ];
}

// The cross-language golden: every spec, run against the shared universe, must
// reproduce each op's blessed `golden/expected/<op>/<name>.json` byte-for-byte.
if (fs.existsSync(path.join(GOLDEN, "specs"))) {
  const universe = loadUniverse();
  const buildCmd = JSON.stringify({ cmd: "build", data: universe });
  for (const specFile of fs.readdirSync(path.join(GOLDEN, "specs")).sort()) {
    if (!specFile.endsWith(".json")) continue;
    const name = path.basename(specFile, ".json");
    const spec = fs.readFileSync(path.join(GOLDEN, "specs", specFile), "utf8");
    for (const [op, cmd] of opsFor(name)) {
      test(`cross-language golden: ${op}/${name}`, () => {
        const g = new Genome(spec);
        g.command(buildCmd);
        const got = g.command(JSON.stringify(cmd));
        const expected = fs
          .readFileSync(path.join(GOLDEN, "expected", op, `${name}.json`), "utf8")
          .trimEnd();
        assert.strictEqual(got, expected, `golden mismatch for ${op}/${name}`);
      });
    }
  }
}
