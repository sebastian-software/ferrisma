//! WP11 differential test: the lossless encoder DSP functions
//! (`src/dsp/lossless_enc.c`) and the four inline functions of
//! `src/dsp/lossless_common.h` that read its tables (`VP8LFastLog2`,
//! `VP8LFastSLog2`, `VP8LPrefixEncodeBits`, `VP8LPrefixEncode`). One script
//! drives the C probe `tools/oracle/probes/wp11/dsp.c` and the Rust
//! translation: the four tables; both logarithms on every table value, around
//! the 4,096 and 65,536 thresholds and up to 2^32 - 1 (the `double` path);
//! both prefix encodings for distances 0 to 2,047 and large ones; subtract-green,
//! the color transform and both color-statistics collectors with padded tiles;
//! the vector mismatch; the color-map bundling for every `xbits`; the extra
//! cost; both vector additions; the six entropy functions on sparse, dense,
//! run-structured and large-valued populations of 1 to 1,304 entries; and all
//! 16 entries of `VP8LPredictorsSub` and `VP8LPredictorsSub_C`.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::fmt::Write as _;

use oracle::{Fields, Oracle};
use webp::dsp::lossless::{VP8LBitEntropy, VP8LMultipliers, VP8LStreaks};
use webp::dsp::lossless_common::{
    vp8l_fast_log2, vp8l_fast_s_log2, vp8l_prefix_encode, vp8l_prefix_encode_bits,
    vp8l_sub_sample_size,
};
use webp::dsp::lossless_enc::{
    LOG2_TABLE, PREFIX_ENCODE_CODE, PREFIX_ENCODE_EXTRA_BITS_VALUE, S_LOG2_TABLE, vp8l_add_vector,
    vp8l_add_vector_eq, vp8l_bits_entropy_unrefined, vp8l_bundle_color_map,
    vp8l_collect_color_blue_transforms, vp8l_collect_color_red_transforms,
    vp8l_combined_shannon_entropy, vp8l_extra_cost, vp8l_get_combined_entropy_unrefined,
    vp8l_get_entropy_unrefined, vp8l_predictors_sub, vp8l_predictors_sub_c, vp8l_shannon_entropy,
    vp8l_subtract_green_from_blue_and_red, vp8l_transform_color, vp8l_vector_mismatch,
};
use webp::enc::histogram_enc::vp8l_bits_entropy;
use wp11_images::Rng;

fn hex32(values: &[u32]) -> String {
    if values.is_empty() {
        return "-".to_owned();
    }
    values.iter().map(|value| format!("{value:08x}")).collect()
}

fn hex64(values: &[u64]) -> String {
    if values.is_empty() {
        return "-".to_owned();
    }
    values.iter().map(|value| format!("{value:016x}")).collect()
}

fn entropy_line(e: &VP8LBitEntropy) -> String {
    format!(
        "{:016x},{},{},{},{}",
        e.entropy, e.sum, e.nonzeros, e.max_val, e.nonzero_code
    )
}

fn streaks_line(s: &VP8LStreaks) -> String {
    format!(
        "{},{},{},{},{},{}",
        s.counts[0],
        s.counts[1],
        s.streaks[0][0],
        s.streaks[0][1],
        s.streaks[1][0],
        s.streaks[1][1]
    )
}

/// A population of `size` counts.
fn population(rng: &mut Rng, size: usize) -> Vec<u32> {
    let mut counts = vec![0_u32; size];
    match rng.below(5) {
        0 => {
            for _ in 0..1 + rng.below(6) {
                counts[rng.below(size as u64) as usize] = 1 + rng.below(1000) as u32;
            }
        }
        1 => {
            for count in &mut counts {
                *count = rng.below(1 << 16) as u32;
            }
        }
        2 => {
            let mut at = 0;
            while at < size {
                let run = (1 + rng.below(12) as usize).min(size - at);
                let value = if rng.below(2) == 0 {
                    0
                } else {
                    rng.below(300) as u32
                };
                counts[at..at + run].fill(value);
                at += run;
            }
        }
        3 => {
            for count in &mut counts {
                *count = rng.below(1 << 26) as u32;
            }
        }
        _ => {
            counts[rng.below(size as u64) as usize] = rng.below(1 << 20) as u32;
        }
    }
    counts
}

/// The sizes `VP8LHistogramAdd` passes to `VP8LAddVector` and
/// `VP8LAddVectorEq`: `NUM_DISTANCE_CODES` (40) and `NUM_LITERAL_CODES +
/// NUM_LENGTH_CODES` (280) plus a color cache of 0 or 2 to 2048 entries.
const HISTOGRAM_SIZES: [usize; 13] = [
    40, 280, 282, 284, 288, 296, 312, 344, 408, 536, 792, 1304, 2328,
];

/// The lengths the encoder passes to `VP8LExtraCost`: the 24 length and the
/// 40 distance prefixes.
const EXTRA_COST_LENGTHS: [usize; 2] = [24, 40];

/// The sizes of the `release-simd` comparison (ADR 0008, owner decision O7):
/// every size from `*_min` in steps of `*_step` is in the domain of the
/// variant that upstream's `VP8LEncDspInit` selects at the level of the run,
/// and no other size is.
struct SimdDomain {
    add_min: usize,
    add_step: usize,
    extra_min: usize,
    extra_step: usize,
}

/// The domain of the variants upstream v1.6.0 selects for `LIBWEBP_SIMD_LEVEL`
/// on this host, derived from the level and the host, not from the port's
/// dispatch. The exclusions, each upstream's own:
///
/// - `ExtraCost_C` (`src/dsp/lossless_enc.c:576-584`, selected at `sse2` and
///   on AArch64, where `VP8LEncDspInitNEON` keeps it) asserts an even length (line 579) and reads `population[4]`
///   and `population[5]` for every length (line 578), out of bounds of a
///   population shorter than 6: even lengths from 6.
/// - `ExtraCost_SSE41` (`src/dsp/lossless_enc_sse41.c:35-50`, selected at
///   `sse41` and `avx2`; there is no AVX2 variant) asserts a length that is a
///   multiple of 8 (line 38) and reads `a[4]` to `a[7]` for every length
///   (line 37), out of bounds of a population shorter than 8: multiples of 8
///   from 8.
/// - `AddVector_C` and `AddVectorEq_C` (`src/dsp/lossless_enc.c:588-599`,
///   selected on AArch64, where `VP8LEncDspInitNEON` in
///   `src/dsp/lossless_enc_neon.c:136-142` sets neither) are
///   defined for every size.
/// - `AddVector_SSE2` and `AddVectorEq_SSE2` (`src/dsp/lossless_enc_sse2.c:184`
///   and `:233`, selected at `sse2` and `sse41`) assert a size of at least 16
///   and an even size (lines 192-193 and 240-241); a smaller size reads and
///   writes 16 entries beyond the vectors in the first pass of their
///   `do`-`while` loop: even sizes from 16.
/// - `AddVector_AVX2` and `AddVectorEq_AVX2` (`src/dsp/lossless_enc_avx2.c:181`
///   and `:230`, selected at `avx2`) assert a size of at least 32 and an even
///   size (lines 189-190 and 237-238), with the same first pass of 32
///   entries: even sizes from 32.
///
/// The levels follow `VP8LEncDspInit` (`src/dsp/lossless_enc.c:753-768`):
/// SSE4.1 only with SSE2, AVX2 only with SSE4.1, each where the host reports
/// it and `LIBWEBP_SIMD_LEVEL` lets it (`tools/oracle/probe_support/simd_level.c`).
fn simd_domain() -> SimdDomain {
    let c = SimdDomain {
        add_min: 0,
        add_step: 1,
        extra_min: 6,
        extra_step: 2,
    };
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let level = std::env::var("LIBWEBP_SIMD_LEVEL").unwrap_or_default();
        let sse2 = std::arch::is_x86_feature_detected!("sse2");
        let sse41 = sse2
            && std::arch::is_x86_feature_detected!("sse4.1")
            && matches!(level.as_str(), "" | "avx2" | "sse41");
        let avx2 = sse41
            && std::arch::is_x86_feature_detected!("avx2")
            && matches!(level.as_str(), "" | "avx2");
        let (add_min, add_step) = if avx2 {
            (32, 2)
        } else if sse2 {
            (16, 2)
        } else {
            (c.add_min, c.add_step)
        };
        let (extra_min, extra_step) = if sse41 {
            (8, 8)
        } else {
            (c.extra_min, c.extra_step)
        };
        SimdDomain {
            add_min,
            add_step,
            extra_min,
            extra_step,
        }
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    c
}

/// One `addvec` observation of `size` random entries per vector.
fn add_vectors(rng: &mut Rng, size: usize, script: &mut String, expected: &mut Fields) {
    let a: Vec<u32> = (0..size).map(|_| rng.u32()).collect();
    let mut b: Vec<u32> = (0..size).map(|_| rng.u32()).collect();
    writeln!(script, "addvec {} {}", hex32(&a), hex32(&b)).unwrap();
    let mut sum = vec![0_u32; size];
    vp8l_add_vector(&a, &b, &mut sum, size as i32);
    vp8l_add_vector_eq(&a, &mut b, size as i32);
    *expected = std::mem::take(expected)
        .with("sum", hex32(&sum))
        .with("sum_eq", hex32(&b));
}

#[test]
fn lossless_encoder_dsp_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("lossless_encoder_dsp_matches_the_c_functions") else {
        return;
    };
    let mut rng = Rng(0xd5b0_11e5);
    let mut script = String::new();
    let mut expected = Fields::new();
    // The histogram sums and the extra cost run on every size against the
    // scalar reference; against `release-simd` they run on exactly the sizes
    // that upstream's selected variant is defined for ([`simd_domain`]), after
    // the other functions.
    let simd = oracle.simd();

    // Tables.
    script.push_str("tables\n");
    let codes: Vec<u32> = PREFIX_ENCODE_CODE
        .iter()
        .map(|c| (u32::from(c.code as u8) << 8) | u32::from(c.extra_bits as u8))
        .collect();
    let extra: Vec<u32> = PREFIX_ENCODE_EXTRA_BITS_VALUE
        .iter()
        .map(|&v| u32::from(v))
        .collect();
    expected = expected
        .with("log2", hex32(&LOG2_TABLE))
        .with("slog2", hex64(&S_LOG2_TABLE))
        .with("prefix_code", hex32(&codes))
        .with("prefix_extra", hex32(&extra));

    // Logarithms.
    let mut values: Vec<u32> = (0..600).collect();
    values.extend(4000..4200);
    values.extend(65_000..65_600);
    values.extend([u32::MAX, u32::MAX - 1, 1 << 31, 1 << 28, 16384 * 16384]);
    for _ in 0..2000 {
        values.push(rng.u32() >> rng.below(32));
    }
    for chunk in values.chunks(500) {
        writeln!(script, "logs {}", hex32(chunk)).unwrap();
        let log2: Vec<u32> = chunk.iter().map(|&v| vp8l_fast_log2(v)).collect();
        let slog2: Vec<u64> = chunk.iter().map(|&v| vp8l_fast_s_log2(v)).collect();
        expected = expected
            .with("fast_log2", hex32(&log2))
            .with("fast_slog2", hex64(&slog2));
    }

    // Prefix encodings.
    let mut distances: Vec<u32> = (0..2048).collect();
    for _ in 0..500 {
        distances.push(3 + rng.below((1 << 20) + 120) as u32);
    }
    for chunk in distances.chunks(512) {
        writeln!(script, "prefix {}", hex32(chunk)).unwrap();
        let mut out = Vec::new();
        for &d in chunk {
            let (code, extra_bits) = vp8l_prefix_encode_bits(d as i32);
            out.extend([code as u32, extra_bits as u32]);
            let (code, extra_bits, extra_bits_value) = vp8l_prefix_encode(d as i32);
            out.extend([code as u32, extra_bits as u32, extra_bits_value as u32]);
        }
        expected = expected.with("prefix", hex32(&out));
    }

    // Pixel transforms and statistics.
    for _ in 0..60 {
        let n = rng.below(70) as usize;
        let mut argb: Vec<u32> = (0..n).map(|_| rng.u32()).collect();
        writeln!(script, "subgreen {}", hex32(&argb)).unwrap();
        vp8l_subtract_green_from_blue_and_red(&mut argb, n as i32);
        expected = expected.with("argb", hex32(&argb));

        let m = VP8LMultipliers {
            green_to_red: rng.below(256) as u8,
            green_to_blue: rng.below(256) as u8,
            red_to_blue: rng.below(256) as u8,
        };
        let mut argb: Vec<u32> = (0..n).map(|_| rng.u32()).collect();
        writeln!(
            script,
            "transform {} {} {} {}",
            m.green_to_red,
            m.green_to_blue,
            m.red_to_blue,
            hex32(&argb)
        )
        .unwrap();
        vp8l_transform_color(&m, &mut argb, n as i32);
        expected = expected.with("argb", hex32(&argb));

        let width = 1 + rng.below(20) as i32;
        let height = 1 + rng.below(20) as i32;
        let stride = width + rng.below(4) as i32;
        let argb: Vec<u32> = (0..stride * height).map(|_| rng.u32()).collect();
        let (g2r, g2b, r2b) = (
            rng.below(512) as i32 - 256,
            rng.below(512) as i32 - 256,
            rng.below(512) as i32 - 256,
        );
        writeln!(
            script,
            "collect {stride} {width} {height} {g2r} {g2b} {r2b} {}",
            hex32(&argb)
        )
        .unwrap();
        let mut red = [0_u32; 256];
        let mut blue = [0_u32; 256];
        vp8l_collect_color_red_transforms(&argb, stride, width, height, g2r, &mut red);
        vp8l_collect_color_blue_transforms(&argb, stride, width, height, g2b, r2b, &mut blue);
        expected = expected.with("red", hex32(&red)).with("blue", hex32(&blue));

        let length = rng.below(50) as usize;
        let a: Vec<u32> = (0..length).map(|_| rng.below(3) as u32).collect();
        let mut b = a.clone();
        if length > 0 && rng.below(3) != 0 {
            let at = rng.below(length as u64) as usize;
            b[at] ^= 1 + rng.below(3) as u32;
        }
        writeln!(script, "mismatch {length} {} {}", hex32(&a), hex32(&b)).unwrap();
        expected = expected.with("mismatch", vp8l_vector_mismatch(&a, &b, length as i32));

        let width = 1 + rng.below(40) as i32;
        let xbits = rng.below(4) as i32;
        let row: Vec<u32> = (0..width).map(|_| rng.below(256) as u32).collect();
        writeln!(script, "bundle {width} {xbits} {}", hex32(&row)).unwrap();
        let bytes: Vec<u8> = row.iter().map(|&v| v as u8).collect();
        let dst_size = vp8l_sub_sample_size(width as u32, xbits as u32) as usize;
        let mut dst = vec![0_u32; dst_size];
        vp8l_bundle_color_map(&bytes, width, xbits, &mut dst);
        expected = expected.with("bundle", hex32(&dst));

        if !simd {
            let length = [24, 40, 8, 12, 2 * (3 + rng.below(29) as i32)][rng.below(5) as usize];
            let pop = population(&mut rng, length as usize);
            writeln!(script, "extra {length} {}", hex32(&pop)).unwrap();
            expected = expected.with("extra", vp8l_extra_cost(&pop, length));

            let size = rng.below(300) as usize;
            add_vectors(&mut rng, size, &mut script, &mut expected);
        }
    }

    if simd {
        let domain = simd_domain();
        let mut rng = Rng(0x5eed_a11d);
        let mut lengths: Vec<usize> = (domain.extra_min..=domain.extra_min + 512)
            .step_by(domain.extra_step)
            .collect();
        lengths.extend(EXTRA_COST_LENGTHS);
        for _ in 0..40 {
            lengths.push(
                domain.extra_min
                    + domain.extra_step * rng.below(4096 / domain.extra_step as u64) as usize,
            );
        }
        let lengths_count = lengths.len();
        for length in lengths {
            let pop = population(&mut rng, length);
            writeln!(script, "extra {length} {}", hex32(&pop)).unwrap();
            expected = expected.with("extra", vp8l_extra_cost(&pop, length as i32));
        }
        let mut sizes: Vec<usize> = (domain.add_min..=domain.add_min + 160)
            .step_by(domain.add_step)
            .collect();
        sizes.extend(HISTOGRAM_SIZES);
        for _ in 0..40 {
            sizes.push(
                domain.add_min
                    + domain.add_step * rng.below(4096 / domain.add_step as u64) as usize,
            );
        }
        eprintln!(
            "release-simd domain: {} extra-cost lengths (from {}, step {}), {} vector sizes (from {}, step {})",
            lengths_count,
            domain.extra_min,
            domain.extra_step,
            sizes.len(),
            domain.add_min,
            domain.add_step
        );
        for size in sizes {
            add_vectors(&mut rng, size, &mut script, &mut expected);
        }
    }

    // Entropies.
    for _ in 0..200 {
        let length = [
            1,
            2,
            5,
            40,
            256,
            280,
            280 + 1024,
            1 + rng.below(1304) as i32,
        ][rng.below(8) as usize];
        let x = population(&mut rng, 1304);
        let y = population(&mut rng, 1304);
        writeln!(script, "entropy {length} {} {}", hex32(&x), hex32(&y)).unwrap();
        let mut e = VP8LBitEntropy::default();
        let mut s = VP8LStreaks::default();
        expected = expected
            .with(
                "combined_shannon",
                format!("{:016x}", vp8l_combined_shannon_entropy(&x, &y)),
            )
            .with(
                "shannon",
                format!("{:016x}", vp8l_shannon_entropy(&x, length)),
            );
        vp8l_bits_entropy_unrefined(&x, length, &mut e);
        expected = expected.with("bits_unrefined", entropy_line(&e)).with(
            "bits_entropy",
            format!("{:016x}", vp8l_bits_entropy(&x, length)),
        );
        vp8l_get_entropy_unrefined(&x, length, &mut e, &mut s);
        expected = expected
            .with("entropy_unrefined", entropy_line(&e))
            .with("entropy_streaks", streaks_line(&s));
        vp8l_get_combined_entropy_unrefined(&x, &y, length, &mut e, &mut s);
        expected = expected
            .with("combined_unrefined", entropy_line(&e))
            .with("combined_streaks", streaks_line(&s));
    }

    // Batch predictors.
    for _ in 0..80 {
        let num = 1 + rng.below(40) as usize;
        let few = rng.below(2) == 0;
        let mut pixel = || {
            if few {
                [0xff00_0000, 0x8040_2010, 0x0000_0000, 0xffff_ffff][rng.below(4) as usize]
            } else {
                rng.u32()
            }
        };
        let upper: Vec<u32> = (0..num + 2).map(|_| pixel()).collect();
        let current: Vec<u32> = (0..num + 1).map(|_| pixel()).collect();
        writeln!(
            script,
            "predsub {num} {} {}",
            hex32(&upper),
            hex32(&current)
        )
        .unwrap();
        let mut data = upper.clone();
        data.extend(&current);
        let (upper_pos, in_pos) = (1, num + 3);
        for mode in 0..16 {
            let mut out = vec![0_u32; num];
            vp8l_predictors_sub(mode, &data, in_pos, upper_pos, num as i32, &mut out);
            expected = expected.with("sub", hex32(&out));
            let mut out = vec![0_u32; num];
            vp8l_predictors_sub_c(mode, &data, in_pos, upper_pos, num as i32, &mut out);
            expected = expected.with("sub_c", hex32(&out));
        }
    }

    let output = oracle
        .probe("wp11", "dsp")
        .run::<&str>(&[], script.as_bytes());
    let actual = output.success().fields();
    assert_eq!(actual.0.len(), expected.0.len(), "number of observations");
    for (index, (a, e)) in actual.0.iter().zip(&expected.0).enumerate() {
        assert!(
            a == e,
            "observation {index}: C {}={} Rust {}={}",
            a.0,
            a.1.chars().take(200).collect::<String>(),
            e.0,
            e.1.chars().take(200).collect::<String>()
        );
    }
}
