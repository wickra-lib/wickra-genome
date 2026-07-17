//! Cross-section normalization of the raw feature vectors, applied per axis over
//! the ready universe. Every reduction runs serially in the row order it is
//! given (symbol-key order from [`crate::universe::Universe::ready`]) so the
//! `f64` result is identical across languages and thread counts.

use crate::spec::Normalize;

/// Normalize a set of raw vectors per feature axis.
///
/// `rows` are `(symbol, raw_vector)` pairs in symbol-key order; every raw vector
/// has the same length (`= features.len()`). Returns the same rows with each
/// axis normalized:
/// - **z-score:** `(x - mean) / std_pop`; a constant axis (`std == 0`) maps to `0`.
/// - **min-max:** `(x - min) / (max - min)` in `[0, 1]`; a constant axis maps to `0`.
///
/// With `n <= 1` there is no meaningful cross-section, so every value is `0`.
pub(crate) fn normalize(rows: &[(String, Vec<f64>)], mode: Normalize) -> Vec<(String, Vec<f64>)> {
    let n = rows.len();
    if n == 0 {
        return Vec::new();
    }
    let dim = rows[0].1.len();
    let mut out: Vec<(String, Vec<f64>)> = rows
        .iter()
        .map(|(s, _)| (s.clone(), vec![0.0; dim]))
        .collect();
    if n <= 1 {
        return out;
    }

    for axis in 0..dim {
        match mode {
            Normalize::ZScore => {
                let mut sum = 0.0;
                for (_, v) in rows {
                    sum += v[axis];
                }
                let mean = sum / n as f64;
                let mut var = 0.0;
                for (_, v) in rows {
                    let d = v[axis] - mean;
                    var += d * d;
                }
                let std = (var / n as f64).sqrt();
                if std != 0.0 {
                    for (row, (_, v)) in out.iter_mut().zip(rows) {
                        row.1[axis] = (v[axis] - mean) / std;
                    }
                }
            }
            Normalize::MinMax => {
                let mut min = rows[0].1[axis];
                let mut max = rows[0].1[axis];
                for (_, v) in &rows[1..] {
                    if v[axis] < min {
                        min = v[axis];
                    }
                    if v[axis] > max {
                        max = v[axis];
                    }
                }
                let range = max - min;
                if range != 0.0 {
                    for (row, (_, v)) in out.iter_mut().zip(rows) {
                        row.1[axis] = (v[axis] - min) / range;
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    fn rows() -> Vec<(String, Vec<f64>)> {
        vec![
            ("A".into(), vec![1.0, 5.0]),
            ("B".into(), vec![2.0, 5.0]),
            ("C".into(), vec![3.0, 5.0]),
        ]
    }

    #[test]
    fn zscore_centers_and_zeroes_constant_axis() {
        let out = normalize(&rows(), Normalize::ZScore);
        // axis 0: mean 2, std_pop = sqrt(2/3); B is exactly the mean -> 0.
        assert!((out[1].1[0]).abs() < 1e-12);
        assert!(out[0].1[0] < 0.0 && out[2].1[0] > 0.0);
        // axis 1 is constant -> all zero.
        assert_eq!(out[0].1[1], 0.0);
        assert_eq!(out[2].1[1], 0.0);
    }

    #[test]
    fn minmax_maps_to_unit_interval() {
        let out = normalize(&rows(), Normalize::MinMax);
        assert_eq!(out[0].1[0], 0.0);
        assert_eq!(out[2].1[0], 1.0);
        assert!((out[1].1[0] - 0.5).abs() < 1e-12);
        assert_eq!(out[1].1[1], 0.0); // constant axis
    }

    #[test]
    fn single_row_is_all_zero() {
        let out = normalize(&[("A".into(), vec![9.0, 9.0])], Normalize::ZScore);
        assert_eq!(out[0].1, vec![0.0, 0.0]);
    }
}
