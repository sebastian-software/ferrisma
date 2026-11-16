//! Ferrisma's command line with libavif-rs as the encoder (ADR-0024): the
//! same arguments, report line, error output and exit status as `ferrisma`, with `--avif-quality=`,
//! `--avif-alpha-quality=`, `--avif-speed=`, `--avif-depth=` and
//! `--avif-chroma=` on libavif's scales.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    ferrisma::cli::main(&args, Some(&ferrisma_libavif::LibavifBackend))
}
