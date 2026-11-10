//! WP10: the merged upsamplers of the x86-64 SIMD extensions
//! (`src/jdmerge_sse2.rs`, `src/jdmrgext_sse2.rs`, `src/jdmerge_avx2.rs`
//! and `src/jdmrgext_avx2.rs`) against upstream's asm kernels, through the
//! probe `tools/oracle/probes/wp10/merge.c` (ADR 0004, ADR 0008).
//!
//! For each instruction set, kernel (h2v1, h2v2) and pixel layout, the test
//! draws the probe's cases from `tests/support/kernels.rs`: output widths 1
//! to 66, 101, 227 and 1000, a random number of row groups and row group,
//! and random luma and chroma rows, padding included, so that the samples
//! reach the saturations. The output rows are separate rows of exactly the
//! output width, as an application's rows may be, so that a store past the
//! width panics, and their hashes must equal the probe's.

#![cfg(all(with_simd, target_arch = "x86_64"))]

#[path = "support/kernels.rs"]
mod kernels;
#[path = "support/oracle.rs"]
mod oracle;

use jpeg::jdmerge_avx2::{
    jsimd_h2v1_extbgr_merged_upsample_avx2, jsimd_h2v1_extbgrx_merged_upsample_avx2,
    jsimd_h2v1_extrgb_merged_upsample_avx2, jsimd_h2v1_extrgbx_merged_upsample_avx2,
    jsimd_h2v1_extxbgr_merged_upsample_avx2, jsimd_h2v1_extxrgb_merged_upsample_avx2,
    jsimd_h2v1_merged_upsample_avx2, jsimd_h2v2_extbgr_merged_upsample_avx2,
    jsimd_h2v2_extbgrx_merged_upsample_avx2, jsimd_h2v2_extrgb_merged_upsample_avx2,
    jsimd_h2v2_extrgbx_merged_upsample_avx2, jsimd_h2v2_extxbgr_merged_upsample_avx2,
    jsimd_h2v2_extxrgb_merged_upsample_avx2, jsimd_h2v2_merged_upsample_avx2,
};
use jpeg::jdmerge_sse2::{
    jsimd_h2v1_extbgr_merged_upsample_sse2, jsimd_h2v1_extbgrx_merged_upsample_sse2,
    jsimd_h2v1_extrgb_merged_upsample_sse2, jsimd_h2v1_extrgbx_merged_upsample_sse2,
    jsimd_h2v1_extxbgr_merged_upsample_sse2, jsimd_h2v1_extxrgb_merged_upsample_sse2,
    jsimd_h2v1_merged_upsample_sse2, jsimd_h2v2_extbgr_merged_upsample_sse2,
    jsimd_h2v2_extbgrx_merged_upsample_sse2, jsimd_h2v2_extrgb_merged_upsample_sse2,
    jsimd_h2v2_extrgbx_merged_upsample_sse2, jsimd_h2v2_extxbgr_merged_upsample_sse2,
    jsimd_h2v2_extxrgb_merged_upsample_sse2, jsimd_h2v2_merged_upsample_sse2,
};
use jpeg::jpegint::merged_upsample_simd_ptr;
use jpeg::jpeglib::{JSAMPROWS_AT, SampleArray};
use kernels::{Rng, fill_rows, hash, rows};
use oracle::Oracle;

/// The seeds of the cases.
const SEEDS: [u64; 3] = [1, 2, 20_261_108];

/// A kernel of the probe: its key, its output rows per row group, its pixel
/// size and the port's kernel.
struct Kernel {
    key: &'static str,
    v_samp: usize,
    pixelsize: usize,
    kernel: merged_upsample_simd_ptr,
}

/// The kernels of one instruction set in the probe's order.
macro_rules! kernels {
    ($($key:literal, $v:literal, $pixelsize:literal, $kernel:ident;)+) => {
        vec![$(Kernel { key: $key, v_samp: $v, pixelsize: $pixelsize, kernel: $kernel }),+]
    };
}

fn sse2_kernels() -> Vec<Kernel> {
    kernels![
        "h2v1", 1, 3, jsimd_h2v1_merged_upsample_sse2;
        "h2v1_extrgb", 1, 3, jsimd_h2v1_extrgb_merged_upsample_sse2;
        "h2v1_extrgbx", 1, 4, jsimd_h2v1_extrgbx_merged_upsample_sse2;
        "h2v1_extbgr", 1, 3, jsimd_h2v1_extbgr_merged_upsample_sse2;
        "h2v1_extbgrx", 1, 4, jsimd_h2v1_extbgrx_merged_upsample_sse2;
        "h2v1_extxbgr", 1, 4, jsimd_h2v1_extxbgr_merged_upsample_sse2;
        "h2v1_extxrgb", 1, 4, jsimd_h2v1_extxrgb_merged_upsample_sse2;
        "h2v2", 2, 3, jsimd_h2v2_merged_upsample_sse2;
        "h2v2_extrgb", 2, 3, jsimd_h2v2_extrgb_merged_upsample_sse2;
        "h2v2_extrgbx", 2, 4, jsimd_h2v2_extrgbx_merged_upsample_sse2;
        "h2v2_extbgr", 2, 3, jsimd_h2v2_extbgr_merged_upsample_sse2;
        "h2v2_extbgrx", 2, 4, jsimd_h2v2_extbgrx_merged_upsample_sse2;
        "h2v2_extxbgr", 2, 4, jsimd_h2v2_extxbgr_merged_upsample_sse2;
        "h2v2_extxrgb", 2, 4, jsimd_h2v2_extxrgb_merged_upsample_sse2;
    ]
}

fn avx2_kernels() -> Vec<Kernel> {
    kernels![
        "h2v1", 1, 3, jsimd_h2v1_merged_upsample_avx2;
        "h2v1_extrgb", 1, 3, jsimd_h2v1_extrgb_merged_upsample_avx2;
        "h2v1_extrgbx", 1, 4, jsimd_h2v1_extrgbx_merged_upsample_avx2;
        "h2v1_extbgr", 1, 3, jsimd_h2v1_extbgr_merged_upsample_avx2;
        "h2v1_extbgrx", 1, 4, jsimd_h2v1_extbgrx_merged_upsample_avx2;
        "h2v1_extxbgr", 1, 4, jsimd_h2v1_extxbgr_merged_upsample_avx2;
        "h2v1_extxrgb", 1, 4, jsimd_h2v1_extxrgb_merged_upsample_avx2;
        "h2v2", 2, 3, jsimd_h2v2_merged_upsample_avx2;
        "h2v2_extrgb", 2, 3, jsimd_h2v2_extrgb_merged_upsample_avx2;
        "h2v2_extrgbx", 2, 4, jsimd_h2v2_extrgbx_merged_upsample_avx2;
        "h2v2_extbgr", 2, 3, jsimd_h2v2_extbgr_merged_upsample_avx2;
        "h2v2_extbgrx", 2, 4, jsimd_h2v2_extbgrx_merged_upsample_avx2;
        "h2v2_extxbgr", 2, 4, jsimd_h2v2_extxbgr_merged_upsample_avx2;
        "h2v2_extxrgb", 2, 4, jsimd_h2v2_extxrgb_merged_upsample_avx2;
    ]
}

/// The probe's output widths: 1 to 66, then wider odd ones.
fn widths() -> impl Iterator<Item = usize> {
    (1..=66).chain([101, 227, 1000])
}

/// The row list of `array` from row 0 on, as the kernels take it.
fn rows_at(array: &SampleArray) -> JSAMPROWS_AT<'_> {
    JSAMPROWS_AT {
        storage: &array.buffer,
        rows: &array.rows,
        origin: 0,
    }
}

/// Runs the probe's cases of every kernel of `kernels` with the generator
/// seeded `seed` and returns the probe's line for each kernel.
fn run(kernels: &[Kernel], seed: u64, call: impl Fn(&Kernel, CallArgs<'_, '_>)) -> Vec<String> {
    let mut rng = Rng::new(seed);
    kernels
        .iter()
        .map(|kernel| {
            let mut hashes = Vec::new();
            for width in widths() {
                let chroma_width = width.div_ceil(2);
                let groups = 1 + rng.rand(3) as usize;
                let ctr = rng.rand(u32::try_from(groups).unwrap());
                let mut luma = rows(groups * kernel.v_samp, width);
                let mut cb = rows(groups, chroma_width);
                let mut cr = rows(groups, chroma_width);
                fill_rows(&mut rng, &mut luma);
                fill_rows(&mut rng, &mut cb);
                fill_rows(&mut rng, &mut cr);
                let len = width * kernel.pixelsize;
                let mut output: Vec<Vec<u8>> = vec![vec![0; len]; kernel.v_samp];
                let mut output_rows: Vec<&mut [u8]> =
                    output.iter_mut().map(Vec::as_mut_slice).collect();
                let image = [rows_at(&luma), rows_at(&cb), rows_at(&cr)];
                call(
                    kernel,
                    CallArgs {
                        output_width: u32::try_from(width).unwrap(),
                        input_buf: &image,
                        in_row_group_ctr: ctr,
                        output_buf: &mut output_rows,
                    },
                );
                hashes.extend(output.iter().map(|row| format!("{:016x}", hash(row))));
            }
            hashes.join(",")
        })
        .collect()
}

/// The arguments of one kernel call.
struct CallArgs<'a, 'r> {
    output_width: u32,
    input_buf: &'a [JSAMPROWS_AT<'a>],
    in_row_group_ctr: u32,
    output_buf: &'a mut [&'r mut [u8]],
}

/// Compares the port's kernels of `isa` with the probe's for every seed.
fn check(test: &str, isa: &str, kernels: &[Kernel], call: impl Fn(&Kernel, CallArgs<'_, '_>)) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    if isa == "avx2" {
        assert!(
            is_x86_feature_detected!("avx2"),
            "the AVX2 kernels need a CPU with AVX2; under Rosetta 2, set ROSETTA_ADVERTISE_AVX=1"
        );
    }
    for seed in SEEDS {
        let args = [
            "-isa".to_owned(),
            isa.to_owned(),
            "-seed".to_owned(),
            seed.to_string(),
        ];
        let fields = oracle
            .probe("wp10", "merge")
            .run(&args, &[])
            .success()
            .fields();
        let lines = run(kernels, seed, &call);
        for (kernel, line) in kernels.iter().zip(&lines) {
            assert_eq!(
                fields.get(kernel.key),
                Some(line.as_str()),
                "{isa} {} seed {seed}",
                kernel.key
            );
        }
    }
}

#[test]
fn sse2_kernels_match_the_asm() {
    check(
        "sse2_kernels_match_the_asm",
        "sse2",
        &sse2_kernels(),
        |kernel, args| {
            // SAFETY: the kernel is an SSE2 kernel, and every x86-64 target
            // enables SSE2.
            unsafe {
                (kernel.kernel)(
                    args.output_width,
                    args.input_buf,
                    args.in_row_group_ctr,
                    args.output_buf,
                );
            }
        },
    );
}

#[test]
fn avx2_kernels_match_the_asm() {
    check(
        "avx2_kernels_match_the_asm",
        "avx2",
        &avx2_kernels(),
        |kernel, args| {
            // SAFETY: the kernel is an AVX2 kernel, and `check` calls it only
            // after it asserted that the CPU has AVX2.
            unsafe {
                (kernel.kernel)(
                    args.output_width,
                    args.input_buf,
                    args.in_row_group_ctr,
                    args.output_buf,
                );
            }
        },
    );
}
