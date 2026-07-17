#![no_main]
//! Fuzz the spec-parsing surface: arbitrary bytes are parsed as a `GenomeSpec`
//! (JSON). Malformed input must surface as a clean `Err`, never a panic. A
//! successfully parsed spec re-serializes and re-parses to an equal value.

use genome_core::GenomeSpec;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(spec) = serde_json::from_str::<GenomeSpec>(text) else {
        return;
    };
    // The public validating parser must not panic; either verdict is fine.
    let _ = GenomeSpec::from_json(text);
    let serialized = serde_json::to_string(&spec).expect("serialize a parsed spec");
    let reparsed: GenomeSpec =
        serde_json::from_str(&serialized).expect("re-parse a serialized spec");
    assert_eq!(reparsed, spec, "GenomeSpec serde round-trip is not stable");
});
