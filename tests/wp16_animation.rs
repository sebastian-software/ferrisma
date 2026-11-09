//! WP16 differential test: the animation decoder and the container view of
//! the safe decoder API (`AnimationDecoder`, `Container`) against libwebp's
//! `WebPAnimDecoder` and demuxer, through WP14's probe
//! `tools/oracle/probes/wp14/anim.c` (ADR 0002, ADR 0007).
//!
//! For each `anim` command the probe prints `WebPAnimDecoderGetInfo`, the
//! `WebPDemuxGetI` values and the `ICCP`, `EXIF` and `XMP ` chunk walks of the
//! decoder's demuxer, then `WebPAnimDecoderHasMoreFrames` before every
//! `WebPAnimDecoderGetNext`, each frame's result, timestamp and canvas digest,
//! a reset in the middle, a whole canvas in hex and the end state. The API's
//! replay prints the same from `AnimationDecoder` (`info`, `has_more_frames`,
//! `next_frame`, `reset`) and `Container::new` over the same bytes
//! (`canvas_width`, `canvas_height`, `loop_count`, `background_color`,
//! `frame_count`, `has_alpha` and `has_animation` against the bits of
//! `WEBP_FF_FORMAT_FLAGS`, and `chunks`); the two unknown features are not
//! compared. The inputs are WP14's: hand-assembled and `img2webp`
//! animations, the still vectors, and invalid, truncated and mutated files.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::fmt::Write as _;
use std::path::PathBuf;

use oracle::Oracle;
use webp::api::decode::{AnimationDecoder, AnimationMode, AnimationOptions, Container, Features};
use wp8_support::{digest, hex};
use wp14_support::{
    AnimCommand, MODES, anim_commands, img2webp_animations, split_commands, synthetic_animations,
    work_dir, write_all,
};

/// The API's mode for the probe's `MODE`.
fn animation_mode(mode: i32) -> AnimationMode {
    match mode {
        1 => AnimationMode::Rgba,
        3 => AnimationMode::Bgra,
        7 => AnimationMode::PremultipliedRgba,
        8 => AnimationMode::PremultipliedBgra,
        _ => panic!("the API has four animation modes, not {mode}"),
    }
}

/// `demux.feature0=V` as the two flags that the API exposes.
fn flags_line(alpha: bool, animation: bool) -> String {
    format!(
        "demux.flags=alpha:{},animation:{}",
        u8::from(alpha),
        u8::from(animation)
    )
}

/// The probe's lines with the format flags reduced to the two the API
/// exposes and the unknown features dropped.
fn probe_lines(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("demux.feature0=") {
            let flags: u32 = value.parse().unwrap();
            out.push_str(&flags_line(flags & 0x10 != 0, flags & 0x02 != 0));
        } else if line.starts_with("demux.feature6=") || line.starts_with("demux.feature99=") {
            continue;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

/// The probe's lines for `command`, from the API.
fn api_report(command: &AnimCommand) -> String {
    let bytes = std::fs::read(&command.path).unwrap();
    let mut out = String::new();
    let options = AnimationOptions {
        mode: animation_mode(command.mode),
        multithreaded: command.threads != 0,
    };
    let decoder = AnimationDecoder::new(&bytes, options);
    writeln!(out, "new={}", i32::from(decoder.is_ok())).unwrap();
    let Ok(mut decoder) = decoder else {
        return out;
    };
    let info = decoder.info();
    writeln!(
        out,
        "info={},{},{},{},{},0,0,0,0",
        info.canvas_width,
        info.canvas_height,
        info.loop_count,
        info.background_color,
        info.frame_count
    )
    .unwrap();
    container_lines(&mut out, &bytes);
    let stride = info.canvas_width as usize * 4;
    let canvas_size = stride * info.canvas_height as usize;

    let (mut pass, mut frame, mut failed) = (0, 0, false);
    loop {
        let more = decoder.has_more_frames();
        writeln!(out, "pass.{pass}.{}.more={}", frame + 1, i32::from(more)).unwrap();
        if !more {
            break;
        }
        frame += 1;
        match decoder.next_frame() {
            Ok(Some(next)) => {
                assert_eq!((next.canvas.len(), next.stride), (canvas_size, stride));
                writeln!(out, "pass.{pass}.{frame}.ok=1").unwrap();
                writeln!(out, "pass.{pass}.{frame}.timestamp={}", next.timestamp_ms).unwrap();
                writeln!(
                    out,
                    "pass.{pass}.{frame}.digest={:016x}",
                    digest(next.canvas)
                )
                .unwrap();
                if pass == 0 && frame == command.dump {
                    writeln!(out, "pass.{pass}.{frame}.canvas={}", hex(next.canvas)).unwrap();
                }
            }
            Ok(None) => panic!("has_more_frames promised a frame"),
            Err(error) => {
                assert_eq!(error, webp::api::decode::DecodeError::FrameFailed);
                writeln!(out, "pass.{pass}.{frame}.ok=0").unwrap();
                failed = true;
                break;
            }
        }
        if pass == 0 && frame == command.reset {
            decoder.reset();
            writeln!(out, "reset={}", i32::from(decoder.has_more_frames())).unwrap();
            pass = 1;
            frame = 0;
        }
    }
    writeln!(out, "failed={}", i32::from(failed)).unwrap();
    let more = decoder.has_more_frames();
    writeln!(out, "end.more={}", i32::from(more)).unwrap();
    match decoder.next_frame() {
        Ok(Some(next)) => {
            writeln!(out, "end.next=1").unwrap();
            writeln!(out, "end.timestamp={}", next.timestamp_ms).unwrap();
            writeln!(out, "end.digest={:016x}", digest(next.canvas)).unwrap();
        }
        Ok(None) | Err(_) => writeln!(out, "end.next=0").unwrap(),
    }
    out
}

/// `PrintDemuxer` of the probe, from a `Container` over the same bytes.
fn container_lines(out: &mut String, bytes: &[u8]) {
    let container = Container::new(bytes).expect("the animation decoder's file parses");
    assert!(container.is_complete());
    out.push_str("demux=1\n");
    writeln!(
        out,
        "{}",
        flags_line(container.has_alpha(), container.has_animation())
    )
    .unwrap();
    for (feature, value) in [
        (1, container.canvas_width()),
        (2, container.canvas_height()),
        (3, container.loop_count()),
        (4, container.background_color()),
        (5, container.frame_count()),
    ] {
        writeln!(out, "demux.feature{feature}={value}").unwrap();
    }
    let first = [container.icc_profile(), container.exif(), container.xmp()];
    for (index, tag) in [b"ICCP", b"EXIF", b"XMP "].into_iter().enumerate() {
        let payloads: Vec<&[u8]> = container.chunks(*tag).collect();
        assert_eq!(first[index], payloads.first().copied());
        for (count, payload) in payloads.iter().enumerate() {
            writeln!(
                out,
                "demux.meta{index}.{count}={},{},{},{:016x}",
                count + 1,
                payloads.len(),
                payload.len(),
                digest(payload)
            )
            .unwrap();
        }
        writeln!(out, "demux.meta{index}.count={}", payloads.len()).unwrap();
    }
}

/// Runs the commands through the C probe and compares each with the API.
fn compare(oracle: &Oracle, commands: &[AnimCommand]) -> (usize, usize) {
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
        let expected = probe_lines(expected);
        assert_eq!(api_report(command), expected, "{command:?}");
        frames += expected.matches(".ok=1").count();
    }
    eprintln!(
        "compared {} commands, {frames} decoded frames",
        commands.len()
    );
    (commands.len(), frames)
}

#[test]
fn animations_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("animations_match_the_c_decoder") else {
        return;
    };
    let directory = work_dir("wp16_animations");
    let mut files = write_all(&directory, &synthetic_animations());
    files.extend(img2webp_animations(&oracle, &directory));
    let (commands, frames) = compare(&oracle, &anim_commands(&files));
    assert!(
        commands > 700 && frames > 4000,
        "{commands} commands, {frames} frames"
    );
}

/// The still images of the test data with at most `limit` pixels.
fn small_vectors(limit: u64) -> Vec<PathBuf> {
    oracle::test_vectors()
        .into_iter()
        .filter(|path| {
            let bytes = std::fs::read(path).unwrap();
            Features::parse(&bytes)
                .map_or(true, |f| u64::from(f.width) * u64::from(f.height) <= limit)
        })
        .collect()
}

#[test]
fn still_images_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("still_images_match_the_c_decoder") else {
        return;
    };
    let commands: Vec<AnimCommand> = small_vectors(200_000)
        .into_iter()
        .enumerate()
        .map(|(index, path)| AnimCommand {
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
fn invalid_inputs_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("invalid_inputs_match_the_c_decoder") else {
        return;
    };
    let directory = work_dir("wp16_animations_invalid");
    let mut inputs: Vec<(String, Vec<u8>)> = vec![
        ("empty".to_owned(), Vec::new()),
        ("one_byte".to_owned(), vec![0x52]),
        ("riff_only".to_owned(), b"RIFF\x04\0\0\0WEBP".to_vec()),
        (
            "not_webp".to_owned(),
            b"GIF89a, not a WebP file at all".to_vec(),
        ),
    ];
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
    for (tag, payload) in wp8_support::top_level_chunks(&still) {
        if &tag == b"VP8 " {
            inputs.push(("raw_vp8".to_owned(), payload));
        }
    }
    let lossless = std::fs::read(oracle::test_data().join("lossless_vec_1_0.webp")).unwrap();
    inputs.push(("raw_vp8l".to_owned(), lossless[20..].to_vec()));
    let (_, base) = animations
        .iter()
        .find(|(name, _)| name == "random_03")
        .unwrap();
    let mut rng = wp8_support::Rng(0x0a11_ce16);
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
