// libjpeg-turbo 3.2.1, src/jdcolor.c:
//
// /*
//  * jdcolor.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Modified 2011 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2009, 2011-2012, 2014-2015, 2022, 2024-2025, D. R. Commander.
//  * Copyright (C) 2013, Linaro Limited.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains output colorspace conversion routines.
//  */
//! Translation of `src/jdcolor.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
//!
//! `BITS_IN_JSAMPLE` is 8, so the 16-bit error branches are not compiled,
//! and with `D_LOSSLESS_SUPPORTED` treated as undefined neither are the
//! lossless checks (ADR 0005). `WITH_SIMD` is `cfg(with_simd)` (ADR 0007):
//! the YCbCr-to-RGB branch then asks `jsimd_set_ycc_rgb` for a Neon kernel
//! first, in a labeled block that it leaves when it gets one, as C's
//! `if ... else` across the `#ifdef` skips the C routines and their tables.
//! The RGB565 routines of `jdcol565.c` and their SIMD selection are out of
//! scope: `JCS_RGB565` output fails with `JERR_CONVERSION_NOTIMPL` (ADR 0005,
//! "Out-of-scope branches"). The instances of `jdcolext.c`'s routines are
//! constants that name a generic function of `jdcolext` with the layout of
//! the C instance. The conversion tables are `None` until built, where C's
//! pointers are left unset.

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jdcolext;
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemmgr::{alloc_small_box, alloc_small_vec};
use crate::jmorecfg::{
    C_POINTER, CENTERJSAMPLE, CLayout, EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE, EXT_BGR_RED,
    EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN,
    EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE,
    EXT_RGBX_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE,
    EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, JDIMENSION, JSAMPLE, MAXJSAMPLE, RGB_BLUE,
    RGB_GREEN, RGB_PIXELSIZE, RGB_RED, rgb_blue, rgb_green, rgb_pixelsize, rgb_red,
};
use crate::jpegint::{JLONG, RIGHT_SHIFT, color_convert_method_ptr, jpeg_color_deconverter};
use crate::jpeglib::{
    J_COLOR_SPACE, JPOOL_IMAGE, JSAMPROWS, JSAMPROWS_MUT, j_decompress_ptr, jpeg_decompress_struct,
};
#[cfg(with_simd)]
use crate::jsimd::{jsimd_color_deconvert, jsimd_set_ycc_rgb};
use crate::jutils::jcopy_sample_rows;

// upstream: src/jdcolor.c my_color_deconverter
/// Private subobject.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Debug)]
pub struct my_color_deconverter {
    /// Public fields.
    pub pub_: jpeg_color_deconverter,

    /* Private state for YCC->RGB conversion */
    /// Table for Cr to R conversion.
    pub Cr_r_tab: Option<Vec<i32>>,
    /// Table for Cb to B conversion.
    pub Cb_b_tab: Option<Vec<i32>>,
    /// Table for Cr to G conversion.
    pub Cr_g_tab: Option<Vec<JLONG>>,
    /// Table for Cb to G conversion.
    pub Cb_g_tab: Option<Vec<JLONG>>,

    /* Private state for RGB->Y conversion */
    /// Table for RGB to Y conversion.
    pub rgb_y_tab: Option<Vec<JLONG>>,
}

/// `sizeof(my_color_deconverter)` on the target, with the
/// `color_convert_12`, `color_convert_16` and `color_convert_simd` pointers
/// of `struct jpeg_color_deconverter`.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_color_deconverter: usize = CLayout::structure(&[
    CLayout::structure(&[C_POINTER.array(5)]),
    C_POINTER.array(5),
])
.size;

/**************** YCbCr -> RGB conversion: most common case **************/
/****************   RGB -> Y   conversion: less common case **************/

// upstream: src/jdcolor.c SCALEBITS
/// Speediest right-shift on some machines.
pub const SCALEBITS: u32 = 16;
// upstream: src/jdcolor.c ONE_HALF
pub const ONE_HALF: JLONG = 1 << (SCALEBITS - 1);

// upstream: src/jdcolor.c FIX
/// Converts a real constant to an integer scaled by `2 ** SCALEBITS`,
/// rounding, in C's `double` arithmetic.
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "C's conversions between JLONG and double"
)]
#[inline]
#[must_use]
pub const fn FIX(x: f64) -> JLONG {
    (x * ((1 as core::ffi::c_long) << SCALEBITS) as f64 + 0.5) as JLONG
}

/* We allocate one big table for RGB->Y conversion and divide it up into
 * three parts, instead of doing three alloc_small requests.  This lets us
 * use a single table base address, which can be held in a register in the
 * inner loops on many machines (more than can hold all three addresses,
 * anyway).
 */

// upstream: src/jdcolor.c R_Y_OFF
/// Offset to R => Y section.
const R_Y_OFF: usize = 0;
// upstream: src/jdcolor.c G_Y_OFF
/// Offset to G => Y section.
const G_Y_OFF: usize = (MAXJSAMPLE + 1) as usize;
// upstream: src/jdcolor.c B_Y_OFF
/// Offset to B => Y section.
const B_Y_OFF: usize = 2 * (MAXJSAMPLE + 1) as usize;
// upstream: src/jdcolor.c TABLE_SIZE
const TABLE_SIZE: usize = 3 * (MAXJSAMPLE + 1) as usize;

/* Include inline routines for colorspace extensions */

// upstream: src/jdcolor.c ycc_rgb_convert
/// `jdcolext.c`'s `ycc_rgb_convert` with `jmorecfg.h`'s default RGB layout.
#[expect(non_upper_case_globals, reason = "C function name")]
const ycc_rgb_convert: color_convert_method_ptr =
    jdcolext::ycc_rgb_convert::<RGB_RED, RGB_GREEN, RGB_BLUE, -1, RGB_PIXELSIZE>;
// upstream: src/jdcolor.c gray_rgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const gray_rgb_convert: color_convert_method_ptr =
    jdcolext::gray_rgb_convert::<RGB_RED, RGB_GREEN, RGB_BLUE, -1, RGB_PIXELSIZE>;
// upstream: src/jdcolor.c rgb_rgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const rgb_rgb_convert: color_convert_method_ptr =
    jdcolext::rgb_rgb_convert::<RGB_RED, RGB_GREEN, RGB_BLUE, -1, RGB_PIXELSIZE>;

// upstream: src/jdcolor.c ycc_extrgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const ycc_extrgb_convert: color_convert_method_ptr =
    jdcolext::ycc_rgb_convert::<EXT_RGB_RED, EXT_RGB_GREEN, EXT_RGB_BLUE, -1, EXT_RGB_PIXELSIZE>;
// upstream: src/jdcolor.c gray_extrgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const gray_extrgb_convert: color_convert_method_ptr =
    jdcolext::gray_rgb_convert::<EXT_RGB_RED, EXT_RGB_GREEN, EXT_RGB_BLUE, -1, EXT_RGB_PIXELSIZE>;
// upstream: src/jdcolor.c rgb_extrgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const rgb_extrgb_convert: color_convert_method_ptr =
    jdcolext::rgb_rgb_convert::<EXT_RGB_RED, EXT_RGB_GREEN, EXT_RGB_BLUE, -1, EXT_RGB_PIXELSIZE>;

// upstream: src/jdcolor.c ycc_extrgbx_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const ycc_extrgbx_convert: color_convert_method_ptr =
    jdcolext::ycc_rgb_convert::<EXT_RGBX_RED, EXT_RGBX_GREEN, EXT_RGBX_BLUE, 3, EXT_RGBX_PIXELSIZE>;
// upstream: src/jdcolor.c gray_extrgbx_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const gray_extrgbx_convert: color_convert_method_ptr = jdcolext::gray_rgb_convert::<
    EXT_RGBX_RED,
    EXT_RGBX_GREEN,
    EXT_RGBX_BLUE,
    3,
    EXT_RGBX_PIXELSIZE,
>;
// upstream: src/jdcolor.c rgb_extrgbx_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const rgb_extrgbx_convert: color_convert_method_ptr =
    jdcolext::rgb_rgb_convert::<EXT_RGBX_RED, EXT_RGBX_GREEN, EXT_RGBX_BLUE, 3, EXT_RGBX_PIXELSIZE>;

// upstream: src/jdcolor.c ycc_extbgr_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const ycc_extbgr_convert: color_convert_method_ptr =
    jdcolext::ycc_rgb_convert::<EXT_BGR_RED, EXT_BGR_GREEN, EXT_BGR_BLUE, -1, EXT_BGR_PIXELSIZE>;
// upstream: src/jdcolor.c gray_extbgr_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const gray_extbgr_convert: color_convert_method_ptr =
    jdcolext::gray_rgb_convert::<EXT_BGR_RED, EXT_BGR_GREEN, EXT_BGR_BLUE, -1, EXT_BGR_PIXELSIZE>;
// upstream: src/jdcolor.c rgb_extbgr_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const rgb_extbgr_convert: color_convert_method_ptr =
    jdcolext::rgb_rgb_convert::<EXT_BGR_RED, EXT_BGR_GREEN, EXT_BGR_BLUE, -1, EXT_BGR_PIXELSIZE>;

// upstream: src/jdcolor.c ycc_extbgrx_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const ycc_extbgrx_convert: color_convert_method_ptr =
    jdcolext::ycc_rgb_convert::<EXT_BGRX_RED, EXT_BGRX_GREEN, EXT_BGRX_BLUE, 3, EXT_BGRX_PIXELSIZE>;
// upstream: src/jdcolor.c gray_extbgrx_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const gray_extbgrx_convert: color_convert_method_ptr = jdcolext::gray_rgb_convert::<
    EXT_BGRX_RED,
    EXT_BGRX_GREEN,
    EXT_BGRX_BLUE,
    3,
    EXT_BGRX_PIXELSIZE,
>;
// upstream: src/jdcolor.c rgb_extbgrx_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const rgb_extbgrx_convert: color_convert_method_ptr =
    jdcolext::rgb_rgb_convert::<EXT_BGRX_RED, EXT_BGRX_GREEN, EXT_BGRX_BLUE, 3, EXT_BGRX_PIXELSIZE>;

// upstream: src/jdcolor.c ycc_extxbgr_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const ycc_extxbgr_convert: color_convert_method_ptr =
    jdcolext::ycc_rgb_convert::<EXT_XBGR_RED, EXT_XBGR_GREEN, EXT_XBGR_BLUE, 0, EXT_XBGR_PIXELSIZE>;
// upstream: src/jdcolor.c gray_extxbgr_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const gray_extxbgr_convert: color_convert_method_ptr = jdcolext::gray_rgb_convert::<
    EXT_XBGR_RED,
    EXT_XBGR_GREEN,
    EXT_XBGR_BLUE,
    0,
    EXT_XBGR_PIXELSIZE,
>;
// upstream: src/jdcolor.c rgb_extxbgr_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const rgb_extxbgr_convert: color_convert_method_ptr =
    jdcolext::rgb_rgb_convert::<EXT_XBGR_RED, EXT_XBGR_GREEN, EXT_XBGR_BLUE, 0, EXT_XBGR_PIXELSIZE>;

// upstream: src/jdcolor.c ycc_extxrgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const ycc_extxrgb_convert: color_convert_method_ptr =
    jdcolext::ycc_rgb_convert::<EXT_XRGB_RED, EXT_XRGB_GREEN, EXT_XRGB_BLUE, 0, EXT_XRGB_PIXELSIZE>;
// upstream: src/jdcolor.c gray_extxrgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const gray_extxrgb_convert: color_convert_method_ptr = jdcolext::gray_rgb_convert::<
    EXT_XRGB_RED,
    EXT_XRGB_GREEN,
    EXT_XRGB_BLUE,
    0,
    EXT_XRGB_PIXELSIZE,
>;
// upstream: src/jdcolor.c rgb_extxrgb_convert
#[expect(non_upper_case_globals, reason = "C function name")]
const rgb_extxrgb_convert: color_convert_method_ptr =
    jdcolext::rgb_rgb_convert::<EXT_XRGB_RED, EXT_XRGB_GREEN, EXT_XRGB_BLUE, 0, EXT_XRGB_PIXELSIZE>;

/// The color deconverter, mutably, which C reaches through `cinfo->cconvert`.
fn cconvert_mut<'a>(cinfo: &'a mut jpeg_decompress_struct<'_>) -> &'a mut my_color_deconverter {
    cinfo
        .cconvert
        .as_deref_mut()
        .expect("cinfo->cconvert is NULL")
}

// upstream: src/jdcolor.c build_ycc_rgb_table
/// Initializes tables for YCC->RGB colorspace conversion.
///
/// # Errors
///
/// What the allocations report.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversions of table values and sizes"
)]
#[expect(non_snake_case, reason = "C names")]
#[expect(clippy::explicit_counter_loop, reason = "C's loop")]
fn build_ycc_rgb_table(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    let n = (MAXJSAMPLE + 1) as usize;
    let mut Cr_r_tab: Vec<i32> = alloc_small_vec(cinfo, JPOOL_IMAGE, n * size_of::<i32>(), n, 0)?;
    let mut Cb_b_tab: Vec<i32> = alloc_small_vec(cinfo, JPOOL_IMAGE, n * size_of::<i32>(), n, 0)?;
    let mut Cr_g_tab: Vec<JLONG> =
        alloc_small_vec(cinfo, JPOOL_IMAGE, n * size_of::<JLONG>(), n, 0)?;
    let mut Cb_g_tab: Vec<JLONG> =
        alloc_small_vec(cinfo, JPOOL_IMAGE, n * size_of::<JLONG>(), n, 0)?;

    let mut x: JLONG = -JLONG::from(CENTERJSAMPLE);
    for i in 0..=MAXJSAMPLE as usize {
        /* i is the actual input pixel value, in the range 0.._MAXJSAMPLE */
        /* The Cb or Cr value we are thinking of is x = i - _CENTERJSAMPLE */
        /* Cr=>R value is nearest int to 1.40200 * x */
        Cr_r_tab[i] = RIGHT_SHIFT(FIX(1.40200) * x + ONE_HALF, SCALEBITS) as i32;
        /* Cb=>B value is nearest int to 1.77200 * x */
        Cb_b_tab[i] = RIGHT_SHIFT(FIX(1.77200) * x + ONE_HALF, SCALEBITS) as i32;
        /* Cr=>G value is scaled-up -0.71414 * x */
        Cr_g_tab[i] = (-FIX(0.71414)) * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        Cb_g_tab[i] = (-FIX(0.34414)) * x + ONE_HALF;
        x += 1;
    }
    let cconvert = cconvert_mut(cinfo);
    cconvert.Cr_r_tab = Some(Cr_r_tab);
    cconvert.Cb_b_tab = Some(Cb_b_tab);
    cconvert.Cr_g_tab = Some(Cr_g_tab);
    cconvert.Cb_g_tab = Some(Cb_g_tab);
    Ok(())
}

/**************** Cases other than YCbCr -> RGB **************/

// upstream: src/jdcolor.c build_rgb_y_table
/// Initializes for RGB->grayscale colorspace conversion.
///
/// # Errors
///
/// What the allocation reports.
#[expect(clippy::cast_sign_loss, reason = "MAXJSAMPLE is positive")]
#[expect(clippy::cast_possible_truncation, reason = "C's conversions")]
fn build_rgb_y_table(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Allocate and fill in the conversion tables. */
    let mut rgb_y_tab: Vec<JLONG> = alloc_small_vec(
        cinfo,
        JPOOL_IMAGE,
        TABLE_SIZE * size_of::<JLONG>(),
        TABLE_SIZE,
        0,
    )?;

    for i in 0..=JLONG::from(MAXJSAMPLE) {
        let index = i as usize;
        rgb_y_tab[index + R_Y_OFF] = FIX(0.29900) * i;
        rgb_y_tab[index + G_Y_OFF] = FIX(0.58700) * i;
        rgb_y_tab[index + B_Y_OFF] = FIX(0.11400) * i + ONE_HALF;
    }
    cconvert_mut(cinfo).rgb_y_tab = Some(rgb_y_tab);
    Ok(())
}

// upstream: src/jdcolor.c rgb_gray_convert
/// Converts RGB to grayscale.
///
/// # Panics
///
/// When the table is absent or a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of samples and indexes"
)]
fn rgb_gray_convert(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    mut num_rows: i32,
) {
    let cconvert = cinfo.cconvert.as_deref().expect("cinfo->cconvert is NULL");
    let ctab = cconvert.rgb_y_tab.as_deref().expect("rgb_y_tab is NULL");
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
        for col in 0..num_cols as usize {
            let r = usize::from(inptr0[col]);
            let g = usize::from(inptr1[col]);
            let b = usize::from(inptr2[col]);
            /* Y */
            outptr[col] = ((ctab[r + R_Y_OFF] + ctab[g + G_Y_OFF] + ctab[b + B_Y_OFF]) >> SCALEBITS)
                as JSAMPLE;
        }
    }
}

// upstream: src/jdcolor.c null_convert
/// Color conversion for no colorspace change: just copy the data,
/// converting from separate-planes to interleaved representation.
///
/// # Panics
///
/// When a row is shorter than C reads or writes.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
#[expect(clippy::needless_range_loop, reason = "C's loop")]
fn null_convert(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    mut num_rows: i32,
) {
    let num_components = cinfo.num_components;
    let num_cols: JDIMENSION = cinfo.output_width;

    let mut out_row = 0;
    if num_components == 3 {
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
                outptr[out] = inptr0[col];
                outptr[out + 1] = inptr1[col];
                outptr[out + 2] = inptr2[col];
                out += 3;
            }
        }
    } else if num_components == 4 {
        loop {
            num_rows -= 1;
            if num_rows < 0 {
                break;
            }
            let inptr0 = input_buf[0].row(input_row as usize);
            let inptr1 = input_buf[1].row(input_row as usize);
            let inptr2 = input_buf[2].row(input_row as usize);
            let inptr3 = input_buf[3].row(input_row as usize);
            input_row += 1;
            let outptr = output_buf.row_mut(out_row);
            out_row += 1;
            let mut out = 0;
            for col in 0..num_cols as usize {
                outptr[out] = inptr0[col];
                outptr[out + 1] = inptr1[col];
                outptr[out + 2] = inptr2[col];
                outptr[out + 3] = inptr3[col];
                out += 4;
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if num_rows < 0 {
                break;
            }
            for ci in 0..num_components as usize {
                let inptr = input_buf[ci].row(input_row as usize);
                let outptr = output_buf.row_mut(out_row);
                let mut out = 0;
                for col in 0..num_cols as usize {
                    outptr[out + ci] = inptr[col];
                    out += num_components as usize;
                }
            }
            out_row += 1;
            input_row += 1;
        }
    }
}

// upstream: src/jdcolor.c grayscale_convert
/// Color conversion for grayscale: just copy the data. This also works for
/// YCbCr -> grayscale conversion, in which we just copy the Y (luminance)
/// component and ignore chrominance.
#[expect(clippy::cast_possible_wrap, reason = "C's (int)input_row")]
fn grayscale_convert(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    input_row: JDIMENSION,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    num_rows: i32,
) {
    jcopy_sample_rows(
        input_buf[0],
        input_row as i32,
        output_buf,
        0,
        num_rows,
        cinfo.output_width,
    );
}

// upstream: src/jdcolor.c ycck_cmyk_convert
/// Adobe-style YCCK->CMYK conversion. We convert YCbCr to R=1-C, G=1-M,
/// and B=1-Y using the same conversion as above, while passing K (black)
/// unchanged. We assume `build_ycc_rgb_table` has been called.
///
/// # Panics
///
/// When a table is absent or a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversions of samples and table values"
)]
#[expect(non_snake_case, reason = "C names")]
fn ycck_cmyk_convert(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    mut num_rows: i32,
) {
    let cconvert = cinfo.cconvert.as_deref().expect("cinfo->cconvert is NULL");
    let num_cols: JDIMENSION = cinfo.output_width;
    /* copy these pointers into registers if possible */
    let range_limit = cinfo.sample_range_limit_signed();
    let Crrtab = cconvert.Cr_r_tab.as_deref().expect("Cr_r_tab is NULL");
    let Cbbtab = cconvert.Cb_b_tab.as_deref().expect("Cb_b_tab is NULL");
    let Crgtab = cconvert.Cr_g_tab.as_deref().expect("Cr_g_tab is NULL");
    let Cbgtab = cconvert.Cb_g_tab.as_deref().expect("Cb_g_tab is NULL");

    let mut out_row = 0;
    loop {
        num_rows -= 1;
        if num_rows < 0 {
            break;
        }
        let inptr0 = input_buf[0].row(input_row as usize);
        let inptr1 = input_buf[1].row(input_row as usize);
        let inptr2 = input_buf[2].row(input_row as usize);
        let inptr3 = input_buf[3].row(input_row as usize);
        input_row += 1;
        let outptr = output_buf.row_mut(out_row);
        out_row += 1;
        let mut out = 0;
        for col in 0..num_cols as usize {
            let y = i32::from(inptr0[col]);
            let cb = usize::from(inptr1[col]);
            let cr = usize::from(inptr2[col]);
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            outptr[out] = range_limit.at(MAXJSAMPLE - (y + Crrtab[cr])); /* red */
            outptr[out + 1] = range_limit
                .at(MAXJSAMPLE - (y + RIGHT_SHIFT(Cbgtab[cb] + Crgtab[cr], SCALEBITS) as i32)); /* green */
            outptr[out + 2] = range_limit.at(MAXJSAMPLE - (y + Cbbtab[cb])); /* blue */
            /* K passes through unchanged */
            outptr[out + 3] = inptr3[col];
            out += 4;
        }
    }
}

// upstream: src/jdcolor.c start_pass_dcolor
/// Empty method for `start_pass`.
fn start_pass_dcolor(_cinfo: j_decompress_ptr<'_, '_>) {
    /* no work needed */
}

/// The table index of a color space, which C uses as a subscript. Not an
/// upstream item.
fn cs_index(cs: J_COLOR_SPACE) -> usize {
    usize::try_from(cs.0).expect("color space index")
}

// upstream: src/jdcolor.c _jinit_color_deconverter
/// Module initialization routine for output colorspace conversion.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_BAD_J_COLORSPACE`, `JERR_CONVERSION_NOTIMPL`,
/// and what the allocations report.
///
/// # Panics
///
/// When `cinfo.comp_info` is absent.
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
pub fn jinit_color_deconverter(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    let cconvert = alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_my_color_deconverter,
        my_color_deconverter {
            pub_: jpeg_color_deconverter {
                start_pass: start_pass_dcolor,
                color_convert: None,
                #[cfg(with_simd)]
                color_convert_simd: None,
            },
            Cr_r_tab: None,
            Cb_b_tab: None,
            Cr_g_tab: None,
            Cb_g_tab: None,
            rgb_y_tab: None,
        },
    )?;
    cinfo.cconvert = Some(cconvert);
    let set = |cinfo: j_decompress_ptr<'_, '_>, color_convert: color_convert_method_ptr| {
        cconvert_mut(cinfo).pub_.color_convert = Some(color_convert);
    };

    /* Make sure num_components agrees with jpeg_color_space */
    match cinfo.jpeg_color_space {
        J_COLOR_SPACE::JCS_GRAYSCALE => {
            if cinfo.num_components != 1 {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_J_COLORSPACE));
            }
        }
        J_COLOR_SPACE::JCS_RGB | J_COLOR_SPACE::JCS_YCbCr => {
            if cinfo.num_components != 3 {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_J_COLORSPACE));
            }
        }
        J_COLOR_SPACE::JCS_CMYK | J_COLOR_SPACE::JCS_YCCK => {
            if cinfo.num_components != 4 {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_J_COLORSPACE));
            }
        }
        _ => {
            /* JCS_UNKNOWN can be anything */
            if cinfo.num_components < 1 {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_J_COLORSPACE));
            }
        }
    }

    /* Set out_color_components and conversion method based on requested space.
     * Also clear the component_needed flags for any unused components,
     * so that earlier pipeline stages can avoid useless computation.
     * NOTE: We do not allow any lossy color conversion algorithms in lossless
     * mode.
     */

    match cinfo.out_color_space {
        J_COLOR_SPACE::JCS_GRAYSCALE => {
            cinfo.out_color_components = 1;
            if cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_GRAYSCALE
                || cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_YCbCr
            {
                set(cinfo, grayscale_convert);
                /* For color->grayscale conversion, only the Y (0) component is needed */
                for ci in 1..cinfo.num_components as usize {
                    cinfo.comp_info_mut()[ci].component_needed = false;
                }
            } else if cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_RGB {
                set(cinfo, rgb_gray_convert);
                build_rgb_y_table(cinfo)?;
            } else {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL));
            }
        }

        J_COLOR_SPACE::JCS_RGB
        | J_COLOR_SPACE::JCS_EXT_RGB
        | J_COLOR_SPACE::JCS_EXT_RGBX
        | J_COLOR_SPACE::JCS_EXT_BGR
        | J_COLOR_SPACE::JCS_EXT_BGRX
        | J_COLOR_SPACE::JCS_EXT_XBGR
        | J_COLOR_SPACE::JCS_EXT_XRGB
        | J_COLOR_SPACE::JCS_EXT_RGBA
        | J_COLOR_SPACE::JCS_EXT_BGRA
        | J_COLOR_SPACE::JCS_EXT_ABGR
        | J_COLOR_SPACE::JCS_EXT_ARGB => {
            let out = cs_index(cinfo.out_color_space);
            cinfo.out_color_components = rgb_pixelsize[out];
            if cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_YCbCr {
                #[cfg_attr(
                    not(with_simd),
                    expect(unused_labels, reason = "the label serves the WITH_SIMD branch")
                )]
                'simd: {
                    #[cfg(with_simd)]
                    if jsimd_set_ycc_rgb(cinfo) != 0 {
                        set(cinfo, jsimd_color_deconvert);
                        break 'simd;
                    }
                    let color_convert = match cinfo.out_color_space {
                        J_COLOR_SPACE::JCS_EXT_RGB => ycc_extrgb_convert,
                        J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                            ycc_extrgbx_convert
                        }
                        J_COLOR_SPACE::JCS_EXT_BGR => ycc_extbgr_convert,
                        J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                            ycc_extbgrx_convert
                        }
                        J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                            ycc_extxbgr_convert
                        }
                        J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                            ycc_extxrgb_convert
                        }
                        _ => ycc_rgb_convert,
                    };
                    set(cinfo, color_convert);
                    build_ycc_rgb_table(cinfo)?;
                }
            } else if cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_GRAYSCALE {
                let color_convert = match cinfo.out_color_space {
                    J_COLOR_SPACE::JCS_EXT_RGB => gray_extrgb_convert,
                    J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                        gray_extrgbx_convert
                    }
                    J_COLOR_SPACE::JCS_EXT_BGR => gray_extbgr_convert,
                    J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                        gray_extbgrx_convert
                    }
                    J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                        gray_extxbgr_convert
                    }
                    J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                        gray_extxrgb_convert
                    }
                    _ => gray_rgb_convert,
                };
                set(cinfo, color_convert);
            } else if cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_RGB {
                if rgb_red[out] == 0
                    && rgb_green[out] == 1
                    && rgb_blue[out] == 2
                    && rgb_pixelsize[out] == 3
                {
                    set(cinfo, null_convert);
                } else {
                    let color_convert = match cinfo.out_color_space {
                        J_COLOR_SPACE::JCS_EXT_RGB => rgb_extrgb_convert,
                        J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                            rgb_extrgbx_convert
                        }
                        J_COLOR_SPACE::JCS_EXT_BGR => rgb_extbgr_convert,
                        J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                            rgb_extbgrx_convert
                        }
                        J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                            rgb_extxbgr_convert
                        }
                        J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                            rgb_extxrgb_convert
                        }
                        _ => rgb_rgb_convert,
                    };
                    set(cinfo, color_convert);
                }
            } else {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL));
            }
        }

        J_COLOR_SPACE::JCS_RGB565 => {
            /* RGB565 output is out of scope (ADR 0005) */
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL));
        }

        J_COLOR_SPACE::JCS_CMYK => {
            cinfo.out_color_components = 4;
            if cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_YCCK {
                set(cinfo, ycck_cmyk_convert);
                build_ycc_rgb_table(cinfo)?;
            } else if cinfo.jpeg_color_space == J_COLOR_SPACE::JCS_CMYK {
                set(cinfo, null_convert);
            } else {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL));
            }
        }

        _ => {
            /* Permit null conversion to same output space */
            if cinfo.out_color_space == cinfo.jpeg_color_space {
                cinfo.out_color_components = cinfo.num_components;
                set(cinfo, null_convert);
            } else {
                /* unsupported non-null conversion */
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL));
            }
        }
    }

    if cinfo.quantize_colors {
        cinfo.output_components = 1; /* single colormapped output component */
    } else {
        cinfo.output_components = cinfo.out_color_components;
    }
    Ok(())
}
