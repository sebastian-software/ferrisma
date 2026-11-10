//! WP15 differential tests of the C ABI of `mux.h` (`src/abi/mux.rs`): the
//! probes `mux.c` and `anim_enc.c`, built once against the C library and once
//! against the Rust library (`mux_rs`, `anim_enc_rs`), run the same scripts
//! and must print the same standard output and error and exit alike. The
//! scripts are those of `tests/wp15_mux.rs` and `tests/wp15_anim_encode.rs`
//! plus the cases that exist only at the C ABI: `NULL` arguments and bytes,
//! data sizes above `MAX_CHUNK_PAYLOAD`, commands on a deleted mux or a
//! failed encoder, version mismatches, verbose encoders (their messages go to
//! standard error) and YUV frames, which the ABI converts per call.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp15_support/mod.rs"]
mod wp15_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::path::Path;

use oracle::{Oracle, Output};
use wp14_support::work_dir;
use wp15_support::{
    Case, Frames, animation_scripts, compare_anim, edit_scripts, error_cases, keyframe_cases,
    matrix_cases, single_cases, truncated_scripts, vector_scripts, version_script,
};

/// Runs `script` through the C-linked and the Rust-linked probe `name` and
/// compares their outputs; returns the number of `assemble.hook` lines where
/// C's encoder called the hook again (a recorded deviation, see
/// `src/mux/anim_encode.rs`, `# Deviations from upstream`, item 2).
fn check(oracle: &Oracle, name: &str, test: &str, script: &str) -> usize {
    let c: Output = oracle
        .probe("wp15", name)
        .run::<&str>(&[], script.as_bytes());
    let rust: Output = oracle
        .probe("wp15", &format!("{name}_rs"))
        .run::<&str>(&[], script.as_bytes());
    let context: String = script.lines().take(3).collect::<Vec<_>>().join(" | ");
    assert_eq!(
        c.status.code(),
        rust.status.code(),
        "{test}: exit statuses differ\n  in: {context}\n  C stderr: {}\n  Rust stderr: {}",
        String::from_utf8_lossy(&c.stderr),
        String::from_utf8_lossy(&rust.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&c.stderr),
        String::from_utf8_lossy(&rust.stderr),
        "{test}: standard error differs\n  in: {context}"
    );
    let (c_out, rust_out) = (
        String::from_utf8(c.stdout).unwrap(),
        String::from_utf8(rust.stdout).unwrap(),
    );
    if name == "anim_enc" {
        compare_anim(test, &context, &c_out, &rust_out)
    } else {
        wp15_support::compare(test, &context, &c_out, &rust_out);
        0
    }
}

fn check_scripts(oracle: &Oracle, name: &str, test: &str, scripts: &[String]) -> usize {
    let mut commands = 0;
    for batch in scripts.chunks(16) {
        let script = batch.concat();
        commands += script.lines().count();
        check(oracle, name, test, &script);
    }
    commands
}

#[test]
fn abi_mux_matches_the_c_library() {
    let Some(oracle) = Oracle::locate("abi_mux_matches_the_c_library") else {
        return;
    };
    let directory = work_dir("wp15_abi_mux");
    let mut scripts = vector_scripts();
    scripts.extend(animation_scripts(&oracle, &directory));
    scripts.extend(truncated_scripts(&directory));
    scripts.extend(edit_scripts(&directory));
    let commands = check_scripts(&oracle, "mux", "abi_mux", &scripts);
    eprintln!(
        "abi_mux_matches_the_c_library: {} scripts, {commands} commands",
        scripts.len()
    );
}

/// The scripts that only the C ABI can run.
fn abi_only_mux_scripts(directory: &Path) -> Vec<String> {
    let image = wp15_support::file_token(
        directory,
        "image.webp",
        &std::fs::read(oracle::test_data().join("lossy_alpha1.webp")).unwrap(),
    );
    let mut scripts = vec![
        "version\nnulls\n".to_owned(),
        // NULL bytes and sizes above MAX_CHUNK_PAYLOAD, refused before a read.
        format!(
            "new\nsetchunk ICCP 0 null\nsetchunk ICCP 1 huge\nsetchunk abcd 0 huge\n\
             setimage 0 null\nsetimage 1 huge\npushframe 0 0 0 10 3 0 0 null\n\
             pushframe 1 0 0 10 3 0 0 huge\nsetimage 0 {image}\nsetchunk null 1 hex:00\n\
             getchunk null\ndelchunk null\nstate\nassemble 1\n"
        ),
        "create 0 null\ncreate 1 hex:\nstate\n".to_owned(),
        // Every command on a deleted mux.
        format!(
            "new\nsetimage 1 {image}\ndelete\nsetchunk ICCP 1 hex:01\ngetchunk ICCP\n\
             delchunk ICCP\nsetimage 1 {image}\npushframe 1 0 0 10 3 0 0 {image}\n\
             getframe 1 0\ndelframe 1\nsetanim 0 0\ngetanim\nsetcanvas 4 4\ngetcanvas\n\
             features\nnumchunks 1\nassemble 0\nstate\n"
        ),
        // Version checks.
        format!("newversion 0\nnewversion 512\nnewversion 265\ncreateversion 0 0 {image}\nstate\n"),
        // Chunk ids outside the enum.
        format!("create 1 {image}\nnumchunks -1\nnumchunks 99\nnumchunks 4\nnumchunks 10\n"),
    ];
    // Frames of every id, dispose and blend value, through the mirror.
    for id in [0, 3, 6, 10, 99] {
        scripts.push(format!(
            "new\npushframe 1 4 6 50 {id} 0 1 {image}\npushframe 1 1 1 50 3 1 0 {image}\n\
             pushframe 0 3 3 70 3 7 9 {image}\nstate\nassemble 1\n"
        ));
    }
    scripts
}

#[test]
fn abi_mux_answers_null_and_invalid_arguments_as_c() {
    let Some(oracle) = Oracle::locate("abi_mux_answers_null_and_invalid_arguments_as_c") else {
        return;
    };
    let directory = work_dir("wp15_abi_mux_nulls");
    let scripts = abi_only_mux_scripts(&directory);
    let commands = check_scripts(&oracle, "mux", "abi_mux_nulls", &scripts);
    eprintln!("abi_mux_answers_null_and_invalid_arguments_as_c: {commands} commands");
}

/// The scripts of encoder cases, with their outputs in `directory`.
fn anim_scripts(directory: &Path, cases: &[Case]) -> Vec<String> {
    let frames = Frames {
        directory: directory.to_path_buf(),
    };
    cases
        .iter()
        .enumerate()
        .map(|(number, case)| case.script(&frames, &directory.join(format!("out{number}.webp"))))
        .collect()
}

#[test]
fn abi_animation_encoder_matches_the_c_library() {
    let Some(oracle) = Oracle::locate("abi_animation_encoder_matches_the_c_library") else {
        return;
    };
    let directory = work_dir("wp15_abi_anim");
    let mut cases = matrix_cases();
    cases.extend(keyframe_cases());
    cases.extend(error_cases(&directory));
    cases.extend(single_cases());
    let scripts = anim_scripts(&directory, &cases);
    let mut commands = 0;
    let mut deviations = 0;
    for script in &scripts {
        commands += script.lines().count();
        deviations += check(&oracle, "anim_enc", "abi_anim", script);
    }
    eprintln!(
        "abi_animation_encoder_matches_the_c_library: {} cases, {commands} commands, {deviations} assemble.hook deviations",
        scripts.len()
    );
}

#[test]
fn abi_animation_encoder_answers_null_and_verbose_as_c() {
    let Some(oracle) = Oracle::locate("abi_animation_encoder_answers_null_and_verbose_as_c") else {
        return;
    };
    let directory = work_dir("wp15_abi_anim_nulls");
    let mut scripts = vec![version_script(), "nulls\n".to_owned()];
    // Commands on an encoder that New refused.
    scripts.push(
        "new 0 0\naddnull 0\nassemble -\nsetchunk ICCP 1 hex:01\ngetchunk ICCP\ndelchunk ICCP\n"
            .to_owned(),
    );
    // Verbose encoders: the warnings of SanitizeEncoderOptions and the frame
    // messages of FlushFrames and of the YUV conversion go to standard error.
    let mut verbose = Vec::new();
    for (kmin, kmax) in [(0, 5), (7, 5), (1, 40), (3, 3), (-2, -2), (0, 1), (20, 100)] {
        for minimize in [0, 1] {
            let mut case = Case::new(
                &format!("verbose kmin {kmin} kmax {kmax}"),
                20,
                16,
                wp15_support::Background::Half,
                6,
            );
            case.options = format!("options {minimize} {kmin} {kmax} 1 1 0 0");
            case.argb = vec![1, 0];
            verbose.push(case);
        }
    }
    scripts.extend(anim_scripts(&directory, &verbose));
    let mut commands = 0;
    for script in &scripts {
        commands += script.lines().count();
        check(&oracle, "anim_enc", "abi_anim_nulls", script);
    }
    eprintln!("abi_animation_encoder_answers_null_and_verbose_as_c: {commands} commands");
}
