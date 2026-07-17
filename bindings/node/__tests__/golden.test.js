"use strict";

// Determinism: the same commands yield the byte-identical response string. The
// full cross-language golden (asserting the response equals a blessed
// golden/expected file) lands with the golden corpus in P-GEN-4; here we pin the
// core invariant that the results are byte-reproducible, which every binding
// must preserve by forwarding the command string verbatim. When the golden
// corpus is present, every fixture is checked against the shared expected files.

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

// The cross-language golden lands in P-GEN-4; when the corpus exists, verify the
// Node binding reproduces every blessed fixture byte-for-byte.
if (fs.existsSync(path.join(GOLDEN, "specs"))) {
  for (const specFile of fs.readdirSync(path.join(GOLDEN, "specs")).sort()) {
    const name = path.basename(specFile, ".json");
    test(`cross-language golden: ${name}`, () => {
      const spec = fs.readFileSync(path.join(GOLDEN, "specs", specFile), "utf8");
      const cmds = JSON.parse(fs.readFileSync(path.join(GOLDEN, "cmds", `${name}.json`), "utf8"));
      const expected = fs
        .readFileSync(path.join(GOLDEN, "expected", `${name}.json`), "utf8")
        .trimEnd();
      const g = new Genome(spec);
      const got = cmds.map((c) => g.command(JSON.stringify(c))).join("\n");
      assert.strictEqual(got, expected, `golden mismatch for ${name}`);
    });
  }
}
