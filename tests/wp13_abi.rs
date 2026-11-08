//! WP13 differential test of the C ABI: the probes of `tools/oracle/probes/wp13/`
//! built against the C library and against the Rust static library through
//! `src/abi/encode.rs` (`probes/wp13/<name>_rs`, the consumer build of
//! `tools/oracle/probes/wp13/oracle.cmake`) read the same input, and the outputs
//! must be identical, byte for byte.
//!
//! - `encode`: the whole encoder matrix of `tests/wp13_encode.rs`
//!   (`support/wp13_matrix.rs`) through `WebPEncode`, with library memory,
//!   caller memory and views (whose source the C ABI writes through, as C
//!   does), `WebPMemoryWriter` and custom writers, progress hooks and aborts.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp13_cases.rs"]
mod wp13_cases;
#[path = "support/wp13_matrix.rs"]
mod wp13_matrix;

use oracle::{Fields, Oracle};
use wp13_cases::{Case, records, split_records};
use wp13_matrix as matrix;

/// Runs the probe `name` and its Rust-linked build on `input` and compares the
/// outputs line by line.
fn compare_probe(test: &str, oracle: &Oracle, name: &str, input: &[u8]) -> Fields {
    let c = oracle.probe("wp13", name).run::<&str>(&[], input);
    c.success();
    let rust = oracle
        .probe("wp13", &format!("{name}_rs"))
        .run::<&str>(&[], input);
    rust.success();
    let (c_fields, rust_fields) = (c.fields(), rust.fields());
    for (index, (a, b)) in c_fields.0.iter().zip(&rust_fields.0).enumerate() {
        assert_eq!(a, b, "{test}: line {index} differs");
    }
    assert_eq!(
        c_fields.0.len(),
        rust_fields.0.len(),
        "{test}: the outputs have different lengths"
    );
    assert_eq!(
        c.stderr, rust.stderr,
        "{test}: the probes wrote different diagnostics"
    );
    c_fields
}

/// Runs one group of the encoder matrix through both builds of `encode`.
fn run_encode(test: &str, cases: impl FnOnce(&Oracle) -> Vec<Case>) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let cases = cases(&oracle);
    let fields = compare_probe(test, &oracle, "encode", &records(&cases));
    let outputs = split_records(fields);
    assert_eq!(outputs.len(), cases.len(), "{test}: one output per record");
    let encoded = outputs.iter().filter(|f| f.get("ret") == Some("1")).count();
    eprintln!("{test}: {} cases, {encoded} encoded", cases.len());
}

#[test]
fn abi_lossy_quality_method_and_preset_match_c() {
    run_encode("abi_lossy_quality_method_and_preset_match_c", |_| {
        matrix::lossy_quality_method_and_preset_cases()
    });
}

#[test]
fn abi_lossy_tools_match_c() {
    run_encode("abi_lossy_tools_match_c", |_| matrix::lossy_tools_cases());
}

#[test]
fn abi_lossy_alpha_matches_c() {
    run_encode("abi_lossy_alpha_matches_c", |_| matrix::lossy_alpha_cases());
}

#[test]
fn abi_lossless_matches_c() {
    run_encode("abi_lossless_matches_c", |_| matrix::lossless_cases());
}

#[test]
fn abi_rate_control_matches_c() {
    run_encode("abi_rate_control_matches_c", |_| {
        matrix::rate_control_cases()
    });
}

#[test]
fn abi_progress_aborts_and_writers_match_c() {
    run_encode("abi_progress_aborts_and_writers_match_c", |_| {
        matrix::progress_aborts_and_writers_cases()
    });
}

#[test]
fn abi_views_and_caller_memory_match_c() {
    run_encode("abi_views_and_caller_memory_match_c", |_| {
        matrix::views_and_caller_memory_cases()
    });
}

#[test]
fn abi_dimensions_and_invalid_arguments_match_c() {
    run_encode("abi_dimensions_and_invalid_arguments_match_c", |_| {
        matrix::dimensions_and_invalid_arguments_cases()
    });
}

#[test]
fn abi_random_configurations_match_c() {
    run_encode("abi_random_configurations_match_c", |_| {
        matrix::random_configurations_cases()
    });
}

#[test]
fn abi_vectors_match_c() {
    run_encode("abi_vectors_match_c", matrix::vector_cases);
}
