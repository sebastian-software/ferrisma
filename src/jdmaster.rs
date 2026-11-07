// libjpeg-turbo 3.2.1, src/jdmaster.h:
//
// /*
//  * jdmaster.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1995, Thomas G. Lane.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the master control structure for the JPEG decompressor.
//  */
// libjpeg-turbo 3.2.1, src/jdmaster.c:
//
// /*
//  * jdmaster.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Modified 2002-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2009-2011, 2016, 2019, 2022-2024, 2026, D. R. Commander.
//  * Copyright (C) 2013, Linaro Limited.
//  * Copyright (C) 2015, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains master control logic for the JPEG decompressor.
//  * These routines are concerned with selecting the modules to be executed
//  * and with determining the number of passes and the work to be done in each
//  * pass.
//  */
//! Translation of `src/jdmaster.h` and `src/jdmaster.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 6 (Control and API).
//!
//! Work package 2 translates the struct of `jdmaster.h`, because
//! `jpeg_CreateDecompress` creates it and `jdinput.c` and `jdmarker.c` use
//! its fields (ADR 0005, decisions of 2026-11-07).
//!
//! The port is built for 8-bit samples without lossless mode or color
//! quantization (ADR 0001). Following ADR 0005 ("Out-of-scope branches inside
//! in-scope files"), the branches that would select a 12- or 16-bit module
//! raise `JERR_BAD_PRECISION`, and with `D_LOSSLESS_SUPPORTED`,
//! `QUANT_1PASS_SUPPORTED` and `QUANT_2PASS_SUPPORTED` treated as undefined,
//! the lossless and quantizer branches raise `JERR_NOT_COMPILED` where
//! upstream's `#else` does. Without the quantizers, the slot
//! `cinfo->cquantize`, `cinfo->colormap` and the master's `quantizer_1pass`
//! and `quantizer_2pass` are absent: `colormap` is always C's null pointer,
//! and a branch that would call the quantizer raises `JERR_NOT_COMPILED`.
//! `JPEG_LIB_VERSION` is 62, so `jpeg_core_output_dimensions` is local and
//! the scaled sizes are `DCT_scaled_size` and `min_DCT_scaled_size`.
//! `WITH_PROFILE` is off.

use core::ffi::c_long;

use crate::jconfig::D_ARITH_CODING_SUPPORTED;
use crate::jdarith::jinit_arith_decoder;
use crate::jdcoefct::jinit_d_coef_controller;
use crate::jdcolor::jinit_color_deconverter;
use crate::jddctmgr::jinit_inverse_dct;
use crate::jdhuff::jinit_huff_decoder;
use crate::jdmainct::jinit_d_main_controller;
use crate::jdmerge::jinit_merged_upsampler;
use crate::jdphuff::jinit_phuff_decoder;
use crate::jdpostct::jinit_d_post_controller;
use crate::jdsample::jinit_upsampler;
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemmgr::alloc_small_vec;
use crate::jmorecfg::{
    C_BOOLEAN, C_INT, C_JDIMENSION, C_POINTER, CENTERJSAMPLE, CLayout, D_LOSSLESS_SUPPORTED,
    D_MULTISCAN_FILES_SUPPORTED, D_PROGRESSIVE_SUPPORTED, IDCT_SCALING_SUPPORTED, JDIMENSION,
    JSAMPLE, MAX_COMPONENTS, MAXJSAMPLE, QUANT_1PASS_SUPPORTED, QUANT_2PASS_SUPPORTED,
    UPSAMPLE_MERGING_SUPPORTED, rgb_pixelsize,
};
use crate::jpegint::{DSTATE_BUFIMAGE, DSTATE_READY, J_BUF_MODE, jpeg_decomp_master};
use crate::jpeglib::{
    DCTSIZE, J_COLOR_SPACE, JPOOL_IMAGE, j_decompress_ptr, jpeg_decompress_struct,
};
use crate::jutils::jdiv_round_up;

// upstream: src/jdmaster.h my_decomp_master
/// The private state of the master control module. `quantizer_1pass` and
/// `quantizer_2pass` belong to color quantization, which is out of scope
/// (ADR 0001).
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug, Default)]
pub struct my_decomp_master {
    /// Public fields.
    pub pub_: jpeg_decomp_master,
    /// Number of passes completed.
    pub pass_number: i32,
    /// True if using merged upsample/cconvert.
    pub using_merged_upsample: bool,
}

/// `sizeof(my_decomp_master)` on the target, with the method pointers of
/// `struct jpeg_decomp_master` and the quantizer pointers.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_decomp_master: usize = CLayout::structure(&[
    // struct jpeg_decomp_master
    CLayout::structure(&[
        C_POINTER.array(2),
        C_BOOLEAN,
        C_BOOLEAN,
        C_JDIMENSION,
        C_JDIMENSION,
        C_JDIMENSION.array(MAX_COMPONENTS),
        C_JDIMENSION.array(MAX_COMPONENTS),
        C_BOOLEAN,
        C_JDIMENSION,
        C_POINTER,
        C_INT,
        C_INT,
        C_INT,
    ]),
    C_INT,
    C_BOOLEAN,
    C_POINTER,
    C_POINTER,
])
.size;

// upstream: src/jdmaster.c use_merged_upsample
/// Determines whether merged upsample/color conversion should be used.
/// CRUCIAL: this must match the actual capabilities of `jdmerge.c`!
///
/// # Panics
///
/// When `cinfo.master` or `cinfo.comp_info` is absent.
fn use_merged_upsample(cinfo: &jpeg_decompress_struct<'_>) -> bool {
    use J_COLOR_SPACE as CS;

    if !UPSAMPLE_MERGING_SUPPORTED {
        return false;
    }
    /* Colorspace conversion is not supported with lossless JPEG images */
    if cinfo.master().pub_.lossless {
        return false;
    }
    /* Merging is the equivalent of plain box-filter upsampling */
    if cinfo.do_fancy_upsampling || cinfo.CCIR601_sampling {
        return false;
    }
    /* jdmerge.c only supports YCC=>RGB and YCC=>RGB565 color conversion */
    let out = cinfo.out_color_space;
    if cinfo.jpeg_color_space != CS::JCS_YCbCr
        || cinfo.num_components != 3
        || (out != CS::JCS_RGB
            && out != CS::JCS_RGB565
            && out != CS::JCS_EXT_RGB
            && out != CS::JCS_EXT_RGBX
            && out != CS::JCS_EXT_BGR
            && out != CS::JCS_EXT_BGRX
            && out != CS::JCS_EXT_XBGR
            && out != CS::JCS_EXT_XRGB
            && out != CS::JCS_EXT_RGBA
            && out != CS::JCS_EXT_BGRA
            && out != CS::JCS_EXT_ABGR
            && out != CS::JCS_EXT_ARGB)
    {
        return false;
    }
    if (out == CS::JCS_RGB565 && cinfo.out_color_components != 3)
        || (out != CS::JCS_RGB565 && cinfo.out_color_components != rgb_pixelsize[out.0 as usize])
    {
        return false;
    }
    /* and it only handles 2h1v or 2h2v sampling ratios */
    let comp_info = cinfo.comp_info();
    if comp_info[0].h_samp_factor != 2
        || comp_info[1].h_samp_factor != 1
        || comp_info[2].h_samp_factor != 1
        || comp_info[0].v_samp_factor > 2
        || comp_info[1].v_samp_factor != 1
        || comp_info[2].v_samp_factor != 1
    {
        return false;
    }
    /* furthermore, it doesn't work if we've scaled the IDCTs differently */
    if comp_info[0].DCT_scaled_size != cinfo.min_DCT_scaled_size
        || comp_info[1].DCT_scaled_size != cinfo.min_DCT_scaled_size
        || comp_info[2].DCT_scaled_size != cinfo.min_DCT_scaled_size
    {
        return false;
    }
    /* ??? also need to test for upsample-time rescaling, when & if supported */
    true /* by golly, it'll work... */
}

// upstream: src/jdmaster.c jpeg_core_output_dimensions
/// Computes output image dimensions and related values: the computations
/// that are needed before the master selection phase, for transcoding and
/// full decompression. It mustn't do anything that can't be done twice.
/// With `JPEG_LIB_VERSION` 62 it is local, as in C.
///
/// # Panics
///
/// When `cinfo.master` is absent.
#[expect(non_snake_case, reason = "C names")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "C's conversions from long to JDIMENSION and of DCTSIZE"
)]
fn jpeg_core_output_dimensions(cinfo: j_decompress_ptr<'_, '_>) {
    if IDCT_SCALING_SUPPORTED && !cinfo.master().pub_.lossless {
        /* Compute actual output image dimensions and DCT scaling choices.
         * C spells out one case per block size; each case provides
         * k/block_size scaling. C's unsigned arithmetic wraps.
         */
        let num = cinfo.scale_num.wrapping_mul(DCTSIZE as u32);
        let denom = cinfo.scale_denom;
        let k: c_long = if num <= denom {
            1
        } else if num <= denom.wrapping_mul(2) {
            2
        } else if num <= denom.wrapping_mul(3) {
            3
        } else if num <= denom.wrapping_mul(4) {
            4
        } else if num <= denom.wrapping_mul(5) {
            5
        } else if num <= denom.wrapping_mul(6) {
            6
        } else if num <= denom.wrapping_mul(7) {
            7
        } else if num <= denom.wrapping_mul(8) {
            8
        } else if num <= denom.wrapping_mul(9) {
            9
        } else if num <= denom.wrapping_mul(10) {
            10
        } else if num <= denom.wrapping_mul(11) {
            11
        } else if num <= denom.wrapping_mul(12) {
            12
        } else if num <= denom.wrapping_mul(13) {
            13
        } else if num <= denom.wrapping_mul(14) {
            14
        } else if num <= denom.wrapping_mul(15) {
            15
        } else {
            16
        };
        cinfo.output_width =
            jdiv_round_up(c_long::from(cinfo.image_width) * k, DCTSIZE as c_long) as JDIMENSION;
        cinfo.output_height =
            jdiv_round_up(c_long::from(cinfo.image_height) * k, DCTSIZE as c_long) as JDIMENSION;
        cinfo.min_DCT_scaled_size = k as i32;

        /* Recompute dimensions of components */
        let min_DCT_scaled_size = cinfo.min_DCT_scaled_size;
        for compptr in cinfo.comp_info_mut() {
            compptr.DCT_scaled_size = min_DCT_scaled_size;
        }
    } else {
        /* Hardwire it to "no scaling" */
        cinfo.output_width = cinfo.image_width;
        cinfo.output_height = cinfo.image_height;
        /* jdinput.c has already initialized DCT_scaled_size,
         * and has computed unscaled downsampled_width and downsampled_height.
         */
    }
}

// upstream: src/jdmaster.c jpeg_calc_output_dimensions
/// Computes output image dimensions and related values: the computations
/// that are needed before the master selection phase. It is exported for
/// use by the application, so it mustn't do anything that can't be done
/// twice. It may be called before the master module is initialized.
///
/// # Errors
///
/// `JERR_BAD_STATE` unless `jpeg_read_header` has just finished.
///
/// # Panics
///
/// When `cinfo.master` or `cinfo.comp_info` is absent.
#[expect(non_snake_case, reason = "C names")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "C's conversions from long to JDIMENSION and of DCTSIZE"
)]
#[expect(clippy::similar_names, reason = "C's field names")]
pub fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Prevent application from calling me at wrong times */
    if cinfo.global_state != DSTATE_READY {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }

    /* Compute core output image dimensions and DCT scaling choices. */
    jpeg_core_output_dimensions(cinfo);

    if IDCT_SCALING_SUPPORTED && !cinfo.master().pub_.lossless {
        /* In selecting the actual DCT scaling for each component, we try to
         * scale up the chroma components via IDCT scaling rather than upsampling.
         * This saves time if the upsampler gets to use 1:1 scaling.
         * Note this code adapts subsampling ratios which are powers of 2.
         */
        let (max_h_samp_factor, max_v_samp_factor, min_DCT_scaled_size) = (
            cinfo.max_h_samp_factor,
            cinfo.max_v_samp_factor,
            cinfo.min_DCT_scaled_size,
        );
        for compptr in cinfo.comp_info_mut() {
            let mut ssize = min_DCT_scaled_size;
            while ssize < DCTSIZE as i32
                && (max_h_samp_factor * min_DCT_scaled_size) % (compptr.h_samp_factor * ssize * 2)
                    == 0
                && (max_v_samp_factor * min_DCT_scaled_size) % (compptr.v_samp_factor * ssize * 2)
                    == 0
            {
                ssize *= 2;
            }
            compptr.DCT_scaled_size = ssize;
        }

        /* Recompute downsampled dimensions of components;
         * application needs to know these if using raw downsampled data.
         */
        let (image_width, image_height) = (cinfo.image_width, cinfo.image_height);
        for compptr in cinfo.comp_info_mut() {
            /* Size in samples, after IDCT scaling */
            compptr.downsampled_width = jdiv_round_up(
                c_long::from(image_width)
                    * c_long::from(compptr.h_samp_factor * compptr.DCT_scaled_size),
                c_long::from(max_h_samp_factor * DCTSIZE as i32),
            ) as JDIMENSION;
            compptr.downsampled_height = jdiv_round_up(
                c_long::from(image_height)
                    * c_long::from(compptr.v_samp_factor * compptr.DCT_scaled_size),
                c_long::from(max_v_samp_factor * DCTSIZE as i32),
            ) as JDIMENSION;
        }
    } else {
        /* Hardwire it to "no scaling" */
        cinfo.output_width = cinfo.image_width;
        cinfo.output_height = cinfo.image_height;
        /* jdinput.c has already initialized DCT_scaled_size to DCTSIZE,
         * and has computed unscaled downsampled_width and downsampled_height.
         */
    }

    /* Report number of components in selected colorspace. */
    /* Probably this should be in the color conversion module... */
    cinfo.out_color_components = match cinfo.out_color_space {
        J_COLOR_SPACE::JCS_GRAYSCALE => 1,
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
        | J_COLOR_SPACE::JCS_EXT_ARGB => rgb_pixelsize[cinfo.out_color_space.0 as usize],
        J_COLOR_SPACE::JCS_YCbCr | J_COLOR_SPACE::JCS_RGB565 => 3,
        J_COLOR_SPACE::JCS_CMYK | J_COLOR_SPACE::JCS_YCCK => 4,
        /* else must be same colorspace as in file */
        _ => cinfo.num_components,
    };
    cinfo.output_components = if cinfo.quantize_colors {
        1
    } else {
        cinfo.out_color_components
    };

    /* See if upsampler will want to emit more than one row at a time */
    if use_merged_upsample(cinfo) {
        cinfo.rec_outbuf_height = cinfo.max_v_samp_factor;
    } else {
        cinfo.rec_outbuf_height = 1;
    }
    Ok(())
}

// upstream: src/jdmaster.c prepare_range_limit_table
/// Allocates and fills in the `sample_range_limit` table.
///
/// Several decompression processes need to range-limit values to the range
/// 0..MAXJSAMPLE; the input value may fall somewhat outside this range due
/// to noise introduced by quantization, roundoff error, etc. These processes
/// are inner loops and need to be as fast as possible, so they all use a
/// common table prepared here: `x = sample_range_limit[x]`.
///
/// For most steps we can mathematically guarantee that the initial value of
/// x is within MAXJSAMPLE+1 of the legal range, so a table running from
/// -(MAXJSAMPLE+1) to 2*MAXJSAMPLE+1 is sufficient. But for the initial
/// limiting step (just after the IDCT), a wildly out-of-range value is
/// possible if the input data is corrupt, so the post-IDCT limiting is
/// `x = range_limit[x & MASK]`, where MASK is 2 bits wider than legal sample
/// data, ie 10 bits for 8-bit samples. For the post-IDCT step, we want to
/// convert the data from signed to unsigned representation by adding
/// CENTERJSAMPLE at the same time that we limit it, so the post-IDCT
/// limiting table ends up looking like this:
///
/// ```text
///   CENTERJSAMPLE,CENTERJSAMPLE+1,...,MAXJSAMPLE,
///   MAXJSAMPLE (repeat 2*(MAXJSAMPLE+1)-CENTERJSAMPLE times),
///   0          (repeat 2*(MAXJSAMPLE+1)-CENTERJSAMPLE times),
///   0,1,...,CENTERJSAMPLE-1
/// ```
///
/// Negative inputs select values from the upper half of the table after
/// masking. We can save some space by overlapping the start of the
/// post-IDCT table with the simpler range limiting table. The post-IDCT
/// table begins at `sample_range_limit + CENTERJSAMPLE`.
///
/// C's pointer into the allocation becomes the whole table, which starts
/// `MAXJSAMPLE + 1` samples before C's `sample_range_limit` (ADR 0005).
///
/// # Errors
///
/// `JERR_BAD_PRECISION` for a precision above 8 (ADR 0005) and what the
/// allocation reports.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversions of nonnegative sample values"
)]
fn prepare_range_limit_table(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    if cinfo.data_precision <= 8 {
        let max = MAXJSAMPLE as usize;
        let center = CENTERJSAMPLE as usize;
        let size = 5 * (max + 1) + center;
        let mut table: Vec<JSAMPLE> =
            alloc_small_vec(cinfo, JPOOL_IMAGE, size * size_of::<JSAMPLE>(), size, 0)?;
        let base = max + 1; /* allow negative subscripts of simple table */
        /* First segment of "simple" table: limit[x] = 0 for x < 0 */
        table[base - (max + 1)..base].fill(0);
        /* Main part of "simple" table: limit[x] = x */
        for i in 0..=max {
            table[base + i] = i as JSAMPLE;
        }
        let post = base + center; /* Point to where post-IDCT table starts */
        /* End of simple table, rest of first half of post-IDCT table */
        for i in center..2 * (max + 1) {
            table[post + i] = MAXJSAMPLE as JSAMPLE;
        }
        /* Second half of post-IDCT table */
        table[post + 2 * (max + 1)..post + 2 * (max + 1) + (2 * (max + 1) - center)].fill(0);
        table.copy_within(base..base + center, post + 4 * (max + 1) - center);
        cinfo.sample_range_limit = Some(table);
    } else {
        /* The 12- and 16-bit tables: other precisions are out of scope */
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }
    Ok(())
}

/// The error of a `j12init_*` or `j16init_*` call: other precisions are out
/// of scope (ADR 0005). Not an upstream item.
fn bad_precision(cinfo: j_decompress_ptr<'_, '_>) -> ErrorExit {
    let data_precision = cinfo.data_precision;
    ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_BAD_PRECISION, data_precision)
}

// upstream: src/jdmaster.c master_selection
/// Master selection of decompression modules. This is done once at
/// `jpeg_start_decompress` time. We determine which modules will be used and
/// give them appropriate initialization calls. We also initialize the
/// decompressor input side to begin consuming data.
///
/// Since `jpeg_read_header` has finished, we know what is in the SOF and
/// (first) SOS markers. We also have all the application parameter
/// settings.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_WIDTH_OVERFLOW`, `JERR_NOTIMPL`,
/// `JERR_NOT_COMPILED` for color quantization and lossless images (see the
/// module documentation), and what the module initializations report.
///
/// # Panics
///
/// When `cinfo.master` or `cinfo.inputctl` is absent.
#[expect(non_snake_case, reason = "C names")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions between long and JDIMENSION"
)]
#[expect(clippy::too_many_lines, reason = "C's function")]
fn master_selection(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* When decompressing an 8-bit-per-sample lossy JPEG image, we allow the
     * caller to request 12-bit-per-sample output in order to facilitate shadow
     * recovery in underexposed images.  However, in all other cases, setting the
     * output data precision to a different value than the JPEG data precision
     * will produce unexpected results, such as a bogus output image.
     */
    let jpeg_data_precision = cinfo.master().pub_.jpeg_data_precision;
    if jpeg_data_precision != 0
        && cinfo.data_precision != jpeg_data_precision
        && (cinfo.master().pub_.lossless || cinfo.data_precision != 12)
    {
        return Err(bad_precision(cinfo));
    }

    /* Disable IDCT scaling and raw (downsampled) data output in lossless mode:
     * compiled only with D_LOSSLESS_SUPPORTED, which is treated as undefined.
     */

    /* Initialize dimensions and other stuff */
    jpeg_calc_output_dimensions(cinfo)?;
    prepare_range_limit_table(cinfo)?;

    /* Width of an output scanline must be representable as JDIMENSION. */
    let samplesperrow = c_long::from(cinfo.output_width) * c_long::from(cinfo.out_color_components);
    let jd_samplesperrow = samplesperrow as JDIMENSION;
    if c_long::from(jd_samplesperrow) != samplesperrow {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW));
    }

    /* Initialize my private state */
    let using_merged_upsample = use_merged_upsample(cinfo);
    let master = cinfo.master_mut();
    master.pass_number = 0;
    master.using_merged_upsample = using_merged_upsample;

    /* Color quantizer selection: quantizer_1pass and quantizer_2pass are
     * absent (see the module documentation).
     */
    /* No mode changes if not using buffered-image mode. */
    if !cinfo.quantize_colors || !cinfo.buffered_image {
        cinfo.enable_1pass_quant = false;
        cinfo.enable_external_quant = false;
        cinfo.enable_2pass_quant = false;
    }
    if cinfo.quantize_colors {
        if cinfo.raw_data_out {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOTIMPL));
        }
        /* 2-pass quantizer only works in 3-component color space. */
        if cinfo.out_color_components != 3 || cinfo.out_color_space == J_COLOR_SPACE::JCS_RGB565 {
            cinfo.enable_1pass_quant = true;
            cinfo.enable_external_quant = false;
            cinfo.enable_2pass_quant = false;
            /* cinfo->colormap = NULL: it is absent */
        } else if cinfo.two_pass_quantize {
            /* cinfo->colormap != NULL is never true: it is absent */
            cinfo.enable_2pass_quant = true;
        } else {
            cinfo.enable_1pass_quant = true;
        }

        if cinfo.enable_1pass_quant && !QUANT_1PASS_SUPPORTED {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
        }

        /* We use the 2-pass code to map to external colormaps. */
        if (cinfo.enable_2pass_quant || cinfo.enable_external_quant) && !QUANT_2PASS_SUPPORTED {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
        }
        /* If both quantizers are initialized, the 2-pass one is left active;
         * this is necessary for starting with quantization to an external map.
         */
    }

    /* Post-processing: in particular, color conversion first */
    if !cinfo.raw_data_out {
        if using_merged_upsample {
            if cinfo.data_precision == 8 {
                jinit_merged_upsampler(cinfo)?; /* does color conversion too */
            } else {
                /* j12init_merged_upsampler, or a bad precision */
                return Err(bad_precision(cinfo));
            }
        } else if cinfo.data_precision <= 8 {
            jinit_color_deconverter(cinfo)?;
            jinit_upsampler(cinfo)?;
        } else {
            /* j12init_* and j16init_*: other precisions are out of scope */
            return Err(bad_precision(cinfo));
        }
        if cinfo.data_precision <= 8 {
            let enable_2pass_quant = cinfo.enable_2pass_quant;
            jinit_d_post_controller(cinfo, enable_2pass_quant)?;
        } else {
            return Err(bad_precision(cinfo));
        }
    }

    if cinfo.master().pub_.lossless {
        /* Prediction, sample undifferencing, point transform, sample size
         * scaling, the lossless Huffman decoder and the difference buffer
         * controller: D_LOSSLESS_SUPPORTED is treated as undefined.
         */
        if !D_LOSSLESS_SUPPORTED {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
        }
    } else {
        /* Inverse DCT */
        if cinfo.data_precision == 8 {
            jinit_inverse_dct(cinfo)?;
        } else {
            /* j12init_inverse_dct, or a bad precision */
            return Err(bad_precision(cinfo));
        }
        /* Entropy decoding: either Huffman or arithmetic coding. */
        if cinfo.arith_code {
            if D_ARITH_CODING_SUPPORTED != 0 {
                jinit_arith_decoder(cinfo)?;
            } else {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_ARITH_NOTIMPL));
            }
        } else if cinfo.progressive_mode {
            if D_PROGRESSIVE_SUPPORTED {
                jinit_phuff_decoder(cinfo)?;
            } else {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
            }
        } else {
            jinit_huff_decoder(cinfo)?;
        }

        /* Initialize principal buffer controllers. */
        let use_c_buffer = cinfo.inputctl().pub_.has_multiple_scans || cinfo.buffered_image;
        if cinfo.data_precision == 12 {
            /* j12init_d_coef_controller */
            return Err(bad_precision(cinfo));
        }
        jinit_d_coef_controller(cinfo, use_c_buffer)?;
    }

    if !cinfo.raw_data_out {
        if cinfo.data_precision <= 8 {
            jinit_d_main_controller(cinfo, false /* never need full buffer here */)?;
        } else {
            return Err(bad_precision(cinfo));
        }
    }

    /* We can now tell the memory manager to allocate virtual arrays. */
    let realize_virt_arrays = cinfo.mem().pub_.realize_virt_arrays;
    realize_virt_arrays(cinfo)?;

    /* Initialize input side of decompressor to consume first scan. */
    let start_input_pass = cinfo.inputctl().pub_.start_input_pass;
    start_input_pass(cinfo)?;

    /* Set the first and last iMCU columns to decompress from single-scan images.
     * By default, decompress all of the iMCU columns.
     */
    let MCUs_per_row = cinfo.MCUs_per_row;
    let master = &mut cinfo.master_mut().pub_;
    master.first_iMCU_col = 0;
    master.last_iMCU_col = MCUs_per_row.wrapping_sub(1);
    master.last_good_iMCU_row = 0;

    if D_MULTISCAN_FILES_SUPPORTED {
        /* If jpeg_start_decompress will read the whole file, initialize
         * progress monitoring appropriately.  The input step is counted
         * as one pass.
         */
        if cinfo.progress.is_some()
            && !cinfo.buffered_image
            && cinfo.inputctl().pub_.has_multiple_scans
        {
            /* Estimate number of scans to set pass_limit. */
            let nscans = if cinfo.progressive_mode {
                /* Arbitrarily estimate 2 interleaved DC scans + 3 AC scans/component. */
                2 + 3 * cinfo.num_components
            } else {
                /* For a nonprogressive multiscan file, estimate 1 scan per component. */
                cinfo.num_components
            };
            let total_iMCU_rows = cinfo.total_iMCU_rows;
            let enable_2pass_quant = cinfo.enable_2pass_quant;
            let progress = cinfo.progress.as_deref_mut().expect("checked above");
            progress.pass_counter = 0;
            progress.pass_limit = c_long::from(total_iMCU_rows) * c_long::from(nscans);
            progress.completed_passes = 0;
            progress.total_passes = if enable_2pass_quant { 3 } else { 2 };
            /* Count the input pass as done */
            cinfo.master_mut().pass_number += 1;
        }
    }
    Ok(())
}

// upstream: src/jdmaster.c prepare_for_output_pass
/// Per-pass setup. This is called at the beginning of each output pass. We
/// determine which modules will be active during this pass and give them
/// appropriate `start_pass` calls. We also set `is_dummy_pass` to indicate
/// whether this is a "real" output pass or a dummy pass for color
/// quantization. (In the latter case, `jdapistd.c` will crank the pass to
/// completion.)
///
/// # Errors
///
/// `JERR_NOT_COMPILED` and `JERR_MODE_CHANGE` for color quantization (see
/// the module documentation), and what the modules' `start_pass` methods
/// report.
///
/// # Panics
///
/// When `cinfo.master` or a module it starts is absent.
pub fn prepare_for_output_pass(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    if cinfo.master().pub_.is_dummy_pass {
        /* Final pass of 2-pass quantization */
        if !QUANT_2PASS_SUPPORTED {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
        }
    } else {
        /* cinfo->colormap == NULL is always true: it is absent */
        if cinfo.quantize_colors {
            /* Select new quantization method: assigning cinfo->cquantize
             * needs a quantizer, which is not compiled.
             */
            if (cinfo.two_pass_quantize && cinfo.enable_2pass_quant) || cinfo.enable_1pass_quant {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
            }
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_MODE_CHANGE));
        }
        let start_pass = cinfo.idct().start_pass;
        start_pass(cinfo)?;
        let start_output_pass = cinfo.coef().start_output_pass;
        start_output_pass(cinfo)?;
        if !cinfo.raw_data_out {
            if !cinfo.master().using_merged_upsample {
                let start_pass = cinfo.cconvert().start_pass;
                start_pass(cinfo);
            }
            let start_pass = cinfo.upsample().start_pass;
            start_pass(cinfo);
            /* cinfo->quantize_colors is false here: the quantizer's
             * start_pass is never called.
             */
            let pass_mode = if cinfo.master().pub_.is_dummy_pass {
                J_BUF_MODE::JBUF_SAVE_AND_PASS
            } else {
                J_BUF_MODE::JBUF_PASS_THRU
            };
            let start_pass = cinfo.post().start_pass;
            start_pass(cinfo, pass_mode)?;
            let start_pass = cinfo.main().start_pass;
            start_pass(cinfo, J_BUF_MODE::JBUF_PASS_THRU)?;
        }
    }

    /* Set up progress monitor's pass info if present */
    if cinfo.progress.is_some() {
        let pass_number = cinfo.master().pass_number;
        let is_dummy_pass = cinfo.master().pub_.is_dummy_pass;
        let more_passes = cinfo.buffered_image && !cinfo.inputctl().pub_.eoi_reached;
        let enable_2pass_quant = cinfo.enable_2pass_quant;
        let progress = cinfo.progress.as_deref_mut().expect("checked above");
        progress.completed_passes = pass_number;
        progress.total_passes = pass_number + if is_dummy_pass { 2 } else { 1 };
        /* In buffered-image mode, we assume one more output pass if EOI not
         * yet reached, but no more passes if EOI has been reached.
         */
        if more_passes {
            progress.total_passes += if enable_2pass_quant { 2 } else { 1 };
        }
    }
    Ok(())
}

// upstream: src/jdmaster.c finish_output_pass
/// Finishes up at end of an output pass.
///
/// # Errors
///
/// `JERR_NOT_COMPILED` with color quantization, where C would call the
/// quantizer's `finish_pass` (see the module documentation).
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn finish_output_pass(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    if cinfo.quantize_colors {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
    }
    cinfo.master_mut().pass_number += 1;
    Ok(())
}

// upstream: src/jdmaster.c jpeg_new_colormap
/// Switches to a new external colormap between output passes. Without
/// `cinfo->colormap` (see the module documentation), there is never a
/// colormap to switch to.
///
/// # Errors
///
/// `JERR_BAD_STATE` outside buffered-image mode between passes, and
/// `JERR_MODE_CHANGE` otherwise.
pub fn jpeg_new_colormap(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Prevent application from calling me at wrong times */
    if cinfo.global_state != DSTATE_BUFIMAGE {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }

    /* cinfo->colormap != NULL is never true: the 2-pass quantizer is never
     * selected for external colormap use.
     */
    Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_MODE_CHANGE))
}

// upstream: src/jdmaster.c jinit_master_decompress
/// Initializes master decompression control and selects active modules.
/// This is performed at the start of `jpeg_start_decompress`.
///
/// # Errors
///
/// What `master_selection` reports.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jinit_master_decompress(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    let master = &mut cinfo.master_mut().pub_;
    master.prepare_for_output_pass = Some(prepare_for_output_pass);
    master.finish_output_pass = Some(finish_output_pass);

    master.is_dummy_pass = false;
    master.jinit_upsampler_no_alloc = false;

    master_selection(cinfo)
}
