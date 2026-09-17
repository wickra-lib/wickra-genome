# Fuzzing Wickra Genome

[`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) harnesses for the parsing and stateful entry points of Wickra Genome. Fuzzing requires a nightly Rust toolchain; CI runs every target for 30 seconds on the family's pinned `nightly-2026-07-01`.

## Setup

```bash
cargo install cargo-fuzz
rustup toolchain install nightly-2026-07-01
```

The date is the family's fuzz nightly, pinned in `ci.yml`: a rolling `nightly`
regressed with a codegen ICE unrelated to this code, so every repository moves
the date together, on purpose.

## Targets

| Target | What it exercises |
| --- | --- |
| `spec_parse` | The spec-parsing surface: arbitrary bytes are parsed as a `GenomeSpec` (JSON). |
| `vector_build` | The build + query pipeline: arbitrary bytes drive a three-symbol universe of close prices under a fixed spec; `build` then every query runs over it. |
| `kmeans` | The seeded k-means surface: a fixed universe is clustered with a fuzz-derived `k` and `seed`. |
| `command_json` | The JSON command boundary: arbitrary bytes are handed to a valid genome's `command_json`. |

## Run

```bash
# From the repository root:
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu vector_build
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu kmeans
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu command_json
```

Each run continues until a crash is found or it is interrupted. A short
time-boxed smoke run is what CI does:

```bash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse -- -max_total_time=30
```

The expectation for every target is that it never panics: malformed or
adversarial input must surface as an `Err` or an in-band error, never a crash.
