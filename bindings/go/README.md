<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/ci.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-genome)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-genome/bindings/go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/license.svg)](https://github.com/wickra-lib/wickra-genome#license)

# Wickra Genome — Go

---

**Part of the [Wickra ecosystem](#ecosystem): — for Go. `go get github.com/wickra-lib/wickra-genome/bindings/go` — over the C ABI via cgo, prebuilt library bundled in the module.**

Go bindings for the Wickra Genome vector engine over its C ABI hub (cgo). A
`Genome` is built from a spec JSON and driven with command JSONs, so the same
commands yield the byte-identical similarity, clustering and anomaly results as
every other Wickra Genome binding.

## Install

Use the published **`wickra-genome-go`** module, which bundles the prebuilt C ABI
library for every platform, so `go get` + `go build` works with no extra steps
(a C compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-genome/bindings/go
```

`wickra-genome-go` is generated from this directory by the release pipeline: it mirrors
the Go sources, the vendored C ABI header (`include/wickra_genome.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Linux/macOS the library path is baked
in via rpath; on Windows the DLL must be discoverable at run time (next to the
executable or on `PATH`).

### Requirements

The binding links the prebuilt C ABI library, staged per platform under
`lib/<goos>_<goarch>/`, with the header vendored under `include/`. Build the C
hub first and stage the library:

```bash
cargo build -p wickra-genome-c --release
# then copy target/release/libwickra_genome.{so,dylib} (or wickra_genome.dll)
# into bindings/go/lib/<goos>_<goarch>/
```

### Building from this repository (contributors)

```bash
go test ./...
```

## Quick start

```go
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-genome/bindings/go"
)

func main() {
	spec := `{"features":[{"kind":"price","field":"close"}],` +
		`"symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":24333}`
	g, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer g.Close()

	data := `{"AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],` +
		`"BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],` +
		`"CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}`
	g.Command(`{"cmd":"build","data":` + data + `}`)

	out, _ := g.Command(`{"cmd":"similar","symbol":"AAA","k":2}`)
	fmt.Println(out)
}
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of cgo over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-genome/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-genome>
- **Docs** (guides, spec reference, cookbook): <https://genome.wickra.org>
- **Runnable example:** [`examples/go/`](https://github.com/wickra-lib/wickra-genome/tree/main/examples/go)

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
