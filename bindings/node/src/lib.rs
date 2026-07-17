//! Wickra Genome — node binding. Scaffold; the real surface lands in P-GEN-3.

/// The crate version, forwarded from the core.
#[must_use]
pub fn version() -> &'static str {
    genome_core::version()
}
