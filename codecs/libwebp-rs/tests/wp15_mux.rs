//! WP15 differential tests of the mux (`src/mux/muxedit.rs`, `muxinternal.rs`,
//! `muxread.rs`) against the C oracle.
//!
//! The probe `tools/oracle/probes/wp15/mux.c` runs a script of mux commands
//! against the C library; `wp15_support::mux_report` runs the same script
//! through the translation and must print the same lines: every return code,
//! every output (with the probe's marked output values where C leaves them),
//! and the size, `NULL` flag and digest of every byte string, with the full
//! bytes for chosen cases. The inputs are the pinned test vectors, WP14's
//! hand-assembled animations and `img2webp` animations, their truncations and
//! bit flips, and seeded edit sequences from scratch over generated
//! bitstreams, WebP files and chunk payloads.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp15_support/mod.rs"]
mod wp15_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use oracle::Oracle;
use wp8_support::digest;
use wp14_support::work_dir;
use wp15_support::{
    animation_scripts, compare, edit_scripts, mux_report, truncated_scripts, vector_scripts,
};

/// Runs `script` through the C probe and the translation and compares them.
fn check(oracle: &Oracle, test: &str, script: &str) -> usize {
    let output = oracle
        .probe("wp15", "mux")
        .run::<&str>(&[], script.as_bytes());
    output.success();
    let c = String::from_utf8(output.stdout).unwrap();
    let rust = mux_report(script);
    let context: String = script.lines().take(3).collect::<Vec<_>>().join(" | ");
    compare(test, &context, &c, &rust);
    script.lines().count()
}

/// Runs the scripts in batches of commands, so that one probe process runs
/// several of them; each batch starts from a fresh mux.
fn check_all(oracle: &Oracle, test: &str, scripts: &[String]) -> usize {
    let mut commands = 0;
    for batch in scripts.chunks(16) {
        commands += check(oracle, test, &batch.concat());
    }
    commands
}

#[test]
fn version_matches_the_c_probe() {
    let Some(oracle) = Oracle::locate("version_matches_the_c_probe") else {
        return;
    };
    check(&oracle, "version", "version\n");
}

#[test]
fn test_vectors_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("test_vectors_match_the_c_mux") else {
        return;
    };
    let scripts = vector_scripts();
    let mut round_trips = 0;
    for path in oracle::test_vectors() {
        let bytes = std::fs::read(&path).unwrap();
        // The round trip of an unedited mux.
        let report = mux_report(&format!("create 0 file:{}\nassemble 0\n", path.display()));
        if report.contains(&format!(
            "assemble.data={},0,{:016x}",
            bytes.len(),
            digest(&bytes)
        )) {
            round_trips += 1;
        }
    }
    let commands = check_all(&oracle, "test_vectors", &scripts);
    eprintln!(
        "test_vectors_match_the_c_mux: {} vectors, {commands} commands, {round_trips} exact round trips",
        scripts.len()
    );
    assert!(round_trips > 0);
}

#[test]
fn animations_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("animations_match_the_c_mux") else {
        return;
    };
    let directory = work_dir("wp15_mux_animations");
    let scripts = animation_scripts(&oracle, &directory);
    let commands = check_all(&oracle, "animations", &scripts);
    eprintln!(
        "animations_match_the_c_mux: {} files, {commands} commands",
        scripts.len()
    );
}

#[test]
fn truncated_and_corrupt_files_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("truncated_and_corrupt_files_match_the_c_mux") else {
        return;
    };
    let directory = work_dir("wp15_mux_truncated");
    let scripts = truncated_scripts(&directory);
    let commands = check_all(&oracle, "truncated", &scripts);
    eprintln!(
        "truncated_and_corrupt_files_match_the_c_mux: {} files, {commands} commands",
        scripts.len()
    );
}

#[test]
fn edit_sequences_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("edit_sequences_match_the_c_mux") else {
        return;
    };
    let directory = work_dir("wp15_mux_edits");
    let scripts = edit_scripts(&directory);
    let commands = check_all(&oracle, "edits", &scripts);
    eprintln!(
        "edit_sequences_match_the_c_mux: {} sequences, {commands} commands",
        scripts.len()
    );
}
