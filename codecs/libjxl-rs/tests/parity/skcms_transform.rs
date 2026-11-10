// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
#[path = "../common/skcms_transform.rs"]
mod cases;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("skcms-transform")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    cases::observe(&format!(
        "{kind} {}",
        args.clone().collect::<Vec<_>>().join(" ")
    ))
}
#[test]
fn skcms_transform_all_baseline_formats_and_vector_tails() {
    compare_public(cases::format_requests());
}
#[test]
fn skcms_transform_complete_cicp_and_static_profile_combinations() {
    compare_public(cases::color_requests());
}
#[test]
fn skcms_transform_genuinely_unknown_inactive_profile_bytes() {
    compare_public(
        (0..16)
            .map(|i| format!("skcms-transform-unknown {i}"))
            .collect(),
    );
}

#[test]
fn skcms_transform_same_address_and_partial_overlap() {
    compare_public(cases::alias_requests());
}
#[test]
fn skcms_transform_unnamed_enum_values_and_original_size_wrap_guards() {
    compare_public(cases::guard_requests());
}
#[test]
fn skcms_transform_every_original_icc_in_both_directions() {
    compare_public(cases::icc_requests(
        super::skcms_parser::cases::original_profiles(),
    ));
}
#[test]
fn skcms_transform_every_generated_complete_lut_and_curve_profile() {
    compare_public(cases::icc_requests(
        super::skcms_parser::cases::synthetic_profiles(),
    ));
}
#[test]
fn skcms_transform_destination_usability_and_exact_partial_writes() {
    compare_public(cases::usability_requests(
        super::skcms_parser::cases::original_profiles(),
    ));
}
#[test]
fn skcms_transform_profile_equality_and_cmyk_distinction() {
    compare_public(cases::equal_requests(
        super::skcms_parser::cases::original_profiles(),
    ));
}
#[test]
fn skcms_transform_all_original_profile_trc_inverse_checks() {
    compare_public(cases::inverse_requests(
        super::skcms_parser::cases::original_profiles(),
    ));
}

fn compare_public(requests: Vec<String>) {
    cases::capture(&requests);
    super::compare(requests);
}
fn compare_private(requests: Vec<String>) {
    cases::capture(&requests);
    let binary = std::env::var_os("JXL_SKCMS_TRANSFORM_ORACLE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::var_os("JXL_CPP_ORACLE")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
                })
                .parent()
                .unwrap()
                .join("skcms_transform_private")
        });
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let text = requests.join("\n") + "\n";
    let writer = std::thread::spawn(move || input.write_all(text.as_bytes()));
    let result = child.wait_with_output().unwrap();
    writer.join().unwrap().unwrap();
    assert!(
        result.status.success(),
        "private source failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(result.stderr.is_empty());
    let text = String::from_utf8(result.stdout).unwrap();
    let rows: Vec<_> = text.lines().collect();
    assert_eq!(rows.len(), requests.len());
    for (q, row) in requests.iter().zip(rows) {
        assert_eq!(cases::observe(q), row, "request: {q}");
    }
    println!("{} C++/Rust differential cases matched", requests.len());
}
#[test]
fn skcms_transform_private_complete_curve_selection_and_exact_fusion() {
    compare_private(cases::selection_requests());
}
#[test]
fn skcms_transform_private_every_profile_preparation_and_partial_output() {
    let mut profiles = super::skcms_parser::cases::original_profiles();
    profiles.extend(super::skcms_parser::cases::synthetic_profiles());
    compare_private(cases::prep_requests(profiles));
}
#[test]
fn skcms_transform_private_all_format_widths_and_cicp_functions() {
    let mut rows: Vec<_> = (0..64)
        .map(|i| format!("skcms-transform-private-bytes {i}"))
        .collect();
    rows.extend([
        "skcms-transform-private-cicp 0".into(),
        "skcms-transform-private-cicp 1".into(),
    ]);
    compare_private(rows);
}
#[test]
fn skcms_transform_private_every_work_stage_and_original_clut_shapes() {
    compare_private(cases::program_requests());
}
#[test]
fn skcms_transform_all_65536_half_words_through_original_baseline() {
    compare_public(cases::half_domain_requests());
}
#[test]
fn skcms_transform_normalized_integer_halfway_neighbors() {
    compare_public(cases::normalized_integer_rounding_requests());
}

#[test]
fn skcms_transform_private_genuinely_unknown_function_and_grid_prefixes() {
    compare_private(cases::partial_context_requests());
}
