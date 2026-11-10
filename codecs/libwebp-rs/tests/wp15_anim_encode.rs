//! WP15 differential tests of the animation encoder (`src/mux/anim_encode.rs`)
//! against the C oracle.
//!
//! The probe `tools/oracle/probes/wp15/anim_enc.c` runs a script of encoder
//! commands against the C library; `wp15_support::anim_report` runs it through
//! the translation and must print the same lines: every result, error code,
//! error string, the number of progress-hook calls and the size and digest of
//! every assembled animation (byte-identical output). The one recorded
//! deviation is the hook that C calls again in `WebPAnimEncoderAssemble`
//! (`wp15_support::ANIM_DEVIATION_KEYS`; `src/mux/anim_encode.rs`,
//! `# Deviations from upstream`, item 2). The assembled animations are then
//! decoded with WP14's animation decoder and compared with the C decoder
//! (`tests/wp14_support::anim_report` against `probes/wp14/anim`).
//!
//! Frames are generated RGBA files: moving shapes over an opaque background,
//! over a transparent one whose pixels keep their colors (so that the lossless
//! cleanup of transparent pixels, which C performs through its view of the
//! canvas, matters to later candidates), over a translucent one and over
//! noise, at odd and even sizes, with repeated frames.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp15_support/mod.rs"]
mod wp15_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::path::PathBuf;

use oracle::Oracle;
use wp14_support::{AnimCommand, work_dir};
use wp15_support::{
    Case, Frames, anim_report, compare_anim, error_cases, keyframe_cases, matrix_cases,
    single_cases, version_script,
};

/// Runs the cases through the C probe and the translation, compares the
/// outputs and decodes every assembled animation with both animation
/// decoders. Returns the number of encoder commands.
fn run_cases(oracle: &Oracle, test: &str, cases: &[Case]) -> usize {
    let directory = work_dir(test);
    let frames = Frames {
        directory: directory.to_path_buf(),
    };
    let mut commands = 0;
    let mut deviations = 0;
    let mut outputs = Vec::new();
    for (number, case) in cases.iter().enumerate() {
        let output = directory.join(format!("out{number}.webp"));
        let script = case.script(&frames, &output);
        commands += script.lines().count();
        let result = oracle
            .probe("wp15", "anim_enc")
            .run::<&str>(&[], script.as_bytes());
        result.success();
        let c = String::from_utf8(result.stdout).unwrap();
        let rust = anim_report(&script);
        deviations += compare_anim(test, &case.name, &c, &rust);
        if output.exists() {
            outputs.push(output);
        }
    }
    decode_outputs(oracle, test, &outputs);
    eprintln!(
        "{test}: {} cases, {commands} commands, {} animations decoded, {deviations} assemble.hook deviations",
        cases.len(),
        outputs.len()
    );
    commands
}

/// Decodes the C and the Rust output of each case: the C file with the C
/// animation decoder (WP14's probe), the Rust file with the translation's
/// (`wp14_support::anim_report`); the files are equal, as the digests showed.
fn decode_outputs(oracle: &Oracle, test: &str, outputs: &[PathBuf]) {
    for path in outputs {
        let rust_path = PathBuf::from(format!("{}.rs", path.display()));
        assert_eq!(
            std::fs::read(path).unwrap(),
            std::fs::read(&rust_path).unwrap(),
            "{test}: {} differs",
            path.display()
        );
        for mode in [1, 8] {
            let command = AnimCommand {
                mode,
                threads: 0,
                reset: 0,
                dump: 0,
                path: path.clone(),
            };
            let c = oracle
                .probe("wp14", "anim")
                .run::<&str>(&[], command.line(false).as_bytes());
            c.success();
            let rust_command = AnimCommand {
                path: rust_path.clone(),
                ..command
            };
            let rust = wp14_support::anim_report(&rust_command);
            // The probe numbers its commands; the report has no such line.
            let c: String = String::from_utf8(c.stdout)
                .unwrap()
                .lines()
                .filter(|line| !line.starts_with("command="))
                .flat_map(|line| [line, "\n"])
                .collect();
            assert_eq!(c, rust, "{test}: decoding {} differs", path.display());
        }
    }
}

#[test]
fn option_and_config_matrix_matches_the_c_encoder() {
    let Some(oracle) = Oracle::locate("option_and_config_matrix_matches_the_c_encoder") else {
        return;
    };
    run_cases(&oracle, "wp15_anim_matrix", &matrix_cases());
}

#[test]
fn keyframe_options_match_the_c_encoder() {
    let Some(oracle) = Oracle::locate("keyframe_options_match_the_c_encoder") else {
        return;
    };
    run_cases(&oracle, "wp15_anim_keyframes", &keyframe_cases());
}

#[test]
fn timestamps_errors_and_hooks_match_the_c_encoder() {
    let Some(oracle) = Oracle::locate("timestamps_errors_and_hooks_match_the_c_encoder") else {
        return;
    };
    let directory = work_dir("wp15_anim_errors_files");
    run_cases(&oracle, "wp15_anim_errors", &error_cases(&directory));

    let script = version_script();
    let c = oracle
        .probe("wp15", "anim_enc")
        .run::<&str>(&[], script.as_bytes());
    c.success();
    compare_anim(
        "wp15_anim_versions",
        &script,
        &String::from_utf8(c.stdout).unwrap(),
        &anim_report(&script),
    );
}

#[test]
fn single_frames_and_chunks_match_the_c_encoder() {
    let Some(oracle) = Oracle::locate("single_frames_and_chunks_match_the_c_encoder") else {
        return;
    };
    run_cases(&oracle, "wp15_anim_single", &single_cases());
}
