// libjpeg-turbo 3.2.1, simd/arm/jdmrgext-neon.c:
//
// /*
//  * Merged Upsampling/Color Conversion (Arm Neon)
//  *
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2020, 2025, D. R. Commander.
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
//! Translation of `simd/arm/jdmrgext-neon.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! `jdmerge-neon.c` includes this file once per RGB pixel layout, with
//! `RGB_RED`, `RGB_GREEN`, `RGB_BLUE`, `RGB_PIXELSIZE` and possibly
//! `RGB_ALPHA` defined, and with the functions renamed. Each kernel is a
//! function generic over those values, and `jdmerge_neon.rs` names the
//! instances as C does; `RGB_ALPHA` is -1 where C leaves it undefined, so
//! `#ifdef RGB_ALPHA` is `RGB_ALPHA >= 0`, as in `jdmrgext.rs`.
//!
//! The kernels take their input rows as slices and their output rows as
//! separate slices, as `jdmrgext.rs` does, because the second row of the 2:1
//! vertical case may be the upsampler's spare row. A pointer increment is a
//! reslice, and `jdcolext_neon`'s [`input`] and [`output`] make the arrays
//! that the loads and stores of `neon` take. The input rows extend to the end
//! of their storage, which covers upstream's over-reads into the
//! `alloc_sarray` padding; the stores stay within `output_width`, as
//! upstream's notes require. C's assignments to the `val` array of a Neon
//! multi-vector type become `jdcolext_neon`'s [`rgb_val`] and [`rgba_val`];
//! where the 2:1 vertical kernel computes the assigned vectors, each is
//! first a local, so that the two rows' vectors are computed in C's
//! interleaved order. The
//! fall-through `switch` of the tail is a chain of `if`s, one per `case`, as
//! `cols_remaining` is below 16 there; the `case 8` store, which ends with
//! `break`, closes the chain of cases 15 to 8.

#![forbid(unsafe_code)]

use core::arch::aarch64::{
    int16x4_t, int16x8_t, int32x4_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t, uint8x8x4_t,
    uint8x16x3_t, uint8x16x4_t, vaddw_u8, vcombine_s16, vcombine_u8, vdup_n_u8, vdupq_n_s16,
    vdupq_n_u8, vget_high_s16, vget_low_s16, vmlsl_lane_s16, vmull_lane_s16, vqmovun_s16,
    vqrdmulhq_lane_s16, vreinterpretq_s16_u16, vreinterpretq_u16_s16, vrshrn_n_s32, vshlq_n_s16,
    vzip_u8,
};

use crate::jdcolext_neon::{input, output, rgb_val, rgba_val};
use crate::jdmerge_neon::jsimd_ycc_rgb_convert_neon_consts;
use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::JSAMPROWS_AT;
use crate::neon::{
    vld1_s16, vld1_u8, vld2_u8, vst3_lane_u8, vst3_u8, vst3q_u8, vst4_lane_u8, vst4_u8, vst4q_u8,
};

/* These routines combine simple (non-fancy, i.e. non-smooth) h2v1 or h2v2
 * chroma upsampling and YCbCr -> RGB color conversion into a single function.
 *
 * As with the standalone functions, YCbCr -> RGB conversion is defined by the
 * following equations:
 *    R = Y                        + 1.40200 * (Cr - 128)
 *    G = Y - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128)
 *    B = Y + 1.77200 * (Cb - 128)
 *
 * Scaled integer constants are used to avoid floating-point arithmetic:
 *    0.3441467 = 11277 * 2^-15
 *    0.7141418 = 23401 * 2^-15
 *    1.4020386 = 22971 * 2^-14
 *    1.7720337 = 29033 * 2^-14
 * These constants are defined in jdmerge-neon.c.
 *
 * To ensure correct results, rounding is used when descaling.
 */

/* Notes on safe memory access for merged upsampling/YCbCr -> RGB conversion
 * routines:
 *
 * Input memory buffers can be safely overread up to the next multiple of
 * ALIGN_SIZE bytes, since they are always allocated by alloc_sarray() in
 * jmemmgr.c.
 *
 * The output buffer cannot safely be written beyond output_width, since
 * output_buf points to a possibly unpadded row in the decompressed image
 * buffer allocated by the calling program.
 */

/* Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
 */

// upstream: simd/arm/jdmrgext-neon.c jsimd_h2v1_merged_upsample_neon
/// Upsamples and color-converts the row group `in_row_group_ctr` of
/// `input_buf` into the row `output_buf[0]` of `output_width` pixels in the
/// pixel layout of the parameters.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
///
/// # Panics
///
/// When a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_wrap,
    reason = "C's int cols_remaining = output_width and the row index"
)]
#[expect(clippy::cast_sign_loss, reason = "C's conversions of the pixel size")]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[target_feature(enable = "neon")]
pub fn jsimd_h2v1_merged_upsample_neon<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_ALPHA: i32,
    const RGB_PIXELSIZE: i32,
>(
    output_width: JDIMENSION,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    let consts: int16x4_t = vld1_s16(&jsimd_ycc_rgb_convert_neon_consts);
    let neg_128: int16x8_t = vdupq_n_s16(-128);

    /* Pointers to Y, Cb, and Cr data */
    let mut inptr0 = input_buf[0].row(in_row_group_ctr as isize);
    let mut inptr1 = input_buf[1].row(in_row_group_ctr as isize);
    let mut inptr2 = input_buf[2].row(in_row_group_ctr as isize);
    let mut outptr: &mut [JSAMPLE] = &mut *output_buf[0];

    let mut cols_remaining = output_width as i32;
    while cols_remaining >= 16 {
        /* De-interleave Y component values into two separate vectors, one
         * containing the component values with even-numbered indices and one
         * containing the component values with odd-numbered indices.
         */
        let y: uint8x8x2_t = vld2_u8(input(inptr0));
        let cb: uint8x8_t = vld1_u8(input(inptr1));
        let cr: uint8x8_t = vld1_u8(input(inptr2));
        /* Subtract 128 from Cb and Cr. */
        let cr_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cr));
        let cb_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cb));
        /* Compute G-Y: - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128) */
        let mut g_sub_y_l: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128), consts);
        let mut g_sub_y_h: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128), consts);
        g_sub_y_l = vmlsl_lane_s16::<1>(g_sub_y_l, vget_low_s16(cr_128), consts);
        g_sub_y_h = vmlsl_lane_s16::<1>(g_sub_y_h, vget_high_s16(cr_128), consts);
        /* Descale G components: shift right 15, round, and narrow to 16-bit. */
        let g_sub_y: int16x8_t =
            vcombine_s16(vrshrn_n_s32::<15>(g_sub_y_l), vrshrn_n_s32::<15>(g_sub_y_h));
        /* Compute R-Y: 1.40200 * (Cr - 128) */
        let r_sub_y: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128), consts);
        /* Compute B-Y: 1.77200 * (Cb - 128) */
        let b_sub_y: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128), consts);
        /* Add the chroma-derived values (G-Y, R-Y, and B-Y) to both the "even" and
         * "odd" Y component values.  This effectively upsamples the chroma
         * components horizontally.
         */
        let g_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y.0));
        let r_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y.0));
        let b_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y.0));
        let g_odd: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y.1));
        let r_odd: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y.1));
        let b_odd: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y.1));
        /* Convert each component to unsigned and narrow, clamping to [0-255].
         * Re-interleave the "even" and "odd" component values.
         */
        let r: uint8x8x2_t = vzip_u8(vqmovun_s16(r_even), vqmovun_s16(r_odd));
        let g: uint8x8x2_t = vzip_u8(vqmovun_s16(g_even), vqmovun_s16(g_odd));
        let b: uint8x8x2_t = vzip_u8(vqmovun_s16(b_even), vqmovun_s16(b_odd));

        if RGB_ALPHA >= 0 {
            let rgba = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                vcombine_u8(r.0, r.1),
                vcombine_u8(g.0, g.1),
                vcombine_u8(b.0, b.1),
                /* Set alpha channel to opaque (0xFF). */
                vdupq_n_u8(0xFF),
            );
            let rgba = uint8x16x4_t(rgba[0], rgba[1], rgba[2], rgba[3]);
            /* Store RGBA pixel data to memory. */
            vst4q_u8(output(outptr, 0), rgba);
        } else {
            let rgb = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(
                vcombine_u8(r.0, r.1),
                vcombine_u8(g.0, g.1),
                vcombine_u8(b.0, b.1),
            );
            let rgb = uint8x16x3_t(rgb[0], rgb[1], rgb[2]);
            /* Store RGB pixel data to memory. */
            vst3q_u8(output(outptr, 0), rgb);
        }

        /* Increment pointers. */
        inptr0 = &inptr0[16..];
        inptr1 = &inptr1[8..];
        inptr2 = &inptr2[8..];
        outptr = &mut outptr[RGB_PIXELSIZE as usize * 16..];
        cols_remaining -= 16;
    }

    if cols_remaining > 0 {
        /* De-interleave Y component values into two separate vectors, one
         * containing the component values with even-numbered indices and one
         * containing the component values with odd-numbered indices.
         */
        let y: uint8x8x2_t = vld2_u8(input(inptr0));
        let cb: uint8x8_t = vld1_u8(input(inptr1));
        let cr: uint8x8_t = vld1_u8(input(inptr2));
        /* Subtract 128 from Cb and Cr. */
        let cr_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cr));
        let cb_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cb));
        /* Compute G-Y: - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128) */
        let mut g_sub_y_l: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128), consts);
        let mut g_sub_y_h: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128), consts);
        g_sub_y_l = vmlsl_lane_s16::<1>(g_sub_y_l, vget_low_s16(cr_128), consts);
        g_sub_y_h = vmlsl_lane_s16::<1>(g_sub_y_h, vget_high_s16(cr_128), consts);
        /* Descale G components: shift right 15, round, and narrow to 16-bit. */
        let g_sub_y: int16x8_t =
            vcombine_s16(vrshrn_n_s32::<15>(g_sub_y_l), vrshrn_n_s32::<15>(g_sub_y_h));
        /* Compute R-Y: 1.40200 * (Cr - 128) */
        let r_sub_y: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128), consts);
        /* Compute B-Y: 1.77200 * (Cb - 128) */
        let b_sub_y: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128), consts);
        /* Add the chroma-derived values (G-Y, R-Y, and B-Y) to both the "even" and
         * "odd" Y component values.  This effectively upsamples the chroma
         * components horizontally.
         */
        let g_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y.0));
        let r_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y.0));
        let b_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y.0));
        let g_odd: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y.1));
        let r_odd: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y.1));
        let b_odd: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y.1));
        /* Convert each component to unsigned and narrow, clamping to [0-255].
         * Re-interleave the "even" and "odd" component values.
         */
        let r: uint8x8x2_t = vzip_u8(vqmovun_s16(r_even), vqmovun_s16(r_odd));
        let g: uint8x8x2_t = vzip_u8(vqmovun_s16(g_even), vqmovun_s16(g_odd));
        let b: uint8x8x2_t = vzip_u8(vqmovun_s16(b_even), vqmovun_s16(b_odd));

        if RGB_ALPHA >= 0 {
            let rgba_h = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                r.1,
                g.1,
                b.1,
                /* Set alpha channel to opaque (0xFF). */
                vdup_n_u8(0xFF),
            );
            let rgba_h = uint8x8x4_t(rgba_h[0], rgba_h[1], rgba_h[2], rgba_h[3]);
            let rgba_l = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                r.0,
                g.0,
                b.0,
                /* Set alpha channel to opaque (0xFF). */
                vdup_n_u8(0xFF),
            );
            let rgba_l = uint8x8x4_t(rgba_l[0], rgba_l[1], rgba_l[2], rgba_l[3]);
            /* Store RGBA pixel data to memory. */
            if cols_remaining >= 8 {
                if cols_remaining >= 15 {
                    vst4_lane_u8::<6>(output(outptr, 14 * RGB_PIXELSIZE as usize), rgba_h);
                }
                if cols_remaining >= 14 {
                    vst4_lane_u8::<5>(output(outptr, 13 * RGB_PIXELSIZE as usize), rgba_h);
                }
                if cols_remaining >= 13 {
                    vst4_lane_u8::<4>(output(outptr, 12 * RGB_PIXELSIZE as usize), rgba_h);
                }
                if cols_remaining >= 12 {
                    vst4_lane_u8::<3>(output(outptr, 11 * RGB_PIXELSIZE as usize), rgba_h);
                }
                if cols_remaining >= 11 {
                    vst4_lane_u8::<2>(output(outptr, 10 * RGB_PIXELSIZE as usize), rgba_h);
                }
                if cols_remaining >= 10 {
                    vst4_lane_u8::<1>(output(outptr, 9 * RGB_PIXELSIZE as usize), rgba_h);
                }
                if cols_remaining >= 9 {
                    vst4_lane_u8::<0>(output(outptr, 8 * RGB_PIXELSIZE as usize), rgba_h);
                }
                vst4_u8(output(outptr, 0), rgba_l);
            } else {
                if cols_remaining >= 7 {
                    vst4_lane_u8::<6>(output(outptr, 6 * RGB_PIXELSIZE as usize), rgba_l);
                }
                if cols_remaining >= 6 {
                    vst4_lane_u8::<5>(output(outptr, 5 * RGB_PIXELSIZE as usize), rgba_l);
                }
                if cols_remaining >= 5 {
                    vst4_lane_u8::<4>(output(outptr, 4 * RGB_PIXELSIZE as usize), rgba_l);
                }
                if cols_remaining >= 4 {
                    vst4_lane_u8::<3>(output(outptr, 3 * RGB_PIXELSIZE as usize), rgba_l);
                }
                if cols_remaining >= 3 {
                    vst4_lane_u8::<2>(output(outptr, 2 * RGB_PIXELSIZE as usize), rgba_l);
                }
                if cols_remaining >= 2 {
                    vst4_lane_u8::<1>(output(outptr, RGB_PIXELSIZE as usize), rgba_l);
                }
                if cols_remaining >= 1 {
                    vst4_lane_u8::<0>(output(outptr, 0), rgba_l);
                }
            }
        } else {
            let rgb_h = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(r.1, g.1, b.1);
            let rgb_h = uint8x8x3_t(rgb_h[0], rgb_h[1], rgb_h[2]);
            let rgb_l = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(r.0, g.0, b.0);
            let rgb_l = uint8x8x3_t(rgb_l[0], rgb_l[1], rgb_l[2]);
            /* Store RGB pixel data to memory. */
            if cols_remaining >= 8 {
                if cols_remaining >= 15 {
                    vst3_lane_u8::<6>(output(outptr, 14 * RGB_PIXELSIZE as usize), rgb_h);
                }
                if cols_remaining >= 14 {
                    vst3_lane_u8::<5>(output(outptr, 13 * RGB_PIXELSIZE as usize), rgb_h);
                }
                if cols_remaining >= 13 {
                    vst3_lane_u8::<4>(output(outptr, 12 * RGB_PIXELSIZE as usize), rgb_h);
                }
                if cols_remaining >= 12 {
                    vst3_lane_u8::<3>(output(outptr, 11 * RGB_PIXELSIZE as usize), rgb_h);
                }
                if cols_remaining >= 11 {
                    vst3_lane_u8::<2>(output(outptr, 10 * RGB_PIXELSIZE as usize), rgb_h);
                }
                if cols_remaining >= 10 {
                    vst3_lane_u8::<1>(output(outptr, 9 * RGB_PIXELSIZE as usize), rgb_h);
                }
                if cols_remaining >= 9 {
                    vst3_lane_u8::<0>(output(outptr, 8 * RGB_PIXELSIZE as usize), rgb_h);
                }
                vst3_u8(output(outptr, 0), rgb_l);
            } else {
                if cols_remaining >= 7 {
                    vst3_lane_u8::<6>(output(outptr, 6 * RGB_PIXELSIZE as usize), rgb_l);
                }
                if cols_remaining >= 6 {
                    vst3_lane_u8::<5>(output(outptr, 5 * RGB_PIXELSIZE as usize), rgb_l);
                }
                if cols_remaining >= 5 {
                    vst3_lane_u8::<4>(output(outptr, 4 * RGB_PIXELSIZE as usize), rgb_l);
                }
                if cols_remaining >= 4 {
                    vst3_lane_u8::<3>(output(outptr, 3 * RGB_PIXELSIZE as usize), rgb_l);
                }
                if cols_remaining >= 3 {
                    vst3_lane_u8::<2>(output(outptr, 2 * RGB_PIXELSIZE as usize), rgb_l);
                }
                if cols_remaining >= 2 {
                    vst3_lane_u8::<1>(output(outptr, RGB_PIXELSIZE as usize), rgb_l);
                }
                if cols_remaining >= 1 {
                    vst3_lane_u8::<0>(output(outptr, 0), rgb_l);
                }
            }
        }
    }
}

/* Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
 *
 * See comments above for details regarding color conversion and safe memory
 * access.
 */

// upstream: simd/arm/jdmrgext-neon.c jsimd_h2v2_merged_upsample_neon
/// Upsamples and color-converts the row group `in_row_group_ctr` of
/// `input_buf` into the rows `output_buf[0]` and `output_buf[1]` of
/// `output_width` pixels in the pixel layout of the parameters.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
///
/// # Panics
///
/// When a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_wrap,
    reason = "C's int cols_remaining = output_width and the row indexes"
)]
#[expect(clippy::cast_sign_loss, reason = "C's conversions of the pixel size")]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[target_feature(enable = "neon")]
pub fn jsimd_h2v2_merged_upsample_neon<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_ALPHA: i32,
    const RGB_PIXELSIZE: i32,
>(
    output_width: JDIMENSION,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    let consts: int16x4_t = vld1_s16(&jsimd_ycc_rgb_convert_neon_consts);
    let neg_128: int16x8_t = vdupq_n_s16(-128);

    /* Pointers to Y (both rows), Cb, and Cr data */
    let mut inptr0_0 = input_buf[0].row(in_row_group_ctr as isize * 2);
    let mut inptr0_1 = input_buf[0].row(in_row_group_ctr as isize * 2 + 1);
    let mut inptr1 = input_buf[1].row(in_row_group_ctr as isize);
    let mut inptr2 = input_buf[2].row(in_row_group_ctr as isize);
    let (outptr0, rest) = output_buf.split_at_mut(1);
    let mut outptr0: &mut [JSAMPLE] = &mut *outptr0[0];
    let mut outptr1: &mut [JSAMPLE] = &mut *rest[0];

    let mut cols_remaining = output_width as i32;
    while cols_remaining >= 16 {
        /* For each row, de-interleave Y component values into two separate
         * vectors, one containing the component values with even-numbered indices
         * and one containing the component values with odd-numbered indices.
         */
        let y0: uint8x8x2_t = vld2_u8(input(inptr0_0));
        let y1: uint8x8x2_t = vld2_u8(input(inptr0_1));
        let cb: uint8x8_t = vld1_u8(input(inptr1));
        let cr: uint8x8_t = vld1_u8(input(inptr2));
        /* Subtract 128 from Cb and Cr. */
        let cr_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cr));
        let cb_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cb));
        /* Compute G-Y: - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128) */
        let mut g_sub_y_l: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128), consts);
        let mut g_sub_y_h: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128), consts);
        g_sub_y_l = vmlsl_lane_s16::<1>(g_sub_y_l, vget_low_s16(cr_128), consts);
        g_sub_y_h = vmlsl_lane_s16::<1>(g_sub_y_h, vget_high_s16(cr_128), consts);
        /* Descale G components: shift right 15, round, and narrow to 16-bit. */
        let g_sub_y: int16x8_t =
            vcombine_s16(vrshrn_n_s32::<15>(g_sub_y_l), vrshrn_n_s32::<15>(g_sub_y_h));
        /* Compute R-Y: 1.40200 * (Cr - 128) */
        let r_sub_y: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128), consts);
        /* Compute B-Y: 1.77200 * (Cb - 128) */
        let b_sub_y: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128), consts);
        /* For each row, add the chroma-derived values (G-Y, R-Y, and B-Y) to both
         * the "even" and "odd" Y component values.  This effectively upsamples the
         * chroma components both horizontally and vertically.
         */
        let g0_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y0.0));
        let r0_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y0.0));
        let b0_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y0.0));
        let g0_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y0.1));
        let r0_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y0.1));
        let b0_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y0.1));
        let g1_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y1.0));
        let r1_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y1.0));
        let b1_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y1.0));
        let g1_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y1.1));
        let r1_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y1.1));
        let b1_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y1.1));
        /* Convert each component to unsigned and narrow, clamping to [0-255].
         * Re-interleave the "even" and "odd" component values.
         */
        let r0: uint8x8x2_t = vzip_u8(vqmovun_s16(r0_even), vqmovun_s16(r0_odd));
        let r1: uint8x8x2_t = vzip_u8(vqmovun_s16(r1_even), vqmovun_s16(r1_odd));
        let g0: uint8x8x2_t = vzip_u8(vqmovun_s16(g0_even), vqmovun_s16(g0_odd));
        let g1: uint8x8x2_t = vzip_u8(vqmovun_s16(g1_even), vqmovun_s16(g1_odd));
        let b0: uint8x8x2_t = vzip_u8(vqmovun_s16(b0_even), vqmovun_s16(b0_odd));
        let b1: uint8x8x2_t = vzip_u8(vqmovun_s16(b1_even), vqmovun_s16(b1_odd));

        if RGB_ALPHA >= 0 {
            let rgba0_red = vcombine_u8(r0.0, r0.1);
            let rgba1_red = vcombine_u8(r1.0, r1.1);
            let rgba0_green = vcombine_u8(g0.0, g0.1);
            let rgba1_green = vcombine_u8(g1.0, g1.1);
            let rgba0_blue = vcombine_u8(b0.0, b0.1);
            let rgba1_blue = vcombine_u8(b1.0, b1.1);
            /* Set alpha channel to opaque (0xFF). */
            let rgba0_alpha = vdupq_n_u8(0xFF);
            let rgba1_alpha = vdupq_n_u8(0xFF);
            let rgba0 = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                rgba0_red,
                rgba0_green,
                rgba0_blue,
                rgba0_alpha,
            );
            let rgba1 = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                rgba1_red,
                rgba1_green,
                rgba1_blue,
                rgba1_alpha,
            );
            let rgba0 = uint8x16x4_t(rgba0[0], rgba0[1], rgba0[2], rgba0[3]);
            let rgba1 = uint8x16x4_t(rgba1[0], rgba1[1], rgba1[2], rgba1[3]);
            /* Store RGBA pixel data to memory. */
            vst4q_u8(output(outptr0, 0), rgba0);
            vst4q_u8(output(outptr1, 0), rgba1);
        } else {
            let rgb0_red = vcombine_u8(r0.0, r0.1);
            let rgb1_red = vcombine_u8(r1.0, r1.1);
            let rgb0_green = vcombine_u8(g0.0, g0.1);
            let rgb1_green = vcombine_u8(g1.0, g1.1);
            let rgb0_blue = vcombine_u8(b0.0, b0.1);
            let rgb1_blue = vcombine_u8(b1.0, b1.1);
            let rgb0 = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(rgb0_red, rgb0_green, rgb0_blue);
            let rgb1 = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(rgb1_red, rgb1_green, rgb1_blue);
            let rgb0 = uint8x16x3_t(rgb0[0], rgb0[1], rgb0[2]);
            let rgb1 = uint8x16x3_t(rgb1[0], rgb1[1], rgb1[2]);
            /* Store RGB pixel data to memory. */
            vst3q_u8(output(outptr0, 0), rgb0);
            vst3q_u8(output(outptr1, 0), rgb1);
        }

        /* Increment pointers. */
        inptr0_0 = &inptr0_0[16..];
        inptr0_1 = &inptr0_1[16..];
        inptr1 = &inptr1[8..];
        inptr2 = &inptr2[8..];
        outptr0 = &mut outptr0[RGB_PIXELSIZE as usize * 16..];
        outptr1 = &mut outptr1[RGB_PIXELSIZE as usize * 16..];
        cols_remaining -= 16;
    }

    if cols_remaining > 0 {
        /* For each row, de-interleave Y component values into two separate
         * vectors, one containing the component values with even-numbered indices
         * and one containing the component values with odd-numbered indices.
         */
        let y0: uint8x8x2_t = vld2_u8(input(inptr0_0));
        let y1: uint8x8x2_t = vld2_u8(input(inptr0_1));
        let cb: uint8x8_t = vld1_u8(input(inptr1));
        let cr: uint8x8_t = vld1_u8(input(inptr2));
        /* Subtract 128 from Cb and Cr. */
        let cr_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cr));
        let cb_128: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cb));
        /* Compute G-Y: - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128) */
        let mut g_sub_y_l: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128), consts);
        let mut g_sub_y_h: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128), consts);
        g_sub_y_l = vmlsl_lane_s16::<1>(g_sub_y_l, vget_low_s16(cr_128), consts);
        g_sub_y_h = vmlsl_lane_s16::<1>(g_sub_y_h, vget_high_s16(cr_128), consts);
        /* Descale G components: shift right 15, round, and narrow to 16-bit. */
        let g_sub_y: int16x8_t =
            vcombine_s16(vrshrn_n_s32::<15>(g_sub_y_l), vrshrn_n_s32::<15>(g_sub_y_h));
        /* Compute R-Y: 1.40200 * (Cr - 128) */
        let r_sub_y: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128), consts);
        /* Compute B-Y: 1.77200 * (Cb - 128) */
        let b_sub_y: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128), consts);
        /* For each row, add the chroma-derived values (G-Y, R-Y, and B-Y) to both
         * the "even" and "odd" Y component values.  This effectively upsamples the
         * chroma components both horizontally and vertically.
         */
        let g0_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y0.0));
        let r0_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y0.0));
        let b0_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y0.0));
        let g0_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y0.1));
        let r0_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y0.1));
        let b0_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y0.1));
        let g1_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y1.0));
        let r1_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y1.0));
        let b1_even: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y1.0));
        let g1_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y1.1));
        let r1_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y1.1));
        let b1_odd: int16x8_t =
            vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y1.1));
        /* Convert each component to unsigned and narrow, clamping to [0-255].
         * Re-interleave the "even" and "odd" component values.
         */
        let r0: uint8x8x2_t = vzip_u8(vqmovun_s16(r0_even), vqmovun_s16(r0_odd));
        let r1: uint8x8x2_t = vzip_u8(vqmovun_s16(r1_even), vqmovun_s16(r1_odd));
        let g0: uint8x8x2_t = vzip_u8(vqmovun_s16(g0_even), vqmovun_s16(g0_odd));
        let g1: uint8x8x2_t = vzip_u8(vqmovun_s16(g1_even), vqmovun_s16(g1_odd));
        let b0: uint8x8x2_t = vzip_u8(vqmovun_s16(b0_even), vqmovun_s16(b0_odd));
        let b1: uint8x8x2_t = vzip_u8(vqmovun_s16(b1_even), vqmovun_s16(b1_odd));

        if RGB_ALPHA >= 0 {
            let rgba0_h_red = r0.1;
            let rgba1_h_red = r1.1;
            let rgba0_h_green = g0.1;
            let rgba1_h_green = g1.1;
            let rgba0_h_blue = b0.1;
            let rgba1_h_blue = b1.1;
            /* Set alpha channel to opaque (0xFF). */
            let rgba0_h_alpha = vdup_n_u8(0xFF);
            let rgba1_h_alpha = vdup_n_u8(0xFF);
            let rgba0_h = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                rgba0_h_red,
                rgba0_h_green,
                rgba0_h_blue,
                rgba0_h_alpha,
            );
            let rgba1_h = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                rgba1_h_red,
                rgba1_h_green,
                rgba1_h_blue,
                rgba1_h_alpha,
            );
            let rgba0_h = uint8x8x4_t(rgba0_h[0], rgba0_h[1], rgba0_h[2], rgba0_h[3]);
            let rgba1_h = uint8x8x4_t(rgba1_h[0], rgba1_h[1], rgba1_h[2], rgba1_h[3]);

            let rgba0_l_red = r0.0;
            let rgba1_l_red = r1.0;
            let rgba0_l_green = g0.0;
            let rgba1_l_green = g1.0;
            let rgba0_l_blue = b0.0;
            let rgba1_l_blue = b1.0;
            /* Set alpha channel to opaque (0xFF). */
            let rgba0_l_alpha = vdup_n_u8(0xFF);
            let rgba1_l_alpha = vdup_n_u8(0xFF);
            let rgba0_l = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                rgba0_l_red,
                rgba0_l_green,
                rgba0_l_blue,
                rgba0_l_alpha,
            );
            let rgba1_l = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                rgba1_l_red,
                rgba1_l_green,
                rgba1_l_blue,
                rgba1_l_alpha,
            );
            let rgba0_l = uint8x8x4_t(rgba0_l[0], rgba0_l[1], rgba0_l[2], rgba0_l[3]);
            let rgba1_l = uint8x8x4_t(rgba1_l[0], rgba1_l[1], rgba1_l[2], rgba1_l[3]);
            /* Store RGBA pixel data to memory. */
            if cols_remaining >= 8 {
                if cols_remaining >= 15 {
                    vst4_lane_u8::<6>(output(outptr0, 14 * RGB_PIXELSIZE as usize), rgba0_h);
                    vst4_lane_u8::<6>(output(outptr1, 14 * RGB_PIXELSIZE as usize), rgba1_h);
                }
                if cols_remaining >= 14 {
                    vst4_lane_u8::<5>(output(outptr0, 13 * RGB_PIXELSIZE as usize), rgba0_h);
                    vst4_lane_u8::<5>(output(outptr1, 13 * RGB_PIXELSIZE as usize), rgba1_h);
                }
                if cols_remaining >= 13 {
                    vst4_lane_u8::<4>(output(outptr0, 12 * RGB_PIXELSIZE as usize), rgba0_h);
                    vst4_lane_u8::<4>(output(outptr1, 12 * RGB_PIXELSIZE as usize), rgba1_h);
                }
                if cols_remaining >= 12 {
                    vst4_lane_u8::<3>(output(outptr0, 11 * RGB_PIXELSIZE as usize), rgba0_h);
                    vst4_lane_u8::<3>(output(outptr1, 11 * RGB_PIXELSIZE as usize), rgba1_h);
                }
                if cols_remaining >= 11 {
                    vst4_lane_u8::<2>(output(outptr0, 10 * RGB_PIXELSIZE as usize), rgba0_h);
                    vst4_lane_u8::<2>(output(outptr1, 10 * RGB_PIXELSIZE as usize), rgba1_h);
                }
                if cols_remaining >= 10 {
                    vst4_lane_u8::<1>(output(outptr0, 9 * RGB_PIXELSIZE as usize), rgba0_h);
                    vst4_lane_u8::<1>(output(outptr1, 9 * RGB_PIXELSIZE as usize), rgba1_h);
                }
                if cols_remaining >= 9 {
                    vst4_lane_u8::<0>(output(outptr0, 8 * RGB_PIXELSIZE as usize), rgba0_h);
                    vst4_lane_u8::<0>(output(outptr1, 8 * RGB_PIXELSIZE as usize), rgba1_h);
                }
                vst4_u8(output(outptr0, 0), rgba0_l);
                vst4_u8(output(outptr1, 0), rgba1_l);
            } else {
                if cols_remaining >= 7 {
                    vst4_lane_u8::<6>(output(outptr0, 6 * RGB_PIXELSIZE as usize), rgba0_l);
                    vst4_lane_u8::<6>(output(outptr1, 6 * RGB_PIXELSIZE as usize), rgba1_l);
                }
                if cols_remaining >= 6 {
                    vst4_lane_u8::<5>(output(outptr0, 5 * RGB_PIXELSIZE as usize), rgba0_l);
                    vst4_lane_u8::<5>(output(outptr1, 5 * RGB_PIXELSIZE as usize), rgba1_l);
                }
                if cols_remaining >= 5 {
                    vst4_lane_u8::<4>(output(outptr0, 4 * RGB_PIXELSIZE as usize), rgba0_l);
                    vst4_lane_u8::<4>(output(outptr1, 4 * RGB_PIXELSIZE as usize), rgba1_l);
                }
                if cols_remaining >= 4 {
                    vst4_lane_u8::<3>(output(outptr0, 3 * RGB_PIXELSIZE as usize), rgba0_l);
                    vst4_lane_u8::<3>(output(outptr1, 3 * RGB_PIXELSIZE as usize), rgba1_l);
                }
                if cols_remaining >= 3 {
                    vst4_lane_u8::<2>(output(outptr0, 2 * RGB_PIXELSIZE as usize), rgba0_l);
                    vst4_lane_u8::<2>(output(outptr1, 2 * RGB_PIXELSIZE as usize), rgba1_l);
                }
                if cols_remaining >= 2 {
                    vst4_lane_u8::<1>(output(outptr0, RGB_PIXELSIZE as usize), rgba0_l);
                    vst4_lane_u8::<1>(output(outptr1, RGB_PIXELSIZE as usize), rgba1_l);
                }
                if cols_remaining >= 1 {
                    vst4_lane_u8::<0>(output(outptr0, 0), rgba0_l);
                    vst4_lane_u8::<0>(output(outptr1, 0), rgba1_l);
                }
            }
        } else {
            let rgb0_h = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(r0.1, g0.1, b0.1);
            let rgb1_h = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(r1.1, g1.1, b1.1);
            let rgb0_h = uint8x8x3_t(rgb0_h[0], rgb0_h[1], rgb0_h[2]);
            let rgb1_h = uint8x8x3_t(rgb1_h[0], rgb1_h[1], rgb1_h[2]);

            let rgb0_l = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(r0.0, g0.0, b0.0);
            let rgb1_l = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(r1.0, g1.0, b1.0);
            let rgb0_l = uint8x8x3_t(rgb0_l[0], rgb0_l[1], rgb0_l[2]);
            let rgb1_l = uint8x8x3_t(rgb1_l[0], rgb1_l[1], rgb1_l[2]);
            /* Store RGB pixel data to memory. */
            if cols_remaining >= 8 {
                if cols_remaining >= 15 {
                    vst3_lane_u8::<6>(output(outptr0, 14 * RGB_PIXELSIZE as usize), rgb0_h);
                    vst3_lane_u8::<6>(output(outptr1, 14 * RGB_PIXELSIZE as usize), rgb1_h);
                }
                if cols_remaining >= 14 {
                    vst3_lane_u8::<5>(output(outptr0, 13 * RGB_PIXELSIZE as usize), rgb0_h);
                    vst3_lane_u8::<5>(output(outptr1, 13 * RGB_PIXELSIZE as usize), rgb1_h);
                }
                if cols_remaining >= 13 {
                    vst3_lane_u8::<4>(output(outptr0, 12 * RGB_PIXELSIZE as usize), rgb0_h);
                    vst3_lane_u8::<4>(output(outptr1, 12 * RGB_PIXELSIZE as usize), rgb1_h);
                }
                if cols_remaining >= 12 {
                    vst3_lane_u8::<3>(output(outptr0, 11 * RGB_PIXELSIZE as usize), rgb0_h);
                    vst3_lane_u8::<3>(output(outptr1, 11 * RGB_PIXELSIZE as usize), rgb1_h);
                }
                if cols_remaining >= 11 {
                    vst3_lane_u8::<2>(output(outptr0, 10 * RGB_PIXELSIZE as usize), rgb0_h);
                    vst3_lane_u8::<2>(output(outptr1, 10 * RGB_PIXELSIZE as usize), rgb1_h);
                }
                if cols_remaining >= 10 {
                    vst3_lane_u8::<1>(output(outptr0, 9 * RGB_PIXELSIZE as usize), rgb0_h);
                    vst3_lane_u8::<1>(output(outptr1, 9 * RGB_PIXELSIZE as usize), rgb1_h);
                }
                if cols_remaining >= 9 {
                    vst3_lane_u8::<0>(output(outptr0, 8 * RGB_PIXELSIZE as usize), rgb0_h);
                    vst3_lane_u8::<0>(output(outptr1, 8 * RGB_PIXELSIZE as usize), rgb1_h);
                }
                vst3_u8(output(outptr0, 0), rgb0_l);
                vst3_u8(output(outptr1, 0), rgb1_l);
            } else {
                if cols_remaining >= 7 {
                    vst3_lane_u8::<6>(output(outptr0, 6 * RGB_PIXELSIZE as usize), rgb0_l);
                    vst3_lane_u8::<6>(output(outptr1, 6 * RGB_PIXELSIZE as usize), rgb1_l);
                }
                if cols_remaining >= 6 {
                    vst3_lane_u8::<5>(output(outptr0, 5 * RGB_PIXELSIZE as usize), rgb0_l);
                    vst3_lane_u8::<5>(output(outptr1, 5 * RGB_PIXELSIZE as usize), rgb1_l);
                }
                if cols_remaining >= 5 {
                    vst3_lane_u8::<4>(output(outptr0, 4 * RGB_PIXELSIZE as usize), rgb0_l);
                    vst3_lane_u8::<4>(output(outptr1, 4 * RGB_PIXELSIZE as usize), rgb1_l);
                }
                if cols_remaining >= 4 {
                    vst3_lane_u8::<3>(output(outptr0, 3 * RGB_PIXELSIZE as usize), rgb0_l);
                    vst3_lane_u8::<3>(output(outptr1, 3 * RGB_PIXELSIZE as usize), rgb1_l);
                }
                if cols_remaining >= 3 {
                    vst3_lane_u8::<2>(output(outptr0, 2 * RGB_PIXELSIZE as usize), rgb0_l);
                    vst3_lane_u8::<2>(output(outptr1, 2 * RGB_PIXELSIZE as usize), rgb1_l);
                }
                if cols_remaining >= 2 {
                    vst3_lane_u8::<1>(output(outptr0, RGB_PIXELSIZE as usize), rgb0_l);
                    vst3_lane_u8::<1>(output(outptr1, RGB_PIXELSIZE as usize), rgb1_l);
                }
                if cols_remaining >= 1 {
                    vst3_lane_u8::<0>(output(outptr0, 0), rgb0_l);
                    vst3_lane_u8::<0>(output(outptr1, 0), rgb1_l);
                }
            }
        }
    }
}
