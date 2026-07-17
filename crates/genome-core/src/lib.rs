//! Wickra Genome — the market-genome vector engine. Scaffold; the vector build,
//! similarity search, clustering and anomaly detection land in P-GEN-1.

/// The crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
