# Wickra Genome — C / C++ examples

The Wickra Genome C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_genome.h`](../../bindings/c/include/wickra_genome.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_genome.hpp`](../../bindings/c/include/wickra_genome.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-genome-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_genome.so`     | `-lwickra_genome` |
| macOS    | `libwickra_genome.dylib`  | `-lwickra_genome` |
| Windows (MSVC) | `wickra_genome.dll` | `wickra_genome.dll.lib` (import lib) |

A static library (`libwickra_genome.a` / `wickra_genome.lib`) is emitted alongside.

## Build and run the examples

With CMake, as the CI C ABI job does:

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

## The examples

| Example | What it does |
|---------|--------------|
| `genome.c` | A runnable C example: build a genome over a tiny three-symbol universe through |
| `genome.cpp` | A runnable C++ example: build a genome over a tiny three-symbol universe and print the nearest-neighbour response. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_genome.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
