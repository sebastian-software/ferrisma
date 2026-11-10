//! WP14 differential test: the animation decoder of `src/demux/anim_decode.rs`
//! against `src/demux/anim_decode.c`, through the probe
//! `tools/oracle/probes/wp14/anim.c` (ADR 0002).
//!
//! For every input and option set the probe prints `WebPAnimDecoderGetInfo`,
//! the features and metadata chunks of the decoder's demuxer, and for every
//! `WebPAnimDecoderGetNext` the result, the timestamp and the digest of the
//! whole canvas, with `WebPAnimDecoderHasMoreFrames` before each step, a
//! `WebPAnimDecoderReset` in the middle and the frames after it, a whole canvas
//! in hex, and the end state. `wp14_support::anim_report` builds the same lines
//! from the translation; each command is compared on its own.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **The largest vectors in the debug matrix.** The translation's
//!    still-image test decodes the vectors of at most 200,000 pixels; all of
//!    them go through the Rust library's C ABI in
//!    `abi_animation_decoder_matches_the_c_library` (`tests/wp14_abi.rs`).

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::path::PathBuf;

use oracle::Oracle;
use webp::dec::webp_dec::webp_get_info;
use webp::demux::demux::webp_get_demux_version;
use wp14_support::{
    AnimCommand, MODES, OTHER_MODES, anim_commands, anim_report, img2webp_animations,
    split_commands, synthetic_animations, work_dir, write_all,
};

/// Runs `commands` through the C probe and compares each with the translation.
fn compare(oracle: &Oracle, commands: &[AnimCommand]) {
    let input: String = commands.iter().map(|command| command.line(false)).collect();
    let output = oracle
        .probe("wp14", "anim")
        .run::<&str>(&[], input.as_bytes());
    output.success();
    let text = String::from_utf8(output.stdout.clone()).unwrap();
    let expected = split_commands(&text);
    assert_eq!(expected.len(), commands.len(), "probe output: {text}");
    let mut frames = 0;
    for (command, expected) in commands.iter().zip(&expected) {
        let actual = anim_report(command);
        assert_eq!(actual, *expected, "{command:?}");
        frames += expected.matches(".ok=1").count();
    }
    eprintln!(
        "compared {} commands, {frames} decoded frames",
        commands.len()
    );
}

#[test]
fn version_matches_the_c_probe() {
    let Some(oracle) = Oracle::locate("version_matches_the_c_probe") else {
        return;
    };
    let output = oracle.probe("wp14", "anim").run::<&str>(&[], b"version\n");
    output.success();
    assert_eq!(
        String::from_utf8(output.stdout.clone()).unwrap(),
        format!("command=0\nversion={}\n", webp_get_demux_version())
    );
}

#[test]
fn synthetic_animations_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("synthetic_animations_match_the_c_decoder") else {
        return;
    };
    let directory = work_dir("wp14_anim_decode_synthetic");
    let files = write_all(&directory, &synthetic_animations());
    compare(&oracle, &anim_commands(&files));
}

#[test]
fn img2webp_animations_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("img2webp_animations_match_the_c_decoder") else {
        return;
    };
    let directory = work_dir("wp14_anim_decode_img2webp");
    let files = img2webp_animations(&oracle, &directory);
    compare(&oracle, &anim_commands(&files));
}

/// The still images of the test data with at most `limit` pixels.
fn small_vectors(limit: i64) -> Vec<(String, PathBuf)> {
    oracle::test_vectors()
        .into_iter()
        .filter(|path| {
            let bytes = std::fs::read(path).unwrap();
            let (mut width, mut height) = (0, 0);
            !webp_get_info(&bytes, Some(&mut width), Some(&mut height))
                || i64::from(width) * i64::from(height) <= limit
        })
        .map(|path| {
            (
                path.file_name().unwrap().to_string_lossy().into_owned(),
                path,
            )
        })
        .collect()
}

#[test]
fn still_images_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("still_images_match_the_c_decoder") else {
        return;
    };
    // One run per vector, the color mode and threads cycling; the large vectors
    // take minutes in a debug build and add nothing that the small ones lack.
    let commands: Vec<AnimCommand> = small_vectors(200_000)
        .into_iter()
        .enumerate()
        .map(|(index, (_, path))| AnimCommand {
            mode: MODES[index % 4],
            threads: (index / 4 % 2) as i32,
            reset: (index % 3 == 0).into(),
            dump: 0,
            path,
        })
        .collect();
    compare(&oracle, &commands);
}

#[test]
fn unsupported_options_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("unsupported_options_match_the_c_decoder") else {
        return;
    };
    let directory = work_dir("wp14_anim_decode_options");
    let animations = synthetic_animations();
    let chosen: Vec<(String, Vec<u8>)> = animations
        .into_iter()
        .filter(|(name, _)| name == "edges" || name == "metadata" || name == "still_alpha")
        .collect();
    let files = write_all(&directory, &chosen);
    let mut commands = Vec::new();
    for (_, path) in &files {
        for mode in OTHER_MODES {
            for threads in [0, 1, -3] {
                commands.push(AnimCommand {
                    mode,
                    threads,
                    reset: 1,
                    dump: 1,
                    path: path.clone(),
                });
            }
        }
    }
    compare(&oracle, &commands);
}

#[test]
fn invalid_inputs_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("invalid_inputs_match_the_c_decoder") else {
        return;
    };
    let directory = work_dir("wp14_anim_decode_invalid");
    let mut inputs: Vec<(String, Vec<u8>)> = vec![
        ("empty".to_owned(), Vec::new()),
        ("one_byte".to_owned(), vec![0x52]),
        ("riff_only".to_owned(), b"RIFF\x04\0\0\0WEBP".to_vec()),
        (
            "not_webp".to_owned(),
            b"GIF89a, not a WebP file at all".to_vec(),
        ),
    ];
    // Truncations of an animation and of a still image at chunk boundaries and
    // inside payloads: WebPAnimDecoderNew() parses with WebPDemux(), which
    // refuses incomplete files, and WebPGetFeatures() comes first.
    let animations = synthetic_animations();
    for name in ["edges", "metadata"] {
        let (_, bytes) = animations.iter().find(|(found, _)| found == name).unwrap();
        for point in wp8_support::truncation_points(bytes) {
            inputs.push((format!("{name}_cut_{point}"), bytes[..point].to_vec()));
        }
    }
    let still = std::fs::read(oracle::test_data().join("lossy_alpha1.webp")).unwrap();
    for point in wp8_support::truncation_points(&still) {
        inputs.push((format!("lossy_alpha1_cut_{point}"), still[..point].to_vec()));
    }
    // Raw bitstreams without the RIFF header, which the demuxer accepts.
    for (tag, payload) in wp8_support::top_level_chunks(&still) {
        if &tag == b"VP8 " {
            inputs.push(("raw_vp8".to_owned(), payload));
        }
    }
    let lossless = std::fs::read(oracle::test_data().join("lossless_vec_1_0.webp")).unwrap();
    inputs.push(("raw_vp8l".to_owned(), lossless[20..].to_vec()));
    // Seeded mutations of an animation.
    let (_, base) = animations
        .iter()
        .find(|(name, _)| name == "random_03")
        .unwrap();
    let mut rng = wp8_support::Rng(0x0a11_ce55);
    for index in 0..64 {
        let mut bytes = base.clone();
        for _ in 0..1 + rng.below(4) {
            let at = rng.below(bytes.len() as u64) as usize;
            bytes[at] ^= 1 << rng.below(8);
        }
        inputs.push((format!("mutation_{index:02}"), bytes));
    }
    let files = write_all(&directory, &inputs);
    let commands: Vec<AnimCommand> = files
        .iter()
        .enumerate()
        .map(|(index, (_, path))| AnimCommand {
            mode: MODES[index % 4],
            threads: (index % 2) as i32,
            reset: 0,
            dump: 0,
            path: path.clone(),
        })
        .collect();
    compare(&oracle, &commands);
}
