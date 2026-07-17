//! The distance metric between two normalized feature vectors. Sums run in axis
//! order so the `f64` result is identical everywhere.

use crate::spec::Metric;

/// The distance between two equal-length normalized vectors under `metric`:
/// - **Euclid:** `sqrt(Σ (a_i - b_i)²)`.
/// - **Cosine:** `1 - cos_sim`, where `cos_sim = (a·b) / (‖a‖·‖b‖)`; if either
///   norm is zero the similarity is `0` (distance `1`).
#[must_use]
pub(crate) fn distance(a: &[f64], b: &[f64], metric: Metric) -> f64 {
    match metric {
        Metric::Euclid => {
            let mut sum = 0.0;
            for (x, y) in a.iter().zip(b) {
                let d = x - y;
                sum += d * d;
            }
            sum.sqrt()
        }
        Metric::Cosine => {
            let mut dot = 0.0;
            let mut na = 0.0;
            let mut nb = 0.0;
            for (x, y) in a.iter().zip(b) {
                dot += x * y;
                na += x * x;
                nb += y * y;
            }
            if na == 0.0 || nb == 0.0 {
                return 1.0;
            }
            let cos = dot / (na.sqrt() * nb.sqrt());
            1.0 - cos
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclid_is_l2() {
        let d = distance(&[0.0, 0.0], &[3.0, 4.0], Metric::Euclid);
        assert!((d - 5.0).abs() < 1e-12);
    }

    #[test]
    fn cosine_identical_vectors_zero_distance() {
        let d = distance(&[1.0, 2.0], &[2.0, 4.0], Metric::Cosine);
        assert!(d.abs() < 1e-12);
    }

    #[test]
    fn cosine_orthogonal_is_one() {
        let d = distance(&[1.0, 0.0], &[0.0, 1.0], Metric::Cosine);
        assert!((d - 1.0).abs() < 1e-12);
    }

    #[test]
    fn cosine_zero_norm_is_one() {
        let d = distance(&[0.0, 0.0], &[1.0, 1.0], Metric::Cosine);
        assert!((d - 1.0).abs() < 1e-12);
    }
}
