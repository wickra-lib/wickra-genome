<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/ci.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-genome)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/release.svg)](https://github.com/wickra-lib/wickra-genome/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/license.svg)](https://github.com/wickra-lib/wickra-genome#license)

# Wickra Genome — C / C++

---

**Part of the [Wickra ecosystem](#ecosystem): — for C / C++. `cargo build -p wickra-genome-c --release` — a prebuilt shared/static library plus a generated `wickra_genome.h`, no system dependencies.**

The C ABI hub for Wickra Genome. It builds as a `cdylib` and a `staticlib` and
exposes a tiny JSON-over-C surface that every C-capable language (C, C++, C#, Go,
Java, R) links against. The whole vector engine lives in the Rust core; this
layer only marshals JSON strings across the boundary, so a fixed seed yields the
byte-identical similarity, clustering and anomaly results in every language.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-genome/releases) — each archive
has `wickra_genome.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-genome-c --release
# -> target/release/libwickra_genome.{so,dylib} or wickra_genome.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

### Building from this repository (contributors)

```bash
cargo build -p wickra-genome-c --release
```

This produces `wickra_genome.{dll,so,dylib}` (and a static library) under
`target/release/`. The header is committed at
[`include/wickra_genome.h`](https://github.com/wickra-lib/wickra-genome/blob/main/bindings/c/include/wickra_genome.h) and regenerated with:

```bash
cbindgen --config cbindgen.toml --crate wickra-genome-c --output include/wickra_genome.h
```

## Quick start

```c
#include "wickra_genome.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    WickraGenome *g = wickra_genome_new(
        "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
        "\"symbols\":[\"AAA\"]}");
    const char *cmd = "{\"cmd\":\"version\"}";
    int32_t len = wickra_genome_command(g, cmd, NULL, 0);
    char *buf = malloc((size_t)len + 1);
    wickra_genome_command(g, cmd, buf, (size_t)len + 1);
    printf("%s\n", buf);
    free(buf);
    wickra_genome_free(g);
    return 0;
}
```

### Surface

```c
typedef struct WickraGenome WickraGenome;

WickraGenome *wickra_genome_new(const char *spec_json);   /* NULL on an invalid spec */
void          wickra_genome_free(WickraGenome *handle);   /* NULL-safe */
int32_t       wickra_genome_command(WickraGenome *handle, const char *cmd_json,
                                    char *out, uintptr_t cap);
const char   *wickra_genome_version(void);                /* static NUL string */
```

- `wickra_genome_new` takes a spec JSON (a non-empty `features` and `symbols`);
  it returns `NULL` on a null / non-UTF-8 / invalid spec.
- `wickra_genome_command` applies a command envelope (`{"cmd":"...", ...}` —
  `set_spec`, `feed`, `feed_batch`, `build`, `vector`, `similar`, `cluster`,
  `anomaly`, `reset`, `version`) and uses the classic two-call length-out
  protocol: call with `out = NULL`, `cap = 0` to learn the response length, then
  allocate `len + 1` and call again. A negative return is an unusable-argument
  or caught-panic error code; domain errors (bad spec, unknown symbol) come back
  in-band as `{"ok":false,"error":...}` JSON.
- `wickra_genome_version` returns a static NUL-terminated version string.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-genome/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-genome>
- **Docs** (guides, spec reference, cookbook): <https://genome.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-genome/tree/main/examples/c)

Wickra Genome ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-genome/blob/main/SECURITY.md>.

## Disclaimer

Wickra Genome is research and analytics software. Its similarity, clustering and
anomaly outputs are not investment advice, and nothing here is a recommendation
to trade. Use at your own risk.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-MIT) at your option.
