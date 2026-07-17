#![no_main]
//! Fuzz the JSON command boundary: arbitrary bytes are handed to a valid genome's
//! `command_json`. Every input must return a JSON string — a malformed command or
//! an unknown symbol comes back in-band as `{"ok":false,...}` / an error object,
//! never a panic.

use genome_core::Genome;
use libfuzzer_sys::fuzz_target;

const SPEC: &str = r#"{"features":[{"kind":"price","field":"close"},
    {"kind":"indicator","name":"Rsi","params":[14]}],
    "symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":7}"#;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let mut genome = Genome::new(SPEC).expect("a fixed valid spec");
    let out = genome.command_json(text);
    // The response is always a valid JSON value (success or an error object).
    assert!(
        serde_json::from_str::<serde_json::Value>(&out).is_ok(),
        "command_json must always return valid JSON"
    );
});
