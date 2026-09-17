# Wickra Genome examples

Runnable examples in every Wickra Genome language. Each one builds a genome over
the same tiny three-symbol universe — `AAA` and `BBB` close together, `CCC` far
away — and prints the same summary:

## What every example prints

Runnable examples in every Wickra Genome language. Each one builds a genome over
the same tiny three-symbol universe — `AAA` and `BBB` close together, `CCC` far
away — and prints the same summary:

```
wickra-genome 0.1.1
AAA nearest: BBB
top anomaly: CCC
```

## Rust — `examples/rust/`

As the CI examples job runs it, from the repository root:

```bash
cargo run -q --manifest-path examples/rust/Cargo.toml
```

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: build a genome over a tiny three-symbol universe and print the nearest neighbour and the biggest outlier. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-genome-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `genome.c` | A runnable C example: build a genome over a tiny three-symbol universe through |
| `genome.cpp` | A runnable C++ example: build a genome over a tiny three-symbol universe and print the nearest-neighbour response. |

## C# — `examples/csharp/`

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Genome
```

| Example | What it does |
| --- | --- |
| `Genome/Program.cs` | A runnable C# example: build a genome over a tiny three-symbol universe and print the nearest neighbour and the biggest outlier. |

## Go — `examples/go/`

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

| Example | What it does |
| --- | --- |
| `genome.go` | A runnable Go example: build a genome over a tiny three-symbol universe and print the nearest neighbour and the biggest outlier. |

## R — `examples/r/`

As the CI examples job runs it, from the repository root:

```bash
R CMD INSTALL bindings/r
Rscript examples/r/genome.R
```

| Example | What it does |
| --- | --- |
| `genome.R` | A runnable R example: build a genome over a tiny three-symbol universe and print the nearest neighbour and the biggest outlier. |

## Java — `examples/java/`

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q install -DskipTests
mvn -f examples/java/pom.xml -q compile exec:exec  -Dnative.lib.dir="$PWD/target/release"
```

| Example | What it does |
| --- | --- |
| `src/main/java/org/wickra/genome/examples/Genome.java` | A price-close genome over three symbols: AAA and BBB sit close together, CCC is far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly ranking. |

## Python — `examples/python/`

As the CI examples job runs it, from the repository root:

```bash
python -m pip install --require-hashes -r .github/requirements/ci-dev-py3.txt
( cd bindings/python && maturin build --release --out dist )
python -m pip install --no-index --find-links bindings/python/dist wickra-genome
python examples/python/genome.py
```

| Example | What it does |
| --- | --- |
| `genome.py` | A runnable Python example: build a genome over a tiny three-symbol universe |

## Node.js — `examples/node/`

As the CI examples job runs it, from the repository root:

```bash
( cd bindings/node && npm install --no-audit --no-fund && npx napi build --platform --release )
( cd examples/node && npm install --no-audit --no-fund )
node examples/node/genome.js
```

| Example | What it does |
| --- | --- |
| `genome.js` | A runnable Node.js example: build a genome over a tiny three-symbol universe and print the nearest neighbour and the biggest outlier. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `genome.html` | A runnable example against this binding. |

## Example datasets

The examples read from [`examples/data/`](data/): . The
cross-language golden fixtures, which every binding is checked against byte for
byte, live in [`../golden/`](../golden).

## Per language

- **Rust** — [`rust/`](rust/): `cargo run --manifest-path examples/rust/Cargo.toml`
- **Python** — [`python/genome.py`](python/genome.py): `pip install wickra-genome && python examples/python/genome.py`
- **Node.js** — [`node/`](node/): `cd examples/node && npm install && node genome.js`
- **Go** — [`go/`](go/): `go run examples/go/genome.go` (with the C ABI library staged, see the Go binding README)
- **C#** — [`csharp/Genome/`](csharp/Genome/): `dotnet run --project examples/csharp/Genome`
- **Java** — [`java/`](java/): `mvn -q compile exec:java -Dexec.mainClass=org.wickra.genome.examples.Genome`
- **R** — [`r/genome.R`](r/genome.R): `R CMD INSTALL bindings/r && Rscript examples/r/genome.R`
- **WASM** — [`wasm/genome.html`](wasm/genome.html): `wasm-pack build bindings/wasm --target web`, serve the repository root, then open `examples/wasm/genome.html`
- **C / C++** — [`c/`](c/): build the C ABI, then CMake + ctest:

  ```bash
  cargo build --release -p wickra-genome-c
  cmake -S examples/c -B examples/c/build
  cmake --build examples/c/build --config Release
  ctest --test-dir examples/c/build -C Release --output-on-failure
  ```

The binding examples install the published `wickra-genome` package for their
language; the Rust and C/C++ examples build against the in-repo core.
