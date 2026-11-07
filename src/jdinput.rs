// libjpeg-turbo 3.2.1, src/jdinput.c:
//
// /*
//  * jdinput.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010, 2016, 2018, 2022, 2024, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains input control logic for the JPEG decompressor.
//  * These routines are concerned with controlling the decompressor's input
//  * processing (marker reading and coefficient/difference decoding).
//  * The actual input reading is done in jdmarker.c, jdhuff.c, jdphuff.c,
//  * and jdlhuff.c.
//  */
//! Translation of `src/jdinput.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
//!
//! `start_input_pass` calls the entropy decoder and the coefficient
//! controller, whose slot stays empty until work package 4; it is reached
//! only after `jpeg_start_decompress`. `per_scan_setup` and
//! `latch_quant_tables` are public, so that work package 3's tests can start
//! a scan with a coefficient controller of their own. The `JPEG_LIB_VERSION >= 70`
//! and `>= 80` branches and, with `D_LOSSLESS_SUPPORTED` treated as
//! undefined, the lossless precision check are not compiled (ADR 0005).

use core::ffi::c_long;

use crate::jerror::{ERREXIT, ERREXIT1, ERREXIT2, ErrorExit, J_MESSAGE_CODE, call_reset_error_mgr};
use crate::jmemmgr::alloc_small_box;
use crate::jmorecfg::{C_INT, C_POINTER, CLayout, JDIMENSION, JPEG_MAX_DIMENSION, MAX_COMPONENTS};
use crate::jpegint::{MAX, jpeg_input_controller};
use crate::jpeglib::{
    D_MAX_BLOCKS_IN_MCU, DCTSIZE, JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPOOL_IMAGE, JPOOL_PERMANENT,
    MAX_COMPS_IN_SCAN, MAX_SAMP_FACTOR, NUM_QUANT_TBLS, SIZEOF_JQUANT_TBL, j_decompress_ptr,
};
use crate::jutils::jdiv_round_up;

// upstream: src/jdinput.c my_input_controller
/// The private state of the input controller.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug)]
pub struct my_input_controller {
    /// Public fields.
    pub pub_: jpeg_input_controller,
    /// True until first SOS is reached.
    pub inheaders: bool,
}

/// `sizeof(my_input_controller)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_input_controller: usize = CLayout::structure(&[
    // struct jpeg_input_controller
    CLayout::structure(&[C_POINTER.array(4), C_INT, C_INT]),
    C_INT,
])
.size;

// upstream: src/jdinput.c initial_setup
/// Computes the quantities that depend on the image size; called once, when
/// the first SOS marker is reached.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between long, int and JDIMENSION"
)]
fn initial_setup(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    let data_unit: i32 = if cinfo.master().pub_.lossless {
        1
    } else {
        DCTSIZE as i32
    };

    /* Make sure image isn't bigger than I can handle */
    if c_long::from(cinfo.image_height) > JPEG_MAX_DIMENSION
        || c_long::from(cinfo.image_width) > JPEG_MAX_DIMENSION
    {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_IMAGE_TOO_BIG,
            JPEG_MAX_DIMENSION as i32,
        ));
    }

    /* Lossy JPEG images must have 8 or 12 bits per sample. */
    if cinfo.data_precision != 8 && cinfo.data_precision != 12 {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            cinfo.data_precision,
        ));
    }

    /* Check that number of components won't exceed internal array sizes */
    if cinfo.num_components > MAX_COMPONENTS as i32 {
        return Err(ERREXIT2(
            cinfo,
            J_MESSAGE_CODE::JERR_COMPONENT_COUNT,
            cinfo.num_components,
            MAX_COMPONENTS as i32,
        ));
    }

    /* Compute maximum sampling factors; check factor validity */
    cinfo.max_h_samp_factor = 1;
    cinfo.max_v_samp_factor = 1;
    for ci in 0..cinfo.num_components as usize {
        let compptr = &cinfo.comp_info()[ci];
        let (h, v) = (compptr.h_samp_factor, compptr.v_samp_factor);
        if h <= 0 || h > MAX_SAMP_FACTOR || v <= 0 || v > MAX_SAMP_FACTOR {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_SAMPLING));
        }
        cinfo.max_h_samp_factor = MAX(cinfo.max_h_samp_factor, h);
        cinfo.max_v_samp_factor = MAX(cinfo.max_v_samp_factor, v);
    }

    /* We initialize DCT_scaled_size and min_DCT_scaled_size to DCTSIZE in lossy
     * mode.  In the full decompressor, this will be overridden by jdmaster.c;
     * but in the transcoder, jdmaster.c is not used, so we must do it here.
     */
    cinfo.min_DCT_scaled_size = data_unit;

    /* Compute dimensions of components */
    let image_width = c_long::from(cinfo.image_width);
    let image_height = c_long::from(cinfo.image_height);
    let max_h = cinfo.max_h_samp_factor;
    let max_v = cinfo.max_v_samp_factor;
    for ci in 0..cinfo.num_components as usize {
        let compptr = &mut cinfo.comp_info_mut()[ci];
        compptr.DCT_scaled_size = data_unit;
        /* Size in data units */
        compptr.width_in_blocks = jdiv_round_up(
            image_width * c_long::from(compptr.h_samp_factor),
            c_long::from(max_h * data_unit),
        ) as JDIMENSION;
        compptr.height_in_blocks = jdiv_round_up(
            image_height * c_long::from(compptr.v_samp_factor),
            c_long::from(max_v * data_unit),
        ) as JDIMENSION;
        let width_in_blocks = compptr.width_in_blocks;
        /* Set the first and last MCU columns to decompress from multi-scan images.
         * By default, decompress all of the MCU columns.
         */
        let master = cinfo.master_mut();
        master.pub_.first_MCU_col[ci] = 0;
        master.pub_.last_MCU_col[ci] = width_in_blocks.wrapping_sub(1);
        /* downsampled_width and downsampled_height will also be overridden by
         * jdmaster.c if we are doing full decompression.  The transcoder library
         * doesn't use these values, but the calling application might.
         */
        /* Size in samples */
        let compptr = &mut cinfo.comp_info_mut()[ci];
        compptr.downsampled_width = jdiv_round_up(
            image_width * c_long::from(compptr.h_samp_factor),
            c_long::from(max_h),
        ) as JDIMENSION;
        compptr.downsampled_height = jdiv_round_up(
            image_height * c_long::from(compptr.v_samp_factor),
            c_long::from(max_v),
        ) as JDIMENSION;
        /* Mark component needed, until color conversion says otherwise */
        compptr.component_needed = true;
        /* Mark no quantization table yet saved for component */
        compptr.quant_table = None;
    }

    /* Compute number of fully interleaved MCU rows. */
    cinfo.total_iMCU_rows =
        jdiv_round_up(image_height, c_long::from(max_v * data_unit)) as JDIMENSION;

    /* Decide whether file contains multiple scans */
    cinfo.inputctl_mut().pub_.has_multiple_scans =
        cinfo.comps_in_scan < cinfo.num_components || cinfo.progressive_mode;
    Ok(())
}

// upstream: src/jdinput.c per_scan_setup
/// Does the computations needed before processing a JPEG scan.
/// `comps_in_scan` and `cur_comp_info` were set from the SOS marker.
///
/// # Errors
///
/// `JERR_COMPONENT_COUNT` or `JERR_BAD_MCU_SIZE`.
///
/// # Panics
///
/// When `cinfo.master` or a component of the scan is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between long, int and JDIMENSION"
)]
pub fn per_scan_setup(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    let data_unit: i32 = if cinfo.master().pub_.lossless {
        1
    } else {
        DCTSIZE as i32
    };

    if cinfo.comps_in_scan == 1 {
        /* Noninterleaved (single-component) scan */
        let index = cinfo.cur_comp_info[0].expect("cur_comp_info[0] is NULL");
        let compptr = &mut cinfo.comp_info_mut()[index];

        /* Overall image size in MCUs */
        let (width, height) = (compptr.width_in_blocks, compptr.height_in_blocks);

        /* For noninterleaved scan, always one data unit per MCU */
        compptr.MCU_width = 1;
        compptr.MCU_height = 1;
        compptr.MCU_blocks = 1;
        compptr.MCU_sample_width = compptr.DCT_scaled_size;
        compptr.last_col_width = 1;
        /* For noninterleaved scans, it is convenient to define last_row_height
         * as the number of data unit rows present in the last iMCU row.
         */
        let mut tmp = (compptr.height_in_blocks % compptr.v_samp_factor as JDIMENSION) as i32;
        if tmp == 0 {
            tmp = compptr.v_samp_factor;
        }
        compptr.last_row_height = tmp;

        cinfo.MCUs_per_row = width;
        cinfo.MCU_rows_in_scan = height;

        /* Prepare array describing MCU composition */
        cinfo.blocks_in_MCU = 1;
        cinfo.MCU_membership[0] = 0;
    } else {
        /* Interleaved (multi-component) scan */
        if cinfo.comps_in_scan <= 0 || cinfo.comps_in_scan > MAX_COMPS_IN_SCAN as i32 {
            return Err(ERREXIT2(
                cinfo,
                J_MESSAGE_CODE::JERR_COMPONENT_COUNT,
                cinfo.comps_in_scan,
                MAX_COMPS_IN_SCAN as i32,
            ));
        }

        /* Overall image size in MCUs */
        cinfo.MCUs_per_row = jdiv_round_up(
            c_long::from(cinfo.image_width),
            c_long::from(cinfo.max_h_samp_factor * data_unit),
        ) as JDIMENSION;
        cinfo.MCU_rows_in_scan = jdiv_round_up(
            c_long::from(cinfo.image_height),
            c_long::from(cinfo.max_v_samp_factor * data_unit),
        ) as JDIMENSION;

        cinfo.blocks_in_MCU = 0;

        for ci in 0..cinfo.comps_in_scan {
            let index = cinfo.cur_comp_info[ci as usize].expect("cur_comp_info[ci] is NULL");
            let compptr = &mut cinfo.comp_info_mut()[index];
            /* Sampling factors give # of data units of component in each MCU */
            compptr.MCU_width = compptr.h_samp_factor;
            compptr.MCU_height = compptr.v_samp_factor;
            compptr.MCU_blocks = compptr.MCU_width * compptr.MCU_height;
            compptr.MCU_sample_width = compptr.MCU_width * compptr.DCT_scaled_size;
            /* Figure number of non-dummy data units in last MCU column & row */
            let mut tmp = (compptr.width_in_blocks % compptr.MCU_width as JDIMENSION) as i32;
            if tmp == 0 {
                tmp = compptr.MCU_width;
            }
            compptr.last_col_width = tmp;
            let mut tmp = (compptr.height_in_blocks % compptr.MCU_height as JDIMENSION) as i32;
            if tmp == 0 {
                tmp = compptr.MCU_height;
            }
            compptr.last_row_height = tmp;
            /* Prepare array describing MCU composition */
            let mut mcublks = compptr.MCU_blocks;
            if cinfo.blocks_in_MCU + mcublks > D_MAX_BLOCKS_IN_MCU as i32 {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_MCU_SIZE));
            }
            while mcublks > 0 {
                mcublks -= 1;
                cinfo.MCU_membership[cinfo.blocks_in_MCU as usize] = ci;
                cinfo.blocks_in_MCU += 1;
            }
        }
    }
    Ok(())
}

// upstream: src/jdinput.c latch_quant_tables
/// Saves away a copy of the Q-table referenced by each component present in
/// the current scan, unless already saved during a prior scan, so that each
/// component can be dequantized with the table current at the start of its
/// first scan.
///
/// # Errors
///
/// `JERR_NO_QUANT_TABLE`, or what the memory manager reports.
///
/// # Panics
///
/// When a component of the scan is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the table index is checked to be nonnegative"
)]
pub fn latch_quant_tables(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    for ci in 0..cinfo.comps_in_scan as usize {
        let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
        let compptr = &cinfo.comp_info()[index];
        /* No work if we already saved Q-table for this component */
        if compptr.quant_table.is_some() {
            continue;
        }
        /* Make sure specified quantization table is present */
        let qtblno = compptr.quant_tbl_no;
        if qtblno < 0
            || qtblno >= NUM_QUANT_TBLS as i32
            || cinfo.quant_tbl_ptrs[qtblno as usize].is_none()
        {
            return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_NO_QUANT_TABLE, qtblno));
        }
        /* OK, save away the quantization table */
        let table = cinfo.quant_tbl_ptrs[qtblno as usize]
            .as_deref()
            .expect("checked above")
            .clone();
        let qtbl = alloc_small_box(cinfo, JPOOL_IMAGE, SIZEOF_JQUANT_TBL, table)?;
        cinfo.comp_info_mut()[index].quant_table = Some(qtbl);
    }
    Ok(())
}

// upstream: src/jdinput.c start_input_pass
/// Initializes the input modules to read a scan of compressed data: called
/// first by `jdmaster.c` during `jpeg_start_decompress`, later by
/// `consume_markers`.
///
/// # Errors
///
/// A libjpeg error, recorded in `cinfo.err`.
pub fn start_input_pass(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    per_scan_setup(cinfo)?;
    if !cinfo.master().pub_.lossless {
        latch_quant_tables(cinfo)?;
    }
    let start_pass = cinfo.entropy().start_pass;
    start_pass(cinfo)?;
    let start_input_pass = cinfo.coef().start_input_pass;
    start_input_pass(cinfo);
    let consume_data = cinfo.coef().consume_data;
    cinfo.inputctl_mut().pub_.consume_input = consume_data;
    Ok(())
}

// upstream: src/jdinput.c finish_input_pass
/// Finishes up after inputting a compressed-data scan; called by the
/// coefficient controller after it has read all the data of the scan.
pub fn finish_input_pass(cinfo: j_decompress_ptr<'_, '_>) {
    cinfo.inputctl_mut().pub_.consume_input = consume_markers;
}

// upstream: src/jdinput.c consume_markers
/// Reads JPEG markers before, between or after compressed-data scans and
/// changes state as a new scan is reached: `JPEG_SUSPENDED`,
/// `JPEG_REACHED_SOS` or `JPEG_REACHED_EOI`.
///
/// # Errors
///
/// A libjpeg error, recorded in `cinfo.err`.
pub fn consume_markers(cinfo: j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit> {
    if cinfo.inputctl().pub_.eoi_reached {
        /* After hitting EOI, read no further */
        return Ok(JPEG_REACHED_EOI);
    }

    let read_markers = cinfo.marker().pub_.read_markers;
    let val = read_markers(cinfo)?;

    match val {
        JPEG_REACHED_SOS => {
            /* Found SOS */
            if cinfo.inputctl().inheaders {
                /* 1st SOS */
                initial_setup(cinfo)?;
                cinfo.inputctl_mut().inheaders = false;
                /* Note: start_input_pass must be called by jdmaster.c
                 * before any more input can be consumed.  jdapimin.c is
                 * responsible for enforcing this sequencing.
                 */
            } else {
                /* 2nd or later SOS marker */
                if !cinfo.inputctl().pub_.has_multiple_scans {
                    return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_EOI_EXPECTED)); /* Oops, I wasn't expecting this! */
                }
                start_input_pass(cinfo)?;
            }
        }
        JPEG_REACHED_EOI => {
            /* Found EOI */
            cinfo.inputctl_mut().pub_.eoi_reached = true;
            if cinfo.inputctl().inheaders {
                /* Tables-only datastream, apparently */
                if cinfo.marker().pub_.saw_SOF {
                    return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_SOF_NO_SOS));
                }
            } else {
                /* Prevent infinite loop in coef ctlr's decompress_data routine
                 * if user set output_scan_number larger than number of scans.
                 */
                if cinfo.output_scan_number > cinfo.input_scan_number {
                    cinfo.output_scan_number = cinfo.input_scan_number;
                }
            }
        }
        _ => {} /* JPEG_SUSPENDED */
    }

    Ok(val)
}

// upstream: src/jdinput.c reset_input_controller
/// Resets state to begin a fresh datastream.
pub fn reset_input_controller(cinfo: j_decompress_ptr<'_, '_>) {
    let inputctl = cinfo.inputctl_mut();
    inputctl.pub_.consume_input = consume_markers;
    inputctl.pub_.has_multiple_scans = false; /* "unknown" would be better */
    inputctl.pub_.eoi_reached = false;
    inputctl.inheaders = true;
    /* Reset other modules */
    call_reset_error_mgr(cinfo);
    let reset_marker_reader = cinfo.marker().pub_.reset_marker_reader;
    reset_marker_reader(cinfo);
    /* Reset progression state -- would be cleaner if entropy decoder did this */
    cinfo.coef_bits = None;
}

// upstream: src/jdinput.c jinit_input_controller
/// Initializes the input controller module; called only once, when the
/// decompression object is created.
///
/// # Errors
///
/// What `alloc_small` reports.
pub fn jinit_input_controller(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Create subobject in permanent pool */
    let inputctl = my_input_controller {
        /* Initialize method pointers */
        pub_: jpeg_input_controller {
            consume_input: consume_markers,
            reset_input_controller,
            start_input_pass,
            finish_input_pass,
            /* Initialize state: can't use reset_input_controller since we don't
             * want to try to reset other modules yet.
             */
            has_multiple_scans: false, /* "unknown" would be better */
            eoi_reached: false,
        },
        inheaders: true,
    };
    cinfo.inputctl = Some(alloc_small_box(
        cinfo,
        JPOOL_PERMANENT,
        SIZEOF_my_input_controller,
        inputctl,
    )?);
    Ok(())
}
