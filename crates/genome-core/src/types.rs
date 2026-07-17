//! The serde output types returned across the `command_json` boundary. Every
//! `f64` in these types is rounded to a fixed precision before serialization
//! (see [`crate::query::round_to`]) so that the CLI and every language binding
//! emit byte-identical JSON.

use serde::{Deserialize, Serialize};

/// A symbol's feature vector in fixed axis order, with self-describing keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Vector {
    /// The symbol this vector describes.
    pub symbol: String,
    /// The vector dimension (`= keys.len() = values.len()`).
    pub dim: usize,
    /// One value per feature axis, in spec order. `None` when the symbol is not
    /// ready (every axis is `None` in that case).
    pub values: Vec<Option<f64>>,
    /// The canonical feature key of each axis, in spec order.
    pub keys: Vec<String>,
    /// Whether the symbol is past warmup with a finite value on every axis.
    pub ready: bool,
}

/// A nearest-neighbor result: a symbol and its distance from the query symbol.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Neighbor {
    /// The neighbor symbol.
    pub symbol: String,
    /// The distance under the spec metric.
    pub distance: f64,
}

/// One k-means cluster: its centroid in normalized feature space and its member
/// symbols (sorted by key).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cluster {
    /// The centroid, one value per feature axis.
    pub centroid: Vec<f64>,
    /// The member symbols, sorted ascending by key.
    pub members: Vec<String>,
}

/// A per-symbol anomaly score: the distance to its nearest neighbor.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Anomaly {
    /// The symbol.
    pub symbol: String,
    /// The nearest-neighbor distance (larger = more of an outlier).
    pub score: f64,
}
