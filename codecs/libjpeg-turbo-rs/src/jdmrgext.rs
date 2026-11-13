// libjpeg-turbo 3.2.1, src/jdmrgext.c:
//
// /*
//  * jdmrgext.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2011, 2015, 2020, 2022-2023, 2025, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains code for merged upsampling/color conversion.
//  */
//! Translation of `src/jdmrgext.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
//!
//! `jdmerge.c` includes this file once per RGB pixel layout, as
//! `jdcolor.c` includes `jdcolext.c`: each routine is a function generic
//! over `RGB_RED`, `RGB_GREEN`, `RGB_BLUE`, `RGB_ALPHA` (-1 where C leaves
//! it undefined) and `RGB_PIXELSIZE`, and `jdmerge.rs` names the instances.
//! The routines receive the merged upsampler, which C reaches through
//! `cinfo`, and the output rows as separate slices, because the second row of
//! the 2:1 vertical case may be the upsampler's spare row.

use crate::jdcolext::{TABLE, table};
use crate::jdmerge::{SCALEBITS, my_merged_upsampler};
use crate::jmorecfg::{JDIMENSION, JSAMPLE, MAXJSAMPLE};
use crate::jpegint::{JLONG, RIGHT_SHIFT};
use crate::jpeglib::{JSAMPROWS_AT, jpeg_decompress_struct};

// upstream: src/jdmrgext.c h2v1_merged_upsample
/// Upsamples and color converts for the case of 2:1 horizontal and 1:1
/// vertical.
///
/// # Panics
///
/// When a table is absent or a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of samples, table values and layout indexes"
)]
#[expect(non_snake_case, reason = "C names")]
#[expect(clippy::cast_possible_wrap, reason = "C's row index")]
#[inline]
pub fn h2v1_merged_upsample<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    #[expect(clippy::cast_possible_wrap, reason = "C's conversions")] const RGB_ALPHA: i32,
    #[expect(clippy::cast_possible_wrap, reason = "C's conversions")] const RGB_PIXELSIZE: i32,
>(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_merged_upsampler,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    /* copy these pointers into registers if possible */
    let range_limit = cinfo.sample_range_limit_signed();
    /* The tables as arrays, which a sample indexes without a bounds check. */
    let Crrtab: &[i32; TABLE] = table(upsample.Cr_r_tab.as_deref(), "Cr_r_tab is NULL");
    let Cbbtab: &[i32; TABLE] = table(upsample.Cb_b_tab.as_deref(), "Cb_b_tab is NULL");
    let Crgtab: &[JLONG; TABLE] = table(upsample.Cr_g_tab.as_deref(), "Cr_g_tab is NULL");
    let Cbgtab: &[JLONG; TABLE] = table(upsample.Cb_g_tab.as_deref(), "Cb_g_tab is NULL");

    let row = in_row_group_ctr as isize;
    let inptr0 = input_buf[0].row(row);
    let inptr1 = input_buf[1].row(row);
    let inptr2 = input_buf[2].row(row);
    let outptr = &mut *output_buf[0];
    /* Loop for each pair of output pixels, over the rows split off once, as
     * C's pointers need no bounds checks */
    let col = (cinfo.output_width >> 1) as usize;
    let pixel_pair = 2 * RGB_PIXELSIZE as usize;
    for (((outpair, ys), &cb), &cr) in outptr[..col * pixel_pair]
        .chunks_exact_mut(pixel_pair)
        .zip(inptr0[..2 * col].as_chunks::<2>().0)
        .zip(&inptr1[..col])
        .zip(&inptr2[..col])
    {
        /* Do the chroma part of the calculation */
        let (cb, cr) = (usize::from(cb), usize::from(cr));
        let cred = Crrtab[cr];
        let cgreen = RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS) as i32;
        let cblue = Cbbtab[cb];
        /* Fetch 2 Y values and emit 2 pixels */
        for (pixel, &y) in outpair.chunks_exact_mut(RGB_PIXELSIZE as usize).zip(ys) {
            let y = i32::from(y);
            pixel[RGB_RED as usize] = range_limit.at(y + cred);
            pixel[RGB_GREEN as usize] = range_limit.at(y + cgreen);
            pixel[RGB_BLUE as usize] = range_limit.at(y + cblue);
            if RGB_ALPHA >= 0 {
                pixel[RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
            }
        }
    }
    let (in0, in12, out) = (2 * col, col, col * pixel_pair);
    /* If image width is odd, do the last output column separately */
    if cinfo.output_width & 1 != 0 {
        let cb = usize::from(inptr1[in12]);
        let cr = usize::from(inptr2[in12]);
        let cred = Crrtab[cr];
        let cgreen = RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS) as i32;
        let cblue = Cbbtab[cb];
        let y = i32::from(inptr0[in0]);
        outptr[out + RGB_RED as usize] = range_limit.at(y + cred);
        outptr[out + RGB_GREEN as usize] = range_limit.at(y + cgreen);
        outptr[out + RGB_BLUE as usize] = range_limit.at(y + cblue);
        if RGB_ALPHA >= 0 {
            outptr[out + RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
        }
    }
}

// upstream: src/jdmrgext.c h2v2_merged_upsample
/// Upsamples and color converts for the case of 2:1 horizontal and 2:1
/// vertical.
///
/// # Panics
///
/// When a table is absent or a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of samples, table values and layout indexes"
)]
#[expect(non_snake_case, reason = "C names")]
#[expect(clippy::cast_possible_wrap, reason = "C's row index")]
#[expect(clippy::similar_names, reason = "C variable names")]
#[inline]
pub fn h2v2_merged_upsample<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    #[expect(clippy::similar_names, reason = "C variable names")]
    #[expect(clippy::cast_possible_wrap, reason = "C's conversions")] const RGB_ALPHA: i32,
    #[expect(clippy::similar_names, reason = "C variable names")]
    #[expect(clippy::cast_possible_wrap, reason = "C's conversions")] const RGB_PIXELSIZE: i32,
>(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_merged_upsampler,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    /* copy these pointers into registers if possible */
    let range_limit = cinfo.sample_range_limit_signed();
    /* The tables as arrays, which a sample indexes without a bounds check. */
    let Crrtab: &[i32; TABLE] = table(upsample.Cr_r_tab.as_deref(), "Cr_r_tab is NULL");
    let Cbbtab: &[i32; TABLE] = table(upsample.Cb_b_tab.as_deref(), "Cb_b_tab is NULL");
    let Crgtab: &[JLONG; TABLE] = table(upsample.Cr_g_tab.as_deref(), "Cr_g_tab is NULL");
    let Cbgtab: &[JLONG; TABLE] = table(upsample.Cb_g_tab.as_deref(), "Cb_g_tab is NULL");

    let row = in_row_group_ctr as isize;
    let inptr00 = input_buf[0].row(row * 2);
    let inptr01 = input_buf[0].row(row * 2 + 1);
    let inptr1 = input_buf[1].row(row);
    let inptr2 = input_buf[2].row(row);
    let (outptr0, rest) = output_buf.split_at_mut(1);
    let outptr0 = &mut *outptr0[0];
    let outptr1 = &mut *rest[0];
    /* Loop for each group of output pixels, over the rows split off once, as
     * C's pointers need no bounds checks */
    let col = (cinfo.output_width >> 1) as usize;
    let pixel_pair = 2 * RGB_PIXELSIZE as usize;
    for (((((outpair0, outpair1), ys0), ys1), &cb), &cr) in outptr0[..col * pixel_pair]
        .chunks_exact_mut(pixel_pair)
        .zip(outptr1[..col * pixel_pair].chunks_exact_mut(pixel_pair))
        .zip(inptr00[..2 * col].as_chunks::<2>().0)
        .zip(inptr01[..2 * col].as_chunks::<2>().0)
        .zip(&inptr1[..col])
        .zip(&inptr2[..col])
    {
        /* Do the chroma part of the calculation */
        let (cb, cr) = (usize::from(cb), usize::from(cr));
        let cred = Crrtab[cr];
        let cgreen = RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS) as i32;
        let cblue = Cbbtab[cb];
        /* Fetch 4 Y values and emit 4 pixels */
        for (ys, outpair) in [(ys0, outpair0), (ys1, outpair1)] {
            for (pixel, &y) in outpair.chunks_exact_mut(RGB_PIXELSIZE as usize).zip(ys) {
                let y = i32::from(y);
                pixel[RGB_RED as usize] = range_limit.at(y + cred);
                pixel[RGB_GREEN as usize] = range_limit.at(y + cgreen);
                pixel[RGB_BLUE as usize] = range_limit.at(y + cblue);
                if RGB_ALPHA >= 0 {
                    pixel[RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
                }
            }
        }
    }
    let (in0, in12, out) = (2 * col, col, col * pixel_pair);
    /* If image width is odd, do the last output column separately */
    if cinfo.output_width & 1 != 0 {
        let cb = usize::from(inptr1[in12]);
        let cr = usize::from(inptr2[in12]);
        let cred = Crrtab[cr];
        let cgreen = RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS) as i32;
        let cblue = Cbbtab[cb];
        let y = i32::from(inptr00[in0]);
        outptr0[out + RGB_RED as usize] = range_limit.at(y + cred);
        outptr0[out + RGB_GREEN as usize] = range_limit.at(y + cgreen);
        outptr0[out + RGB_BLUE as usize] = range_limit.at(y + cblue);
        if RGB_ALPHA >= 0 {
            outptr0[out + RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
        }
        let y = i32::from(inptr01[in0]);
        outptr1[out + RGB_RED as usize] = range_limit.at(y + cred);
        outptr1[out + RGB_GREEN as usize] = range_limit.at(y + cgreen);
        outptr1[out + RGB_BLUE as usize] = range_limit.at(y + cblue);
        if RGB_ALPHA >= 0 {
            outptr1[out + RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
        }
    }
}
