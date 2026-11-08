// libjpeg-turbo 3.2.1, simd/arm/jdsample-neon.c:
//
// /*
//  * Upsampling (Arm Neon)
//  *
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2020, 2024-2026, D. R. Commander.
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
//! Translation of `simd/arm/jdsample-neon.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! The kernels serve `jdsample.c` through `simd/jsimd.c`'s entry points
//! (`jsimd.rs`), which store them in the `*_upsample_simd` slots of the
//! upsampler. `GETJSAMPLE` is the identity on the 8-bit `JSAMPLE`.
//!
//! A kernel receives the output rows as a row list rather than C's pointer
//! to the upsampler's `color_buf` entry: the kernels only write the rows
//! that `*output_data_ptr` points at, which are the upsampler's own storage
//! from `alloc_sarray`. The h2v2 and h1v2 kernels write two output rows in
//! one iteration and take them at once (`two_rows_mut`); the first of the
//! two then ends where the second begins, at the end of its padded row.
//!
//! Upstream reads up to 16 samples past the last column of an input row and
//! writes whole vectors up to the padding of an output row, relying on
//! `alloc_sarray` padding every row to a multiple of 64 bytes ("Creation of
//! 2-D sample arrays" in `jmemmgr.c`). The loads and stores take arrays from
//! the row slices with checked conversions, which would panic instead of
//! reaching past the storage (ADR 0007, decision 4).

#![forbid(unsafe_code)]

// Clippy does not report wildcard imports in a test build.
use core::arch::aarch64::{
    uint8x8_t, uint8x16_t, uint8x16x2_t, uint16x8_t, vaddq_u16, vcombine_u8, vdup_n_u8,
    vdupq_n_u16, vget_high_u8, vget_low_u8, vmlal_u8, vmlaq_u16, vmovl_u8, vrshrn_n_u16,
    vshrn_n_u16,
};

use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::{JSAMPROWS_AT, JSAMPROWS_MUT};
use crate::neon::{vld1q_u8, vst1q_u8, vst2q_u8};

/* The diagram below shows a row of samples produced by h2v1 downsampling.
 *
 *                s0        s1        s2
 *            +---------+---------+---------+
 *            |         |         |         |
 *            | c0   c1 | c2   c3 | c4   c5 |
 *            |         |         |         |
 *            +---------+---------+---------+
 *
 * Samples s0-s2 were created by averaging the original component values
 * centered at positions c0-c5 above.  To approximate those original component
 * values, we proportionally blend the adjacent samples in each row.
 *
 * An upsampled component value is computed by blending the sample containing
 * the component center with the nearest neighboring sample, in the ratio 3:1.
 * For example:
 *     c1(upsampled) = 3/4 * s0 + 1/4 * s1
 *     c2(upsampled) = 3/4 * s1 + 1/4 * s0
 * When computing the first and last component values in the row, there is no
 * adjacent sample to blend, so:
 *     c0(upsampled) = s0
 *     c5(upsampled) = s2
 */

// upstream: simd/arm/jdsample-neon.c jsimd_h2v1_fancy_upsample_neon
/// Fancy h2v1 upsampling of `max_v_samp_factor` rows of
/// `downsampled_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(clippy::cast_sign_loss, reason = "C's int row counter")]
#[target_feature(enable = "neon")]
pub fn jsimd_h2v1_fancy_upsample_neon(
    max_v_samp_factor: i32,
    downsampled_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    /* Set up constants. */
    let one_u16: uint16x8_t = vdupq_n_u16(1);
    let three_u8: uint8x8_t = vdup_n_u8(3);

    for inrow in 0..max_v_samp_factor {
        let inptr = input_data.row(inrow as isize);
        let outptr = output_data.row_mut(inrow as usize);
        /* First component value in this row of the original image */
        outptr[0] = inptr[0];

        /*    3/4 * containing sample + 1/4 * nearest neighboring sample
         * For c1: containing sample = s0, nearest neighboring sample = s1
         * For c2: containing sample = s1, nearest neighboring sample = s0
         */
        let mut s0: uint8x16_t = vld1q_u8(inptr.first_chunk::<16>().expect("input row too short"));
        let mut s1: uint8x16_t =
            vld1q_u8(inptr[1..].first_chunk::<16>().expect("input row too short"));
        /* Multiplication makes vectors twice as wide.  '_l' and '_h' suffixes
         * denote low half and high half respectively.
         */
        let mut s1_add_3s0_l: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_low_u8(s1)), vget_low_u8(s0), three_u8);
        let mut s1_add_3s0_h: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_high_u8(s1)), vget_high_u8(s0), three_u8);
        let mut s0_add_3s1_l: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_low_u8(s0)), vget_low_u8(s1), three_u8);
        let mut s0_add_3s1_h: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_high_u8(s0)), vget_high_u8(s1), three_u8);
        /* Add ordered dithering bias to odd component values. */
        s0_add_3s1_l = vaddq_u16(s0_add_3s1_l, one_u16);
        s0_add_3s1_h = vaddq_u16(s0_add_3s1_h, one_u16);

        /* The offset is initially 1, because the first component has already been
         * stored.  However, in subsequent iterations of the SIMD loop, this offset
         * is (2 * colctr - 1) to stay within the bounds of the sample buffers
         * without having to resort to a slow scalar tail case for the last
         * (downsampled_width % 16) samples.  See "Creation of 2-D sample arrays"
         * in jmemmgr.c for more details.
         */
        let mut outptr_offset: u32 = 1;
        let mut output_components: uint8x16x2_t;

        /* We use software pipelining to maximise performance.  The code indented
         * an extra two spaces begins the next iteration of the loop.
         */
        let mut colctr: u32 = 16;
        while colctr < downsampled_width {
            s0 = vld1q_u8(
                inptr[(colctr - 1) as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            s1 = vld1q_u8(
                inptr[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );

            /* Right-shift by 2 (divide by 4), narrow to 8-bit, and combine. */
            output_components = uint8x16x2_t(
                vcombine_u8(
                    vrshrn_n_u16::<2>(s1_add_3s0_l),
                    vrshrn_n_u16::<2>(s1_add_3s0_h),
                ),
                vcombine_u8(
                    vshrn_n_u16::<2>(s0_add_3s1_l),
                    vshrn_n_u16::<2>(s0_add_3s1_h),
                ),
            );

            /* Multiplication makes vectors twice as wide.  '_l' and '_h' suffixes
             * denote low half and high half respectively.
             */
            s1_add_3s0_l = vmlal_u8(vmovl_u8(vget_low_u8(s1)), vget_low_u8(s0), three_u8);
            s1_add_3s0_h = vmlal_u8(vmovl_u8(vget_high_u8(s1)), vget_high_u8(s0), three_u8);
            s0_add_3s1_l = vmlal_u8(vmovl_u8(vget_low_u8(s0)), vget_low_u8(s1), three_u8);
            s0_add_3s1_h = vmlal_u8(vmovl_u8(vget_high_u8(s0)), vget_high_u8(s1), three_u8);
            /* Add ordered dithering bias to odd component values. */
            s0_add_3s1_l = vaddq_u16(s0_add_3s1_l, one_u16);
            s0_add_3s1_h = vaddq_u16(s0_add_3s1_h, one_u16);

            /* Store component values to memory. */
            vst2q_u8(
                outptr[outptr_offset as usize..]
                    .first_chunk_mut::<32>()
                    .expect("output row too short"),
                output_components,
            );
            outptr_offset = 2 * colctr - 1;
            colctr += 16;
        }

        /* Complete the last iteration of the loop. */

        /* Right-shift by 2 (divide by 4), narrow to 8-bit, and combine. */
        output_components = uint8x16x2_t(
            vcombine_u8(
                vrshrn_n_u16::<2>(s1_add_3s0_l),
                vrshrn_n_u16::<2>(s1_add_3s0_h),
            ),
            vcombine_u8(
                vshrn_n_u16::<2>(s0_add_3s1_l),
                vshrn_n_u16::<2>(s0_add_3s1_h),
            ),
        );
        /* Store component values to memory. */
        vst2q_u8(
            outptr[outptr_offset as usize..]
                .first_chunk_mut::<32>()
                .expect("output row too short"),
            output_components,
        );

        /* Last component value in this row of the original image */
        outptr[(2 * downsampled_width - 1) as usize] = inptr[(downsampled_width - 1) as usize];
    }
}

/* The diagram below shows an array of samples produced by h2v2 downsampling.
 *
 *                s0        s1        s2
 *            +---------+---------+---------+
 *            | c0   c1 | c2   c3 | c4   c5 |
 *       sA   |         |         |         |
 *            | c6   c7 | c8   c9 | c10  c11|
 *            +---------+---------+---------+
 *            | c12  c13| c14  c15| c16  c17|
 *       sB   |         |         |         |
 *            | c18  c19| c20  c21| c22  c23|
 *            +---------+---------+---------+
 *            | c24  c25| c26  c27| c28  c29|
 *       sC   |         |         |         |
 *            | c30  c31| c32  c33| c34  c35|
 *            +---------+---------+---------+
 *
 * Samples s0A-s2C were created by averaging the original component values
 * centered at positions c0-c35 above.  To approximate one of those original
 * component values, we proportionally blend the sample containing the
 * component center with the nearest neighboring samples in each row, column,
 * and diagonal.
 *
 * An upsampled component value is computed by first blending the sample
 * containing the component center with the nearest neighboring samples in the
 * same column, in the ratio 3:1, and then blending each column sum with the
 * nearest neighboring column sum, in the ratio 3:1.  For example:
 *     c14(upsampled) = 3/4 * (3/4 * s1B + 1/4 * s1A) +
 *                      1/4 * (3/4 * s0B + 1/4 * s0A)
 *                    = 9/16 * s1B + 3/16 * s1A + 3/16 * s0B + 1/16 * s0A
 * When computing the first and last component values in the row, there is no
 * horizontally adjacent sample to blend, so:
 *     c12(upsampled) = 3/4 * s0B + 1/4 * s0A
 *     c23(upsampled) = 3/4 * s2B + 1/4 * s2C
 * When computing the first and last component values in the column, there is
 * no vertically adjacent sample to blend, so:
 *     c2(upsampled) = 3/4 * s1A + 1/4 * s0A
 *     c33(upsampled) = 3/4 * s1C + 1/4 * s2C
 * When computing the corner component values, there is no adjacent sample to
 * blend, so:
 *     c0(upsampled) = s0A
 *     c35(upsampled) = s2C
 */

// upstream: simd/arm/jdsample-neon.c jsimd_h2v2_fancy_upsample_neon
/// Fancy h2v2 upsampling of `max_v_samp_factor` rows from the input rows
/// and the context rows above and below them.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes, or a context row
/// lies outside the input's row list.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions to JSAMPLE and of the int row counter"
)]
#[expect(clippy::too_many_lines, reason = "the C function")]
#[expect(non_snake_case, reason = "C's variable names")]
#[target_feature(enable = "neon")]
pub fn jsimd_h2v2_fancy_upsample_neon(
    max_v_samp_factor: i32,
    downsampled_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    /* Set up constants. */
    let seven_u16: uint16x8_t = vdupq_n_u16(7);
    let three_u8: uint8x8_t = vdup_n_u8(3);
    let three_u16: uint16x8_t = vdupq_n_u16(3);

    let mut inrow: i32 = 0;
    let mut outrow: i32 = 0;
    while outrow < max_v_samp_factor {
        let inptr0 = input_data.row((inrow - 1) as isize);
        let inptr1 = input_data.row(inrow as isize);
        let inptr2 = input_data.row((inrow + 1) as isize);
        /* Suffixes 0 and 1 denote the upper and lower rows of output components,
         * respectively.
         */
        let (outptr0, outptr1) = output_data.two_rows_mut(outrow as usize);
        outrow += 2;

        /* First component value in this row of the original image */
        let s0colsum0: i32 = i32::from(inptr1[0]) * 3 + i32::from(inptr0[0]);
        outptr0[0] = ((s0colsum0 * 4 + 8) >> 4) as JSAMPLE;
        let s0colsum1: i32 = i32::from(inptr1[0]) * 3 + i32::from(inptr2[0]);
        outptr1[0] = ((s0colsum1 * 4 + 8) >> 4) as JSAMPLE;

        /* Step 1: Blend samples vertically in columns s0 and s1.
         * Leave the divide by 4 until the end, when it can be done for both
         * dimensions at once, right-shifting by 4.
         */

        /* Load and compute s0colsum0 and s0colsum1. */
        let mut s0A: uint8x16_t =
            vld1q_u8(inptr0.first_chunk::<16>().expect("input row too short"));
        let mut s0B: uint8x16_t =
            vld1q_u8(inptr1.first_chunk::<16>().expect("input row too short"));
        let mut s0C: uint8x16_t =
            vld1q_u8(inptr2.first_chunk::<16>().expect("input row too short"));
        /* Multiplication makes vectors twice as wide.  '_l' and '_h' suffixes
         * denote low half and high half respectively.
         */
        let mut s0colsum0_l: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_low_u8(s0A)), vget_low_u8(s0B), three_u8);
        let mut s0colsum0_h: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_high_u8(s0A)), vget_high_u8(s0B), three_u8);
        let mut s0colsum1_l: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_low_u8(s0C)), vget_low_u8(s0B), three_u8);
        let mut s0colsum1_h: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_high_u8(s0C)), vget_high_u8(s0B), three_u8);
        /* Load and compute s1colsum0 and s1colsum1. */
        let mut s1A: uint8x16_t = vld1q_u8(
            inptr0[1..]
                .first_chunk::<16>()
                .expect("input row too short"),
        );
        let mut s1B: uint8x16_t = vld1q_u8(
            inptr1[1..]
                .first_chunk::<16>()
                .expect("input row too short"),
        );
        let mut s1C: uint8x16_t = vld1q_u8(
            inptr2[1..]
                .first_chunk::<16>()
                .expect("input row too short"),
        );
        let mut s1colsum0_l: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_low_u8(s1A)), vget_low_u8(s1B), three_u8);
        let mut s1colsum0_h: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_high_u8(s1A)), vget_high_u8(s1B), three_u8);
        let mut s1colsum1_l: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_low_u8(s1C)), vget_low_u8(s1B), three_u8);
        let mut s1colsum1_h: uint16x8_t =
            vmlal_u8(vmovl_u8(vget_high_u8(s1C)), vget_high_u8(s1B), three_u8);

        /* Step 2: Blend the already-blended columns. */

        let mut output0_c1_l: uint16x8_t = vmlaq_u16(s1colsum0_l, s0colsum0_l, three_u16);
        let mut output0_c1_h: uint16x8_t = vmlaq_u16(s1colsum0_h, s0colsum0_h, three_u16);
        let mut output0_c2_l: uint16x8_t = vmlaq_u16(s0colsum0_l, s1colsum0_l, three_u16);
        let mut output0_c2_h: uint16x8_t = vmlaq_u16(s0colsum0_h, s1colsum0_h, three_u16);
        let mut output1_c1_l: uint16x8_t = vmlaq_u16(s1colsum1_l, s0colsum1_l, three_u16);
        let mut output1_c1_h: uint16x8_t = vmlaq_u16(s1colsum1_h, s0colsum1_h, three_u16);
        let mut output1_c2_l: uint16x8_t = vmlaq_u16(s0colsum1_l, s1colsum1_l, three_u16);
        let mut output1_c2_h: uint16x8_t = vmlaq_u16(s0colsum1_h, s1colsum1_h, three_u16);
        /* Add ordered dithering bias to odd component values. */
        output0_c1_l = vaddq_u16(output0_c1_l, seven_u16);
        output0_c1_h = vaddq_u16(output0_c1_h, seven_u16);
        output1_c1_l = vaddq_u16(output1_c1_l, seven_u16);
        output1_c1_h = vaddq_u16(output1_c1_h, seven_u16);
        /* Right-shift by 4 (divide by 16), narrow to 8-bit, and combine. */
        let mut output_components0 = uint8x16x2_t(
            vcombine_u8(
                vshrn_n_u16::<4>(output0_c1_l),
                vshrn_n_u16::<4>(output0_c1_h),
            ),
            vcombine_u8(
                vrshrn_n_u16::<4>(output0_c2_l),
                vrshrn_n_u16::<4>(output0_c2_h),
            ),
        );
        let mut output_components1 = uint8x16x2_t(
            vcombine_u8(
                vshrn_n_u16::<4>(output1_c1_l),
                vshrn_n_u16::<4>(output1_c1_h),
            ),
            vcombine_u8(
                vrshrn_n_u16::<4>(output1_c2_l),
                vrshrn_n_u16::<4>(output1_c2_h),
            ),
        );

        /* Store component values to memory.
         * The minimum size of the output buffer for each row is 64 bytes => no
         * need to worry about buffer overflow here.  See "Creation of 2-D sample
         * arrays" in jmemmgr.c for more details.
         */
        vst2q_u8(
            outptr0[1..]
                .first_chunk_mut::<32>()
                .expect("output row too short"),
            output_components0,
        );
        vst2q_u8(
            outptr1[1..]
                .first_chunk_mut::<32>()
                .expect("output row too short"),
            output_components1,
        );

        /* The first component of the image shifted our loads and stores by one
         * byte.  We have to re-align on a 32-byte boundary at some point before
         * the end of the row (we do it now on the 32/33 component boundary) to
         * stay within the bounds of the sample buffers without having to resort to
         * a slow scalar tail case for the last (downsampled_width % 16) samples.
         * See "Creation of 2-D sample arrays" in jmemmgr.c for more details.
         */
        let mut colctr: u32 = 16;
        while colctr < downsampled_width {
            /* Step 1: Blend samples vertically in columns s0 and s1. */

            /* Load and compute s0colsum0 and s0colsum1. */
            s0A = vld1q_u8(
                inptr0[(colctr - 1) as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            s0B = vld1q_u8(
                inptr1[(colctr - 1) as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            s0C = vld1q_u8(
                inptr2[(colctr - 1) as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            s0colsum0_l = vmlal_u8(vmovl_u8(vget_low_u8(s0A)), vget_low_u8(s0B), three_u8);
            s0colsum0_h = vmlal_u8(vmovl_u8(vget_high_u8(s0A)), vget_high_u8(s0B), three_u8);
            s0colsum1_l = vmlal_u8(vmovl_u8(vget_low_u8(s0C)), vget_low_u8(s0B), three_u8);
            s0colsum1_h = vmlal_u8(vmovl_u8(vget_high_u8(s0C)), vget_high_u8(s0B), three_u8);
            /* Load and compute s1colsum0 and s1colsum1. */
            s1A = vld1q_u8(
                inptr0[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            s1B = vld1q_u8(
                inptr1[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            s1C = vld1q_u8(
                inptr2[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            s1colsum0_l = vmlal_u8(vmovl_u8(vget_low_u8(s1A)), vget_low_u8(s1B), three_u8);
            s1colsum0_h = vmlal_u8(vmovl_u8(vget_high_u8(s1A)), vget_high_u8(s1B), three_u8);
            s1colsum1_l = vmlal_u8(vmovl_u8(vget_low_u8(s1C)), vget_low_u8(s1B), three_u8);
            s1colsum1_h = vmlal_u8(vmovl_u8(vget_high_u8(s1C)), vget_high_u8(s1B), three_u8);

            /* Step 2: Blend the already-blended columns. */

            output0_c1_l = vmlaq_u16(s1colsum0_l, s0colsum0_l, three_u16);
            output0_c1_h = vmlaq_u16(s1colsum0_h, s0colsum0_h, three_u16);
            output0_c2_l = vmlaq_u16(s0colsum0_l, s1colsum0_l, three_u16);
            output0_c2_h = vmlaq_u16(s0colsum0_h, s1colsum0_h, three_u16);
            output1_c1_l = vmlaq_u16(s1colsum1_l, s0colsum1_l, three_u16);
            output1_c1_h = vmlaq_u16(s1colsum1_h, s0colsum1_h, three_u16);
            output1_c2_l = vmlaq_u16(s0colsum1_l, s1colsum1_l, three_u16);
            output1_c2_h = vmlaq_u16(s0colsum1_h, s1colsum1_h, three_u16);
            /* Add ordered dithering bias to odd component values. */
            output0_c1_l = vaddq_u16(output0_c1_l, seven_u16);
            output0_c1_h = vaddq_u16(output0_c1_h, seven_u16);
            output1_c1_l = vaddq_u16(output1_c1_l, seven_u16);
            output1_c1_h = vaddq_u16(output1_c1_h, seven_u16);
            /* Right-shift by 4 (divide by 16), narrow to 8-bit, and combine. */
            output_components0.0 = vcombine_u8(
                vshrn_n_u16::<4>(output0_c1_l),
                vshrn_n_u16::<4>(output0_c1_h),
            );
            output_components0.1 = vcombine_u8(
                vrshrn_n_u16::<4>(output0_c2_l),
                vrshrn_n_u16::<4>(output0_c2_h),
            );
            output_components1.0 = vcombine_u8(
                vshrn_n_u16::<4>(output1_c1_l),
                vshrn_n_u16::<4>(output1_c1_h),
            );
            output_components1.1 = vcombine_u8(
                vrshrn_n_u16::<4>(output1_c2_l),
                vrshrn_n_u16::<4>(output1_c2_h),
            );
            /* Store component values to memory. */
            vst2q_u8(
                outptr0[(2 * colctr - 1) as usize..]
                    .first_chunk_mut::<32>()
                    .expect("output row too short"),
                output_components0,
            );
            vst2q_u8(
                outptr1[(2 * colctr - 1) as usize..]
                    .first_chunk_mut::<32>()
                    .expect("output row too short"),
                output_components1,
            );
            colctr += 16;
        }

        /* Last component value in this row of the original image */
        let s1colsum0: i32 = i32::from(inptr1[(downsampled_width - 1) as usize]) * 3
            + i32::from(inptr0[(downsampled_width - 1) as usize]);
        outptr0[(2 * downsampled_width - 1) as usize] = ((s1colsum0 * 4 + 7) >> 4) as JSAMPLE;
        let s1colsum1: i32 = i32::from(inptr1[(downsampled_width - 1) as usize]) * 3
            + i32::from(inptr2[(downsampled_width - 1) as usize]);
        outptr1[(2 * downsampled_width - 1) as usize] = ((s1colsum1 * 4 + 7) >> 4) as JSAMPLE;
        inrow += 1;
    }
}

/* The diagram below shows a column of samples produced by h1v2 downsampling
 * (or by losslessly rotating or transposing an h2v1-downsampled image.)
 *
 *            +---------+
 *            |   c0    |
 *     sA     |         |
 *            |   c1    |
 *            +---------+
 *            |   c2    |
 *     sB     |         |
 *            |   c3    |
 *            +---------+
 *            |   c4    |
 *     sC     |         |
 *            |   c5    |
 *            +---------+
 *
 * Samples sA-sC were created by averaging the original component values
 * centered at positions c0-c5 above.  To approximate those original component
 * values, we proportionally blend the adjacent samples in each column.
 *
 * An upsampled component value is computed by blending the sample containing
 * the component center with the nearest neighboring sample, in the ratio 3:1.
 * For example:
 *     c1(upsampled) = 3/4 * sA + 1/4 * sB
 *     c2(upsampled) = 3/4 * sB + 1/4 * sA
 * When computing the first and last component values in the column, there is
 * no adjacent sample to blend, so:
 *     c0(upsampled) = sA
 *     c5(upsampled) = sC
 */

// upstream: simd/arm/jdsample-neon.c jsimd_h1v2_fancy_upsample_neon
/// Fancy h1v2 upsampling of `max_v_samp_factor` rows from the input rows
/// and the context rows above and below them.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes, or a context row
/// lies outside the input's row list.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(clippy::cast_sign_loss, reason = "C's int row counter")]
#[expect(non_snake_case, reason = "C's variable names")]
#[target_feature(enable = "neon")]
pub fn jsimd_h1v2_fancy_upsample_neon(
    max_v_samp_factor: i32,
    downsampled_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    /* Set up constants. */
    let one_u16: uint16x8_t = vdupq_n_u16(1);
    let three_u8: uint8x8_t = vdup_n_u8(3);

    let mut inrow: i32 = 0;
    let mut outrow: i32 = 0;
    while outrow < max_v_samp_factor {
        let inptr0 = input_data.row((inrow - 1) as isize);
        let inptr1 = input_data.row(inrow as isize);
        let inptr2 = input_data.row((inrow + 1) as isize);
        /* Suffixes 0 and 1 denote the upper and lower rows of output components,
         * respectively.
         */
        let (outptr0, outptr1) = output_data.two_rows_mut(outrow as usize);
        outrow += 2;
        inrow += 1;

        /* The size of the input and output buffers is always a multiple of 32
         * bytes => no need to worry about buffer overflow when reading/writing
         * memory.  See "Creation of 2-D sample arrays" in jmemmgr.c for more
         * details.
         */
        let mut colctr: u32 = 0;
        while colctr < downsampled_width {
            /* Load samples. */
            let sA: uint8x16_t = vld1q_u8(
                inptr0[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            let sB: uint8x16_t = vld1q_u8(
                inptr1[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            let sC: uint8x16_t = vld1q_u8(
                inptr2[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            /* Blend samples vertically. */
            let mut colsum0_l: uint16x8_t =
                vmlal_u8(vmovl_u8(vget_low_u8(sA)), vget_low_u8(sB), three_u8);
            let mut colsum0_h: uint16x8_t =
                vmlal_u8(vmovl_u8(vget_high_u8(sA)), vget_high_u8(sB), three_u8);
            let colsum1_l: uint16x8_t =
                vmlal_u8(vmovl_u8(vget_low_u8(sC)), vget_low_u8(sB), three_u8);
            let colsum1_h: uint16x8_t =
                vmlal_u8(vmovl_u8(vget_high_u8(sC)), vget_high_u8(sB), three_u8);
            /* Add ordered dithering bias to component values in even output rows. */
            colsum0_l = vaddq_u16(colsum0_l, one_u16);
            colsum0_h = vaddq_u16(colsum0_h, one_u16);
            /* Right-shift by 2 (divide by 4), narrow to 8-bit, and combine. */
            let output_components0: uint8x16_t =
                vcombine_u8(vshrn_n_u16::<2>(colsum0_l), vshrn_n_u16::<2>(colsum0_h));
            let output_components1: uint8x16_t =
                vcombine_u8(vrshrn_n_u16::<2>(colsum1_l), vrshrn_n_u16::<2>(colsum1_h));
            /* Store component values to memory. */
            vst1q_u8(
                outptr0[colctr as usize..]
                    .first_chunk_mut::<16>()
                    .expect("output row too short"),
                output_components0,
            );
            vst1q_u8(
                outptr1[colctr as usize..]
                    .first_chunk_mut::<16>()
                    .expect("output row too short"),
                output_components1,
            );
            colctr += 16;
        }
    }
}

/* The diagram below shows a row of samples produced by h2v1 downsampling.
 *
 *                s0        s1
 *            +---------+---------+
 *            |         |         |
 *            | c0   c1 | c2   c3 |
 *            |         |         |
 *            +---------+---------+
 *
 * Samples s0 and s1 were created by averaging the original component values
 * centered at positions c0-c3 above.  To approximate those original component
 * values, we duplicate the samples horizontally:
 *     c0(upsampled) = c1(upsampled) = s0
 *     c2(upsampled) = c3(upsampled) = s1
 */

// upstream: simd/arm/jdsample-neon.c jsimd_h2v1_upsample_neon
/// Plain h2v1 upsampling of `max_v_samp_factor` rows into rows of
/// `output_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(clippy::cast_sign_loss, reason = "C's int row counter")]
#[target_feature(enable = "neon")]
pub fn jsimd_h2v1_upsample_neon(
    max_v_samp_factor: i32,
    output_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;

    for inrow in 0..max_v_samp_factor {
        let inptr = input_data.row(inrow as isize);
        let outptr = output_data.row_mut(inrow as usize);
        let mut colctr: u32 = 0;
        while 2 * colctr < output_width {
            let samples: uint8x16_t = vld1q_u8(
                inptr[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            /* Duplicate the samples.  The store operation below interleaves them so
             * that adjacent component values take on the same sample value, per
             * above.
             */
            let output_components = uint8x16x2_t(samples, samples);
            /* Store component values to memory.
             * Due to the way sample buffers are allocated, we don't need to worry
             * about tail cases when output_width is not a multiple of 32.  See
             * "Creation of 2-D sample arrays" in jmemmgr.c for details.
             */
            vst2q_u8(
                outptr[(2 * colctr) as usize..]
                    .first_chunk_mut::<32>()
                    .expect("output row too short"),
                output_components,
            );
            colctr += 16;
        }
    }
}

/* The diagram below shows an array of samples produced by h2v2 downsampling.
 *
 *                s0        s1
 *            +---------+---------+
 *            | c0   c1 | c2   c3 |
 *       sA   |         |         |
 *            | c4   c5 | c6   c7 |
 *            +---------+---------+
 *            | c8   c9 | c10  c11|
 *       sB   |         |         |
 *            | c12  c13| c14  c15|
 *            +---------+---------+
 *
 * Samples s0A-s1B were created by averaging the original component values
 * centered at positions c0-c15 above.  To approximate those original component
 * values, we duplicate the samples both horizontally and vertically:
 *     c0(upsampled) = c1(upsampled) = c4(upsampled) = c5(upsampled) = s0A
 *     c2(upsampled) = c3(upsampled) = c6(upsampled) = c7(upsampled) = s1A
 *     c8(upsampled) = c9(upsampled) = c12(upsampled) = c13(upsampled) = s0B
 *     c10(upsampled) = c11(upsampled) = c14(upsampled) = c15(upsampled) = s1B
 */

// upstream: simd/arm/jdsample-neon.c jsimd_h2v2_upsample_neon
/// Plain h2v2 upsampling of `max_v_samp_factor` rows into rows of
/// `output_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[expect(clippy::cast_sign_loss, reason = "C's int row counter")]
#[target_feature(enable = "neon")]
pub fn jsimd_h2v2_upsample_neon(
    max_v_samp_factor: i32,
    output_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;

    let mut inrow: i32 = 0;
    let mut outrow: i32 = 0;
    while outrow < max_v_samp_factor {
        let inptr = input_data.row(inrow as isize);
        let (outptr0, outptr1) = output_data.two_rows_mut(outrow as usize);
        outrow += 2;

        let mut colctr: u32 = 0;
        while 2 * colctr < output_width {
            let samples: uint8x16_t = vld1q_u8(
                inptr[colctr as usize..]
                    .first_chunk::<16>()
                    .expect("input row too short"),
            );
            /* Duplicate the samples.  The store operation below interleaves them so
             * that adjacent component values take on the same sample value, per
             * above.
             */
            let output_components = uint8x16x2_t(samples, samples);
            /* Store component values for both output rows to memory.
             * Due to the way sample buffers are allocated, we don't need to worry
             * about tail cases when output_width is not a multiple of 32.  See
             * "Creation of 2-D sample arrays" in jmemmgr.c for details.
             */
            vst2q_u8(
                outptr0[(2 * colctr) as usize..]
                    .first_chunk_mut::<32>()
                    .expect("output row too short"),
                output_components,
            );
            vst2q_u8(
                outptr1[(2 * colctr) as usize..]
                    .first_chunk_mut::<32>()
                    .expect("output row too short"),
                output_components,
            );
            colctr += 16;
        }
        inrow += 1;
    }
}
