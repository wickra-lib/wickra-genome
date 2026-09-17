<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/ci.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-genome)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/nuget.svg)](https://www.nuget.org/packages/Wickra.Genome)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/license.svg)](https://github.com/wickra-lib/wickra-genome#license)

# Wickra Genome — C#

---

**Part of the [Wickra ecosystem](#ecosystem): — for C#. `dotnet add package Wickra.Genome` — prebuilt native library, no system dependencies.**

.NET bindings for [`wickra-genome`](https://github.com/wickra-lib/wickra-genome)
over the C ABI hub, via source-generated P/Invoke. Build a `Genome` from a spec
JSON, drive it with command JSON and read back vectors, neighbours, clusters and
anomaly scores — the same protocol the CLI and every other binding speak,
returning the same bytes.

## Install

```bash
dotnet add package Wickra.Genome
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

Requires .NET 8+. The native library (`wickra_genome`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

## Quick start

```csharp
using Wickra.Genome;

const string spec = """
{"features":[{"kind":"indicator","name":"Rsi","params":[14]},
             {"kind":"price","field":"close"}],
 "symbols":["AAA","BBB"],
 "normalize":"z_score","metric":"euclid","seed":24333}
""";

using var genome = new Genome(spec);
genome.Command("""{"cmd":"feed","symbol":"AAA","candle":{"time":1,
  "open":100,"high":101,"low":99,"close":100.5,"volume":1000}}""");
string neighbours = genome.Command("""{"cmd":"similar","symbol":"AAA","k":5}""");
```

An axis whose indicator reads a side feed — a reference series, a derivatives
tick, an order book, the bar's trades, the market cross-section — needs that feed
supplied, either on the `feed` (`"feeds":{…}`) or per symbol in a `build`
payload. A spec naming one without it is refused by name rather than answered
with an axis that can never become ready, which would take the whole symbol out
of similarity search, clustering and anomaly scoring.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-genome/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-genome>
- **Docs** (guides, spec reference, cookbook): <https://genome.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-genome/tree/main/examples/csharp)

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
