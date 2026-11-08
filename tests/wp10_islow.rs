//! WP10: the accurate integer IDCT kernels `jsimd_idct_islow_sse2` and
//! `jsimd_idct_islow_avx2` (`src/jidctint_sse2.rs`, `src/jidctint_avx2.rs`)
//! against the oracle's asm kernels, through the probe
//! `tools/oracle/probes/wp10/islow.c` (ADR 0004, ADR 0008).
//!
//! Each case draws the probe's inputs in the probe's order
//! (`tests/support/kernels.rs` mirrors its generator): a quantization table
//! of values up to 1, 255, 32767 or 65535, turned into the 16-bit multiplier
//! table as `jddctmgr.c`'s `start_pass` does with `WITH_SIMD`; a block from
//! image-like to arbitrary `JCOEF` values, where the asm's 16-bit products
//! and sums wrap and its packs saturate, including the zero-column cases;
//! and 16 padded output rows of 8 to 96 samples, of which the kernel writes
//! 8 rows from row 0 or 8 at a column that the decoder uses or any other.
//! The hash of the whole output array after each case must equal the
//! probe's.

#![cfg(all(with_simd, target_arch = "x86_64"))]

#[path = "support/kernels.rs"]
mod kernels;
#[path = "support/oracle.rs"]
mod oracle;

use jpeg::jddctmgr::multiplier_table;
use jpeg::jidctint_avx2::jsimd_idct_islow_avx2;
use jpeg::jidctint_sse2::jsimd_idct_islow_sse2;
use jpeg::jmorecfg::JCOEF;
use jpeg::jpeglib::{DCTSIZE, DCTSIZE2, JBLOCK, RowListMut};
use kernels::{Rng, fill_rows, hash, rows};
use oracle::Oracle;

/// The probe's `NUM_ROWS`: the rows of each output array.
const NUM_ROWS: usize = 16;
/// The cases of each probe run.
const CASES: usize = 2000;
/// The seeds of the probe runs.
const SEEDS: [u64; 4] = [1, 2, 3, 20_261_108];

/// The probe's `Coef`: an arbitrary 16-bit value or an edge case.
fn coef(rng: &mut Rng) -> JCOEF {
    const EDGES: [JCOEF; 8] = [-32768, 32767, -1, 0, 1, 256, -256, 0x7F00];
    if rng.rand(2) != 0 {
        (rng.next() & 0xFFFF) as u16 as JCOEF
    } else {
        EDGES[rng.rand(8) as usize]
    }
}

/// `Wp10Rand(n)` for a C `int` argument.
fn rand(rng: &mut Rng, n: i32) -> i32 {
    rng.rand(n as u32) as i32
}

/// The probe's `MakeBlock`.
fn make_block(rng: &mut Rng, quantval: &[u32; DCTSIZE2]) -> JBLOCK {
    let mut block = [0; DCTSIZE2];
    let q = |k: usize| quantval[k] as i32;
    match rng.rand(8) {
        0 => {
            for (k, value) in block.iter_mut().enumerate() {
                let lim = 1024 / (1 + (k / DCTSIZE + k % DCTSIZE) as i32);
                if k == 0 || rng.rand(4) == 0 {
                    *value = ((rand(rng, 2 * lim + 1) - lim) / q(k)) as JCOEF;
                }
            }
        }
        1 => {
            block[0] = ((rand(rng, 2049) - 1024) / q(0)) as JCOEF;
            if rng.rand(2) != 0 {
                let k = 1 + rng.rand(DCTSIZE as u32 - 1) as usize;
                block[k] = (rand(rng, 201) - 100) as JCOEF;
            }
        }
        2 => {
            let k = rng.rand(DCTSIZE2 as u32) as usize;
            block[k] = ((rand(rng, 4097) - 2048) / q(k)) as JCOEF;
        }
        3 => {
            for value in &mut block {
                *value = (rand(rng, 257) - 128) as JCOEF;
            }
        }
        4 => {
            for value in &mut block {
                *value = coef(rng);
            }
        }
        5 => {
            for value in &mut block[..DCTSIZE] {
                *value = coef(rng);
            }
        }
        6 => {
            for value in &mut block[..DCTSIZE] {
                *value = coef(rng);
            }
            let k = DCTSIZE + rng.rand((DCTSIZE2 - DCTSIZE) as u32) as usize;
            block[k] = coef(rng);
        }
        _ => {}
    }
    block
}

/// The kernel signature, the `idct_simd_ptr` slot type.
type Kernel = unsafe fn(&multiplier_table, &JBLOCK, RowListMut<'_, '_, u8>, u32);

/// Runs the probe's cases through `kernel` and returns its `out=` value.
fn run_cases(seed: u64, kernel: Kernel) -> String {
    const QMAXES: [u32; 4] = [1, 255, 32767, 65535];
    let mut rng = Rng::new(seed);
    let mut table = multiplier_table::zeroed();
    let mut hashes = Vec::with_capacity(CASES);
    for _ in 0..CASES {
        let qmax = QMAXES[rng.rand(4) as usize];
        let mut quantval = [0; DCTSIZE2];
        for (quant, multiplier) in quantval.iter_mut().zip(table.islow_array_mut()) {
            *quant = 1 + rng.rand(qmax);
            // jddctmgr.c: (ISLOW_MULT_TYPE)qtbl->quantval[i], 16-bit.
            *multiplier = *quant as u16 as i16;
        }
        let block = make_block(&mut rng, &quantval);
        let width = DCTSIZE * (1 + rng.rand(12) as usize);
        let mut array = rows(NUM_ROWS, width);
        fill_rows(&mut rng, &mut array);
        let first_row = DCTSIZE * rng.rand(2) as usize;
        let output_col = if rng.rand(2) != 0 {
            DCTSIZE as u32 * rng.rand((width / DCTSIZE) as u32)
        } else {
            rng.rand((width - DCTSIZE + 1) as u32)
        };
        let mut output = array.as_rows_mut();
        // SAFETY: the caller checked that the CPU runs the kernel's
        // instruction set.
        unsafe { kernel(&table, &block, output.offset(first_row), output_col) };
        hashes.push(format!("{:016x}", hash(&array.buffer)));
    }
    hashes.join(",")
}

/// Compares the port's `kernel` with the probe's `-isa isa` over every seed.
fn check(test: &str, isa: &str, kernel: Kernel) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    for seed in SEEDS {
        let args = [
            "-isa".to_owned(),
            isa.to_owned(),
            "-seed".to_owned(),
            seed.to_string(),
            "-cases".to_owned(),
            CASES.to_string(),
        ];
        let fields = oracle
            .probe("wp10", "islow")
            .run(&args, &[])
            .success()
            .fields();
        let expected = fields.get("out").expect("the probe prints out=");
        let actual = run_cases(seed, kernel);
        if actual != expected {
            let case = actual
                .split(',')
                .zip(expected.split(','))
                .position(|(a, e)| a != e);
            panic!("{isa}, seed {seed}: the first differing case is {case:?}");
        }
    }
}

#[test]
fn sse2_kernel_matches_the_asm() {
    // SAFETY (of the calls in `run_cases`): every x86-64 target enables SSE2.
    check("sse2_kernel_matches_the_asm", "sse2", jsimd_idct_islow_sse2);
}

#[test]
fn avx2_kernel_matches_the_asm() {
    assert!(
        is_x86_feature_detected!("avx2"),
        "the CPU does not report AVX2; under Rosetta 2, set ROSETTA_ADVERTISE_AVX=1"
    );
    // SAFETY (of the calls in `run_cases`): AVX2 is available, as checked
    // above.
    check("avx2_kernel_matches_the_asm", "avx2", jsimd_idct_islow_avx2);
}
