# Wickra Genome — C ABI

The C ABI hub for Wickra Genome. It builds as a `cdylib` and a `staticlib` and
exposes a tiny JSON-over-C surface that every C-capable language (C, C++, C#, Go,
Java, R) links against. The whole vector engine lives in the Rust core; this
layer only marshals JSON strings across the boundary, so a fixed seed yields the
byte-identical similarity, clustering and anomaly results in every language.

## Build

```bash
cargo build -p wickra-genome-c --release
```

This produces `wickra_genome.{dll,so,dylib}` (and a static library) under
`target/release/`. The header is committed at
[`include/wickra_genome.h`](include/wickra_genome.h) and regenerated with:

```bash
cbindgen --config cbindgen.toml --crate wickra-genome-c --output include/wickra_genome.h
```

## Surface

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

## Example

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

## License

Dual-licensed under either of Apache-2.0 or MIT at your option.
