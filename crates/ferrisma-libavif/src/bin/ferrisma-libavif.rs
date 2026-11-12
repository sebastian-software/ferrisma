//! Ferrisma's command line with libavif-rs as the encoder (ADR-0024): the
//! same arguments and report line as `ferrisma`, with `--speed` and
//! `--quality` on libavif's scales.

use std::error::Error;
use std::process::ExitCode;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    ferrisma::cli::run(&args, Some(&ferrisma_libavif::LibavifBackend::default()))
}
