//! Minimal M1 CLI; the implementation is `ferrisma::cli` (ADR-0024).

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    ferrisma::cli::main(&args, None)
}
