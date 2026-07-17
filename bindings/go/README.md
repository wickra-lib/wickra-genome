# Wickra Genome — Go

Go bindings for the Wickra Genome vector engine over its C ABI hub (cgo). A
`Genome` is built from a spec JSON and driven with command JSONs, so the same
commands yield the byte-identical similarity, clustering and anomaly results as
every other Wickra Genome binding.

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/wickra-lib/wickra-genome#license)
[![cgo](https://img.shields.io/badge/bindings-cgo-3b82f6)](https://pkg.go.dev/cmd/cgo)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

## Requirements

The binding links the prebuilt C ABI library, staged per platform under
`lib/<goos>_<goarch>/`, with the header vendored under `include/`. Build the C
hub first and stage the library:

```bash
cargo build -p wickra-genome-c --release
# then copy target/release/libwickra_genome.{so,dylib} (or wickra_genome.dll)
# into bindings/go/lib/<goos>_<goarch>/
```

## Usage

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

## Test

```bash
go test ./...
```

## License

Dual-licensed under either of Apache-2.0 or MIT at your option.
