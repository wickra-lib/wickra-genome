# Threat model

Wickra Genome is a data-processing library. It reads recorded market data and a
JSON spec and emits similarity, cluster and anomaly reports. The asset worth
protecting is the **integrity and determinism** of those reports; there are no
credentials, no funds and no order placement anywhere in the system.

## Actors

- **Operator** — runs the genome over trusted or semi-trusted inputs (candle
  series and a spec).
- **Untrusted input** — the spec JSON and the market data an operator feeds in;
  the primary attack surface.

## Threats and mitigations

- **Malformed input** — every spec and every data bundle is parsed by serde with
  strict typing; a malformed input is a clean `Err`, never a panic. Fuzz targets
  exercise the parse and run surfaces.
- **Resource exhaustion** — the vector build is `O(1)` per bar per indicator and
  bounded by the input size; there is no unbounded recursion or allocation driven
  by input values.
- **Non-determinism** — the whole product is a determinism guarantee. Any
  reduction that could reorder (nearest-neighbour ties, cluster assignment) is
  broken deterministically, the PRNG is a fixed self-written stream (no `rand`),
  and every serialised number is rounded onto a fixed grid, so the report is
  byte-identical across runs, platforms and language bindings. The golden corpus
  pins this.
- **Supply chain** — dependencies are SHA/version-pinned, audited by `cargo-deny`
  and OSV, and CI runs CodeQL, Scorecard and zizmor.

## Out of scope

No secrets, no network egress in the core, no order execution. Live market feeds,
when added, arrive through the separately-audited `wickra-exchange`.
