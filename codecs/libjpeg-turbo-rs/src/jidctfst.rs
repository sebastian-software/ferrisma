// libjpeg-turbo 3.2.1, src/jidctfst.c:
//
// /*
//  * jidctfst.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1998, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015, 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains a fast, not so accurate integer implementation of the
//  * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
//  * must also perform dequantization of the input coefficients.
//  *
//  * A 2-D IDCT can be done by 1-D IDCT on each column followed by 1-D IDCT
//  * on each row (or vice versa, but it's more convenient to emit a row at
//  * a time).  Direct algorithms are also available, but they are much more
//  * complex and seem not to be any faster when reduced to code.
//  *
//  * This implementation is based on Arai, Agui, and Nakajima's algorithm for
//  * scaled DCT.  Their original paper (Trans. IEICE E-71(11):1095) is in
//  * Japanese, but the algorithm is described in the Pennebaker & Mitchell
//  * JPEG textbook (see REFERENCES section in file README.ijg).  The following
//  * code is based directly on figure 4-8 in P&M.
//  * While an 8-point DCT cannot be done in less than 11 multiplies, it is
//  * possible to arrange the computation so that many of the multiplies are
//  * simple scalings of the final outputs.  These multiplies can then be
//  * folded into the multiplications or divisions by the JPEG quantization
//  * table entries.  The AA&N method leaves only 5 multiplies and 29 adds
//  * to be done in the DCT itself.
//  * The primary disadvantage of this method is that with fixed-point math,
//  * accuracy is lost due to imprecise representation of the scaled
//  * quantization values.  The smaller the quantization table entry, the less
//  * precise the scaled value, so this implementation does worse with high-
//  * quality-setting files than with low-quality ones.
//  */
//! Translation of `src/jidctfst.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
//!
//! The 8-bit configuration applies: `CONST_BITS` 8 and `PASS1_BITS` 2.
//! `USE_ACCURATE_ROUNDING` is undefined, so this file's `DESCALE` is
//! `RIGHT_SHIFT`, which truncates, and `MULTIPLY` descales with it.
//! `RIGHT_SHIFT_IS_UNSIGNED` is undefined, so `IRIGHT_SHIFT` is `x >> shft`,
//! and `ISHIFT_TEMPS` and `SCALING_FACTOR` declare nothing. `DCTELEM` is
//! `int`, or `short` with `WITH_SIMD` (ADR 0007), so the function evaluates
//! every expression in `int`, as C does, and converts it to `DCTELEM` where
//! C assigns or casts it (`dctelem`). `NO_ZERO_ROW_TEST` is
//! undefined, so the zero-row tests are kept. The range limit comes from
//! `IDCT_range_limit`. Pointer walks become indexes.

use crate::jdct::{DCTELEM, IDCT_range_limit, IFAST_MULT_TYPE, RANGE_MASK, promote_multiplier};
use crate::jmorecfg::{JCOEF, JDIMENSION, JSAMPLE};
use crate::jpegint::{JLONG, RIGHT_SHIFT};
use crate::jpeglib::{
    DCTSIZE, DCTSIZE2, JBLOCK, JSAMPROWS_MUT, jpeg_component_info, jpeg_decompress_struct,
};

// upstream: src/jidctfst.c CONST_BITS
const CONST_BITS: u32 = 8;
// upstream: src/jidctfst.c PASS1_BITS
const PASS1_BITS: u32 = 2;

// upstream: src/jidctfst.c FIX_1_082392200
/// `FIX(1.082392200)` with `CONST_BITS` 8.
const FIX_1_082392200: JLONG = 277;
// upstream: src/jidctfst.c FIX_1_414213562
/// `FIX(1.414213562)` with `CONST_BITS` 8.
const FIX_1_414213562: JLONG = 362;
// upstream: src/jidctfst.c FIX_1_847759065
/// `FIX(1.847759065)` with `CONST_BITS` 8.
const FIX_1_847759065: JLONG = 473;
// upstream: src/jidctfst.c FIX_2_613125930
/// `FIX(2.613125930)` with `CONST_BITS` 8.
const FIX_2_613125930: JLONG = 669;

// upstream: src/jidctfst.c DESCALE
/// `RIGHT_SHIFT(x, n)`: `USE_ACCURATE_ROUNDING` is undefined in this file,
/// so the descale truncates instead of rounding.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn DESCALE(x: JLONG, n: u32) -> JLONG {
    RIGHT_SHIFT(x, n)
}

/// C's conversion of an `int` expression to `DCTELEM` when it is assigned
/// or cast: the identity without `WITH_SIMD`, a truncation to 16 bits with
/// it. Not an upstream item.
#[inline]
fn dctelem(x: i32) -> DCTELEM {
    x as DCTELEM
}

/// C's integer promotion of a `DCTELEM` operand to `int`. Not an upstream
/// item.
#[cfg_attr(
    not(with_simd),
    expect(
        clippy::useless_conversion,
        reason = "DCTELEM is int without WITH_SIMD"
    )
)]
#[inline]
fn int(x: DCTELEM) -> i32 {
    i32::from(x)
}

// upstream: src/jidctfst.c MULTIPLY
/// Multiplies a `DCTELEM` variable by a `JLONG` constant, descales the
/// product by `CONST_BITS`, and yields a `DCTELEM` result. `var` is the
/// operand expression, which C evaluates in `int`.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn MULTIPLY(var: i32, const_: JLONG) -> DCTELEM {
    DESCALE(JLONG::from(var) * const_, CONST_BITS) as DCTELEM
}

// upstream: src/jidctfst.c DEQUANTIZE
/// Dequantizes a coefficient by multiplying it by the multiplier-table
/// entry: the `int` product, which the caller converts to `DCTELEM`.
/// `(IFAST_MULT_TYPE)coef` is `coef` in both widths of `IFAST_MULT_TYPE`.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn DEQUANTIZE(coef: JCOEF, quantval: IFAST_MULT_TYPE) -> i32 {
    i32::from(coef) * promote_multiplier(quantval)
}

// upstream: src/jidctfst.c IRIGHT_SHIFT
/// `((x) >> (shft))`: the arithmetic shift, because
/// `RIGHT_SHIFT_IS_UNSIGNED` is undefined in this file. Its operands are
/// `int` expressions.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn IRIGHT_SHIFT(x: i32, shft: u32) -> i32 {
    x >> shft
}

// upstream: src/jidctfst.c IDESCALE
/// `((int)IRIGHT_SHIFT(x, n))`, which truncates because
/// `USE_ACCURATE_ROUNDING` is undefined.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn IDESCALE(x: i32, n: u32) -> i32 {
    IRIGHT_SHIFT(x, n)
}

/// `range_limit[(int)x & RANGE_MASK]`: the sample for a descaled IDCT
/// output. Not an upstream item.
#[expect(clippy::cast_sign_loss, reason = "the masked index is nonnegative")]
#[inline]
fn range_limit_sample(range_limit: &[JSAMPLE], x: i32) -> JSAMPLE {
    range_limit[(x & RANGE_MASK) as usize]
}

// upstream: src/jidctfst.c _jpeg_idct_ifast
/// Performs dequantization and inverse DCT on one block of coefficients.
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 8 rows or 8 columns from `output_col` on.
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(clippy::similar_names, reason = "C names tmp10, tmp12 and so on")]
#[expect(clippy::identity_op, reason = "C's DCTSIZE * k indexing")]
#[expect(clippy::erasing_op, reason = "C's DCTSIZE * k indexing")]
pub fn jpeg_idct_ifast(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: DCTELEM;
    let mut tmp1: DCTELEM;
    let mut tmp2: DCTELEM;
    let mut tmp3: DCTELEM;
    let mut tmp4: DCTELEM;
    let mut tmp5: DCTELEM;
    let mut tmp6: DCTELEM;
    let mut tmp7: DCTELEM;
    let mut tmp10: DCTELEM;
    let mut tmp11: DCTELEM;
    let mut tmp12: DCTELEM;
    let mut tmp13: DCTELEM;
    let mut z5: DCTELEM;
    let mut z10: DCTELEM;
    let mut z11: DCTELEM;
    let mut z12: DCTELEM;
    let mut z13: DCTELEM;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; DCTSIZE2]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL")
        .ifast_array();
    let wsptr = &mut workspace;
    for col in 0..DCTSIZE {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */

        if inptr[DCTSIZE * 1 + col] == 0
            && inptr[DCTSIZE * 2 + col] == 0
            && inptr[DCTSIZE * 3 + col] == 0
            && inptr[DCTSIZE * 4 + col] == 0
            && inptr[DCTSIZE * 5 + col] == 0
            && inptr[DCTSIZE * 6 + col] == 0
            && inptr[DCTSIZE * 7 + col] == 0
        {
            /* AC terms all zero */
            let dcval = DEQUANTIZE(inptr[DCTSIZE * 0 + col], quantptr[DCTSIZE * 0 + col]);

            wsptr[DCTSIZE * 0 + col] = dcval;
            wsptr[DCTSIZE * 1 + col] = dcval;
            wsptr[DCTSIZE * 2 + col] = dcval;
            wsptr[DCTSIZE * 3 + col] = dcval;
            wsptr[DCTSIZE * 4 + col] = dcval;
            wsptr[DCTSIZE * 5 + col] = dcval;
            wsptr[DCTSIZE * 6 + col] = dcval;
            wsptr[DCTSIZE * 7 + col] = dcval;

            continue; /* advance pointers to next column */
        }

        /* Even part */

        tmp0 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 0 + col],
            quantptr[DCTSIZE * 0 + col],
        ));
        tmp1 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 2 + col],
            quantptr[DCTSIZE * 2 + col],
        ));
        tmp2 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 4 + col],
            quantptr[DCTSIZE * 4 + col],
        ));
        tmp3 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 6 + col],
            quantptr[DCTSIZE * 6 + col],
        ));

        tmp10 = dctelem(int(tmp0) + int(tmp2)); /* phase 3 */
        tmp11 = dctelem(int(tmp0) - int(tmp2));

        tmp13 = dctelem(int(tmp1) + int(tmp3)); /* phases 5-3 */
        tmp12 = dctelem(int(MULTIPLY(int(tmp1) - int(tmp3), FIX_1_414213562)) - int(tmp13)); /* 2*c4 */

        tmp0 = dctelem(int(tmp10) + int(tmp13)); /* phase 2 */
        tmp3 = dctelem(int(tmp10) - int(tmp13));
        tmp1 = dctelem(int(tmp11) + int(tmp12));
        tmp2 = dctelem(int(tmp11) - int(tmp12));

        /* Odd part */

        tmp4 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 1 + col],
            quantptr[DCTSIZE * 1 + col],
        ));
        tmp5 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 3 + col],
            quantptr[DCTSIZE * 3 + col],
        ));
        tmp6 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 5 + col],
            quantptr[DCTSIZE * 5 + col],
        ));
        tmp7 = dctelem(DEQUANTIZE(
            inptr[DCTSIZE * 7 + col],
            quantptr[DCTSIZE * 7 + col],
        ));

        z13 = dctelem(int(tmp6) + int(tmp5)); /* phase 6 */
        z10 = dctelem(int(tmp6) - int(tmp5));
        z11 = dctelem(int(tmp4) + int(tmp7));
        z12 = dctelem(int(tmp4) - int(tmp7));

        tmp7 = dctelem(int(z11) + int(z13)); /* phase 5 */
        tmp11 = MULTIPLY(int(z11) - int(z13), FIX_1_414213562); /* 2*c4 */

        z5 = MULTIPLY(int(z10) + int(z12), FIX_1_847759065); /* 2*c2 */
        tmp10 = dctelem(int(MULTIPLY(int(z12), FIX_1_082392200)) - int(z5)); /* 2*(c2-c6) */
        tmp12 = dctelem(int(MULTIPLY(int(z10), -FIX_2_613125930)) + int(z5)); /* -2*(c2+c6) */

        tmp6 = dctelem(int(tmp12) - int(tmp7)); /* phase 2 */
        tmp5 = dctelem(int(tmp11) - int(tmp6));
        tmp4 = dctelem(int(tmp10) + int(tmp5));

        wsptr[DCTSIZE * 0 + col] = int(tmp0) + int(tmp7);
        wsptr[DCTSIZE * 7 + col] = int(tmp0) - int(tmp7);
        wsptr[DCTSIZE * 1 + col] = int(tmp1) + int(tmp6);
        wsptr[DCTSIZE * 6 + col] = int(tmp1) - int(tmp6);
        wsptr[DCTSIZE * 2 + col] = int(tmp2) + int(tmp5);
        wsptr[DCTSIZE * 5 + col] = int(tmp2) - int(tmp5);
        wsptr[DCTSIZE * 4 + col] = int(tmp3) + int(tmp4);
        wsptr[DCTSIZE * 3 + col] = int(tmp3) - int(tmp4);
    }

    /* Pass 2: process rows from work array, store into output array. */
    /* Note that we must descale the results by a factor of 8 == 2**3, */
    /* and also undo the PASS1_BITS scaling. */

    for ctr in 0..DCTSIZE {
        let wsptr = &workspace[ctr * DCTSIZE..(ctr + 1) * DCTSIZE];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];
        /* Rows of zeroes can be exploited in the same way as we did with columns.
         * However, the column calculation has created many nonzero AC terms, so
         * the simplification applies less often (typically 5% to 10% of the time).
         * On machines with very fast multiplication, it's possible that the
         * test takes more time than it's worth.  In that case this section
         * may be commented out.
         */

        /* NO_ZERO_ROW_TEST is undefined, so the test is compiled in. */
        if wsptr[1] == 0
            && wsptr[2] == 0
            && wsptr[3] == 0
            && wsptr[4] == 0
            && wsptr[5] == 0
            && wsptr[6] == 0
            && wsptr[7] == 0
        {
            /* AC terms all zero */
            let dcval = range_limit_sample(range_limit, IDESCALE(wsptr[0], PASS1_BITS + 3));

            outptr[0] = dcval;
            outptr[1] = dcval;
            outptr[2] = dcval;
            outptr[3] = dcval;
            outptr[4] = dcval;
            outptr[5] = dcval;
            outptr[6] = dcval;
            outptr[7] = dcval;

            continue; /* advance pointer to next row */
        }

        /* Even part */

        tmp10 = dctelem(int(dctelem(wsptr[0])) + int(dctelem(wsptr[4])));
        tmp11 = dctelem(int(dctelem(wsptr[0])) - int(dctelem(wsptr[4])));

        tmp13 = dctelem(int(dctelem(wsptr[2])) + int(dctelem(wsptr[6])));
        tmp12 = dctelem(
            int(MULTIPLY(
                int(dctelem(wsptr[2])) - int(dctelem(wsptr[6])),
                FIX_1_414213562,
            )) - int(tmp13),
        );

        tmp0 = dctelem(int(tmp10) + int(tmp13));
        tmp3 = dctelem(int(tmp10) - int(tmp13));
        tmp1 = dctelem(int(tmp11) + int(tmp12));
        tmp2 = dctelem(int(tmp11) - int(tmp12));

        /* Odd part */

        z13 = dctelem(int(dctelem(wsptr[5])) + int(dctelem(wsptr[3])));
        z10 = dctelem(int(dctelem(wsptr[5])) - int(dctelem(wsptr[3])));
        z11 = dctelem(int(dctelem(wsptr[1])) + int(dctelem(wsptr[7])));
        z12 = dctelem(int(dctelem(wsptr[1])) - int(dctelem(wsptr[7])));

        tmp7 = dctelem(int(z11) + int(z13)); /* phase 5 */
        tmp11 = MULTIPLY(int(z11) - int(z13), FIX_1_414213562); /* 2*c4 */

        z5 = MULTIPLY(int(z10) + int(z12), FIX_1_847759065); /* 2*c2 */
        tmp10 = dctelem(int(MULTIPLY(int(z12), FIX_1_082392200)) - int(z5)); /* 2*(c2-c6) */
        tmp12 = dctelem(int(MULTIPLY(int(z10), -FIX_2_613125930)) + int(z5)); /* -2*(c2+c6) */

        tmp6 = dctelem(int(tmp12) - int(tmp7)); /* phase 2 */
        tmp5 = dctelem(int(tmp11) - int(tmp6));
        tmp4 = dctelem(int(tmp10) + int(tmp5));

        /* Final output stage: scale down by a factor of 8 and range-limit */

        outptr[0] =
            range_limit_sample(range_limit, IDESCALE(int(tmp0) + int(tmp7), PASS1_BITS + 3));
        outptr[7] =
            range_limit_sample(range_limit, IDESCALE(int(tmp0) - int(tmp7), PASS1_BITS + 3));
        outptr[1] =
            range_limit_sample(range_limit, IDESCALE(int(tmp1) + int(tmp6), PASS1_BITS + 3));
        outptr[6] =
            range_limit_sample(range_limit, IDESCALE(int(tmp1) - int(tmp6), PASS1_BITS + 3));
        outptr[2] =
            range_limit_sample(range_limit, IDESCALE(int(tmp2) + int(tmp5), PASS1_BITS + 3));
        outptr[5] =
            range_limit_sample(range_limit, IDESCALE(int(tmp2) - int(tmp5), PASS1_BITS + 3));
        outptr[4] =
            range_limit_sample(range_limit, IDESCALE(int(tmp3) + int(tmp4), PASS1_BITS + 3));
        outptr[3] =
            range_limit_sample(range_limit, IDESCALE(int(tmp3) - int(tmp4), PASS1_BITS + 3));

        /* advance pointer to next row */
    }
}
