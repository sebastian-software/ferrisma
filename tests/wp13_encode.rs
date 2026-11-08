//! WP13 differential test: `WebPEncode` (`src/enc/webp_enc.c`) over the
//! encoder matrix of `docs/PORT_PLAN.md` (Phase 3). The C probe
//! `tools/oracle/probes/wp13/encode.c` imports each seeded picture with the
//! oracle, optionally moves it into caller memory or takes a view of it, and
//! encodes it with `WebPEncode`; the Rust test does the same with the
//! translation (`webp_encode`). Compared per case: `WebPValidateConfig`, the
//! return value, the error code, every progress report, every writer call and
//! the bytes written (or the `WebPMemoryWriter`), all `WebPAuxStats` fields
//! (floats as bit patterns), the `extra_info` map with a guard byte, and the
//! picture afterwards (planes that the conversion or `show_compressed`
//! produced or the transparent-area cleanup changed), and the caller's buffers
//! with their padding. A view's source is compared separately: the
//! translation leaves it untouched where C writes through the view
//! (`docs/status/wp13.md`, deviation 1). The case groups are in
//! `support/wp13_matrix.rs`; `tests/wp13_abi.rs` runs them through the C ABI.
//!
//! Excluded, as `docs/status/wp10.md` records (undefined behavior in C): the
//! statistics of pictures with alpha and `alpha_compression` 0, the statistics
//! of threaded alpha encodings that fail, and `extra_info_type` 6.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp13_cases.rs"]
mod wp13_cases;
#[path = "support/wp13_matrix.rs"]
mod wp13_matrix;

use oracle::{Fields, Oracle};
use wp13_cases::{Alpha, Case, Content, Rng, records, replay, split_records};
use wp13_matrix as matrix;

/// Runs `cases` through the probe and the Rust translation and compares them.
fn compare(test: &str, oracle: &Oracle, cases: &[Case]) -> Vec<Fields> {
    let output = oracle
        .probe("wp13", "encode")
        .run::<&str>(&[], &records(cases));
    output.success();
    let expected = split_records(output.fields());
    assert_eq!(expected.len(), cases.len(), "{test}: one output per record");
    let outputs = expected.clone();
    let mut encoded = 0;
    let mut source_changed = 0;
    for (case, mut c) in cases.iter().zip(expected) {
        let (rust, source) = replay(case);
        if let Some((before, after)) = source {
            // A view: C writes through it into the source picture where
            // WebPEncode writes into the planes; the translation gives the view
            // its own planes, and the source stays as it was (deviation 1).
            assert_eq!(
                before,
                after,
                "{test}: the Rust source changed: {}",
                case.describe()
            );
            let c_source = c.get("source").expect("source=").to_owned();
            if c_source != before {
                source_changed += 1;
            }
            c.0.retain(|(key, _)| key != "source");
        }
        if c != rust {
            let keys: Vec<_> =
                c.0.iter()
                    .zip(&rust.0)
                    .filter(|(a, b)| a != b)
                    .map(|(a, _)| a.0.clone())
                    .collect();
            panic!(
                "{test}: case {} differs in {keys:?}\nC:    {:?}\nRust: {:?}",
                case.describe(),
                c.0.iter()
                    .filter(|(k, _)| keys.contains(k))
                    .collect::<Vec<_>>(),
                rust.0
                    .iter()
                    .filter(|(k, _)| keys.contains(k))
                    .collect::<Vec<_>>()
            );
        }
        if rust.get("ret") == Some("1") {
            encoded += 1;
        }
    }
    eprintln!(
        "{test}: {} cases, {encoded} encoded, {source_changed} view sources changed by C",
        cases.len()
    );
    outputs
}

/// Runs one group of the matrix; returns the cases and the probe's outputs.
fn run(test: &str, cases: impl FnOnce(&Oracle) -> Vec<Case>) -> Option<(Vec<Case>, Vec<Fields>)> {
    let oracle = Oracle::locate(test)?;
    let cases = cases(&oracle);
    let outputs = compare(test, &oracle, &cases);
    Some((cases, outputs))
}

#[test]
fn lossy_quality_method_and_preset_match_the_c_encoder() {
    run(
        "lossy_quality_method_and_preset_match_the_c_encoder",
        |_| matrix::lossy_quality_method_and_preset_cases(),
    );
}

#[test]
fn lossy_tools_match_the_c_encoder() {
    run("lossy_tools_match_the_c_encoder", |_| {
        matrix::lossy_tools_cases()
    });
}

#[test]
fn lossy_alpha_matches_the_c_encoder() {
    run("lossy_alpha_matches_the_c_encoder", |_| {
        matrix::lossy_alpha_cases()
    });
}

#[test]
fn lossless_matches_the_c_encoder() {
    run("lossless_matches_the_c_encoder", |_| {
        matrix::lossless_cases()
    });
}

#[test]
fn rate_control_matches_the_c_encoder() {
    run("rate_control_matches_the_c_encoder", |_| {
        matrix::rate_control_cases()
    });
}

#[test]
fn progress_aborts_and_writers_match_the_c_encoder() {
    let test = "progress_aborts_and_writers_match_the_c_encoder";
    let Some((cases, outputs)) = run(test, |_| matrix::progress_aborts_and_writers_cases()) else {
        return;
    };
    // Every abort stops at its call with VP8_ENC_ERROR_USER_ABORT, and in each
    // configuration an abort after the last call finishes the encoding, so the
    // aborts cover every reported step.
    let mut steps = Vec::new();
    for (case, c) in cases.iter().zip(&outputs) {
        if case.abort_at < 0 {
            continue;
        }
        let reports = c.get("progress").unwrap().split(',').count();
        if c.get("ret") == Some("0") && c.get("error") == Some("10") {
            assert_eq!(
                reports,
                case.abort_at as usize + 1,
                "{test}: {}",
                case.describe()
            );
        } else if case.abort_at == matrix::ABORT_STEPS - 1 {
            assert_eq!(c.get("ret"), Some("1"), "{test}: {}", case.describe());
            assert!(
                reports < matrix::ABORT_STEPS as usize,
                "{test}: {}",
                case.describe()
            );
            steps.push(reports);
        }
    }
    assert_eq!(
        steps.len(),
        6,
        "{test}: each configuration finished after its last report"
    );
    eprintln!("{test}: progress reports per configuration: {steps:?}");
}

#[test]
fn views_and_caller_memory_match_the_c_encoder() {
    run("views_and_caller_memory_match_the_c_encoder", |_| {
        matrix::views_and_caller_memory_cases()
    });
}

#[test]
fn dimensions_and_invalid_arguments_match_the_c_encoder() {
    run(
        "dimensions_and_invalid_arguments_match_the_c_encoder",
        |_| matrix::dimensions_and_invalid_arguments_cases(),
    );
}

#[test]
fn random_configurations_match_the_c_encoder() {
    run("random_configurations_match_the_c_encoder", |_| {
        matrix::random_configurations_cases()
    });
}

#[test]
fn vectors_match_the_c_encoder() {
    run("vectors_match_the_c_encoder", matrix::vector_cases);
}

/// The replay prints the probe's keys in the probe's order (Rust only).
#[test]
fn replay_prints_the_probe_keys() {
    let mut rng = Rng(0x130a);
    let mut c = matrix::case(&mut rng, Content::Photo, Alpha::Binary, 9, 9);
    c.extra_info_type = 1;
    let (fields, _) = replay(&c);
    assert_eq!(
        fields.keys(),
        [
            "valid", "ret", "error", "progress", "writes", "out", "stats", "extra", "pic"
        ]
    );
}
