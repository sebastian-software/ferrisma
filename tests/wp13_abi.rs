//! WP13 differential test of the C ABI: the probes of `tools/oracle/probes/wp13/`
//! built against the C library and against the Rust static library through
//! `src/abi/encode.rs` (`probes/wp13/<name>_rs`, the consumer build of
//! `tools/oracle/probes/wp13/oracle.cmake`) read the same input, and the outputs
//! must be identical, byte for byte.
//!
//! Excluded (undefined behavior in C): RGBA rows imported into an ARGB picture
//! whose starts are not aligned for `uint32_t` (`docs/status/wp9.md`). Excluded
//! (a failed C assertion): `WebPPlaneDistortion` with `x_step` other than 1 on
//! an empty plane, which calls `WebPSafeMalloc` with a size of 0.
//!
//! - `encode`: the whole encoder matrix of `tests/wp13_encode.rs`
//!   (`support/wp13_matrix.rs`) through `WebPEncode`, with library memory,
//!   caller memory and views (whose source the C ABI writes through, as C
//!   does), `WebPMemoryWriter` and custom writers, progress hooks and aborts.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp13_cases.rs"]
mod wp13_cases;
#[path = "support/wp13_matrix.rs"]
mod wp13_matrix;

use oracle::{Fields, Oracle};
use wp13_cases::{Case, records, split_records};
use wp13_matrix as matrix;

/// Runs the probe `name` and its Rust-linked build on `input` and compares the
/// outputs line by line.
fn compare_probe(test: &str, oracle: &Oracle, name: &str, input: &[u8]) -> Fields {
    let c = oracle.probe("wp13", name).run::<&str>(&[], input);
    c.success();
    let rust = oracle
        .probe("wp13", &format!("{name}_rs"))
        .run::<&str>(&[], input);
    rust.success();
    let (c_fields, rust_fields) = (c.fields(), rust.fields());
    for (index, (a, b)) in c_fields.0.iter().zip(&rust_fields.0).enumerate() {
        assert_eq!(a, b, "{test}: line {index} differs");
    }
    assert_eq!(
        c_fields.0.len(),
        rust_fields.0.len(),
        "{test}: the outputs have different lengths"
    );
    assert_eq!(
        c.stderr, rust.stderr,
        "{test}: the probes wrote different diagnostics"
    );
    c_fields
}

/// Runs one group of the encoder matrix through both builds of `encode`; the
/// outputs must be identical, line by line.
fn run_encode(test: &str, cases: impl FnOnce(&Oracle) -> Vec<Case>) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let cases = cases(&oracle);
    let fields = compare_probe(test, &oracle, "encode", &records(&cases));
    let outputs = split_records(fields);
    assert_eq!(outputs.len(), cases.len(), "{test}: one output per record");
    let encoded = outputs.iter().filter(|f| f.get("ret") == Some("1")).count();
    // Failed encodings with an extra_info map whose prefix C filled (the map
    // starts as zeros, deviation 3).
    let partial = cases
        .iter()
        .zip(&outputs)
        .filter(|(case, f)| {
            f.get("ret") == Some("0")
                && f.get("extra")
                    .is_some_and(|map| map[..map.len() - 2].bytes().any(|digit| digit != b'0'))
                && case.extra_info_type > 0
        })
        .count();
    eprintln!(
        "{test}: {} cases, {encoded} encoded, {partial} failed with a partly filled extra_info map",
        cases.len()
    );
}

#[test]
fn abi_lossy_quality_method_and_preset_match_c() {
    run_encode("abi_lossy_quality_method_and_preset_match_c", |_| {
        matrix::lossy_quality_method_and_preset_cases()
    });
}

#[test]
fn abi_lossy_tools_match_c() {
    run_encode("abi_lossy_tools_match_c", |_| matrix::lossy_tools_cases());
}

#[test]
fn abi_lossy_alpha_matches_c() {
    run_encode("abi_lossy_alpha_matches_c", |_| matrix::lossy_alpha_cases());
}

#[test]
fn abi_lossless_matches_c() {
    run_encode("abi_lossless_matches_c", |_| matrix::lossless_cases());
}

#[test]
fn abi_rate_control_matches_c() {
    run_encode("abi_rate_control_matches_c", |_| {
        matrix::rate_control_cases()
    });
}

#[test]
fn abi_progress_aborts_and_writers_match_c() {
    run_encode("abi_progress_aborts_and_writers_match_c", |_| {
        matrix::progress_aborts_and_writers_cases()
    });
}

#[test]
fn abi_views_and_caller_memory_match_c() {
    run_encode("abi_views_and_caller_memory_match_c", |_| {
        matrix::views_and_caller_memory_cases()
    });
}

#[test]
fn abi_dimensions_and_invalid_arguments_match_c() {
    run_encode("abi_dimensions_and_invalid_arguments_match_c", |_| {
        matrix::dimensions_and_invalid_arguments_cases()
    });
}

#[test]
fn abi_random_configurations_match_c() {
    run_encode("abi_random_configurations_match_c", |_| {
        matrix::random_configurations_cases()
    });
}

#[test]
fn abi_vectors_match_c() {
    run_encode("abi_vectors_match_c", matrix::vector_cases);
}

//------------------------------------------------------------------------------
// The api probe: everything of encode.h apart from WebPEncode's matrix.

/// SplitMix64 for the command generators.
struct Seq(u64);

impl Seq {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: i64) -> i64 {
        (self.next() % bound as u64) as i64
    }
}

/// Whether C imports RGBA rows into an ARGB picture (`WebPPictureImportRGBA`
/// with `use_argb`, or `WebPEncodeLosslessRGBA`) whose starts are not aligned
/// for `uint32_t`: `VP8LConvertBGRAToRGBA` loads them as `uint32_t`, which is
/// undefined (WP9 records it; the asan-ubsan oracle reports it). Such rows
/// are not generated.
fn misaligned_rgba_rows(rgba_into_argb: bool, height: i64, stride: i64) -> bool {
    rgba_into_argb && height > 1 && stride % 4 != 0
}

/// Runs `commands` through both builds of the api probe.
fn run_api(test: &str, commands: &[String]) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let fields = compare_probe(test, &oracle, "api", commands.concat().as_bytes());
    let count = fields.0.iter().filter(|(key, _)| key == "cmd").count();
    assert_eq!(count, commands.len(), "{test}: one output per command");
    // Per command: how often it ran and how often it returned 1 (or a size).
    let mut tally: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
    let mut current = String::new();
    for (key, value) in &fields.0 {
        match key.as_str() {
            "cmd" => {
                current.clone_from(value);
                tally.entry(current.clone()).or_default().0 += 1;
            }
            "ret" if value == "1" => tally.entry(current.clone()).or_default().1 += 1,
            "size" if value != "0" => tally.entry(current.clone()).or_default().1 += 1,
            _ => {}
        }
    }
    eprintln!(
        "{test}: {count} commands, {} lines; ran/succeeded: {tally:?}",
        fields.0.len()
    );
}

#[test]
fn abi_answers_null_arguments_versions_and_configurations_as_c() {
    let mut commands = vec!["version\n".to_owned(), "null\n".to_owned()];
    let nan = f32::NAN.to_bits() as i32;
    for preset in 0..=6 {
        for quality in [0.0_f32, 75.0, 100.0, -1.0, 101.0, 50.5] {
            for version in [0x0210, 0x0200, 0x02ff, 0x0310, 0x0110] {
                commands.push(format!(
                    "config {preset} {} {version}\n",
                    quality.to_bits() as i32
                ));
            }
        }
        commands.push(format!("config {preset} {nan} 528\n"));
    }
    for level in -1..=10 {
        commands.push(format!("lpreset {level}\n"));
    }
    for field in 0..29 {
        for value in [-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 63, 64, 100, 101] {
            commands.push(format!("validate {field} {value}\n"));
        }
    }
    run_api(
        "abi_answers_null_arguments_versions_and_configurations_as_c",
        &commands,
    );
}

#[test]
fn abi_memory_writer_matches_c() {
    let commands: Vec<String> = [
        "writer 0 1 8191 1 8192 100000",
        "writer 10000",
        "writer 0",
        "writer 5 5 5 5 5 5 5",
        "writer 8192 0 16384 1",
    ]
    .iter()
    .map(|line| format!("{line}\n"))
    .collect();
    run_api("abi_memory_writer_matches_c", &commands);
}

#[test]
fn abi_simple_api_matches_c() {
    let mut commands = Vec::new();
    let mut seq = Seq(0x13a1);
    for f in 0..8 {
        let step = if f % 4 < 2 { 3 } else { 4 };
        for (w, h) in [(1, 1), (7, 5), (16, 16), (33, 17), (0, 4), (4, 0), (-3, 2)] {
            let tight = step * w.max(1);
            for stride in [tight, tight + 5, -tight, -(tight + 3), tight - 1, 0] {
                for quality in [0.0_f32, 75.0, 100.0, -1.0] {
                    if f >= 4 && quality != 75.0 {
                        continue;
                    }
                    let seed = seq.below(1 << 30);
                    let amode = seq.below(3);
                    if misaligned_rgba_rows(f == 6, i64::from(h), i64::from(stride)) {
                        continue;
                    }
                    commands.push(format!(
                        "simple {f} {w} {h} {stride} {} {seed} {amode} 0\n",
                        quality.to_bits() as i32
                    ));
                }
            }
        }
        for flags in [1, 2, 3] {
            commands.push(format!(
                "simple {f} 4 4 {} 1112014848 9 0 {flags}\n",
                4 * step
            ));
            commands.push(format!(
                "simple {f} 4 4 {} -1082130432 9 0 {flags}\n",
                4 * step
            ));
        }
    }
    run_api("abi_simple_api_matches_c", &commands);
}

/// A picture of `seq`'s choosing in P[0], imported from generated rows, maybe
/// moved into caller memory; returns its width, height and `use_argb`.
fn start_picture(seq: &mut Seq, commands: &mut Vec<String>) -> (i64, i64, i64) {
    let (w, h) = (1 + seq.below(40), 1 + seq.below(30));
    let use_argb = seq.below(2);
    let kind = seq.below(6);
    let step = if kind == 0 || kind == 3 { 3 } else { 4 };
    let mut stride = match seq.below(4) {
        0 => step * w,
        1 => step * w + 1 + seq.below(9),
        2 => -(step * w + seq.below(5)),
        _ => step * w + 4 * seq.below(3),
    };
    if misaligned_rgba_rows(kind == 1 && use_argb == 1, h, stride) {
        // Rows that C loads as misaligned uint32_t (excluded, see the module
        // documentation): round the stride up to a multiple of 4.
        stride = stride.signum() * ((stride.abs() + 3) / 4 * 4);
    }
    commands.push("init 0\n".to_owned());
    commands.push(format!("set 0 {use_argb} 0 {w} {h}\n"));
    let seed = seq.below(1 << 30);
    let amode = seq.below(3);
    commands.push(format!("import 0 {kind} {stride} {seed} {amode}\n"));
    if seq.below(3) == 0 {
        commands.push(format!("caller 0 {}\n", seq.below(6)));
    }
    (w, h, use_argb)
}

#[test]
fn abi_picture_functions_match_c() {
    let mut commands = Vec::new();
    let mut seq = Seq(0x13a2);
    // Allocation, including invalid dimensions and colorspaces.
    for (use_argb, cs, w, h) in [
        (0, 0, 7, 5),
        (0, 4, 7, 5),
        (1, 0, 7, 5),
        (0, 0, 0, 5),
        (1, 0, 5, -1),
        (0, 1, 4, 4),
        (0, 4, 1, 1),
        (1, 4, 16384, 2),
    ] {
        commands.push("init 3\n".to_owned());
        commands.push(format!("set 3 {use_argb} {cs} {w} {h}\n"));
        commands.push("alloc 3\n".to_owned());
        commands.push("isview 3\n".to_owned());
        commands.push("alloc 3\n".to_owned());
        commands.push("free 3\n".to_owned());
        commands.push("isview 3\n".to_owned());
    }
    for _ in 0..160 {
        let (w, h, _) = start_picture(&mut seq, &mut commands);
        for _ in 0..1 + seq.below(5) {
            match seq.below(13) {
                0 => {
                    commands.push("copy 0 1\n".to_owned());
                    commands.push(format!("distortion 0 1 {}\n", seq.below(3)));
                    commands.push("free 1\n".to_owned());
                }
                1 => {
                    let (l, t) = (seq.below(w + 2) - 1, seq.below(h + 2) - 1);
                    let (vw, vh) = (1 + seq.below(w), 1 + seq.below(h));
                    commands.push("init 2\n".to_owned());
                    commands.push(format!("view 0 {l} {t} {vw} {vh} 2\n"));
                    commands.push("isview 2\n".to_owned());
                    commands.push("transparent 2\n".to_owned());
                    match seq.below(4) {
                        0 => commands.push("cleanup 2\n".to_owned()),
                        1 => commands.push(format!("blend 2 {}\n", seq.below(1 << 24))),
                        2 => commands.push("yuva2argb 2\n".to_owned()),
                        _ => commands.push("argb2yuva 2 0 -1\n".to_owned()),
                    }
                    commands.push("print 0\n".to_owned());
                    commands.push("free 2\n".to_owned());
                }
                2 => {
                    let (l, t) = (seq.below(w + 1), seq.below(h + 1));
                    let (cw, ch) = (seq.below(w + 1), seq.below(h + 1));
                    commands.push(format!("crop 0 {l} {t} {cw} {ch}\n"));
                    break;
                }
                3 => {
                    let (rw, rh) = (seq.below(2 * w + 2), seq.below(2 * h + 2));
                    commands.push(format!("rescale 0 {rw} {rh}\n"));
                    break;
                }
                4 => {
                    let dithering = [
                        -1_i32,
                        0,
                        0.5_f32.to_bits() as i32,
                        1.0_f32.to_bits() as i32,
                    ][seq.below(4) as usize];
                    commands.push(format!(
                        "argb2yuva 0 {} {dithering}\n",
                        [0, 4, 1][seq.below(3) as usize]
                    ));
                }
                5 => commands.push("sharp 0\n".to_owned()),
                6 => commands.push("smart 0\n".to_owned()),
                7 => commands.push("yuva2argb 0\n".to_owned()),
                8 => commands.push("cleanup 0\n".to_owned()),
                9 => commands.push(format!("blend 0 {}\n", seq.below(1 << 24))),
                10 => {
                    commands.push("transparent 0\n".to_owned());
                    commands.push("isview 0\n".to_owned());
                }
                11 => {
                    commands.push("copy 0 1\n".to_owned());
                    commands.push(format!("rescale 1 {} {}\n", 1 + seq.below(w), 0));
                    commands.push(format!("distortion 0 1 {}\n", seq.below(3)));
                    commands.push("free 1\n".to_owned());
                }
                _ => {
                    commands.push(format!("caller 0 {}\n", seq.below(4)));
                }
            }
        }
        commands.push("print 0\n".to_owned());
        commands.push("free 0\n".to_owned());
    }
    for _ in 0..60 {
        let (w, h) = (seq.below(20), seq.below(12));
        let x_step = [1, 1, 2, 4][seq.below(4) as usize];
        let stride = x_step * w + seq.below(4) - 1;
        if x_step != 1 && (w == 0 || h == 0) {
            // C allocates its packed planes with WebPSafeMalloc(0), which fails
            // upstream's assertion (excluded, see the module documentation).
            continue;
        }
        commands.push(format!(
            "planedist {w} {h} {stride} {x_step} {} {} {}\n",
            seq.below(3),
            seq.below(1 << 30),
            seq.below(1 << 30)
        ));
    }
    run_api("abi_picture_functions_match_c", &commands);
}

#[test]
fn abi_accepts_the_layouts_that_c_never_reads() {
    // Calls that leave a storage alone (the one that use_argb does not select,
    // `WebPPictureView`'s samples, the luma and chroma planes for
    // `WebPPictureHasTransparency`) accept any address and stride there, as C
    // does: junk pointers with negative strides, and planes turned upside down.
    let mut commands = Vec::new();
    let mut seq = Seq(0x13a3);
    for round in 0..80 {
        let (w, h, use_argb) = start_picture(&mut seq, &mut commands);
        if round % 2 == 0 {
            commands.push("junk 0\n".to_owned());
            commands.push("transparent 0\n".to_owned());
            commands.push("isview 0\n".to_owned());
            let (l, t) = (seq.below(w), seq.below(h));
            commands.push("init 2\n".to_owned());
            commands.push(format!("view 0 {l} {t} {} {} 2\n", w - l, h - t));
            commands.push("free 2\n".to_owned());
            commands.push("copy 0 1\n".to_owned());
            commands.push("free 1\n".to_owned());
            if use_argb == 1 {
                commands.push(format!("encode 0 1 {}\n", seq.below(7)));
            }
            commands.push(format!("encode 0 0 {}\n", seq.below(7)));
            if use_argb == 0 {
                match seq.below(3) {
                    0 => commands.push("cleanup 0\n".to_owned()),
                    1 => commands.push(format!("blend 0 {}\n", seq.below(1 << 24))),
                    _ => commands.push(format!("crop 0 1 1 {} {}\n", w.max(2) - 1, h.max(2) - 1)),
                }
            }
        } else {
            // Upside-down luma and chroma, or an upside-down ARGB plane: a view
            // only computes addresses; the alpha plane, which
            // WebPPictureHasTransparency reads, stays upright.
            commands.push(format!("flip 0 {}\n", if use_argb == 1 { 8 } else { 3 }));
            if use_argb == 0 {
                commands.push("transparent 0\n".to_owned());
            }
            let (l, t) = (seq.below(w), seq.below(h));
            commands.push("init 2\n".to_owned());
            commands.push(format!("view 0 {l} {t} {} {} 2\n", w - l, h - t));
            commands.push("print 2\n".to_owned());
            commands.push("free 2\n".to_owned());
        }
        commands.push("free 0\n".to_owned());
    }
    run_api("abi_accepts_the_layouts_that_c_never_reads", &commands);
}
