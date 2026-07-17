//! The `wickra-genome` reference CLI.
//!
//! Loads a `GenomeSpec` and a universe of candles (a directory of `<SYMBOL>.csv`
//! files or a JSON dataset on stdin), builds the market genome through
//! `genome-core`, runs one query (`vector`, `similar`, `cluster` or `anomaly`)
//! and prints the answer as text or as the raw `command_json` response.

mod args;
mod run;

use args::Args;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    match run::run(&args) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("wickra-genome: {err}");
            ExitCode::FAILURE
        }
    }
}
