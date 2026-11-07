// libjpeg-turbo 3.2.1, src/jidctint.c:
//
// /*
//  * jidctint.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1998, Thomas G. Lane.
//  * Modification developed 2002-2018 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015, 2020, 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains a slower but more accurate integer implementation of the
//  * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
//  * must also perform dequantization of the input coefficients.
//  *
//  * A 2-D IDCT can be done by 1-D IDCT on each column followed by 1-D IDCT
//  * on each row (or vice versa, but it's more convenient to emit a row at
//  * a time).  Direct algorithms are also available, but they are much more
//  * complex and seem not to be any faster when reduced to code.
//  *
//  * This implementation is based on an algorithm described in
//  *   C. Loeffler, A. Ligtenberg and G. Moschytz, "Practical Fast 1-D DCT
//  *   Algorithms with 11 Multiplications", Proc. Int'l. Conf. on Acoustics,
//  *   Speech, and Signal Processing 1989 (ICASSP '89), pp. 988-991.
//  * The primary algorithm described there uses 11 multiplies and 29 adds.
//  * We use their alternate method with 12 multiplies and 32 adds.
//  * The advantage of this method is that no data path contains more than one
//  * multiplication; this allows a very simple and accurate implementation in
//  * scaled fixed-point arithmetic, with a minimal number of shifts.
//  *
//  * We also provide IDCT routines with various output sample block sizes for
//  * direct resolution reduction or enlargement without additional resampling:
//  * NxN (N=1...16) samples for one 8x8 input DCT block.
//  *
//  * For N<8 we simply take the corresponding low-frequency coefficients of
//  * the 8x8 input DCT block and apply an NxN point IDCT on the sub-block
//  * to yield the downscaled outputs.
//  * This can be seen as direct low-pass downsampling from the DCT domain
//  * point of view rather than the usual spatial domain point of view,
//  * yielding significant computational savings and results at least
//  * as good as common bilinear (averaging) spatial downsampling.
//  *
//  * For N>8 we apply a partial NxN IDCT on the 8 input coefficients as
//  * lower frequencies and higher frequencies assumed to be zero.
//  * It turns out that the computational effort is similar to the 8x8 IDCT
//  * regarding the output size.
//  * Furthermore, the scaling and descaling is the same for all IDCT sizes.
//  *
//  * CAUTION: We rely on the FIX() macro except for the N=1,2,4,8 cases
//  * since there would be too many additional constants to pre-calculate.
//  */
//! Translation of `src/jidctint.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
//!
//! The 8-bit configuration applies: `CONST_BITS` 13, `PASS1_BITS` 2, and
//! `MULTIPLY` is `MULTIPLY16C16`. `NO_ZERO_ROW_TEST` is undefined. `SCALING_FACTOR`
//! declares nothing for 8-bit samples. Pointer walks over the coefficient
//! block, the multiplier table and the workspace become indexes. A
//! conversion to `int` truncates, as `as i32` does.

use crate::jdct::{DESCALE, IDCT_range_limit, ISLOW_MULT_TYPE, MULTIPLY16C16, ONE, RANGE_MASK};
use crate::jmorecfg::{JCOEF, JDIMENSION, JSAMPLE};
use crate::jpegint::{JLONG, LEFT_SHIFT, RIGHT_SHIFT};
use crate::jpeglib::{
    DCTSIZE, DCTSIZE2, JBLOCK, JSAMPROWS_MUT, jpeg_component_info, jpeg_decompress_struct,
};

// upstream: src/jidctint.c CONST_BITS
const CONST_BITS: u32 = 13;
// upstream: src/jidctint.c PASS1_BITS
const PASS1_BITS: u32 = 2;

// upstream: src/jidctint.c FIX_0_298631336
const FIX_0_298631336: JLONG = 2446; /* FIX(0.298_631_336) */
// upstream: src/jidctint.c FIX_0_390180644
const FIX_0_390180644: JLONG = 3196; /* FIX(0.390_180_644) */
// upstream: src/jidctint.c FIX_0_541196100
const FIX_0_541196100: JLONG = 4433; /* FIX(0.541_196_100) */
// upstream: src/jidctint.c FIX_0_765366865
const FIX_0_765366865: JLONG = 6270; /* FIX(0.765_366_865) */
// upstream: src/jidctint.c FIX_0_899976223
const FIX_0_899976223: JLONG = 7373; /* FIX(0.899_976_223) */
// upstream: src/jidctint.c FIX_1_175875602
const FIX_1_175875602: JLONG = 9633; /* FIX(1.175_875_602) */
// upstream: src/jidctint.c FIX_1_501321110
const FIX_1_501321110: JLONG = 12299; /* FIX(1.501_321_110) */
// upstream: src/jidctint.c FIX_1_847759065
const FIX_1_847759065: JLONG = 15137; /* FIX(1.847_759_065) */
// upstream: src/jidctint.c FIX_1_961570560
const FIX_1_961570560: JLONG = 16069; /* FIX(1.961_570_560) */
// upstream: src/jidctint.c FIX_2_053119869
const FIX_2_053119869: JLONG = 16819; /* FIX(2.053_119_869) */
// upstream: src/jidctint.c FIX_2_562915447
const FIX_2_562915447: JLONG = 20995; /* FIX(2.562_915_447) */
// upstream: src/jidctint.c FIX_3_072711026
const FIX_3_072711026: JLONG = 25172; /* FIX(3.072_711_026) */

// upstream: src/jdct.h FIX
/// `FIX(x)` with this file's `CONST_BITS`.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
const fn FIX(x: f64) -> JLONG {
    crate::jdct::FIX(x, CONST_BITS)
}

// upstream: src/jidctint.c MULTIPLY
/// Multiplies a `JLONG` variable by a `JLONG` constant to yield a `JLONG`
/// result.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn MULTIPLY(var: JLONG, const_: JLONG) -> JLONG {
    MULTIPLY16C16(var, const_)
}

// upstream: src/jidctint.c DEQUANTIZE
/// Dequantizes a coefficient by multiplying it by the multiplier-table
/// entry; produces an `int` result, widened to `JLONG` as every use assigns
/// it.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn DEQUANTIZE(coef: JCOEF, quantval: ISLOW_MULT_TYPE) -> JLONG {
    JLONG::from(ISLOW_MULT_TYPE::from(coef) * quantval)
}

/// `range_limit[(int)x & RANGE_MASK]`: the sample for a descaled IDCT
/// output. Not an upstream item.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversion to int, then a nonnegative masked index"
)]
#[inline]
fn range_limit_sample(range_limit: &[JSAMPLE], x: JLONG) -> JSAMPLE {
    range_limit[((x as i32) & RANGE_MASK) as usize]
}

/// The multiplier table of `compptr`, as `(ISLOW_MULT_TYPE *)compptr->dct_table`
/// reads it. Not an upstream item.
///
/// # Panics
///
/// When the component has no multiplier table.
#[inline]
pub(crate) fn islow_table(compptr: &jpeg_component_info) -> &[ISLOW_MULT_TYPE; DCTSIZE2] {
    compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL")
        .islow_array()
}

// upstream: src/jidctint.c _jpeg_idct_islow
/// Performs dequantization and inverse DCT on one block of coefficients.
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 8 rows or columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
pub fn jpeg_idct_islow(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp1: JLONG;
    let mut tmp2: JLONG;
    let mut tmp3: JLONG;
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let mut z5: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; DCTSIZE2]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */
    /* Note results are scaled up by sqrt(8) compared to a true IDCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
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
            let dcval = LEFT_SHIFT(
                DEQUANTIZE(inptr[DCTSIZE * 0 + col], quantptr[DCTSIZE * 0 + col]),
                PASS1_BITS,
            ) as i32;

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

        /* Even part: reverse the even part of the forward DCT. */
        /* The rotator is sqrt(2)*c(-6). */

        z2 = DEQUANTIZE(inptr[DCTSIZE * 2 + col], quantptr[DCTSIZE * 2 + col]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 6 + col], quantptr[DCTSIZE * 6 + col]);

        z1 = MULTIPLY(z2 + z3, FIX_0_541196100);
        tmp2 = z1 + MULTIPLY(z3, -FIX_1_847759065);
        tmp3 = z1 + MULTIPLY(z2, FIX_0_765366865);

        z2 = DEQUANTIZE(inptr[DCTSIZE * 0 + col], quantptr[DCTSIZE * 0 + col]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 4 + col], quantptr[DCTSIZE * 4 + col]);

        tmp0 = LEFT_SHIFT(z2 + z3, CONST_BITS);
        tmp1 = LEFT_SHIFT(z2 - z3, CONST_BITS);

        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;

        /* Odd part per figure 8; the matrix is unitary and hence its
         * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
         */

        tmp0 = DEQUANTIZE(inptr[DCTSIZE * 7 + col], quantptr[DCTSIZE * 7 + col]);
        tmp1 = DEQUANTIZE(inptr[DCTSIZE * 5 + col], quantptr[DCTSIZE * 5 + col]);
        tmp2 = DEQUANTIZE(inptr[DCTSIZE * 3 + col], quantptr[DCTSIZE * 3 + col]);
        tmp3 = DEQUANTIZE(inptr[DCTSIZE * 1 + col], quantptr[DCTSIZE * 1 + col]);

        z1 = tmp0 + tmp3;
        z2 = tmp1 + tmp2;
        z3 = tmp0 + tmp2;
        z4 = tmp1 + tmp3;
        z5 = MULTIPLY(z3 + z4, FIX_1_175875602); /* sqrt(2) * c3 */

        tmp0 = MULTIPLY(tmp0, FIX_0_298631336); /* sqrt(2) * (-c1+c3+c5-c7) */
        tmp1 = MULTIPLY(tmp1, FIX_2_053119869); /* sqrt(2) * ( c1+c3-c5+c7) */
        tmp2 = MULTIPLY(tmp2, FIX_3_072711026); /* sqrt(2) * ( c1+c3+c5-c7) */
        tmp3 = MULTIPLY(tmp3, FIX_1_501321110); /* sqrt(2) * ( c1+c3-c5-c7) */
        z1 = MULTIPLY(z1, -FIX_0_899976223); /* sqrt(2) * ( c7-c3) */
        z2 = MULTIPLY(z2, -FIX_2_562915447); /* sqrt(2) * (-c1-c3) */
        z3 = MULTIPLY(z3, -FIX_1_961570560); /* sqrt(2) * (-c3-c5) */
        z4 = MULTIPLY(z4, -FIX_0_390180644); /* sqrt(2) * ( c5-c3) */

        z3 += z5;
        z4 += z5;

        tmp0 += z1 + z3;
        tmp1 += z2 + z4;
        tmp2 += z2 + z3;
        tmp3 += z1 + z4;

        /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */

        wsptr[DCTSIZE * 0 + col] = DESCALE(tmp10 + tmp3, CONST_BITS - PASS1_BITS) as i32;
        wsptr[DCTSIZE * 7 + col] = DESCALE(tmp10 - tmp3, CONST_BITS - PASS1_BITS) as i32;
        wsptr[DCTSIZE * 1 + col] = DESCALE(tmp11 + tmp2, CONST_BITS - PASS1_BITS) as i32;
        wsptr[DCTSIZE * 6 + col] = DESCALE(tmp11 - tmp2, CONST_BITS - PASS1_BITS) as i32;
        wsptr[DCTSIZE * 2 + col] = DESCALE(tmp12 + tmp1, CONST_BITS - PASS1_BITS) as i32;
        wsptr[DCTSIZE * 5 + col] = DESCALE(tmp12 - tmp1, CONST_BITS - PASS1_BITS) as i32;
        wsptr[DCTSIZE * 3 + col] = DESCALE(tmp13 + tmp0, CONST_BITS - PASS1_BITS) as i32;
        wsptr[DCTSIZE * 4 + col] = DESCALE(tmp13 - tmp0, CONST_BITS - PASS1_BITS) as i32;
        /* advance pointers to next column */
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

        if wsptr[1] == 0
            && wsptr[2] == 0
            && wsptr[3] == 0
            && wsptr[4] == 0
            && wsptr[5] == 0
            && wsptr[6] == 0
            && wsptr[7] == 0
        {
            /* AC terms all zero */
            let dcval =
                range_limit_sample(range_limit, DESCALE(JLONG::from(wsptr[0]), PASS1_BITS + 3));

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

        /* Even part: reverse the even part of the forward DCT. */
        /* The rotator is sqrt(2)*c(-6). */

        z2 = JLONG::from(wsptr[2]);
        z3 = JLONG::from(wsptr[6]);

        z1 = MULTIPLY(z2 + z3, FIX_0_541196100);
        tmp2 = z1 + MULTIPLY(z3, -FIX_1_847759065);
        tmp3 = z1 + MULTIPLY(z2, FIX_0_765366865);

        tmp0 = LEFT_SHIFT(JLONG::from(wsptr[0]) + JLONG::from(wsptr[4]), CONST_BITS);
        tmp1 = LEFT_SHIFT(JLONG::from(wsptr[0]) - JLONG::from(wsptr[4]), CONST_BITS);

        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;

        /* Odd part per figure 8; the matrix is unitary and hence its
         * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
         */

        tmp0 = JLONG::from(wsptr[7]);
        tmp1 = JLONG::from(wsptr[5]);
        tmp2 = JLONG::from(wsptr[3]);
        tmp3 = JLONG::from(wsptr[1]);

        z1 = tmp0 + tmp3;
        z2 = tmp1 + tmp2;
        z3 = tmp0 + tmp2;
        z4 = tmp1 + tmp3;
        z5 = MULTIPLY(z3 + z4, FIX_1_175875602); /* sqrt(2) * c3 */

        tmp0 = MULTIPLY(tmp0, FIX_0_298631336); /* sqrt(2) * (-c1+c3+c5-c7) */
        tmp1 = MULTIPLY(tmp1, FIX_2_053119869); /* sqrt(2) * ( c1+c3-c5+c7) */
        tmp2 = MULTIPLY(tmp2, FIX_3_072711026); /* sqrt(2) * ( c1+c3+c5-c7) */
        tmp3 = MULTIPLY(tmp3, FIX_1_501321110); /* sqrt(2) * ( c1+c3-c5-c7) */
        z1 = MULTIPLY(z1, -FIX_0_899976223); /* sqrt(2) * ( c7-c3) */
        z2 = MULTIPLY(z2, -FIX_2_562915447); /* sqrt(2) * (-c1-c3) */
        z3 = MULTIPLY(z3, -FIX_1_961570560); /* sqrt(2) * (-c3-c5) */
        z4 = MULTIPLY(z4, -FIX_0_390180644); /* sqrt(2) * ( c5-c3) */

        z3 += z5;
        z4 += z5;

        tmp0 += z1 + z3;
        tmp1 += z2 + z4;
        tmp2 += z2 + z3;
        tmp3 += z1 + z4;

        /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */

        outptr[0] = range_limit_sample(
            range_limit,
            DESCALE(tmp10 + tmp3, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            DESCALE(tmp10 - tmp3, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            DESCALE(tmp11 + tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            DESCALE(tmp11 - tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            DESCALE(tmp12 + tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            DESCALE(tmp12 - tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            DESCALE(tmp13 + tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            DESCALE(tmp13 - tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        /* advance pointer to next row */
    }
}

// upstream: src/jidctint.c _jpeg_idct_7x7
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 7x7 output block.
///
/// Optimized algorithm with 12 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/14).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 7 rows, or fewer than 7 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::approx_constant,
    reason = "FIX() takes C decimal constants that approximate sqrt(2)"
)]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_7x7(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp1: JLONG;
    let mut tmp2: JLONG;
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 7 * 7]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..7 {
        /* Even part */

        tmp13 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        tmp13 = LEFT_SHIFT(tmp13, CONST_BITS);
        /* Add fudge factor here for final descale. */
        tmp13 += ONE << (CONST_BITS - PASS1_BITS - 1);

        z1 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);

        tmp10 = MULTIPLY(z2 - z3, FIX(0.881_747_734)); /* c4 */
        tmp12 = MULTIPLY(z1 - z2, FIX(0.314_692_123)); /* c6 */
        tmp11 = tmp10 + tmp12 + tmp13 - MULTIPLY(z2, FIX(1.841_218_003)); /* c2+c4-c6 */
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = MULTIPLY(tmp0, FIX(1.274_162_392)) + tmp13; /* c2 */
        tmp10 += tmp0 - MULTIPLY(z3, FIX(0.077_722_536)); /* c2-c4-c6 */
        tmp12 += tmp0 - MULTIPLY(z1, FIX(2.470_602_249)); /* c2+c4+c6 */
        tmp13 += MULTIPLY(z2, FIX(1.414_213_562)); /* c0 */

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);

        tmp1 = MULTIPLY(z1 + z2, FIX(0.935_414_347)); /* (c3+c1-c5)/2 */
        tmp2 = MULTIPLY(z1 - z2, FIX(0.170_262_339)); /* (c3+c5-c1)/2 */
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = MULTIPLY(z2 + z3, -FIX(1.378_756_276)); /* -c1 */
        tmp1 += tmp2;
        z2 = MULTIPLY(z1 + z3, FIX(0.613_604_268)); /* c5 */
        tmp0 += z2;
        tmp2 += z2 + MULTIPLY(z3, FIX(1.870_828_693)); /* c3+c1-c5 */

        /* Final output stage */

        workspace[7 * 0 + ctr] = RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[7 * 6 + ctr] = RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[7 * 1 + ctr] = RIGHT_SHIFT(tmp11 + tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[7 * 5 + ctr] = RIGHT_SHIFT(tmp11 - tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[7 * 2 + ctr] = RIGHT_SHIFT(tmp12 + tmp2, CONST_BITS - PASS1_BITS) as i32;
        workspace[7 * 4 + ctr] = RIGHT_SHIFT(tmp12 - tmp2, CONST_BITS - PASS1_BITS) as i32;
        workspace[7 * 3 + ctr] = RIGHT_SHIFT(tmp13, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 7 rows from work array, store into output array. */

    for ctr in 0..7 {
        let wsptr = &workspace[ctr * 7..(ctr + 1) * 7];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        tmp13 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        tmp13 = LEFT_SHIFT(tmp13, CONST_BITS);

        z1 = JLONG::from(wsptr[2]);
        z2 = JLONG::from(wsptr[4]);
        z3 = JLONG::from(wsptr[6]);

        tmp10 = MULTIPLY(z2 - z3, FIX(0.881_747_734)); /* c4 */
        tmp12 = MULTIPLY(z1 - z2, FIX(0.314_692_123)); /* c6 */
        tmp11 = tmp10 + tmp12 + tmp13 - MULTIPLY(z2, FIX(1.841_218_003)); /* c2+c4-c6 */
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = MULTIPLY(tmp0, FIX(1.274_162_392)) + tmp13; /* c2 */
        tmp10 += tmp0 - MULTIPLY(z3, FIX(0.077_722_536)); /* c2-c4-c6 */
        tmp12 += tmp0 - MULTIPLY(z1, FIX(2.470_602_249)); /* c2+c4+c6 */
        tmp13 += MULTIPLY(z2, FIX(1.414_213_562)); /* c0 */

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);

        tmp1 = MULTIPLY(z1 + z2, FIX(0.935_414_347)); /* (c3+c1-c5)/2 */
        tmp2 = MULTIPLY(z1 - z2, FIX(0.170_262_339)); /* (c3+c5-c1)/2 */
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = MULTIPLY(z2 + z3, -FIX(1.378_756_276)); /* -c1 */
        tmp1 += tmp2;
        z2 = MULTIPLY(z1 + z3, FIX(0.613_604_268)); /* c5 */
        tmp0 += z2;
        tmp2 += z2 + MULTIPLY(z3, FIX(1.870_828_693)); /* c3+c1-c5 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 + tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 - tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp12 + tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp12 - tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] =
            range_limit_sample(range_limit, RIGHT_SHIFT(tmp13, CONST_BITS + PASS1_BITS + 3));
    }
}

// upstream: src/jidctint.c _jpeg_idct_6x6
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 6x6 output block.
///
/// Optimized algorithm with 3 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/12).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 6 rows, or fewer than 6 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::approx_constant,
    reason = "FIX() takes C decimal constants that approximate sqrt(2)"
)]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::similar_names, reason = "C variable names")]
pub fn jpeg_idct_6x6(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp1: JLONG;
    let mut tmp2: JLONG;
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 6 * 6]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..6 {
        /* Even part */

        tmp0 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);
        /* Add fudge factor here for final descale. */
        tmp0 += ONE << (CONST_BITS - PASS1_BITS - 1);
        tmp2 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        tmp10 = MULTIPLY(tmp2, FIX(0.707_106_781)); /* c4 */
        tmp1 = tmp0 + tmp10;
        tmp11 = RIGHT_SHIFT(tmp0 - tmp10 - tmp10, CONST_BITS - PASS1_BITS);
        tmp10 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        tmp0 = MULTIPLY(tmp10, FIX(1.224_744_871)); /* c2 */
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        tmp1 = MULTIPLY(z1 + z3, FIX(0.366_025_404)); /* c5 */
        tmp0 = tmp1 + LEFT_SHIFT(z1 + z2, CONST_BITS);
        tmp2 = tmp1 + LEFT_SHIFT(z3 - z2, CONST_BITS);
        tmp1 = LEFT_SHIFT(z1 - z2 - z3, PASS1_BITS);

        /* Final output stage */

        workspace[6 * 0 + ctr] = RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[6 * 5 + ctr] = RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[6 * 1 + ctr] = (tmp11 + tmp1) as i32;
        workspace[6 * 4 + ctr] = (tmp11 - tmp1) as i32;
        workspace[6 * 2 + ctr] = RIGHT_SHIFT(tmp12 + tmp2, CONST_BITS - PASS1_BITS) as i32;
        workspace[6 * 3 + ctr] = RIGHT_SHIFT(tmp12 - tmp2, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 6 rows from work array, store into output array. */

    for ctr in 0..6 {
        let wsptr = &workspace[ctr * 6..(ctr + 1) * 6];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        tmp0 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);
        tmp2 = JLONG::from(wsptr[4]);
        tmp10 = MULTIPLY(tmp2, FIX(0.707_106_781)); /* c4 */
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10;
        tmp10 = JLONG::from(wsptr[2]);
        tmp0 = MULTIPLY(tmp10, FIX(1.224_744_871)); /* c2 */
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        tmp1 = MULTIPLY(z1 + z3, FIX(0.366_025_404)); /* c5 */
        tmp0 = tmp1 + LEFT_SHIFT(z1 + z2, CONST_BITS);
        tmp2 = tmp1 + LEFT_SHIFT(z3 - z2, CONST_BITS);
        tmp1 = LEFT_SHIFT(z1 - z2 - z3, CONST_BITS);

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 + tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 - tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp12 + tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp12 - tmp2, CONST_BITS + PASS1_BITS + 3),
        );
    }
}

// upstream: src/jidctint.c _jpeg_idct_5x5
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 5x5 output block.
///
/// Optimized algorithm with 5 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/10).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 5 rows, or fewer than 5 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::similar_names, reason = "C variable names")]
pub fn jpeg_idct_5x5(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp1: JLONG;
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 5 * 5]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..5 {
        /* Even part */

        tmp12 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        tmp12 = LEFT_SHIFT(tmp12, CONST_BITS);
        /* Add fudge factor here for final descale. */
        tmp12 += ONE << (CONST_BITS - PASS1_BITS - 1);
        tmp0 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        tmp1 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z1 = MULTIPLY(tmp0 + tmp1, FIX(0.790_569_415)); /* (c2+c4)/2 */
        z2 = MULTIPLY(tmp0 - tmp1, FIX(0.353_553_391)); /* (c2-c4)/2 */
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= LEFT_SHIFT(z2, 2);

        /* Odd part */

        z2 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);

        z1 = MULTIPLY(z2 + z3, FIX(0.831_253_876)); /* c3 */
        tmp0 = z1 + MULTIPLY(z2, FIX(0.513_743_148)); /* c1-c3 */
        tmp1 = z1 - MULTIPLY(z3, FIX(2.176_250_899)); /* c1+c3 */

        /* Final output stage */

        workspace[5 * 0 + ctr] = RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[5 * 4 + ctr] = RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[5 * 1 + ctr] = RIGHT_SHIFT(tmp11 + tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[5 * 3 + ctr] = RIGHT_SHIFT(tmp11 - tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[5 * 2 + ctr] = RIGHT_SHIFT(tmp12, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 5 rows from work array, store into output array. */

    for ctr in 0..5 {
        let wsptr = &workspace[ctr * 5..(ctr + 1) * 5];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        tmp12 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        tmp12 = LEFT_SHIFT(tmp12, CONST_BITS);
        tmp0 = JLONG::from(wsptr[2]);
        tmp1 = JLONG::from(wsptr[4]);
        z1 = MULTIPLY(tmp0 + tmp1, FIX(0.790_569_415)); /* (c2+c4)/2 */
        z2 = MULTIPLY(tmp0 - tmp1, FIX(0.353_553_391)); /* (c2-c4)/2 */
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= LEFT_SHIFT(z2, 2);

        /* Odd part */

        z2 = JLONG::from(wsptr[1]);
        z3 = JLONG::from(wsptr[3]);

        z1 = MULTIPLY(z2 + z3, FIX(0.831_253_876)); /* c3 */
        tmp0 = z1 + MULTIPLY(z2, FIX(0.513_743_148)); /* c1-c3 */
        tmp1 = z1 - MULTIPLY(z3, FIX(2.176_250_899)); /* c1+c3 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 + tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 - tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] =
            range_limit_sample(range_limit, RIGHT_SHIFT(tmp12, CONST_BITS + PASS1_BITS + 3));
    }
}

// upstream: src/jidctint.c _jpeg_idct_3x3
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 3x3 output block.
///
/// Optimized algorithm with 2 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/6).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 3 rows, or fewer than 3 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::approx_constant,
    reason = "FIX() takes C decimal constants that approximate sqrt(2)"
)]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::similar_names, reason = "C variable names")]
pub fn jpeg_idct_3x3(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp2: JLONG;
    let mut tmp10: JLONG;
    let mut tmp12: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 3 * 3]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..3 {
        /* Even part */

        tmp0 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);
        /* Add fudge factor here for final descale. */
        tmp0 += ONE << (CONST_BITS - PASS1_BITS - 1);
        tmp2 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        tmp12 = MULTIPLY(tmp2, FIX(0.707_106_781)); /* c2 */
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;

        /* Odd part */

        tmp12 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        tmp0 = MULTIPLY(tmp12, FIX(1.224_744_871)); /* c1 */

        /* Final output stage */

        workspace[3 * 0 + ctr] = RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[3 * 2 + ctr] = RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[3 * 1 + ctr] = RIGHT_SHIFT(tmp2, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 3 rows from work array, store into output array. */

    for ctr in 0..3 {
        let wsptr = &workspace[ctr * 3..(ctr + 1) * 3];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        tmp0 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);
        tmp2 = JLONG::from(wsptr[2]);
        tmp12 = MULTIPLY(tmp2, FIX(0.707_106_781)); /* c2 */
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;

        /* Odd part */

        tmp12 = JLONG::from(wsptr[1]);
        tmp0 = MULTIPLY(tmp12, FIX(1.224_744_871)); /* c1 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(range_limit, RIGHT_SHIFT(tmp2, CONST_BITS + PASS1_BITS + 3));
    }
}

// upstream: src/jidctint.c _jpeg_idct_9x9
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a 9x9 output block.
///
/// Optimized algorithm with 10 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/18).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 9 rows, or fewer than 9 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::approx_constant,
    reason = "FIX() takes C decimal constants that approximate sqrt(2)"
)]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_9x9(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp1: JLONG;
    let mut tmp2: JLONG;
    let mut tmp3: JLONG;
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp14: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 9]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        tmp0 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);
        /* Add fudge factor here for final descale. */
        tmp0 += ONE << (CONST_BITS - PASS1_BITS - 1);

        z1 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);

        tmp3 = MULTIPLY(z3, FIX(0.707_106_781)); /* c6 */
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;

        tmp0 = MULTIPLY(z1 - z2, FIX(0.707_106_781)); /* c6 */
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;

        tmp0 = MULTIPLY(z1 + z2, FIX(1.328_926_049)); /* c2 */
        tmp2 = MULTIPLY(z1, FIX(1.083_350_441)); /* c4 */
        tmp3 = MULTIPLY(z2, FIX(0.245_575_608)); /* c8 */

        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);

        z2 = MULTIPLY(z2, -FIX(1.224_744_871)); /* -c3 */

        tmp2 = MULTIPLY(z1 + z3, FIX(0.909_038_955)); /* c5 */
        tmp3 = MULTIPLY(z1 + z4, FIX(0.483_689_525)); /* c7 */
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 = MULTIPLY(z3 - z4, FIX(1.392_728_481)); /* c1 */
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = MULTIPLY(z1 - z3 - z4, FIX(1.224_744_871)); /* c3 */

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp11 + tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 7 + ctr] = RIGHT_SHIFT(tmp11 - tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = RIGHT_SHIFT(tmp12 + tmp2, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp12 - tmp2, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 3 + ctr] = RIGHT_SHIFT(tmp13 + tmp3, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp13 - tmp3, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp14, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 9 rows from work array, store into output array. */

    for ctr in 0..9 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        tmp0 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);

        z1 = JLONG::from(wsptr[2]);
        z2 = JLONG::from(wsptr[4]);
        z3 = JLONG::from(wsptr[6]);

        tmp3 = MULTIPLY(z3, FIX(0.707_106_781)); /* c6 */
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;

        tmp0 = MULTIPLY(z1 - z2, FIX(0.707_106_781)); /* c6 */
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;

        tmp0 = MULTIPLY(z1 + z2, FIX(1.328_926_049)); /* c2 */
        tmp2 = MULTIPLY(z1, FIX(1.083_350_441)); /* c4 */
        tmp3 = MULTIPLY(z2, FIX(0.245_575_608)); /* c8 */

        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        z4 = JLONG::from(wsptr[7]);

        z2 = MULTIPLY(z2, -FIX(1.224_744_871)); /* -c3 */

        tmp2 = MULTIPLY(z1 + z3, FIX(0.909_038_955)); /* c5 */
        tmp3 = MULTIPLY(z1 + z4, FIX(0.483_689_525)); /* c7 */
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 = MULTIPLY(z3 - z4, FIX(1.392_728_481)); /* c1 */
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = MULTIPLY(z1 - z3 - z4, FIX(1.224_744_871)); /* c3 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 + tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp10 - tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 + tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp11 - tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp12 + tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp12 - tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp13 + tmp3, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp13 - tmp3, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] =
            range_limit_sample(range_limit, RIGHT_SHIFT(tmp14, CONST_BITS + PASS1_BITS + 3));
    }
}

// upstream: src/jidctint.c _jpeg_idct_10x10
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a 10x10 output block.
///
/// Optimized algorithm with 12 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/20).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 10 rows, or fewer than 10 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_10x10(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp14: JLONG;
    let mut tmp20: JLONG;
    let mut tmp21: JLONG;
    let mut tmp22: JLONG;
    let mut tmp23: JLONG;
    let mut tmp24: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 10]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        z3 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        z3 = LEFT_SHIFT(z3, CONST_BITS);
        /* Add fudge factor here for final descale. */
        z3 += ONE << (CONST_BITS - PASS1_BITS - 1);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z1 = MULTIPLY(z4, FIX(1.144_122_806)); /* c4 */
        z2 = MULTIPLY(z4, FIX(0.437_016_024)); /* c8 */
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;

        tmp22 = RIGHT_SHIFT(z3 - LEFT_SHIFT(z1 - z2, 1), CONST_BITS - PASS1_BITS); /* c0 = (c4-c8)*2 */

        z2 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);

        z1 = MULTIPLY(z2 + z3, FIX(0.831_253_876)); /* c6 */
        tmp12 = z1 + MULTIPLY(z2, FIX(0.513_743_148)); /* c2-c6 */
        tmp13 = z1 - MULTIPLY(z3, FIX(2.176_250_899)); /* c2+c6 */

        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);

        tmp11 = z2 + z4;
        tmp13 = z2 - z4;

        tmp12 = MULTIPLY(tmp13, FIX(0.309_016_994)); /* (c3-c7)/2 */
        let z5: JLONG = LEFT_SHIFT(z3, CONST_BITS);

        z2 = MULTIPLY(tmp11, FIX(0.951_056_516)); /* (c3+c7)/2 */
        z4 = z5 + tmp12;

        tmp10 = MULTIPLY(z1, FIX(1.396_802_247)) + z2 + z4; /* c1 */
        tmp14 = MULTIPLY(z1, FIX(0.221_231_742)) - z2 + z4; /* c9 */

        z2 = MULTIPLY(tmp11, FIX(0.587_785_252)); /* (c1-c9)/2 */
        z4 = z5 - tmp12 - LEFT_SHIFT(tmp13, CONST_BITS - 1);

        tmp12 = LEFT_SHIFT(z1 - tmp13 - z3, PASS1_BITS);

        tmp11 = MULTIPLY(z1, FIX(1.260_073_511)) - z2 - z4; /* c3 */
        tmp13 = MULTIPLY(z1, FIX(0.642_039_522)) - z2 + z4; /* c7 */

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 9 + ctr] = RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = (tmp22 + tmp12) as i32;
        workspace[8 * 7 + ctr] = (tmp22 - tmp12) as i32;
        workspace[8 * 3 + ctr] = RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 10 rows from work array, store into output array. */

    for ctr in 0..10 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        z3 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        z3 = LEFT_SHIFT(z3, CONST_BITS);
        z4 = JLONG::from(wsptr[4]);
        z1 = MULTIPLY(z4, FIX(1.144_122_806)); /* c4 */
        z2 = MULTIPLY(z4, FIX(0.437_016_024)); /* c8 */
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;

        tmp22 = z3 - LEFT_SHIFT(z1 - z2, 1); /* c0 = (c4-c8)*2 */

        z2 = JLONG::from(wsptr[2]);
        z3 = JLONG::from(wsptr[6]);

        z1 = MULTIPLY(z2 + z3, FIX(0.831_253_876)); /* c6 */
        tmp12 = z1 + MULTIPLY(z2, FIX(0.513_743_148)); /* c2-c6 */
        tmp13 = z1 - MULTIPLY(z3, FIX(2.176_250_899)); /* c2+c6 */

        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        z3 = LEFT_SHIFT(z3, CONST_BITS);
        z4 = JLONG::from(wsptr[7]);

        tmp11 = z2 + z4;
        tmp13 = z2 - z4;

        tmp12 = MULTIPLY(tmp13, FIX(0.309_016_994)); /* (c3-c7)/2 */

        z2 = MULTIPLY(tmp11, FIX(0.951_056_516)); /* (c3+c7)/2 */
        z4 = z3 + tmp12;

        tmp10 = MULTIPLY(z1, FIX(1.396_802_247)) + z2 + z4; /* c1 */
        tmp14 = MULTIPLY(z1, FIX(0.221_231_742)) - z2 + z4; /* c9 */

        z2 = MULTIPLY(tmp11, FIX(0.587_785_252)); /* (c1-c9)/2 */
        z4 = z3 - tmp12 - LEFT_SHIFT(tmp13, CONST_BITS - 1);

        tmp12 = LEFT_SHIFT(z1 - tmp13, CONST_BITS) - z3;

        tmp11 = MULTIPLY(z1, FIX(1.260_073_511)) - z2 - z4; /* c3 */
        tmp13 = MULTIPLY(z1, FIX(0.642_039_522)) - z2 + z4; /* c7 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[9] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS + PASS1_BITS + 3),
        );
    }
}

// upstream: src/jidctint.c _jpeg_idct_11x11
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing an 11x11 output block.
///
/// Optimized algorithm with 24 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/22).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 11 rows, or fewer than 11 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::approx_constant,
    reason = "FIX() takes C decimal constants that approximate sqrt(2)"
)]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_11x11(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp14: JLONG;
    let mut tmp20: JLONG;
    let mut tmp21: JLONG;
    let mut tmp22: JLONG;
    let mut tmp23: JLONG;
    let mut tmp24: JLONG;
    let mut tmp25: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 11]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        tmp10 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        tmp10 = LEFT_SHIFT(tmp10, CONST_BITS);
        /* Add fudge factor here for final descale. */
        tmp10 += ONE << (CONST_BITS - PASS1_BITS - 1);

        z1 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);

        tmp20 = MULTIPLY(z2 - z3, FIX(2.546_640_132)); /* c2+c4 */
        tmp23 = MULTIPLY(z2 - z1, FIX(0.430_815_045)); /* c2-c6 */
        z4 = z1 + z3;
        tmp24 = MULTIPLY(z4, -FIX(1.155_664_402)); /* -(c2-c10) */
        z4 -= z2;
        tmp25 = tmp10 + MULTIPLY(z4, FIX(1.356_927_976)); /* c2 */
        tmp21 = tmp20 + tmp23 + tmp25 - MULTIPLY(z2, FIX(1.821_790_775)); /* c2+c4+c10-c6 */
        tmp20 += tmp25 + MULTIPLY(z3, FIX(2.115_825_087)); /* c4+c6 */
        tmp23 += tmp25 - MULTIPLY(z1, FIX(1.513_598_477)); /* c6+c8 */
        tmp24 += tmp25;
        tmp22 = tmp24 - MULTIPLY(z3, FIX(0.788_749_120)); /* c8+c10 */
        tmp24 += MULTIPLY(z2, FIX(1.944_413_522)) -        /* c2+c8 */
                 MULTIPLY(z1, FIX(1.390_975_730)); /* c4+c10 */
        tmp25 = tmp10 - MULTIPLY(z4, FIX(1.414_213_562)); /* c0 */

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);

        tmp11 = z1 + z2;
        tmp14 = MULTIPLY(tmp11 + z3 + z4, FIX(0.398_430_003)); /* c9 */
        tmp11 = MULTIPLY(tmp11, FIX(0.887_983_902)); /* c3-c9 */
        tmp12 = MULTIPLY(z1 + z3, FIX(0.670_361_295)); /* c5-c9 */
        tmp13 = tmp14 + MULTIPLY(z1 + z4, FIX(0.366_151_574)); /* c7-c9 */
        tmp10 = tmp11 + tmp12 + tmp13 - MULTIPLY(z1, FIX(0.923_107_866)); /* c7+c5+c3-c1-2*c9 */
        z1 = tmp14 - MULTIPLY(z2 + z3, FIX(1.163_011_579)); /* c7+c9 */
        tmp11 += z1 + MULTIPLY(z2, FIX(2.073_276_588)); /* c1+c7+3*c9-c3 */
        tmp12 += z1 - MULTIPLY(z3, FIX(1.192_193_623)); /* c3+c5-c7-c9 */
        z1 = MULTIPLY(z2 + z4, -FIX(1.798_248_910)); /* -(c1+c9) */
        tmp11 += z1;
        tmp13 += z1 + MULTIPLY(z4, FIX(2.102_458_632)); /* c1+c5+c9-c7 */
        tmp14 += MULTIPLY(z2, -FIX(1.467_221_301)) +           /* -(c5+c9) */
                 MULTIPLY(z3, FIX(1.001_388_905)) -            /* c1-c9 */
                 MULTIPLY(z4, FIX(1.684_843_907)); /* c3+c9 */

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 10 + ctr] = RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 9 + ctr] = RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 3 + ctr] = RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 7 + ctr] = RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp25, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 11 rows from work array, store into output array. */

    for ctr in 0..11 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        tmp10 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        tmp10 = LEFT_SHIFT(tmp10, CONST_BITS);

        z1 = JLONG::from(wsptr[2]);
        z2 = JLONG::from(wsptr[4]);
        z3 = JLONG::from(wsptr[6]);

        tmp20 = MULTIPLY(z2 - z3, FIX(2.546_640_132)); /* c2+c4 */
        tmp23 = MULTIPLY(z2 - z1, FIX(0.430_815_045)); /* c2-c6 */
        z4 = z1 + z3;
        tmp24 = MULTIPLY(z4, -FIX(1.155_664_402)); /* -(c2-c10) */
        z4 -= z2;
        tmp25 = tmp10 + MULTIPLY(z4, FIX(1.356_927_976)); /* c2 */
        tmp21 = tmp20 + tmp23 + tmp25 - MULTIPLY(z2, FIX(1.821_790_775)); /* c2+c4+c10-c6 */
        tmp20 += tmp25 + MULTIPLY(z3, FIX(2.115_825_087)); /* c4+c6 */
        tmp23 += tmp25 - MULTIPLY(z1, FIX(1.513_598_477)); /* c6+c8 */
        tmp24 += tmp25;
        tmp22 = tmp24 - MULTIPLY(z3, FIX(0.788_749_120)); /* c8+c10 */
        tmp24 += MULTIPLY(z2, FIX(1.944_413_522)) -        /* c2+c8 */
                 MULTIPLY(z1, FIX(1.390_975_730)); /* c4+c10 */
        tmp25 = tmp10 - MULTIPLY(z4, FIX(1.414_213_562)); /* c0 */

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        z4 = JLONG::from(wsptr[7]);

        tmp11 = z1 + z2;
        tmp14 = MULTIPLY(tmp11 + z3 + z4, FIX(0.398_430_003)); /* c9 */
        tmp11 = MULTIPLY(tmp11, FIX(0.887_983_902)); /* c3-c9 */
        tmp12 = MULTIPLY(z1 + z3, FIX(0.670_361_295)); /* c5-c9 */
        tmp13 = tmp14 + MULTIPLY(z1 + z4, FIX(0.366_151_574)); /* c7-c9 */
        tmp10 = tmp11 + tmp12 + tmp13 - MULTIPLY(z1, FIX(0.923_107_866)); /* c7+c5+c3-c1-2*c9 */
        z1 = tmp14 - MULTIPLY(z2 + z3, FIX(1.163_011_579)); /* c7+c9 */
        tmp11 += z1 + MULTIPLY(z2, FIX(2.073_276_588)); /* c1+c7+3*c9-c3 */
        tmp12 += z1 - MULTIPLY(z3, FIX(1.192_193_623)); /* c3+c5-c7-c9 */
        z1 = MULTIPLY(z2 + z4, -FIX(1.798_248_910)); /* -(c1+c9) */
        tmp11 += z1;
        tmp13 += z1 + MULTIPLY(z4, FIX(2.102_458_632)); /* c1+c5+c9-c7 */
        tmp14 += MULTIPLY(z2, -FIX(1.467_221_301)) +           /* -(c5+c9) */
                 MULTIPLY(z3, FIX(1.001_388_905)) -            /* c1-c9 */
                 MULTIPLY(z4, FIX(1.684_843_907)); /* c3+c9 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[10] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[9] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] =
            range_limit_sample(range_limit, RIGHT_SHIFT(tmp25, CONST_BITS + PASS1_BITS + 3));
    }
}

// upstream: src/jidctint.c _jpeg_idct_12x12
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a 12x12 output block.
///
/// Optimized algorithm with 15 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/24).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 12 rows, or fewer than 12 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_12x12(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp14: JLONG;
    let mut tmp15: JLONG;
    let mut tmp20: JLONG;
    let mut tmp21: JLONG;
    let mut tmp22: JLONG;
    let mut tmp23: JLONG;
    let mut tmp24: JLONG;
    let mut tmp25: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 12]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        z3 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        z3 = LEFT_SHIFT(z3, CONST_BITS);
        /* Add fudge factor here for final descale. */
        z3 += ONE << (CONST_BITS - PASS1_BITS - 1);

        z4 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z4 = MULTIPLY(z4, FIX(1.224_744_871)); /* c4 */

        tmp10 = z3 + z4;
        tmp11 = z3 - z4;

        z1 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z4 = MULTIPLY(z1, FIX(1.366_025_404)); /* c2 */
        z1 = LEFT_SHIFT(z1, CONST_BITS);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);
        z2 = LEFT_SHIFT(z2, CONST_BITS);

        tmp12 = z1 - z2;

        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;

        tmp12 = z4 + z2;

        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;

        tmp12 = z4 - z1 - z2;

        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);

        tmp11 = MULTIPLY(z2, FIX(1.306_562_965)); /* c3 */
        tmp14 = MULTIPLY(z2, -FIX_0_541196100); /* -c9 */

        tmp10 = z1 + z3;
        tmp15 = MULTIPLY(tmp10 + z4, FIX(0.860_918_669)); /* c7 */
        tmp12 = tmp15 + MULTIPLY(tmp10, FIX(0.261_052_384)); /* c5-c7 */
        tmp10 = tmp12 + tmp11 + MULTIPLY(z1, FIX(0.280_143_716)); /* c1-c5 */
        tmp13 = MULTIPLY(z3 + z4, -FIX(1.045_510_580)); /* -(c7+c11) */
        tmp12 += tmp13 + tmp14 - MULTIPLY(z3, FIX(1.478_575_242)); /* c1+c5-c7-c11 */
        tmp13 += tmp15 - tmp11 + MULTIPLY(z4, FIX(1.586_706_681)); /* c1+c11 */
        tmp15 += tmp14 - MULTIPLY(z1, FIX(0.676_326_758)) -        /* c7-c11 */
                 MULTIPLY(z4, FIX(1.982_889_723)); /* c5+c7 */

        z1 -= z4;
        z2 -= z3;
        z3 = MULTIPLY(z1 + z2, FIX_0_541196100); /* c9 */
        tmp11 = z3 + MULTIPLY(z1, FIX_0_765366865); /* c3-c9 */
        tmp14 = z3 - MULTIPLY(z2, FIX_1_847759065); /* c3+c9 */

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 11 + ctr] = RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 10 + ctr] = RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 9 + ctr] = RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 3 + ctr] = RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 7 + ctr] = RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 12 rows from work array, store into output array. */

    for ctr in 0..12 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        z3 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        z3 = LEFT_SHIFT(z3, CONST_BITS);

        z4 = JLONG::from(wsptr[4]);
        z4 = MULTIPLY(z4, FIX(1.224_744_871)); /* c4 */

        tmp10 = z3 + z4;
        tmp11 = z3 - z4;

        z1 = JLONG::from(wsptr[2]);
        z4 = MULTIPLY(z1, FIX(1.366_025_404)); /* c2 */
        z1 = LEFT_SHIFT(z1, CONST_BITS);
        z2 = JLONG::from(wsptr[6]);
        z2 = LEFT_SHIFT(z2, CONST_BITS);

        tmp12 = z1 - z2;

        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;

        tmp12 = z4 + z2;

        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;

        tmp12 = z4 - z1 - z2;

        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        z4 = JLONG::from(wsptr[7]);

        tmp11 = MULTIPLY(z2, FIX(1.306_562_965)); /* c3 */
        tmp14 = MULTIPLY(z2, -FIX_0_541196100); /* -c9 */

        tmp10 = z1 + z3;
        tmp15 = MULTIPLY(tmp10 + z4, FIX(0.860_918_669)); /* c7 */
        tmp12 = tmp15 + MULTIPLY(tmp10, FIX(0.261_052_384)); /* c5-c7 */
        tmp10 = tmp12 + tmp11 + MULTIPLY(z1, FIX(0.280_143_716)); /* c1-c5 */
        tmp13 = MULTIPLY(z3 + z4, -FIX(1.045_510_580)); /* -(c7+c11) */
        tmp12 += tmp13 + tmp14 - MULTIPLY(z3, FIX(1.478_575_242)); /* c1+c5-c7-c11 */
        tmp13 += tmp15 - tmp11 + MULTIPLY(z4, FIX(1.586_706_681)); /* c1+c11 */
        tmp15 += tmp14 - MULTIPLY(z1, FIX(0.676_326_758)) -        /* c7-c11 */
                 MULTIPLY(z4, FIX(1.982_889_723)); /* c5+c7 */

        z1 -= z4;
        z2 -= z3;
        z3 = MULTIPLY(z1 + z2, FIX_0_541196100); /* c9 */
        tmp11 = z3 + MULTIPLY(z1, FIX_0_765366865); /* c3-c9 */
        tmp14 = z3 - MULTIPLY(z2, FIX_1_847759065); /* c3+c9 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[11] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[10] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[9] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS + PASS1_BITS + 3),
        );
    }
}

// upstream: src/jidctint.c _jpeg_idct_13x13
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a 13x13 output block.
///
/// Optimized algorithm with 29 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/26).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 13 rows, or fewer than 13 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::approx_constant,
    reason = "FIX() takes C decimal constants that approximate sqrt(2)"
)]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_13x13(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp14: JLONG;
    let mut tmp15: JLONG;
    let mut tmp20: JLONG;
    let mut tmp21: JLONG;
    let mut tmp22: JLONG;
    let mut tmp23: JLONG;
    let mut tmp24: JLONG;
    let mut tmp25: JLONG;
    let mut tmp26: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 13]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        z1 = LEFT_SHIFT(z1, CONST_BITS);
        /* Add fudge factor here for final descale. */
        z1 += ONE << (CONST_BITS - PASS1_BITS - 1);

        z2 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);

        tmp10 = z3 + z4;
        tmp11 = z3 - z4;

        tmp12 = MULTIPLY(tmp10, FIX(1.155_388_986)); /* (c4+c6)/2 */
        tmp13 = MULTIPLY(tmp11, FIX(0.096_834_934)) + z1; /* (c4-c6)/2 */

        tmp20 = MULTIPLY(z2, FIX(1.373_119_086)) + tmp12 + tmp13; /* c2 */
        tmp22 = MULTIPLY(z2, FIX(0.501_487_041)) - tmp12 + tmp13; /* c10 */

        tmp12 = MULTIPLY(tmp10, FIX(0.316_450_131)); /* (c8-c12)/2 */
        tmp13 = MULTIPLY(tmp11, FIX(0.486_914_739)) + z1; /* (c8+c12)/2 */

        tmp21 = MULTIPLY(z2, FIX(1.058_554_052)) - tmp12 + tmp13; /* c6 */
        tmp25 = MULTIPLY(z2, -FIX(1.252_223_920)) + tmp12 + tmp13; /* c4 */

        tmp12 = MULTIPLY(tmp10, FIX(0.435_816_023)); /* (c2-c10)/2 */
        tmp13 = MULTIPLY(tmp11, FIX(0.937_303_064)) - z1; /* (c2+c10)/2 */

        tmp23 = MULTIPLY(z2, -FIX(0.170_464_608)) - tmp12 - tmp13; /* c12 */
        tmp24 = MULTIPLY(z2, -FIX(0.803_364_869)) + tmp12 - tmp13; /* c8 */

        tmp26 = MULTIPLY(tmp11 - z2, FIX(1.414_213_562)) + z1; /* c0 */

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);

        tmp11 = MULTIPLY(z1 + z2, FIX(1.322_312_651)); /* c3 */
        tmp12 = MULTIPLY(z1 + z3, FIX(1.163_874_945)); /* c5 */
        tmp15 = z1 + z4;
        tmp13 = MULTIPLY(tmp15, FIX(0.937_797_057)); /* c7 */
        tmp10 = tmp11 + tmp12 + tmp13 - MULTIPLY(z1, FIX(2.020_082_300)); /* c7+c5+c3-c1 */
        tmp14 = MULTIPLY(z2 + z3, -FIX(0.338_443_458)); /* -c11 */
        tmp11 += tmp14 + MULTIPLY(z2, FIX(0.837_223_564)); /* c5+c9+c11-c3 */
        tmp12 += tmp14 - MULTIPLY(z3, FIX(1.572_116_027)); /* c1+c5-c9-c11 */
        tmp14 = MULTIPLY(z2 + z4, -FIX(1.163_874_945)); /* -c5 */
        tmp11 += tmp14;
        tmp13 += tmp14 + MULTIPLY(z4, FIX(2.205_608_352)); /* c3+c5+c9-c7 */
        tmp14 = MULTIPLY(z3 + z4, -FIX(0.657_217_813)); /* -c9 */
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = MULTIPLY(tmp15, FIX(0.338_443_458)); /* c11 */
        tmp14 = tmp15 + MULTIPLY(z1, FIX(0.318_774_355)) - /* c9-c11 */
                MULTIPLY(z2, FIX(0.466_105_296)); /* c1-c7 */
        z1 = MULTIPLY(z3 - z2, FIX(0.937_797_057)); /* c7 */
        tmp14 += z1;
        tmp15 += z1 + MULTIPLY(z3, FIX(0.384_515_595)) -   /* c3-c7 */
                 MULTIPLY(z4, FIX(1.742_345_811)); /* c1+c11 */

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 12 + ctr] = RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 11 + ctr] = RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 10 + ctr] = RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 3 + ctr] = RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 9 + ctr] = RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 7 + ctr] = RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp26, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 13 rows from work array, store into output array. */

    for ctr in 0..13 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        z1 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        z1 = LEFT_SHIFT(z1, CONST_BITS);

        z2 = JLONG::from(wsptr[2]);
        z3 = JLONG::from(wsptr[4]);
        z4 = JLONG::from(wsptr[6]);

        tmp10 = z3 + z4;
        tmp11 = z3 - z4;

        tmp12 = MULTIPLY(tmp10, FIX(1.155_388_986)); /* (c4+c6)/2 */
        tmp13 = MULTIPLY(tmp11, FIX(0.096_834_934)) + z1; /* (c4-c6)/2 */

        tmp20 = MULTIPLY(z2, FIX(1.373_119_086)) + tmp12 + tmp13; /* c2 */
        tmp22 = MULTIPLY(z2, FIX(0.501_487_041)) - tmp12 + tmp13; /* c10 */

        tmp12 = MULTIPLY(tmp10, FIX(0.316_450_131)); /* (c8-c12)/2 */
        tmp13 = MULTIPLY(tmp11, FIX(0.486_914_739)) + z1; /* (c8+c12)/2 */

        tmp21 = MULTIPLY(z2, FIX(1.058_554_052)) - tmp12 + tmp13; /* c6 */
        tmp25 = MULTIPLY(z2, -FIX(1.252_223_920)) + tmp12 + tmp13; /* c4 */

        tmp12 = MULTIPLY(tmp10, FIX(0.435_816_023)); /* (c2-c10)/2 */
        tmp13 = MULTIPLY(tmp11, FIX(0.937_303_064)) - z1; /* (c2+c10)/2 */

        tmp23 = MULTIPLY(z2, -FIX(0.170_464_608)) - tmp12 - tmp13; /* c12 */
        tmp24 = MULTIPLY(z2, -FIX(0.803_364_869)) + tmp12 - tmp13; /* c8 */

        tmp26 = MULTIPLY(tmp11 - z2, FIX(1.414_213_562)) + z1; /* c0 */

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        z4 = JLONG::from(wsptr[7]);

        tmp11 = MULTIPLY(z1 + z2, FIX(1.322_312_651)); /* c3 */
        tmp12 = MULTIPLY(z1 + z3, FIX(1.163_874_945)); /* c5 */
        tmp15 = z1 + z4;
        tmp13 = MULTIPLY(tmp15, FIX(0.937_797_057)); /* c7 */
        tmp10 = tmp11 + tmp12 + tmp13 - MULTIPLY(z1, FIX(2.020_082_300)); /* c7+c5+c3-c1 */
        tmp14 = MULTIPLY(z2 + z3, -FIX(0.338_443_458)); /* -c11 */
        tmp11 += tmp14 + MULTIPLY(z2, FIX(0.837_223_564)); /* c5+c9+c11-c3 */
        tmp12 += tmp14 - MULTIPLY(z3, FIX(1.572_116_027)); /* c1+c5-c9-c11 */
        tmp14 = MULTIPLY(z2 + z4, -FIX(1.163_874_945)); /* -c5 */
        tmp11 += tmp14;
        tmp13 += tmp14 + MULTIPLY(z4, FIX(2.205_608_352)); /* c3+c5+c9-c7 */
        tmp14 = MULTIPLY(z3 + z4, -FIX(0.657_217_813)); /* -c9 */
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = MULTIPLY(tmp15, FIX(0.338_443_458)); /* c11 */
        tmp14 = tmp15 + MULTIPLY(z1, FIX(0.318_774_355)) - /* c9-c11 */
                MULTIPLY(z2, FIX(0.466_105_296)); /* c1-c7 */
        z1 = MULTIPLY(z3 - z2, FIX(0.937_797_057)); /* c7 */
        tmp14 += z1;
        tmp15 += z1 + MULTIPLY(z3, FIX(0.384_515_595)) -   /* c3-c7 */
                 MULTIPLY(z4, FIX(1.742_345_811)); /* c1+c11 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[12] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[11] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[10] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[9] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] =
            range_limit_sample(range_limit, RIGHT_SHIFT(tmp26, CONST_BITS + PASS1_BITS + 3));
    }
}

// upstream: src/jidctint.c _jpeg_idct_14x14
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a 14x14 output block.
///
/// Optimized algorithm with 20 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/28).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 14 rows, or fewer than 14 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_14x14(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp14: JLONG;
    let mut tmp15: JLONG;
    let mut tmp16: JLONG;
    let mut tmp20: JLONG;
    let mut tmp21: JLONG;
    let mut tmp22: JLONG;
    let mut tmp23: JLONG;
    let mut tmp24: JLONG;
    let mut tmp25: JLONG;
    let mut tmp26: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 14]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        z1 = LEFT_SHIFT(z1, CONST_BITS);
        /* Add fudge factor here for final descale. */
        z1 += ONE << (CONST_BITS - PASS1_BITS - 1);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z2 = MULTIPLY(z4, FIX(1.274_162_392)); /* c4 */
        z3 = MULTIPLY(z4, FIX(0.314_692_123)); /* c12 */
        z4 = MULTIPLY(z4, FIX(0.881_747_734)); /* c8 */

        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;

        tmp23 = RIGHT_SHIFT(z1 - LEFT_SHIFT(z2 + z3 - z4, 1), CONST_BITS - PASS1_BITS); /* c0 = (c4+c12-c8)*2 */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);

        z3 = MULTIPLY(z1 + z2, FIX(1.105_676_686)); /* c6 */

        tmp13 = z3 + MULTIPLY(z1, FIX(0.273_079_590)); /* c2-c6 */
        tmp14 = z3 - MULTIPLY(z2, FIX(1.719_280_954)); /* c6+c10 */
        tmp15 = MULTIPLY(z1, FIX(0.613_604_268)) -     /* c10 */
                MULTIPLY(z2, FIX(1.378_756_276)); /* c2 */

        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);
        tmp13 = LEFT_SHIFT(z4, CONST_BITS);

        tmp14 = z1 + z3;
        tmp11 = MULTIPLY(z1 + z2, FIX(1.334_852_607)); /* c3 */
        tmp12 = MULTIPLY(tmp14, FIX(1.197_448_846)); /* c5 */
        tmp10 = tmp11 + tmp12 + tmp13 - MULTIPLY(z1, FIX(1.126_980_169)); /* c3+c5-c1 */
        tmp14 = MULTIPLY(tmp14, FIX(0.752_406_978)); /* c9 */
        tmp16 = tmp14 - MULTIPLY(z1, FIX(1.061_150_426)); /* c9+c11-c13 */
        z1 -= z2;
        tmp15 = MULTIPLY(z1, FIX(0.467_085_129)) - tmp13; /* c11 */
        tmp16 += tmp15;
        z1 += z4;
        z4 = MULTIPLY(z2 + z3, -FIX(0.158_341_681)) - tmp13; /* -c13 */
        tmp11 += z4 - MULTIPLY(z2, FIX(0.424_103_948)); /* c3-c9-c13 */
        tmp12 += z4 - MULTIPLY(z3, FIX(2.373_959_773)); /* c3+c5-c13 */
        z4 = MULTIPLY(z3 - z2, FIX(1.405_321_284)); /* c1 */
        tmp14 += z4 + tmp13 - MULTIPLY(z3, FIX(1.690_643_133_4)); /* c1+c9-c11 */
        tmp15 += z4 + MULTIPLY(z2, FIX(0.674_957_567)); /* c1+c11-c5 */

        tmp13 = LEFT_SHIFT(z1 - z3, PASS1_BITS);

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 13 + ctr] = RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 12 + ctr] = RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 11 + ctr] = RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 3 + ctr] = (tmp23 + tmp13) as i32;
        workspace[8 * 10 + ctr] = (tmp23 - tmp13) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 9 + ctr] = RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp26 + tmp16, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 7 + ctr] = RIGHT_SHIFT(tmp26 - tmp16, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 14 rows from work array, store into output array. */

    for ctr in 0..14 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        z1 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        z1 = LEFT_SHIFT(z1, CONST_BITS);
        z4 = JLONG::from(wsptr[4]);
        z2 = MULTIPLY(z4, FIX(1.274_162_392)); /* c4 */
        z3 = MULTIPLY(z4, FIX(0.314_692_123)); /* c12 */
        z4 = MULTIPLY(z4, FIX(0.881_747_734)); /* c8 */

        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;

        tmp23 = z1 - LEFT_SHIFT(z2 + z3 - z4, 1); /* c0 = (c4+c12-c8)*2 */

        z1 = JLONG::from(wsptr[2]);
        z2 = JLONG::from(wsptr[6]);

        z3 = MULTIPLY(z1 + z2, FIX(1.105_676_686)); /* c6 */

        tmp13 = z3 + MULTIPLY(z1, FIX(0.273_079_590)); /* c2-c6 */
        tmp14 = z3 - MULTIPLY(z2, FIX(1.719_280_954)); /* c6+c10 */
        tmp15 = MULTIPLY(z1, FIX(0.613_604_268)) -     /* c10 */
                MULTIPLY(z2, FIX(1.378_756_276)); /* c2 */

        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        z4 = JLONG::from(wsptr[7]);
        z4 = LEFT_SHIFT(z4, CONST_BITS);

        tmp14 = z1 + z3;
        tmp11 = MULTIPLY(z1 + z2, FIX(1.334_852_607)); /* c3 */
        tmp12 = MULTIPLY(tmp14, FIX(1.197_448_846)); /* c5 */
        tmp10 = tmp11 + tmp12 + z4 - MULTIPLY(z1, FIX(1.126_980_169)); /* c3+c5-c1 */
        tmp14 = MULTIPLY(tmp14, FIX(0.752_406_978)); /* c9 */
        tmp16 = tmp14 - MULTIPLY(z1, FIX(1.061_150_426)); /* c9+c11-c13 */
        z1 -= z2;
        tmp15 = MULTIPLY(z1, FIX(0.467_085_129)) - z4; /* c11 */
        tmp16 += tmp15;
        tmp13 = MULTIPLY(z2 + z3, -FIX(0.158_341_681)) - z4; /* -c13 */
        tmp11 += tmp13 - MULTIPLY(z2, FIX(0.424_103_948)); /* c3-c9-c13 */
        tmp12 += tmp13 - MULTIPLY(z3, FIX(2.373_959_773)); /* c3+c5-c13 */
        tmp13 = MULTIPLY(z3 - z2, FIX(1.405_321_284)); /* c1 */
        tmp14 += tmp13 + z4 - MULTIPLY(z3, FIX(1.690_643_133_4)); /* c1+c9-c11 */
        tmp15 += tmp13 + MULTIPLY(z2, FIX(0.674_957_567)); /* c1+c11-c5 */

        tmp13 = LEFT_SHIFT(z1 - z3, CONST_BITS) + z4;

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[13] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[12] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[11] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[10] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[9] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp26 + tmp16, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp26 - tmp16, CONST_BITS + PASS1_BITS + 3),
        );
    }
}

// upstream: src/jidctint.c _jpeg_idct_15x15
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a 15x15 output block.
///
/// Optimized algorithm with 22 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/30).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 15 rows, or fewer than 15 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_15x15(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp14: JLONG;
    let mut tmp15: JLONG;
    let mut tmp16: JLONG;
    let mut tmp20: JLONG;
    let mut tmp21: JLONG;
    let mut tmp22: JLONG;
    let mut tmp23: JLONG;
    let mut tmp24: JLONG;
    let mut tmp25: JLONG;
    let mut tmp26: JLONG;
    let mut tmp27: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 15]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        z1 = LEFT_SHIFT(z1, CONST_BITS);
        /* Add fudge factor here for final descale. */
        z1 += ONE << (CONST_BITS - PASS1_BITS - 1);

        z2 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);

        tmp10 = MULTIPLY(z4, FIX(0.437_016_024)); /* c12 */
        tmp11 = MULTIPLY(z4, FIX(1.144_122_806)); /* c6 */

        tmp12 = z1 - tmp10;
        tmp13 = z1 + tmp11;
        z1 -= LEFT_SHIFT(tmp11 - tmp10, 1); /* c0 = (c6-c12)*2 */

        z4 = z2 - z3;
        z3 += z2;
        tmp10 = MULTIPLY(z3, FIX(1.337_628_990)); /* (c2+c4)/2 */
        tmp11 = MULTIPLY(z4, FIX(0.045_680_613)); /* (c2-c4)/2 */
        z2 = MULTIPLY(z2, FIX(1.439_773_946)); /* c4+c14 */

        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;

        tmp10 = MULTIPLY(z3, FIX(0.547_059_574)); /* (c8+c14)/2 */
        tmp11 = MULTIPLY(z4, FIX(0.399_234_004)); /* (c8-c14)/2 */

        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;

        tmp10 = MULTIPLY(z3, FIX(0.790_569_415)); /* (c6+c12)/2 */
        tmp11 = MULTIPLY(z4, FIX(0.353_553_391)); /* (c6-c12)/2 */

        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11; /* c10 = c6-c12 */
        tmp27 = z1 - tmp11 - tmp11; /* c0 = (c6-c12)*2 */

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z3 = MULTIPLY(z4, FIX(1.224_744_871)); /* c5 */
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);

        tmp13 = z2 - z4;
        tmp15 = MULTIPLY(z1 + tmp13, FIX(0.831_253_876)); /* c9 */
        tmp11 = tmp15 + MULTIPLY(z1, FIX(0.513_743_148)); /* c3-c9 */
        tmp14 = tmp15 - MULTIPLY(tmp13, FIX(2.176_250_899)); /* c3+c9 */

        tmp13 = MULTIPLY(z2, -FIX(0.831_253_876)); /* -c9 */
        tmp15 = MULTIPLY(z2, -FIX(1.344_997_024)); /* -c3 */
        z2 = z1 - z4;
        tmp12 = z3 + MULTIPLY(z2, FIX(1.406_466_353)); /* c1 */

        tmp10 = tmp12 + MULTIPLY(z4, FIX(2.457_431_844)) - tmp15; /* c1+c7 */
        tmp16 = tmp12 - MULTIPLY(z1, FIX(1.112_434_820)) + tmp13; /* c1-c13 */
        tmp12 = MULTIPLY(z2, FIX(1.224_744_871)) - z3; /* c5 */
        z2 = MULTIPLY(z1 + z4, FIX(0.575_212_477)); /* c11 */
        tmp13 += z2 + MULTIPLY(z1, FIX(0.475_753_014)) - z3; /* c7-c11 */
        tmp15 += z2 - MULTIPLY(z4, FIX(0.869_244_010)) + z3; /* c11+c13 */

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 14 + ctr] = RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 13 + ctr] = RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 12 + ctr] = RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 3 + ctr] = RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 11 + ctr] = RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 10 + ctr] = RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 9 + ctr] = RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp26 + tmp16, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp26 - tmp16, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 7 + ctr] = RIGHT_SHIFT(tmp27, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 15 rows from work array, store into output array. */

    for ctr in 0..15 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        z1 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        z1 = LEFT_SHIFT(z1, CONST_BITS);

        z2 = JLONG::from(wsptr[2]);
        z3 = JLONG::from(wsptr[4]);
        z4 = JLONG::from(wsptr[6]);

        tmp10 = MULTIPLY(z4, FIX(0.437_016_024)); /* c12 */
        tmp11 = MULTIPLY(z4, FIX(1.144_122_806)); /* c6 */

        tmp12 = z1 - tmp10;
        tmp13 = z1 + tmp11;
        z1 -= LEFT_SHIFT(tmp11 - tmp10, 1); /* c0 = (c6-c12)*2 */

        z4 = z2 - z3;
        z3 += z2;
        tmp10 = MULTIPLY(z3, FIX(1.337_628_990)); /* (c2+c4)/2 */
        tmp11 = MULTIPLY(z4, FIX(0.045_680_613)); /* (c2-c4)/2 */
        z2 = MULTIPLY(z2, FIX(1.439_773_946)); /* c4+c14 */

        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;

        tmp10 = MULTIPLY(z3, FIX(0.547_059_574)); /* (c8+c14)/2 */
        tmp11 = MULTIPLY(z4, FIX(0.399_234_004)); /* (c8-c14)/2 */

        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;

        tmp10 = MULTIPLY(z3, FIX(0.790_569_415)); /* (c6+c12)/2 */
        tmp11 = MULTIPLY(z4, FIX(0.353_553_391)); /* (c6-c12)/2 */

        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11; /* c10 = c6-c12 */
        tmp27 = z1 - tmp11 - tmp11; /* c0 = (c6-c12)*2 */

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z4 = JLONG::from(wsptr[5]);
        z3 = MULTIPLY(z4, FIX(1.224_744_871)); /* c5 */
        z4 = JLONG::from(wsptr[7]);

        tmp13 = z2 - z4;
        tmp15 = MULTIPLY(z1 + tmp13, FIX(0.831_253_876)); /* c9 */
        tmp11 = tmp15 + MULTIPLY(z1, FIX(0.513_743_148)); /* c3-c9 */
        tmp14 = tmp15 - MULTIPLY(tmp13, FIX(2.176_250_899)); /* c3+c9 */

        tmp13 = MULTIPLY(z2, -FIX(0.831_253_876)); /* -c9 */
        tmp15 = MULTIPLY(z2, -FIX(1.344_997_024)); /* -c3 */
        z2 = z1 - z4;
        tmp12 = z3 + MULTIPLY(z2, FIX(1.406_466_353)); /* c1 */

        tmp10 = tmp12 + MULTIPLY(z4, FIX(2.457_431_844)) - tmp15; /* c1+c7 */
        tmp16 = tmp12 - MULTIPLY(z1, FIX(1.112_434_820)) + tmp13; /* c1-c13 */
        tmp12 = MULTIPLY(z2, FIX(1.224_744_871)) - z3; /* c5 */
        z2 = MULTIPLY(z1 + z4, FIX(0.575_212_477)); /* c11 */
        tmp13 += z2 + MULTIPLY(z1, FIX(0.475_753_014)) - z3; /* c7-c11 */
        tmp15 += z2 - MULTIPLY(z4, FIX(0.869_244_010)) + z3; /* c11+c13 */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 + tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[14] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 - tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 + tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[13] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 - tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 + tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[12] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 - tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 + tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[11] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 - tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 + tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[10] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 - tmp14, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 + tmp15, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[9] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 - tmp15, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp26 + tmp16, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp26 - tmp16, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] =
            range_limit_sample(range_limit, RIGHT_SHIFT(tmp27, CONST_BITS + PASS1_BITS + 3));
    }
}

// upstream: src/jidctint.c _jpeg_idct_16x16
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a 16x16 output block.
///
/// Optimized algorithm with 28 multiplications in the 1-D kernel.
/// cK represents sqrt(2) * cos(K*pi/32).
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 16 rows, or fewer than 16 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(
    clippy::erasing_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(
    clippy::identity_op,
    reason = "C writes DCTSIZE * k row offsets with k = 0 and 1"
)]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jpeg_idct_16x16(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp1: JLONG;
    let mut tmp2: JLONG;
    let mut tmp3: JLONG;
    let mut tmp10: JLONG;
    let mut tmp11: JLONG;
    let mut tmp12: JLONG;
    let mut tmp13: JLONG;
    let mut tmp20: JLONG;
    let mut tmp21: JLONG;
    let mut tmp22: JLONG;
    let mut tmp23: JLONG;
    let mut tmp24: JLONG;
    let mut tmp25: JLONG;
    let mut tmp26: JLONG;
    let mut tmp27: JLONG;
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; 8 * 16]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    for ctr in 0..8 {
        /* Even part */

        tmp0 = DEQUANTIZE(inptr[DCTSIZE * 0 + ctr], quantptr[DCTSIZE * 0 + ctr]);
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);
        /* Add fudge factor here for final descale. */
        tmp0 += ONE << (CONST_BITS - PASS1_BITS - 1);

        z1 = DEQUANTIZE(inptr[DCTSIZE * 4 + ctr], quantptr[DCTSIZE * 4 + ctr]);
        tmp1 = MULTIPLY(z1, FIX(1.306_562_965)); /* c4[16] = c2[8] */
        tmp2 = MULTIPLY(z1, FIX_0_541196100); /* c12[16] = c6[8] */

        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;

        z1 = DEQUANTIZE(inptr[DCTSIZE * 2 + ctr], quantptr[DCTSIZE * 2 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 6 + ctr], quantptr[DCTSIZE * 6 + ctr]);
        z3 = z1 - z2;
        z4 = MULTIPLY(z3, FIX(0.275_899_379)); /* c14[16] = c7[8] */
        z3 = MULTIPLY(z3, FIX(1.387_039_845)); /* c2[16] = c1[8] */

        tmp0 = z3 + MULTIPLY(z2, FIX_2_562915447); /* (c6+c2)[16] = (c3+c1)[8] */
        tmp1 = z4 + MULTIPLY(z1, FIX_0_899976223); /* (c6-c14)[16] = (c3-c7)[8] */
        tmp2 = z3 - MULTIPLY(z1, FIX(0.601_344_887)); /* (c2-c10)[16] = (c1-c5)[8] */
        tmp3 = z4 - MULTIPLY(z2, FIX(0.509_795_579)); /* (c10-c14)[16] = (c5-c7)[8] */

        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + ctr], quantptr[DCTSIZE * 1 + ctr]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 3 + ctr], quantptr[DCTSIZE * 3 + ctr]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 5 + ctr], quantptr[DCTSIZE * 5 + ctr]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 7 + ctr], quantptr[DCTSIZE * 7 + ctr]);

        tmp11 = z1 + z3;

        tmp1 = MULTIPLY(z1 + z2, FIX(1.353_318_001)); /* c3 */
        tmp2 = MULTIPLY(tmp11, FIX(1.247_225_013)); /* c5 */
        tmp3 = MULTIPLY(z1 + z4, FIX(1.093_201_867)); /* c7 */
        tmp10 = MULTIPLY(z1 - z4, FIX(0.897_167_586)); /* c9 */
        tmp11 = MULTIPLY(tmp11, FIX(0.666_655_658)); /* c11 */
        tmp12 = MULTIPLY(z1 - z2, FIX(0.410_524_528)); /* c13 */
        tmp0 = tmp1 + tmp2 + tmp3 - MULTIPLY(z1, FIX(2.286_341_144)); /* c7+c5+c3-c1 */
        tmp13 = tmp10 + tmp11 + tmp12 - MULTIPLY(z1, FIX(1.835_730_603)); /* c9+c11+c13-c15 */
        z1 = MULTIPLY(z2 + z3, FIX(0.138_617_169)); /* c15 */
        tmp1 += z1 + MULTIPLY(z2, FIX(0.071_888_074)); /* c9+c11-c3-c15 */
        tmp2 += z1 - MULTIPLY(z3, FIX(1.125_726_048)); /* c5+c7+c15-c3 */
        z1 = MULTIPLY(z3 - z2, FIX(1.407_403_738)); /* c1 */
        tmp11 += z1 - MULTIPLY(z3, FIX(0.766_367_282)); /* c1+c11-c9-c13 */
        tmp12 += z1 + MULTIPLY(z2, FIX(1.971_951_411)); /* c1+c5+c13-c7 */
        z2 += z4;
        z1 = MULTIPLY(z2, -FIX(0.666_655_658)); /* -c11 */
        tmp1 += z1;
        tmp3 += z1 + MULTIPLY(z4, FIX(1.065_388_962)); /* c3+c11+c15-c7 */
        z2 = MULTIPLY(z2, -FIX(1.247_225_013)); /* -c5 */
        tmp10 += z2 + MULTIPLY(z4, FIX(3.141_271_809)); /* c1+c5+c9-c13 */
        tmp12 += z2;
        z2 = MULTIPLY(z3 + z4, -FIX(1.353_318_001)); /* -c3 */
        tmp2 += z2;
        tmp3 += z2;
        z2 = MULTIPLY(z4 - z3, FIX(0.410_524_528)); /* c13 */
        tmp10 += z2;
        tmp11 += z2;

        /* Final output stage */

        workspace[8 * 0 + ctr] = RIGHT_SHIFT(tmp20 + tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 15 + ctr] = RIGHT_SHIFT(tmp20 - tmp0, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 1 + ctr] = RIGHT_SHIFT(tmp21 + tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 14 + ctr] = RIGHT_SHIFT(tmp21 - tmp1, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 2 + ctr] = RIGHT_SHIFT(tmp22 + tmp2, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 13 + ctr] = RIGHT_SHIFT(tmp22 - tmp2, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 3 + ctr] = RIGHT_SHIFT(tmp23 + tmp3, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 12 + ctr] = RIGHT_SHIFT(tmp23 - tmp3, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 4 + ctr] = RIGHT_SHIFT(tmp24 + tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 11 + ctr] = RIGHT_SHIFT(tmp24 - tmp10, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 5 + ctr] = RIGHT_SHIFT(tmp25 + tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 10 + ctr] = RIGHT_SHIFT(tmp25 - tmp11, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 6 + ctr] = RIGHT_SHIFT(tmp26 + tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 9 + ctr] = RIGHT_SHIFT(tmp26 - tmp12, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 7 + ctr] = RIGHT_SHIFT(tmp27 + tmp13, CONST_BITS - PASS1_BITS) as i32;
        workspace[8 * 8 + ctr] = RIGHT_SHIFT(tmp27 - tmp13, CONST_BITS - PASS1_BITS) as i32;
    }

    /* Pass 2: process 16 rows from work array, store into output array. */

    for ctr in 0..16 {
        let wsptr = &workspace[ctr * 8..(ctr + 1) * 8];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];

        /* Even part */

        /* Add fudge factor here for final descale. */
        tmp0 = JLONG::from(wsptr[0]) + (ONE << (PASS1_BITS + 2));
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS);

        z1 = JLONG::from(wsptr[4]);
        tmp1 = MULTIPLY(z1, FIX(1.306_562_965)); /* c4[16] = c2[8] */
        tmp2 = MULTIPLY(z1, FIX_0_541196100); /* c12[16] = c6[8] */

        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;

        z1 = JLONG::from(wsptr[2]);
        z2 = JLONG::from(wsptr[6]);
        z3 = z1 - z2;
        z4 = MULTIPLY(z3, FIX(0.275_899_379)); /* c14[16] = c7[8] */
        z3 = MULTIPLY(z3, FIX(1.387_039_845)); /* c2[16] = c1[8] */

        tmp0 = z3 + MULTIPLY(z2, FIX_2_562915447); /* (c6+c2)[16] = (c3+c1)[8] */
        tmp1 = z4 + MULTIPLY(z1, FIX_0_899976223); /* (c6-c14)[16] = (c3-c7)[8] */
        tmp2 = z3 - MULTIPLY(z1, FIX(0.601_344_887)); /* (c2-c10)[16] = (c1-c5)[8] */
        tmp3 = z4 - MULTIPLY(z2, FIX(0.509_795_579)); /* (c10-c14)[16] = (c5-c7)[8] */

        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;

        /* Odd part */

        z1 = JLONG::from(wsptr[1]);
        z2 = JLONG::from(wsptr[3]);
        z3 = JLONG::from(wsptr[5]);
        z4 = JLONG::from(wsptr[7]);

        tmp11 = z1 + z3;

        tmp1 = MULTIPLY(z1 + z2, FIX(1.353_318_001)); /* c3 */
        tmp2 = MULTIPLY(tmp11, FIX(1.247_225_013)); /* c5 */
        tmp3 = MULTIPLY(z1 + z4, FIX(1.093_201_867)); /* c7 */
        tmp10 = MULTIPLY(z1 - z4, FIX(0.897_167_586)); /* c9 */
        tmp11 = MULTIPLY(tmp11, FIX(0.666_655_658)); /* c11 */
        tmp12 = MULTIPLY(z1 - z2, FIX(0.410_524_528)); /* c13 */
        tmp0 = tmp1 + tmp2 + tmp3 - MULTIPLY(z1, FIX(2.286_341_144)); /* c7+c5+c3-c1 */
        tmp13 = tmp10 + tmp11 + tmp12 - MULTIPLY(z1, FIX(1.835_730_603)); /* c9+c11+c13-c15 */
        z1 = MULTIPLY(z2 + z3, FIX(0.138_617_169)); /* c15 */
        tmp1 += z1 + MULTIPLY(z2, FIX(0.071_888_074)); /* c9+c11-c3-c15 */
        tmp2 += z1 - MULTIPLY(z3, FIX(1.125_726_048)); /* c5+c7+c15-c3 */
        z1 = MULTIPLY(z3 - z2, FIX(1.407_403_738)); /* c1 */
        tmp11 += z1 - MULTIPLY(z3, FIX(0.766_367_282)); /* c1+c11-c9-c13 */
        tmp12 += z1 + MULTIPLY(z2, FIX(1.971_951_411)); /* c1+c5+c13-c7 */
        z2 += z4;
        z1 = MULTIPLY(z2, -FIX(0.666_655_658)); /* -c11 */
        tmp1 += z1;
        tmp3 += z1 + MULTIPLY(z4, FIX(1.065_388_962)); /* c3+c11+c15-c7 */
        z2 = MULTIPLY(z2, -FIX(1.247_225_013)); /* -c5 */
        tmp10 += z2 + MULTIPLY(z4, FIX(3.141_271_809)); /* c1+c5+c9-c13 */
        tmp12 += z2;
        z2 = MULTIPLY(z3 + z4, -FIX(1.353_318_001)); /* -c3 */
        tmp2 += z2;
        tmp3 += z2;
        z2 = MULTIPLY(z4 - z3, FIX(0.410_524_528)); /* c13 */
        tmp10 += z2;
        tmp11 += z2;

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 + tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[15] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp20 - tmp0, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 + tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[14] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp21 - tmp1, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 + tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[13] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp22 - tmp2, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 + tmp3, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[12] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp23 - tmp3, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[4] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 + tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[11] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp24 - tmp10, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[5] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 + tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[10] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp25 - tmp11, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[6] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp26 + tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[9] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp26 - tmp12, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[7] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp27 + tmp13, CONST_BITS + PASS1_BITS + 3),
        );
        outptr[8] = range_limit_sample(
            range_limit,
            RIGHT_SHIFT(tmp27 - tmp13, CONST_BITS + PASS1_BITS + 3),
        );
    }
}
