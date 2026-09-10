# wickra-genome (C#)

.NET bindings for [`wickra-genome`](https://github.com/wickra-lib/wickra-genome)
over the C ABI hub, via source-generated P/Invoke. Build a `Genome` from a spec
JSON, drive it with command JSON and read back vectors, neighbours, clusters and
anomaly scores — the same protocol the CLI and every other binding speak,
returning the same bytes.

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

Requires .NET 8+. The native library (`wickra_genome`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

Licensed under either of [MIT](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-genome/blob/main/LICENSE-APACHE) at your option.
