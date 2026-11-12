//! Minimal M1 CLI; the implementation is `ferrisma::cli` (ADR-0024).

use std::error::Error;
use std::process::ExitCode;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    ferrisma::cli::run(&args, None)
}
