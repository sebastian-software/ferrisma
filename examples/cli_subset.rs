//! The `ferrisma` command line with any feature subset, for benchmarks on
//! targets where the full `cli` feature set does not build (the generated
//! libavif/AOM code supports only macOS ARM64 and GNU Linux x86-64).
//!
//! It runs exactly what `src/bin/ferrisma.rs` runs; formats whose features
//! are not enabled report `FeatureDisabled`. Example for Linux ARM64:
//!
//! ```sh
//! cargo build --release -p ferrisma --example cli_subset \
//!   --features jpeg-read,webp-read,webp-write,jpeg-xl-read,jpeg-xl-own-read,jpeg-xl-write,avif-write,icc
//! ```

use std::error::Error;
use std::process::ExitCode;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    ferrisma::cli::run(&args, None)
}
