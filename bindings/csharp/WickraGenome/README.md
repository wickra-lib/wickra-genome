# Wickra Genome — C#

.NET bindings for the Wickra Genome vector engine over its C ABI hub. A `Genome`
is built from a spec JSON and driven over a JSON boundary, so the similarity,
clustering and anomaly results are byte-identical to every other Wickra Genome
binding.

## Install

```bash
dotnet add package Wickra.Genome
```

The package ships the native C ABI library per runtime identifier under
`runtimes/<rid>/native/`. For a local build, `cargo build -p wickra-genome-c --release`
places the library in `target/release/`; the bundled `DllImportResolver` probes
the Cargo `target/` tree, so tests and apps in the repo find it without extra
steps.

## Usage

```csharp
using Wickra.Genome;

var spec = "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}]," +
    "\"symbols\":[\"AAA\",\"BBB\",\"CCC\"],\"normalize\":\"z_score\",\"metric\":\"euclid\",\"seed\":24333}";
using var g = new Genome(spec);

var data = "{\"AAA\":[{\"time\":0,\"open\":1,\"high\":1,\"low\":1,\"close\":1,\"volume\":0}]," +
    "\"BBB\":[{\"time\":0,\"open\":2,\"high\":2,\"low\":2,\"close\":2,\"volume\":0}]," +
    "\"CCC\":[{\"time\":0,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":0}]}";
g.Command("{\"cmd\":\"build\",\"data\":" + data + "}");

Console.WriteLine(g.Command("{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":2}"));
Console.WriteLine(g.Command("{\"cmd\":\"anomaly\"}"));
```

The command protocol (`build`, `feed`, `vector`, `similar`, `cluster`,
`anomaly`, `version`) is identical across every binding; only the Rust core
computes, so a fixed seed gives the byte-identical clustering everywhere.

## License

Dual-licensed under either of Apache-2.0 or MIT at your option.
