// libjpeg-turbo 3.2.1, src/jdcolext.c:
//
// /*
//  * jdcolext.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2009, 2011, 2015, 2022-2023, 2025, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains output colorspace conversion routines.
//  */
//! Translation of `src/jdcolext.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
//!
//! `jdcolor.c` includes this file once per RGB pixel layout, with
//! `RGB_RED`, `RGB_GREEN`, `RGB_BLUE`, `RGB_PIXELSIZE` and possibly
//! `RGB_ALPHA` defined, and with the function names renamed. Each routine
//! is a function generic over those values, and `jdcolor.rs` names the
//! instances as C does; `RGB_ALPHA` is -1 where C leaves it undefined.
//! `BITS_IN_JSAMPLE` is 8, so the `ERREXIT` branches for 16-bit samples are
//! not compiled.

use crate::jdcolor::{SCALEBITS, my_color_deconverter};
use crate::jmorecfg::{JDIMENSION, JSAMPLE, MAXJSAMPLE};
use crate::jpegint::{JLONG, RIGHT_SHIFT};
use crate::jpeglib::{JSAMPROWS, JSAMPROWS_MUT, jpeg_decompress_struct};

/// The number of entries of a conversion table, `MAXJSAMPLE + 1`, one per
/// sample value. Not an upstream item.
pub(crate) const TABLE: usize = MAXJSAMPLE as usize + 1;

/// A conversion table of the color deconverter as an array of its `TABLE`
/// entries. Not an upstream item.
///
/// # Panics
///
/// When the table is absent (with `message`) or shorter.
pub(crate) fn table<'a, T>(table: Option<&'a [T]>, message: &str) -> &'a [T; TABLE] {
    table
        .expect(message)
        .first_chunk()
        .expect("a conversion table has MAXJSAMPLE + 1 entries")
}

/// The color deconverter's private part, which C reaches through
/// `cinfo->cconvert`.
///
/// # Panics
///
/// When the module is absent.
fn cconvert<'a>(cinfo: &'a jpeg_decompress_struct<'_>) -> &'a my_color_deconverter {
    cinfo.cconvert.as_deref().expect("cinfo->cconvert is NULL")
}

// upstream: src/jdcolext.c ycc_rgb_convert
/// Converts some rows of samples from YCbCr to RGB in the pixel layout of
/// the parameters. Range-limiting is essential due to noise introduced by
/// DCT losses.
///
/// # Panics
///
/// When the conversion tables or the range-limit table are absent, or a row
/// is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of samples, table values and layout indexes"
)]
#[expect(non_snake_case, reason = "C names")]
#[inline]
pub fn ycc_rgb_convert<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_ALPHA: i32,
    const RGB_PIXELSIZE: i32,
>(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    mut num_rows: i32,
) {
    let cconvert = cconvert(cinfo);
    let num_cols: JDIMENSION = cinfo.output_width;
    /* copy these pointers into registers if possible */
    let range_limit = cinfo.sample_range_limit_signed();
    /* The tables as arrays of their MAXJSAMPLE + 1 entries, which a sample
     * indexes without a bounds check, as in C. */
    let Crrtab: &[i32; TABLE] = table(cconvert.Cr_r_tab.as_deref(), "Cr_r_tab is NULL");
    let Cbbtab: &[i32; TABLE] = table(cconvert.Cb_b_tab.as_deref(), "Cb_b_tab is NULL");
    let Crgtab: &[JLONG; TABLE] = table(cconvert.Cr_g_tab.as_deref(), "Cr_g_tab is NULL");
    let Cbgtab: &[JLONG; TABLE] = table(cconvert.Cb_g_tab.as_deref(), "Cb_g_tab is NULL");

    let mut out_row = 0;
    loop {
        num_rows -= 1;
        if num_rows < 0 {
            break;
        }
        /* The rows split off to num_cols once, as C's pointers need no bounds
         * checks. */
        let n = num_cols as usize;
        let inptr0 = &input_buf[0].row(input_row as usize)[..n];
        let inptr1 = &input_buf[1].row(input_row as usize)[..n];
        let inptr2 = &input_buf[2].row(input_row as usize)[..n];
        input_row += 1;
        let outptr = &mut output_buf.row_mut(out_row)[..n * RGB_PIXELSIZE as usize];
        out_row += 1;
        for (col, pixel) in outptr.chunks_exact_mut(RGB_PIXELSIZE as usize).enumerate() {
            let y = i32::from(inptr0[col]);
            let cb = usize::from(inptr1[col]);
            let cr = usize::from(inptr2[col]);
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            pixel[RGB_RED as usize] = range_limit.at(y + Crrtab[cr]);
            pixel[RGB_GREEN as usize] =
                range_limit.at(y + RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS) as i32);
            pixel[RGB_BLUE as usize] = range_limit.at(y + Cbbtab[cb]);
            /* Set unused byte to _MAXJSAMPLE so it can be interpreted as an */
            /* opaque alpha channel value */
            if RGB_ALPHA >= 0 {
                pixel[RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
            }
        }
    }
}

// upstream: src/jdcolext.c gray_rgb_convert
/// Converts grayscale to RGB in the pixel layout of the parameters.
///
/// # Panics
///
/// When a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of layout indexes"
)]
#[expect(clippy::needless_range_loop, reason = "C's loop")]
#[inline]
pub fn gray_rgb_convert<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    #[expect(clippy::needless_range_loop, reason = "C's loop")] const RGB_ALPHA: i32,
    #[expect(clippy::needless_range_loop, reason = "C's loop")] const RGB_PIXELSIZE: i32,
>(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    mut num_rows: i32,
) {
    let num_cols: JDIMENSION = cinfo.output_width;

    let mut out_row = 0;
    loop {
        num_rows -= 1;
        if num_rows < 0 {
            break;
        }
        let inptr = input_buf[0].row(input_row as usize);
        input_row += 1;
        let outptr = output_buf.row_mut(out_row);
        out_row += 1;
        let mut out = 0;
        for col in 0..num_cols as usize {
            outptr[out + RGB_BLUE as usize] = inptr[col];
            outptr[out + RGB_GREEN as usize] = outptr[out + RGB_BLUE as usize];
            outptr[out + RGB_RED as usize] = outptr[out + RGB_GREEN as usize];
            /* Set unused byte to _MAXJSAMPLE so it can be interpreted as an */
            /* opaque alpha channel value */
            if RGB_ALPHA >= 0 {
                outptr[out + RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
            }
            out += RGB_PIXELSIZE as usize;
        }
    }
}

// upstream: src/jdcolext.c rgb_rgb_convert
/// Converts RGB to extended RGB: just swaps the order around.
///
/// # Panics
///
/// When a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of layout indexes"
)]
#[inline]
pub fn rgb_rgb_convert<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_ALPHA: i32,
    const RGB_PIXELSIZE: i32,
>(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    mut num_rows: i32,
) {
    let num_cols: JDIMENSION = cinfo.output_width;

    let mut out_row = 0;
    loop {
        num_rows -= 1;
        if num_rows < 0 {
            break;
        }
        let inptr0 = input_buf[0].row(input_row as usize);
        let inptr1 = input_buf[1].row(input_row as usize);
        let inptr2 = input_buf[2].row(input_row as usize);
        input_row += 1;
        let outptr = output_buf.row_mut(out_row);
        out_row += 1;
        let mut out = 0;
        for col in 0..num_cols as usize {
            outptr[out + RGB_RED as usize] = inptr0[col];
            outptr[out + RGB_GREEN as usize] = inptr1[col];
            outptr[out + RGB_BLUE as usize] = inptr2[col];
            /* Set unused byte to _MAXJSAMPLE so it can be interpreted as an */
            /* opaque alpha channel value */
            if RGB_ALPHA >= 0 {
                outptr[out + RGB_ALPHA as usize] = MAXJSAMPLE as JSAMPLE;
            }
            out += RGB_PIXELSIZE as usize;
        }
    }
}
