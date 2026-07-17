//! A runnable Rust example: build a genome over a tiny three-symbol universe and
//! print the nearest neighbour and the biggest outlier. Every language example
//! runs the same request and prints the same summary — that is the cross-language
//! guarantee.
//!
//! ```bash
//! cargo run --manifest-path examples/rust/Cargo.toml
//! ```

use genome_core::Genome;
use serde_json::Value;

// A price-close genome over three symbols: AAA and BBB sit close together, CCC is
// far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly ranking.
const SPEC: &str = r#"{"features":[{"kind":"price","field":"close"}],
    "symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":24333}"#;

const BUILD: &str = r#"{"cmd":"build","data":{
    "AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],
    "BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],
    "CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}}"#;

fn main() {
    let mut genome = Genome::new(SPEC).expect("valid spec");
    genome.command_json(BUILD);

    let version: Value =
        serde_json::from_str(&genome.command_json(r#"{"cmd":"version"}"#)).unwrap();
    let similar: Value =
        serde_json::from_str(&genome.command_json(r#"{"cmd":"similar","symbol":"AAA","k":2}"#))
            .unwrap();
    let anomaly: Value =
        serde_json::from_str(&genome.command_json(r#"{"cmd":"anomaly"}"#)).unwrap();

    println!("wickra-genome {}", version["version"].as_str().unwrap());
    println!(
        "AAA nearest: {}",
        similar["neighbors"][0]["symbol"].as_str().unwrap()
    );
    println!(
        "top anomaly: {}",
        anomaly["anomalies"][0]["symbol"].as_str().unwrap()
    );
}
