// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
#[path = "../common/default_cms_cases.rs"]
mod cases;
#[path = "../common/default_cms.rs"]
mod observation;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("default-cms-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    observation::observe(&format!(
        "{kind} {}",
        args.clone().collect::<Vec<_>>().join(" ")
    ))
}
fn compare(family: &str, private: bool, requests: Vec<String>) {
    cases::capture(family, &requests);
    let name = if private { "private" } else { "public" };
    let variable = if private {
        "JXL_DEFAULT_CMS_PRIVATE_ORACLE"
    } else {
        "JXL_DEFAULT_CMS_ORACLE"
    };
    let executable = std::env::var_os(variable)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::var_os("JXL_CPP_ORACLE")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
                })
                .parent()
                .unwrap()
                .join(format!("default_cms_{name}"))
        });
    let mut child = Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let text = requests.join("\n") + "\n";
    let writer = std::thread::spawn(move || input.write_all(text.as_bytes()));
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap().unwrap();
    assert!(
        output.status.success(),
        "actual {name} CMS failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "actual CMS diagnostic: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let rows: Vec<_> = text.lines().collect();
    assert_eq!(rows.len(), requests.len(), "{family} observation count");
    for (index, (request, cpp)) in requests.iter().zip(rows).enumerate() {
        assert_eq!(
            observation::observe(request),
            cpp.trim_end(),
            "{family} case {index}"
        );
    }
    println!("{} C++/Rust differential cases matched", requests.len());
}
#[test]
fn default_cms_interface_guards_and_every_original_profile() {
    compare(
        "guards",
        false,
        cases::guards(&super::skcms_parser::cases::original_profiles()),
    );
}
#[test]
fn default_cms_private_every_original_profile_metadata() {
    compare(
        "detail",
        true,
        cases::detail(&super::skcms_parser::cases::original_profiles()),
    );
}
#[test]
fn default_cms_matrix_checkpoints_and_actual_aliasing() {
    compare("matrix", true, cases::matrix());
}
#[test]
fn default_cms_exceptional_matrix_coefficients_inputs_and_aliases() {
    compare("matrix-exceptional", true, cases::matrix_exceptional());
}
#[test]
fn default_cms_profile_hdr_rows_and_context_lifecycle() {
    compare(
        "rows",
        false,
        cases::rows(&super::skcms_parser::cases::original_profiles()),
    );
}
#[test]
fn default_cms_complete_cicp_byte_domains_and_partial_outputs() {
    compare("cicp", true, cases::cicp());
}
#[test]
fn default_cms_original_ndebug_hlg_exceptional_values() {
    compare("exceptional", true, cases::exceptional());
}
#[test]
fn default_cms_dense_transfer_ootf_and_source_defined_no_access_paths() {
    compare("tf-domain", true, cases::tf_domain());
}
#[test]
fn default_cms_genuinely_unknown_unreached_profile_words() {
    compare("prefixes", true, cases::prefixes());
}
#[test]
fn default_cms_init_prefix_guards_and_unwritten_failure_outputs() {
    compare("init-prefixes", false, cases::init_prefixes());
}
#[test]
fn default_cms_callback_data_function_and_descriptor_reload_order() {
    compare("callbacks", false, cases::callbacks());
}
#[test]
fn default_cms_private_numeric_helpers_and_complete_known_values() {
    compare("numeric", true, cases::numeric());
}
#[test]
fn default_cms_repeated_aligned_allocations_and_original_unsigned_sizes() {
    compare("allocation", true, cases::allocation());
}
#[test]
fn default_cms_private_luminance_mixed_precision_and_no_write_exits() {
    compare(
        "luminances",
        true,
        cases::luminances(&super::skcms_parser::cases::original_profiles()),
    );
}
#[test]
fn default_cms_profile_reconstruction_equality() {
    compare(
        "profile-equality",
        true,
        cases::profile_equality(&super::skcms_parser::cases::original_profiles()),
    );
}
#[test]
fn default_cms_every_synthetic_complete_profile_field_admission() {
    compare(
        "synthetic-fields",
        false,
        cases::fields_for_profiles(super::skcms_parser::cases::synthetic_profiles()),
    );
}
#[test]
fn default_cms_malformed_profile_field_admission_and_partial_state() {
    compare(
        "malformed-fields",
        false,
        cases::fields_for_parser_requests(
            super::skcms_parser::cases::malformed_and_truncated_requests(),
        ),
    );
}
