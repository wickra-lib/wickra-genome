<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Genome — a vector database of the whole market" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/ci.svg)](https://github.com/wickra-lib/wickra-genome/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-genome)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-genome)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-genome/license.svg)](https://github.com/wickra-lib/wickra-genome#license)

# Wickra Genome — Java

---

**Part of the [Wickra ecosystem](#ecosystem): — for Java. `org.wickra:wickra-genome` — prebuilt native library inside the jar, no JNI, no system dependencies.**

JVM bindings for the Wickra Genome vector engine over its C ABI hub, using the
Foreign Function & Memory API (FFM / Panama). A `Genome` is built from a spec
JSON and driven over a JSON boundary, so the similarity, clustering and anomaly
results are byte-identical to every other Wickra Genome binding.

## Requirements

- JDK 22+ (the FFM API is stable since Java 22). Run with
  `--enable-native-access=ALL-UNNAMED`.
- The native C ABI library, built by `cargo build -p wickra-genome-c`.
  The binding loads it from the directory named by the `native.lib.dir` system
  property (the Maven build points it at the workspace `target/debug`).

## Install

Maven:

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-genome</artifactId>
  <version>0.1.1</version>
</dependency>
```

Gradle:

```kotlin
implementation("org.wickra:wickra-genome:0.1.1")
```

The native library ships prebuilt per platform inside the jar and is
extracted automatically on first use. There is nothing to compile.

### Building from this repository (contributors)

```bash
cargo build -p wickra-genome-c
mvn test
```

## Quick start

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

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the Java Foreign Function & Memory API over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-genome/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-genome>
- **Docs** (guides, spec reference, cookbook): <https://genome.wickra.org>
- **Runnable example:** [`examples/java/`](https://github.com/wickra-lib/wickra-genome/tree/main/examples/java)

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
