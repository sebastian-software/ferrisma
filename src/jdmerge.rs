// libjpeg-turbo 3.2.1, src/jdmerge.h:
//
// /*
//  * jdmerge.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2020, 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdmerge.c:
//
// /*
//  * jdmerge.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2009, 2011, 2014-2015, 2020, 2022, 2025, D. R. Commander.
//  * Copyright (C) 2013, Linaro Limited.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains code for merged upsampling/color conversion.
//  *
//  * This file combines functions from jdsample.c and jdcolor.c;
//  * read those files first to understand what's going on.
//  *
//  * When the chroma components are to be upsampled by simple replication
//  * (ie, box filtering), we can save some work in color conversion by
//  * calculating all the output pixels corresponding to a pair of chroma
//  * samples at one time.  In the conversion equations
//  *      R = Y           + K1 * Cr
//  *      G = Y + K2 * Cb + K3 * Cr
//  *      B = Y + K4 * Cb
//  * only the Y term varies among the group of pixels corresponding to a pair
//  * of chroma samples, so the rest of the terms can be calculated just once.
//  * At typical sampling ratios, this eliminates half or three-quarters of the
//  * multiplications needed for color conversion.
//  *
//  * This file currently provides implementations for the following cases:
//  *      YCbCr => RGB color conversion only.
//  *      Sampling ratios of 2h1v or 2h2v.
//  *      No scaling needed at upsample time.
//  *      Corner-aligned (non-CCIR601) sampling alignment.
//  * Other special cases could be added, but in most applications these are
//  * the only common cases.  (For uncommon cases we fall back on the more
//  * general code in jdsample.c and jdcolor.c.)
//  */
//! Translation of `src/jdmerge.h` and `src/jdmerge.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
//!
//! `UPSAMPLE_MERGING_SUPPORTED` is defined and `WITH_PROFILE` is not.
//! `WITH_SIMD` is `cfg(with_simd)` (ADR 0007): `jinit_merged_upsampler` then
//! asks `jsimd_set_h2v1_merged_upsample` or `jsimd_set_h2v2_merged_upsample`
//! for a SIMD kernel first, in a labeled block that yields the `jsimd`
//! wrapper as `upmethod` when it gets one, as C's `if ... else` across the
//! `#ifdef` skips the C routines. The RGB565 routines of `jdmrg565.c` are out of
//! scope: `JCS_RGB565` output fails with `JERR_CONVERSION_NOTIMPL` where
//! `jinit_merged_upsampler` would select them (ADR 0005, "Out-of-scope
//! branches"). The instances of `jdmrgext.c`'s routines are constants that
//! name a generic function of `jdmrgext`. The control routines take the
//! module out of `cinfo.upsample` and pass it to `upmethod`; for the 2:1
//! vertical case they take the spare row out of the module for the call, so
//! that it can be the second output row.

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jdmrgext;
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemmgr::{alloc_large_vec, alloc_small_box, alloc_small_vec};
use crate::jmorecfg::{
    C_BOOLEAN, C_JDIMENSION, C_POINTER, CENTERJSAMPLE, CLayout, EXT_BGR_BLUE, EXT_BGR_GREEN,
    EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE,
    EXT_BGRX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_RGBX_BLUE,
    EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN,
    EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE,
    EXT_XRGB_RED, JDIMENSION, JSAMPLE, MAXJSAMPLE, RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED,
};
use crate::jpegint::{JLONG, RIGHT_SHIFT, jpeg_upsampler, jpeg_upsampler_impl};
use crate::jpeglib::{
    J_COLOR_SPACE, JPOOL_IMAGE, JSAMPROWS, JSAMPROWS_AT, JSAMPROWS_MUT, j_decompress_ptr,
    jpeg_decompress_struct,
};
#[cfg(with_simd)]
use crate::jsimd::{
    jsimd_h2v1_merged_upsample, jsimd_h2v2_merged_upsample, jsimd_set_h2v1_merged_upsample,
    jsimd_set_h2v2_merged_upsample,
};
use crate::jutils::jcopy_sample_rows;

/// The type of `my_merged_upsampler::upmethod`: upsamples and color-converts
/// one row group into one or two output rows. Not an upstream item.
#[expect(non_camel_case_types, reason = "named after the C field")]
pub type merged_upmethod_ptr = fn(
    &jpeg_decompress_struct<'_>,
    &my_merged_upsampler,
    &[JSAMPROWS_AT<'_>],
    JDIMENSION,
    &mut [&mut [JSAMPLE]],
);

// upstream: src/jdmerge.h my_merged_upsampler
/// Private subobject.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Debug)]
pub struct my_merged_upsampler {
    /// Public fields.
    pub pub_: jpeg_upsampler,

    /// Routine to do actual upsampling/conversion of one row group; `None`
    /// until `jinit_merged_upsampler` selects it.
    pub upmethod: Option<merged_upmethod_ptr>,

    /* Private state for YCC->RGB conversion */
    /// Table for Cr to R conversion.
    pub Cr_r_tab: Option<Vec<i32>>,
    /// Table for Cb to B conversion.
    pub Cb_b_tab: Option<Vec<i32>>,
    /// Table for Cr to G conversion.
    pub Cr_g_tab: Option<Vec<JLONG>>,
    /// Table for Cb to G conversion.
    pub Cb_g_tab: Option<Vec<JLONG>>,

    /// For 2:1 vertical sampling, we produce two output rows at a time. We
    /// need a "spare" row buffer to hold the second output row if the
    /// application provides just a one-row buffer; we also use the spare to
    /// discard the dummy last row if the image height is odd. `None` in the
    /// 1:1 case.
    pub spare_row: Option<Vec<JSAMPLE>>,
    /// `true` if spare buffer is occupied.
    pub spare_full: bool,

    /// Samples per output row.
    pub out_row_width: JDIMENSION,
    /// Counts rows remaining in image.
    pub rows_to_go: JDIMENSION,
}

/// `sizeof(my_merged_upsampler)` on the target, with the `upsample_12`,
/// `upsample_16` and four SIMD pointers of `struct jpeg_upsampler`.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_merged_upsampler: usize = CLayout::structure(&[
    // struct jpeg_upsampler
    CLayout::structure(&[C_POINTER.array(8), C_BOOLEAN]),
    C_POINTER,
    C_POINTER.array(4),
    C_POINTER,
    C_BOOLEAN,
    C_JDIMENSION,
    C_JDIMENSION,
])
.size;

// upstream: src/jdmerge.c SCALEBITS
/// Speediest right-shift on some machines.
pub const SCALEBITS: u32 = 16;
// upstream: src/jdmerge.c ONE_HALF
const ONE_HALF: JLONG = 1 << (SCALEBITS - 1);

// upstream: src/jdmerge.c FIX
/// Converts a real constant to an integer scaled by `2 ** SCALEBITS`,
/// rounding, in C's `double` arithmetic.
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "C's conversions between JLONG and double"
)]
#[inline]
const fn FIX(x: f64) -> JLONG {
    (x * ((1 as core::ffi::c_long) << SCALEBITS) as f64 + 0.5) as JLONG
}

/* Include inline routines for colorspace extensions */

// upstream: src/jdmerge.c h2v1_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const h2v1_merged_upsample: merged_upmethod_ptr =
    jdmrgext::h2v1_merged_upsample::<RGB_RED, RGB_GREEN, RGB_BLUE, -1, RGB_PIXELSIZE>;
// upstream: src/jdmerge.c h2v2_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const h2v2_merged_upsample: merged_upmethod_ptr =
    jdmrgext::h2v2_merged_upsample::<RGB_RED, RGB_GREEN, RGB_BLUE, -1, RGB_PIXELSIZE>;
// upstream: src/jdmerge.c extrgb_h2v1_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extrgb_h2v1_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v1_merged_upsample::<
    EXT_RGB_RED,
    EXT_RGB_GREEN,
    EXT_RGB_BLUE,
    -1,
    EXT_RGB_PIXELSIZE,
>;
// upstream: src/jdmerge.c extrgb_h2v2_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extrgb_h2v2_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v2_merged_upsample::<
    EXT_RGB_RED,
    EXT_RGB_GREEN,
    EXT_RGB_BLUE,
    -1,
    EXT_RGB_PIXELSIZE,
>;
// upstream: src/jdmerge.c extrgbx_h2v1_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extrgbx_h2v1_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v1_merged_upsample::<
    EXT_RGBX_RED,
    EXT_RGBX_GREEN,
    EXT_RGBX_BLUE,
    3,
    EXT_RGBX_PIXELSIZE,
>;
// upstream: src/jdmerge.c extrgbx_h2v2_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extrgbx_h2v2_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v2_merged_upsample::<
    EXT_RGBX_RED,
    EXT_RGBX_GREEN,
    EXT_RGBX_BLUE,
    3,
    EXT_RGBX_PIXELSIZE,
>;
// upstream: src/jdmerge.c extbgr_h2v1_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extbgr_h2v1_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v1_merged_upsample::<
    EXT_BGR_RED,
    EXT_BGR_GREEN,
    EXT_BGR_BLUE,
    -1,
    EXT_BGR_PIXELSIZE,
>;
// upstream: src/jdmerge.c extbgr_h2v2_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extbgr_h2v2_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v2_merged_upsample::<
    EXT_BGR_RED,
    EXT_BGR_GREEN,
    EXT_BGR_BLUE,
    -1,
    EXT_BGR_PIXELSIZE,
>;
// upstream: src/jdmerge.c extbgrx_h2v1_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extbgrx_h2v1_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v1_merged_upsample::<
    EXT_BGRX_RED,
    EXT_BGRX_GREEN,
    EXT_BGRX_BLUE,
    3,
    EXT_BGRX_PIXELSIZE,
>;
// upstream: src/jdmerge.c extbgrx_h2v2_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extbgrx_h2v2_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v2_merged_upsample::<
    EXT_BGRX_RED,
    EXT_BGRX_GREEN,
    EXT_BGRX_BLUE,
    3,
    EXT_BGRX_PIXELSIZE,
>;
// upstream: src/jdmerge.c extxbgr_h2v1_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extxbgr_h2v1_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v1_merged_upsample::<
    EXT_XBGR_RED,
    EXT_XBGR_GREEN,
    EXT_XBGR_BLUE,
    0,
    EXT_XBGR_PIXELSIZE,
>;
// upstream: src/jdmerge.c extxbgr_h2v2_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extxbgr_h2v2_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v2_merged_upsample::<
    EXT_XBGR_RED,
    EXT_XBGR_GREEN,
    EXT_XBGR_BLUE,
    0,
    EXT_XBGR_PIXELSIZE,
>;
// upstream: src/jdmerge.c extxrgb_h2v1_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extxrgb_h2v1_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v1_merged_upsample::<
    EXT_XRGB_RED,
    EXT_XRGB_GREEN,
    EXT_XRGB_BLUE,
    0,
    EXT_XRGB_PIXELSIZE,
>;
// upstream: src/jdmerge.c extxrgb_h2v2_merged_upsample
#[expect(non_upper_case_globals, reason = "C function name")]
const extxrgb_h2v2_merged_upsample: merged_upmethod_ptr = jdmrgext::h2v2_merged_upsample::<
    EXT_XRGB_RED,
    EXT_XRGB_GREEN,
    EXT_XRGB_BLUE,
    0,
    EXT_XRGB_PIXELSIZE,
>;

/// Runs `body` with the merged upsampler taken out of `cinfo.upsample` and
/// puts it back afterwards (ADR 0005, "A method's own state"). Not an
/// upstream item.
///
/// # Panics
///
/// When the slot is empty or holds `my_upsampler`.
fn with_merged<R>(
    cinfo: j_decompress_ptr<'_, '_>,
    body: impl FnOnce(j_decompress_ptr<'_, '_>, &mut my_merged_upsampler) -> R,
) -> R {
    let mut slot = cinfo.upsample.take().expect("cinfo->upsample is NULL");
    let result = match &mut *slot {
        jpeg_upsampler_impl::my_merged_upsampler(upsample) => body(cinfo, upsample),
        jpeg_upsampler_impl::my_upsampler(_) => {
            unreachable!("cinfo->upsample is not a my_merged_upsampler")
        }
    };
    cinfo.upsample = Some(slot);
    result
}

// upstream: src/jdmerge.c build_ycc_rgb_table
/// Initializes tables for YCC->RGB colorspace conversion. This is taken
/// directly from `jdcolor.c`; see that file for more info.
///
/// # Errors
///
/// What the allocations report.
///
/// # Panics
///
/// When the slot does not hold the merged upsampler.
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
    with_merged(cinfo, |_cinfo, upsample| {
        upsample.Cr_r_tab = Some(Cr_r_tab);
        upsample.Cb_b_tab = Some(Cb_b_tab);
        upsample.Cr_g_tab = Some(Cr_g_tab);
        upsample.Cb_g_tab = Some(Cb_g_tab);
    });
    Ok(())
}

// upstream: src/jdmerge.c start_pass_merged_upsample
/// Initializes for an upsampling pass.
///
/// # Panics
///
/// When the slot does not hold the merged upsampler.
pub fn start_pass_merged_upsample(cinfo: j_decompress_ptr<'_, '_>) {
    with_merged(cinfo, |cinfo, upsample| {
        /* Mark the spare buffer empty */
        upsample.spare_full = false;
        /* Initialize total-height counter for detecting bottom of image */
        upsample.rows_to_go = cinfo.output_height;
    });
}

// upstream: src/jdmerge.c merged_2v_upsample
/// Control routine to do upsampling (and color conversion). The control
/// routine just handles the row buffering considerations. 2:1 vertical
/// sampling case: may need a spare row.
///
/// # Panics
///
/// When the module, its method or its spare row is absent, or a row is
/// shorter than C reads or writes.
#[expect(clippy::similar_names, reason = "C variable names")]
pub fn merged_2v_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: &mut JDIMENSION,
    _in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    out_row_ctr: &mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    with_merged(cinfo, |cinfo, upsample| {
        let cinfo: &jpeg_decompress_struct<'_> = cinfo;
        let num_rows: JDIMENSION; /* number of rows returned to caller */

        if upsample.spare_full {
            /* If we have a spare row saved from a previous cycle, just return it. */
            let mut size: JDIMENSION = upsample.out_row_width;
            if cinfo.out_color_space == J_COLOR_SPACE::JCS_RGB565 {
                size = cinfo.output_width * 2;
            }
            let spare_row = upsample.spare_row.as_deref().expect("spare_row is NULL");
            let spare_rows = [0usize];
            jcopy_sample_rows(
                JSAMPROWS {
                    storage: spare_row,
                    rows: &spare_rows,
                },
                0,
                output_buf.offset(*out_row_ctr as usize),
                0,
                1,
                size,
            );
            num_rows = 1;
            upsample.spare_full = false;
        } else {
            /* Figure number of rows to return to caller. */
            let mut rows: JDIMENSION = 2;
            /* Not more than the distance to the end of the image. */
            if rows > upsample.rows_to_go {
                rows = upsample.rows_to_go;
            }
            /* And not more than what the client can accept: */
            out_rows_avail -= *out_row_ctr;
            if rows > out_rows_avail {
                rows = out_rows_avail;
            }
            num_rows = rows;
            /* Create output pointer array for upsampler. */
            let upmethod = upsample.upmethod.expect("upmethod is NULL");
            let mut spare_row = upsample.spare_row.take().expect("spare_row is NULL");
            if num_rows > 1 {
                let (row0, row1) = output_buf.two_rows_mut(*out_row_ctr as usize);
                let mut work_ptrs: [&mut [JSAMPLE]; 2] = [row0, row1];
                /* Now do the upsampling. */
                upmethod(
                    cinfo,
                    upsample,
                    input_buf,
                    *in_row_group_ctr,
                    &mut work_ptrs,
                );
            } else {
                upsample.spare_full = true;
                let row0 = output_buf.row_mut(*out_row_ctr as usize);
                let mut work_ptrs: [&mut [JSAMPLE]; 2] = [row0, &mut spare_row];
                /* Now do the upsampling. */
                upmethod(
                    cinfo,
                    upsample,
                    input_buf,
                    *in_row_group_ctr,
                    &mut work_ptrs,
                );
            }
            upsample.spare_row = Some(spare_row);
        }

        /* Adjust counts */
        *out_row_ctr += num_rows;
        upsample.rows_to_go -= num_rows;
        /* When the buffer is emptied, declare this input row group consumed */
        if !upsample.spare_full {
            *in_row_group_ctr += 1;
        }
    });
}

// upstream: src/jdmerge.c merged_1v_upsample
/// 1:1 vertical sampling case: much easier, never need a spare row.
///
/// # Panics
///
/// When the module or its method is absent, or a row is shorter than C
/// reads or writes.
pub fn merged_1v_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: &mut JDIMENSION,
    _in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    out_row_ctr: &mut JDIMENSION,
    _out_rows_avail: JDIMENSION,
) {
    with_merged(cinfo, |cinfo, upsample| {
        let cinfo: &jpeg_decompress_struct<'_> = cinfo;
        /* Just do the upsampling. */
        let upmethod = upsample.upmethod.expect("upmethod is NULL");
        let row = output_buf.row_mut(*out_row_ctr as usize);
        upmethod(cinfo, upsample, input_buf, *in_row_group_ctr, &mut [row]);
        /* Adjust counts */
        *out_row_ctr += 1;
        *in_row_group_ctr += 1;
    });
}

// upstream: src/jdmerge.c _jinit_merged_upsampler
/// Module initialization routine for merged upsampling/color conversion.
///
/// NB: this is called under the conditions determined by
/// `use_merged_upsample()` in `jdmaster.c`. That routine MUST correspond to
/// the actual capabilities of this module; no safety checks are made here.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_CONVERSION_NOTIMPL` for RGB565 output, and
/// what the allocations report.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
pub fn jinit_merged_upsampler(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    let out_row_width = cinfo.output_width * cinfo.out_color_components as JDIMENSION;
    let upsample = alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_my_merged_upsampler,
        jpeg_upsampler_impl::my_merged_upsampler(my_merged_upsampler {
            pub_: jpeg_upsampler {
                start_pass: start_pass_merged_upsample,
                upsample: merged_1v_upsample,
                need_context_rows: false,
                #[cfg(with_simd)]
                h2v1_upsample_simd: None,
                #[cfg(with_simd)]
                h2v2_upsample_simd: None,
                #[cfg(with_simd)]
                h1v2_upsample_simd: None,
                #[cfg(with_simd)]
                merged_upsample_simd: None,
            },
            upmethod: None,
            Cr_r_tab: None,
            Cb_b_tab: None,
            Cr_g_tab: None,
            Cb_g_tab: None,
            spare_row: None,
            spare_full: false,
            out_row_width,
            rows_to_go: 0,
        }),
    )?;
    cinfo.upsample = Some(upsample);

    let set = |cinfo: j_decompress_ptr<'_, '_>,
               upsample_fn: crate::jpegint::upsample_method_ptr,
               upmethod: merged_upmethod_ptr| {
        with_merged(cinfo, |_cinfo, upsample| {
            upsample.pub_.upsample = upsample_fn;
            upsample.upmethod = Some(upmethod);
        });
    };

    if cinfo.max_v_samp_factor == 2 {
        #[cfg_attr(
            not(with_simd),
            expect(unused_labels, reason = "the label serves the WITH_SIMD branch")
        )]
        let upmethod: merged_upmethod_ptr = 'simd: {
            #[cfg(with_simd)]
            if jsimd_set_h2v2_merged_upsample(cinfo) != 0 {
                break 'simd jsimd_h2v2_merged_upsample;
            }
            match cinfo.out_color_space {
                J_COLOR_SPACE::JCS_EXT_RGB => extrgb_h2v2_merged_upsample,
                J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                    extrgbx_h2v2_merged_upsample
                }
                J_COLOR_SPACE::JCS_EXT_BGR => extbgr_h2v2_merged_upsample,
                J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                    extbgrx_h2v2_merged_upsample
                }
                J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                    extxbgr_h2v2_merged_upsample
                }
                J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                    extxrgb_h2v2_merged_upsample
                }
                _ => h2v2_merged_upsample,
            }
        };
        set(cinfo, merged_2v_upsample, upmethod);
        if cinfo.out_color_space == J_COLOR_SPACE::JCS_RGB565 {
            /* RGB565 output is out of scope (ADR 0005) */
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL));
        }
        /* Allocate a spare row buffer */
        let len = out_row_width as usize;
        let spare_row = alloc_large_vec(cinfo, JPOOL_IMAGE, len * size_of::<JSAMPLE>(), len, 0)?;
        with_merged(cinfo, |_cinfo, upsample| {
            upsample.spare_row = Some(spare_row);
        });
    } else {
        #[cfg_attr(
            not(with_simd),
            expect(unused_labels, reason = "the label serves the WITH_SIMD branch")
        )]
        let upmethod: merged_upmethod_ptr = 'simd: {
            #[cfg(with_simd)]
            if jsimd_set_h2v1_merged_upsample(cinfo) != 0 {
                break 'simd jsimd_h2v1_merged_upsample;
            }
            match cinfo.out_color_space {
                J_COLOR_SPACE::JCS_EXT_RGB => extrgb_h2v1_merged_upsample,
                J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                    extrgbx_h2v1_merged_upsample
                }
                J_COLOR_SPACE::JCS_EXT_BGR => extbgr_h2v1_merged_upsample,
                J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                    extbgrx_h2v1_merged_upsample
                }
                J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                    extxbgr_h2v1_merged_upsample
                }
                J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                    extxrgb_h2v1_merged_upsample
                }
                _ => h2v1_merged_upsample,
            }
        };
        set(cinfo, merged_1v_upsample, upmethod);
        if cinfo.out_color_space == J_COLOR_SPACE::JCS_RGB565 {
            /* RGB565 output is out of scope (ADR 0005) */
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL));
        }
        /* No spare row needed */
        with_merged(cinfo, |_cinfo, upsample| upsample.spare_row = None);
    }

    build_ycc_rgb_table(cinfo)
}
