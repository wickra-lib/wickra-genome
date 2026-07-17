# Wickra Genome — Java

JVM bindings for the Wickra Genome vector engine over its C ABI hub, using the
Foreign Function & Memory API (FFM / Panama). A `Genome` is built from a spec
JSON and driven over a JSON boundary, so the similarity, clustering and anomaly
results are byte-identical to every other Wickra Genome binding.

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/wickra-lib/wickra-genome#license)
[![FFM / Panama](https://img.shields.io/badge/bindings-FFM%20%2F%20Panama-3b82f6)](https://openjdk.org/jeps/454)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

## Requirements

- JDK 22+ (the FFM API is stable since Java 22). Run with
  `--enable-native-access=ALL-UNNAMED`.
- The native C ABI library, built by `cargo build -p wickra-genome-c`.
  The binding loads it from the directory named by the `native.lib.dir` system
  property (the Maven build points it at the workspace `target/debug`).

## Usage

```java
import org.wickra.genome.Genome;

String spec = "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
        + "\"symbols\":[\"AAA\",\"BBB\",\"CCC\"],\"normalize\":\"z_score\","
        + "\"metric\":\"euclid\",\"seed\":24333}";
try (Genome g = new Genome(spec)) {
    String data = "{\"AAA\":[{\"time\":0,\"open\":1,\"high\":1,\"low\":1,\"close\":1,\"volume\":0}],"
            + "\"BBB\":[{\"time\":0,\"open\":2,\"high\":2,\"low\":2,\"close\":2,\"volume\":0}],"
            + "\"CCC\":[{\"time\":0,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":0}]}";
    g.command("{\"cmd\":\"build\",\"data\":" + data + "}");
    System.out.println(g.command("{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":2}"));
}
```

## Test

```bash
cargo build -p wickra-genome-c
mvn test
```

## License

Dual-licensed under either of Apache-2.0 or MIT at your option.
