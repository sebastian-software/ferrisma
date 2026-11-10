//! WP14 differential test of the C ABI (`src/abi/demux.rs`, ADR 0004): the
//! probes that use only `demux.h` run once against the C library and once
//! against the Rust static library, with the same commands, and their outputs
//! must be identical.
//!
//! - `probes/wp14/anim` and `anim_rs` (`tools/oracle/probes/wp14/anim.c`): the
//!   animation decoder on the synthetic, `img2webp`, still and invalid inputs of
//!   `tests/wp14_anim_decode.rs`, with the canvas that a failed
//!   `WebPAnimDecoderGetNext()` leaves behind, and the `NULL` and invalid
//!   arguments of every function of the header.
//! - `probes/wp8/demux` and `probes/wp14/demux_rs`
//!   (`tools/oracle/probes/wp8/demux.c`): WP8's walk over the demuxer, its
//!   iterators and chunks, on the test vectors, their truncations with and
//!   without `allow_partial`, and the synthetic animations.
//!
//! The Rust-linked probes exist when the oracle was built with
//! `tools/build_oracle.py --rust-lib`, as `tools/check.py --oracle` does.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::path::PathBuf;

use oracle::Oracle;
use wp14_support::{
    AnimCommand, MODES, OTHER_MODES, anim_commands, img2webp_animations, split_commands,
    synthetic_animations, work_dir, write_all,
};

/// Runs `input` through the C-linked and the Rust-linked probe and compares the
/// outputs command by command. Returns the number of commands.
fn compare_probes(
    oracle: &Oracle,
    package: &str,
    c_probe: &str,
    rs_probe: &str,
    input: &str,
) -> usize {
    let c = oracle
        .probe(package, c_probe)
        .run::<&str>(&[], input.as_bytes());
    c.success();
    let rs = oracle
        .probe("wp14", rs_probe)
        .run::<&str>(&[], input.as_bytes());
    rs.success();
    let c_text = String::from_utf8(c.stdout.clone()).unwrap();
    let rs_text = String::from_utf8(rs.stdout.clone()).unwrap();
    if c_text != rs_text {
        let lines: Vec<&str> = input.lines().collect();
        let c_commands = split_commands(&c_text);
        let rs_commands = split_commands(&rs_text);
        for (index, (c_command, rs_command)) in c_commands.iter().zip(&rs_commands).enumerate() {
            assert_eq!(
                rs_command,
                c_command,
                "{rs_probe} differs from {c_probe} at command {index}: {:?}",
                lines.get(index)
            );
        }
        assert_eq!(rs_text, c_text, "{rs_probe} differs from {c_probe}");
    }
    input.lines().count()
}

/// The animation decoder commands with the partial canvas, as lines.
fn anim_lines(commands: &[AnimCommand]) -> String {
    commands.iter().map(|command| command.line(true)).collect()
}

#[test]
fn abi_animation_decoder_matches_the_c_library() {
    let Some(oracle) = Oracle::locate("abi_animation_decoder_matches_the_c_library") else {
        return;
    };
    let directory = work_dir("wp14_abi_anim");
    let mut files = write_all(&directory, &synthetic_animations());
    files.extend(img2webp_animations(&oracle, &directory));
    let mut commands = anim_commands(&files);
    for path in oracle::test_vectors() {
        for (index, mode) in MODES.into_iter().enumerate() {
            commands.push(AnimCommand {
                mode,
                threads: (index % 2) as i32,
                reset: 1,
                dump: 0,
                path: path.clone(),
            });
        }
    }
    for (_, path) in files.iter().take(8) {
        for mode in OTHER_MODES {
            commands.push(AnimCommand {
                mode,
                threads: 1,
                reset: 2,
                dump: 1,
                path: path.clone(),
            });
        }
    }
    let count = compare_probes(&oracle, "wp14", "anim", "anim_rs", &anim_lines(&commands));
    eprintln!("compared {count} commands");
}

#[test]
fn abi_answers_null_and_invalid_arguments_as_c() {
    let Some(oracle) = Oracle::locate("abi_answers_null_and_invalid_arguments_as_c") else {
        return;
    };
    let directory = work_dir("wp14_abi_nulls");
    let animations: Vec<(String, Vec<u8>)> = synthetic_animations()
        .into_iter()
        .filter(|(name, _)| {
            ["metadata", "edges", "single_small", "still_alpha"].contains(&name.as_str())
        })
        .collect();
    let mut paths: Vec<PathBuf> = write_all(&directory, &animations)
        .into_iter()
        .map(|(_, path)| path)
        .collect();
    for name in ["lossless_vec_1_0.webp", "lossy_alpha1.webp", "test.webp"] {
        paths.push(oracle::test_data().join(name));
    }
    let mut input = String::from("version\n");
    for path in &paths {
        input.push_str(&format!("nulls {}\n", path.display()));
    }
    let count = compare_probes(&oracle, "wp14", "anim", "anim_rs", &input);
    eprintln!("compared {count} commands");
}

#[test]
fn abi_demuxer_matches_the_c_library() {
    let Some(oracle) = Oracle::locate("abi_demuxer_matches_the_c_library") else {
        return;
    };
    let directory = work_dir("wp14_abi_demux");
    let mut input = String::from("version\n");
    for path in oracle::test_vectors() {
        let size = std::fs::metadata(&path).unwrap().len() as usize;
        let bytes = std::fs::read(&path).unwrap();
        for allow in [0, 1] {
            input.push_str(&format!("file {} 0 {size} {allow}\n", path.display()));
        }
        for point in wp8_support::truncation_points(&bytes) {
            input.push_str(&format!("file {} 0 {point} 1\n", path.display()));
        }
        input.push_str(&format!("file {} 1 {} 0\n", path.display(), size - 1));
    }
    for (_, path) in write_all(&directory, &synthetic_animations()) {
        let bytes = std::fs::read(&path).unwrap();
        let size = bytes.len();
        input.push_str(&format!("file {} 0 {size} 0\n", path.display()));
        for point in wp8_support::truncation_points(&bytes)
            .into_iter()
            .step_by(3)
        {
            input.push_str(&format!("file {} 0 {point} 1\n", path.display()));
        }
    }
    let count = compare_probes(&oracle, "wp8", "demux", "demux_rs", &input);
    eprintln!("compared {count} commands");
}
