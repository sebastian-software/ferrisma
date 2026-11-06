// Supplementary transcription of upstream/aom/test/intrapred_test.cc
// (HighbdIntraPredTest.Bitexact, NEON instantiation
// HighbdIntraPredTestVectorNeon). Seed, iteration count, buffer layout, edge
// fill (saturated first, then Rand16() & mask), stride, bit depth (12), and
// the exact-match check follow the original. DISABLED_Speed is not
// transcribed.
#![cfg(target_arch = "aarch64")]
use aom::simd::aom_dsp__arm__highbd_intrapred_neon::*;

use super::acm_random::ACMRandom;

type HighbdIntraPred = unsafe extern "C-unwind" fn(*mut u16, isize, *const u16, *const u16, i32);

const COUNT_TEST_BLOCK: usize = 100000;

struct IntraPredFunc {
    pred_fn: HighbdIntraPred,
    ref_fn: HighbdIntraPred,
    block_width: usize,
    block_height: usize,
    bit_depth: i32,
    name: &'static str,
}

#[repr(C, align(16))]
struct Aligned16<T>(T);

/// AV1IntraPredTest<HighbdIntraPred, uint16_t>::RunTest for one parameter.
/// Returns the number of blocks compared.
fn run_test(params: &IntraPredFunc) -> usize {
    // max block size is 64
    let mut left_col = Box::new(Aligned16([0u16; 2 * 64]));
    let mut above_data = Box::new(Aligned16([0u16; 2 * 64 + 64]));
    let mut dst = Box::new(Aligned16([0u16; 3 * 64 * 64]));
    let mut ref_dst = Box::new(Aligned16([0u16; 3 * 64 * 64]));

    let stride = params.block_width * 3;
    let mask = ((1i32 << params.bit_depth) - 1) as u16;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let block_width = params.block_width;
    let block_height = params.block_height;
    let above_row = 16usize; // above_data + 16
    for i in 0..COUNT_TEST_BLOCK {
        // Fill edges with random data, try first with saturated values.
        for x in -1..=(block_width as isize * 2) {
            let idx = (above_row as isize + x) as usize;
            above_data.0[idx] = if i == 0 { mask } else { rnd.rand16() & mask };
        }
        for y in 0..block_height {
            left_col.0[y] = if i == 0 { mask } else { rnd.rand16() & mask };
        }
        // SAFETY: the buffers hold the largest (64x64) block at stride 3 * 64
        // plus the edge extents that the original test provides.
        unsafe {
            (params.ref_fn)(ref_dst.0.as_mut_ptr(), stride as isize, above_data.0.as_ptr().add(above_row),
                            left_col.0.as_ptr(), params.bit_depth);
            (params.pred_fn)(dst.0.as_mut_ptr(), stride as isize, above_data.0.as_ptr().add(above_row),
                             left_col.0.as_ptr(), params.bit_depth);
        }
        for y in 0..block_height {
            for x in 0..block_width {
                assert_eq!(ref_dst.0[x + y * stride], dst.0[x + y * stride],
                           "{} {}x{} bd {}: Failed on Test Case Number {} location: x = {} y = {}",
                           params.name, block_width, block_height, params.bit_depth, i, x, y);
            }
        }
    }
    COUNT_TEST_BLOCK
}

macro_rules! highbd_entry {
    ($neon:ident, $c:ident, $w:literal, $h:literal, $bd:literal) => {
        IntraPredFunc {
            pred_fn: $neon,
            ref_fn: $c,
            block_width: $w,
            block_height: $h,
            bit_depth: $bd,
            name: stringify!($neon),
        }
    };
}

fn run_all(funcs: &[IntraPredFunc]) {
    let mut blocks = 0;
    for f in funcs {
        blocks += run_test(f);
    }
    eprintln!("{} parameter sets, {} blocks compared", funcs.len(), blocks);
}

extern "C-unwind" {
    fn aom_highbd_dc_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_top_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_left_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_dc_128_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_v_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_h_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_paeth_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_v_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_4x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_4x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_4x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_8x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_8x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_8x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_8x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_16x4_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_16x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_16x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_16x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_16x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_32x8_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_32x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_32x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_32x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_64x16_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_64x32_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
    fn aom_highbd_smooth_h_predictor_64x64_c(dst: *mut u16, stride: isize, above: *const u16, left: *const u16, bd: i32);
}

#[test]
fn highbd_intra_pred_dc_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_dc_predictor_4x4_neon, aom_highbd_dc_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_dc_predictor_4x8_neon, aom_highbd_dc_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_dc_predictor_4x16_neon, aom_highbd_dc_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_dc_predictor_8x4_neon, aom_highbd_dc_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_dc_predictor_8x8_neon, aom_highbd_dc_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_dc_predictor_8x16_neon, aom_highbd_dc_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_dc_predictor_8x32_neon, aom_highbd_dc_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_dc_predictor_16x4_neon, aom_highbd_dc_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_dc_predictor_16x8_neon, aom_highbd_dc_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_dc_predictor_16x16_neon, aom_highbd_dc_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_dc_predictor_16x32_neon, aom_highbd_dc_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_dc_predictor_16x64_neon, aom_highbd_dc_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_dc_predictor_32x8_neon, aom_highbd_dc_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_dc_predictor_32x16_neon, aom_highbd_dc_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_dc_predictor_32x32_neon, aom_highbd_dc_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_dc_predictor_32x64_neon, aom_highbd_dc_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_dc_predictor_64x16_neon, aom_highbd_dc_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_dc_predictor_64x32_neon, aom_highbd_dc_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_dc_predictor_64x64_neon, aom_highbd_dc_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_dc_top_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_dc_top_predictor_4x4_neon, aom_highbd_dc_top_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_4x8_neon, aom_highbd_dc_top_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_4x16_neon, aom_highbd_dc_top_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_8x4_neon, aom_highbd_dc_top_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_8x8_neon, aom_highbd_dc_top_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_8x16_neon, aom_highbd_dc_top_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_8x32_neon, aom_highbd_dc_top_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_16x4_neon, aom_highbd_dc_top_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_16x8_neon, aom_highbd_dc_top_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_16x16_neon, aom_highbd_dc_top_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_16x32_neon, aom_highbd_dc_top_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_16x64_neon, aom_highbd_dc_top_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_32x8_neon, aom_highbd_dc_top_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_32x16_neon, aom_highbd_dc_top_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_32x32_neon, aom_highbd_dc_top_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_32x64_neon, aom_highbd_dc_top_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_64x16_neon, aom_highbd_dc_top_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_64x32_neon, aom_highbd_dc_top_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_dc_top_predictor_64x64_neon, aom_highbd_dc_top_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_dc_left_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_dc_left_predictor_4x4_neon, aom_highbd_dc_left_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_4x8_neon, aom_highbd_dc_left_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_4x16_neon, aom_highbd_dc_left_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_8x4_neon, aom_highbd_dc_left_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_8x8_neon, aom_highbd_dc_left_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_8x16_neon, aom_highbd_dc_left_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_8x32_neon, aom_highbd_dc_left_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_16x4_neon, aom_highbd_dc_left_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_16x8_neon, aom_highbd_dc_left_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_16x16_neon, aom_highbd_dc_left_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_16x32_neon, aom_highbd_dc_left_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_16x64_neon, aom_highbd_dc_left_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_32x8_neon, aom_highbd_dc_left_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_32x16_neon, aom_highbd_dc_left_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_32x32_neon, aom_highbd_dc_left_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_32x64_neon, aom_highbd_dc_left_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_64x16_neon, aom_highbd_dc_left_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_64x32_neon, aom_highbd_dc_left_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_dc_left_predictor_64x64_neon, aom_highbd_dc_left_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_dc_128_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_dc_128_predictor_4x4_neon, aom_highbd_dc_128_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_4x8_neon, aom_highbd_dc_128_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_4x16_neon, aom_highbd_dc_128_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_8x4_neon, aom_highbd_dc_128_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_8x8_neon, aom_highbd_dc_128_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_8x16_neon, aom_highbd_dc_128_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_8x32_neon, aom_highbd_dc_128_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_16x4_neon, aom_highbd_dc_128_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_16x8_neon, aom_highbd_dc_128_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_16x16_neon, aom_highbd_dc_128_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_16x32_neon, aom_highbd_dc_128_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_16x64_neon, aom_highbd_dc_128_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_32x8_neon, aom_highbd_dc_128_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_32x16_neon, aom_highbd_dc_128_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_32x32_neon, aom_highbd_dc_128_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_32x64_neon, aom_highbd_dc_128_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_64x16_neon, aom_highbd_dc_128_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_64x32_neon, aom_highbd_dc_128_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_dc_128_predictor_64x64_neon, aom_highbd_dc_128_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_v_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_v_predictor_4x4_neon, aom_highbd_v_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_v_predictor_4x8_neon, aom_highbd_v_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_v_predictor_4x16_neon, aom_highbd_v_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_v_predictor_8x4_neon, aom_highbd_v_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_v_predictor_8x8_neon, aom_highbd_v_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_v_predictor_8x16_neon, aom_highbd_v_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_v_predictor_8x32_neon, aom_highbd_v_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_v_predictor_16x4_neon, aom_highbd_v_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_v_predictor_16x8_neon, aom_highbd_v_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_v_predictor_16x16_neon, aom_highbd_v_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_v_predictor_16x32_neon, aom_highbd_v_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_v_predictor_16x64_neon, aom_highbd_v_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_v_predictor_32x8_neon, aom_highbd_v_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_v_predictor_32x16_neon, aom_highbd_v_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_v_predictor_32x32_neon, aom_highbd_v_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_v_predictor_32x64_neon, aom_highbd_v_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_v_predictor_64x16_neon, aom_highbd_v_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_v_predictor_64x32_neon, aom_highbd_v_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_v_predictor_64x64_neon, aom_highbd_v_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_h_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_h_predictor_4x4_neon, aom_highbd_h_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_h_predictor_4x8_neon, aom_highbd_h_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_h_predictor_4x16_neon, aom_highbd_h_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_h_predictor_8x4_neon, aom_highbd_h_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_h_predictor_8x8_neon, aom_highbd_h_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_h_predictor_8x16_neon, aom_highbd_h_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_h_predictor_8x32_neon, aom_highbd_h_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_h_predictor_16x4_neon, aom_highbd_h_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_h_predictor_16x8_neon, aom_highbd_h_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_h_predictor_16x16_neon, aom_highbd_h_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_h_predictor_16x32_neon, aom_highbd_h_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_h_predictor_16x64_neon, aom_highbd_h_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_h_predictor_32x8_neon, aom_highbd_h_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_h_predictor_32x16_neon, aom_highbd_h_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_h_predictor_32x32_neon, aom_highbd_h_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_h_predictor_32x64_neon, aom_highbd_h_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_h_predictor_64x16_neon, aom_highbd_h_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_h_predictor_64x32_neon, aom_highbd_h_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_h_predictor_64x64_neon, aom_highbd_h_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_paeth_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_paeth_predictor_4x4_neon, aom_highbd_paeth_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_paeth_predictor_4x8_neon, aom_highbd_paeth_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_paeth_predictor_4x16_neon, aom_highbd_paeth_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_paeth_predictor_8x4_neon, aom_highbd_paeth_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_paeth_predictor_8x8_neon, aom_highbd_paeth_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_paeth_predictor_8x16_neon, aom_highbd_paeth_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_paeth_predictor_8x32_neon, aom_highbd_paeth_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_paeth_predictor_16x4_neon, aom_highbd_paeth_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_paeth_predictor_16x8_neon, aom_highbd_paeth_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_paeth_predictor_16x16_neon, aom_highbd_paeth_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_paeth_predictor_16x32_neon, aom_highbd_paeth_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_paeth_predictor_16x64_neon, aom_highbd_paeth_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_paeth_predictor_32x8_neon, aom_highbd_paeth_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_paeth_predictor_32x16_neon, aom_highbd_paeth_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_paeth_predictor_32x32_neon, aom_highbd_paeth_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_paeth_predictor_32x64_neon, aom_highbd_paeth_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_paeth_predictor_64x16_neon, aom_highbd_paeth_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_paeth_predictor_64x32_neon, aom_highbd_paeth_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_paeth_predictor_64x64_neon, aom_highbd_paeth_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_smooth_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_smooth_predictor_4x4_neon, aom_highbd_smooth_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_smooth_predictor_4x8_neon, aom_highbd_smooth_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_smooth_predictor_4x16_neon, aom_highbd_smooth_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_smooth_predictor_8x4_neon, aom_highbd_smooth_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_smooth_predictor_8x8_neon, aom_highbd_smooth_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_smooth_predictor_8x16_neon, aom_highbd_smooth_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_smooth_predictor_8x32_neon, aom_highbd_smooth_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_smooth_predictor_16x4_neon, aom_highbd_smooth_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_smooth_predictor_16x8_neon, aom_highbd_smooth_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_smooth_predictor_16x16_neon, aom_highbd_smooth_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_smooth_predictor_16x32_neon, aom_highbd_smooth_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_smooth_predictor_16x64_neon, aom_highbd_smooth_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_smooth_predictor_32x8_neon, aom_highbd_smooth_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_smooth_predictor_32x16_neon, aom_highbd_smooth_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_smooth_predictor_32x32_neon, aom_highbd_smooth_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_smooth_predictor_32x64_neon, aom_highbd_smooth_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_smooth_predictor_64x16_neon, aom_highbd_smooth_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_smooth_predictor_64x32_neon, aom_highbd_smooth_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_smooth_predictor_64x64_neon, aom_highbd_smooth_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_smooth_v_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_smooth_v_predictor_4x4_neon, aom_highbd_smooth_v_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_4x8_neon, aom_highbd_smooth_v_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_4x16_neon, aom_highbd_smooth_v_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_8x4_neon, aom_highbd_smooth_v_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_8x8_neon, aom_highbd_smooth_v_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_8x16_neon, aom_highbd_smooth_v_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_8x32_neon, aom_highbd_smooth_v_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_16x4_neon, aom_highbd_smooth_v_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_16x8_neon, aom_highbd_smooth_v_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_16x16_neon, aom_highbd_smooth_v_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_16x32_neon, aom_highbd_smooth_v_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_16x64_neon, aom_highbd_smooth_v_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_32x8_neon, aom_highbd_smooth_v_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_32x16_neon, aom_highbd_smooth_v_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_32x32_neon, aom_highbd_smooth_v_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_32x64_neon, aom_highbd_smooth_v_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_64x16_neon, aom_highbd_smooth_v_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_64x32_neon, aom_highbd_smooth_v_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_smooth_v_predictor_64x64_neon, aom_highbd_smooth_v_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}

#[test]
fn highbd_intra_pred_smooth_h_neon_bitexact() {
    let funcs = [
        highbd_entry!(aom_highbd_smooth_h_predictor_4x4_neon, aom_highbd_smooth_h_predictor_4x4_c, 4, 4, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_4x8_neon, aom_highbd_smooth_h_predictor_4x8_c, 4, 8, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_4x16_neon, aom_highbd_smooth_h_predictor_4x16_c, 4, 16, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_8x4_neon, aom_highbd_smooth_h_predictor_8x4_c, 8, 4, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_8x8_neon, aom_highbd_smooth_h_predictor_8x8_c, 8, 8, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_8x16_neon, aom_highbd_smooth_h_predictor_8x16_c, 8, 16, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_8x32_neon, aom_highbd_smooth_h_predictor_8x32_c, 8, 32, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_16x4_neon, aom_highbd_smooth_h_predictor_16x4_c, 16, 4, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_16x8_neon, aom_highbd_smooth_h_predictor_16x8_c, 16, 8, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_16x16_neon, aom_highbd_smooth_h_predictor_16x16_c, 16, 16, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_16x32_neon, aom_highbd_smooth_h_predictor_16x32_c, 16, 32, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_16x64_neon, aom_highbd_smooth_h_predictor_16x64_c, 16, 64, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_32x8_neon, aom_highbd_smooth_h_predictor_32x8_c, 32, 8, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_32x16_neon, aom_highbd_smooth_h_predictor_32x16_c, 32, 16, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_32x32_neon, aom_highbd_smooth_h_predictor_32x32_c, 32, 32, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_32x64_neon, aom_highbd_smooth_h_predictor_32x64_c, 32, 64, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_64x16_neon, aom_highbd_smooth_h_predictor_64x16_c, 64, 16, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_64x32_neon, aom_highbd_smooth_h_predictor_64x32_c, 64, 32, 12),
        highbd_entry!(aom_highbd_smooth_h_predictor_64x64_neon, aom_highbd_smooth_h_predictor_64x64_c, 64, 64, 12),
    ];
    run_all(&funcs);
}
