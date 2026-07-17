//! CLI argument parsing.

use clap::{ArgGroup, Parser, ValueEnum};
use std::path::PathBuf;

/// Query the market genome over a spec and a universe of candles.
#[derive(Parser, Debug)]
#[command(name = "wickra-genome", version, about)]
#[command(group(ArgGroup::new("source").required(true).args(["data", "stdin"])))]
pub struct Args {
    /// Path to the genome spec (JSON or TOML, chosen by file extension).
    #[arg(long)]
    pub spec: PathBuf,

    /// Directory of per-symbol CSV candle files (`<SYMBOL>.csv`).
    #[arg(long)]
    pub data: Option<PathBuf>,

    /// Read the universe as a JSON dataset from standard input instead.
    #[arg(long)]
    pub stdin: bool,

    /// Which query to run over the vector space.
    #[arg(long, value_enum)]
    pub op: Op,

    /// The query symbol (required by `vector` and `similar`).
    #[arg(long)]
    pub symbol: Option<String>,

    /// The neighbor / cluster count (required by `similar` and `cluster`).
    #[arg(long)]
    pub k: Option<usize>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
}

/// The query to run.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Op {
    /// The self-describing feature vector of one symbol.
    Vector,
    /// The k nearest neighbors of one symbol.
    Similar,
    /// The seeded k-means clustering of the ready universe.
    Cluster,
    /// Each ready symbol's nearest-neighbor anomaly score.
    Anomaly,
}

/// The output format.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Format {
    /// A human-readable rendering.
    Text,
    /// The raw `command_json` response.
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn arg_config_is_valid() {
        Args::command().debug_assert();
    }

    #[test]
    fn parses_a_similar_query() {
        let args = Args::try_parse_from([
            "wickra-genome",
            "--spec",
            "s.json",
            "--data",
            "dir",
            "--op",
            "similar",
            "--symbol",
            "AAA",
            "--k",
            "2",
        ])
        .unwrap();
        assert_eq!(args.op, Op::Similar);
        assert_eq!(args.symbol.as_deref(), Some("AAA"));
        assert_eq!(args.k, Some(2));
        assert_eq!(args.format, Format::Text);
    }

    #[test]
    fn data_and_stdin_conflict() {
        assert!(Args::try_parse_from([
            "wickra-genome",
            "--spec",
            "s.json",
            "--op",
            "anomaly",
            "--data",
            "d",
            "--stdin",
        ])
        .is_err());
    }

    #[test]
    fn a_source_is_required() {
        assert!(
            Args::try_parse_from(["wickra-genome", "--spec", "s.json", "--op", "anomaly"]).is_err()
        );
    }
}
