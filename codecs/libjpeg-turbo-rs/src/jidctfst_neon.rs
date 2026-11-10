// libjpeg-turbo 3.2.1, simd/arm/jidctfst-neon.c:
//
// /*
//  * Fast Integer Inverse DCT (Arm Neon)
//  *
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2024-2025, D. R. Commander.
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
//! Translation of `simd/arm/jidctfst-neon.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! `IFAST_MULT_TYPE` is 16-bit with `WITH_SIMD` and `IFAST_SCALE_BITS` is 2,
//! as `jsimd_set_idct_ifast` requires, and the multiplier table is the
//! component's `ifast_array`.
//!
//! The pointers of the C code become slices: an output row `output_buf[i] +
//! output_col` is the row's slice from `output_col` on, and each load or
//! store converts the first elements of its slice to the array that the
//! `neon` wrapper takes. The kernel computes and stores one output row
//! pointer at a time, as `output_buf` lends one row at a time, where C
//! computes the eight pointers before the eight stores. Each store writes
//! the 8 samples of the block's row and nothing past them.

#![forbid(unsafe_code)]

use core::arch::aarch64::{
    vadd_s16, vaddq_s8, vaddq_s16, vcombine_s8, vcombine_s16, vdupq_n_u8, vget_high_s16,
    vget_low_s16, vgetq_lane_s64, vmul_s16, vmulq_s16, vorrq_s16, vqdmulh_lane_s16,
    vqdmulhq_lane_s16, vqshrn_n_s16, vreinterpretq_s8_u8, vreinterpretq_s16_s32,
    vreinterpretq_s32_s16, vreinterpretq_s64_s16, vreinterpretq_u8_s8, vreinterpretq_u8_u16,
    vreinterpretq_u8_u32, vreinterpretq_u16_u8, vreinterpretq_u32_u8, vreinterpretq_u64_u8,
    vsub_s16, vsubq_s16, vtrnq_s16, vtrnq_s32, vtrnq_u8, vtrnq_u16, vzipq_s32, vzipq_u32,
};

use crate::jddctmgr::multiplier_table;
use crate::jmorecfg::{CENTERJSAMPLE, JDIMENSION};
use crate::jpeglib::{DCTSIZE, JBLOCK, JSAMPROWS_MUT};
use crate::neon::{vld1_s16, vld1q_s16, vst1q_lane_u64};

/* jsimd_idct_ifast_neon() performs dequantization and a fast, not so accurate
 * inverse DCT (Discrete Cosine Transform) on one block of coefficients.  It
 * uses the same calculations and produces exactly the same output as IJG's
 * original jpeg_idct_ifast() function, which can be found in jidctfst.c.
 *
 * Scaled integer constants are used to avoid floating-point arithmetic:
 *    0.082392200 =  2688 * 2^-15
 *    0.414213562 = 13568 * 2^-15
 *    0.847759065 = 27776 * 2^-15
 *    0.613125930 = 20096 * 2^-15
 *
 * See jidctfst.c for further details of the IDCT algorithm.  Where possible,
 * the variable names and comments here in jsimd_idct_ifast_neon() match up
 * with those in jpeg_idct_ifast().
 */

// upstream: simd/arm/jidctfst-neon.c PASS1_BITS
const PASS1_BITS: i32 = 2;

// upstream: simd/arm/jidctfst-neon.c F_0_082
const F_0_082: i16 = 2688;
// upstream: simd/arm/jidctfst-neon.c F_0_414
const F_0_414: i16 = 13568;
// upstream: simd/arm/jidctfst-neon.c F_0_847
const F_0_847: i16 = 27776;
// upstream: simd/arm/jidctfst-neon.c F_0_613
const F_0_613: i16 = 20096;

// upstream: simd/arm/jidctfst-neon.c jsimd_idct_ifast_neon_consts
#[expect(non_upper_case_globals, reason = "C variable name")]
static jsimd_idct_ifast_neon_consts: [i16; 4] = [F_0_082, F_0_414, F_0_847, F_0_613];

/// The message of a coefficient load that cannot fail. Not an upstream item.
const COEFS: &str = "a block row holds 8 coefficients";
/// The message of a multiplier-table load that cannot fail. Not an upstream
/// item.
const QUANTS: &str = "a table row holds 8 multipliers";
/// The message of an output row that is too short. Not an upstream item.
const OUTPUT: &str = "an output row holds 8 samples from output_col on";

// upstream: simd/arm/jidctfst-neon.c jsimd_idct_ifast_neon
/// Performs dequantization and the fast integer inverse DCT on one block of
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
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "C writes k * DCTSIZE row offsets with k = 0 and 1"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversion of CENTERJSAMPLE to uint8_t"
)]
#[target_feature(enable = "neon")]
pub fn jsimd_idct_ifast_neon(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let quantptr = dct_table.ifast_array();

    /* Load DCT coefficients. */
    let mut row0 = vld1q_s16(coef_block[0 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row1 = vld1q_s16(coef_block[1 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row2 = vld1q_s16(coef_block[2 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row3 = vld1q_s16(coef_block[3 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row4 = vld1q_s16(coef_block[4 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row5 = vld1q_s16(coef_block[5 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row6 = vld1q_s16(coef_block[6 * DCTSIZE..].first_chunk().expect(COEFS));
    let mut row7 = vld1q_s16(coef_block[7 * DCTSIZE..].first_chunk().expect(COEFS));

    /* Load quantization table values for DC coefficients. */
    let quant_row0 = vld1q_s16(quantptr[0 * DCTSIZE..].first_chunk().expect(QUANTS));
    /* Dequantize DC coefficients. */
    row0 = vmulq_s16(row0, quant_row0);

    /* Construct bitmap to test if all AC coefficients are 0. */
    let mut bitmap = vorrq_s16(row1, row2);
    bitmap = vorrq_s16(bitmap, row3);
    bitmap = vorrq_s16(bitmap, row4);
    bitmap = vorrq_s16(bitmap, row5);
    bitmap = vorrq_s16(bitmap, row6);
    bitmap = vorrq_s16(bitmap, row7);

    let left_ac_bitmap: i64 = vgetq_lane_s64::<0>(vreinterpretq_s64_s16(bitmap));
    let right_ac_bitmap: i64 = vgetq_lane_s64::<1>(vreinterpretq_s64_s16(bitmap));

    /* Load IDCT conversion constants. */
    let consts = vld1_s16(&jsimd_idct_ifast_neon_consts);

    if left_ac_bitmap == 0 && right_ac_bitmap == 0 {
        /* All AC coefficients are zero.
         * Compute DC values and duplicate into vectors.
         */
        let dcval = row0;
        row1 = dcval;
        row2 = dcval;
        row3 = dcval;
        row4 = dcval;
        row5 = dcval;
        row6 = dcval;
        row7 = dcval;
    } else if left_ac_bitmap == 0 {
        /* AC coefficients are zero for columns 0, 1, 2, and 3.
         * Use DC values for these columns.
         */
        let dcval = vget_low_s16(row0);

        /* Commence regular fast IDCT computation for columns 4, 5, 6, and 7. */

        /* Load quantization table. */
        let quant_row1 = vld1_s16(quantptr[1 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row2 = vld1_s16(quantptr[2 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row3 = vld1_s16(quantptr[3 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row4 = vld1_s16(quantptr[4 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row5 = vld1_s16(quantptr[5 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row6 = vld1_s16(quantptr[6 * DCTSIZE + 4..].first_chunk().expect(QUANTS));
        let quant_row7 = vld1_s16(quantptr[7 * DCTSIZE + 4..].first_chunk().expect(QUANTS));

        /* Even part: dequantize DCT coefficients. */
        let mut tmp0 = vget_high_s16(row0);
        let mut tmp1 = vmul_s16(vget_high_s16(row2), quant_row2);
        let mut tmp2 = vmul_s16(vget_high_s16(row4), quant_row4);
        let mut tmp3 = vmul_s16(vget_high_s16(row6), quant_row6);

        let mut tmp10 = vadd_s16(tmp0, tmp2); /* phase 3 */
        let mut tmp11 = vsub_s16(tmp0, tmp2);

        let tmp13 = vadd_s16(tmp1, tmp3); /* phases 5-3 */
        let tmp1_sub_tmp3 = vsub_s16(tmp1, tmp3);
        let mut tmp12 = vqdmulh_lane_s16::<1>(tmp1_sub_tmp3, consts);
        tmp12 = vadd_s16(tmp12, tmp1_sub_tmp3);
        tmp12 = vsub_s16(tmp12, tmp13);

        tmp0 = vadd_s16(tmp10, tmp13); /* phase 2 */
        tmp3 = vsub_s16(tmp10, tmp13);
        tmp1 = vadd_s16(tmp11, tmp12);
        tmp2 = vsub_s16(tmp11, tmp12);

        /* Odd part: dequantize DCT coefficients. */
        let mut tmp4 = vmul_s16(vget_high_s16(row1), quant_row1);
        let mut tmp5 = vmul_s16(vget_high_s16(row3), quant_row3);
        let mut tmp6 = vmul_s16(vget_high_s16(row5), quant_row5);
        let mut tmp7 = vmul_s16(vget_high_s16(row7), quant_row7);

        let z13 = vadd_s16(tmp6, tmp5); /* phase 6 */
        let neg_z10 = vsub_s16(tmp5, tmp6);
        let z11 = vadd_s16(tmp4, tmp7);
        let z12 = vsub_s16(tmp4, tmp7);

        tmp7 = vadd_s16(z11, z13); /* phase 5 */
        let z11_sub_z13 = vsub_s16(z11, z13);
        tmp11 = vqdmulh_lane_s16::<1>(z11_sub_z13, consts);
        tmp11 = vadd_s16(tmp11, z11_sub_z13);

        let z10_add_z12 = vsub_s16(z12, neg_z10);
        let mut z5 = vqdmulh_lane_s16::<2>(z10_add_z12, consts);
        z5 = vadd_s16(z5, z10_add_z12);
        tmp10 = vqdmulh_lane_s16::<0>(z12, consts);
        tmp10 = vadd_s16(tmp10, z12);
        tmp10 = vsub_s16(tmp10, z5);
        tmp12 = vqdmulh_lane_s16::<3>(neg_z10, consts);
        tmp12 = vadd_s16(tmp12, vadd_s16(neg_z10, neg_z10));
        tmp12 = vadd_s16(tmp12, z5);

        tmp6 = vsub_s16(tmp12, tmp7); /* phase 2 */
        tmp5 = vsub_s16(tmp11, tmp6);
        tmp4 = vadd_s16(tmp10, tmp5);

        row0 = vcombine_s16(dcval, vadd_s16(tmp0, tmp7));
        row7 = vcombine_s16(dcval, vsub_s16(tmp0, tmp7));
        row1 = vcombine_s16(dcval, vadd_s16(tmp1, tmp6));
        row6 = vcombine_s16(dcval, vsub_s16(tmp1, tmp6));
        row2 = vcombine_s16(dcval, vadd_s16(tmp2, tmp5));
        row5 = vcombine_s16(dcval, vsub_s16(tmp2, tmp5));
        row4 = vcombine_s16(dcval, vadd_s16(tmp3, tmp4));
        row3 = vcombine_s16(dcval, vsub_s16(tmp3, tmp4));
    } else if right_ac_bitmap == 0 {
        /* AC coefficients are zero for columns 4, 5, 6, and 7.
         * Use DC values for these columns.
         */
        let dcval = vget_high_s16(row0);

        /* Commence regular fast IDCT computation for columns 0, 1, 2, and 3. */

        /* Load quantization table. */
        let quant_row1 = vld1_s16(quantptr[1 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row2 = vld1_s16(quantptr[2 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row3 = vld1_s16(quantptr[3 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row4 = vld1_s16(quantptr[4 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row5 = vld1_s16(quantptr[5 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row6 = vld1_s16(quantptr[6 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row7 = vld1_s16(quantptr[7 * DCTSIZE..].first_chunk().expect(QUANTS));

        /* Even part: dequantize DCT coefficients. */
        let mut tmp0 = vget_low_s16(row0);
        let mut tmp1 = vmul_s16(vget_low_s16(row2), quant_row2);
        let mut tmp2 = vmul_s16(vget_low_s16(row4), quant_row4);
        let mut tmp3 = vmul_s16(vget_low_s16(row6), quant_row6);

        let mut tmp10 = vadd_s16(tmp0, tmp2); /* phase 3 */
        let mut tmp11 = vsub_s16(tmp0, tmp2);

        let tmp13 = vadd_s16(tmp1, tmp3); /* phases 5-3 */
        let tmp1_sub_tmp3 = vsub_s16(tmp1, tmp3);
        let mut tmp12 = vqdmulh_lane_s16::<1>(tmp1_sub_tmp3, consts);
        tmp12 = vadd_s16(tmp12, tmp1_sub_tmp3);
        tmp12 = vsub_s16(tmp12, tmp13);

        tmp0 = vadd_s16(tmp10, tmp13); /* phase 2 */
        tmp3 = vsub_s16(tmp10, tmp13);
        tmp1 = vadd_s16(tmp11, tmp12);
        tmp2 = vsub_s16(tmp11, tmp12);

        /* Odd part: dequantize DCT coefficients. */
        let mut tmp4 = vmul_s16(vget_low_s16(row1), quant_row1);
        let mut tmp5 = vmul_s16(vget_low_s16(row3), quant_row3);
        let mut tmp6 = vmul_s16(vget_low_s16(row5), quant_row5);
        let mut tmp7 = vmul_s16(vget_low_s16(row7), quant_row7);

        let z13 = vadd_s16(tmp6, tmp5); /* phase 6 */
        let neg_z10 = vsub_s16(tmp5, tmp6);
        let z11 = vadd_s16(tmp4, tmp7);
        let z12 = vsub_s16(tmp4, tmp7);

        tmp7 = vadd_s16(z11, z13); /* phase 5 */
        let z11_sub_z13 = vsub_s16(z11, z13);
        tmp11 = vqdmulh_lane_s16::<1>(z11_sub_z13, consts);
        tmp11 = vadd_s16(tmp11, z11_sub_z13);

        let z10_add_z12 = vsub_s16(z12, neg_z10);
        let mut z5 = vqdmulh_lane_s16::<2>(z10_add_z12, consts);
        z5 = vadd_s16(z5, z10_add_z12);
        tmp10 = vqdmulh_lane_s16::<0>(z12, consts);
        tmp10 = vadd_s16(tmp10, z12);
        tmp10 = vsub_s16(tmp10, z5);
        tmp12 = vqdmulh_lane_s16::<3>(neg_z10, consts);
        tmp12 = vadd_s16(tmp12, vadd_s16(neg_z10, neg_z10));
        tmp12 = vadd_s16(tmp12, z5);

        tmp6 = vsub_s16(tmp12, tmp7); /* phase 2 */
        tmp5 = vsub_s16(tmp11, tmp6);
        tmp4 = vadd_s16(tmp10, tmp5);

        row0 = vcombine_s16(vadd_s16(tmp0, tmp7), dcval);
        row7 = vcombine_s16(vsub_s16(tmp0, tmp7), dcval);
        row1 = vcombine_s16(vadd_s16(tmp1, tmp6), dcval);
        row6 = vcombine_s16(vsub_s16(tmp1, tmp6), dcval);
        row2 = vcombine_s16(vadd_s16(tmp2, tmp5), dcval);
        row5 = vcombine_s16(vsub_s16(tmp2, tmp5), dcval);
        row4 = vcombine_s16(vadd_s16(tmp3, tmp4), dcval);
        row3 = vcombine_s16(vsub_s16(tmp3, tmp4), dcval);
    } else {
        /* Some AC coefficients are non-zero; full IDCT calculation required. */

        /* Load quantization table. */
        let quant_row1 = vld1q_s16(quantptr[1 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row2 = vld1q_s16(quantptr[2 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row3 = vld1q_s16(quantptr[3 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row4 = vld1q_s16(quantptr[4 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row5 = vld1q_s16(quantptr[5 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row6 = vld1q_s16(quantptr[6 * DCTSIZE..].first_chunk().expect(QUANTS));
        let quant_row7 = vld1q_s16(quantptr[7 * DCTSIZE..].first_chunk().expect(QUANTS));

        /* Even part: dequantize DCT coefficients. */
        let mut tmp0 = row0;
        let mut tmp1 = vmulq_s16(row2, quant_row2);
        let mut tmp2 = vmulq_s16(row4, quant_row4);
        let mut tmp3 = vmulq_s16(row6, quant_row6);

        let mut tmp10 = vaddq_s16(tmp0, tmp2); /* phase 3 */
        let mut tmp11 = vsubq_s16(tmp0, tmp2);

        let tmp13 = vaddq_s16(tmp1, tmp3); /* phases 5-3 */
        let tmp1_sub_tmp3 = vsubq_s16(tmp1, tmp3);
        let mut tmp12 = vqdmulhq_lane_s16::<1>(tmp1_sub_tmp3, consts);
        tmp12 = vaddq_s16(tmp12, tmp1_sub_tmp3);
        tmp12 = vsubq_s16(tmp12, tmp13);

        tmp0 = vaddq_s16(tmp10, tmp13); /* phase 2 */
        tmp3 = vsubq_s16(tmp10, tmp13);
        tmp1 = vaddq_s16(tmp11, tmp12);
        tmp2 = vsubq_s16(tmp11, tmp12);

        /* Odd part: dequantize DCT coefficients. */
        let mut tmp4 = vmulq_s16(row1, quant_row1);
        let mut tmp5 = vmulq_s16(row3, quant_row3);
        let mut tmp6 = vmulq_s16(row5, quant_row5);
        let mut tmp7 = vmulq_s16(row7, quant_row7);

        let z13 = vaddq_s16(tmp6, tmp5); /* phase 6 */
        let neg_z10 = vsubq_s16(tmp5, tmp6);
        let z11 = vaddq_s16(tmp4, tmp7);
        let z12 = vsubq_s16(tmp4, tmp7);

        tmp7 = vaddq_s16(z11, z13); /* phase 5 */
        let z11_sub_z13 = vsubq_s16(z11, z13);
        tmp11 = vqdmulhq_lane_s16::<1>(z11_sub_z13, consts);
        tmp11 = vaddq_s16(tmp11, z11_sub_z13);

        let z10_add_z12 = vsubq_s16(z12, neg_z10);
        let mut z5 = vqdmulhq_lane_s16::<2>(z10_add_z12, consts);
        z5 = vaddq_s16(z5, z10_add_z12);
        tmp10 = vqdmulhq_lane_s16::<0>(z12, consts);
        tmp10 = vaddq_s16(tmp10, z12);
        tmp10 = vsubq_s16(tmp10, z5);
        tmp12 = vqdmulhq_lane_s16::<3>(neg_z10, consts);
        tmp12 = vaddq_s16(tmp12, vaddq_s16(neg_z10, neg_z10));
        tmp12 = vaddq_s16(tmp12, z5);

        tmp6 = vsubq_s16(tmp12, tmp7); /* phase 2 */
        tmp5 = vsubq_s16(tmp11, tmp6);
        tmp4 = vaddq_s16(tmp10, tmp5);

        row0 = vaddq_s16(tmp0, tmp7);
        row7 = vsubq_s16(tmp0, tmp7);
        row1 = vaddq_s16(tmp1, tmp6);
        row6 = vsubq_s16(tmp1, tmp6);
        row2 = vaddq_s16(tmp2, tmp5);
        row5 = vsubq_s16(tmp2, tmp5);
        row4 = vaddq_s16(tmp3, tmp4);
        row3 = vsubq_s16(tmp3, tmp4);
    }

    /* Transpose rows to work on columns in pass 2. */
    let rows_01 = vtrnq_s16(row0, row1);
    let rows_23 = vtrnq_s16(row2, row3);
    let rows_45 = vtrnq_s16(row4, row5);
    let rows_67 = vtrnq_s16(row6, row7);

    let rows_0145_l = vtrnq_s32(
        vreinterpretq_s32_s16(rows_01.0),
        vreinterpretq_s32_s16(rows_45.0),
    );
    let rows_0145_h = vtrnq_s32(
        vreinterpretq_s32_s16(rows_01.1),
        vreinterpretq_s32_s16(rows_45.1),
    );
    let rows_2367_l = vtrnq_s32(
        vreinterpretq_s32_s16(rows_23.0),
        vreinterpretq_s32_s16(rows_67.0),
    );
    let rows_2367_h = vtrnq_s32(
        vreinterpretq_s32_s16(rows_23.1),
        vreinterpretq_s32_s16(rows_67.1),
    );

    let cols_04 = vzipq_s32(rows_0145_l.0, rows_2367_l.0);
    let cols_15 = vzipq_s32(rows_0145_h.0, rows_2367_h.0);
    let cols_26 = vzipq_s32(rows_0145_l.1, rows_2367_l.1);
    let cols_37 = vzipq_s32(rows_0145_h.1, rows_2367_h.1);

    let mut col0 = vreinterpretq_s16_s32(cols_04.0);
    let mut col1 = vreinterpretq_s16_s32(cols_15.0);
    let mut col2 = vreinterpretq_s16_s32(cols_26.0);
    let mut col3 = vreinterpretq_s16_s32(cols_37.0);
    let mut col4 = vreinterpretq_s16_s32(cols_04.1);
    let mut col5 = vreinterpretq_s16_s32(cols_15.1);
    let mut col6 = vreinterpretq_s16_s32(cols_26.1);
    let mut col7 = vreinterpretq_s16_s32(cols_37.1);

    /* 1-D IDCT, pass 2 */

    /* Even part */
    let mut tmp10 = vaddq_s16(col0, col4);
    let mut tmp11 = vsubq_s16(col0, col4);

    let tmp13 = vaddq_s16(col2, col6);
    let col2_sub_col6 = vsubq_s16(col2, col6);
    let mut tmp12 = vqdmulhq_lane_s16::<1>(col2_sub_col6, consts);
    tmp12 = vaddq_s16(tmp12, col2_sub_col6);
    tmp12 = vsubq_s16(tmp12, tmp13);

    let tmp0 = vaddq_s16(tmp10, tmp13);
    let tmp3 = vsubq_s16(tmp10, tmp13);
    let tmp1 = vaddq_s16(tmp11, tmp12);
    let tmp2 = vsubq_s16(tmp11, tmp12);

    /* Odd part */
    let z13 = vaddq_s16(col5, col3);
    let neg_z10 = vsubq_s16(col3, col5);
    let z11 = vaddq_s16(col1, col7);
    let z12 = vsubq_s16(col1, col7);

    let tmp7 = vaddq_s16(z11, z13); /* phase 5 */
    let z11_sub_z13 = vsubq_s16(z11, z13);
    tmp11 = vqdmulhq_lane_s16::<1>(z11_sub_z13, consts);
    tmp11 = vaddq_s16(tmp11, z11_sub_z13);

    let z10_add_z12 = vsubq_s16(z12, neg_z10);
    let mut z5 = vqdmulhq_lane_s16::<2>(z10_add_z12, consts);
    z5 = vaddq_s16(z5, z10_add_z12);
    tmp10 = vqdmulhq_lane_s16::<0>(z12, consts);
    tmp10 = vaddq_s16(tmp10, z12);
    tmp10 = vsubq_s16(tmp10, z5);
    tmp12 = vqdmulhq_lane_s16::<3>(neg_z10, consts);
    tmp12 = vaddq_s16(tmp12, vaddq_s16(neg_z10, neg_z10));
    tmp12 = vaddq_s16(tmp12, z5);

    let tmp6 = vsubq_s16(tmp12, tmp7); /* phase 2 */
    let tmp5 = vsubq_s16(tmp11, tmp6);
    let tmp4 = vaddq_s16(tmp10, tmp5);

    col0 = vaddq_s16(tmp0, tmp7);
    col7 = vsubq_s16(tmp0, tmp7);
    col1 = vaddq_s16(tmp1, tmp6);
    col6 = vsubq_s16(tmp1, tmp6);
    col2 = vaddq_s16(tmp2, tmp5);
    col5 = vsubq_s16(tmp2, tmp5);
    col4 = vaddq_s16(tmp3, tmp4);
    col3 = vsubq_s16(tmp3, tmp4);

    /* Scale down by a factor of 8, narrowing to 8-bit. */
    let cols_01_s8 = vcombine_s8(
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col0),
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col1),
    );
    let cols_45_s8 = vcombine_s8(
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col4),
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col5),
    );
    let cols_23_s8 = vcombine_s8(
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col2),
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col3),
    );
    let cols_67_s8 = vcombine_s8(
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col6),
        vqshrn_n_s16::<{ PASS1_BITS + 3 }>(col7),
    );
    /* Clamp to range [0-255]. */
    let cols_01 = vreinterpretq_u8_s8(vaddq_s8(
        cols_01_s8,
        vreinterpretq_s8_u8(vdupq_n_u8(CENTERJSAMPLE as u8)),
    ));
    let cols_45 = vreinterpretq_u8_s8(vaddq_s8(
        cols_45_s8,
        vreinterpretq_s8_u8(vdupq_n_u8(CENTERJSAMPLE as u8)),
    ));
    let cols_23 = vreinterpretq_u8_s8(vaddq_s8(
        cols_23_s8,
        vreinterpretq_s8_u8(vdupq_n_u8(CENTERJSAMPLE as u8)),
    ));
    let cols_67 = vreinterpretq_u8_s8(vaddq_s8(
        cols_67_s8,
        vreinterpretq_s8_u8(vdupq_n_u8(CENTERJSAMPLE as u8)),
    ));

    /* Transpose block to prepare for store. */
    let cols_0415 = vzipq_u32(vreinterpretq_u32_u8(cols_01), vreinterpretq_u32_u8(cols_45));
    let cols_2637 = vzipq_u32(vreinterpretq_u32_u8(cols_23), vreinterpretq_u32_u8(cols_67));

    let cols_0145 = vtrnq_u8(
        vreinterpretq_u8_u32(cols_0415.0),
        vreinterpretq_u8_u32(cols_0415.1),
    );
    let cols_2367 = vtrnq_u8(
        vreinterpretq_u8_u32(cols_2637.0),
        vreinterpretq_u8_u32(cols_2637.1),
    );
    let rows_0426 = vtrnq_u16(
        vreinterpretq_u16_u8(cols_0145.0),
        vreinterpretq_u16_u8(cols_2367.0),
    );
    let rows_1537 = vtrnq_u16(
        vreinterpretq_u16_u8(cols_0145.1),
        vreinterpretq_u16_u8(cols_2367.1),
    );

    let rows_04 = vreinterpretq_u8_u16(rows_0426.0);
    let rows_15 = vreinterpretq_u8_u16(rows_1537.0);
    let rows_26 = vreinterpretq_u8_u16(rows_0426.1);
    let rows_37 = vreinterpretq_u8_u16(rows_1537.1);

    /* Store DCT block to memory. */
    let outptr0 = &mut output_buf.row_mut(0)[output_col as usize..];
    vst1q_lane_u64::<0>(
        outptr0.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_04),
    );
    let outptr1 = &mut output_buf.row_mut(1)[output_col as usize..];
    vst1q_lane_u64::<0>(
        outptr1.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_15),
    );
    let outptr2 = &mut output_buf.row_mut(2)[output_col as usize..];
    vst1q_lane_u64::<0>(
        outptr2.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_26),
    );
    let outptr3 = &mut output_buf.row_mut(3)[output_col as usize..];
    vst1q_lane_u64::<0>(
        outptr3.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_37),
    );
    let outptr4 = &mut output_buf.row_mut(4)[output_col as usize..];
    vst1q_lane_u64::<1>(
        outptr4.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_04),
    );
    let outptr5 = &mut output_buf.row_mut(5)[output_col as usize..];
    vst1q_lane_u64::<1>(
        outptr5.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_15),
    );
    let outptr6 = &mut output_buf.row_mut(6)[output_col as usize..];
    vst1q_lane_u64::<1>(
        outptr6.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_26),
    );
    let outptr7 = &mut output_buf.row_mut(7)[output_col as usize..];
    vst1q_lane_u64::<1>(
        outptr7.first_chunk_mut().expect(OUTPUT),
        vreinterpretq_u64_u8(rows_37),
    );
}
