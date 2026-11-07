//! WP12 differential test: the incremental decoder of `src/dec/idec_dec.c`
//! against the C probe `tools/oracle/probes/wp12/decode.c`.
//!
//! Every test vector (but the two largest, see `tests/wp12_decode.rs`) is fed
//! in steps of 1 (files up to 4 KiB), 7 (up to 32 KiB), 64 (up to 512 KiB) and
//! 4,096 bytes, through `WebPIAppend` and through `WebPIUpdate` with a fresh
//! copy of the data each time, and once with the two mixed. After every step
//! the test compares the status, `WebPIDecodedArea` and `WebPIDecGetRGB` or
//! `WebPIDecGetYUVA` with a digest of the rows decoded so far; at the end it
//! compares the whole output buffer, including the guard bytes around caller
//! memory. The decoders come from `WebPIDecode` with a configuration (every
//! colorspace, library or caller memory, and the options of the advanced
//! matrix including `use_threads`, `flip`, cropping, scaling and dithering),
//! `WebPINewDecoder`, `WebPINewRGB`, `WebPINewYUVA` and `WebPINewYUV`, with
//! library or caller memory; truncated inputs end suspended.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp12_support/mod.rs"]
mod wp12_support;

use std::fmt::Write as _;
use std::path::Path;

use oracle::Oracle;
use wp12_support::{
    Geometry, Options, Rng, compare, geometry_for, incremental, output_dimensions, read_prefix,
    vectors,
};

/// The width, height and alpha flag of a vector.
fn dimensions(path: &Path) -> (i32, i32, bool) {
    let data = read_prefix(path, -1);
    let mut f = webp::webp::decode::WebPBitstreamFeatures::default();
    let _ = webp::webp::decode::webp_get_features(&data, &mut f);
    (f.width, f.height, f.has_alpha != 0)
}

/// One `incr` command.
struct Case<'p> {
    path: &'p Path,
    len: i64,
    chunk: usize,
    append: u32,
    kind: &'static str,
    mode: u32,
    options: Options,
    geometry: Geometry,
}

impl Case<'_> {
    fn push(&self, commands: &mut Vec<String>, expected: &mut String) {
        let line = format!(
            "incr {} {} {} {} {} {} {} {}\n",
            self.path.display(),
            self.len,
            self.chunk,
            self.append,
            self.kind,
            self.mode,
            self.options.token(),
            self.geometry.token()
        );
        write!(expected, "> {line}").unwrap();
        incremental(
            expected,
            &read_prefix(self.path, self.len),
            self.chunk,
            self.append,
            self.kind,
            self.mode,
            self.options,
            &self.geometry,
        );
        commands.push(line);
    }
}

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

/// The chunk sizes for a file of `size` bytes.
fn chunk_sizes(size: u64) -> Vec<usize> {
    let mut chunks = Vec::new();
    if size <= 4096 {
        chunks.push(1);
    }
    if size <= 32768 {
        chunks.push(7);
    }
    if size <= 512 * 1024 {
        chunks.push(64);
    }
    chunks.push(4096);
    chunks
}

fn chunk_size_cases(append: u32) -> (Vec<String>, String) {
    let mut commands = Vec::new();
    let mut expected = String::new();
    let mut rng = Rng(u64::from(append) + 40);
    for path in vectors() {
        let size = std::fs::metadata(&path).unwrap().len();
        for chunk in chunk_sizes(size) {
            Case {
                path: &path,
                len: -1,
                chunk,
                append,
                kind: "config",
                mode: rng.below(13) as u32,
                options: Options::default(),
                geometry: Geometry::default(),
            }
            .push(&mut commands, &mut expected);
        }
    }
    (commands, expected)
}

#[test]
fn incremental_append_matches_at_every_chunk_size() {
    let (commands, expected) = chunk_size_cases(1);
    run(
        "incremental_append_matches_at_every_chunk_size",
        &commands,
        &expected,
    );
}

#[test]
fn incremental_update_matches_at_every_chunk_size() {
    let (commands, expected) = chunk_size_cases(0);
    run(
        "incremental_update_matches_at_every_chunk_size",
        &commands,
        &expected,
    );
}

#[test]
fn incremental_decoders_match_with_options_and_buffers() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    let mut rng = Rng(4242);
    for path in vectors() {
        let (width, height, alpha) = dimensions(&path);
        if width <= 0 {
            continue;
        }
        let size = std::fs::metadata(&path).unwrap().len();
        let chunk = if size <= 32768 { 64 } else { 4096 };
        let append = rng.below(2) as u32;
        let mut option_sets = vec![Options::default()];
        let mut with = |changes: &[(usize, i32)]| {
            let mut options = Options::default();
            for &(field, value) in changes {
                options.0[field] = value;
            }
            option_sets.push(options);
        };
        with(&[(Options::THREADS, 1)]);
        with(&[(Options::FLIP, 1), (Options::THREADS, 1)]);
        with(&[
            (Options::DITHER, 70),
            (Options::ALPHA_DITHER, 100),
            (Options::BYPASS, 1),
        ]);
        with(&[(Options::NOFANCY, 1), (Options::FLIP, 1)]);
        with(&[
            (Options::CROP, 1),
            (Options::CROP_LEFT, width / 3),
            (Options::CROP_TOP, height / 5 + 1),
            (Options::CROP_WIDTH, (width / 2).max(1)),
            (Options::CROP_HEIGHT, (height / 2).max(1)),
        ]);
        with(&[
            (Options::SCALE, 1),
            (Options::SCALED_WIDTH, (width / 2).max(1)),
            (Options::SCALED_HEIGHT, (height * 2 / 3).max(1)),
            (Options::FLIP, 1),
        ]);
        for options in option_sets {
            let mode = rng.below(13) as u32;
            Case {
                path: &path,
                len: -1,
                chunk,
                append,
                kind: "config",
                mode,
                options,
                geometry: Geometry::default(),
            }
            .push(&mut commands, &mut expected);
            if let Some((w, h)) = output_dimensions(width, height, &options) {
                let mode = rng.below(13) as u32;
                let geometry = geometry_for(
                    mode,
                    w,
                    h,
                    rng.range(0, 7) as i32,
                    mode == 12 || (mode == 11 && alpha),
                );
                Case {
                    path: &path,
                    len: -1,
                    chunk,
                    append: 1 - append,
                    kind: "config",
                    mode,
                    options,
                    geometry,
                }
                .push(&mut commands, &mut expected);
            }
        }
        // The other constructors, with library and caller memory.
        let rgb_mode = rng.below(11) as u32;
        for (kind, mode, geometry) in [
            ("new", 0, Geometry::default()),
            ("rgb", rgb_mode, Geometry::default()),
            (
                "rgb",
                rgb_mode,
                geometry_for(rgb_mode, width, height, rng.range(0, 5) as i32, false),
            ),
            ("yuva", 12, Geometry::default()),
            ("yuva", 12, geometry_for(12, width, height, 3, true)),
            ("yuva", 11, geometry_for(11, width, height, 0, false)),
            ("yuv", 11, geometry_for(11, width, height, 1, false)),
            ("yuv", 11, Geometry::default()),
        ] {
            Case {
                path: &path,
                len: -1,
                chunk,
                append,
                kind,
                mode,
                options: Options::default(),
                geometry,
            }
            .push(&mut commands, &mut expected);
        }
        // Truncated input, mixed functions and invalid constructor arguments.
        let len = size as i64;
        for (cut, append) in [(len * 2 / 3, 1), (len - 1, 0), (-1, 2)] {
            Case {
                path: &path,
                len: cut,
                chunk: 64,
                append,
                kind: "config",
                mode: rng.below(13) as u32,
                options: Options::default(),
                geometry: Geometry::default(),
            }
            .push(&mut commands, &mut expected);
        }
    }
    for (kind, mode, geometry) in [
        ("rgb", 11, Geometry::default()),
        ("rgb", 1, Geometry(vec![(0, 400)])),
        ("rgb", 1, Geometry(vec![(40, 0)])),
        ("yuva", 12, Geometry(vec![(16, 256), (8, 64)])),
        ("yuva", 12, Geometry(vec![(16, 256), (0, 64), (8, 64)])),
        (
            "yuva",
            12,
            Geometry(vec![(16, 256), (8, 64), (8, 64), (16, 0)]),
        ),
        ("yuv", 11, Geometry(vec![(16, 0), (8, 64), (8, 64)])),
    ] {
        let path = oracle::test_data().join("test.webp");
        Case {
            path: &path,
            len: -1,
            chunk: 4096,
            append: 1,
            kind,
            mode,
            options: Options::default(),
            geometry,
        }
        .push(&mut commands, &mut expected);
    }
    run(
        "incremental_decoders_match_with_options_and_buffers",
        &commands,
        &expected,
    );
}

#[test]
fn incremental_threads_match_on_wide_lossy_images() {
    let mut commands = Vec::new();
    let mut expected = String::new();
    let mut rng = Rng(77);
    for name in [
        "lossy_alpha1.webp",
        "lossy_alpha2.webp",
        "lossy_alpha3.webp",
        "vp80-05-sharpness-1443.webp",
        "lossy_extreme_probabilities.webp",
        "vp80-00-comprehensive-008.webp",
    ] {
        let path = oracle::test_data().join(name);
        let (width, height, alpha) = dimensions(&path);
        for (chunk, append) in [(7, 1), (64, 0), (257, 1), (4096, 0)] {
            let mut options = Options::default();
            options.0[Options::THREADS] = 1;
            options.0[Options::FLIP] = rng.below(2) as i32;
            options.0[Options::DITHER] = [0, 50, 100][rng.below(3) as usize];
            options.0[Options::ALPHA_DITHER] = [0, 100][rng.below(2) as usize];
            let mode = rng.below(13) as u32;
            let geometry = if rng.below(2) == 0 {
                Geometry::default()
            } else {
                geometry_for(mode, width, height, 2, mode == 12 || (mode == 11 && alpha))
            };
            Case {
                path: &path,
                len: -1,
                chunk,
                append,
                kind: "config",
                mode,
                options,
                geometry,
            }
            .push(&mut commands, &mut expected);
        }
    }
    run(
        "incremental_threads_match_on_wide_lossy_images",
        &commands,
        &expected,
    );
}
