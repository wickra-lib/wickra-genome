"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Genome } = require("../index.js");

const SPEC = JSON.stringify({
  features: [{ kind: "price", field: "close" }],
  symbols: ["AAA"],
});

test("the Genome surface exposes command and version", () => {
  const genome = new Genome(SPEC);
  assert.strictEqual(typeof genome.command, "function");
  assert.strictEqual(typeof genome.version, "function");
});
