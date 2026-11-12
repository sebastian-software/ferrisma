//! WP12 differential test: the decoding API of `src/dec/webp_dec.c`,
//! `src/dec/buffer_dec.c` and `src/dec/idec_dec.c` against the C probe
//! `tools/oracle/probes/wp12/decode.c`, on the pinned test vectors.
//!
//! - `WebPGetFeatures` and `WebPGetInfo` on every vector, its truncations and
//!   seeded mutations of its headers;
//! - `WebPDecode` in every `WEBP_CSP_MODE` (and an invalid one), with library
//!   memory and with caller buffers surrounded by guard bytes, and the
//!   advanced-API matrix: cropping at odd and even origins, scaling, both
//!   together, `bypass_filtering`, `no_fancy_upsampling`, `flip`,
//!   `use_threads`, `dithering_strength`, `alpha_dithering_strength`, invalid
//!   options and too small caller buffers;
//! - `WebPValidateDecoderConfig` on boundary options;
//! - the simple API (`WebPDecodeRGB` to `WebPDecodeYUV`) and the `...Into`
//!   functions with exact, padded and too small buffers;
//! - the incremental decoder (`tests/wp12_incremental.rs`).
//!
//! The two largest vectors (`bryce.webp`, 11158 by 2156, and
//! `lossless_big_random_alpha.webp`, 2048 by 2048) take minutes per decoding in
//! a debug build; they get the features, one default decoding and the
//! simple API, and the upstream scripts cover them through `dwebp`.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **`NULL` pointers** exist only at the C ABI (PORTING rule 8); every
//!    `NULL` case of `decode.h` is compared there
//!    (`abi_answers_null_and_invalid_arguments_as_c` in `tests/wp12_abi.rs`).

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp12_support/mod.rs"]
mod wp12_support;

use std::fmt::Write as _;
use std::path::Path;

use oracle::Oracle;
use wp12_support::{
    Geometry, Options, Rng, compare, consistent_cut, decode, features, geometry_for, into,
    output_dimensions, read_prefix, simple, token, validate, vectors,
};

/// The width, height and alpha flag of a vector, from the Rust translation
/// (the probe prints the same values).
fn dimensions(path: &Path) -> (i32, i32, bool) {
    let data = read_prefix(path, -1);
    let mut f = webp::webp::decode::WebPBitstreamFeatures::default();
    let _ = webp::webp::decode::webp_get_features(&data, &mut f);
    (f.width, f.height, f.has_alpha != 0)
}

/// Runs the commands through the probe and the translation and compares.
fn run(test: &str, commands: &[String], expected: &str) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let input: String = commands.concat();
    let output = oracle
        .probe("wp12", "decode")
        .run::<&str>(&[], input.as_bytes());
    output.success();
    compare(test, &output.stdout, expected);
}

#[test]
fn features_match_on_vectors_truncations_and_mutations() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    let mut rng = Rng(12);
    for path in oracle::test_vectors() {
        let data = read_prefix(&path, -1);
        let mut lengths: Vec<i64> = (0..=48).collect();
        lengths.extend([-1, data.len() as i64 / 2, data.len() as i64 - 1]);
        for len in lengths {
            let line = format!("features {} {len}\n", path.display());
            write!(expected, "> {line}").unwrap();
            features(&mut expected, &read_prefix(&path, len));
            commands.push(line);
        }
        // Mutated headers: one to three bytes of the first 64 replaced.
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
            let line = format!("featureshex {}\n", token(&bytes));
            write!(expected, "> {line}").unwrap();
            features(&mut expected, &bytes);
            commands.push(line);
        }
    }
    // Synthetic headers: VP8X canvases at the area limit, chunk sizes at the
    // payload limit, RIFF sizes that disagree, raw VP8L and ALPH + VP8.
    let mut synthetic: Vec<Vec<u8>> = Vec::new();
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
    for (w, h) in [
        (1, 1),
        (65536, 65536),
        (65535, 65537),
        (1 << 24, 256),
        (16384, 16384),
    ] {
        for flags in [0, 0x2, 0x10, 0x12, 0x3e] {
            let chunk = vp8x(flags, w, h);
            synthetic.push(riff(&chunk, 4 + chunk.len() as u32));
            synthetic.push(riff(&chunk, 100));
        }
    }
    for size in [0u32, 3, 11, 12, 0xffff_fff6, 0xffff_fff7, u32::MAX] {
        synthetic.push(riff(b"VP8 \x0a\x00\x00\x00", size));
        let mut chunk = vp8x(0x10, 16, 16);
        chunk.extend(b"ALPH");
        chunk.extend(size.to_le_bytes());
        synthetic.push(riff(&chunk, 400));
    }
    synthetic.push(b"\x2f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec());
    synthetic.push(b"\x2f\x3f\xc0\x0f\x10\x00\x00\x00\x00\x00\x00\x00".to_vec());
    synthetic.push(b"ALPH\x02\x00\x00\x00\x00\x00VP8 \x0a\x00\x00\x00".to_vec());
    for bytes in synthetic {
        let line = format!("featureshex {}\n", token(&bytes));
        write!(expected, "> {line}").unwrap();
        features(&mut expected, &bytes);
        commands.push(line);
    }
    run(
        "features_match_on_vectors_truncations_and_mutations",
        &commands,
        &expected,
    );
}

fn push_decode(
    commands: &mut Vec<String>,
    expected: &mut String,
    path: &Path,
    len: i64,
    mode: u32,
    options: Options,
    geometry: &Geometry,
) {
    let line = format!(
        "decode {} {len} {mode} {} {}\n",
        path.display(),
        options.token(),
        geometry.token()
    );
    write!(expected, "> {line}").unwrap();
    decode(expected, &read_prefix(path, len), mode, options, geometry);
    commands.push(line);
}

#[test]
fn advanced_decoding_matches_in_every_colorspace() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    for path in oracle::test_vectors() {
        let large = !vectors().contains(&path);
        let modes: Vec<u32> = if large {
            vec![1, 11]
        } else {
            (0..=13).collect()
        };
        for mode in modes {
            push_decode(
                &mut commands,
                &mut expected,
                &path,
                -1,
                mode,
                Options::default(),
                &Geometry::default(),
            );
        }
    }
    run(
        "advanced_decoding_matches_in_every_colorspace",
        &commands,
        &expected,
    );
}

/// The option sets of the matrix for an image of `width` by `height`.
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
    // Cropping at odd and even origins.
    for _ in 0..3 {
        let left = rng.range(0, i64::from(width - 1)) as i32;
        let top = rng.range(0, i64::from(height - 1)) as i32;
        let cw = rng.range(1, i64::from(width - left)) as i32;
        let ch = rng.range(1, i64::from(height - top)) as i32;
        with(&[
            (Options::CROP, 1),
            (Options::CROP_LEFT, left),
            (Options::CROP_TOP, top),
            (Options::CROP_WIDTH, cw),
            (Options::CROP_HEIGHT, ch),
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
    // Scaling down, up and with one derived side.
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
    with(&[(Options::SCALE, 1)]);
    with(&[(Options::DITHER, 101)]);
    sets
}

#[test]
fn advanced_decoding_matches_with_options_and_caller_buffers() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    let mut rng = Rng(1212);
    for path in vectors() {
        let (width, height, alpha) = dimensions(&path);
        if width <= 0 {
            continue;
        }
        for options in option_sets(width, height, &mut rng) {
            let mode = rng.below(13) as u32;
            // Library memory.
            push_decode(
                &mut commands,
                &mut expected,
                &path,
                -1,
                mode,
                options,
                &Geometry::default(),
            );
            // A caller buffer with padded rows, when the options are valid.
            if let Some((w, h)) = output_dimensions(width, height, &options) {
                let mode = rng.below(13) as u32;
                let pad = rng.range(0, 9) as i32;
                let with_alpha = mode == 12 || (mode == 11 && alpha && rng.below(2) == 0);
                let geometry = geometry_for(mode, w, h, pad, with_alpha);
                push_decode(
                    &mut commands,
                    &mut expected,
                    &path,
                    -1,
                    mode,
                    options,
                    &geometry,
                );
            }
        }
        // Caller buffers that are one byte too small, too narrow, or YUVA
        // without an alpha plane.
        let options = Options::default();
        for mode in [1u32, 6, 11, 12] {
            let mut geometry = geometry_for(mode, width, height, 0, mode == 12);
            geometry.0[0].1 -= 1;
            push_decode(
                &mut commands,
                &mut expected,
                &path,
                -1,
                mode,
                options,
                &geometry,
            );
            let mut geometry = geometry_for(mode, width, height, 0, mode == 12);
            geometry.0[0].0 -= 1;
            push_decode(
                &mut commands,
                &mut expected,
                &path,
                -1,
                mode,
                options,
                &geometry,
            );
        }
        let geometry = geometry_for(12, width, height, 2, false);
        push_decode(
            &mut commands,
            &mut expected,
            &path,
            -1,
            12,
            options,
            &geometry,
        );
        // Truncated input.
        let len = std::fs::metadata(&path).unwrap().len() as i64;
        for cut in [len / 3, len - 1] {
            push_decode(
                &mut commands,
                &mut expected,
                &path,
                cut,
                rng.below(13) as u32,
                Options::default(),
                &Geometry::default(),
            );
            let geometry = geometry_for(1, width, height, 0, false);
            push_decode(
                &mut commands,
                &mut expected,
                &path,
                cut,
                1,
                Options::default(),
                &geometry,
            );
        }
    }
    run(
        "advanced_decoding_matches_with_options_and_caller_buffers",
        &commands,
        &expected,
    );
}

#[test]
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
fn validation_matches_on_boundary_options() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    let mut rng = Rng(7);
    let values = [-1, 0, 1, 2, 99, 100, 101, i32::MAX, i32::MIN];
    for _ in 0..3000 {
        let mut options = Options::default();
        for field in 0..14 {
            if rng.below(3) == 0 {
                options.0[field] = values[rng.below(values.len() as u64) as usize];
            }
        }
        let mode = rng.below(15) as u32;
        let width = values[rng.below(values.len() as u64) as usize].max(0) % 300;
        let height = values[rng.below(values.len() as u64) as usize].max(0) % 300;
        let line = format!("validate {mode} {} {width} {height}\n", options.token());
        write!(expected, "> {line}").unwrap();
        validate(&mut expected, mode, options, width, height);
        commands.push(line);
    }
    run(
        "validation_matches_on_boundary_options",
        &commands,
        &expected,
    );
}

#[test]
fn simple_api_and_into_functions_match() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    let mut rng = Rng(99);
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
                let line = format!("simple {} {cut} {func}\n", path.display());
                write!(expected, "> {line}").unwrap();
                simple(&mut expected, &read_prefix(&path, cut), func);
                commands.push(line);
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
                let line = format!("into {} -1 {func} {}\n", path.display(), geometry.token());
                write!(expected, "> {line}").unwrap();
                into(&mut expected, &read_prefix(&path, -1), func, &geometry);
                commands.push(line);
            }
        }
    }
    run("simple_api_and_into_functions_match", &commands, &expected);
}

#[test]
fn slow_memory_decodes_as_c() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    // Slow external memory (is_external_memory 2) takes the temporary-buffer
    // path only for a premultiplied colorspace and an image with alpha; the
    // other combinations decode directly. Each case runs with slow and with
    // ordinary caller memory, so that the two paths stand side by side: a valid
    // buffer, a buffer one byte too small (an invalid buffer), and a truncated
    // input (a failed decoding) into a valid and into an invalid buffer.
    for path in vectors() {
        let (width, height, alpha) = dimensions(&path);
        if width <= 0 {
            continue;
        }
        let len = std::fs::metadata(&path).unwrap().len() as i64;
        let cut_path = consistent_cut(&path, 3, 4);
        let modes: &[u32] = if alpha { &[7, 8, 9, 10, 1] } else { &[7, 10] };
        for &mode in modes {
            let valid = geometry_for(mode, width, height, 3, false);
            let mut small = geometry_for(mode, width, height, 0, false);
            small.0[0].1 -= 1;
            for geometry in [valid, small] {
                for slow in [false, true] {
                    let geometry = if slow {
                        geometry.clone().slow()
                    } else {
                        geometry.clone()
                    };
                    for cut in [-1, len * 3 / 4] {
                        push_decode(
                            &mut commands,
                            &mut expected,
                            &path,
                            cut,
                            mode,
                            Options::default(),
                            &geometry,
                        );
                    }
                    // A decoding that fails after the buffer is allocated.
                    push_decode(
                        &mut commands,
                        &mut expected,
                        &cut_path,
                        -1,
                        mode,
                        Options::default(),
                        &geometry,
                    );
                }
            }
        }
        if alpha {
            // With options: flip, and a crop with scaling.
            let mut flip = Options::default();
            flip.0[Options::FLIP] = 1;
            let mut crop = Options::default();
            crop.0[Options::CROP] = 1;
            crop.0[Options::CROP_LEFT] = width / 4;
            crop.0[Options::CROP_TOP] = height / 4;
            crop.0[Options::CROP_WIDTH] = (width / 2).max(1);
            crop.0[Options::CROP_HEIGHT] = (height / 2).max(1);
            crop.0[Options::SCALE] = 1;
            crop.0[Options::SCALED_WIDTH] = (width / 3).max(1);
            crop.0[Options::SCALED_HEIGHT] = (height / 3).max(1);
            for options in [flip, crop] {
                let (w, h) = output_dimensions(width, height, &options).unwrap();
                let geometry = geometry_for(8, w, h, 1, false).slow();
                push_decode(
                    &mut commands,
                    &mut expected,
                    &path,
                    -1,
                    8,
                    options,
                    &geometry,
                );
            }
        }
    }
    run("slow_memory_decodes_as_c", &commands, &expected);
}
