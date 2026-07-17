//! The engine handle and the `command_json` FFI boundary. `Genome` owns a spec
//! and a rolling universe; every binding drives it through the single
//! [`Genome::command_json`] entry point, which returns a JSON string (a success
//! payload or `{"ok":false,"error":...}`), so all ten languages return
//! byte-identical answers.

use crate::error::{Error, Result};
use crate::query::{anomaly, similar, vector};
use crate::spec::GenomeSpec;
use crate::types::{Anomaly, Cluster, Neighbor, Vector};
use crate::universe::Universe;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use wickra_backtest_core::Candle;

/// The `similar` response envelope. A typed struct (not the `json!` macro) so
/// serde preserves field order and every binding emits byte-identical JSON.
#[derive(Serialize)]
struct Neighbors {
    neighbors: Vec<Neighbor>,
}

/// The `cluster` response envelope.
#[derive(Serialize)]
struct Clusters {
    clusters: Vec<Cluster>,
}

/// The `anomaly` response envelope.
#[derive(Serialize)]
struct Anomalies {
    anomalies: Vec<Anomaly>,
}

/// The `version` response envelope.
#[derive(Serialize)]
struct VersionResp {
    version: &'static str,
}

/// The market-genome engine: a validated spec plus the rolling cross-section.
pub struct Genome {
    spec: GenomeSpec,
    universe: Universe,
}

impl Genome {
    /// Build a genome from a JSON spec string.
    ///
    /// # Errors
    /// Propagates spec parse/validation errors.
    pub fn new(spec_json: &str) -> Result<Self> {
        let spec = GenomeSpec::from_json(spec_json)?;
        Ok(Self {
            spec,
            universe: Universe::new(),
        })
    }

    /// Build a genome from an already-parsed spec.
    #[must_use]
    pub fn with_spec(spec: GenomeSpec) -> Self {
        Self {
            spec,
            universe: Universe::new(),
        }
    }

    /// The crate version string.
    #[must_use]
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Replace the spec, keeping the current universe. The universe states were
    /// built for the previous spec; callers replacing the spec should also
    /// [`reset`](Self::reset) if the feature set changed.
    pub fn set_spec(&mut self, spec: GenomeSpec) {
        self.spec = spec;
    }

    /// Clear the universe, keeping the spec.
    pub fn reset(&mut self) {
        self.universe = Universe::new();
    }

    /// Fold one candle into a symbol's rolling state.
    ///
    /// # Errors
    /// Propagates registry errors when the spec references an unknown indicator.
    pub fn feed(&mut self, symbol: &str, candle: &Candle) -> Result<()> {
        self.universe.fold(symbol, candle, &self.spec)
    }

    /// The self-describing feature vector for a symbol.
    ///
    /// # Errors
    /// [`Error::UnknownSymbol`] if the symbol is not in the universe.
    pub fn vector(&self, symbol: &str) -> Result<Vector> {
        vector(&self.universe, &self.spec, symbol)
    }

    /// The `k` nearest neighbors of a symbol.
    ///
    /// # Errors
    /// [`Error::UnknownSymbol`] if the symbol is not a ready member.
    pub fn similar(&self, symbol: &str, k: usize) -> Result<Vec<Neighbor>> {
        similar(&self.universe, &self.spec, symbol, k)
    }

    /// The seeded k-means clustering of the ready universe.
    #[must_use]
    pub fn cluster(&self, k: usize) -> Vec<Cluster> {
        let rows =
            crate::normalize::normalize(&self.universe.ready(&self.spec), self.spec.normalize);
        crate::cluster::kmeans(&rows, k, self.spec.metric, self.spec.seed)
    }

    /// Each ready symbol's nearest-neighbor anomaly score.
    ///
    /// # Errors
    /// Never fails today; returns [`Result`] for symmetry with the other queries.
    pub fn anomaly(&self) -> Result<Vec<Anomaly>> {
        anomaly(&self.universe, &self.spec)
    }

    /// The single FFI entry point. Parses a command envelope, dispatches it, and
    /// returns a JSON string — the success payload, or `{"ok":false,"error":...}`
    /// on any failure. Never panics on bad input.
    #[must_use]
    pub fn command_json(&mut self, cmd_json: &str) -> String {
        match self.dispatch(cmd_json) {
            Ok(s) => s,
            Err(e) => error_json(&e.to_string()),
        }
    }

    /// The fallible dispatcher behind [`command_json`](Self::command_json).
    fn dispatch(&mut self, cmd_json: &str) -> Result<String> {
        let env: Value = serde_json::from_str(cmd_json)?;
        let cmd = env
            .get("cmd")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::BadSpec("missing cmd".into()))?;
        match cmd {
            "set_spec" => {
                let spec: GenomeSpec = field(&env, "spec")?;
                spec.validate()?;
                self.spec = spec;
                self.universe = Universe::new();
                Ok(ok_json())
            }
            "feed" => {
                let symbol = str_field(&env, "symbol")?;
                let candle: Candle = field(&env, "candle")?;
                self.feed(&symbol, &candle)?;
                Ok(ok_json())
            }
            "feed_batch" => {
                let symbol = str_field(&env, "symbol")?;
                let candles: Vec<Candle> = field(&env, "candles")?;
                for c in &candles {
                    self.feed(&symbol, c)?;
                }
                Ok(ok_json())
            }
            "build" => {
                let data: BTreeMap<String, Vec<Candle>> = field(&env, "data")?;
                let genome = build(&data, &self.spec)?;
                self.universe = genome.universe;
                Ok(ok_json())
            }
            "vector" => {
                let symbol = str_field(&env, "symbol")?;
                Ok(serde_json::to_string(&self.vector(&symbol)?)?)
            }
            "similar" => {
                let symbol = str_field(&env, "symbol")?;
                let k = usize_field(&env, "k")?;
                let neighbors = self.similar(&symbol, k)?;
                Ok(serde_json::to_string(&Neighbors { neighbors })?)
            }
            "cluster" => {
                let k = usize_field(&env, "k")?;
                let clusters = self.cluster(k);
                Ok(serde_json::to_string(&Clusters { clusters })?)
            }
            "anomaly" => {
                let anomalies = self.anomaly()?;
                Ok(serde_json::to_string(&Anomalies { anomalies })?)
            }
            "reset" => {
                self.reset();
                Ok(ok_json())
            }
            "version" => Ok(serde_json::to_string(&VersionResp {
                version: Self::version(),
            })?),
            other => Err(Error::BadSpec(format!("unknown cmd: {other}"))),
        }
    }
}

/// Fold every symbol's full candle history into a fresh universe, then return the
/// built genome. Symbols are independent, so the per-symbol fold parallelizes
/// (feature `parallel`) with no effect on the result — the cross-section
/// reductions downstream always run serially in key order.
///
/// # Errors
/// Propagates registry errors from building a symbol's indicator set.
pub fn build(data: &BTreeMap<String, Vec<Candle>>, spec: &GenomeSpec) -> Result<Genome> {
    #[cfg(feature = "parallel")]
    let states: Vec<(String, crate::symbol_state::SymbolState)> = {
        use rayon::prelude::*;
        data.par_iter()
            .map(|(sym, candles)| fold_symbol(sym, candles, spec))
            .collect::<Result<Vec<_>>>()?
    };
    #[cfg(not(feature = "parallel"))]
    let states: Vec<(String, crate::symbol_state::SymbolState)> = data
        .iter()
        .map(|(sym, candles)| fold_symbol(sym, candles, spec))
        .collect::<Result<Vec<_>>>()?;

    let mut universe = Universe::new();
    for (sym, state) in states {
        universe.symbols.insert(sym, state);
    }
    Ok(Genome {
        spec: spec.clone(),
        universe,
    })
}

/// Fold one symbol's candles into a fresh state.
fn fold_symbol(
    sym: &str,
    candles: &[Candle],
    spec: &GenomeSpec,
) -> Result<(String, crate::symbol_state::SymbolState)> {
    let mut state = crate::symbol_state::SymbolState::new(spec)?;
    for c in candles {
        state.fold(c);
    }
    Ok((sym.to_string(), state))
}

/// Deserialize a required object field into `T`.
fn field<T: for<'de> Deserialize<'de>>(env: &Value, key: &str) -> Result<T> {
    let v = env
        .get(key)
        .ok_or_else(|| Error::BadSpec(format!("missing {key}")))?;
    serde_json::from_value(v.clone()).map_err(Into::into)
}

/// Read a required string field.
fn str_field(env: &Value, key: &str) -> Result<String> {
    env.get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| Error::BadSpec(format!("missing {key}")))
}

/// Read a required non-negative integer field as `usize`.
fn usize_field(env: &Value, key: &str) -> Result<usize> {
    env.get(key)
        .and_then(Value::as_u64)
        .map(|n| n as usize)
        .ok_or_else(|| Error::BadSpec(format!("missing {key}")))
}

/// The canonical `{"ok":true}` response.
fn ok_json() -> String {
    "{\"ok\":true}".to_string()
}

/// A `{"ok":false,"error":...}` response with the message JSON-escaped.
fn error_json(message: &str) -> String {
    json!({ "ok": false, "error": message }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec_json() -> &'static str {
        r#"{"features":[{"kind":"price","field":"close"}],
            "symbols":["AAA","BBB","CCC"],
            "normalize":"z_score","metric":"euclid","seed":24333}"#
    }

    fn candle(close: f64) -> String {
        format!(
            r#"{{"time":0,"open":{close},"high":{close},"low":{close},"close":{close},"volume":0}}"#
        )
    }

    #[test]
    fn version_command() {
        let mut g = Genome::new(spec_json()).unwrap();
        let out = g.command_json(r#"{"cmd":"version"}"#);
        assert_eq!(out, r#"{"version":"0.1.0"}"#);
    }

    #[test]
    fn unknown_cmd_is_error_json_not_panic() {
        let mut g = Genome::new(spec_json()).unwrap();
        let out = g.command_json(r#"{"cmd":"nope"}"#);
        assert!(out.contains("\"ok\":false"));
        assert!(out.contains("unknown cmd"));
    }

    #[test]
    fn malformed_json_is_error() {
        let mut g = Genome::new(spec_json()).unwrap();
        let out = g.command_json("not json");
        assert!(out.contains("\"ok\":false"));
    }

    #[test]
    fn feed_then_similar_and_anomaly() {
        let mut g = Genome::new(spec_json()).unwrap();
        for (sym, c) in [("AAA", 1.0), ("BBB", 2.0), ("CCC", 100.0)] {
            let cmd = format!(
                r#"{{"cmd":"feed","symbol":"{sym}","candle":{}}}"#,
                candle(c)
            );
            assert_eq!(g.command_json(&cmd), "{\"ok\":true}");
        }
        let sim = g.command_json(r#"{"cmd":"similar","symbol":"AAA","k":2}"#);
        assert!(sim.contains("\"neighbors\""));
        assert!(sim.contains("BBB"));
        let anom = g.command_json(r#"{"cmd":"anomaly"}"#);
        assert!(anom.starts_with("{\"anomalies\":[{\"symbol\":\"CCC\""));
    }

    #[test]
    fn build_via_command_matches_feed() {
        // build path and feed path must agree.
        let mut fed = Genome::new(spec_json()).unwrap();
        for (sym, c) in [("AAA", 1.0), ("BBB", 2.0), ("CCC", 100.0)] {
            let cmd = format!(
                r#"{{"cmd":"feed","symbol":"{sym}","candle":{}}}"#,
                candle(c)
            );
            let _ = fed.command_json(&cmd);
        }
        let mut built = Genome::new(spec_json()).unwrap();
        let build_cmd = format!(
            r#"{{"cmd":"build","data":{{"AAA":[{}],"BBB":[{}],"CCC":[{}]}}}}"#,
            candle(1.0),
            candle(2.0),
            candle(100.0)
        );
        let _ = built.command_json(&build_cmd);
        let q = r#"{"cmd":"anomaly"}"#;
        assert_eq!(fed.command_json(q), built.command_json(q));
    }

    #[test]
    fn reset_clears_universe() {
        let mut g = Genome::new(spec_json()).unwrap();
        let cmd = format!(
            r#"{{"cmd":"feed","symbol":"AAA","candle":{}}}"#,
            candle(1.0)
        );
        let _ = g.command_json(&cmd);
        assert_eq!(g.command_json(r#"{"cmd":"reset"}"#), "{\"ok\":true}");
        let v = g.command_json(r#"{"cmd":"vector","symbol":"AAA"}"#);
        assert!(v.contains("\"ok\":false")); // AAA no longer in universe
    }
}
