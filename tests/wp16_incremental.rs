//! WP16 differential test: the incremental decoder of the safe decoder API
//! (`IncrementalDecoder`) against libwebp's `WebPIDecoder`, through WP12's
//! probe `tools/oracle/probes/wp12/decode.c` (ADR 0002, ADR 0007).
//!
//! The `incr` commands are WP12's: a vector fed in chunks through
//! `WebPIAppend` or `WebPIUpdate`. After every step the probe prints the
//! status, `WebPIDecodedArea`, the result of `WebPIDecGetRGB` or
//! `WebPIDecGetYUVA` and the digest of the rows decoded so far; the API's
//! replay prints the same from `append` or `update`, `output_size`,
//! `decoded_rows`, `rows` or `planes` (plane offsets are not exposed and are
//! not compared). At the end: the output size, and the digest of the library
//! memory (`into_image`) or of the caller's memory with WP12's guard bytes.
//!
//! The configuration kind is `WebPIDecode(NULL, 0, &config)`, which the API's
//! decoder over its own buffer with the same options equals; `WebPINewRGB`,
//! `WebPINewYUVA`, `WebPINewYUV` and `WebPINewDecoder` set no options and are
//! replayed with `Config::new(mode)`.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp12_support/mod.rs"]
mod wp12_support;
#[path = "wp16_support/mod.rs"]
mod wp16_support;

use std::path::Path;

use oracle::Oracle;
use webp::api::decode::Features;
use wp12_support::{Geometry, Options, Rng, geometry_for, output_dimensions, read_prefix, vectors};
use wp16_support::{Deviation, Incremental, Replay, Tally, check, replay_incremental, split};

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

/// One `incr` command: the probe's line and the API's replay.
struct Case {
    line: String,
    replay: Replay,
}

#[expect(clippy::too_many_arguments, reason = "the fields of the command")]
fn case(
    path: &Path,
    len: i64,
    chunk: usize,
    append: u32,
    kind: &str,
    mode: u32,
    options: Options,
    geometry: Geometry,
) -> Case {
    let (width, height, _) = dimensions(path);
    let data = read_prefix(path, len);
    let line = format!(
        "incr {} {len} {chunk} {append} {kind} {mode} {} {}\n",
        path.display(),
        options.token(),
        geometry.token()
    );
    let replay = replay_incremental(&Incremental {
        data: &data,
        chunk,
        append,
        kind,
        mode,
        options,
        geometry,
        width,
        height,
    });
    Case { line, replay }
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
    let mut steps = 0;
    for (case, (command, lines)) in cases.into_iter().zip(blocks) {
        assert_eq!(command, case.line.trim_end(), "{test}: the probe's echo");
        steps += lines
            .iter()
            .filter(|line| line.starts_with("step="))
            .count();
        check(test, &command, &lines, case.replay, &mut tally);
    }
    eprintln!("{test}: {tally:?}, {steps} steps");
    tally
}

/// WP12's chunk sizes for a file of `size` bytes.
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

fn chunk_size_cases(append: u32) -> Vec<Case> {
    let mut cases = Vec::new();
    let mut rng = Rng(u64::from(append) + 160);
    for path in vectors() {
        let size = std::fs::metadata(&path).unwrap().len();
        for chunk in chunk_sizes(size) {
            let mode = rng.below(13) as u32;
            cases.push(case(
                &path,
                -1,
                chunk,
                append,
                "config",
                mode,
                Options::default(),
                Geometry::default(),
            ));
        }
    }
    cases
}

#[test]
fn incremental_append_matches_at_every_chunk_size() {
    let Some(oracle) = Oracle::locate("incremental_append_matches_at_every_chunk_size") else {
        return;
    };
    let tally = run(
        &oracle,
        "incremental_append_matches_at_every_chunk_size",
        chunk_size_cases(1),
    );
    assert_eq!(tally.skipped, 0);
}

#[test]
fn incremental_update_matches_at_every_chunk_size() {
    let Some(oracle) = Oracle::locate("incremental_update_matches_at_every_chunk_size") else {
        return;
    };
    let tally = run(
        &oracle,
        "incremental_update_matches_at_every_chunk_size",
        chunk_size_cases(0),
    );
    assert_eq!(tally.skipped, 0);
}

#[test]
fn incremental_decoders_match_with_options_and_memory() {
    let Some(oracle) = Oracle::locate("incremental_decoders_match_with_options_and_memory") else {
        return;
    };
    let mut cases = Vec::new();
    let mut rng = Rng(4216);
    for path in vectors() {
        let (width, height, _) = dimensions(&path);
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
        // WP12's crop, whose top is odd for most images, and the same at even
        // offsets.
        for even in [false, true] {
            let mask = if even { !1 } else { !0 };
            with(&[
                (Options::CROP, 1),
                (Options::CROP_LEFT, (width / 3) & mask),
                (Options::CROP_TOP, (height / 5 + 1) & mask),
                (Options::CROP_WIDTH, (width / 2).max(1)),
                (Options::CROP_HEIGHT, (height / 2).max(1)),
            ]);
        }
        with(&[
            (Options::SCALE, 1),
            (Options::SCALED_WIDTH, (width / 2).max(1)),
            (Options::SCALED_HEIGHT, (height * 2 / 3).max(1)),
            (Options::FLIP, 1),
        ]);
        // A crop outside the image, which libwebp refuses at the header.
        with(&[
            (Options::CROP, 1),
            (Options::CROP_LEFT, (width + 1) & !1),
            (Options::CROP_WIDTH, 1),
            (Options::CROP_HEIGHT, 1),
        ]);
        for options in option_sets {
            let mode = rng.below(13) as u32;
            cases.push(case(
                &path,
                -1,
                chunk,
                append,
                "config",
                mode,
                options,
                Geometry::default(),
            ));
            if let Some((w, h)) = output_dimensions(width, height, &options) {
                let mode = rng.below(13) as u32;
                let geometry = geometry_for(mode, w, h, rng.range(0, 7) as i32, mode == 12);
                cases.push(case(
                    &path,
                    -1,
                    chunk,
                    1 - append,
                    "config",
                    mode,
                    options,
                    geometry,
                ));
            }
        }
        // Caller memory that is one byte too short for the image, which
        // libwebp refuses when the header arrives.
        let mut short = geometry_for(3, width, height, 0, false);
        short.0[0].1 -= 1;
        cases.push(case(
            &path,
            -1,
            chunk,
            append,
            "config",
            3,
            Options::default(),
            short,
        ));
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
            cases.push(case(
                &path,
                -1,
                chunk,
                append,
                kind,
                mode,
                Options::default(),
                geometry,
            ));
        }
        // Truncated input and mixed functions.
        let len = size as i64;
        for (cut, append) in [(len * 2 / 3, 1), (len - 1, 0), (-1, 2)] {
            let mode = rng.below(13) as u32;
            cases.push(case(
                &path,
                cut,
                64,
                append,
                "config",
                mode,
                Options::default(),
                Geometry::default(),
            ));
        }
    }
    // Constructor arguments that libwebp refuses (and the API with them).
    let path = oracle::test_data().join("test.webp");
    for (kind, mode, geometry) in [
        ("rgb", 1, Geometry(vec![(0, 400)], false)),
        ("rgb", 1, Geometry(vec![(40, 0)], false)),
        (
            "yuva",
            12,
            Geometry(vec![(16, 256), (0, 64), (0, 64)], false),
        ),
        (
            "yuva",
            12,
            Geometry(vec![(16, 256), (8, 64), (8, 64), (16, 0)], false),
        ),
        ("yuv", 11, Geometry(vec![(16, 0), (8, 64), (8, 64)], false)),
    ] {
        cases.push(case(
            &path,
            -1,
            4096,
            1,
            kind,
            mode,
            Options::default(),
            geometry,
        ));
    }
    let tally = run(
        &oracle,
        "incremental_decoders_match_with_options_and_memory",
        cases,
    );
    assert_eq!(tally.skipped, 0, "{tally:?}");
    assert!(tally.refused.get(&Deviation::OddCrop).copied().unwrap_or(0) > 0);
}

#[test]
fn incremental_threads_match_on_wide_lossy_images() {
    let Some(oracle) = Oracle::locate("incremental_threads_match_on_wide_lossy_images") else {
        return;
    };
    let mut cases = Vec::new();
    let mut rng = Rng(7716);
    for name in [
        "lossy_alpha1.webp",
        "lossy_alpha2.webp",
        "lossy_alpha3.webp",
        "vp80-05-sharpness-1443.webp",
        "lossy_extreme_probabilities.webp",
        "vp80-00-comprehensive-008.webp",
    ] {
        let path = oracle::test_data().join(name);
        let (width, height, _) = dimensions(&path);
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
                geometry_for(mode, width, height, 2, mode == 12)
            };
            cases.push(case(
                &path, -1, chunk, append, "config", mode, options, geometry,
            ));
        }
    }
    let tally = run(
        &oracle,
        "incremental_threads_match_on_wide_lossy_images",
        cases,
    );
    assert_eq!(tally.skipped, 0);
}
