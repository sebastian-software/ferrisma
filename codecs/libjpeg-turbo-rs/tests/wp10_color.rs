//! WP10: the YCbCr-to-RGB kernels of `simd/x86_64` (`jdcolor_sse2`,
//! `jdcolext_sse2`, `jdcolor_avx2`, `jdcolext_avx2`) against the oracle's
//! asm kernels, through the probe `tools/oracle/probes/wp10/color.c`
//! (ADR 0004, ADR 0008).
//!
//! For each instruction set and each of the seven instances, the probe and
//! this test draw the same cases from the generator of
//! `tests/support/kernels.rs`: every width from 1 to 66 and the wider odd
//! ones 101, 227 and 1000, so that each kernel's whole vectors and every
//! branch of its tail run, with random `input_row` and `num_rows` (zero
//! included), and random samples, which reach the saturations of the
//! conversion. The input components are padded rows from `kernels::rows`;
//! the output rows are `RowListMut::Separate` rows of exactly
//! `out_width * RGB_PIXELSIZE` bytes, as an application's rows may be, so
//! that a store past the width panics, and both sides hash exactly those
//! bytes.

#![cfg(all(with_simd, target_arch = "x86_64"))]

#[path = "support/kernels.rs"]
mod kernels;
#[path = "support/oracle.rs"]
mod oracle;

use jpeg::jdcolor_avx2::{
    jsimd_ycc_extbgr_convert_avx2, jsimd_ycc_extbgrx_convert_avx2, jsimd_ycc_extrgb_convert_avx2,
    jsimd_ycc_extrgbx_convert_avx2, jsimd_ycc_extxbgr_convert_avx2, jsimd_ycc_extxrgb_convert_avx2,
    jsimd_ycc_rgb_convert_avx2,
};
use jpeg::jdcolor_sse2::{
    jsimd_ycc_extbgr_convert_sse2, jsimd_ycc_extbgrx_convert_sse2, jsimd_ycc_extrgb_convert_sse2,
    jsimd_ycc_extrgbx_convert_sse2, jsimd_ycc_extxbgr_convert_sse2, jsimd_ycc_extxrgb_convert_sse2,
    jsimd_ycc_rgb_convert_sse2,
};
use jpeg::jpegint::color_convert_simd_ptr;
use jpeg::jpeglib::{JSAMPROWS, RowListMut};
use kernels::{Rng, fill_rows, row_hashes, rows};
use oracle::Oracle;

/// The seeds of the cases; each runs every width twice.
const SEEDS: [u64; 3] = [1, 20_251_108, 0xC010_4C0D];

/// The cases per seed: twice the 69 widths of the probe.
const CASES: usize = 2 * 69;

/// The probe's widths: 1 to 66, 101, 227 and 1000.
fn widths() -> Vec<usize> {
    (1..=66).chain([101, 227, 1000]).collect()
}

/// The instance of each `-format` of the probe and its `RGB_PIXELSIZE`.
fn instances(isa: &str) -> [(color_convert_simd_ptr, usize); 7] {
    match isa {
        "sse2" => [
            (jsimd_ycc_rgb_convert_sse2, 3),
            (jsimd_ycc_extrgb_convert_sse2, 3),
            (jsimd_ycc_extrgbx_convert_sse2, 4),
            (jsimd_ycc_extbgr_convert_sse2, 3),
            (jsimd_ycc_extbgrx_convert_sse2, 4),
            (jsimd_ycc_extxbgr_convert_sse2, 4),
            (jsimd_ycc_extxrgb_convert_sse2, 4),
        ],
        "avx2" => [
            (jsimd_ycc_rgb_convert_avx2, 3),
            (jsimd_ycc_extrgb_convert_avx2, 3),
            (jsimd_ycc_extrgbx_convert_avx2, 4),
            (jsimd_ycc_extbgr_convert_avx2, 3),
            (jsimd_ycc_extbgrx_convert_avx2, 4),
            (jsimd_ycc_extxbgr_convert_avx2, 4),
            (jsimd_ycc_extxrgb_convert_avx2, 4),
        ],
        _ => unreachable!("unknown instruction set {isa}"),
    }
}

/// Runs the probe's cases for `seed` on `kernel` and returns, per case, its
/// parameters and the `case<c>` value: the hashes of the output rows.
#[allow(unsafe_code, reason = "ADR 0008: a call of a #[target_feature] kernel")]
fn port_cases(
    kernel: color_convert_simd_ptr,
    pixelsize: usize,
    seed: u64,
) -> Vec<(String, String)> {
    let widths = widths();
    let mut rng = Rng::new(seed);
    let mut results = Vec::new();
    for case in 0..CASES {
        let width = widths[case % widths.len()];
        let len = width * pixelsize;
        let input_row = rng.rand(3);
        let num_rows = rng.rand(4);
        let numrows = (input_row + num_rows + 1) as usize;
        let components: Vec<_> = (0..3)
            .map(|_| {
                let mut array = rows(numrows, width);
                fill_rows(&mut rng, &mut array);
                array
            })
            .collect();
        let mut output: Vec<Vec<u8>> = (0..num_rows)
            .map(|_| (0..len).map(|_| rng.sample()).collect())
            .collect();

        let input_buf: Vec<JSAMPROWS<'_>> =
            components.iter().map(|array| array.as_rows()).collect();
        let mut output_rows: Vec<&mut [u8]> = output.iter_mut().map(Vec::as_mut_slice).collect();
        // SAFETY: an SSE2 kernel runs on every x86-64 target, and
        // `avx2_color_converters_match_the_asm` asserts that the CPU has
        // AVX2 before it passes an AVX2 kernel.
        unsafe {
            kernel(
                u32::try_from(width).unwrap(),
                &input_buf,
                input_row,
                RowListMut::Separate(&mut output_rows),
                i32::try_from(num_rows).unwrap(),
            );
        }

        results.push((
            format!("width {width}, input_row {input_row}, num_rows {num_rows}"),
            row_hashes(&output, len),
        ));
    }
    results
}

/// Compares every instance of `isa` with the probe, for every seed.
fn check(oracle: &Oracle, isa: &str) {
    for (format, (kernel, pixelsize)) in instances(isa).into_iter().enumerate() {
        for seed in SEEDS {
            let args = [
                "-isa".to_owned(),
                isa.to_owned(),
                "-format".to_owned(),
                format.to_string(),
                "-seed".to_owned(),
                seed.to_string(),
                "-cases".to_owned(),
                CASES.to_string(),
            ];
            let fields = oracle
                .probe("wp10", "color")
                .run(&args, &[])
                .success()
                .fields();
            for (case, (parameters, hashes)) in
                port_cases(kernel, pixelsize, seed).into_iter().enumerate()
            {
                assert_eq!(
                    fields.get(&format!("case{case}")),
                    Some(hashes.as_str()),
                    "{isa} format {format} seed {seed} case {case}: {parameters}"
                );
            }
        }
    }
}

#[test]
fn sse2_color_converters_match_the_asm() {
    let Some(oracle) = Oracle::locate("sse2_color_converters_match_the_asm") else {
        return;
    };
    check(&oracle, "sse2");
}

#[test]
fn avx2_color_converters_match_the_asm() {
    let Some(oracle) = Oracle::locate("avx2_color_converters_match_the_asm") else {
        return;
    };
    assert!(
        is_x86_feature_detected!("avx2"),
        "avx2_color_converters_match_the_asm needs AVX2; under Rosetta 2 set \
         ROSETTA_ADVERTISE_AVX=1"
    );
    check(&oracle, "avx2");
}
