# Wickra Genome examples — C#

Runnable C# examples for the [Wickra Genome C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-genome-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Genome
```

## The examples

| Example | What it does |
|---------|--------------|
| `Genome/Program.cs` | A runnable C# example: build a genome over a tiny three-symbol universe and print the nearest neighbour and the biggest outlier. |
