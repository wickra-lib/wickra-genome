# Examples

Runnable examples in every Wickra Genome language. Each one builds a genome over
the same tiny three-symbol universe — `AAA` and `BBB` close together, `CCC` far
away — and prints the same summary:

```
wickra-genome 0.1.0
AAA nearest: BBB
top anomaly: CCC
```

That identical output across ten languages is the cross-language guarantee: the
vector engine (feature extraction, cross-section normalization, distance metric
and seeded k-means) lives once in the Rust core and every binding forwards the
command JSON verbatim. See [`golden/README.md`](../golden/README.md) for the
byte-golden corpus these results are pinned against.

The canonical spec and a larger universe are also in [`data/`](data/) for the
CLI:

```bash
cargo run -p genome-cli -- --spec examples/data/specs/dna.json \
  --data examples/data/universe --op similar --symbol AAA --k 3
```

## Per language

- **Rust** — [`rust/`](rust/): `cargo run --manifest-path examples/rust/Cargo.toml`
- **Python** — [`python/genome.py`](python/genome.py): `pip install wickra-genome && python examples/python/genome.py`
- **Node.js** — [`node/`](node/): `cd examples/node && npm install && node genome.js`
- **Go** — [`go/`](go/): `go run examples/go/genome.go` (with the C ABI library staged, see the Go binding README)
- **C#** — [`csharp/Genome/`](csharp/Genome/): `dotnet run --project examples/csharp/Genome`
- **Java** — [`java/`](java/): `mvn -q compile exec:java -Dexec.mainClass=org.wickra.genome.examples.Genome`
- **R** — [`r/genome.R`](r/genome.R): `R CMD INSTALL bindings/r && Rscript examples/r/genome.R`
- **C / C++** — [`c/`](c/): build the C ABI, then CMake + ctest:

  ```bash
  cargo build --release -p wickra-genome-c
  cmake -S examples/c -B examples/c/build
  cmake --build examples/c/build --config Release
  ctest --test-dir examples/c/build -C Release --output-on-failure
  ```

The binding examples install the published `wickra-genome` package for their
language; the Rust and C/C++ examples build against the in-repo core.
