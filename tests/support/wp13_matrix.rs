//! The case groups of the WP13 encoder matrix (`docs/PORT_PLAN.md`, Phase 3),
//! shared by `tests/wp13_encode.rs` (the Rust translation against the oracle)
//! and `tests/wp13_abi.rs` (the C ABI against the oracle). Each group is one
//! test; `support/wp13_cases.rs` holds the records and the replay.
//!
//! Include with `#[path = "support/wp13_matrix.rs"] mod wp13_matrix;` next to
//! `support/oracle.rs` and `support/wp13_cases.rs`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use super::oracle::{self, Oracle};
use super::wp13_cases::{
    ALPHAS, Alpha, CONTENTS, Case, Content, F, Layout, Rng, picture, read_pnm,
};

/// A seeded case.
pub fn case(rng: &mut Rng, content: Content, alpha: Alpha, width: i32, height: i32) -> Case {
    Case::new(width, height, picture(rng, content, alpha, width, height))
}

/// Drops the statistics where C reads uninitialized memory (see the module
/// documentation).
pub fn exclude_undefined(mut case: Case) -> Case {
    let has_alpha = case.source == 1 || case.source == 2;
    let lossless = case.get(F::Lossless) == 1 || case.lossless_level.is_some();
    if has_alpha && !lossless && case.get(F::AlphaCompression) == 0 {
        case.with_stats = false;
    }
    if has_alpha
        && !lossless
        && case.get(F::ThreadLevel) > 0
        && (case.abort_at >= 0 || case.fail_write_at >= 0)
    {
        case.with_stats = false;
    }
    assert_ne!(case.extra_info_type, 6, "extra_info_type 6 is excluded");
    case
}

/// The cases of `lossy_quality_method_and_preset_match_the_c_encoder`.
pub fn lossy_quality_method_and_preset_cases() -> Vec<Case> {
    let mut rng = Rng(0x1301);
    let opaque = case(&mut rng, Content::Photo, Alpha::Opaque, 24, 20);
    let alpha = case(&mut rng, Content::Palette, Alpha::Gradient, 17, 13);
    let mut cases = Vec::new();
    for &quality in &[0.0, 10.0, 50.0, 75.0, 90.0, 100.0] {
        for method in 0..=6 {
            for preset in 0..=5 {
                // Opaque YUV input; alpha ARGB input, converted by WebPEncode.
                let mut c = opaque.clone();
                c.source = 0;
                c.preset = Some(preset);
                c.quality = quality;
                c.set(F::Method, method);
                cases.push(c);
                let mut c = alpha.clone();
                c.source = 2;
                c.preset = Some(preset);
                c.quality = quality;
                c.set(F::Method, method);
                c.set(F::AlphaCompression, 1);
                cases.push(exclude_undefined(c));
            }
        }
    }
    cases
}

/// The cases of `lossy_tools_match_the_c_encoder`.
pub fn lossy_tools_cases() -> Vec<Case> {
    let mut rng = Rng(0x1302);
    let mut cases = Vec::new();
    let pictures = [
        case(&mut rng, Content::Photo, Alpha::Opaque, 40, 36),
        case(&mut rng, Content::Noise, Alpha::Binary, 35, 33),
    ];
    for (index, base) in pictures.iter().enumerate() {
        for source in [index as i32, 2 + (index as i32 ^ 1)] {
            // segments, partitions, low_memory, thread_level by method.
            for segments in [1, 4] {
                for partitions in [0, 3] {
                    for low_memory in [0, 1] {
                        for threads in [0, 1] {
                            for method in [2, 4, 6] {
                                let mut c = base.clone();
                                c.source = source;
                                c.set(F::Segments, segments)
                                    .set(F::Partitions, partitions)
                                    .set(F::LowMemory, low_memory)
                                    .set(F::ThreadLevel, threads)
                                    .set(F::Method, method)
                                    .set(F::AlphaCompression, 1);
                                cases.push(exclude_undefined(c));
                            }
                        }
                    }
                }
            }
            // SNS and filter defaults and extremes, filter type, autofilter.
            for sns in [0, 50, 100] {
                for strength in [0, 60, 100] {
                    for filter_type in [0, 1] {
                        for autofilter in [0, 1] {
                            let mut c = base.clone();
                            c.source = source;
                            c.set(F::Sns, sns)
                                .set(F::FilterStrength, strength)
                                .set(F::FilterType, filter_type)
                                .set(F::Autofilter, autofilter)
                                .set(F::FilterSharpness, (sns / 20 + strength / 30) % 8)
                                .set(F::AlphaCompression, 1);
                            cases.push(exclude_undefined(c));
                        }
                    }
                }
            }
            // emulate_jpeg_size, sharp YUV and the preprocessing bits.
            for jpeg in [0, 1] {
                for quality in [10.0, 75.0, 95.0] {
                    let mut c = base.clone();
                    c.source = source;
                    c.quality = quality;
                    c.set(F::EmulateJpegSize, jpeg).set(F::AlphaCompression, 1);
                    cases.push(exclude_undefined(c));
                }
            }
            for sharp in [0, 1] {
                for preprocessing in [0, 1, 2, 4, 7] {
                    let mut c = base.clone();
                    c.source = source;
                    c.set(F::UseSharpYuv, sharp)
                        .set(F::Preprocessing, preprocessing)
                        .set(F::AlphaCompression, 1);
                    cases.push(exclude_undefined(c));
                }
            }
            // exact, show_compressed and extra_info.
            for exact in [0, 1] {
                for show in [0, 1] {
                    for extra in [0, 1, 2, 3, 4, 5, 7, 8] {
                        let mut c = base.clone();
                        c.source = source;
                        c.extra_info_type = extra;
                        c.set(F::Exact, exact)
                            .set(F::ShowCompressed, show)
                            .set(F::AlphaCompression, 1);
                        cases.push(exclude_undefined(c));
                    }
                }
            }
        }
    }
    cases
}

/// The cases of `lossy_alpha_matches_the_c_encoder`.
pub fn lossy_alpha_cases() -> Vec<Case> {
    let mut rng = Rng(0x1303);
    let mut cases = Vec::new();
    for alpha in [Alpha::Binary, Alpha::Gradient, Alpha::Noise] {
        let base = case(&mut rng, Content::Photo, alpha, 33, 31);
        for quality in [0, 50, 100] {
            for filtering in 0..=2 {
                for compression in [0, 1] {
                    for threads in [0, 1] {
                        let mut c = base.clone();
                        c.source = rng.pick(&[1, 2]);
                        c.set(F::AlphaQuality, quality)
                            .set(F::AlphaFiltering, filtering)
                            .set(F::AlphaCompression, compression)
                            .set(F::ThreadLevel, threads)
                            .set(F::Method, rng.pick(&[0, 3, 4, 6]))
                            .set(F::Exact, rng.pick(&[0, 1]));
                        cases.push(exclude_undefined(c));
                    }
                }
            }
        }
    }
    cases
}

/// The cases of `lossless_matches_the_c_encoder`.
pub fn lossless_cases() -> Vec<Case> {
    let mut rng = Rng(0x1304);
    let mut cases = Vec::new();
    let pictures = [
        case(&mut rng, Content::Photo, Alpha::Binary, 13, 11),
        case(&mut rng, Content::Palette, Alpha::Noise, 12, 9),
    ];
    for (index, base) in pictures.iter().enumerate() {
        for quality in [0, 50, 100] {
            for method in 0..=6 {
                for exact in [0, 1] {
                    for near in [0, 40, 60, 80, 100] {
                        for delta in [0, 1] {
                            let mut c = base.clone();
                            c.source = if (method + near) % 2 == 0 { 2 } else { 1 };
                            c.quality = quality as f32;
                            c.set(F::Lossless, 1)
                                .set(F::Method, method)
                                .set(F::Exact, exact)
                                .set(F::NearLossless, near)
                                .set(F::UseDeltaPalette, delta)
                                .set(F::ThreadLevel, (index as i32 + method) % 2);
                            cases.push(c);
                        }
                    }
                }
            }
        }
        for level in 0..=9 {
            for hint in 0..=3 {
                let mut c = base.clone();
                c.source = rng.pick(&[0, 1, 2, 3]);
                c.lossless_level = Some(level);
                c.set(F::ImageHint, hint)
                    .set(F::ThreadLevel, rng.pick(&[0, 1]));
                cases.push(c);
            }
        }
    }
    cases
}

/// The cases of `rate_control_matches_the_c_encoder`.
pub fn rate_control_cases() -> Vec<Case> {
    let mut rng = Rng(0x1305);
    let mut cases = Vec::new();
    let base = case(&mut rng, Content::Photo, Alpha::Opaque, 64, 48);
    for pass in [1, 3, 10] {
        for method in [2, 4, 6] {
            for size in [300, 1500, 6000] {
                let mut c = base.clone();
                c.source = 0;
                c.set(F::Pass, pass)
                    .set(F::Method, method)
                    .set(F::TargetSize, size);
                cases.push(c);
            }
            for psnr in [30.0_f32, 38.5, 45.0] {
                let mut c = base.clone();
                c.source = 0;
                c.set(F::Pass, pass)
                    .set(F::Method, method)
                    .set(F::TargetPsnr, psnr.to_bits() as i32);
                cases.push(c);
            }
            let mut c = base.clone();
            c.source = 0;
            c.set(F::Pass, pass)
                .set(F::Method, method)
                .set(F::TargetSize, 1000)
                .set(F::Qmin, 20)
                .set(F::Qmax, 60);
            cases.push(c);
        }
    }
    cases
}

/// A picture, its source kind and configuration fields.
type Setting<'a> = (&'a Case, i32, &'a [(F, i32)]);

/// The cases of `progress_aborts_and_writers_match_the_c_encoder`.
pub fn progress_aborts_and_writers_cases() -> Vec<Case> {
    let mut rng = Rng(0x1306);
    let mut cases = Vec::new();
    let opaque = case(&mut rng, Content::Photo, Alpha::Opaque, 48, 40);
    let alpha = case(&mut rng, Content::Flat, Alpha::Binary, 40, 34);
    let configs: [Setting<'_>; 6] = [
        (&opaque, 0, &[(F::Method, 4)]),
        (&opaque, 0, &[(F::Method, 2), (F::Partitions, 2)]),
        (&alpha, 2, &[(F::AlphaCompression, 1), (F::ThreadLevel, 1)]),
        (&alpha, 1, &[(F::AlphaCompression, 1), (F::Pass, 3)]),
        (&alpha, 2, &[(F::Lossless, 1), (F::Method, 4)]),
        (&opaque, 3, &[(F::Lossless, 1), (F::ThreadLevel, 1)]),
    ];
    for (base, source, settings) in configs {
        let configure = |mut c: Case| {
            c.source = source;
            for &(field, value) in settings {
                c.set(field, value);
            }
            c
        };
        // No hook, then an abort at each of the first 16 progress calls.
        let mut c = configure(base.clone());
        c.abort_at = -2;
        cases.push(exclude_undefined(c));
        for abort_at in 0..16 {
            let mut c = configure(base.clone());
            c.abort_at = abort_at;
            cases.push(exclude_undefined(c));
        }
        // A writer failure at each of the first 12 calls.
        for fail in 0..12 {
            let mut c = configure(base.clone());
            c.fail_write_at = fail;
            cases.push(exclude_undefined(c));
        }
        // WebPMemoryWriter.
        let mut c = configure(base.clone());
        c.memory_writer = true;
        cases.push(exclude_undefined(c));
    }
    cases
}

/// The cases of `views_and_caller_memory_match_the_c_encoder`.
pub fn views_and_caller_memory_cases() -> Vec<Case> {
    let mut rng = Rng(0x1307);
    let mut cases = Vec::new();
    for round in 0..64 {
        let alpha = ALPHAS[round % 4];
        let content = CONTENTS[(round / 4) % 4];
        let (width, height) = (5 + rng.below(40) as i32, 3 + rng.below(40) as i32);
        let mut c = case(&mut rng, content, alpha, width, height);
        c.source = (round % 4) as i32;
        c.layout = if round % 2 == 0 {
            Layout::Caller {
                pad: rng.below(9) as i32,
            }
        } else {
            let left = rng.below(width as u64) as i32;
            let top = rng.below(height as u64) as i32;
            Layout::View {
                left,
                top,
                width: 1 + rng.below((width - left) as u64) as i32,
                height: 1 + rng.below((height - top) as u64) as i32,
            }
        };
        let lossless = rng.below(3) == 0;
        c.set(F::Lossless, i32::from(lossless))
            .set(F::Exact, rng.pick(&[0, 0, 1]))
            .set(F::ShowCompressed, rng.pick(&[0, 1]))
            .set(F::Method, rng.pick(&[0, 2, 4, 6]))
            .set(F::AlphaCompression, 1)
            .set(F::ThreadLevel, rng.pick(&[0, 1]));
        c.memory_writer = rng.below(2) == 0;
        cases.push(exclude_undefined(c));
    }
    // A view whose rectangle lies outside the picture is refused.
    let mut c = case(&mut rng, Content::Photo, Alpha::Opaque, 10, 10);
    c.layout = Layout::View {
        left: 4,
        top: 4,
        width: 8,
        height: 2,
    };
    cases.push(c);
    cases
}

/// The cases of `dimensions_and_invalid_arguments_match_the_c_encoder`.
pub fn dimensions_and_invalid_arguments_cases() -> Vec<Case> {
    let mut rng = Rng(0x1308);
    let mut cases = Vec::new();
    let sizes = [
        (1, 1),
        (1, 2),
        (2, 1),
        (3, 5),
        (15, 17),
        (16, 16),
        (17, 1),
        (1, 31),
        (16383, 1),
        (1, 16383),
        (2, 16383),
        (16384, 1),
        (1, 16384),
    ];
    for (width, height) in sizes {
        for lossless in [0, 1] {
            let alpha = if width * height > 64 {
                Alpha::Opaque
            } else {
                rng.pick(&ALPHAS)
            };
            let mut c = case(&mut rng, Content::Photo, alpha, width, height);
            c.source = rng.pick(&[0, 1, 2, 3]);
            c.set(F::Lossless, lossless).set(F::AlphaCompression, 1);
            cases.push(exclude_undefined(c));
        }
    }
    // Configurations that WebPValidateConfig rejects.
    let base = case(&mut rng, Content::Photo, Alpha::Opaque, 9, 7);
    let invalid: [(F, i32); 12] = [
        (F::Method, 7),
        (F::Method, -1),
        (F::Segments, 5),
        (F::Partitions, 4),
        (F::Sns, 101),
        (F::FilterSharpness, 8),
        (F::Pass, 11),
        (F::AlphaFiltering, 3),
        (F::NearLossless, 101),
        (F::ImageHint, 4),
        (F::Qmin, 70),
        (F::ThreadLevel, -1),
    ];
    for (field, value) in invalid {
        let mut c = base.clone();
        c.set(field, value);
        if field == F::Qmin {
            c.set(F::Qmax, 60);
        }
        cases.push(c);
    }
    let mut c = base.clone();
    c.quality = 100.5;
    cases.push(c);
    cases
}

/// The cases of `random_configurations_match_the_c_encoder`.
pub fn random_configurations_cases() -> Vec<Case> {
    let mut rng = Rng(0x1309);
    let mut cases = Vec::new();
    for _ in 0..240 {
        let (width, height) = (1 + rng.below(70) as i32, 1 + rng.below(50) as i32);
        let alpha = rng.pick(&ALPHAS);
        let content = rng.pick(&CONTENTS);
        let mut c = case(&mut rng, content, alpha, width, height);
        c.source = rng.below(4) as i32;
        c.preset = if rng.below(3) == 0 {
            None
        } else {
            Some(rng.below(6) as u32)
        };
        c.quality = rng.below(101) as f32;
        let lossless = rng.below(3) == 0;
        c.set(F::Lossless, i32::from(lossless))
            .set(F::Method, rng.below(7) as i32)
            .set(F::Segments, 1 + rng.below(4) as i32)
            .set(F::Partitions, rng.below(4) as i32)
            .set(F::Sns, rng.below(101) as i32)
            .set(F::FilterStrength, rng.below(101) as i32)
            .set(F::FilterSharpness, rng.below(8) as i32)
            .set(F::FilterType, rng.below(2) as i32)
            .set(F::Autofilter, rng.below(2) as i32)
            .set(F::Pass, 1 + rng.below(10) as i32)
            .set(F::EmulateJpegSize, rng.below(2) as i32)
            .set(F::LowMemory, rng.below(2) as i32)
            .set(F::ThreadLevel, rng.below(2) as i32)
            .set(F::UseSharpYuv, rng.below(2) as i32)
            .set(F::Preprocessing, rng.below(8) as i32)
            .set(F::AlphaQuality, rng.below(101) as i32)
            .set(F::AlphaFiltering, rng.below(3) as i32)
            .set(F::AlphaCompression, rng.below(2) as i32)
            .set(F::Exact, rng.below(2) as i32)
            .set(F::NearLossless, rng.pick(&[100, 100, 60, 0]))
            .set(F::UseDeltaPalette, rng.below(2) as i32)
            .set(F::ImageHint, rng.below(4) as i32)
            .set(F::ShowCompressed, rng.below(2) as i32);
        match rng.below(6) {
            0 => {
                c.set(F::TargetSize, 100 + rng.below(4000) as i32);
            }
            1 => {
                c.set(
                    F::TargetPsnr,
                    (25.0 + rng.below(25) as f32).to_bits() as i32,
                );
            }
            _ => {}
        }
        c.extra_info_type = rng.pick(&[0, 0, 1, 2, 3, 4, 5, 7, 8]);
        c.memory_writer = rng.below(4) == 0;
        if rng.below(8) == 0 {
            c.abort_at = rng.below(10) as i32;
        }
        if rng.below(8) == 0 {
            c.fail_write_at = rng.below(6) as i32;
        }
        cases.push(exclude_undefined(c));
    }
    cases
}

/// The test data's PAM and PPM pictures, and some vectors decoded to PAM by the
/// oracle's `dwebp`.
pub fn vector_pictures(oracle: &Oracle) -> Vec<(String, i32, i32, Vec<u32>)> {
    let data = oracle::test_data();
    let mut pictures = Vec::new();
    for name in ["grid.pam", "peak.pam", "grid.ppm", "peak.ppm"] {
        let (w, h, rgba) = read_pnm(&std::fs::read(data.join(name)).unwrap());
        pictures.push((name.to_owned(), w, h, rgba));
    }
    let dwebp = oracle.tool("dwebp");
    let scratch = std::env::temp_dir().join(format!("wp13-vectors-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    for name in [
        "alpha_filter_1.webp",
        "alpha_no_compression.webp",
        "lossless_vec_1_0.webp",
        "lossless_vec_2_7.webp",
        "lossy_alpha2.webp",
        "bug3.webp",
        "vp80-00-comprehensive-001.webp",
        "vp80-05-sharpness-1438.webp",
        "test-nostrong.webp",
    ] {
        let out = scratch.join(format!("{name}.pam"));
        let status = std::process::Command::new(&dwebp)
            .arg(data.join(name))
            .arg("-pam")
            .arg("-o")
            .arg(&out)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(status.success(), "dwebp {name}");
        let (w, h, rgba) = read_pnm(&std::fs::read(&out).unwrap());
        pictures.push((name.to_owned(), w, h, rgba));
    }
    std::fs::remove_dir_all(&scratch).unwrap();
    pictures
}

/// The cases of `vectors_match_the_c_encoder`, from `vector_pictures`.
pub fn vector_cases(oracle: &Oracle) -> Vec<Case> {
    let mut cases = Vec::new();
    for (index, (_, w, h, rgba)) in vector_pictures(oracle).into_iter().enumerate() {
        let base = Case::new(w, h, rgba);
        let settings: [&[(F, i32)]; 6] = [
            &[],
            &[(F::Method, 6), (F::AlphaCompression, 1)],
            &[(F::Method, 0), (F::Segments, 1), (F::AlphaCompression, 1)],
            &[(F::Lossless, 1)],
            &[(F::Lossless, 1), (F::Method, 6), (F::NearLossless, 60)],
            &[
                (F::UseSharpYuv, 1),
                (F::ThreadLevel, 1),
                (F::AlphaCompression, 1),
            ],
        ];
        // The large pictures take the default lossy and lossless settings only.
        let count = if w * h > 100_000 { 4 } else { settings.len() };
        for (k, set) in settings.iter().enumerate().take(count) {
            if count == 4 && (k == 1 || k == 2) {
                continue;
            }
            let mut c = base.clone();
            c.source = if (index + k) % 2 == 0 { 1 } else { 2 };
            for &(field, value) in *set {
                c.set(field, value);
            }
            cases.push(exclude_undefined(c));
        }
    }
    cases
}
