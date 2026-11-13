// libjpeg-turbo 3.2.1, simd/arm/jidctint-neon.c:
//
// /*
//  * Accurate Integer Inverse DCT (Arm Neon)
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
//! Translation of `simd/arm/jidctint-neon.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! `HAVE_VLD1_S16_X3` is defined, so the helpers load their constants with
//! `vld1_s16_x3`. `ISLOW_MULT_TYPE` is 16-bit with `WITH_SIMD`, and the
//! multiplier table is the component's `islow_array`. The forward
//! declarations of the helpers are not needed in Rust and are left out.
//!
//! The pointers of the C code become slices: a workspace pointer the slice
//! of the workspace from that element on, an output row `output_buf[i] +
//! output_col` the row's slice from `output_col` on, and each load or store
//! converts the first elements of its slice to the array that the `neon`
//! wrapper takes. The second pass computes and stores one output row
//! pointer at a time, as `output_buf` lends one row at a time, where C
//! computes the four pointers before the four stores. Each store writes the
//! 8 samples of the block's row and nothing past them.

#![forbid(unsafe_code)]

use core::arch::aarch64::{
    int16x4_t, int16x4x3_t, int16x4x4_t, uint16x4x4_t, vadd_s16, vadd_u8, vaddhn_s32, vaddq_s32,
    vcombine_s16, vdup_n_u8, vget_lane_s64, vmlal_lane_s16, vmlsl_lane_s16, vmul_s16,
    vmull_lane_s16, vorr_s16, vqrshrn_n_s16, vreinterpret_s64_s16, vreinterpret_u8_s8,
    vreinterpret_u16_u8, vrshrn_n_s32, vshl_n_s16, vshll_n_s16, vsub_s16, vsubhn_s32, vsubq_s32,
    vzip_u8,
};

use crate::jddctmgr::multiplier_table;
use crate::jmorecfg::{CENTERJSAMPLE, JDIMENSION};
use crate::jpeglib::{DCTSIZE, JBLOCK, JSAMPROWS_MUT};
use crate::neon::{vld1_s16, vld1_s16_x3, vst4_lane_u16, vst4_s16};

// upstream: simd/arm/jidctint-neon.c CONST_BITS
const CONST_BITS: i32 = 13;
// upstream: simd/arm/jidctint-neon.c PASS1_BITS
const PASS1_BITS: i32 = 2;

// upstream: simd/arm/jidctint-neon.c DESCALE_P1
const DESCALE_P1: i32 = CONST_BITS - PASS1_BITS;
// upstream: simd/arm/jidctint-neon.c DESCALE_P2
const DESCALE_P2: i32 = CONST_BITS + PASS1_BITS + 3;

/* The computation of the inverse DCT requires the use of constants known at
 * compile time.  Scaled integer constants are used to avoid floating-point
 * arithmetic:
 *    0.298631336 =  2446 * 2^-13
 *    0.390180644 =  3196 * 2^-13
 *    0.541196100 =  4433 * 2^-13
 *    0.765366865 =  6270 * 2^-13
 *    0.899976223 =  7373 * 2^-13
 *    1.175875602 =  9633 * 2^-13
 *    1.501321110 = 12299 * 2^-13
 *    1.847759065 = 15137 * 2^-13
 *    1.961570560 = 16069 * 2^-13
 *    2.053119869 = 16819 * 2^-13
 *    2.562915447 = 20995 * 2^-13
 *    3.072711026 = 25172 * 2^-13
 */

// upstream: simd/arm/jidctint-neon.c F_0_298
const F_0_298: i16 = 2446;
// upstream: simd/arm/jidctint-neon.c F_0_390
const F_0_390: i16 = 3196;
// upstream: simd/arm/jidctint-neon.c F_0_541
const F_0_541: i16 = 4433;
// upstream: simd/arm/jidctint-neon.c F_0_765
const F_0_765: i16 = 6270;
// upstream: simd/arm/jidctint-neon.c F_0_899
const F_0_899: i16 = 7373;
// upstream: simd/arm/jidctint-neon.c F_1_175
const F_1_175: i16 = 9633;
// upstream: simd/arm/jidctint-neon.c F_1_501
const F_1_501: i16 = 12299;
// upstream: simd/arm/jidctint-neon.c F_1_847
const F_1_847: i16 = 15137;
// upstream: simd/arm/jidctint-neon.c F_1_961
const F_1_961: i16 = 16069;
// upstream: simd/arm/jidctint-neon.c F_2_053
const F_2_053: i16 = 16819;
// upstream: simd/arm/jidctint-neon.c F_2_562
const F_2_562: i16 = 20995;
// upstream: simd/arm/jidctint-neon.c F_3_072
const F_3_072: i16 = 25172;

// upstream: simd/arm/jidctint-neon.c F_1_175_MINUS_1_961
const F_1_175_MINUS_1_961: i16 = F_1_175 - F_1_961;
// upstream: simd/arm/jidctint-neon.c F_1_175_MINUS_0_390
const F_1_175_MINUS_0_390: i16 = F_1_175 - F_0_390;
// upstream: simd/arm/jidctint-neon.c F_0_541_MINUS_1_847
const F_0_541_MINUS_1_847: i16 = F_0_541 - F_1_847;
// upstream: simd/arm/jidctint-neon.c F_3_072_MINUS_2_562
const F_3_072_MINUS_2_562: i16 = F_3_072 - F_2_562;
// upstream: simd/arm/jidctint-neon.c F_0_298_MINUS_0_899
const F_0_298_MINUS_0_899: i16 = F_0_298 - F_0_899;
// upstream: simd/arm/jidctint-neon.c F_1_501_MINUS_0_899
const F_1_501_MINUS_0_899: i16 = F_1_501 - F_0_899;
// upstream: simd/arm/jidctint-neon.c F_2_053_MINUS_2_562
const F_2_053_MINUS_2_562: i16 = F_2_053 - F_2_562;
// upstream: simd/arm/jidctint-neon.c F_0_541_PLUS_0_765
const F_0_541_PLUS_0_765: i16 = F_0_541 + F_0_765;

// upstream: simd/arm/jidctint-neon.c jsimd_idct_islow_neon_consts
#[expect(non_upper_case_globals, reason = "C variable name")]
#[rustfmt::skip]
static jsimd_idct_islow_neon_consts: [i16; 16] = [
  F_0_899,             F_0_541,
  F_2_562,             F_0_298_MINUS_0_899,
  F_1_501_MINUS_0_899, F_2_053_MINUS_2_562,
  F_0_541_PLUS_0_765,  F_1_175,
  F_1_175_MINUS_0_390, F_0_541_MINUS_1_847,
  F_3_072_MINUS_2_562, F_1_175_MINUS_1_961,
  0, 0, 0, 0
];

/// The message of a coefficient load that cannot fail: a block row holds
/// `DCTSIZE` coefficients. Not an upstream item.
const COEFS: &str = "a block row holds 8 coefficients";
/// The message of a multiplier-table load that cannot fail. Not an upstream
/// item.
const QUANTS: &str = "a table row holds 8 multipliers";
/// The message of a workspace access that cannot fail. Not an upstream item.
const WORKSPACE: &str = "a workspace holds 8 rows of 4 elements";
/// The message of an output row that is too short. Not an upstream item.
const OUTPUT: &str = "an output row holds 8 samples from output_col on";

/// The first 12 constants of `jsimd_idct_islow_neon_consts`, which
/// `vld1_s16_x3` reads from its start. Not an upstream item.
fn islow_consts_x3() -> &'static [i16; 12] {
    jsimd_idct_islow_neon_consts
        .first_chunk()
        .expect("the table has 16 constants")
}

/* Perform dequantization and inverse DCT on one block of coefficients.  For
 * reference, the C implementation (jpeg_idct_slow()) can be found in
 * jidctint.c.
 *
 * Optimization techniques used for fast data access:
 *
 * In each pass, the inverse DCT is computed for the left and right 4x8 halves
 * of the DCT block.  This avoids spilling due to register pressure, and the
 * increased granularity allows for an optimized calculation depending on the
 * values of the DCT coefficients.  Between passes, intermediate data is stored
 * in 4x8 workspace buffers.
 *
 * Transposing the 8x8 DCT block after each pass can be achieved by transposing
 * each of the four 4x4 quadrants and swapping quadrants 1 and 2 (refer to the
 * diagram below.)  Swapping quadrants is cheap, since the second pass can just
 * swap the workspace buffer pointers.
 *
 *      +-------+-------+                   +-------+-------+
 *      |       |       |                   |       |       |
 *      |   0   |   1   |                   |   0   |   2   |
 *      |       |       |    transpose      |       |       |
 *      +-------+-------+     ------>       +-------+-------+
 *      |       |       |                   |       |       |
 *      |   2   |   3   |                   |   1   |   3   |
 *      |       |       |                   |       |       |
 *      +-------+-------+                   +-------+-------+
 *
 * Optimization techniques used to accelerate the inverse DCT calculation:
 *
 * In a DCT coefficient block, the coefficients are increasingly likely to be 0
 * as you move diagonally from top left to bottom right.  If whole rows of
 * coefficients are 0, then the inverse DCT calculation can be simplified.  On
 * the first pass of the inverse DCT, we test for three special cases before
 * defaulting to a full "regular" inverse DCT:
 *
 * 1) Coefficients in rows 4-7 are all zero.  In this case, we perform a
 *    "sparse" simplified inverse DCT on rows 0-3.
 * 2) AC coefficients (rows 1-7) are all zero.  In this case, the inverse DCT
 *    result is equal to the dequantized DC coefficients.
 * 3) AC and DC coefficients are all zero.  In this case, the inverse DCT
 *    result is all zero.  For the left 4x8 half, this is handled identically
 *    to Case 2 above.  For the right 4x8 half, we do no work and signal that
 *    the "sparse" algorithm is required for the second pass.
 *
 * In the second pass, only a single special case is tested: whether the AC and
 * DC coefficients were all zero in the right 4x8 block during the first pass
 * (refer to Case 3 above.)  If this is the case, then a "sparse" variant of
 * the second pass is performed for both the left and right halves of the DCT
 * block.  (The transposition after the first pass means that the right 4x8
 * block during the first pass becomes rows 4-7 during the second pass.)
 */

// upstream: simd/arm/jidctint-neon.c jsimd_idct_islow_neon
/// Performs dequantization and the accurate inverse DCT on one block of
/// coefficients.
///
/// # Panics
///
/// When `output_buf` has fewer than 8 rows or 8 columns from `output_col`
/// on.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "C writes k * DCTSIZE row offsets with k = 0 and 1"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the pass-2 helpers' conversion of CENTERJSAMPLE to uint8_t"
)]
#[target_feature(enable = "neon")]
pub fn jsimd_idct_islow_neon(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let quantptr = dct_table.islow_array();

    let mut workspace_l = [0i16; 8 * DCTSIZE / 2];
    let mut workspace_r = [0i16; 8 * DCTSIZE / 2];

    /* Compute IDCT first pass on left 4x8 coefficient block. */

    /* Load DCT coefficients in left 4x8 block. */
    let mut row0 = vld1_s16(coef_block[0 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row1 = vld1_s16(coef_block[1 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row2 = vld1_s16(coef_block[2 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row3 = vld1_s16(coef_block[3 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row4 = vld1_s16(coef_block[4 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row5 = vld1_s16(coef_block[5 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row6 = vld1_s16(coef_block[6 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row7 = vld1_s16(coef_block[7 * DCTSIZE..].first_chunk().expect(COEFS));

    /* Load quantization table for left 4x8 block. */
    let mut quant_row0 = vld1_s16(quantptr[0 * DCTSIZE..].first_chunk().expect(QUANTS));
    let mut quant_row1 = vld1_s16(quantptr[1 * DCTSIZE..].first_chunk().expect(QUANTS));
    let mut quant_row2 = vld1_s16(quantptr[2 * DCTSIZE..].first_chunk().expect(QUANTS));
    let mut quant_row3 = vld1_s16(quantptr[3 * DCTSIZE..].first_chunk().expect(QUANTS));
    let mut quant_row4 = vld1_s16(quantptr[4 * DCTSIZE..].first_chunk().expect(QUANTS));
    let mut quant_row5 = vld1_s16(quantptr[5 * DCTSIZE..].first_chunk().expect(QUANTS));
    let mut quant_row6 = vld1_s16(quantptr[6 * DCTSIZE..].first_chunk().expect(QUANTS));
    let mut quant_row7 = vld1_s16(quantptr[7 * DCTSIZE..].first_chunk().expect(QUANTS));

    /* Construct bitmap to test if DCT coefficients in left 4x8 block are 0. */
    let mut bitmap = vorr_s16(row7, row6);
    bitmap = vorr_s16(bitmap, row5);
    bitmap = vorr_s16(bitmap, row4);
    let mut bitmap_rows_4567: i64 = vget_lane_s64::<0>(vreinterpret_s64_s16(bitmap));

    if bitmap_rows_4567 == 0 {
        bitmap = vorr_s16(bitmap, row3);
        bitmap = vorr_s16(bitmap, row2);
        bitmap = vorr_s16(bitmap, row1);
        let left_ac_bitmap: i64 = vget_lane_s64::<0>(vreinterpret_s64_s16(bitmap));

        if left_ac_bitmap == 0 {
            let dcval = vshl_n_s16::<PASS1_BITS>(vmul_s16(row0, quant_row0));
            let quadrant = int16x4x4_t(dcval, dcval, dcval, dcval);
            /* Store 4x4 blocks to workspace, transposing in the process. */
            vst4_s16(workspace_l.first_chunk_mut().expect(WORKSPACE), quadrant);
            vst4_s16(workspace_r.first_chunk_mut().expect(WORKSPACE), quadrant);
        } else {
            jsimd_idct_islow_pass1_sparse(
                row0,
                row1,
                row2,
                row3,
                quant_row0,
                quant_row1,
                quant_row2,
                quant_row3,
                &mut workspace_l,
                &mut workspace_r,
            );
        }
    } else {
        jsimd_idct_islow_pass1_regular(
            row0,
            row1,
            row2,
            row3,
            row4,
            row5,
            row6,
            row7,
            quant_row0,
            quant_row1,
            quant_row2,
            quant_row3,
            quant_row4,
            quant_row5,
            quant_row6,
            quant_row7,
            &mut workspace_l,
            &mut workspace_r,
        );
    }

    /* Compute IDCT first pass on right 4x8 coefficient block. */

    /* Load DCT coefficients in right 4x8 block. */
    row0 = vld1_s16(coef_block[0 * DCTSIZE + 4..].first_chunk().expect(COEFS));
    row1 = vld1_s16(coef_block[1 * DCTSIZE + 4..].first_chunk().expect(COEFS));
    row2 = vld1_s16(coef_block[2 * DCTSIZE + 4..].first_chunk().expect(COEFS));
    row3 = vld1_s16(coef_block[3 * DCTSIZE + 4..].first_chunk().expect(COEFS));
    row4 = vld1_s16(coef_block[4 * DCTSIZE + 4..].first_chunk().expect(COEFS));
    row5 = vld1_s16(coef_block[5 * DCTSIZE + 4..].first_chunk().expect(COEFS));
    row6 = vld1_s16(coef_block[6 * DCTSIZE + 4..].first_chunk().expect(COEFS));
    row7 = vld1_s16(coef_block[7 * DCTSIZE + 4..].first_chunk().expect(COEFS));

    /* Load quantization table for right 4x8 block. */
    quant_row0 = vld1_s16(quantptr[0 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
    quant_row1 = vld1_s16(quantptr[1 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
    quant_row2 = vld1_s16(quantptr[2 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
    quant_row3 = vld1_s16(quantptr[3 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
    quant_row4 = vld1_s16(quantptr[4 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
    quant_row5 = vld1_s16(quantptr[5 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
    quant_row6 = vld1_s16(quantptr[6 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
    quant_row7 = vld1_s16(quantptr[7 * DCTSIZE + 4..].first_chunk().expect(QUANTS));

    /* Construct bitmap to test if DCT coefficients in right 4x8 block are 0. */
    bitmap = vorr_s16(row7, row6);
    bitmap = vorr_s16(bitmap, row5);
    bitmap = vorr_s16(bitmap, row4);
    bitmap_rows_4567 = vget_lane_s64::<0>(vreinterpret_s64_s16(bitmap));
    bitmap = vorr_s16(bitmap, row3);
    bitmap = vorr_s16(bitmap, row2);
    bitmap = vorr_s16(bitmap, row1);
    let right_ac_bitmap: i64 = vget_lane_s64::<0>(vreinterpret_s64_s16(bitmap));

    /* If this remains non-zero, a "regular" second pass will be performed. */
    let mut right_ac_dc_bitmap: i64 = 1;

    if right_ac_bitmap == 0 {
        bitmap = vorr_s16(bitmap, row0);
        right_ac_dc_bitmap = vget_lane_s64::<0>(vreinterpret_s64_s16(bitmap));

        if right_ac_dc_bitmap != 0 {
            let dcval = vshl_n_s16::<PASS1_BITS>(vmul_s16(row0, quant_row0));
            let quadrant = int16x4x4_t(dcval, dcval, dcval, dcval);
            /* Store 4x4 blocks to workspace, transposing in the process. */
            vst4_s16(
                workspace_l[4 * DCTSIZE / 2..]
                    .first_chunk_mut()
                    .expect(WORKSPACE),
                quadrant,
            );
            vst4_s16(
                workspace_r[4 * DCTSIZE / 2..]
                    .first_chunk_mut()
                    .expect(WORKSPACE),
                quadrant,
            );
        }
    } else if bitmap_rows_4567 == 0 {
        jsimd_idct_islow_pass1_sparse(
            row0,
            row1,
            row2,
            row3,
            quant_row0,
            quant_row1,
            quant_row2,
            quant_row3,
            &mut workspace_l[4 * DCTSIZE / 2..],
            &mut workspace_r[4 * DCTSIZE / 2..],
        );
    } else {
        jsimd_idct_islow_pass1_regular(
            row0,
            row1,
            row2,
            row3,
            row4,
            row5,
            row6,
            row7,
            quant_row0,
            quant_row1,
            quant_row2,
            quant_row3,
            quant_row4,
            quant_row5,
            quant_row6,
            quant_row7,
            &mut workspace_l[4 * DCTSIZE / 2..],
            &mut workspace_r[4 * DCTSIZE / 2..],
        );
    }

    /* Second pass: compute IDCT on rows in workspace. */

    /* If all coefficients in right 4x8 block are 0, use "sparse" second pass. */
    if right_ac_dc_bitmap == 0 {
        jsimd_idct_islow_pass2_sparse!(&workspace_l, output_buf.reborrow(), output_col, 0);
        jsimd_idct_islow_pass2_sparse!(&workspace_r, output_buf.reborrow(), output_col, 4);
    } else {
        jsimd_idct_islow_pass2_regular!(&workspace_l, output_buf.reborrow(), output_col, 0);
        jsimd_idct_islow_pass2_regular!(&workspace_r, output_buf.reborrow(), output_col, 4);
    }
}

/* Perform dequantization and the first pass of the accurate inverse DCT on a
 * 4x8 block of coefficients.  (To process the full 8x8 DCT block, this
 * function-- or some other optimized variant-- needs to be called for both the
 * left and right 4x8 blocks.)
 *
 * This "regular" version assumes that no optimization can be made to the IDCT
 * calculation, since no useful set of AC coefficients is all 0.
 *
 * The original C implementation of the accurate IDCT (jpeg_idct_slow()) can be
 * found in jidctint.c.  Algorithmic changes made here are documented inline.
 */

// upstream: simd/arm/jidctint-neon.c jsimd_idct_islow_pass1_regular
#[expect(clippy::too_many_arguments, reason = "the C function's parameters")]
#[expect(clippy::similar_names, reason = "C variable names")]
#[target_feature(enable = "neon")]
#[inline]
fn jsimd_idct_islow_pass1_regular(
    row0: int16x4_t,
    row1: int16x4_t,
    row2: int16x4_t,
    row3: int16x4_t,
    row4: int16x4_t,
    row5: int16x4_t,
    row6: int16x4_t,
    row7: int16x4_t,
    quant_row0: int16x4_t,
    quant_row1: int16x4_t,
    quant_row2: int16x4_t,
    quant_row3: int16x4_t,
    quant_row4: int16x4_t,
    quant_row5: int16x4_t,
    quant_row6: int16x4_t,
    quant_row7: int16x4_t,
    workspace_1: &mut [i16],
    workspace_2: &mut [i16],
) {
    /* Load constants for IDCT computation. */
    let consts: int16x4x3_t = vld1_s16_x3(islow_consts_x3());

    /* Even part */
    let mut z2_s16 = vmul_s16(row2, quant_row2);
    let mut z3_s16 = vmul_s16(row6, quant_row6);

    let mut tmp2 = vmull_lane_s16::<1>(z2_s16, consts.0);
    let mut tmp3 = vmull_lane_s16::<2>(z2_s16, consts.1);
    tmp2 = vmlal_lane_s16::<1>(tmp2, z3_s16, consts.2);
    tmp3 = vmlal_lane_s16::<1>(tmp3, z3_s16, consts.0);

    z2_s16 = vmul_s16(row0, quant_row0);
    z3_s16 = vmul_s16(row4, quant_row4);

    let mut tmp0 = vshll_n_s16::<CONST_BITS>(vadd_s16(z2_s16, z3_s16));
    let mut tmp1 = vshll_n_s16::<CONST_BITS>(vsub_s16(z2_s16, z3_s16));

    let tmp10 = vaddq_s32(tmp0, tmp3);
    let tmp13 = vsubq_s32(tmp0, tmp3);
    let tmp11 = vaddq_s32(tmp1, tmp2);
    let tmp12 = vsubq_s32(tmp1, tmp2);

    /* Odd part */
    let tmp0_s16 = vmul_s16(row7, quant_row7);
    let tmp1_s16 = vmul_s16(row5, quant_row5);
    let tmp2_s16 = vmul_s16(row3, quant_row3);
    let tmp3_s16 = vmul_s16(row1, quant_row1);

    z3_s16 = vadd_s16(tmp0_s16, tmp2_s16);
    let z4_s16 = vadd_s16(tmp1_s16, tmp3_s16);

    /* Implementation as per jpeg_idct_islow() in jidctint.c:
     *   z5 = (z3 + z4) * 1.175875602;
     *   z3 = z3 * -1.961570560;  z4 = z4 * -0.390180644;
     *   z3 += z5;  z4 += z5;
     *
     * This implementation:
     *   z3 = z3 * (1.175875602 - 1.961570560) + z4 * 1.175875602;
     *   z4 = z3 * 1.175875602 + z4 * (1.175875602 - 0.390180644);
     */

    let mut z3 = vmull_lane_s16::<3>(z3_s16, consts.2);
    let mut z4 = vmull_lane_s16::<3>(z3_s16, consts.1);
    z3 = vmlal_lane_s16::<3>(z3, z4_s16, consts.1);
    z4 = vmlal_lane_s16::<0>(z4, z4_s16, consts.2);

    /* Implementation as per jpeg_idct_islow() in jidctint.c:
     *   z1 = tmp0 + tmp3;  z2 = tmp1 + tmp2;
     *   tmp0 = tmp0 * 0.298631336;  tmp1 = tmp1 * 2.053119869;
     *   tmp2 = tmp2 * 3.072711026;  tmp3 = tmp3 * 1.501321110;
     *   z1 = z1 * -0.899976223;  z2 = z2 * -2.562915447;
     *   tmp0 += z1 + z3;  tmp1 += z2 + z4;
     *   tmp2 += z2 + z3;  tmp3 += z1 + z4;
     *
     * This implementation:
     *   tmp0 = tmp0 * (0.298631336 - 0.899976223) + tmp3 * -0.899976223;
     *   tmp1 = tmp1 * (2.053119869 - 2.562915447) + tmp2 * -2.562915447;
     *   tmp2 = tmp1 * -2.562915447 + tmp2 * (3.072711026 - 2.562915447);
     *   tmp3 = tmp0 * -0.899976223 + tmp3 * (1.501321110 - 0.899976223);
     *   tmp0 += z3;  tmp1 += z4;
     *   tmp2 += z3;  tmp3 += z4;
     */

    tmp0 = vmull_lane_s16::<3>(tmp0_s16, consts.0);
    tmp1 = vmull_lane_s16::<1>(tmp1_s16, consts.1);
    tmp2 = vmull_lane_s16::<2>(tmp2_s16, consts.2);
    tmp3 = vmull_lane_s16::<0>(tmp3_s16, consts.1);

    tmp0 = vmlsl_lane_s16::<0>(tmp0, tmp3_s16, consts.0);
    tmp1 = vmlsl_lane_s16::<2>(tmp1, tmp2_s16, consts.0);
    tmp2 = vmlsl_lane_s16::<2>(tmp2, tmp1_s16, consts.0);
    tmp3 = vmlsl_lane_s16::<0>(tmp3, tmp0_s16, consts.0);

    tmp0 = vaddq_s32(tmp0, z3);
    tmp1 = vaddq_s32(tmp1, z4);
    tmp2 = vaddq_s32(tmp2, z3);
    tmp3 = vaddq_s32(tmp3, z4);

    /* Final output stage: descale and narrow to 16-bit. */
    let rows_0123 = int16x4x4_t(
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp10, tmp3)),
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp11, tmp2)),
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp12, tmp1)),
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp13, tmp0)),
    );
    let rows_4567 = int16x4x4_t(
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp13, tmp0)),
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp12, tmp1)),
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp11, tmp2)),
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp10, tmp3)),
    );

    /* Store 4x4 blocks to the intermediate workspace, ready for the second pass.
     * (VST4 transposes the blocks.  We need to operate on rows in the next
     * pass.)
     */
    vst4_s16(workspace_1.first_chunk_mut().expect(WORKSPACE), rows_0123);
    vst4_s16(workspace_2.first_chunk_mut().expect(WORKSPACE), rows_4567);
}

/* Perform dequantization and the first pass of the accurate inverse DCT on a
 * 4x8 block of coefficients.
 *
 * This "sparse" version assumes that the AC coefficients in rows 4-7 are all
 * 0.  This simplifies the IDCT calculation, accelerating overall performance.
 */

// upstream: simd/arm/jidctint-neon.c jsimd_idct_islow_pass1_sparse
#[expect(clippy::too_many_arguments, reason = "the C function's parameters")]
#[expect(clippy::similar_names, reason = "C variable names")]
#[target_feature(enable = "neon")]
#[inline]
fn jsimd_idct_islow_pass1_sparse(
    row0: int16x4_t,
    row1: int16x4_t,
    row2: int16x4_t,
    row3: int16x4_t,
    quant_row0: int16x4_t,
    quant_row1: int16x4_t,
    quant_row2: int16x4_t,
    quant_row3: int16x4_t,
    workspace_1: &mut [i16],
    workspace_2: &mut [i16],
) {
    /* Load constants for IDCT computation. */
    let consts: int16x4x3_t = vld1_s16_x3(islow_consts_x3());

    /* Even part (z3 is all 0) */
    let mut z2_s16 = vmul_s16(row2, quant_row2);

    let mut tmp2 = vmull_lane_s16::<1>(z2_s16, consts.0);
    let mut tmp3 = vmull_lane_s16::<2>(z2_s16, consts.1);

    z2_s16 = vmul_s16(row0, quant_row0);
    let mut tmp0 = vshll_n_s16::<CONST_BITS>(z2_s16);
    let mut tmp1 = vshll_n_s16::<CONST_BITS>(z2_s16);

    let tmp10 = vaddq_s32(tmp0, tmp3);
    let tmp13 = vsubq_s32(tmp0, tmp3);
    let tmp11 = vaddq_s32(tmp1, tmp2);
    let tmp12 = vsubq_s32(tmp1, tmp2);

    /* Odd part (tmp0 and tmp1 are both all 0) */
    let tmp2_s16 = vmul_s16(row3, quant_row3);
    let tmp3_s16 = vmul_s16(row1, quant_row1);

    let z3_s16 = tmp2_s16;
    let z4_s16 = tmp3_s16;

    let mut z3 = vmull_lane_s16::<3>(z3_s16, consts.2);
    let mut z4 = vmull_lane_s16::<3>(z3_s16, consts.1);
    z3 = vmlal_lane_s16::<3>(z3, z4_s16, consts.1);
    z4 = vmlal_lane_s16::<0>(z4, z4_s16, consts.2);

    tmp0 = vmlsl_lane_s16::<0>(z3, tmp3_s16, consts.0);
    tmp1 = vmlsl_lane_s16::<2>(z4, tmp2_s16, consts.0);
    tmp2 = vmlal_lane_s16::<2>(z3, tmp2_s16, consts.2);
    tmp3 = vmlal_lane_s16::<0>(z4, tmp3_s16, consts.1);

    /* Final output stage: descale and narrow to 16-bit. */
    let rows_0123 = int16x4x4_t(
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp10, tmp3)),
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp11, tmp2)),
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp12, tmp1)),
        vrshrn_n_s32::<DESCALE_P1>(vaddq_s32(tmp13, tmp0)),
    );
    let rows_4567 = int16x4x4_t(
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp13, tmp0)),
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp12, tmp1)),
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp11, tmp2)),
        vrshrn_n_s32::<DESCALE_P1>(vsubq_s32(tmp10, tmp3)),
    );

    /* Store 4x4 blocks to the intermediate workspace, ready for the second pass.
     * (VST4 transposes the blocks.  We need to operate on rows in the next
     * pass.)
     */
    vst4_s16(workspace_1.first_chunk_mut().expect(WORKSPACE), rows_0123);
    vst4_s16(workspace_2.first_chunk_mut().expect(WORKSPACE), rows_4567);
}

/* Perform the second pass of the accurate inverse DCT on a 4x8 block of
 * coefficients.  (To process the full 8x8 DCT block, this function-- or some
 * other optimized variant-- needs to be called for both the right and left 4x8
 * blocks.)
 *
 * This "regular" version assumes that no optimization can be made to the IDCT
 * calculation, since no useful set of coefficient values are all 0 after the
 * first pass.
 *
 * Again, the original C implementation of the accurate IDCT (jpeg_idct_slow())
 * can be found in jidctint.c.  Algorithmic changes made here are documented
 * inline.
 */

// upstream: simd/arm/jidctint-neon.c jsimd_idct_islow_pass2_regular
// C defines the helper `static INLINE`, which upstream's build makes
// `__attribute__((always_inline))`. Rust rejects `#[inline(always)]` on a
// `#[target_feature]` function, and LLVM keeps the `#[inline]` function out
// of line, so the helper is a macro that expands in its caller's Neon
// context, with the C parameters as `let` bindings (ADR 0004, 2026-11-13
// amendment).
macro_rules! jsimd_idct_islow_pass2_regular {
    ($workspace:expr, $output_buf:expr, $output_col:expr, $buf_offset:expr) => {{
        let workspace: &[i16] = $workspace;
        let mut output_buf: JSAMPROWS_MUT<'_, '_> = $output_buf;
        let output_col: JDIMENSION = $output_col;
        let buf_offset: u32 = $buf_offset;

        /* Load constants for IDCT computation. */
        let consts: int16x4x3_t = vld1_s16_x3(islow_consts_x3());

        /* Even part */
        let mut z2_s16 = vld1_s16(workspace[2 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));
        let mut z3_s16 = vld1_s16(workspace[6 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));

        let mut tmp2 = vmull_lane_s16::<1>(z2_s16, consts.0);
        let mut tmp3 = vmull_lane_s16::<2>(z2_s16, consts.1);
        tmp2 = vmlal_lane_s16::<1>(tmp2, z3_s16, consts.2);
        tmp3 = vmlal_lane_s16::<1>(tmp3, z3_s16, consts.0);

        z2_s16 = vld1_s16(workspace[0 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));
        z3_s16 = vld1_s16(workspace[4 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));

        let mut tmp0 = vshll_n_s16::<CONST_BITS>(vadd_s16(z2_s16, z3_s16));
        let mut tmp1 = vshll_n_s16::<CONST_BITS>(vsub_s16(z2_s16, z3_s16));

        let tmp10 = vaddq_s32(tmp0, tmp3);
        let tmp13 = vsubq_s32(tmp0, tmp3);
        let tmp11 = vaddq_s32(tmp1, tmp2);
        let tmp12 = vsubq_s32(tmp1, tmp2);

        /* Odd part */
        let tmp0_s16 = vld1_s16(workspace[7 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));
        let tmp1_s16 = vld1_s16(workspace[5 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));
        let tmp2_s16 = vld1_s16(workspace[3 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));
        let tmp3_s16 = vld1_s16(workspace[1 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));

        z3_s16 = vadd_s16(tmp0_s16, tmp2_s16);
        let z4_s16 = vadd_s16(tmp1_s16, tmp3_s16);

        /* Implementation as per jpeg_idct_islow() in jidctint.c:
         *   z5 = (z3 + z4) * 1.175875602;
         *   z3 = z3 * -1.961570560;  z4 = z4 * -0.390180644;
         *   z3 += z5;  z4 += z5;
         *
         * This implementation:
         *   z3 = z3 * (1.175875602 - 1.961570560) + z4 * 1.175875602;
         *   z4 = z3 * 1.175875602 + z4 * (1.175875602 - 0.390180644);
         */

        let mut z3 = vmull_lane_s16::<3>(z3_s16, consts.2);
        let mut z4 = vmull_lane_s16::<3>(z3_s16, consts.1);
        z3 = vmlal_lane_s16::<3>(z3, z4_s16, consts.1);
        z4 = vmlal_lane_s16::<0>(z4, z4_s16, consts.2);

        /* Implementation as per jpeg_idct_islow() in jidctint.c:
         *   z1 = tmp0 + tmp3;  z2 = tmp1 + tmp2;
         *   tmp0 = tmp0 * 0.298631336;  tmp1 = tmp1 * 2.053119869;
         *   tmp2 = tmp2 * 3.072711026;  tmp3 = tmp3 * 1.501321110;
         *   z1 = z1 * -0.899976223;  z2 = z2 * -2.562915447;
         *   tmp0 += z1 + z3;  tmp1 += z2 + z4;
         *   tmp2 += z2 + z3;  tmp3 += z1 + z4;
         *
         * This implementation:
         *   tmp0 = tmp0 * (0.298631336 - 0.899976223) + tmp3 * -0.899976223;
         *   tmp1 = tmp1 * (2.053119869 - 2.562915447) + tmp2 * -2.562915447;
         *   tmp2 = tmp1 * -2.562915447 + tmp2 * (3.072711026 - 2.562915447);
         *   tmp3 = tmp0 * -0.899976223 + tmp3 * (1.501321110 - 0.899976223);
         *   tmp0 += z3;  tmp1 += z4;
         *   tmp2 += z3;  tmp3 += z4;
         */

        tmp0 = vmull_lane_s16::<3>(tmp0_s16, consts.0);
        tmp1 = vmull_lane_s16::<1>(tmp1_s16, consts.1);
        tmp2 = vmull_lane_s16::<2>(tmp2_s16, consts.2);
        tmp3 = vmull_lane_s16::<0>(tmp3_s16, consts.1);

        tmp0 = vmlsl_lane_s16::<0>(tmp0, tmp3_s16, consts.0);
        tmp1 = vmlsl_lane_s16::<2>(tmp1, tmp2_s16, consts.0);
        tmp2 = vmlsl_lane_s16::<2>(tmp2, tmp1_s16, consts.0);
        tmp3 = vmlsl_lane_s16::<0>(tmp3, tmp0_s16, consts.0);

        tmp0 = vaddq_s32(tmp0, z3);
        tmp1 = vaddq_s32(tmp1, z4);
        tmp2 = vaddq_s32(tmp2, z3);
        tmp3 = vaddq_s32(tmp3, z4);

        /* Final output stage: descale and narrow to 16-bit. */
        let cols_02_s16 = vcombine_s16(vaddhn_s32(tmp10, tmp3), vaddhn_s32(tmp12, tmp1));
        let cols_13_s16 = vcombine_s16(vaddhn_s32(tmp11, tmp2), vaddhn_s32(tmp13, tmp0));
        let cols_46_s16 = vcombine_s16(vsubhn_s32(tmp13, tmp0), vsubhn_s32(tmp11, tmp2));
        let cols_57_s16 = vcombine_s16(vsubhn_s32(tmp12, tmp1), vsubhn_s32(tmp10, tmp3));
        /* Descale and narrow to 8-bit. */
        let cols_02_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_02_s16);
        let cols_13_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_13_s16);
        let cols_46_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_46_s16);
        let cols_57_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_57_s16);
        /* Clamp to range [0-255]. */
        let cols_02_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_02_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );
        let cols_13_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_13_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );
        let cols_46_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_46_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );
        let cols_57_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_57_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );

        /* Transpose 4x8 block and store to memory.  (Zipping adjacent columns
         * together allows us to store 16-bit elements.)
         */
        let cols_01_23 = vzip_u8(cols_02_u8, cols_13_u8);
        let cols_45_67 = vzip_u8(cols_46_u8, cols_57_u8);
        let cols_01_23_45_67 = uint16x4x4_t(
            vreinterpret_u16_u8(cols_01_23.0),
            vreinterpret_u16_u8(cols_01_23.1),
            vreinterpret_u16_u8(cols_45_67.0),
            vreinterpret_u16_u8(cols_45_67.1),
        );

        /* VST4 of 16-bit elements completes the transpose. */
        let outptr0 = &mut output_buf.row_mut((buf_offset + 0) as usize)[output_col as usize..];
        vst4_lane_u16::<0>(outptr0.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
        let outptr1 = &mut output_buf.row_mut((buf_offset + 1) as usize)[output_col as usize..];
        vst4_lane_u16::<1>(outptr1.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
        let outptr2 = &mut output_buf.row_mut((buf_offset + 2) as usize)[output_col as usize..];
        vst4_lane_u16::<2>(outptr2.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
        let outptr3 = &mut output_buf.row_mut((buf_offset + 3) as usize)[output_col as usize..];
        vst4_lane_u16::<3>(outptr3.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
    }};
}
use jsimd_idct_islow_pass2_regular;

/* Performs the second pass of the accurate inverse DCT on a 4x8 block
 * of coefficients.
 *
 * This "sparse" version assumes that the coefficient values (after the first
 * pass) in rows 4-7 are all 0.  This simplifies the IDCT calculation,
 * accelerating overall performance.
 */

// upstream: simd/arm/jidctint-neon.c jsimd_idct_islow_pass2_sparse
// C defines the helper `static INLINE`, which upstream's build makes
// `__attribute__((always_inline))`. Rust rejects `#[inline(always)]` on a
// `#[target_feature]` function, and LLVM keeps the `#[inline]` function out
// of line, so the helper is a macro that expands in its caller's Neon
// context, with the C parameters as `let` bindings (ADR 0004, 2026-11-13
// amendment).
macro_rules! jsimd_idct_islow_pass2_sparse {
    ($workspace:expr, $output_buf:expr, $output_col:expr, $buf_offset:expr) => {{
        let workspace: &[i16] = $workspace;
        let mut output_buf: JSAMPROWS_MUT<'_, '_> = $output_buf;
        let output_col: JDIMENSION = $output_col;
        let buf_offset: u32 = $buf_offset;

        /* Load constants for IDCT computation. */
        let consts: int16x4x3_t = vld1_s16_x3(islow_consts_x3());

        /* Even part (z3 is all 0) */
        let mut z2_s16 = vld1_s16(workspace[2 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));

        let mut tmp2 = vmull_lane_s16::<1>(z2_s16, consts.0);
        let mut tmp3 = vmull_lane_s16::<2>(z2_s16, consts.1);

        z2_s16 = vld1_s16(workspace[0 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));
        let mut tmp0 = vshll_n_s16::<CONST_BITS>(z2_s16);
        let mut tmp1 = vshll_n_s16::<CONST_BITS>(z2_s16);

        let tmp10 = vaddq_s32(tmp0, tmp3);
        let tmp13 = vsubq_s32(tmp0, tmp3);
        let tmp11 = vaddq_s32(tmp1, tmp2);
        let tmp12 = vsubq_s32(tmp1, tmp2);

        /* Odd part (tmp0 and tmp1 are both all 0) */
        let tmp2_s16 = vld1_s16(workspace[3 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));
        let tmp3_s16 = vld1_s16(workspace[1 * DCTSIZE / 2..].first_chunk().expect(WORKSPACE));

        let z3_s16 = tmp2_s16;
        let z4_s16 = tmp3_s16;

        let mut z3 = vmull_lane_s16::<3>(z3_s16, consts.2);
        z3 = vmlal_lane_s16::<3>(z3, z4_s16, consts.1);
        let mut z4 = vmull_lane_s16::<3>(z3_s16, consts.1);
        z4 = vmlal_lane_s16::<0>(z4, z4_s16, consts.2);

        tmp0 = vmlsl_lane_s16::<0>(z3, tmp3_s16, consts.0);
        tmp1 = vmlsl_lane_s16::<2>(z4, tmp2_s16, consts.0);
        tmp2 = vmlal_lane_s16::<2>(z3, tmp2_s16, consts.2);
        tmp3 = vmlal_lane_s16::<0>(z4, tmp3_s16, consts.1);

        /* Final output stage: descale and narrow to 16-bit. */
        let cols_02_s16 = vcombine_s16(vaddhn_s32(tmp10, tmp3), vaddhn_s32(tmp12, tmp1));
        let cols_13_s16 = vcombine_s16(vaddhn_s32(tmp11, tmp2), vaddhn_s32(tmp13, tmp0));
        let cols_46_s16 = vcombine_s16(vsubhn_s32(tmp13, tmp0), vsubhn_s32(tmp11, tmp2));
        let cols_57_s16 = vcombine_s16(vsubhn_s32(tmp12, tmp1), vsubhn_s32(tmp10, tmp3));
        /* Descale and narrow to 8-bit. */
        let cols_02_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_02_s16);
        let cols_13_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_13_s16);
        let cols_46_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_46_s16);
        let cols_57_s8 = vqrshrn_n_s16::<{ DESCALE_P2 - 16 }>(cols_57_s16);
        /* Clamp to range [0-255]. */
        let cols_02_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_02_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );
        let cols_13_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_13_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );
        let cols_46_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_46_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );
        let cols_57_u8 = vadd_u8(
            vreinterpret_u8_s8(cols_57_s8),
            vdup_n_u8(CENTERJSAMPLE as u8),
        );

        /* Transpose 4x8 block and store to memory.  (Zipping adjacent columns
         * together allows us to store 16-bit elements.)
         */
        let cols_01_23 = vzip_u8(cols_02_u8, cols_13_u8);
        let cols_45_67 = vzip_u8(cols_46_u8, cols_57_u8);
        let cols_01_23_45_67 = uint16x4x4_t(
            vreinterpret_u16_u8(cols_01_23.0),
            vreinterpret_u16_u8(cols_01_23.1),
            vreinterpret_u16_u8(cols_45_67.0),
            vreinterpret_u16_u8(cols_45_67.1),
        );

        /* VST4 of 16-bit elements completes the transpose. */
        let outptr0 = &mut output_buf.row_mut((buf_offset + 0) as usize)[output_col as usize..];
        vst4_lane_u16::<0>(outptr0.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
        let outptr1 = &mut output_buf.row_mut((buf_offset + 1) as usize)[output_col as usize..];
        vst4_lane_u16::<1>(outptr1.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
        let outptr2 = &mut output_buf.row_mut((buf_offset + 2) as usize)[output_col as usize..];
        vst4_lane_u16::<2>(outptr2.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
        let outptr3 = &mut output_buf.row_mut((buf_offset + 3) as usize)[output_col as usize..];
        vst4_lane_u16::<3>(outptr3.first_chunk_mut().expect(OUTPUT), cols_01_23_45_67);
    }};
}
use jsimd_idct_islow_pass2_sparse;
