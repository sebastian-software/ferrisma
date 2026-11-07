// libjpeg-turbo 3.2.1, src/jdsample.h:
//
// /*
//  * jdsample.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdsample.c:
//
// /*
//  * jdsample.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2010, 2015-2016, 2022, 2024-2026, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * Copyright (C) 2019-2020, Arm Limited.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains upsampling routines.
//  *
//  * Upsampling input data is counted in "row groups".  A row group is defined to
//  * be (v_samp_factor * DCT_scaled_size / min_DCT_scaled_size) sample rows of
//  * each component.  Upsampling will normally produce max_v_samp_factor rows of
//  * each component from each row group (but this could vary if the upsampler is
//  * applying a scale factor of its own).
//  *
//  * An excellent reference for image resampling is
//  *   Digital Image Warping, George Wolberg, 1990.
//  *   Pub. by IEEE Computer Society Press, Los Alamitos, CA. ISBN 0-8186-8944-7.
//  */
//! Translation of `src/jdsample.h` and `src/jdsample.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
//!
//! `BITS_IN_JSAMPLE` is 8 and `WITH_SIMD` and `WITH_PROFILE` are undefined.
//! A component's `color_buf` entry is its own sample array, the input rows
//! that `fullsize_upsample` points it at, or nothing ([`ColorBuf`]); the
//! input rows are resolved against the `input_buf` of the call that uses
//! them, which is the `input_buf` C's pointer was taken from, because the
//! main controller passes the same rows until the row group is consumed.
//! `sep_upsample` takes its module out of `cinfo.upsample` for its duration
//! and passes it to the per-component methods, which C give only `cinfo`;
//! the color converter they call never reaches the upsampler.

use core::ffi::c_long;

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemmgr::alloc_small_box;
use crate::jmorecfg::{
    C_BOOLEAN, C_INT, C_JDIMENSION, C_POINTER, CLayout, JDIMENSION, JSAMPLE, MAX_COMPONENTS, UINT8,
};
use crate::jpegint::{jpeg_upsampler, jpeg_upsampler_impl};
use crate::jpeglib::{
    JPOOL_IMAGE, JSAMPROWS, JSAMPROWS_AT, JSAMPROWS_MUT, RowList, SampleArray, j_decompress_ptr,
    jpeg_component_info, jpeg_decompress_struct,
};
use crate::jutils::jround_up;

/// A component's entry in `color_buf`: C's `JSAMPARRAY` that points either
/// at storage of the upsampler or at the input data. Not an upstream item.
#[derive(Clone, Debug, Default)]
pub enum ColorBuf {
    /// C's null pointer: no storage and no input rows yet, or a component
    /// that is not upsampled.
    #[default]
    None,
    /// The upsampler's own rows from `alloc_sarray`.
    Own(SampleArray),
    /// The rows of the component's `input_buf` from this index of its row
    /// list on.
    Input(usize),
}

// upstream: src/jdsample.h upsample1_ptr
/// Pointer to routine to upsample a single component. It receives the
/// upsampler, which C reaches through `cinfo`, and the component's
/// `color_buf` entry, which `sep_upsample` takes out of it for the call.
#[expect(non_camel_case_types, reason = "C type name")]
pub type upsample1_ptr = fn(
    &jpeg_decompress_struct<'_>,
    &my_upsampler,
    &jpeg_component_info,
    JSAMPROWS_AT<'_>,
    &mut ColorBuf,
);

// upstream: src/jdsample.h my_upsampler
/// Private subobject.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug)]
pub struct my_upsampler {
    /// Public fields.
    pub pub_: jpeg_upsampler,

    /// Color conversion buffer. When using separate upsampling and color
    /// conversion steps, this buffer holds one upsampled row group until it
    /// has been color converted and output. Note: we do not allocate any
    /// storage for component(s) which are full-size, ie do not need
    /// rescaling. The corresponding entry of `color_buf[]` is simply set to
    /// point to the input data array, thereby avoiding copying.
    pub color_buf: [ColorBuf; MAX_COMPONENTS],

    /// Per-component upsampling method pointers; `None` where C leaves them
    /// unset.
    pub methods: [Option<upsample1_ptr>; MAX_COMPONENTS],

    /// Counts rows emitted from `color_buf`.
    pub next_row_out: i32,
    /// Counts rows remaining in image.
    pub rows_to_go: JDIMENSION,

    /// Height of an input row group for each component.
    pub rowgroup_height: [i32; MAX_COMPONENTS],

    /// These arrays save component expansion factors so that `int_expand`
    /// need not recompute them each time. They are unused for other
    /// upsampling methods.
    pub h_expand: [UINT8; MAX_COMPONENTS],
    /// See `h_expand`.
    pub v_expand: [UINT8; MAX_COMPONENTS],
}

/// `sizeof(my_upsampler)` on the target, with the `upsample_12`,
/// `upsample_16` and four SIMD pointers of `struct jpeg_upsampler`.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_upsampler: usize = CLayout::structure(&[
    // struct jpeg_upsampler
    CLayout::structure(&[C_POINTER.array(8), C_BOOLEAN]),
    C_POINTER.array(MAX_COMPONENTS),
    C_POINTER.array(MAX_COMPONENTS),
    C_INT,
    C_JDIMENSION,
    C_INT.array(MAX_COMPONENTS),
    CLayout::of::<UINT8>().array(MAX_COMPONENTS),
    CLayout::of::<UINT8>().array(MAX_COMPONENTS),
])
.size;

/// Runs `body` with the upsampler taken out of `cinfo.upsample` and puts it
/// back afterwards (ADR 0005, "A method's own state"). Not an upstream item.
///
/// # Panics
///
/// When the slot is empty or holds the merged upsampler.
fn with_upsample<R>(
    cinfo: j_decompress_ptr<'_, '_>,
    body: impl FnOnce(j_decompress_ptr<'_, '_>, &mut my_upsampler) -> R,
) -> R {
    let mut slot = cinfo.upsample.take().expect("cinfo->upsample is NULL");
    let result = match &mut *slot {
        jpeg_upsampler_impl::my_upsampler(upsample) => body(cinfo, upsample),
        jpeg_upsampler_impl::my_merged_upsampler(_) => {
            unreachable!("cinfo->upsample is not a my_upsampler")
        }
    };
    cinfo.upsample = Some(slot);
    result
}

// upstream: src/jdsample.c start_pass_upsample
/// Initializes for an upsampling pass.
///
/// # Panics
///
/// When `cinfo.upsample` does not hold `my_upsampler`.
pub fn start_pass_upsample(cinfo: j_decompress_ptr<'_, '_>) {
    with_upsample(cinfo, |cinfo, upsample| {
        /* Mark the conversion buffer empty */
        upsample.next_row_out = cinfo.max_v_samp_factor;
        /* Initialize total-height counter for detecting bottom of image */
        upsample.rows_to_go = cinfo.output_height;
    });
}

// upstream: src/jdsample.c sep_upsample
/// Control routine to do upsampling (and color conversion). In this version
/// we upsample each component independently. We upsample one row group into
/// the conversion buffer, then apply color conversion a row at a time.
///
/// # Panics
///
/// When a module, a method or a buffer is absent, or a row is shorter than C
/// reads or writes.
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "C's conversions between int and JDIMENSION"
)]
#[expect(clippy::needless_range_loop, reason = "C's loop")]
pub fn sep_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: &mut JDIMENSION,
    _in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    out_row_ctr: &mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    with_upsample(cinfo, |cinfo, upsample| {
        let cinfo: &jpeg_decompress_struct<'_> = cinfo;
        /* Fill the conversion buffer, if it's empty */
        if upsample.next_row_out >= cinfo.max_v_samp_factor {
            for ci in 0..cinfo.num_components as usize {
                let compptr = &cinfo.comp_info()[ci];
                /* Invoke per-component upsample method.  Notice we pass a POINTER
                 * to color_buf[ci], so that fullsize_upsample can change it.
                 */
                let method = upsample.methods[ci].expect("upsample->methods[ci] is NULL");
                let mut color_buf = core::mem::take(&mut upsample.color_buf[ci]);
                method(
                    cinfo,
                    upsample,
                    compptr,
                    input_buf[ci].offset(
                        (*in_row_group_ctr as usize) * upsample.rowgroup_height[ci] as usize,
                    ),
                    &mut color_buf,
                );
                upsample.color_buf[ci] = color_buf;
            }
            upsample.next_row_out = 0;
        }

        /* Color-convert and emit rows */

        /* How many we have in the buffer: */
        let mut num_rows = (cinfo.max_v_samp_factor - upsample.next_row_out) as JDIMENSION;
        /* Not more than the distance to the end of the image.  Need this test
         * in case the image height is not a multiple of max_v_samp_factor:
         */
        if num_rows > upsample.rows_to_go {
            num_rows = upsample.rows_to_go;
        }
        /* And not more than what the client can accept: */
        out_rows_avail -= *out_row_ctr;
        if num_rows > out_rows_avail {
            num_rows = out_rows_avail;
        }

        let mut color_image: [JSAMPROWS<'_>; MAX_COMPONENTS] = [RowList {
            storage: &[],
            rows: &[],
        }; MAX_COMPONENTS];
        for (ci, entry) in color_image
            .iter_mut()
            .enumerate()
            .take(cinfo.num_components as usize)
        {
            *entry = match &upsample.color_buf[ci] {
                ColorBuf::None => RowList {
                    storage: &[],
                    rows: &[],
                },
                ColorBuf::Own(array) => array.as_rows(),
                ColorBuf::Input(index) => RowList {
                    storage: input_buf[ci].storage,
                    rows: &input_buf[ci].rows[*index..],
                },
            };
        }
        let color_convert = cinfo
            .cconvert()
            .color_convert
            .expect("color_convert is NULL");
        color_convert(
            cinfo,
            &color_image[..cinfo.num_components as usize],
            upsample.next_row_out as JDIMENSION,
            output_buf.offset(*out_row_ctr as usize),
            num_rows as i32,
        );

        /* Adjust counts */
        *out_row_ctr += num_rows;
        upsample.rows_to_go -= num_rows;
        upsample.next_row_out += num_rows as i32;
        /* When the buffer is emptied, declare this input row group consumed */
        if upsample.next_row_out >= cinfo.max_v_samp_factor {
            *in_row_group_ctr += 1;
        }
    });
}

/*
 * These are the routines invoked by sep_upsample to upsample pixel values
 * of a single component.  One row group is processed per call.
 */

/// The upsampler's own rows behind a `color_buf` entry, which C's methods
/// write through `*output_data_ptr`. Not an upstream item.
///
/// # Panics
///
/// When the entry is not the upsampler's own storage.
fn own_rows(output_data_ptr: &mut ColorBuf) -> &mut SampleArray {
    match output_data_ptr {
        ColorBuf::Own(array) => array,
        _ => panic!("color_buf is not the upsampler's storage"),
    }
}

// upstream: src/jdsample.c fullsize_upsample
/// For full-size components, we just make `color_buf[ci]` point at the
/// input buffer, and thus avoid copying any data. Note that this is safe
/// only because `sep_upsample` doesn't declare the input row group "consumed"
/// until we are done color converting and emitting it.
pub fn fullsize_upsample(
    _cinfo: &jpeg_decompress_struct<'_>,
    _upsample: &my_upsampler,
    _compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    *output_data_ptr = ColorBuf::Input(input_data.origin);
}

// upstream: src/jdsample.c noop_upsample
/// This is a no-op version used for "uninteresting" components. These
/// components will not be referenced by color conversion.
pub fn noop_upsample(
    _cinfo: &jpeg_decompress_struct<'_>,
    _upsample: &my_upsampler,
    _compptr: &jpeg_component_info,
    _input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    *output_data_ptr = ColorBuf::None; /* safety check */
}

// upstream: src/jdsample.c int_upsample
/// This version handles any integral sampling ratios. This is not used for
/// typical JPEG files, so it need not be fast. Nor, for that matter, is it
/// particularly accurate: the algorithm is simple replication of the input
/// pixel onto the corresponding output pixels. The hi-falutin sampling
/// literature refers to this as a "box filter". In general the
/// characteristics of a box filter are better than one might expect, but
/// they are not quite as good as one would hope.
///
/// # Panics
///
/// When `color_buf` is not the upsampler's storage, or a row is shorter than
/// C reads or writes.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
pub fn int_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_upsampler,
    compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let output_data = own_rows(output_data_ptr);

    let h_expand = i32::from(upsample.h_expand[compptr.component_index as usize]);
    let v_expand = i32::from(upsample.v_expand[compptr.component_index as usize]);

    let mut inrow: isize = 0;
    let mut outrow: i32 = 0;
    while outrow < cinfo.max_v_samp_factor {
        /* Generate one output row with proper horizontal expansion */
        let inptr = input_data.row(inrow);
        let outptr = output_data.row_mut(outrow as usize);
        let outend = cinfo.output_width as usize;
        let mut out = 0;
        let mut inp = 0;
        while out < outend {
            let invalue: JSAMPLE = inptr[inp];
            inp += 1;
            let mut h = h_expand;
            while h > 0 {
                outptr[out] = invalue;
                out += 1;
                h -= 1;
            }
        }
        /* Generate any additional output rows by duplicating the first one */
        if v_expand > 1 {
            let mut rows = output_data.as_rows_mut();
            for k in 0..(v_expand - 1) as usize {
                rows.copy_row(
                    outrow as usize + k,
                    outrow as usize + 1 + k,
                    cinfo.output_width as usize,
                );
            }
        }
        inrow += 1;
        outrow += v_expand;
    }
}

// upstream: src/jdsample.c h2v1_upsample
/// Fast processing for the common case of 2:1 horizontal and 1:1 vertical.
/// It's still a box filter.
///
/// # Panics
///
/// When `color_buf` is not the upsampler's storage, or a row is shorter than
/// C reads or writes.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
#[expect(clippy::cast_possible_wrap, reason = "C's conversions")]
pub fn h2v1_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    _upsample: &my_upsampler,
    _compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let output_data = own_rows(output_data_ptr);

    for inrow in 0..cinfo.max_v_samp_factor as usize {
        let inptr = input_data.row(inrow as isize);
        let outptr = output_data.row_mut(inrow);
        let outend = cinfo.output_width as usize;
        let mut out = 0;
        let mut inp = 0;
        while out < outend {
            let invalue: JSAMPLE = inptr[inp];
            inp += 1;
            outptr[out] = invalue;
            outptr[out + 1] = invalue;
            out += 2;
        }
    }
}

// upstream: src/jdsample.c h2v2_upsample
/// Fast processing for the common case of 2:1 horizontal and 2:1 vertical.
/// It's still a box filter.
///
/// # Panics
///
/// When `color_buf` is not the upsampler's storage, or a row is shorter than
/// C reads or writes.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
pub fn h2v2_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    _upsample: &my_upsampler,
    _compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let output_data = own_rows(output_data_ptr);

    let mut inrow: isize = 0;
    let mut outrow: i32 = 0;
    while outrow < cinfo.max_v_samp_factor {
        let inptr = input_data.row(inrow);
        let outptr = output_data.row_mut(outrow as usize);
        let outend = cinfo.output_width as usize;
        let mut out = 0;
        let mut inp = 0;
        while out < outend {
            let invalue: JSAMPLE = inptr[inp];
            inp += 1;
            outptr[out] = invalue;
            outptr[out + 1] = invalue;
            out += 2;
        }
        output_data.as_rows_mut().copy_row(
            outrow as usize,
            outrow as usize + 1,
            cinfo.output_width as usize,
        );
        inrow += 1;
        outrow += 2;
    }
}

// upstream: src/jdsample.c h2v1_fancy_upsample
/// Fancy processing for the common case of 2:1 horizontal and 1:1 vertical.
///
/// The upsampling algorithm is linear interpolation between pixel centers,
/// also known as a "triangle filter". This is a good compromise between
/// speed and visual quality. The centers of the output pixels are 1/4 and
/// 3/4 of the way between input pixel centers.
///
/// A note about the "bias" calculations: when rounding fractional values to
/// integer, we do not want to always round 0.5 up to the next integer. If we
/// did that, we'd introduce a noticeable bias towards larger values.
/// Instead, this code is arranged so that 0.5 will be rounded up or down at
/// alternate pixel locations (a simple ordered dither pattern).
///
/// # Panics
///
/// When `color_buf` is not the upsampler's storage, or a row is shorter than
/// C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions to JSAMPLE and of nonnegative counts"
)]
#[expect(clippy::cast_possible_wrap, reason = "C's conversions")]
pub fn h2v1_fancy_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    _upsample: &my_upsampler,
    compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let output_data = own_rows(output_data_ptr);

    for inrow in 0..cinfo.max_v_samp_factor as usize {
        let inptr = input_data.row(inrow as isize);
        let outptr = output_data.row_mut(inrow);
        let mut inp = 0;
        let mut out = 0;
        /* Special case for first column */
        let mut invalue = i32::from(inptr[inp]);
        inp += 1;
        outptr[out] = invalue as JSAMPLE;
        outptr[out + 1] = ((invalue * 3 + i32::from(inptr[inp]) + 2) >> 2) as JSAMPLE;
        out += 2;

        let mut colctr = compptr.downsampled_width - 2;
        while colctr > 0 {
            /* General case: 3/4 * nearer component + 1/4 * further component */
            invalue = i32::from(inptr[inp]) * 3;
            inp += 1;
            outptr[out] = ((invalue + i32::from(inptr[inp - 2]) + 1) >> 2) as JSAMPLE;
            outptr[out + 1] = ((invalue + i32::from(inptr[inp]) + 2) >> 2) as JSAMPLE;
            out += 2;
            colctr -= 1;
        }

        /* Special case for last column */
        invalue = i32::from(inptr[inp]);
        outptr[out] = ((invalue * 3 + i32::from(inptr[inp - 1]) + 1) >> 2) as JSAMPLE;
        outptr[out + 1] = invalue as JSAMPLE;
    }
}

// upstream: src/jdsample.c h1v2_fancy_upsample
/// Fancy processing for 1:1 horizontal and 2:1 vertical (4:4:0
/// subsampling). This is a less common case, but it can be encountered when
/// losslessly rotating/transposing a JPEG file that uses 4:2:2 chroma
/// subsampling.
///
/// # Panics
///
/// When `color_buf` is not the upsampler's storage, or a row is shorter than
/// C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions to JSAMPLE and of nonnegative counts"
)]
#[expect(clippy::cast_possible_wrap, reason = "C's conversions")]
pub fn h1v2_fancy_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    _upsample: &my_upsampler,
    compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let output_data = own_rows(output_data_ptr);

    let mut inrow: isize = 0;
    let mut outrow: usize = 0;
    while (outrow as i32) < cinfo.max_v_samp_factor {
        for v in 0..2 {
            /* inptr0 points to nearest input row, inptr1 points to next nearest */
            let inptr0 = input_data.row(inrow);
            let (inptr1, bias) = if v == 0 {
                /* next nearest is row above */
                (input_data.row(inrow - 1), 1)
            } else {
                /* next nearest is row below */
                (input_data.row(inrow + 1), 2)
            };
            let outptr = output_data.row_mut(outrow);
            outrow += 1;

            for colctr in 0..compptr.downsampled_width as usize {
                let thiscolsum: i32 = i32::from(inptr0[colctr]) * 3 + i32::from(inptr1[colctr]);
                outptr[colctr] = ((thiscolsum + bias) >> 2) as JSAMPLE;
            }
        }
        inrow += 1;
    }
}

// upstream: src/jdsample.c h2v2_fancy_upsample
/// Fancy processing for the common case of 2:1 horizontal and 2:1 vertical.
/// Again a triangle filter; see comments for `h2v1` case, above.
///
/// It is OK for us to reference the adjacent input rows because we demanded
/// context from the main buffer controller (see initialization code).
///
/// # Panics
///
/// When `color_buf` is not the upsampler's storage, or a row is shorter than
/// C reads or writes.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions to JSAMPLE and of nonnegative counts"
)]
#[expect(clippy::cast_possible_wrap, reason = "C's conversions")]
pub fn h2v2_fancy_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    _upsample: &my_upsampler,
    compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let output_data = own_rows(output_data_ptr);

    let mut inrow: isize = 0;
    let mut outrow: usize = 0;
    while (outrow as i32) < cinfo.max_v_samp_factor {
        for v in 0..2 {
            /* inptr0 points to nearest input row, inptr1 points to next nearest */
            let inptr0 = input_data.row(inrow);
            let inptr1 = if v == 0 {
                /* next nearest is row above */
                input_data.row(inrow - 1)
            } else {
                /* next nearest is row below */
                input_data.row(inrow + 1)
            };
            let outptr = output_data.row_mut(outrow);
            outrow += 1;

            let mut inp = 0;
            let mut out = 0;
            /* Special case for first column */
            let mut thiscolsum: i32 = i32::from(inptr0[inp]) * 3 + i32::from(inptr1[inp]);
            inp += 1;
            let mut nextcolsum: i32 = i32::from(inptr0[inp]) * 3 + i32::from(inptr1[inp]);
            inp += 1;
            outptr[out] = ((thiscolsum * 4 + 8) >> 4) as JSAMPLE;
            outptr[out + 1] = ((thiscolsum * 3 + nextcolsum + 7) >> 4) as JSAMPLE;
            out += 2;
            let mut lastcolsum = thiscolsum;
            thiscolsum = nextcolsum;

            let mut colctr = compptr.downsampled_width - 2;
            while colctr > 0 {
                /* General case: 3/4 * nearer component + 1/4 * further component in
                 * each dimension, thus 9/16, 3/16, 3/16, 1/16 overall
                 */
                nextcolsum = i32::from(inptr0[inp]) * 3 + i32::from(inptr1[inp]);
                inp += 1;
                outptr[out] = ((thiscolsum * 3 + lastcolsum + 8) >> 4) as JSAMPLE;
                outptr[out + 1] = ((thiscolsum * 3 + nextcolsum + 7) >> 4) as JSAMPLE;
                out += 2;
                lastcolsum = thiscolsum;
                thiscolsum = nextcolsum;
                colctr -= 1;
            }

            /* Special case for last column */
            outptr[out] = ((thiscolsum * 3 + lastcolsum + 8) >> 4) as JSAMPLE;
            outptr[out + 1] = ((thiscolsum * 4 + 7) >> 4) as JSAMPLE;
        }
        inrow += 1;
    }
}

// upstream: src/jdsample.c _jinit_upsampler
/// Module initialization routine for upsampling. With
/// `jinit_upsampler_no_alloc`, it reuses the upsampler and its buffers and
/// only selects the methods again, as `jpeg_crop_scanline` needs.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_CCIR601_NOTIMPL`, `JERR_FRACT_SAMPLE_NOTIMPL`
/// and what the allocations report.
///
/// # Panics
///
/// When `cinfo.master` or `cinfo.comp_info` is absent, or the upsampler to
/// reuse is not `my_upsampler`.
pub fn jinit_upsampler(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    if !cinfo.master().pub_.jinit_upsampler_no_alloc {
        let upsample = alloc_small_box(
            cinfo,
            JPOOL_IMAGE,
            SIZEOF_my_upsampler,
            jpeg_upsampler_impl::my_upsampler(my_upsampler {
                pub_: jpeg_upsampler {
                    start_pass: start_pass_upsample,
                    upsample: sep_upsample,
                    need_context_rows: false, /* until we find out differently */
                },
                color_buf: Default::default(),
                methods: [None; MAX_COMPONENTS],
                next_row_out: 0,
                rows_to_go: 0,
                rowgroup_height: [0; MAX_COMPONENTS],
                h_expand: [0; MAX_COMPONENTS],
                v_expand: [0; MAX_COMPONENTS],
            }),
        )?;
        cinfo.upsample = Some(upsample);
    }

    let mut slot = cinfo.upsample.take().expect("cinfo->upsample is NULL");
    let result = match &mut *slot {
        jpeg_upsampler_impl::my_upsampler(upsample) => select_methods(cinfo, upsample),
        jpeg_upsampler_impl::my_merged_upsampler(_) => {
            unreachable!("cinfo->upsample is not a my_upsampler")
        }
    };
    cinfo.upsample = Some(slot);
    result
}

/// The part of `jinit_upsampler` after the module exists, with the module
/// taken out of `cinfo.upsample`. Not an upstream item.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of factors and sizes"
)]
fn select_methods(
    cinfo: j_decompress_ptr<'_, '_>,
    upsample: &mut my_upsampler,
) -> Result<(), ErrorExit> {
    if cinfo.CCIR601_sampling {
        /* this isn't supported */
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CCIR601_NOTIMPL));
    }

    /* jdmainct.c doesn't support context rows when min_DCT_scaled_size = 1,
     * so don't ask for it.
     */
    let do_fancy = cinfo.do_fancy_upsampling && cinfo.min_DCT_scaled_size > 1;

    /* Verify we can handle the sampling factors, select per-component methods,
     * and create storage as needed.
     */
    for ci in 0..cinfo.num_components as usize {
        let compptr = &cinfo.comp_info()[ci];
        /* Compute size of an "input group" after IDCT scaling.  This many samples
         * are to be converted to max_h_samp_factor * max_v_samp_factor components.
         */
        let h_in_group =
            (compptr.h_samp_factor * compptr.DCT_scaled_size) / cinfo.min_DCT_scaled_size;
        let v_in_group =
            (compptr.v_samp_factor * compptr.DCT_scaled_size) / cinfo.min_DCT_scaled_size;
        let h_out_group = cinfo.max_h_samp_factor;
        let v_out_group = cinfo.max_v_samp_factor;
        upsample.rowgroup_height[ci] = v_in_group; /* save for use later */
        let mut need_buffer = true;
        if !compptr.component_needed {
            /* Don't bother to upsample an uninteresting component. */
            upsample.methods[ci] = Some(noop_upsample);
            need_buffer = false;
        } else if h_in_group == h_out_group && v_in_group == v_out_group {
            /* Fullsize components can be processed without any work. */
            upsample.methods[ci] = Some(fullsize_upsample);
            need_buffer = false;
        } else if h_in_group * 2 == h_out_group && v_in_group == v_out_group {
            /* Special cases for 2h1v upsampling */
            if do_fancy && compptr.downsampled_width > 2 {
                upsample.methods[ci] = Some(h2v1_fancy_upsample);
            } else {
                upsample.methods[ci] = Some(h2v1_upsample);
            }
        } else if h_in_group == h_out_group && v_in_group * 2 == v_out_group && do_fancy {
            /* Non-fancy upsampling is handled by the generic method */
            upsample.methods[ci] = Some(h1v2_fancy_upsample);
            upsample.pub_.need_context_rows = true;
        } else if h_in_group * 2 == h_out_group && v_in_group * 2 == v_out_group {
            /* Special cases for 2h2v upsampling */
            if do_fancy && compptr.downsampled_width > 2 {
                upsample.methods[ci] = Some(h2v2_fancy_upsample);
                upsample.pub_.need_context_rows = true;
            } else {
                upsample.methods[ci] = Some(h2v2_upsample);
            }
        } else if (h_out_group % h_in_group) == 0 && (v_out_group % v_in_group) == 0 {
            /* Generic integral-factors upsampling method */
            upsample.methods[ci] = Some(int_upsample);
            upsample.h_expand[ci] = (h_out_group / h_in_group) as UINT8;
            upsample.v_expand[ci] = (v_out_group / v_in_group) as UINT8;
        } else {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_FRACT_SAMPLE_NOTIMPL));
        }
        if need_buffer && !cinfo.master().pub_.jinit_upsampler_no_alloc {
            let samplesperrow = jround_up(
                c_long::from(cinfo.output_width),
                c_long::from(cinfo.max_h_samp_factor),
            ) as JDIMENSION;
            let numrows = cinfo.max_v_samp_factor as JDIMENSION;
            let alloc_sarray = cinfo.mem().pub_.alloc_sarray;
            upsample.color_buf[ci] =
                ColorBuf::Own(alloc_sarray(cinfo, JPOOL_IMAGE, samplesperrow, numrows)?);
        }
    }
    Ok(())
}
