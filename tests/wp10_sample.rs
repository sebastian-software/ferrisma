//! WP10: the x86-64 upsampling kernels (`jdsample_sse2.rs`,
//! `jdsample_avx2.rs`) against upstream's asm kernels, through the probe
//! `tools/oracle/probes/wp10/sample.c` (ADR 0004, ADR 0008).
//!
//! For each kernel and seed, the probe and this test draw the same inputs
//! (`tests/support/kernels.rs`): every width of 1 to 66, 101, 227, 1000 and
//! random widths up to 1000, each with every `max_v_samp_factor` that the
//! decoder passes to the kernel, input rows of random samples, padding
//! included, and output rows of random samples, so that a write into the
//! padding shows in the hash of the row's whole stride. The fancy h2v2
//! kernels also read the context rows above and below, which in two of
//! three cases repeat the first or last input row, as `jdmainct.c`'s
//! pointer lists do at the image edges.

#![cfg(all(with_simd, target_arch = "x86_64"))]

#[path = "support/kernels.rs"]
mod kernels;
#[path = "support/oracle.rs"]
mod oracle;

use jpeg::jdsample_avx2::{
    jsimd_h2v1_fancy_upsample_avx2, jsimd_h2v1_upsample_avx2, jsimd_h2v2_fancy_upsample_avx2,
    jsimd_h2v2_upsample_avx2,
};
use jpeg::jdsample_sse2::{
    jsimd_h2v1_fancy_upsample_sse2, jsimd_h2v1_upsample_sse2, jsimd_h2v2_fancy_upsample_sse2,
    jsimd_h2v2_upsample_sse2,
};
use jpeg::jpegint::upsample_simd_ptr;
use jpeg::jpeglib::RowListAt;
use kernels::{Rng, array_rows, fill_rows, row_hashes, rows, stride};
use oracle::Oracle;

/// The seeds of each kernel's run.
const SEEDS: [u64; 3] = [1, 2, 20_261_108];

/// `RANDOM_WIDTHS` of the probe.
const RANDOM_WIDTHS: usize = 12;

/// One kernel: the probe's name, the port's function, whether it is a fancy
/// upsampler and whether it is an h2v2 one.
struct Kernel {
    name: &'static str,
    kernel: upsample_simd_ptr,
    fancy: bool,
    v2: bool,
    avx2: bool,
}

/// The probe's `RunCase`: the output rows of one case, as `key` and value.
fn run_case(
    rng: &mut Rng,
    kernel: &Kernel,
    width: u32,
    max_v_samp_factor: i32,
    index: usize,
) -> (String, String) {
    let max_v = usize::try_from(max_v_samp_factor).unwrap();
    let in_rows = if kernel.v2 { max_v / 2 } else { max_v };
    let context = kernel.fancy && kernel.v2;
    let numrows = in_rows + if context { 2 } else { 0 };
    let width_usize = usize::try_from(width).unwrap();
    let in_width = if kernel.fancy {
        width_usize
    } else {
        width_usize.div_ceil(2)
    };
    let out_width = if kernel.fancy {
        2 * width_usize
    } else {
        width_usize
    };
    let mut input = rows(numrows, in_width);
    let mut output = rows(max_v, out_width);
    fill_rows(rng, &mut input);
    fill_rows(rng, &mut output);
    let mut list = input.rows.clone();
    if context {
        if index % 3 == 1 {
            list[0] = list[1];
        } else if index % 3 == 2 {
            list[numrows - 1] = list[numrows - 2];
        }
    }
    let input_data = RowListAt {
        storage: &input.buffer,
        rows: &list,
        origin: usize::from(context),
    };
    if kernel.avx2 {
        assert!(
            std::is_x86_feature_detected!("avx2"),
            "the AVX2 kernels need a CPU with AVX2; under Rosetta 2 set ROSETTA_ADVERTISE_AVX=1"
        );
    }
    // SAFETY: the kernel is an SSE2 kernel, which every x86-64 target runs,
    // or an AVX2 kernel, and the CPU has AVX2 (asserted above).
    unsafe {
        (kernel.kernel)(max_v_samp_factor, width, input_data, output.as_rows_mut());
    }
    (
        format!("out.{width}.{max_v_samp_factor}"),
        row_hashes(&array_rows(&output, stride(out_width)), stride(out_width)),
    )
}

/// Runs `kernel` on the probe's cases for every seed and compares the hashes.
fn check(kernel: &Kernel) {
    let test = format!("wp10_sample::{}", kernel.name);
    let Some(oracle) = Oracle::locate(&test) else {
        return;
    };
    let probe = oracle.probe("wp10", "sample");
    for seed in SEEDS {
        let args = [
            "-kernel".to_owned(),
            kernel.name.to_owned(),
            "-seed".to_owned(),
            seed.to_string(),
        ];
        let fields = probe.run(&args, &[]).success().fields();

        let mut rng = Rng::new(seed);
        let mut widths: Vec<u32> = (1..=66).chain([101, 227, 1000]).collect();
        for _ in 0..RANDOM_WIDTHS {
            widths.push(rng.rand(1000) + 1);
        }
        let factors: &[i32] = if kernel.v2 { &[2, 4] } else { &[1, 2, 3, 4] };
        let mut expected = Vec::new();
        let mut index = 0;
        for &width in &widths {
            for &max_v_samp_factor in factors {
                expected.push(run_case(&mut rng, kernel, width, max_v_samp_factor, index));
                index += 1;
            }
        }

        assert_eq!(
            fields.0.len(),
            expected.len(),
            "{}: seed {seed}: number of cases",
            kernel.name
        );
        for ((key, value), (expected_key, expected_value)) in fields.0.iter().zip(&expected) {
            assert_eq!(key, expected_key, "{}: seed {seed}", kernel.name);
            assert_eq!(
                value, expected_value,
                "{}: seed {seed}: {key} (output row hashes)",
                kernel.name
            );
        }
    }
}

#[test]
fn h2v1_fancy_sse2() {
    check(&Kernel {
        name: "h2v1_fancy_sse2",
        kernel: jsimd_h2v1_fancy_upsample_sse2,
        fancy: true,
        v2: false,
        avx2: false,
    });
}

#[test]
fn h2v2_fancy_sse2() {
    check(&Kernel {
        name: "h2v2_fancy_sse2",
        kernel: jsimd_h2v2_fancy_upsample_sse2,
        fancy: true,
        v2: true,
        avx2: false,
    });
}

#[test]
fn h2v1_sse2() {
    check(&Kernel {
        name: "h2v1_sse2",
        kernel: jsimd_h2v1_upsample_sse2,
        fancy: false,
        v2: false,
        avx2: false,
    });
}

#[test]
fn h2v2_sse2() {
    check(&Kernel {
        name: "h2v2_sse2",
        kernel: jsimd_h2v2_upsample_sse2,
        fancy: false,
        v2: true,
        avx2: false,
    });
}

#[test]
fn h2v1_fancy_avx2() {
    check(&Kernel {
        name: "h2v1_fancy_avx2",
        kernel: jsimd_h2v1_fancy_upsample_avx2,
        fancy: true,
        v2: false,
        avx2: true,
    });
}

#[test]
fn h2v2_fancy_avx2() {
    check(&Kernel {
        name: "h2v2_fancy_avx2",
        kernel: jsimd_h2v2_fancy_upsample_avx2,
        fancy: true,
        v2: true,
        avx2: true,
    });
}

#[test]
fn h2v1_avx2() {
    check(&Kernel {
        name: "h2v1_avx2",
        kernel: jsimd_h2v1_upsample_avx2,
        fancy: false,
        v2: false,
        avx2: true,
    });
}

#[test]
fn h2v2_avx2() {
    check(&Kernel {
        name: "h2v2_avx2",
        kernel: jsimd_h2v2_upsample_avx2,
        fancy: false,
        v2: true,
        avx2: true,
    });
}
