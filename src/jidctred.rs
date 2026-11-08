// libjpeg-turbo 3.2.1, src/jidctred.c:
//
// /*
//  * jidctred.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1998, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015, 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains inverse-DCT routines that produce reduced-size output:
//  * either 4x4, 2x2, or 1x1 samples from an 8x8 DCT block.
//  *
//  * The implementation is based on the Loeffler, Ligtenberg and Moschytz (LL&M)
//  * algorithm used in jidctint.c.  We simply replace each 8-to-8 1-D IDCT step
//  * with an 8-to-4 step that produces the four averages of two adjacent outputs
//  * (or an 8-to-2 step producing two averages of four outputs, for 2x2 output).
//  * These steps were derived by computing the corresponding values at the end
//  * of the normal LL&M code, then simplifying as much as possible.
//  *
//  * 1x1 is trivial: just take the DC coefficient divided by 8.
//  *
//  * See jidctint.c for additional comments.
//  */
//! Translation of `src/jidctred.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
//!
//! The 8-bit configuration applies: `CONST_BITS` 13 and `PASS1_BITS` 2. The
//! whole C file is compiled only when `IDCT_SCALING_SUPPORTED` is defined,
//! which this port assumes. `FIX` is not needed, since `CONST_BITS` is 13 and
//! the pre-computed constants apply. `MULTIPLY` is `MULTIPLY16C16`, and
//! `DESCALE` is the rounding one from `crate::jdct`. `SCALING_FACTOR` declares
//! nothing for 8-bit samples. `NO_ZERO_ROW_TEST` is undefined, so the zero-row
//! tests are kept. Pass 1 walks the columns with `col`; C's `ctr` is
//! `DCTSIZE - col`, so the column skips apply to `col` 4, or 2, 4 and 6.
//! Pointer walks become indexes.

use crate::jdct::{
    DESCALE, IDCT_range_limit, ISLOW_MULT_TYPE, MULTIPLY16C16, RANGE_MASK, promote_multiplier,
};
use crate::jidctint::islow_table;
use crate::jmorecfg::{JCOEF, JDIMENSION, JSAMPLE};
use crate::jpegint::{JLONG, LEFT_SHIFT};
use crate::jpeglib::{DCTSIZE, JBLOCK, JSAMPROWS_MUT, jpeg_component_info, jpeg_decompress_struct};

// upstream: src/jidctred.c CONST_BITS
const CONST_BITS: u32 = 13;
// upstream: src/jidctred.c PASS1_BITS
const PASS1_BITS: u32 = 2;

// upstream: src/jidctred.c FIX_0_211164243
/// `FIX(0.211164243)` with `CONST_BITS` 13.
const FIX_0_211164243: JLONG = 1730;
// upstream: src/jidctred.c FIX_0_509795579
/// `FIX(0.509795579)` with `CONST_BITS` 13.
const FIX_0_509795579: JLONG = 4176;
// upstream: src/jidctred.c FIX_0_601344887
/// `FIX(0.601344887)` with `CONST_BITS` 13.
const FIX_0_601344887: JLONG = 4926;
// upstream: src/jidctred.c FIX_0_720959822
/// `FIX(0.720959822)` with `CONST_BITS` 13.
const FIX_0_720959822: JLONG = 5906;
// upstream: src/jidctred.c FIX_0_765366865
/// `FIX(0.765366865)` with `CONST_BITS` 13.
const FIX_0_765366865: JLONG = 6270;
// upstream: src/jidctred.c FIX_0_850430095
/// `FIX(0.850430095)` with `CONST_BITS` 13.
const FIX_0_850430095: JLONG = 6967;
// upstream: src/jidctred.c FIX_0_899976223
/// `FIX(0.899976223)` with `CONST_BITS` 13.
const FIX_0_899976223: JLONG = 7373;
// upstream: src/jidctred.c FIX_1_061594337
/// `FIX(1.061594337)` with `CONST_BITS` 13.
const FIX_1_061594337: JLONG = 8697;
// upstream: src/jidctred.c FIX_1_272758580
/// `FIX(1.272758580)` with `CONST_BITS` 13.
const FIX_1_272758580: JLONG = 10426;
// upstream: src/jidctred.c FIX_1_451774981
/// `FIX(1.451774981)` with `CONST_BITS` 13.
const FIX_1_451774981: JLONG = 11893;
// upstream: src/jidctred.c FIX_1_847759065
/// `FIX(1.847759065)` with `CONST_BITS` 13.
const FIX_1_847759065: JLONG = 15137;
// upstream: src/jidctred.c FIX_2_172734803
/// `FIX(2.172734803)` with `CONST_BITS` 13.
const FIX_2_172734803: JLONG = 17799;
// upstream: src/jidctred.c FIX_2_562915447
/// `FIX(2.562915447)` with `CONST_BITS` 13.
const FIX_2_562915447: JLONG = 20995;
// upstream: src/jidctred.c FIX_3_624509785
/// `FIX(3.624509785)` with `CONST_BITS` 13.
const FIX_3_624509785: JLONG = 29692;

// upstream: src/jidctred.c MULTIPLY
/// Multiplies a `JLONG` variable by a `JLONG` constant to yield a `JLONG`
/// result.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn MULTIPLY(var: JLONG, const_: JLONG) -> JLONG {
    MULTIPLY16C16(var, const_)
}

// upstream: src/jidctred.c DEQUANTIZE
/// Dequantizes a coefficient by multiplying it by the multiplier-table
/// entry; produces an `int` result, widened to `JLONG` as every use assigns
/// it. `(ISLOW_MULT_TYPE)coef` is `coef` in both widths of
/// `ISLOW_MULT_TYPE`, and C multiplies in `int`.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn DEQUANTIZE(coef: JCOEF, quantval: ISLOW_MULT_TYPE) -> JLONG {
    JLONG::from(i32::from(coef) * promote_multiplier(quantval))
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

// upstream: src/jidctred.c _jpeg_idct_4x4
/// Performs dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 4x4 output block.
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 4 rows or 4 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(clippy::similar_names, reason = "C names tmp10, tmp12 and so on")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(clippy::identity_op, reason = "C's DCTSIZE * k indexing")]
#[expect(clippy::erasing_op, reason = "C's DCTSIZE * k indexing")]
pub fn jpeg_idct_4x4(
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
    let mut z1: JLONG;
    let mut z2: JLONG;
    let mut z3: JLONG;
    let mut z4: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; DCTSIZE * 4]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    let wsptr = &mut workspace;
    for col in 0..DCTSIZE {
        /* Don't bother to process column 4, because second pass won't use it */
        /* (C's ctr is DCTSIZE - col, so ctr == DCTSIZE - 4 is col == 4) */
        if col == 4 {
            continue;
        }
        if inptr[DCTSIZE * 1 + col] == 0
            && inptr[DCTSIZE * 2 + col] == 0
            && inptr[DCTSIZE * 3 + col] == 0
            && inptr[DCTSIZE * 5 + col] == 0
            && inptr[DCTSIZE * 6 + col] == 0
            && inptr[DCTSIZE * 7 + col] == 0
        {
            /* AC terms all zero; we need not examine term 4 for 4x4 output */
            let dcval = LEFT_SHIFT(
                DEQUANTIZE(inptr[DCTSIZE * 0 + col], quantptr[DCTSIZE * 0 + col]),
                PASS1_BITS,
            ) as i32;

            wsptr[DCTSIZE * 0 + col] = dcval;
            wsptr[DCTSIZE * 1 + col] = dcval;
            wsptr[DCTSIZE * 2 + col] = dcval;
            wsptr[DCTSIZE * 3 + col] = dcval;

            continue;
        }

        /* Even part */

        tmp0 = DEQUANTIZE(inptr[DCTSIZE * 0 + col], quantptr[DCTSIZE * 0 + col]);
        tmp0 = LEFT_SHIFT(tmp0, CONST_BITS + 1);

        z2 = DEQUANTIZE(inptr[DCTSIZE * 2 + col], quantptr[DCTSIZE * 2 + col]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 6 + col], quantptr[DCTSIZE * 6 + col]);

        tmp2 = MULTIPLY(z2, FIX_1_847759065) + MULTIPLY(z3, -FIX_0_765366865);

        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 7 + col], quantptr[DCTSIZE * 7 + col]);
        z2 = DEQUANTIZE(inptr[DCTSIZE * 5 + col], quantptr[DCTSIZE * 5 + col]);
        z3 = DEQUANTIZE(inptr[DCTSIZE * 3 + col], quantptr[DCTSIZE * 3 + col]);
        z4 = DEQUANTIZE(inptr[DCTSIZE * 1 + col], quantptr[DCTSIZE * 1 + col]);

        tmp0 = MULTIPLY(z1, -FIX_0_211164243) + /* sqrt(2) * ( c3-c1) */
            MULTIPLY(z2, FIX_1_451774981) + /* sqrt(2) * ( c3+c7) */
            MULTIPLY(z3, -FIX_2_172734803) + /* sqrt(2) * (-c1-c5) */
            MULTIPLY(z4, FIX_1_061594337); /* sqrt(2) * ( c5+c7) */

        tmp2 = MULTIPLY(z1, -FIX_0_509795579) + /* sqrt(2) * (c7-c5) */
            MULTIPLY(z2, -FIX_0_601344887) + /* sqrt(2) * (c5-c1) */
            MULTIPLY(z3, FIX_0_899976223) + /* sqrt(2) * (c3-c7) */
            MULTIPLY(z4, FIX_2_562915447); /* sqrt(2) * (c1+c3) */

        /* Final output stage */

        wsptr[DCTSIZE * 0 + col] = DESCALE(tmp10 + tmp2, CONST_BITS - PASS1_BITS + 1) as i32;
        wsptr[DCTSIZE * 3 + col] = DESCALE(tmp10 - tmp2, CONST_BITS - PASS1_BITS + 1) as i32;
        wsptr[DCTSIZE * 1 + col] = DESCALE(tmp12 + tmp0, CONST_BITS - PASS1_BITS + 1) as i32;
        wsptr[DCTSIZE * 2 + col] = DESCALE(tmp12 - tmp0, CONST_BITS - PASS1_BITS + 1) as i32;
    }

    /* Pass 2: process 4 rows from work array, store into output array. */

    for ctr in 0..4 {
        let wsptr = &workspace[ctr * DCTSIZE..(ctr + 1) * DCTSIZE];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];
        /* It's not clear whether a zero row test is worthwhile here ... */

        /* NO_ZERO_ROW_TEST is undefined, so the test is compiled in. */
        if wsptr[1] == 0
            && wsptr[2] == 0
            && wsptr[3] == 0
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

            continue; /* advance pointer to next row */
        }

        /* Even part */

        tmp0 = LEFT_SHIFT(JLONG::from(wsptr[0]), CONST_BITS + 1);

        tmp2 = MULTIPLY(JLONG::from(wsptr[2]), FIX_1_847759065)
            + MULTIPLY(JLONG::from(wsptr[6]), -FIX_0_765366865);

        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;

        /* Odd part */

        z1 = JLONG::from(wsptr[7]);
        z2 = JLONG::from(wsptr[5]);
        z3 = JLONG::from(wsptr[3]);
        z4 = JLONG::from(wsptr[1]);

        tmp0 = MULTIPLY(z1, -FIX_0_211164243) + /* sqrt(2) * ( c3-c1) */
            MULTIPLY(z2, FIX_1_451774981) + /* sqrt(2) * ( c3+c7) */
            MULTIPLY(z3, -FIX_2_172734803) + /* sqrt(2) * (-c1-c5) */
            MULTIPLY(z4, FIX_1_061594337); /* sqrt(2) * ( c5+c7) */

        tmp2 = MULTIPLY(z1, -FIX_0_509795579) + /* sqrt(2) * (c7-c5) */
            MULTIPLY(z2, -FIX_0_601344887) + /* sqrt(2) * (c5-c1) */
            MULTIPLY(z3, FIX_0_899976223) + /* sqrt(2) * (c3-c7) */
            MULTIPLY(z4, FIX_2_562915447); /* sqrt(2) * (c1+c3) */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            DESCALE(tmp10 + tmp2, CONST_BITS + PASS1_BITS + 3 + 1),
        );
        outptr[3] = range_limit_sample(
            range_limit,
            DESCALE(tmp10 - tmp2, CONST_BITS + PASS1_BITS + 3 + 1),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            DESCALE(tmp12 + tmp0, CONST_BITS + PASS1_BITS + 3 + 1),
        );
        outptr[2] = range_limit_sample(
            range_limit,
            DESCALE(tmp12 - tmp0, CONST_BITS + PASS1_BITS + 3 + 1),
        );

        /* advance pointer to next row */
    }
}

// upstream: src/jidctred.c _jpeg_idct_2x2
/// Performs dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 2x2 output block.
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has fewer than 2 rows or 2 columns from `output_col` on.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
#[expect(clippy::similar_names, reason = "C names tmp10, tmp12 and so on")]
#[expect(clippy::identity_op, reason = "C's DCTSIZE * k indexing")]
#[expect(clippy::erasing_op, reason = "C's DCTSIZE * k indexing")]
pub fn jpeg_idct_2x2(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: JLONG;
    let mut tmp10: JLONG;
    let mut z1: JLONG;
    let range_limit = IDCT_range_limit(cinfo);
    let mut workspace = [0i32; DCTSIZE * 2]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = islow_table(compptr);
    let wsptr = &mut workspace;
    for col in 0..DCTSIZE {
        /* Don't bother to process columns 2,4,6 */
        /* (C's ctr is DCTSIZE - col, so ctr == 6, 4, 2 is col == 2, 4, 6) */
        if col == 2 || col == 4 || col == 6 {
            continue;
        }
        if inptr[DCTSIZE * 1 + col] == 0
            && inptr[DCTSIZE * 3 + col] == 0
            && inptr[DCTSIZE * 5 + col] == 0
            && inptr[DCTSIZE * 7 + col] == 0
        {
            /* AC terms all zero; we need not examine terms 2,4,6 for 2x2 output */
            let dcval = LEFT_SHIFT(
                DEQUANTIZE(inptr[DCTSIZE * 0 + col], quantptr[DCTSIZE * 0 + col]),
                PASS1_BITS,
            ) as i32;

            wsptr[DCTSIZE * 0 + col] = dcval;
            wsptr[DCTSIZE * 1 + col] = dcval;

            continue;
        }

        /* Even part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 0 + col], quantptr[DCTSIZE * 0 + col]);
        tmp10 = LEFT_SHIFT(z1, CONST_BITS + 2);

        /* Odd part */

        z1 = DEQUANTIZE(inptr[DCTSIZE * 7 + col], quantptr[DCTSIZE * 7 + col]);
        tmp0 = MULTIPLY(z1, -FIX_0_720959822); /* sqrt(2) * ( c7-c5+c3-c1) */
        z1 = DEQUANTIZE(inptr[DCTSIZE * 5 + col], quantptr[DCTSIZE * 5 + col]);
        tmp0 += MULTIPLY(z1, FIX_0_850430095); /* sqrt(2) * (-c1+c3+c5+c7) */
        z1 = DEQUANTIZE(inptr[DCTSIZE * 3 + col], quantptr[DCTSIZE * 3 + col]);
        tmp0 += MULTIPLY(z1, -FIX_1_272758580); /* sqrt(2) * (-c1+c3-c5-c7) */
        z1 = DEQUANTIZE(inptr[DCTSIZE * 1 + col], quantptr[DCTSIZE * 1 + col]);
        tmp0 += MULTIPLY(z1, FIX_3_624509785); /* sqrt(2) * ( c1+c3+c5+c7) */

        /* Final output stage */

        wsptr[DCTSIZE * 0 + col] = DESCALE(tmp10 + tmp0, CONST_BITS - PASS1_BITS + 2) as i32;
        wsptr[DCTSIZE * 1 + col] = DESCALE(tmp10 - tmp0, CONST_BITS - PASS1_BITS + 2) as i32;
    }

    /* Pass 2: process 2 rows from work array, store into output array. */

    for ctr in 0..2 {
        let wsptr = &workspace[ctr * DCTSIZE..(ctr + 1) * DCTSIZE];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];
        /* It's not clear whether a zero row test is worthwhile here ... */

        /* NO_ZERO_ROW_TEST is undefined, so the test is compiled in. */
        if wsptr[1] == 0 && wsptr[3] == 0 && wsptr[5] == 0 && wsptr[7] == 0 {
            /* AC terms all zero */
            let dcval =
                range_limit_sample(range_limit, DESCALE(JLONG::from(wsptr[0]), PASS1_BITS + 3));

            outptr[0] = dcval;
            outptr[1] = dcval;

            continue; /* advance pointer to next row */
        }

        /* Even part */

        tmp10 = LEFT_SHIFT(JLONG::from(wsptr[0]), CONST_BITS + 2);

        /* Odd part */

        tmp0 = MULTIPLY(JLONG::from(wsptr[7]), -FIX_0_720959822) /* sqrt(2) * ( c7-c5+c3-c1) */
            + MULTIPLY(JLONG::from(wsptr[5]), FIX_0_850430095) /* sqrt(2) * (-c1+c3+c5+c7) */
            + MULTIPLY(JLONG::from(wsptr[3]), -FIX_1_272758580) /* sqrt(2) * (-c1+c3-c5-c7) */
            + MULTIPLY(JLONG::from(wsptr[1]), FIX_3_624509785); /* sqrt(2) * ( c1+c3+c5+c7) */

        /* Final output stage */

        outptr[0] = range_limit_sample(
            range_limit,
            DESCALE(tmp10 + tmp0, CONST_BITS + PASS1_BITS + 3 + 2),
        );
        outptr[1] = range_limit_sample(
            range_limit,
            DESCALE(tmp10 - tmp0, CONST_BITS + PASS1_BITS + 3 + 2),
        );

        /* advance pointer to next row */
    }
}

// upstream: src/jidctred.c _jpeg_idct_1x1
/// Performs dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 1x1 output block.
///
/// # Panics
///
/// When the multiplier table or the range-limit table is absent, or
/// `output_buf` has no row 0 or no column `output_col`.
#[expect(clippy::cast_possible_truncation, reason = "C's conversions to int")]
pub fn jpeg_idct_1x1(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let range_limit = IDCT_range_limit(cinfo);

    /* We hardly need an inverse DCT routine for this: just take the
     * average sample value, which is one-eighth of the DC coefficient.
     */
    let quantptr = islow_table(compptr);
    let mut dcval = DEQUANTIZE(coef_block[0], quantptr[0]) as i32;
    dcval = DESCALE(JLONG::from(dcval), 3) as i32;

    output_buf.row_mut(0)[output_col as usize] =
        range_limit_sample(range_limit, JLONG::from(dcval));
}
