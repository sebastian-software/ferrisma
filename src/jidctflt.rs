// libjpeg-turbo 3.2.1, src/jidctflt.c:
//
// /*
//  * jidctflt.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1998, Thomas G. Lane.
//  * Modified 2010 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2014, 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains a floating-point implementation of the
//  * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
//  * must also perform dequantization of the input coefficients.
//  *
//  * This implementation should be more accurate than either of the integer
//  * IDCT implementations.  However, it may not give the same results on all
//  * machines because of differences in roundoff behavior.  Speed will depend
//  * on the hardware's floating point capacity.
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
//  * The primary disadvantage of this method is that with a fixed-point
//  * implementation, accuracy is lost due to imprecise representation of the
//  * scaled quantization values.  However, that problem does not arise if
//  * we use floating point arithmetic.
//  */
//! Translation of `src/jidctflt.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
//!
//! The 8-bit configuration applies, so the 12-bit `SCALING_FACTOR` and
//! `DEQUANTIZE` are not used. The range limit is `cinfo.sample_range_limit()`,
//! read directly as the C reads `cinfo->sample_range_limit`. Float constants
//! are converted from `double` literals, as the C `(FAST_FLOAT)` casts do, and
//! no float expression is fused or reordered. Float-to-int conversions use
//! `as`, which saturates for out-of-range values where C is undefined.
//! Pointer walks become indexes. `NO_ZERO_ROW_TEST` does not apply: the C file
//! has no zero-row test in pass 2.

use crate::jdct::{FLOAT_MULT_TYPE, RANGE_MASK};
use crate::jmorecfg::{CENTERJSAMPLE, FAST_FLOAT, JCOEF, JDIMENSION, JSAMPLE};
use crate::jpeglib::{
    DCTSIZE, DCTSIZE2, JBLOCK, JSAMPROWS_MUT, jpeg_component_info, jpeg_decompress_struct,
};

// upstream: src/jidctflt.c _0_125
/// `(FLOAT_MULT_TYPE)0.125`, which folds the scale factor of `1/8` into
/// each quantization multiplier.
const _0_125: FLOAT_MULT_TYPE = 0.125;

// upstream: src/jidctflt.c DEQUANTIZE
/// Dequantizes a coefficient by multiplying it by the multiplier-table
/// entry; produces a floating-point result.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
fn DEQUANTIZE(coef: JCOEF, quantval: FLOAT_MULT_TYPE) -> FAST_FLOAT {
    FAST_FLOAT::from(coef) * quantval
}

/// `range_limit[((int)x) & RANGE_MASK]`: the sample for a floating-point
/// IDCT output. `as i32` saturates for out-of-range values, where C's
/// conversion is undefined. Not an upstream item.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversion of a float to int, then a nonnegative masked index"
)]
#[inline]
fn range_limit_sample(range_limit: &[JSAMPLE], x: FAST_FLOAT) -> JSAMPLE {
    range_limit[((x as i32) & RANGE_MASK) as usize]
}

// upstream: src/jidctflt.c _jpeg_idct_float
/// Performs dequantization and inverse DCT on one block of coefficients.
///
/// # Panics
///
/// When the multiplier table is absent, or `output_buf` has fewer than 8 rows
/// or 8 columns from `output_col` on.
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(clippy::similar_names, reason = "C names tmp10, tmp12 and so on")]
#[expect(
    clippy::cast_precision_loss,
    reason = "CENTERJSAMPLE is 128, which is exact as a float"
)]
#[expect(clippy::identity_op, reason = "C's DCTSIZE * k indexing")]
#[expect(clippy::erasing_op, reason = "C's DCTSIZE * k indexing")]
#[expect(clippy::cast_possible_truncation, reason = "C's conversions")]
#[expect(clippy::approx_constant, reason = "C's constants")]
pub fn jpeg_idct_float(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut tmp0: FAST_FLOAT;
    let mut tmp1: FAST_FLOAT;
    let mut tmp2: FAST_FLOAT;
    let mut tmp3: FAST_FLOAT;
    let mut tmp4: FAST_FLOAT;
    let mut tmp5: FAST_FLOAT;
    let mut tmp6: FAST_FLOAT;
    let mut tmp7: FAST_FLOAT;
    let mut tmp10: FAST_FLOAT;
    let mut tmp11: FAST_FLOAT;
    let mut tmp12: FAST_FLOAT;
    let mut tmp13: FAST_FLOAT;
    let mut z5: FAST_FLOAT;
    let mut z10: FAST_FLOAT;
    let mut z11: FAST_FLOAT;
    let mut z12: FAST_FLOAT;
    let mut z13: FAST_FLOAT;
    let range_limit = cinfo.sample_range_limit();
    let mut workspace = [0.0 as FAST_FLOAT; DCTSIZE2]; /* buffers data between passes */

    /* Pass 1: process columns from input, store into work array. */

    let inptr = coef_block;
    let quantptr = compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL");
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
            let dcval = DEQUANTIZE(
                inptr[DCTSIZE * 0 + col],
                quantptr.float_array(DCTSIZE * 0 + col) * _0_125,
            );

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

        tmp0 = DEQUANTIZE(
            inptr[DCTSIZE * 0 + col],
            quantptr.float_array(DCTSIZE * 0 + col) * _0_125,
        );
        tmp1 = DEQUANTIZE(
            inptr[DCTSIZE * 2 + col],
            quantptr.float_array(DCTSIZE * 2 + col) * _0_125,
        );
        tmp2 = DEQUANTIZE(
            inptr[DCTSIZE * 4 + col],
            quantptr.float_array(DCTSIZE * 4 + col) * _0_125,
        );
        tmp3 = DEQUANTIZE(
            inptr[DCTSIZE * 6 + col],
            quantptr.float_array(DCTSIZE * 6 + col) * _0_125,
        );

        tmp10 = tmp0 + tmp2; /* phase 3 */
        tmp11 = tmp0 - tmp2;

        tmp13 = tmp1 + tmp3; /* phases 5-3 */
        tmp12 = (tmp1 - tmp3) * (1.414_213_562_f64 as FAST_FLOAT) - tmp13; /* 2*c4 */

        tmp0 = tmp10 + tmp13; /* phase 2 */
        tmp3 = tmp10 - tmp13;
        tmp1 = tmp11 + tmp12;
        tmp2 = tmp11 - tmp12;

        /* Odd part */

        tmp4 = DEQUANTIZE(
            inptr[DCTSIZE * 1 + col],
            quantptr.float_array(DCTSIZE * 1 + col) * _0_125,
        );
        tmp5 = DEQUANTIZE(
            inptr[DCTSIZE * 3 + col],
            quantptr.float_array(DCTSIZE * 3 + col) * _0_125,
        );
        tmp6 = DEQUANTIZE(
            inptr[DCTSIZE * 5 + col],
            quantptr.float_array(DCTSIZE * 5 + col) * _0_125,
        );
        tmp7 = DEQUANTIZE(
            inptr[DCTSIZE * 7 + col],
            quantptr.float_array(DCTSIZE * 7 + col) * _0_125,
        );

        z13 = tmp6 + tmp5; /* phase 6 */
        z10 = tmp6 - tmp5;
        z11 = tmp4 + tmp7;
        z12 = tmp4 - tmp7;

        tmp7 = z11 + z13; /* phase 5 */
        tmp11 = (z11 - z13) * (1.414_213_562_f64 as FAST_FLOAT); /* 2*c4 */

        z5 = (z10 + z12) * (1.847_759_065_f64 as FAST_FLOAT); /* 2*c2 */
        tmp10 = z5 - z12 * (1.082_392_200_f64 as FAST_FLOAT); /* 2*(c2-c6) */
        tmp12 = z5 - z10 * (2.613_125_930_f64 as FAST_FLOAT); /* 2*(c2+c6) */

        tmp6 = tmp12 - tmp7; /* phase 2 */
        tmp5 = tmp11 - tmp6;
        tmp4 = tmp10 - tmp5;

        wsptr[DCTSIZE * 0 + col] = tmp0 + tmp7;
        wsptr[DCTSIZE * 7 + col] = tmp0 - tmp7;
        wsptr[DCTSIZE * 1 + col] = tmp1 + tmp6;
        wsptr[DCTSIZE * 6 + col] = tmp1 - tmp6;
        wsptr[DCTSIZE * 2 + col] = tmp2 + tmp5;
        wsptr[DCTSIZE * 5 + col] = tmp2 - tmp5;
        wsptr[DCTSIZE * 3 + col] = tmp3 + tmp4;
        wsptr[DCTSIZE * 4 + col] = tmp3 - tmp4;
    }

    /* Pass 2: process rows from work array, store into output array. */

    for ctr in 0..DCTSIZE {
        let wsptr = &workspace[ctr * DCTSIZE..(ctr + 1) * DCTSIZE];
        let outptr = &mut output_buf.row_mut(ctr)[output_col as usize..];
        /* Rows of zeroes can be exploited in the same way as we did with columns.
         * However, the column calculation has created many nonzero AC terms, so
         * the simplification applies less often (typically 5% to 10% of the time).
         * And testing floats for zero is relatively expensive, so we don't bother.
         */

        /* Even part */

        /* Apply signed->unsigned and prepare float->int conversion */
        z5 = wsptr[0] + (CENTERJSAMPLE as FAST_FLOAT + 0.5);
        tmp10 = z5 + wsptr[4];
        tmp11 = z5 - wsptr[4];

        tmp13 = wsptr[2] + wsptr[6];
        tmp12 = (wsptr[2] - wsptr[6]) * (1.414_213_562_f64 as FAST_FLOAT) - tmp13;

        tmp0 = tmp10 + tmp13;
        tmp3 = tmp10 - tmp13;
        tmp1 = tmp11 + tmp12;
        tmp2 = tmp11 - tmp12;

        /* Odd part */

        z13 = wsptr[5] + wsptr[3];
        z10 = wsptr[5] - wsptr[3];
        z11 = wsptr[1] + wsptr[7];
        z12 = wsptr[1] - wsptr[7];

        tmp7 = z11 + z13;
        tmp11 = (z11 - z13) * (1.414_213_562_f64 as FAST_FLOAT);

        z5 = (z10 + z12) * (1.847_759_065_f64 as FAST_FLOAT); /* 2*c2 */
        tmp10 = z5 - z12 * (1.082_392_200_f64 as FAST_FLOAT); /* 2*(c2-c6) */
        tmp12 = z5 - z10 * (2.613_125_930_f64 as FAST_FLOAT); /* 2*(c2+c6) */

        tmp6 = tmp12 - tmp7;
        tmp5 = tmp11 - tmp6;
        tmp4 = tmp10 - tmp5;

        /* Final output stage: float->int conversion and range-limit */

        outptr[0] = range_limit_sample(range_limit, tmp0 + tmp7);
        outptr[7] = range_limit_sample(range_limit, tmp0 - tmp7);
        outptr[1] = range_limit_sample(range_limit, tmp1 + tmp6);
        outptr[6] = range_limit_sample(range_limit, tmp1 - tmp6);
        outptr[2] = range_limit_sample(range_limit, tmp2 + tmp5);
        outptr[5] = range_limit_sample(range_limit, tmp2 - tmp5);
        outptr[3] = range_limit_sample(range_limit, tmp3 + tmp4);
        outptr[4] = range_limit_sample(range_limit, tmp3 - tmp4);

        /* advance pointer to next row */
    }
}
