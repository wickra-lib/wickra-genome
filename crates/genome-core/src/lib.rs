//! Wickra Genome — the market-genome vector engine.
//!
//! Every asset in a universe becomes a live feature vector over the 514 O(1)
//! streaming indicators of `wickra-core`: a vector database of the whole market.
//! Four queries run over that one vector space — [`Genome::vector`] (a symbol's
//! axes), [`Genome::similar`] (k nearest neighbors), [`Genome::cluster`] (seeded
//! k-means) and [`Genome::anomaly`] (nearest-neighbor outlier scores) — so you
//! can ask "which symbols behave right now like this one?".
//!
//! The engine is data-driven: a [`GenomeSpec`] (a serde value, not Rust closures)
//! fixes the feature axes, the cross-section normalization and the distance
//! metric. It is usable in ten languages over a single JSON-over-C-ABI boundary,
//! [`Genome::command_json`], and every language returns byte-identical answers
//! because only this Rust core computes them. Determinism is enforced end to end:
//! `BTreeMap` ordering, serial reductions in symbol-key order, a portable seeded
//! PRNG for k-means, and fixed `1e-8` output rounding.
//!
//! ```
//! use genome_core::Genome;
//! let spec = r#"{"features":[{"kind":"price","field":"close"}],
//!                "symbols":["AAA","BBB"],"metric":"euclid"}"#;
//! let mut g = Genome::new(spec).unwrap();
//! let feed = r#"{"cmd":"feed","symbol":"AAA",
//!                "candle":{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}}"#;
//! assert_eq!(g.command_json(feed), "{\"ok\":true}");
//! ```

mod cluster;
mod config;
mod error;
mod feature;
mod genome;
mod indicator_set;
mod metric;
mod normalize;
mod prng;
mod query;
mod spec;
mod symbol_state;
mod types;
mod universe;

pub use config::Config;
pub use error::{Error, Result};
pub use feature::{Feature, PriceField};
pub use genome::{build, Genome};
pub use spec::{default_seed, GenomeSpec, Metric, Normalize};
pub use types::{Anomaly, Cluster, Neighbor, Vector};

/// The OHLCV candle type consumed by the engine (re-exported from
/// `wickra-backtest-core`), so consumers build a universe without a direct
/// dependency on the engine crate.
pub use wickra_backtest_core::Candle;

/// The crate version (`CARGO_PKG_VERSION`).
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
