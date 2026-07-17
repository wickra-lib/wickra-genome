//! A portable, seeded `SplitMix64` PRNG used to make k-means++ initialization
//! deterministic across every language binding. The algorithm is fixed by this
//! implementation and documented in `docs/CLUSTERING.md`: only the Rust core
//! draws from it (bindings forward the JSON verbatim), but pinning it here means
//! any re-implementation reproduces the exact same cluster assignment.
//!
//! `SplitMix64` (Steele, Lea & Flood, 2014): the state advances by the fixed
//! increment `0x9E3779B97F4A7C15`; each output is mixed with two shift-xor-multiply
//! rounds. `next_f64` takes the top 53 bits and scales to `[0, 1)`.

/// A `SplitMix64` generator seeded with a `u64`.
pub(crate) struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Create a generator from a seed.
    pub(crate) fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// The next 64-bit output.
    pub(crate) fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// The next `f64` in `[0, 1)`, using the top 53 bits (one `f64` mantissa).
    pub(crate) fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_stream() {
        let mut a = SplitMix64::new(42);
        let mut b = SplitMix64::new(42);
        for _ in 0..8 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seed_diverges() {
        let mut a = SplitMix64::new(1);
        let mut b = SplitMix64::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn f64_in_unit_interval() {
        let mut r = SplitMix64::new(7);
        for _ in 0..1000 {
            let x = r.next_f64();
            assert!((0.0..1.0).contains(&x));
        }
    }

    #[test]
    fn first_output_is_pinned() {
        // Pins the exact algorithm so any re-implementation matches byte-for-byte.
        let mut r = SplitMix64::new(0);
        assert_eq!(r.next_u64(), 0xE220_A839_7B1D_CDAF);
    }
}
