// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
#[path = "../common/skcms_parser.rs"]
pub(super) mod cases;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("skcms-profile-") || kind.starts_with("skcms-curve-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    cases::observe(&format!(
        "{kind} {}",
        args.clone().collect::<Vec<_>>().join(" ")
    ))
}
#[test]
fn skcms_parser_layout_and_immutable_profiles() {
    compare(vec![
        "skcms-profile-layout".into(),
        "skcms-profile-static 0".into(),
        "skcms-profile-static 1".into(),
    ]);
}
#[test]
fn skcms_parser_every_original_icc_profile() {
    compare(cases::genuine_requests());
}
#[test]
fn skcms_parser_generated_complete_lut_and_curve_domains() {
    compare(cases::synthetic_requests());
}
#[test]
fn skcms_parser_malformed_headers_and_structure_truncations() {
    compare(cases::malformed_and_truncated_requests());
}
fn compare(requests: Vec<String>) {
    cases::capture(&requests);
    super::compare(requests);
}
fn compare_private(requests: Vec<String>) {
    cases::capture(&requests);
    let original = std::env::var_os("JXL_CPP_ORACLE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
        });
    let executable = original.parent().unwrap().join("skcms_parser_private");
    assert!(executable.is_file(), "private original helper missing");
    let mut child = Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let lines = requests.join("\n") + "\n";
    let writer = std::thread::spawn(move || input.write_all(lines.as_bytes()));
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "private source failure: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    writer.join().unwrap().unwrap();
    let observed = String::from_utf8(output.stdout).unwrap();
    let observed: Vec<_> = observed.lines().collect();
    assert_eq!(requests.len(), observed.len());
    for (request, cpp) in requests.iter().zip(observed) {
        assert_eq!(cases::observe(request), cpp, "request: {request}");
    }
    println!("{} C++/Rust differential cases matched", requests.len());
}
#[test]
fn skcms_parser_private_eval_linear_aliases_and_identity() {
    compare_private(cases::curve_requests());
}
#[test]
fn skcms_parser_private_curve_read_partial_writes_and_null_guards() {
    compare_private(cases::read_curve_requests());
}
#[test]
fn skcms_parser_all_original_public_priority_lists() {
    compare(cases::priority_requests());
}
#[test]
fn skcms_parser_tag_boundary_first_match_chad_whitepoint_and_channels() {
    compare(cases::getter_requests());
}
#[test]
fn skcms_parser_exact_zeroing_setter_self_aliases_and_null_getters() {
    compare(cases::lifecycle_requests());
}
#[test]
fn skcms_parser_large_count_defined_reached_indices_and_guards() {
    compare_private(cases::large_defined_controls());
}
