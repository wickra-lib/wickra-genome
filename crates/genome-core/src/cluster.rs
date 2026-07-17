//! Deterministic, seeded k-means (Lloyd's algorithm with a k-means++ seed) over
//! the normalized feature vectors of the ready universe. Determinism is the
//! product's moat: the same seed and data produce byte-identical clusters in
//! every language, because only this Rust core computes them and every reduction
//! runs in a fixed key order (see [`crate::prng::SplitMix64`]).

use crate::metric::distance;
use crate::prng::SplitMix64;
use crate::query::round_to;
use crate::spec::Metric;
use crate::types::Cluster;

/// The iteration ceiling for Lloyd's algorithm; also converges early on a stable
/// assignment.
const MAX_ITERS: usize = 100;

/// Run seeded k-means over `rows` (`(symbol, normalized_vector)` in key order).
///
/// `k` is clamped to `1..=rows.len()`. Returns the non-empty clusters, each with
/// its members sorted by key and its centroid rounded to `1e-8`; the cluster
/// list is sorted by each cluster's smallest member key, giving a stable,
/// seed-independent output order.
#[must_use]
pub(crate) fn kmeans(
    rows: &[(String, Vec<f64>)],
    k: usize,
    metric: Metric,
    seed: u64,
) -> Vec<Cluster> {
    let n = rows.len();
    if n == 0 {
        return Vec::new();
    }
    let k = k.clamp(1, n);
    let dim = rows[0].1.len();

    let mut centroids = seed_centroids(rows, k, metric, seed);
    let mut assignment = vec![usize::MAX; n];

    for _ in 0..MAX_ITERS {
        let mut changed = false;
        for (i, (_, v)) in rows.iter().enumerate() {
            let nearest = nearest_centroid(v, &centroids, metric);
            if nearest != assignment[i] {
                assignment[i] = nearest;
                changed = true;
            }
        }
        if !changed {
            break;
        }
        // Update: each centroid becomes the mean of its members, summed in the
        // rows' key order; an empty cluster keeps its previous centroid.
        for (c, centroid) in centroids.iter_mut().enumerate() {
            let mut sum = vec![0.0; dim];
            let mut count = 0usize;
            for (i, (_, v)) in rows.iter().enumerate() {
                if assignment[i] == c {
                    for (s, x) in sum.iter_mut().zip(v) {
                        *s += x;
                    }
                    count += 1;
                }
            }
            if count > 0 {
                for (axis, s) in centroid.iter_mut().zip(&sum) {
                    *axis = s / count as f64;
                }
            }
        }
    }

    // Canonicalize: gather members per cluster (rows are already in key order),
    // drop empty clusters, round centroids, sort clusters by smallest member.
    let mut clusters: Vec<Cluster> = (0..k)
        .filter_map(|c| {
            let members: Vec<String> = rows
                .iter()
                .enumerate()
                .filter(|(i, _)| assignment[*i] == c)
                .map(|(_, (sym, _))| sym.clone())
                .collect();
            if members.is_empty() {
                return None;
            }
            let centroid = centroids[c].iter().map(|x| round_to(*x)).collect();
            Some(Cluster { centroid, members })
        })
        .collect();
    clusters.sort_by(|a, b| a.members[0].cmp(&b.members[0]));
    clusters
}

/// k-means++ seeding: the first centroid is the smallest-key row (a deterministic
/// anchor); each further centroid is drawn with probability proportional to its
/// squared distance from the nearest chosen centroid, using the seeded PRNG and
/// walking candidates in key order.
fn seed_centroids(
    rows: &[(String, Vec<f64>)],
    k: usize,
    metric: Metric,
    seed: u64,
) -> Vec<Vec<f64>> {
    let mut rng = SplitMix64::new(seed);
    let mut centroids: Vec<Vec<f64>> = vec![rows[0].1.clone()];

    while centroids.len() < k {
        let d2: Vec<f64> = rows
            .iter()
            .map(|(_, v)| {
                let nearest = centroids
                    .iter()
                    .map(|c| distance(v, c, metric))
                    .fold(f64::INFINITY, f64::min);
                nearest * nearest
            })
            .collect();
        let total: f64 = d2.iter().sum();
        let pick = if total > 0.0 {
            let target = rng.next_f64() * total;
            let mut acc = 0.0;
            let mut chosen = rows.len() - 1;
            for (i, w) in d2.iter().enumerate() {
                acc += w;
                if acc > target {
                    chosen = i;
                    break;
                }
            }
            chosen
        } else {
            // All rows coincide with a centroid; take the first not yet chosen.
            (0..rows.len())
                .find(|i| !centroids.iter().any(|c| c == &rows[*i].1))
                .unwrap_or(0)
        };
        centroids.push(rows[pick].1.clone());
    }
    centroids
}

/// The index of the nearest centroid to `v`; ties break to the smallest index.
fn nearest_centroid(v: &[f64], centroids: &[Vec<f64>], metric: Metric) -> usize {
    let mut best = 0;
    let mut best_dist = f64::INFINITY;
    for (i, c) in centroids.iter().enumerate() {
        let d = distance(v, c, metric);
        if d < best_dist {
            best_dist = d;
            best = i;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows() -> Vec<(String, Vec<f64>)> {
        vec![
            ("AAA".into(), vec![0.0, 0.0]),
            ("BBB".into(), vec![0.1, 0.1]),
            ("CCC".into(), vec![10.0, 10.0]),
            ("DDD".into(), vec![10.1, 9.9]),
        ]
    }

    #[test]
    fn separates_two_obvious_clusters() {
        let cs = kmeans(&rows(), 2, Metric::Euclid, 24333);
        assert_eq!(cs.len(), 2);
        // The two low points cluster together, the two high points together.
        let low = cs
            .iter()
            .find(|c| c.members.contains(&"AAA".to_string()))
            .unwrap();
        assert_eq!(low.members, vec!["AAA", "BBB"]);
    }

    #[test]
    fn same_seed_same_result() {
        let a = kmeans(&rows(), 2, Metric::Euclid, 99);
        let b = kmeans(&rows(), 2, Metric::Euclid, 99);
        assert_eq!(a, b);
    }

    #[test]
    fn cluster_list_sorted_by_smallest_member() {
        let cs = kmeans(&rows(), 2, Metric::Euclid, 1);
        assert!(cs[0].members[0] < cs[1].members[0]);
    }

    #[test]
    fn k_at_least_one() {
        let cs = kmeans(&rows(), 0, Metric::Euclid, 1);
        assert_eq!(cs.len(), 1);
        assert_eq!(cs[0].members.len(), 4);
    }

    #[test]
    fn k_ge_n_gives_singletons() {
        let cs = kmeans(&rows(), 9, Metric::Euclid, 1);
        assert_eq!(cs.len(), 4);
        assert!(cs.iter().all(|c| c.members.len() == 1));
    }

    #[test]
    fn empty_rows_empty_result() {
        assert!(kmeans(&[], 2, Metric::Euclid, 1).is_empty());
    }
}
