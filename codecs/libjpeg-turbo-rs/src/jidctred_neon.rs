// libjpeg-turbo 3.2.1, simd/arm/jidctred-neon.c:
//
// /*
//  * Scaled Integer Inverse DCT (Arm Neon)
//  *
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2020, 2024-2025, D. R. Commander.
//  *
//  * This software is provided 'as-is', without any express or implied
//  * warranty.  In no event will the authors be held liable for any damages
//  * arising from the use of this software.
//  *
//  * Permission is granted to anyone to use this software for any purpose,
//  * including commercial applications, and to alter it and redistribute it
//  * freely, subject to the following restrictions:
//  *
//  * 1. The origin of this software must not be misrepresented; you must not
//  *    claim that you wrote the original software. If you use this software
//  *    in a product, an acknowledgment in the product documentation would be
//  *    appreciated but is not required.
//  * 2. Altered source versions must be plainly marked as such, and must not be
//  *    misrepresented as being the original software.
//  * 3. This notice may not be removed or altered from any source distribution.
//  */
//! Translation of `simd/arm/jidctred-neon.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! `HAVE_VLD1_S16_X3` is defined, so `jsimd_idct_4x4_neon` loads its
//! constants with `vld1_s16_x3`. `ISLOW_MULT_TYPE` is 16-bit with
//! `WITH_SIMD`, and the multiplier table is the component's `islow_array`.
//!
//! The pointers of the C code become slices: an output row `output_buf[i] +
//! output_col` is the row's slice from `output_col` on, and each load or
//! store converts the first elements of its slice to the array or element
//! that the `neon` wrapper takes. `jsimd_idct_4x4_neon` computes and stores
//! one output row pointer at a time, as `output_buf` lends one row at a
//! time, where C computes the four pointers before the four stores. The
//! stores write the 2 or 4 samples of the block's row and nothing past them.

#![forbid(unsafe_code)]

use core::arch::aarch64::{
    int16x4x3_t, uint16x4x2_t, vaddhn_s32, vaddq_s32, vcombine_s16, vdupq_n_s16, vget_high_s16,
    vget_high_s32, vget_low_s16, vget_low_s32, vgetq_lane_s64, vmlal_lane_s16, vmul_s16,
    vmull_lane_s16, vmulq_s16, vorrq_s16, vqmovun_s16, vreinterpret_s16_s32, vreinterpret_u16_u8,
    vreinterpretq_s16_s32, vreinterpretq_s32_s16, vreinterpretq_s64_s16, vrshrn_n_s32,
    vrsraq_n_s16, vshl_n_s16, vshll_n_s16, vshlq_n_s16, vsubhn_s32, vsubq_s32, vtrnq_s16,
    vtrnq_s32, vzip_u8,
};

use crate::jddctmgr::multiplier_table;
use crate::jmorecfg::{CENTERJSAMPLE, JDIMENSION};
use crate::jpeglib::{DCTSIZE, JBLOCK, JSAMPROWS_MUT};
use crate::neon::{vld1_s16, vld1_s16_x3, vld1q_s16, vst1_lane_u8, vst2_lane_u16};

// upstream: simd/arm/jidctred-neon.c CONST_BITS
const CONST_BITS: i32 = 13;
// upstream: simd/arm/jidctred-neon.c PASS1_BITS
const PASS1_BITS: i32 = 2;

// upstream: simd/arm/jidctred-neon.c F_0_211
const F_0_211: i16 = 1730;
// upstream: simd/arm/jidctred-neon.c F_0_509
const F_0_509: i16 = 4176;
// upstream: simd/arm/jidctred-neon.c F_0_601
const F_0_601: i16 = 4926;
// upstream: simd/arm/jidctred-neon.c F_0_720
const F_0_720: i16 = 5906;
// upstream: simd/arm/jidctred-neon.c F_0_765
const F_0_765: i16 = 6270;
// upstream: simd/arm/jidctred-neon.c F_0_850
const F_0_850: i16 = 6967;
// upstream: simd/arm/jidctred-neon.c F_0_899
const F_0_899: i16 = 7373;
// upstream: simd/arm/jidctred-neon.c F_1_061
const F_1_061: i16 = 8697;
// upstream: simd/arm/jidctred-neon.c F_1_272
const F_1_272: i16 = 10426;
// upstream: simd/arm/jidctred-neon.c F_1_451
const F_1_451: i16 = 11893;
// upstream: simd/arm/jidctred-neon.c F_1_847
const F_1_847: i16 = 15137;
// upstream: simd/arm/jidctred-neon.c F_2_172
const F_2_172: i16 = 17799;
// upstream: simd/arm/jidctred-neon.c F_2_562
const F_2_562: i16 = 20995;
// upstream: simd/arm/jidctred-neon.c F_3_624
const F_3_624: i16 = 29692;

/// The message of a coefficient load that cannot fail. Not an upstream item.
const COEFS: &str = "a block row holds 8 coefficients";
/// The message of a multiplier-table load that cannot fail. Not an upstream
/// item.
const QUANTS: &str = "a table row holds 8 multipliers";
/// The message of an output row that is too short for the 4x4 block. Not an
/// upstream item.
const OUTPUT_4: &str = "an output row holds 4 samples from output_col on";

/* jsimd_idct_2x2_neon() is an inverse DCT function that produces reduced-size
 * 2x2 output from an 8x8 DCT block.  It uses the same calculations and
 * produces exactly the same output as IJG's original jpeg_idct_2x2() function
 * from jpeg-6b, which can be found in jidctred.c.
 *
 * Scaled integer constants are used to avoid floating-point arithmetic:
 *    0.720959822 =  5906 * 2^-13
 *    0.850430095 =  6967 * 2^-13
 *    1.272758580 = 10426 * 2^-13
 *    3.624509785 = 29692 * 2^-13
 *
 * See jidctred.c for further details of the 2x2 IDCT algorithm.  Where
 * possible, the variable names and comments here in jsimd_idct_2x2_neon()
 * match up with those in jpeg_idct_2x2().
 */

// upstream: simd/arm/jidctred-neon.c jsimd_idct_2x2_neon_consts
#[expect(non_upper_case_globals, reason = "C variable name")]
static jsimd_idct_2x2_neon_consts: [i16; 4] = [-F_0_720, F_0_850, -F_1_272, F_3_624];

// upstream: simd/arm/jidctred-neon.c jsimd_idct_2x2_neon
/// Performs dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 2x2 output block.
///
/// # Panics
///
/// When `output_buf` has fewer than 2 rows or 2 columns from `output_col`
/// on.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "C writes k * DCTSIZE row offsets with k = 0 and 1"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversion of CENTERJSAMPLE to int16_t"
)]
#[target_feature(enable = "neon")]
pub fn jsimd_idct_2x2_neon(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let quantptr = dct_table.islow_array();

    /* Load DCT coefficients. */
    let mut row0 = vld1q_s16(coef_block[0 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row1 = vld1q_s16(coef_block[1 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row3 = vld1q_s16(coef_block[3 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row5 = vld1q_s16(coef_block[5 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row7 = vld1q_s16(coef_block[7 * DCTSIZE..].first_chunk().expect(COEFS));

    /* Load quantization table values. */
    let quant_row0 = vld1q_s16(quantptr[0 * DCTSIZE..].first_chunk().expect(QUANTS));
    let quant_row1 = vld1q_s16(quantptr[1 * DCTSIZE..].first_chunk().expect(QUANTS));
    let quant_row3 = vld1q_s16(quantptr[3 * DCTSIZE..].first_chunk().expect(QUANTS));
    let quant_row5 = vld1q_s16(quantptr[5 * DCTSIZE..].first_chunk().expect(QUANTS));
    let quant_row7 = vld1q_s16(quantptr[7 * DCTSIZE..].first_chunk().expect(QUANTS));

    /* Dequantize DCT coefficients. */
    row0 = vmulq_s16(row0, quant_row0);
    row1 = vmulq_s16(row1, quant_row1);
    row3 = vmulq_s16(row3, quant_row3);
    row5 = vmulq_s16(row5, quant_row5);
    row7 = vmulq_s16(row7, quant_row7);

    /* Load IDCT conversion constants. */
    let consts = vld1_s16(&jsimd_idct_2x2_neon_consts);

    /* Pass 1: process columns from input, put results in vectors row0 and
     * row1.
     */

    /* Even part */
    let tmp10_l = vshll_n_s16::<{ CONST_BITS + 2 }>(vget_low_s16(row0));
    let tmp10_h = vshll_n_s16::<{ CONST_BITS + 2 }>(vget_high_s16(row0));

    /* Odd part */
    let mut tmp0_l = vmull_lane_s16::<3>(vget_low_s16(row1), consts);
    tmp0_l = vmlal_lane_s16::<2>(tmp0_l, vget_low_s16(row3), consts);
    tmp0_l = vmlal_lane_s16::<1>(tmp0_l, vget_low_s16(row5), consts);
    tmp0_l = vmlal_lane_s16::<0>(tmp0_l, vget_low_s16(row7), consts);
    let mut tmp0_h = vmull_lane_s16::<3>(vget_high_s16(row1), consts);
    tmp0_h = vmlal_lane_s16::<2>(tmp0_h, vget_high_s16(row3), consts);
    tmp0_h = vmlal_lane_s16::<1>(tmp0_h, vget_high_s16(row5), consts);
    tmp0_h = vmlal_lane_s16::<0>(tmp0_h, vget_high_s16(row7), consts);

    /* Final output stage: descale and narrow to 16-bit. */
    row0 = vcombine_s16(
        vrshrn_n_s32::<CONST_BITS>(vaddq_s32(tmp10_l, tmp0_l)),
        vrshrn_n_s32::<CONST_BITS>(vaddq_s32(tmp10_h, tmp0_h)),
    );
    row1 = vcombine_s16(
        vrshrn_n_s32::<CONST_BITS>(vsubq_s32(tmp10_l, tmp0_l)),
        vrshrn_n_s32::<CONST_BITS>(vsubq_s32(tmp10_h, tmp0_h)),
    );

    /* Transpose two rows, ready for second pass. */
    let cols_0246_1357 = vtrnq_s16(row0, row1);
    let cols_0246 = cols_0246_1357.0;
    let cols_1357 = cols_0246_1357.1;
    /* Duplicate columns such that each is accessible in its own vector. */
    let cols_1155_3377 = vtrnq_s32(
        vreinterpretq_s32_s16(cols_1357),
        vreinterpretq_s32_s16(cols_1357),
    );
    let cols_1155 = vreinterpretq_s16_s32(cols_1155_3377.0);
    let cols_3377 = vreinterpretq_s16_s32(cols_1155_3377.1);

    /* Pass 2: process two rows, store to output array. */

    /* Even part: we're only interested in col0; the top half of tmp10 is "don't
     * care."
     */
    let tmp10 = vshll_n_s16::<{ CONST_BITS + 2 }>(vget_low_s16(cols_0246));

    /* Odd part: we're only interested in the bottom half of tmp0. */
    let mut tmp0 = vmull_lane_s16::<3>(vget_low_s16(cols_1155), consts);
    tmp0 = vmlal_lane_s16::<2>(tmp0, vget_low_s16(cols_3377), consts);
    tmp0 = vmlal_lane_s16::<1>(tmp0, vget_high_s16(cols_1155), consts);
    tmp0 = vmlal_lane_s16::<0>(tmp0, vget_high_s16(cols_3377), consts);

    /* Final output stage: descale and clamp to range [0-255]. */
    let mut output_s16 = vcombine_s16(vaddhn_s32(tmp10, tmp0), vsubhn_s32(tmp10, tmp0));
    output_s16 = vrsraq_n_s16::<{ CONST_BITS + PASS1_BITS + 3 + 2 - 16 }>(
        vdupq_n_s16(CENTERJSAMPLE as i16),
        output_s16,
    );
    /* Narrow to 8-bit and convert to unsigned. */
    let output_u8 = vqmovun_s16(output_s16);

    /* Store 2x2 block to memory. */
    vst1_lane_u8::<0>(&mut output_buf.row_mut(0)[output_col as usize], output_u8);
    vst1_lane_u8::<1>(&mut output_buf.row_mut(1)[output_col as usize], output_u8);
    vst1_lane_u8::<4>(
        &mut output_buf.row_mut(0)[output_col as usize + 1],
        output_u8,
    );
    vst1_lane_u8::<5>(
        &mut output_buf.row_mut(1)[output_col as usize + 1],
        output_u8,
    );
}

/* jsimd_idct_4x4_neon() is an inverse DCT function that produces reduced-size
 * 4x4 output from an 8x8 DCT block.  It uses the same calculations and
 * produces exactly the same output as IJG's original jpeg_idct_4x4() function
 * from jpeg-6b, which can be found in jidctred.c.
 *
 * Scaled integer constants are used to avoid floating-point arithmetic:
 *    0.211164243 =  1730 * 2^-13
 *    0.509795579 =  4176 * 2^-13
 *    0.601344887 =  4926 * 2^-13
 *    0.765366865 =  6270 * 2^-13
 *    0.899976223 =  7373 * 2^-13
 *    1.061594337 =  8697 * 2^-13
 *    1.451774981 = 11893 * 2^-13
 *    1.847759065 = 15137 * 2^-13
 *    2.172734803 = 17799 * 2^-13
 *    2.562915447 = 20995 * 2^-13
 *
 * See jidctred.c for further details of the 4x4 IDCT algorithm.  Where
 * possible, the variable names and comments here in jsimd_idct_4x4_neon()
 * match up with those in jpeg_idct_4x4().
 */

// upstream: simd/arm/jidctred-neon.c jsimd_idct_4x4_neon_consts
#[expect(non_upper_case_globals, reason = "C variable name")]
#[rustfmt::skip]
static jsimd_idct_4x4_neon_consts: [i16; 12] = [
  F_1_847, -F_0_765, -F_0_211,  F_1_451,
 -F_2_172,  F_1_061, -F_0_509, -F_0_601,
  F_0_899,  F_2_562,        0,        0
];

// upstream: simd/arm/jidctred-neon.c jsimd_idct_4x4_neon
/// Performs dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 4x4 output block.
///
/// # Panics
///
/// When `output_buf` has fewer than 4 rows or 4 columns from `output_col`
/// on.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "C writes k * DCTSIZE row offsets with k = 0 and 1"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversion of CENTERJSAMPLE to int16_t"
)]
#[target_feature(enable = "neon")]
pub fn jsimd_idct_4x4_neon(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let quantptr = dct_table.islow_array();

    /* Load DCT coefficients. */
    let mut row0 = vld1q_s16(coef_block[0 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row1 = vld1q_s16(coef_block[1 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row2 = vld1q_s16(coef_block[2 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row3 = vld1q_s16(coef_block[3 * DCTSIZE..].first_chunk().expect(COEFS));
    let row5 = vld1q_s16(coef_block[5 * DCTSIZE..].first_chunk().expect(COEFS));
    let row6 = vld1q_s16(coef_block[6 * DCTSIZE..].first_chunk().expect(COEFS));
    let row7 = vld1q_s16(coef_block[7 * DCTSIZE..].first_chunk().expect(COEFS));

    /* Load quantization table values for DC coefficients. */
    let quant_row0 = vld1q_s16(quantptr[0 * DCTSIZE..].first_chunk().expect(QUANTS));
    /* Dequantize DC coefficients. */
    row0 = vmulq_s16(row0, quant_row0);

    /* Construct bitmap to test if all AC coefficients are 0. */
    let mut bitmap = vorrq_s16(row1, row2);
    bitmap = vorrq_s16(bitmap, row3);
    bitmap = vorrq_s16(bitmap, row5);
    bitmap = vorrq_s16(bitmap, row6);
    bitmap = vorrq_s16(bitmap, row7);

    let left_ac_bitmap: i64 = vgetq_lane_s64::<0>(vreinterpretq_s64_s16(bitmap));
    let right_ac_bitmap: i64 = vgetq_lane_s64::<1>(vreinterpretq_s64_s16(bitmap));

    /* Load constants for IDCT computation. */
    let consts: int16x4x3_t = vld1_s16_x3(&jsimd_idct_4x4_neon_consts);

    if left_ac_bitmap == 0 && right_ac_bitmap == 0 {
        /* All AC coefficients are zero.
         * Compute DC values and duplicate into row vectors 0, 1, 2, and 3.
         */
        let dcval = vshlq_n_s16::<PASS1_BITS>(row0);
        row0 = dcval;
        row1 = dcval;
        row2 = dcval;
        row3 = dcval;
    } else if left_ac_bitmap == 0 {
        /* AC coefficients are zero for columns 0, 1, 2, and 3.
         * Compute DC values for these columns.
         */
        let dcval = vshl_n_s16::<PASS1_BITS>(vget_low_s16(row0));

        /* Commence regular IDCT computation for columns 4, 5, 6, and 7. */

        /* Load quantization table. */
        let quant_row1 = vld1_s16(quantptr[1 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row2 = vld1_s16(quantptr[2 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row3 = vld1_s16(quantptr[3 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row5 = vld1_s16(quantptr[5 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row6 = vld1_s16(quantptr[6 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row7 = vld1_s16(quantptr[7 * DCTSIZE + 4..].first_chunk().expect(QUANTS));

        /* Even part */
        let mut tmp0 = vshll_n_s16::<{ CONST_BITS + 1 }>(vget_high_s16(row0));

        let mut z2 = vmul_s16(vget_high_s16(row2), quant_row2);
        let mut z3 = vmul_s16(vget_high_s16(row6), quant_row6);

        let mut tmp2 = vmull_lane_s16::<0>(z2, consts.0);
        tmp2 = vmlal_lane_s16::<1>(tmp2, z3, consts.0);

        let tmp10 = vaddq_s32(tmp0, tmp2);
        let tmp12 = vsubq_s32(tmp0, tmp2);

        /* Odd part */
        let z1 = vmul_s16(vget_high_s16(row7), quant_row7);
        z2 = vmul_s16(vget_high_s16(row5), quant_row5);
        z3 = vmul_s16(vget_high_s16(row3), quant_row3);
        let z4 = vmul_s16(vget_high_s16(row1), quant_row1);

        tmp0 = vmull_lane_s16::<2>(z1, consts.0);
        tmp0 = vmlal_lane_s16::<3>(tmp0, z2, consts.0);
        tmp0 = vmlal_lane_s16::<0>(tmp0, z3, consts.1);
        tmp0 = vmlal_lane_s16::<1>(tmp0, z4, consts.1);

        tmp2 = vmull_lane_s16::<2>(z1, consts.1);
        tmp2 = vmlal_lane_s16::<3>(tmp2, z2, consts.1);
        tmp2 = vmlal_lane_s16::<0>(tmp2, z3, consts.2);
        tmp2 = vmlal_lane_s16::<1>(tmp2, z4, consts.2);

        /* Final output stage: descale and narrow to 16-bit. */
        row0 = vcombine_s16(
            dcval,
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp10, tmp2)),
        );
        row3 = vcombine_s16(
            dcval,
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp10, tmp2)),
        );
        row1 = vcombine_s16(
            dcval,
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp12, tmp0)),
        );
        row2 = vcombine_s16(
            dcval,
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp12, tmp0)),
        );
    } else if right_ac_bitmap == 0 {
        /* AC coefficients are zero for columns 4, 5, 6, and 7.
         * Compute DC values for these columns.
         */
        let dcval = vshl_n_s16::<PASS1_BITS>(vget_high_s16(row0));

        /* Commence regular IDCT computation for columns 0, 1, 2, and 3. */

        /* Load quantization table. */
        let quant_row1 = vld1_s16(quantptr[1 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row2 = vld1_s16(quantptr[2 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row3 = vld1_s16(quantptr[3 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row5 = vld1_s16(quantptr[5 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row6 = vld1_s16(quantptr[6 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row7 = vld1_s16(quantptr[7 * DCTSIZE..].first_chunk().expect(QUANTS));

        /* Even part */
        let mut tmp0 = vshll_n_s16::<{ CONST_BITS + 1 }>(vget_low_s16(row0));

        let mut z2 = vmul_s16(vget_low_s16(row2), quant_row2);
        let mut z3 = vmul_s16(vget_low_s16(row6), quant_row6);

        let mut tmp2 = vmull_lane_s16::<0>(z2, consts.0);
        tmp2 = vmlal_lane_s16::<1>(tmp2, z3, consts.0);

        let tmp10 = vaddq_s32(tmp0, tmp2);
        let tmp12 = vsubq_s32(tmp0, tmp2);

        /* Odd part */
        let z1 = vmul_s16(vget_low_s16(row7), quant_row7);
        z2 = vmul_s16(vget_low_s16(row5), quant_row5);
        z3 = vmul_s16(vget_low_s16(row3), quant_row3);
        let z4 = vmul_s16(vget_low_s16(row1), quant_row1);

        tmp0 = vmull_lane_s16::<2>(z1, consts.0);
        tmp0 = vmlal_lane_s16::<3>(tmp0, z2, consts.0);
        tmp0 = vmlal_lane_s16::<0>(tmp0, z3, consts.1);
        tmp0 = vmlal_lane_s16::<1>(tmp0, z4, consts.1);

        tmp2 = vmull_lane_s16::<2>(z1, consts.1);
        tmp2 = vmlal_lane_s16::<3>(tmp2, z2, consts.1);
        tmp2 = vmlal_lane_s16::<0>(tmp2, z3, consts.2);
        tmp2 = vmlal_lane_s16::<1>(tmp2, z4, consts.2);

        /* Final output stage: descale and narrow to 16-bit. */
        row0 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp10, tmp2)),
            dcval,
        );
        row3 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp10, tmp2)),
            dcval,
        );
        row1 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp12, tmp0)),
            dcval,
        );
        row2 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp12, tmp0)),
            dcval,
        );
    } else {
        /* All AC coefficients are non-zero; full IDCT calculation required. */
        let quant_row1 = vld1q_s16(quantptr[1 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row2 = vld1q_s16(quantptr[2 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row3 = vld1q_s16(quantptr[3 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row5 = vld1q_s16(quantptr[5 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row6 = vld1q_s16(quantptr[6 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row7 = vld1q_s16(quantptr[7 * DCTSIZE..].first_chunk().expect(QUANTS));

        /* Even part */
        let mut tmp0_l = vshll_n_s16::<{ CONST_BITS + 1 }>(vget_low_s16(row0));
        let mut tmp0_h = vshll_n_s16::<{ CONST_BITS + 1 }>(vget_high_s16(row0));

        let mut z2 = vmulq_s16(row2, quant_row2);
        let mut z3 = vmulq_s16(row6, quant_row6);

        let mut tmp2_l = vmull_lane_s16::<0>(vget_low_s16(z2), consts.0);
        let mut tmp2_h = vmull_lane_s16::<0>(vget_high_s16(z2), consts.0);
        tmp2_l = vmlal_lane_s16::<1>(tmp2_l, vget_low_s16(z3), consts.0);
        tmp2_h = vmlal_lane_s16::<1>(tmp2_h, vget_high_s16(z3), consts.0);

        let tmp10_l = vaddq_s32(tmp0_l, tmp2_l);
        let tmp10_h = vaddq_s32(tmp0_h, tmp2_h);
        let tmp12_l = vsubq_s32(tmp0_l, tmp2_l);
        let tmp12_h = vsubq_s32(tmp0_h, tmp2_h);

        /* Odd part */
        let z1 = vmulq_s16(row7, quant_row7);
        z2 = vmulq_s16(row5, quant_row5);
        z3 = vmulq_s16(row3, quant_row3);
        let z4 = vmulq_s16(row1, quant_row1);

        tmp0_l = vmull_lane_s16::<2>(vget_low_s16(z1), consts.0);
        tmp0_l = vmlal_lane_s16::<3>(tmp0_l, vget_low_s16(z2), consts.0);
        tmp0_l = vmlal_lane_s16::<0>(tmp0_l, vget_low_s16(z3), consts.1);
        tmp0_l = vmlal_lane_s16::<1>(tmp0_l, vget_low_s16(z4), consts.1);
        tmp0_h = vmull_lane_s16::<2>(vget_high_s16(z1), consts.0);
        tmp0_h = vmlal_lane_s16::<3>(tmp0_h, vget_high_s16(z2), consts.0);
        tmp0_h = vmlal_lane_s16::<0>(tmp0_h, vget_high_s16(z3), consts.1);
        tmp0_h = vmlal_lane_s16::<1>(tmp0_h, vget_high_s16(z4), consts.1);

        tmp2_l = vmull_lane_s16::<2>(vget_low_s16(z1), consts.1);
        tmp2_l = vmlal_lane_s16::<3>(tmp2_l, vget_low_s16(z2), consts.1);
        tmp2_l = vmlal_lane_s16::<0>(tmp2_l, vget_low_s16(z3), consts.2);
        tmp2_l = vmlal_lane_s16::<1>(tmp2_l, vget_low_s16(z4), consts.2);
        tmp2_h = vmull_lane_s16::<2>(vget_high_s16(z1), consts.1);
        tmp2_h = vmlal_lane_s16::<3>(tmp2_h, vget_high_s16(z2), consts.1);
        tmp2_h = vmlal_lane_s16::<0>(tmp2_h, vget_high_s16(z3), consts.2);
        tmp2_h = vmlal_lane_s16::<1>(tmp2_h, vget_high_s16(z4), consts.2);

        /* Final output stage: descale and narrow to 16-bit. */
        row0 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp10_l, tmp2_l)),
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp10_h, tmp2_h)),
        );
        row3 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp10_l, tmp2_l)),
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp10_h, tmp2_h)),
        );
        row1 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp12_l, tmp0_l)),
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vaddq_s32(tmp12_h, tmp0_h)),
        );
        row2 = vcombine_s16(
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp12_l, tmp0_l)),
            vrshrn_n_s32::<{ CONST_BITS - PASS1_BITS + 1 }>(vsubq_s32(tmp12_h, tmp0_h)),
        );
    }

    /* Transpose 8x4 block to perform IDCT on rows in second pass. */
    let row_01 = vtrnq_s16(row0, row1);
    let row_23 = vtrnq_s16(row2, row3);

    let cols_0426 = vtrnq_s32(
        vreinterpretq_s32_s16(row_01.0),
        vreinterpretq_s32_s16(row_23.0),
    );
    let cols_1537 = vtrnq_s32(
        vreinterpretq_s32_s16(row_01.1),
        vreinterpretq_s32_s16(row_23.1),
    );

    let col0 = vreinterpret_s16_s32(vget_low_s32(cols_0426.0));
    let col1 = vreinterpret_s16_s32(vget_low_s32(cols_1537.0));
    let col2 = vreinterpret_s16_s32(vget_low_s32(cols_0426.1));
    let col3 = vreinterpret_s16_s32(vget_low_s32(cols_1537.1));
    let col5 = vreinterpret_s16_s32(vget_high_s32(cols_1537.0));
    let col6 = vreinterpret_s16_s32(vget_high_s32(cols_0426.1));
    let col7 = vreinterpret_s16_s32(vget_high_s32(cols_1537.1));

    /* Commence second pass of IDCT. */

    /* Even part */
    let mut tmp0 = vshll_n_s16::<{ CONST_BITS + 1 }>(col0);
    let mut tmp2 = vmull_lane_s16::<0>(col2, consts.0);
    tmp2 = vmlal_lane_s16::<1>(tmp2, col6, consts.0);

    let tmp10 = vaddq_s32(tmp0, tmp2);
    let tmp12 = vsubq_s32(tmp0, tmp2);

    /* Odd part */
    tmp0 = vmull_lane_s16::<2>(col7, consts.0);
    tmp0 = vmlal_lane_s16::<3>(tmp0, col5, consts.0);
    tmp0 = vmlal_lane_s16::<0>(tmp0, col3, consts.1);
    tmp0 = vmlal_lane_s16::<1>(tmp0, col1, consts.1);

    tmp2 = vmull_lane_s16::<2>(col7, consts.1);
    tmp2 = vmlal_lane_s16::<3>(tmp2, col5, consts.1);
    tmp2 = vmlal_lane_s16::<0>(tmp2, col3, consts.2);
    tmp2 = vmlal_lane_s16::<1>(tmp2, col1, consts.2);

    /* Final output stage: descale and clamp to range [0-255]. */
    let mut output_cols_02 = vcombine_s16(vaddhn_s32(tmp10, tmp2), vsubhn_s32(tmp12, tmp0));
    let mut output_cols_13 = vcombine_s16(vaddhn_s32(tmp12, tmp0), vsubhn_s32(tmp10, tmp2));
    output_cols_02 = vrsraq_n_s16::<{ CONST_BITS + PASS1_BITS + 3 + 1 - 16 }>(
        vdupq_n_s16(CENTERJSAMPLE as i16),
        output_cols_02,
    );
    output_cols_13 = vrsraq_n_s16::<{ CONST_BITS + PASS1_BITS + 3 + 1 - 16 }>(
        vdupq_n_s16(CENTERJSAMPLE as i16),
        output_cols_13,
    );
    /* Narrow to 8-bit and convert to unsigned while zipping 8-bit elements.
     * An interleaving store completes the transpose.
     */
    let output_0123 = vzip_u8(vqmovun_s16(output_cols_02), vqmovun_s16(output_cols_13));
    let output_01_23 = uint16x4x2_t(
        vreinterpret_u16_u8(output_0123.0),
        vreinterpret_u16_u8(output_0123.1),
    );

    /* Store 4x4 block to memory. */
    let outptr0 = &mut output_buf.row_mut(0)[output_col as usize..];
    vst2_lane_u16::<0>(outptr0.first_chunk_mut().expect(OUTPUT_4), output_01_23);
    let outptr1 = &mut output_buf.row_mut(1)[output_col as usize..];
    vst2_lane_u16::<1>(outptr1.first_chunk_mut().expect(OUTPUT_4), output_01_23);
    let outptr2 = &mut output_buf.row_mut(2)[output_col as usize..];
    vst2_lane_u16::<2>(outptr2.first_chunk_mut().expect(OUTPUT_4), output_01_23);
    let outptr3 = &mut output_buf.row_mut(3)[output_col as usize..];
    vst2_lane_u16::<3>(outptr3.first_chunk_mut().expect(OUTPUT_4), output_01_23);
}
