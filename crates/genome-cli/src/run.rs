//! Load the spec and universe, run the query, and render the answer.

use crate::args::{Args, Format, Op};
use genome_core::{build, Candle, Cluster, Config, Genome, GenomeSpec, Neighbor, Vector};
use serde_json::json;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io::Read;
use std::path::Path;

/// Load the inputs, run the query and return the rendered output.
pub fn run(args: &Args) -> Result<String, String> {
    let spec = load_spec(&args.spec)?;
    let data = if args.stdin {
        load_stdin()?
    } else if let Some(dir) = &args.data {
        load_data_dir(dir)?
    } else {
        return Err("no data source (pass --data or --stdin)".to_string());
    };

    let mut genome = build(&data, &spec).map_err(|e| e.to_string())?;

    let output = match args.op {
        Op::Vector => {
            let symbol = require_symbol(args)?;
            let vector = genome.vector(&symbol).map_err(|e| e.to_string())?;
            render(
                args.format,
                &json!({"cmd":"vector","symbol":symbol}),
                &mut genome,
                || render_vector(&vector),
            )
        }
        Op::Similar => {
            let symbol = require_symbol(args)?;
            let k = require_k(args)?;
            let neighbors = genome.similar(&symbol, k).map_err(|e| e.to_string())?;
            render(
                args.format,
                &json!({"cmd":"similar","symbol":symbol,"k":k}),
                &mut genome,
                || render_neighbors(&symbol, &neighbors),
            )
        }
        Op::Cluster => {
            let k = require_k(args)?;
            let clusters = genome.cluster(k);
            render(
                args.format,
                &json!({"cmd":"cluster","k":k}),
                &mut genome,
                || render_clusters(&clusters),
            )
        }
        Op::Anomaly => {
            let anomalies = genome.anomaly().map_err(|e| e.to_string())?;
            render(args.format, &json!({"cmd":"anomaly"}), &mut genome, || {
                render_anomalies(&anomalies)
            })
        }
    };
    Ok(output)
}

/// Render either the raw `command_json` response (JSON) or a text view. The JSON
/// path forwards the command through the engine so the output is byte-identical
/// to what every language binding returns.
fn render(
    format: Format,
    cmd: &serde_json::Value,
    genome: &mut Genome,
    text: impl FnOnce() -> String,
) -> String {
    match format {
        Format::Json => {
            let mut out = genome.command_json(&cmd.to_string());
            out.push('\n');
            out
        }
        Format::Text => text(),
    }
}

/// The `--symbol` argument, required by `vector` and `similar`.
fn require_symbol(args: &Args) -> Result<String, String> {
    args.symbol
        .clone()
        .ok_or_else(|| format!("--symbol is required for --op {:?}", args.op))
}

/// The `--k` argument, required by `similar` and `cluster`.
fn require_k(args: &Args) -> Result<usize, String> {
    args.k
        .ok_or_else(|| format!("--k is required for --op {:?}", args.op))
}

/// Read and parse a spec file, choosing JSON or TOML by extension.
fn load_spec(path: &Path) -> Result<GenomeSpec, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("read spec {}: {e}", path.display()))?;
    let is_toml = path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("toml"));
    let cfg = if is_toml {
        Config::from_toml(&content)
    } else {
        Config::from_json(&content)
    };
    cfg.map(|c| c.spec).map_err(|e| e.to_string())
}

/// Load a universe from a directory of `<SYMBOL>.csv` files.
fn load_data_dir(dir: &Path) -> Result<BTreeMap<String, Vec<Candle>>, String> {
    let mut data = BTreeMap::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("read dir {}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("csv") {
            continue;
        }
        let symbol = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("bad file name: {}", path.display()))?
            .to_string();
        let content =
            fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        data.insert(symbol, parse_csv(&content)?);
    }
    Ok(data)
}

/// Load a universe as a JSON dataset (`{"SYMBOL": [candle, ...]}`) from stdin.
fn load_stdin() -> Result<BTreeMap<String, Vec<Candle>>, String> {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&buf).map_err(|e| format!("parse stdin dataset: {e}"))
}

/// Parse OHLCV rows (`ts,open,high,low,close,volume`) into candles; a
/// non-numeric first row is treated as a header and skipped.
fn parse_csv(content: &str) -> Result<Vec<Candle>, String> {
    let mut candles = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(str::trim).collect();
        if cols.len() < 6 {
            return Err(format!(
                "CSV line {}: expected 6 columns, got {}",
                idx + 1,
                cols.len()
            ));
        }
        let time = match cols[0].parse::<i64>() {
            Ok(t) => t,
            Err(_) if idx == 0 => continue, // header row
            Err(e) => return Err(format!("CSV line {}: bad timestamp: {e}", idx + 1)),
        };
        let field = |i: usize, name: &str| {
            cols[i]
                .parse::<f64>()
                .map_err(|e| format!("CSV line {}: {name}: {e}", idx + 1))
        };
        candles.push(Candle {
            time,
            open: field(1, "open")?,
            high: field(2, "high")?,
            low: field(3, "low")?,
            close: field(4, "close")?,
            volume: field(5, "volume")?,
        });
    }
    Ok(candles)
}

/// Render a symbol's feature vector as an axis/value table.
fn render_vector(vector: &Vector) -> String {
    let mut out = format!(
        "vector {}  (dim {}, {})\n",
        vector.symbol,
        vector.dim,
        if vector.ready { "ready" } else { "not ready" }
    );
    for (key, value) in vector.keys.iter().zip(&vector.values) {
        let shown = value.map_or_else(|| "null".to_string(), |v| format!("{v}"));
        let _ = writeln!(out, "  {key:<24}  {shown}");
    }
    out
}

/// Render the nearest-neighbor list.
fn render_neighbors(symbol: &str, neighbors: &[Neighbor]) -> String {
    if neighbors.is_empty() {
        return format!("no neighbors for {symbol}\n");
    }
    let mut out = format!("nearest neighbors of {symbol}\n");
    for n in neighbors {
        let _ = writeln!(out, "  {:<12}  {}", n.symbol, n.distance);
    }
    out
}

/// Render the clusters and their members.
fn render_clusters(clusters: &[Cluster]) -> String {
    if clusters.is_empty() {
        return "no clusters (empty ready universe)\n".to_string();
    }
    let mut out = format!("{} cluster(s)\n", clusters.len());
    for (i, c) in clusters.iter().enumerate() {
        let _ = writeln!(out, "  cluster {i}: {}", c.members.join(", "));
    }
    out
}

/// Render the anomaly scores (already sorted, biggest outlier first).
fn render_anomalies(anomalies: &[genome_core::Anomaly]) -> String {
    if anomalies.is_empty() {
        return "no anomalies (empty ready universe)\n".to_string();
    }
    let mut out = String::from("anomaly scores (biggest outlier first)\n");
    for a in anomalies {
        let _ = writeln!(out, "  {:<12}  {}", a.symbol, a.score);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_csv_with_a_header() {
        let csv = "ts,open,high,low,close,volume\n1,10,11,9,10.5,100\n2,10.5,12,10,11,200\n";
        let candles = parse_csv(csv).unwrap();
        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].time, 1);
        assert!((candles[1].close - 11.0).abs() < 1e-9);
    }

    #[test]
    fn parse_csv_rejects_a_short_row() {
        assert!(parse_csv("1,2,3\n").is_err());
    }

    #[test]
    fn render_vector_shows_null_axis() {
        let v = Vector {
            symbol: "AAA".into(),
            dim: 2,
            values: vec![Some(1.5), None],
            keys: vec!["price.close".into(), "Rsi(14)".into()],
            ready: false,
        };
        let text = render_vector(&v);
        assert!(text.contains("not ready"));
        assert!(text.contains("null"));
    }

    #[test]
    fn render_neighbors_lists_symbols() {
        let n = vec![Neighbor {
            symbol: "BBB".into(),
            distance: 0.5,
        }];
        assert!(render_neighbors("AAA", &n).contains("BBB"));
    }
}
