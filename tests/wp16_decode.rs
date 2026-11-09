//! WP16 differential test: the features probe and the whole-image decoding of
//! the safe decoder API (`webp::api::decode`) against libwebp, through WP12's
//! probe `tools/oracle/probes/wp12/decode.c` (ADR 0002, ADR 0007).
//!
//! The commands are WP12's (`features`, `featureshex`, `decode`, `simple`,
//! `into`); each runs through the C probe in a separate process and through
//! the API, and the probe's lines are compared with the API's, with wildcards
//! where the API exposes nothing (`wp16_support`):
//!
//! - `Features::parse` on every vector, its prefixes and seeded mutations of
//!   its header, and WP12's synthetic headers;
//! - `decode` in all 13 color modes on every vector, against `WebPDecode` into
//!   library memory: status, size and the digest of the whole memory;
//! - WP12's options matrix (filters, upsampling, flip, threads, dithering,
//!   crops, scales, invalid options) with even crops added, into library
//!   memory and into caller rows and planes with padded strides and WP12's
//!   guard bytes, caller memory one byte too short or narrow, and truncated
//!   input; options that the API refuses although libwebp decodes them (odd
//!   crop offsets, dithering above 100) are the recorded deviations, counted;
//! - `decode` with `Config::new(mode)` against the simple API
//!   (`WebPDecodeRGB` to `WebPDecodeYUV`) and `decode_into` against the
//!   `...Into` functions.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp12_support/mod.rs"]
mod wp12_support;
#[path = "wp16_support/mod.rs"]
mod wp16_support;

use std::fmt::Write as _;
use std::path::Path;

use oracle::Oracle;
use webp::api::decode::{
    ColorMode, Config, Features, Format, Output, YuvaPlanes, decode, decode_into,
};
use wp12_support::{
    Geometry, Options, Rng, digest, geometry_for, output_dimensions, read_prefix, token, vectors,
};
use wp16_support::{Replay, Tally, check, ext_lines, replay_decode, split, status_code};

/// The width, height and alpha flag of a vector.
fn dimensions(path: &Path) -> (i32, i32, bool) {
    match Features::parse(&read_prefix(path, -1)) {
        Ok(f) => (
            i32::try_from(f.width).unwrap(),
            i32::try_from(f.height).unwrap(),
            f.has_alpha,
        ),
        Err(_) => (0, 0, false),
    }
}

/// One command: the probe's line and the API's replay.
struct Case {
    line: String,
    replay: Replay,
}

/// Runs the commands through the probe and compares each with its replay.
/// Each test locates the oracle first, so that a run without it skips the
/// replays as well.
fn run(oracle: &Oracle, test: &str, cases: Vec<Case>) -> Tally {
    let mut tally = Tally::default();
    let input: String = cases.iter().map(|case| case.line.as_str()).collect();
    let output = oracle
        .probe("wp12", "decode")
        .run::<&str>(&[], input.as_bytes());
    output.success();
    let text = String::from_utf8(output.stdout.clone()).expect("the probe prints UTF-8");
    let blocks = split(&text);
    assert_eq!(blocks.len(), cases.len(), "{test}: one block per command");
    for (case, (command, lines)) in cases.into_iter().zip(blocks) {
        assert_eq!(command, case.line.trim_end(), "{test}: the probe's echo");
        check(test, &command, &lines, case.replay, &mut tally);
    }
    eprintln!("{test}: {tally:?}");
    tally
}

/// The probe's `Features` lines, from `Features::parse`.
fn features_replay(data: &[u8]) -> Replay {
    let result = Features::parse(data);
    let mut out = String::new();
    writeln!(out, "status={}", status_code(&result)).unwrap();
    match result {
        Ok(f) => {
            let format = match f.format {
                Some(Format::Lossy) => 1,
                Some(Format::Lossless) => 2,
                None => 0,
            };
            writeln!(
                out,
                "features={},{},{},{},{format}",
                f.width,
                f.height,
                i32::from(f.has_alpha),
                i32::from(f.has_animation)
            )
            .unwrap();
        }
        // libwebp writes what it parsed before the failure; the API returns
        // no features then.
        Err(_) => out.push_str("features=*\n"),
    }
    out.push_str("info=*\n");
    Replay::Lines(out)
}

#[test]
fn features_match_on_vectors_prefixes_and_mutations() {
    let Some(oracle) = Oracle::locate("features_match_on_vectors_prefixes_and_mutations") else {
        return;
    };
    let mut cases = Vec::new();
    let mut rng = Rng(16);
    for path in oracle::test_vectors() {
        let data = read_prefix(&path, -1);
        let mut lengths: Vec<i64> = (0..=48).collect();
        lengths.extend([-1, data.len() as i64 / 2, data.len() as i64 - 1]);
        for len in lengths {
            cases.push(Case {
                line: format!("features {} {len}\n", path.display()),
                replay: features_replay(&read_prefix(&path, len)),
            });
        }
        for _ in 0..12 {
            let mut bytes = data[..data.len().min(96)].to_vec();
            for _ in 0..rng.range(1, 3) {
                let at = rng.below(bytes.len().min(64) as u64) as usize;
                bytes[at] = match rng.below(4) {
                    0 => 0,
                    1 => 0xff,
                    2 => bytes[at] ^ (1 << rng.below(8)),
                    _ => rng.below(256) as u8,
                };
            }
            cases.push(Case {
                line: format!("featureshex {}\n", token(&bytes)),
                replay: features_replay(&bytes),
            });
        }
    }
    // WP12's synthetic headers: VP8X canvases at the area limit, chunk sizes
    // at the payload limit, raw VP8L and ALPH + VP8.
    let riff = |payload: &[u8], size: u32| {
        let mut file = b"RIFF".to_vec();
        file.extend(size.to_le_bytes());
        file.extend(b"WEBP");
        file.extend(payload);
        file
    };
    let vp8x = |flags: u32, w: u32, h: u32| {
        let mut chunk = b"VP8X".to_vec();
        chunk.extend(10u32.to_le_bytes());
        chunk.extend(flags.to_le_bytes());
        chunk.extend(&(w - 1).to_le_bytes()[..3]);
        chunk.extend(&(h - 1).to_le_bytes()[..3]);
        chunk
    };
    let mut synthetic: Vec<Vec<u8>> = Vec::new();
    for (w, h) in [(1, 1), (65536, 65536), (1 << 24, 256), (16384, 16384)] {
        for flags in [0, 0x2, 0x10, 0x12, 0x3e] {
            let chunk = vp8x(flags, w, h);
            synthetic.push(riff(&chunk, 4 + chunk.len() as u32));
        }
    }
    for size in [0u32, 11, 0xffff_fff6, u32::MAX] {
        synthetic.push(riff(b"VP8 \x0a\x00\x00\x00", size));
    }
    synthetic.push(b"\x2f\x3f\xc0\x0f\x10\x00\x00\x00\x00\x00\x00\x00".to_vec());
    synthetic.push(b"ALPH\x02\x00\x00\x00\x00\x00VP8 \x0a\x00\x00\x00".to_vec());
    for bytes in synthetic {
        cases.push(Case {
            line: format!("featureshex {}\n", token(&bytes)),
            replay: features_replay(&bytes),
        });
    }
    let tally = run(
        &oracle,
        "features_match_on_vectors_prefixes_and_mutations",
        cases,
    );
    assert_eq!(tally.skipped, 0);
}

/// A `decode` command and its replay.
fn decode_case(path: &Path, len: i64, mode: u32, options: Options, geometry: &Geometry) -> Case {
    let (width, height, _) = dimensions(path);
    Case {
        line: format!(
            "decode {} {len} {mode} {} {}\n",
            path.display(),
            options.token(),
            geometry.token()
        ),
        replay: replay_decode(
            &read_prefix(path, len),
            mode,
            &options,
            geometry,
            width,
            height,
        ),
    }
}

#[test]
fn decoding_matches_in_every_color_mode() {
    let Some(oracle) = Oracle::locate("decoding_matches_in_every_color_mode") else {
        return;
    };
    let mut cases = Vec::new();
    for path in oracle::test_vectors() {
        let large = !vectors().contains(&path);
        let modes: Vec<u32> = if large {
            vec![1, 11]
        } else {
            (0..13).collect()
        };
        for mode in modes {
            cases.push(decode_case(
                &path,
                -1,
                mode,
                Options::default(),
                &Geometry::default(),
            ));
        }
    }
    let tally = run(&oracle, "decoding_matches_in_every_color_mode", cases);
    assert_eq!(tally.skipped, 0);
    assert!(tally.refused.is_empty());
}

/// WP12's option sets (`tests/wp12_decode.rs`) for an image of `width` by
/// `height`, and the same random crops moved to even offsets.
fn option_sets(width: i32, height: i32, rng: &mut Rng) -> Vec<Options> {
    let mut sets = Vec::new();
    let mut with = |changes: &[(usize, i32)]| {
        let mut options = Options::default();
        for &(field, value) in changes {
            options.0[field] = value;
        }
        sets.push(options);
    };
    with(&[(Options::BYPASS, 1)]);
    with(&[(Options::NOFANCY, 1)]);
    with(&[(Options::FLIP, 1)]);
    with(&[(Options::THREADS, 1)]);
    with(&[
        (Options::THREADS, 1),
        (Options::FLIP, 1),
        (Options::DITHER, 50),
    ]);
    with(&[(Options::DITHER, 100)]);
    with(&[(Options::DITHER, 30), (Options::ALPHA_DITHER, 100)]);
    with(&[(Options::ALPHA_DITHER, 50), (Options::NOFANCY, 1)]);
    for _ in 0..3 {
        let left = rng.range(0, i64::from(width - 1)) as i32;
        let top = rng.range(0, i64::from(height - 1)) as i32;
        let cw = rng.range(1, i64::from(width - left)) as i32;
        let ch = rng.range(1, i64::from(height - top)) as i32;
        let crop = [
            (Options::CROP, 1),
            (Options::CROP_LEFT, left),
            (Options::CROP_TOP, top),
            (Options::CROP_WIDTH, cw),
            (Options::CROP_HEIGHT, ch),
        ];
        with(&crop);
        // The same rectangle at the even position below it, as the API asks.
        with(&[
            (Options::CROP, 1),
            (Options::CROP_LEFT, left & !1),
            (Options::CROP_TOP, top & !1),
            (Options::CROP_WIDTH, cw),
            (Options::CROP_HEIGHT, ch),
            (Options::FLIP, rng.below(2) as i32),
        ]);
    }
    with(&[
        (Options::CROP, 1),
        (Options::CROP_LEFT, 1),
        (Options::CROP_TOP, 1),
        (Options::CROP_WIDTH, (width - 1).max(1)),
        (Options::CROP_HEIGHT, (height - 1).max(1)),
        (Options::FLIP, 1),
    ]);
    with(&[
        (Options::SCALE, 1),
        (Options::SCALED_WIDTH, (width / 3).max(1)),
        (Options::SCALED_HEIGHT, (height / 2).max(1)),
    ]);
    with(&[
        (Options::SCALE, 1),
        (Options::SCALED_WIDTH, width * 2 - 1),
        (Options::SCALED_HEIGHT, height + 3),
        (Options::FLIP, 1),
    ]);
    with(&[
        (Options::SCALE, 1),
        (Options::SCALED_WIDTH, (width / 2).max(1)),
    ]);
    with(&[
        (Options::CROP, 1),
        (Options::CROP_LEFT, width / 4),
        (Options::CROP_TOP, height / 4),
        (Options::CROP_WIDTH, (width / 2).max(1)),
        (Options::CROP_HEIGHT, (height / 2).max(1)),
        (Options::SCALE, 1),
        (Options::SCALED_WIDTH, (width / 3).max(1)),
        (Options::SCALED_HEIGHT, 0),
        (Options::THREADS, 1),
    ]);
    // Invalid options.
    with(&[
        (Options::CROP, 1),
        (Options::CROP_LEFT, width),
        (Options::CROP_WIDTH, 1),
        (Options::CROP_HEIGHT, 1),
    ]);
    with(&[
        (Options::CROP, 1),
        (Options::CROP_LEFT, (width + 1) & !1),
        (Options::CROP_WIDTH, 2),
        (Options::CROP_HEIGHT, 1),
    ]);
    with(&[(Options::SCALE, 1)]);
    with(&[(Options::DITHER, 101)]);
    sets
}

#[test]
fn decoding_matches_with_options_and_caller_memory() {
    let Some(oracle) = Oracle::locate("decoding_matches_with_options_and_caller_memory") else {
        return;
    };
    let mut cases = Vec::new();
    let mut rng = Rng(1616);
    for path in vectors() {
        let (width, height, _) = dimensions(&path);
        if width <= 0 {
            continue;
        }
        for options in option_sets(width, height, &mut rng) {
            let mode = rng.below(13) as u32;
            cases.push(decode_case(&path, -1, mode, options, &Geometry::default()));
            if let Some((w, h)) = output_dimensions(width, height, &options) {
                let mode = rng.below(13) as u32;
                let pad = rng.range(0, 9) as i32;
                let geometry = geometry_for(mode, w, h, pad, mode == 12);
                cases.push(decode_case(&path, -1, mode, options, &geometry));
            }
        }
        // Caller memory one byte too short or too narrow, and YUVA without an
        // alpha plane.
        for mode in [1u32, 6, 11, 12] {
            let mut short = geometry_for(mode, width, height, 0, mode == 12);
            short.0[0].1 -= 1;
            cases.push(decode_case(&path, -1, mode, Options::default(), &short));
            let mut narrow = geometry_for(mode, width, height, 0, mode == 12);
            narrow.0[0].0 -= 1;
            cases.push(decode_case(&path, -1, mode, Options::default(), &narrow));
            if mode >= 11 {
                // A short chroma plane.
                let mut chroma = geometry_for(mode, width, height, 1, mode == 12);
                chroma.0[2].1 -= 1;
                cases.push(decode_case(&path, -1, mode, Options::default(), &chroma));
            }
        }
        let no_alpha = geometry_for(12, width, height, 2, false);
        cases.push(decode_case(&path, -1, 12, Options::default(), &no_alpha));
        // Truncated input, into library and caller memory.
        let len = std::fs::metadata(&path).unwrap().len() as i64;
        for cut in [len / 3, len - 1] {
            let mode = rng.below(13) as u32;
            cases.push(decode_case(
                &path,
                cut,
                mode,
                Options::default(),
                &Geometry::default(),
            ));
            let geometry = geometry_for(1, width, height, 0, false);
            cases.push(decode_case(&path, cut, 1, Options::default(), &geometry));
        }
    }
    let tally = run(
        &oracle,
        "decoding_matches_with_options_and_caller_memory",
        cases,
    );
    assert!(tally.compared > 4000, "{tally:?}");
}

/// The color mode of a simple-API function.
fn func_mode(func: &str) -> ColorMode {
    match func {
        "RGB" => ColorMode::Rgb,
        "RGBA" => ColorMode::Rgba,
        "ARGB" => ColorMode::Argb,
        "BGR" => ColorMode::Bgr,
        "BGRA" => ColorMode::Bgra,
        _ => ColorMode::Yuv,
    }
}

/// The probe's `Simple` lines, from `decode` with `Config::new(mode)`.
fn simple_replay(data: &[u8], func: &str) -> Replay {
    let mut out = String::new();
    match decode(data, &Config::new(func_mode(func))) {
        Ok(image) => {
            writeln!(out, "dims={},{}", image.width(), image.height()).unwrap();
            if func == "YUV" {
                let planes = image.planes().expect("a YUV image");
                let joined = |rows: &webp::api::decode::Rows<'_>| -> Vec<u8> {
                    rows.iter().flatten().copied().collect()
                };
                out.push_str("yuv=*\n");
                writeln!(out, "y={:016x}", digest(&joined(&planes.y))).unwrap();
                writeln!(out, "u={:016x}", digest(&joined(&planes.u))).unwrap();
                writeln!(out, "v={:016x}", digest(&joined(&planes.v))).unwrap();
            } else {
                writeln!(out, "out={:016x}", digest(image.samples())).unwrap();
            }
        }
        Err(_) => out.push_str("dims=*\nout=null\n"),
    }
    Replay::Lines(out)
}

/// The probe's `Into` lines, from `decode_into` with `Config::new(mode)`.
fn into_replay(data: &[u8], func: &str, geometry: &Geometry) -> Replay {
    let mode = func_mode(func);
    let mut blocks = geometry.blocks();
    let ok = {
        let mut slices = wp12_support::plane_slices(&mut blocks).into_iter();
        let stride = |i: usize| usize::try_from(geometry.0[i].0).unwrap();
        let output = if mode.is_yuv() {
            Output::Planes(YuvaPlanes {
                y: slices.next().unwrap(),
                y_stride: stride(0),
                u: slices.next().unwrap(),
                v: slices.next().unwrap(),
                uv_stride: stride(1),
                a: None,
                a_stride: 0,
            })
        } else {
            Output::Rows {
                data: slices.next().unwrap(),
                stride: stride(0),
            }
        };
        decode_into(data, &Config::new(mode), output).is_ok()
    };
    let mut out = format!("result={}\n", if ok { "buffer" } else { "null" });
    ext_lines(&mut out, &blocks);
    Replay::Lines(out)
}

#[test]
fn simple_and_into_functions_match_decode() {
    let Some(oracle) = Oracle::locate("simple_and_into_functions_match_decode") else {
        return;
    };
    let mut cases = Vec::new();
    let mut rng = Rng(916);
    for path in oracle::test_vectors() {
        let large = !vectors().contains(&path);
        let (width, height, _) = dimensions(&path);
        let len = std::fs::metadata(&path).unwrap().len() as i64;
        let funcs: &[&str] = if large {
            &["RGBA"]
        } else {
            &["RGB", "RGBA", "ARGB", "BGR", "BGRA", "YUV"]
        };
        for func in funcs {
            for cut in [-1, len - 7] {
                cases.push(Case {
                    line: format!("simple {} {cut} {func}\n", path.display()),
                    replay: simple_replay(&read_prefix(&path, cut), func),
                });
            }
        }
        if large || width <= 0 {
            continue;
        }
        for (func, mode) in [
            ("RGB", 0u32),
            ("RGBA", 1),
            ("ARGB", 4),
            ("BGR", 2),
            ("BGRA", 3),
            ("YUV", 11),
        ] {
            let pad = rng.range(0, 5) as i32;
            let mut geometries = vec![geometry_for(mode, width, height, pad, false)];
            let mut small = geometry_for(mode, width, height, 0, false);
            small.0[0].1 -= 1;
            geometries.push(small);
            let mut narrow = geometry_for(mode, width, height, 0, false);
            narrow.0[0].0 -= 1;
            geometries.push(narrow);
            for geometry in geometries {
                cases.push(Case {
                    line: format!("into {} -1 {func} {}\n", path.display(), geometry.token()),
                    replay: into_replay(&read_prefix(&path, -1), func, &geometry),
                });
            }
        }
    }
    let tally = run(&oracle, "simple_and_into_functions_match_decode", cases);
    assert_eq!(tally.skipped, 0);
}
