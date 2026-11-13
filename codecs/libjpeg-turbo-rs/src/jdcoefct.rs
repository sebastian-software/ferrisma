// libjpeg-turbo 3.2.1, src/jdcoefct.h:
//
// /*
//  * jdcoefct.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2020, Google, Inc.
//  * Copyright (C) 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdcoefct.c:
//
// /*
//  * jdcoefct.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2010, 2015-2016, 2019-2020, 2022-2026, D. R. Commander.
//  * Copyright (C) 2015, 2020, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the coefficient buffer controller for decompression.
//  * This controller is the top level of the lossy JPEG decompressor proper.
//  * The coefficient buffer lies between entropy decoding and inverse-DCT steps.
//  *
//  * In buffered-image mode, this controller is the interface between
//  * input-oriented processing and output-oriented processing.
//  * Also, the input side (only) is used when reading a file for transcoding.
//  */
//! Translation of `src/jdcoefct.h` and `src/jdcoefct.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
//!
//! `D_MULTISCAN_FILES_SUPPORTED` and `BLOCK_SMOOTHING_SUPPORTED` are
//! defined, `WITH_PROFILE` is not. The method functions take the module out
//! of `cinfo.coef` for their duration (ADR 0005, "A method's own state");
//! `decompress_data` and `decompress_smooth_data` first run their input loop,
//! which reaches `consume_data` through `cinfo`, and take the module only
//! afterwards. The entropy decoder, the IDCT and the input controller's
//! `finish_input_pass`, which the functions call while they hold the module,
//! never reach it in upstream.
//!
//! In single-pass mode `MCU_buffer` owns the one-MCU workspace whose blocks
//! C's `MCU_buffer[]` points to. In multi-pass mode C points `MCU_buffer[]`
//! into the virtual arrays for each MCU; `consume_data` instead lends the
//! MCU's blocks to `decode_mcu` straight from the arrays, which it takes out
//! of the memory manager while it decodes the iMCU row and puts back before
//! it returns, errors included. `decode_mcu` reaches only the entropy
//! decoder, the data source, the error manager and the marker reader.
//! `whole_image` holds `None` where C's `memset` leaves a null pointer, and
//! `coef_arrays` is a copy of it. An IDCT routine receives `cinfo` shared
//! (see `inverse_DCT_method_ptr`).

use core::ffi::c_long;

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jerror::{ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemmgr::{alloc_large_vec, alloc_small_box, alloc_small_vec, jvirt_barray_control};
use crate::jmorecfg::{C_INT, C_JDIMENSION, C_POINTER, CLayout, JCOEF, JDIMENSION, MAX_COMPONENTS};
use crate::jpegint::{JLONG, jpeg_d_coef_controller};
use crate::jpeglib::{
    D_MAX_BLOCKS_IN_MCU, DCTSIZE2, JBLOCK, JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, JPEG_SUSPENDED,
    JPOOL_IMAGE, JSAMPROWS_MUT, MAX_COMPS_IN_SCAN, SIZEOF_JBLOCK, j_decompress_ptr,
    jpeg_decompress_struct, jvirt_barray_ptr,
};
use crate::jutils::{jcopy_block_row, jround_up, jzero_far};

// upstream: src/jdcoefct.h my_coef_controller
/// Private buffer controller object.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Debug)]
pub struct my_coef_controller {
    /// Public fields.
    pub pub_: jpeg_d_coef_controller,

    /* These variables keep track of the current location of the input side. */
    /* cinfo->input_iMCU_row is also used for this. */
    /// Counts MCUs processed in the current row.
    pub MCU_ctr: JDIMENSION,
    /// Counts MCU rows within the iMCU row.
    pub MCU_vert_offset: i32,
    /// Number of such rows needed.
    pub MCU_rows_per_iMCU_row: i32,

    /* The output side's location is represented by cinfo->output_iMCU_row. */
    /// In single-pass modes, it's sufficient to buffer just one MCU: a
    /// workspace of `D_MAX_BLOCKS_IN_MCU` coefficient blocks that the entropy
    /// decoder writes into each time. `None` in multi-pass modes; see the
    /// module documentation.
    pub MCU_buffer: Option<Box<[JBLOCK; D_MAX_BLOCKS_IN_MCU]>>,

    /// Temporary workspace for one MCU; `None` until `jinit_d_coef_controller`
    /// allocates it.
    pub workspace: Option<Box<JBLOCK>>,

    /// In multi-pass modes, we need a virtual block array for each component.
    pub whole_image: [Option<jvirt_barray_ptr>; MAX_COMPONENTS],

    /// When doing block smoothing, we latch coefficient Al values here.
    pub coef_bits_latch: Option<Vec<i32>>,
}

// upstream: src/jdcoefct.h SAVED_COEFS
/// We save `coef_bits[0..9]`.
const SAVED_COEFS: usize = 10;

/// `sizeof(my_coef_controller)` on the target, with the
/// `decompress_data_12` and `decompress_data_16` pointers of
/// `struct jpeg_d_coef_controller`.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_coef_controller: usize = CLayout::structure(&[
    // struct jpeg_d_coef_controller
    CLayout::structure(&[C_POINTER.array(6), C_JDIMENSION, C_INT, C_INT, C_POINTER]),
    C_JDIMENSION,
    C_INT,
    C_INT,
    C_POINTER.array(D_MAX_BLOCKS_IN_MCU),
    C_POINTER,
    C_POINTER.array(MAX_COMPONENTS),
    C_POINTER,
])
.size;

/// Runs `body` with the coefficient controller taken out of `cinfo.coef`
/// and puts it back on every path (ADR 0005, "A method's own state"). Not an
/// upstream item.
fn with_coef<R>(
    cinfo: j_decompress_ptr<'_, '_>,
    body: impl FnOnce(j_decompress_ptr<'_, '_>, &mut my_coef_controller) -> Result<R, ErrorExit>,
) -> Result<R, ErrorExit> {
    let mut coef = cinfo.coef.take().expect("cinfo->coef is NULL");
    let result = body(cinfo, &mut coef);
    cinfo.coef = Some(coef);
    result
}

// upstream: src/jdcoefct.h start_iMCU_row
/// Resets within-iMCU-row counters for a new row (input side).
///
/// # Panics
///
/// When the first component of the scan is absent.
#[expect(non_snake_case, reason = "C names")]
pub fn start_iMCU_row(cinfo: &jpeg_decompress_struct<'_>, coef: &mut my_coef_controller) {
    /* In an interleaved scan, an MCU row is the same as an iMCU row.
     * In a noninterleaved scan, an iMCU row has v_samp_factor MCU rows.
     * But at the bottom of the image, process only what's left.
     */
    if cinfo.comps_in_scan > 1 {
        coef.MCU_rows_per_iMCU_row = 1;
    } else {
        let compptr = &cinfo.comp_info()[cinfo.cur_comp_info[0].expect("cur_comp_info[0] is NULL")];
        if cinfo.input_iMCU_row < cinfo.total_iMCU_rows.wrapping_sub(1) {
            coef.MCU_rows_per_iMCU_row = compptr.v_samp_factor;
        } else {
            coef.MCU_rows_per_iMCU_row = compptr.last_row_height;
        }
    }

    coef.MCU_ctr = 0;
    coef.MCU_vert_offset = 0;
}

// upstream: src/jdcoefct.c start_input_pass
/// Initializes for an input processing pass.
///
/// # Panics
///
/// When `cinfo.coef` is absent.
pub fn start_input_pass(cinfo: j_decompress_ptr<'_, '_>) {
    cinfo.input_iMCU_row = 0;
    let mut coef = cinfo.coef.take().expect("cinfo->coef is NULL");
    start_iMCU_row(cinfo, &mut coef);
    cinfo.coef = Some(coef);
}

// upstream: src/jdcoefct.c start_output_pass
/// Initializes for an output processing pass.
///
/// # Errors
///
/// What allocating the block-smoothing latch reports.
///
/// # Panics
///
/// When `cinfo.coef` is absent.
pub fn start_output_pass(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* If multipass, check to see whether to use block smoothing on this pass */
    if cinfo.coef().coef_arrays.is_some() {
        if cinfo.do_block_smoothing && smoothing_ok(cinfo)? {
            cinfo.coef_mut().decompress_data = decompress_smooth_data;
        } else {
            cinfo.coef_mut().decompress_data = decompress_data;
        }
    }
    cinfo.output_iMCU_row = 0;
    Ok(())
}

// upstream: src/jdcoefct.c decompress_onepass
/// Decompresses and returns some data in the single-pass case. Always
/// attempts to emit one fully interleaved MCU row ("iMCU" row). Input and
/// output must run in lockstep since we have only a one-MCU buffer.
/// Returns `JPEG_ROW_COMPLETED`, `JPEG_SCAN_COMPLETED` or `JPEG_SUSPENDED`.
///
/// NB: `output_buf` contains a plane for each component in image, which we
/// index according to the component's SOF position.
///
/// # Errors
///
/// What the entropy decoder reports.
///
/// # Panics
///
/// When a module, the MCU buffer, a component of the scan or an IDCT
/// routine is absent.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
#[expect(non_snake_case, reason = "C names")]
pub fn decompress_onepass(
    cinfo: j_decompress_ptr<'_, '_>,
    output_buf: &mut [JSAMPROWS_MUT<'_, '_>],
) -> Result<i32, ErrorExit> {
    with_coef(cinfo, |cinfo, coef| {
        let last_MCU_col: JDIMENSION = cinfo.MCUs_per_row.wrapping_sub(1);
        let last_iMCU_row: JDIMENSION = cinfo.total_iMCU_rows.wrapping_sub(1);
        let MCU_buffer = coef
            .MCU_buffer
            .as_deref_mut()
            .expect("coef->MCU_buffer is NULL");

        /* Loop to process as much as one whole iMCU row */
        let mut yoffset = coef.MCU_vert_offset;
        while yoffset < coef.MCU_rows_per_iMCU_row {
            let mut MCU_col_num = coef.MCU_ctr; /* index of current MCU within row */
            while MCU_col_num <= last_MCU_col {
                /* Try to fetch an MCU.  Entropy decoder expects buffer to be zeroed. */
                jzero_far(
                    &mut MCU_buffer[..],
                    cinfo.blocks_in_MCU as usize * SIZEOF_JBLOCK,
                );
                if !cinfo.entropy().insufficient_data {
                    cinfo.master_mut().pub_.last_good_iMCU_row = cinfo.input_iMCU_row;
                }
                let decode_mcu = cinfo.entropy().decode_mcu.expect("decode_mcu is NULL");
                let mut MCU_data = MCU_buffer.each_mut();
                if !decode_mcu(cinfo, Some(&mut MCU_data[..]))? {
                    /* Suspension forced; update state counters and exit */
                    coef.MCU_vert_offset = yoffset;
                    coef.MCU_ctr = MCU_col_num;
                    return Ok(JPEG_SUSPENDED);
                }

                /* Only perform the IDCT on blocks that are contained within the desired
                 * cropping region.
                 */
                let cinfo: &jpeg_decompress_struct<'_> = cinfo;
                let master = &cinfo.master().pub_;
                if MCU_col_num >= master.first_iMCU_col && MCU_col_num <= master.last_iMCU_col {
                    /* Determine where data should go in output_buf and do the IDCT thing.
                     * We skip dummy blocks at the right and bottom edges (but blkn gets
                     * incremented past them!).  Note the inner loop relies on having
                     * allocated the MCU_buffer[] blocks sequentially.
                     */
                    let mut blkn: usize = 0; /* index of current DCT block within MCU */
                    for ci in 0..cinfo.comps_in_scan as usize {
                        let compptr = &cinfo.comp_info()
                            [cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL")];
                        /* Don't bother to IDCT an uninteresting component. */
                        if !compptr.component_needed {
                            blkn += compptr.MCU_blocks as usize;
                            continue;
                        }
                        let inverse_DCT = cinfo.idct().inverse_DCT
                            [compptr.component_index as usize]
                            .expect("inverse_DCT is NULL");
                        let useful_width = if MCU_col_num < last_MCU_col {
                            compptr.MCU_width
                        } else {
                            compptr.last_col_width
                        };
                        let output_ptr = &mut output_buf[compptr.component_index as usize];
                        let mut output_row = (yoffset * compptr.DCT_scaled_size) as usize;
                        let start_col: JDIMENSION = (MCU_col_num - master.first_iMCU_col)
                            * compptr.MCU_sample_width as JDIMENSION;
                        for yindex in 0..compptr.MCU_height {
                            if cinfo.input_iMCU_row < last_iMCU_row
                                || yoffset + yindex < compptr.last_row_height
                            {
                                let mut output_col = start_col;
                                for xindex in 0..useful_width as usize {
                                    inverse_DCT(
                                        cinfo,
                                        compptr,
                                        &MCU_buffer[blkn + xindex],
                                        output_ptr.offset(output_row),
                                        output_col,
                                    );
                                    output_col += compptr.DCT_scaled_size as JDIMENSION;
                                }
                            }
                            blkn += compptr.MCU_width as usize;
                            output_row += compptr.DCT_scaled_size as usize;
                        }
                    }
                }
                MCU_col_num += 1;
            }
            /* Completed an MCU row, but perhaps not an iMCU row */
            coef.MCU_ctr = 0;
            yoffset += 1;
        }
        /* Completed the iMCU row, advance counters for next one */
        cinfo.output_iMCU_row += 1;
        cinfo.input_iMCU_row += 1;
        if cinfo.input_iMCU_row < cinfo.total_iMCU_rows {
            start_iMCU_row(cinfo, coef);
            return Ok(JPEG_ROW_COMPLETED);
        }
        /* Completed the scan */
        let finish_input_pass = cinfo.inputctl().pub_.finish_input_pass;
        finish_input_pass(cinfo);
        Ok(JPEG_SCAN_COMPLETED)
    })
}

// upstream: src/jdcoefct.c dummy_consume_data
/// Dummy consume-input routine for single-pass operation.
///
/// # Errors
///
/// None; the `Result` is the method's type.
pub fn dummy_consume_data(_cinfo: j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit> {
    Ok(JPEG_SUSPENDED) /* Always indicate nothing was done */
}

// upstream: src/jdcoefct.c consume_data
/// Consumes input data and stores it in the full-image coefficient buffer.
/// We read as much as one fully interleaved MCU row ("iMCU" row) per call,
/// ie, `v_samp_factor` block rows for each component in the scan. Returns
/// `JPEG_ROW_COMPLETED`, `JPEG_SCAN_COMPLETED` or `JPEG_SUSPENDED`.
///
/// # Errors
///
/// What `access_virt_barray` and the entropy decoder report.
///
/// # Panics
///
/// When a module, a component of the scan or its virtual array is absent.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
pub fn consume_data(cinfo: j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit> {
    with_coef(cinfo, |cinfo, coef| {
        let comps_in_scan = cinfo.comps_in_scan as usize;
        /* The array and the index of its first row in the array's buffer, as
         * C's buffer[ci] points there. */
        let mut buffer: [(jvirt_barray_ptr, usize); MAX_COMPS_IN_SCAN] =
            [(jvirt_barray_ptr(0), 0); MAX_COMPS_IN_SCAN];

        /* Align the virtual buffers for the components used in this scan. */
        for (ci, entry) in buffer.iter_mut().enumerate().take(comps_in_scan) {
            let compptr =
                &cinfo.comp_info()[cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL")];
            let array = coef.whole_image[compptr.component_index as usize]
                .expect("coef->whole_image is NULL");
            let start_row = cinfo.input_iMCU_row * compptr.v_samp_factor as JDIMENSION;
            let num_rows = compptr.v_samp_factor as JDIMENSION;
            let access_virt_barray = cinfo.mem().pub_.access_virt_barray;
            *entry = (
                array,
                access_virt_barray(cinfo, array, start_row, num_rows, true)?,
            );
            /* Note: entropy decoder expects buffer to be zeroed,
             * but this is handled automatically by the memory manager
             * because we requested a pre-zeroed array.
             */
        }

        /* Lend the arrays' blocks to the entropy decoder (see the module
         * documentation). */
        let mut arrays = core::mem::take(&mut cinfo.mem_mut().virt_barray_list);
        let completed = consume_iMCU_row(cinfo, coef, &mut arrays, &buffer[..comps_in_scan]);
        cinfo.mem_mut().virt_barray_list = arrays;
        if !completed? {
            return Ok(JPEG_SUSPENDED);
        }

        /* Completed the iMCU row, advance counters for next one */
        cinfo.input_iMCU_row += 1;
        if cinfo.input_iMCU_row < cinfo.total_iMCU_rows {
            start_iMCU_row(cinfo, coef);
            return Ok(JPEG_ROW_COMPLETED);
        }
        /* Completed the scan */
        let finish_input_pass = cinfo.inputctl().pub_.finish_input_pass;
        finish_input_pass(cinfo);
        Ok(JPEG_SCAN_COMPLETED)
    })
}

/// The loop of `consume_data` that processes one whole iMCU row, with the
/// virtual arrays out of the memory manager: `false` when the entropy
/// decoder suspended. Not an upstream item.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
#[expect(non_snake_case, reason = "C names")]
fn consume_iMCU_row(
    cinfo: j_decompress_ptr<'_, '_>,
    coef: &mut my_coef_controller,
    arrays: &mut [jvirt_barray_control],
    buffer: &[(jvirt_barray_ptr, usize)],
) -> Result<bool, ErrorExit> {
    /* The blocks that the unused entries of the MCU's list point to, as C's
     * fixed-size coef->MCU_buffer[] has entries past blkn. */
    let mut unused: [JBLOCK; D_MAX_BLOCKS_IN_MCU] = [[0; DCTSIZE2]; D_MAX_BLOCKS_IN_MCU];
    /* Loop to process one whole iMCU row */
    let mut yoffset = coef.MCU_vert_offset;
    while yoffset < coef.MCU_rows_per_iMCU_row {
        let mut MCU_col_num = coef.MCU_ctr; /* index of current MCU within row */
        while MCU_col_num < cinfo.MCUs_per_row {
            /* Construct list of pointers to DCT blocks belonging to this MCU */
            let (mut MCU_buffer, blkn) = mcu_blocks(
                cinfo,
                arrays,
                buffer,
                yoffset as usize,
                MCU_col_num,
                &mut unused,
            );
            if !cinfo.entropy().insufficient_data {
                cinfo.master_mut().pub_.last_good_iMCU_row = cinfo.input_iMCU_row;
            }
            /* Try to fetch the MCU. */
            let decode_mcu = cinfo.entropy().decode_mcu.expect("decode_mcu is NULL");
            if !decode_mcu(cinfo, Some(&mut MCU_buffer[..blkn]))? {
                /* Suspension forced; update state counters and exit */
                coef.MCU_vert_offset = yoffset;
                coef.MCU_ctr = MCU_col_num;
                return Ok(false);
            }
            MCU_col_num += 1;
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        coef.MCU_ctr = 0;
        yoffset += 1;
    }
    Ok(true)
}

/// The blocks of the MCU at `MCU_col_num` and MCU row `yoffset` of the
/// iMCU row, in C's `MCU_buffer[]` order: for each component of the scan,
/// `MCU_height` rows of `MCU_width` blocks. `buffer[ci]` names the array of
/// scan component `ci` and the index of its first row of the iMCU row.
/// Returns C's fixed-size `MCU_buffer[]` and `blkn`, the number of its
/// entries that the MCU fills; the others point into `unused`. Not an
/// upstream item.
///
/// # Panics
///
/// When an array is unrealized, two scan components share an array, which
/// `get_sos` rules out, or the MCU has more than `D_MAX_BLOCKS_IN_MCU`
/// blocks, which `per_scan_setup` rules out.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
#[expect(non_snake_case, reason = "C names")]
fn mcu_blocks<'a>(
    cinfo: &jpeg_decompress_struct<'_>,
    arrays: &'a mut [jvirt_barray_control],
    buffer: &[(jvirt_barray_ptr, usize)],
    yoffset: usize,
    MCU_col_num: JDIMENSION,
    unused: &'a mut [JBLOCK; D_MAX_BLOCKS_IN_MCU],
) -> ([&'a mut JBLOCK; D_MAX_BLOCKS_IN_MCU], usize) {
    let mut controls: [Option<&'a mut jvirt_barray_control>; MAX_COMPS_IN_SCAN] =
        Default::default();
    for (index, control) in arrays.iter_mut().enumerate() {
        if let Some(ci) = buffer.iter().position(|&(array, _)| array.0 == index) {
            controls[ci] = Some(control);
        }
    }

    let mut MCU_buffer = unused.each_mut();
    let mut blkn = 0; /* index of current DCT block within MCU */
    for (ci, &(_, first_row)) in buffer.iter().enumerate() {
        let compptr =
            &cinfo.comp_info()[cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL")];
        let MCU_width = compptr.MCU_width as usize;
        let start_col = MCU_col_num as usize * MCU_width;
        let mem_buffer = controls[ci]
            .take()
            .expect("two scan components share a virtual array")
            .mem_buffer
            .as_mut()
            .expect("virtual array not realized");
        let rows = &mem_buffer.rows;
        /* The rows lie in ascending order in one buffer, so the blocks are
         * split off one after another. */
        let mut rest: &'a mut [JBLOCK] = &mut mem_buffer.buffer[..];
        let mut consumed = 0;
        for yindex in 0..compptr.MCU_height as usize {
            let start = rows[first_row + yindex + yoffset] + start_col;
            let (_, tail) = core::mem::take(&mut rest).split_at_mut(start - consumed);
            let (blocks, tail) = tail.split_at_mut(MCU_width);
            for buffer_ptr in blocks {
                MCU_buffer[blkn] = buffer_ptr;
                blkn += 1;
            }
            rest = tail;
            consumed = start + MCU_width;
        }
    }
    (MCU_buffer, blkn)
}

// upstream: src/jdcoefct.c decompress_data
/// Decompresses and returns some data in the multi-pass case. Always
/// attempts to emit one fully interleaved MCU row ("iMCU" row). Returns
/// `JPEG_ROW_COMPLETED`, `JPEG_SCAN_COMPLETED` or `JPEG_SUSPENDED`.
///
/// NB: `output_buf` contains a plane for each component in image.
///
/// # Errors
///
/// What `consume_input` and `access_virt_barray` report.
///
/// # Panics
///
/// When a module, a virtual array or an IDCT routine is absent.
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "C's conversions between int and JDIMENSION"
)]
#[expect(non_snake_case, reason = "C names")]
#[allow(clippy::needless_range_loop, reason = "C's component loop")]
pub fn decompress_data(
    cinfo: j_decompress_ptr<'_, '_>,
    output_buf: &mut [JSAMPROWS_MUT<'_, '_>],
) -> Result<i32, ErrorExit> {
    let last_iMCU_row: JDIMENSION = cinfo.total_iMCU_rows.wrapping_sub(1);

    /* Force some input to be done if we are getting ahead of the input. */
    while cinfo.input_scan_number < cinfo.output_scan_number
        || (cinfo.input_scan_number == cinfo.output_scan_number
            && cinfo.input_iMCU_row <= cinfo.output_iMCU_row)
    {
        let consume_input = cinfo.inputctl().pub_.consume_input;
        if consume_input(cinfo)? == JPEG_SUSPENDED {
            return Ok(JPEG_SUSPENDED);
        }
    }

    /* OK, output from the virtual arrays. */
    for ci in 0..cinfo.num_components as usize {
        let compptr = &cinfo.comp_info()[ci];
        /* Don't bother to IDCT an uninteresting component. */
        if !compptr.component_needed {
            continue;
        }
        let v_samp_factor = compptr.v_samp_factor;
        let height_in_blocks = compptr.height_in_blocks;
        let whole_image = cinfo
            .coef
            .as_deref()
            .expect("cinfo->coef is NULL")
            .whole_image[ci]
            .expect("coef->whole_image is NULL");
        /* Align the virtual buffer for this component. */
        let access_virt_barray = cinfo.mem().pub_.access_virt_barray;
        let buffer = access_virt_barray(
            cinfo,
            whole_image,
            cinfo.output_iMCU_row * v_samp_factor as JDIMENSION,
            v_samp_factor as JDIMENSION,
            false,
        )?;
        /* Count non-dummy DCT block rows in this iMCU row. */
        let block_rows = if cinfo.output_iMCU_row < last_iMCU_row {
            v_samp_factor
        } else {
            /* NB: can't use last_row_height here; it is input-side-dependent! */
            let block_rows = (height_in_blocks % v_samp_factor as JDIMENSION) as i32;
            if block_rows == 0 {
                v_samp_factor
            } else {
                block_rows
            }
        };
        let cinfo: &jpeg_decompress_struct<'_> = cinfo;
        let compptr = &cinfo.comp_info()[ci];
        let master = &cinfo.master().pub_;
        let inverse_DCT = cinfo.idct().inverse_DCT[ci].expect("inverse_DCT is NULL");
        let mem_buffer = cinfo.mem().virt_barray_list[whole_image.0]
            .mem_buffer
            .as_ref()
            .expect("virtual array not realized");
        let output_ptr = &mut output_buf[ci];
        /* Loop over all DCT blocks to be processed. */
        for block_row in 0..block_rows as usize {
            let buffer_ptr = mem_buffer.row(buffer + block_row);
            let mut output_col: JDIMENSION = 0;
            for block_num in master.first_MCU_col[ci]..=master.last_MCU_col[ci] {
                inverse_DCT(
                    cinfo,
                    compptr,
                    &buffer_ptr[block_num as usize],
                    output_ptr.offset(block_row * compptr.DCT_scaled_size as usize),
                    output_col,
                );
                output_col += compptr.DCT_scaled_size as JDIMENSION;
            }
        }
    }

    cinfo.output_iMCU_row += 1;
    if cinfo.output_iMCU_row < cinfo.total_iMCU_rows {
        return Ok(JPEG_ROW_COMPLETED);
    }
    Ok(JPEG_SCAN_COMPLETED)
}

/* Natural-order array positions of the first 9 zigzag-order coefficients */
// upstream: src/jdcoefct.c Q01_POS
const Q01_POS: usize = 1;
// upstream: src/jdcoefct.c Q10_POS
const Q10_POS: usize = 8;
// upstream: src/jdcoefct.c Q20_POS
const Q20_POS: usize = 16;
// upstream: src/jdcoefct.c Q11_POS
const Q11_POS: usize = 9;
// upstream: src/jdcoefct.c Q02_POS
const Q02_POS: usize = 2;
// upstream: src/jdcoefct.c Q03_POS
const Q03_POS: usize = 3;
// upstream: src/jdcoefct.c Q12_POS
const Q12_POS: usize = 10;
// upstream: src/jdcoefct.c Q21_POS
const Q21_POS: usize = 17;
// upstream: src/jdcoefct.c Q30_POS
const Q30_POS: usize = 24;

// upstream: src/jdcoefct.c smoothing_ok
/// Determines whether block smoothing is applicable and safe. We also latch
/// the current states of the `coef_bits[]` entries for the AC coefficients;
/// otherwise, if the input side of the decompressor advances into a new
/// scan, we might think the coefficients are known more accurately than they
/// really are.
///
/// # Errors
///
/// What allocating the latch reports.
///
/// # Panics
///
/// When `cinfo.coef` or `cinfo.comp_info` is absent.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
fn smoothing_ok(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut smoothing_useful = false;

    if !cinfo.progressive_mode || cinfo.coef_bits.is_none() {
        return Ok(false);
    }

    let num_components = cinfo.num_components as usize;
    /* Allocate latch area if not already done */
    if cinfo
        .coef
        .as_deref()
        .expect("cinfo->coef is NULL")
        .coef_bits_latch
        .is_none()
    {
        let len = num_components * 2 * SAVED_COEFS;
        let latch = alloc_small_vec(
            cinfo,
            JPOOL_IMAGE,
            num_components * 2 * (SAVED_COEFS * size_of::<i32>()),
            len,
            0,
        )?;
        cinfo
            .coef
            .as_deref_mut()
            .expect("cinfo->coef is NULL")
            .coef_bits_latch = Some(latch);
    }
    let coef_bits_latch = cinfo
        .coef
        .as_deref_mut()
        .expect("cinfo->coef is NULL")
        .coef_bits_latch
        .as_deref_mut()
        .expect("allocated above");
    let comp_info = cinfo
        .comp_info
        .as_deref()
        .expect("cinfo->comp_info is NULL");
    let all_coef_bits = cinfo.coef_bits.as_deref().expect("checked above");
    let mut latch: usize = 0;
    let mut prev_latch: usize = num_components * SAVED_COEFS;

    for (ci, compptr) in comp_info.iter().enumerate().take(num_components) {
        /* All components' quantization values must already be latched. */
        let Some(qtable) = compptr.quant_table.as_deref() else {
            return Ok(false);
        };
        /* Verify DC & first 9 AC quantizers are nonzero to avoid zero-divide. */
        if qtable.quantval[0] == 0
            || qtable.quantval[Q01_POS] == 0
            || qtable.quantval[Q10_POS] == 0
            || qtable.quantval[Q20_POS] == 0
            || qtable.quantval[Q11_POS] == 0
            || qtable.quantval[Q02_POS] == 0
            || qtable.quantval[Q03_POS] == 0
            || qtable.quantval[Q12_POS] == 0
            || qtable.quantval[Q21_POS] == 0
            || qtable.quantval[Q30_POS] == 0
        {
            return Ok(false);
        }
        /* DC values must be at least partly known for all components. */
        let coef_bits = &all_coef_bits[ci];
        let prev_coef_bits = &all_coef_bits[ci + num_components];
        if coef_bits[0] < 0 {
            return Ok(false);
        }
        coef_bits_latch[latch] = coef_bits[0];
        /* Block smoothing is helpful if some AC coefficients remain inaccurate. */
        for coefi in 1..SAVED_COEFS {
            if cinfo.input_scan_number > 1 {
                coef_bits_latch[prev_latch + coefi] = prev_coef_bits[coefi];
            } else {
                coef_bits_latch[prev_latch + coefi] = -1;
            }
            coef_bits_latch[latch + coefi] = coef_bits[coefi];
            if coef_bits[coefi] != 0 {
                smoothing_useful = true;
            }
        }
        latch += SAVED_COEFS;
        prev_latch += SAVED_COEFS;
    }

    Ok(smoothing_useful)
}

// upstream: src/jdcoefct.c decompress_smooth_data
/// Variant of `decompress_data` for use when doing block smoothing.
///
/// # Errors
///
/// What `consume_input` and `access_virt_barray` report.
///
/// # Panics
///
/// When a module, a virtual array, a quantization table, the latch or an
/// IDCT routine is absent.
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "C's conversions between int, JLONG, JCOEF and JDIMENSION"
)]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(non_snake_case, reason = "C names")]
pub fn decompress_smooth_data(
    cinfo: j_decompress_ptr<'_, '_>,
    output_buf: &mut [JSAMPROWS_MUT<'_, '_>],
) -> Result<i32, ErrorExit> {
    let last_iMCU_row: JDIMENSION = cinfo.total_iMCU_rows.wrapping_sub(1);

    /* Force some input to be done if we are getting ahead of the input. */
    while cinfo.input_scan_number <= cinfo.output_scan_number && !cinfo.inputctl().pub_.eoi_reached
    {
        if cinfo.input_scan_number == cinfo.output_scan_number {
            /* If input is working on current scan, we ordinarily want it to
             * have completed the current row.  But if input scan is DC,
             * we want it to keep two rows ahead so that next two block rows' DC
             * values are up to date.
             */
            let delta: JDIMENSION = if cinfo.Ss == 0 { 2 } else { 0 };
            if cinfo.input_iMCU_row > cinfo.output_iMCU_row + delta {
                break;
            }
        }
        let consume_input = cinfo.inputctl().pub_.consume_input;
        if consume_input(cinfo)? == JPEG_SUSPENDED {
            return Ok(JPEG_SUSPENDED);
        }
    }

    with_coef(cinfo, |cinfo, coef| {
        /* Keep a local variable to avoid looking it up more than once */
        let workspace = coef
            .workspace
            .as_deref_mut()
            .expect("coef->workspace is NULL");

        /* OK, output from the virtual arrays. */
        for ci in 0..cinfo.num_components as usize {
            let compptr = &cinfo.comp_info()[ci];
            /* Don't bother to IDCT an uninteresting component. */
            if !compptr.component_needed {
                continue;
            }
            let v_samp_factor = compptr.v_samp_factor;
            let height_in_blocks = compptr.height_in_blocks;
            /* Count non-dummy DCT block rows in this iMCU row. */
            let block_rows: i32;
            let mut access_rows: i32;
            if cinfo.output_iMCU_row + 1 < last_iMCU_row {
                block_rows = v_samp_factor;
                access_rows = block_rows * 3; /* this and next two iMCU rows */
            } else if cinfo.output_iMCU_row < last_iMCU_row {
                block_rows = v_samp_factor;
                access_rows = block_rows * 2; /* this and next iMCU row */
            } else {
                /* NB: can't use last_row_height here; it is input-side-dependent! */
                let rows = (height_in_blocks % v_samp_factor as JDIMENSION) as i32;
                block_rows = if rows == 0 { v_samp_factor } else { rows };
                access_rows = block_rows; /* this iMCU row only */
            }
            let whole_image = coef.whole_image[ci].expect("coef->whole_image is NULL");
            let access_virt_barray = cinfo.mem().pub_.access_virt_barray;
            /* Align the virtual buffer for this component; `buffer` is the
             * index of the current iMCU row's first row in the array's buffer. */
            let buffer: usize = if cinfo.output_iMCU_row > 1 {
                access_rows += 2 * v_samp_factor; /* prior two iMCU rows too */
                access_virt_barray(
                    cinfo,
                    whole_image,
                    (cinfo.output_iMCU_row - 2) * v_samp_factor as JDIMENSION,
                    access_rows as JDIMENSION,
                    false,
                )? + 2 * v_samp_factor as usize /* point to current iMCU row */
            } else if cinfo.output_iMCU_row > 0 {
                access_rows += v_samp_factor; /* prior iMCU row too */
                access_virt_barray(
                    cinfo,
                    whole_image,
                    (cinfo.output_iMCU_row - 1) * v_samp_factor as JDIMENSION,
                    access_rows as JDIMENSION,
                    false,
                )? + v_samp_factor as usize /* point to current iMCU row */
            } else {
                access_virt_barray(cinfo, whole_image, 0, access_rows as JDIMENSION, false)?
            };
            let cinfo: &jpeg_decompress_struct<'_> = cinfo;
            let compptr = &cinfo.comp_info()[ci];
            let master = &cinfo.master().pub_;
            let mem_buffer = cinfo.mem().virt_barray_list[whole_image.0]
                .mem_buffer
                .as_ref()
                .expect("virtual array not realized");
            /* Fetch component-dependent info.
             * If the current scan is incomplete, then we use the component-dependent
             * info from the previous scan.
             */
            let coef_bits_latch = coef
                .coef_bits_latch
                .as_deref()
                .expect("coef->coef_bits_latch is NULL");
            let coef_bits = if cinfo.output_iMCU_row > master.last_good_iMCU_row {
                &coef_bits_latch[(ci + cinfo.num_components as usize) * SAVED_COEFS..]
            } else {
                &coef_bits_latch[ci * SAVED_COEFS..]
            };

            /* We only do DC interpolation if no AC coefficient data is available. */
            let change_dc = coef_bits[1] == -1
                && coef_bits[2] == -1
                && coef_bits[3] == -1
                && coef_bits[4] == -1
                && coef_bits[5] == -1
                && coef_bits[6] == -1
                && coef_bits[7] == -1
                && coef_bits[8] == -1
                && coef_bits[9] == -1;

            let quanttbl = compptr
                .quant_table
                .as_deref()
                .expect("compptr->quant_table is NULL");
            let Q00 = JLONG::from(quanttbl.quantval[0]);
            let Q01 = JLONG::from(quanttbl.quantval[Q01_POS]);
            let Q10 = JLONG::from(quanttbl.quantval[Q10_POS]);
            let Q20 = JLONG::from(quanttbl.quantval[Q20_POS]);
            let Q11 = JLONG::from(quanttbl.quantval[Q11_POS]);
            let Q02 = JLONG::from(quanttbl.quantval[Q02_POS]);
            let mut Q03: JLONG = 0;
            let mut Q12: JLONG = 0;
            let mut Q21: JLONG = 0;
            let mut Q30: JLONG = 0;
            if change_dc {
                Q03 = JLONG::from(quanttbl.quantval[Q03_POS]);
                Q12 = JLONG::from(quanttbl.quantval[Q12_POS]);
                Q21 = JLONG::from(quanttbl.quantval[Q21_POS]);
                Q30 = JLONG::from(quanttbl.quantval[Q30_POS]);
            }
            let inverse_DCT = cinfo.idct().inverse_DCT[ci].expect("inverse_DCT is NULL");
            let output_ptr = &mut output_buf[ci];
            /* Loop over all DCT blocks to be processed. */
            let image_block_rows =
                (block_rows as JDIMENSION).wrapping_mul(cinfo.total_iMCU_rows) as i32;
            for block_row in 0..block_rows {
                let image_block_row = cinfo
                    .output_iMCU_row
                    .wrapping_mul(block_rows as JDIMENSION)
                    .wrapping_add(block_row as JDIMENSION)
                    as i32;
                let row = buffer + block_row as usize;
                let buffer_ptr = mem_buffer.row(row);

                let prev_block_row = if image_block_row > 0 {
                    mem_buffer.row(row - 1)
                } else {
                    buffer_ptr
                };

                let prev_prev_block_row = if image_block_row > 1 {
                    mem_buffer.row(row - 2)
                } else {
                    prev_block_row
                };

                let next_block_row = if image_block_row < image_block_rows - 1 {
                    mem_buffer.row(row + 1)
                } else {
                    buffer_ptr
                };

                let next_next_block_row = if image_block_row < image_block_rows - 2 {
                    mem_buffer.row(row + 2)
                } else {
                    next_block_row
                };

                /* The blocks of these rows from first_MCU_col on, as C's row
                 * pointers advance from there. */
                let first = master.first_MCU_col[ci] as usize;
                let buffer_ptr = &buffer_ptr[first..];
                let prev_block_row = &prev_block_row[first..];
                let prev_prev_block_row = &prev_prev_block_row[first..];
                let next_block_row = &next_block_row[first..];
                let next_next_block_row = &next_next_block_row[first..];

                /* We fetch the surrounding DC values using a sliding-register approach.
                 * Initialize all 25 here so as to do the right thing on narrow pics.
                 */
                let mut DC01 = i32::from(prev_prev_block_row[0][0]);
                let mut DC02 = DC01;
                let mut DC03 = DC01;
                let mut DC04 = DC01;
                let mut DC05 = DC01;
                let mut DC06 = i32::from(prev_block_row[0][0]);
                let mut DC07 = DC06;
                let mut DC08 = DC06;
                let mut DC09 = DC06;
                let mut DC10 = DC06;
                let mut DC11 = i32::from(buffer_ptr[0][0]);
                let mut DC12 = DC11;
                let mut DC13 = DC11;
                let mut DC14 = DC11;
                let mut DC15 = DC11;
                let mut DC16 = i32::from(next_block_row[0][0]);
                let mut DC17 = DC16;
                let mut DC18 = DC16;
                let mut DC19 = DC16;
                let mut DC20 = DC16;
                let mut DC21 = i32::from(next_next_block_row[0][0]);
                let mut DC22 = DC21;
                let mut DC23 = DC21;
                let mut DC24 = DC21;
                let mut DC25 = DC21;
                let mut output_col: JDIMENSION = 0;
                let last_block_column: JDIMENSION = compptr.width_in_blocks.wrapping_sub(1);
                for block_num in master.first_MCU_col[ci]..=master.last_MCU_col[ci] {
                    /* The block of block_num in each row: C's row pointers,
                     * advanced once per block. */
                    let x = (block_num - master.first_MCU_col[ci]) as usize;
                    /* Fetch current DCT block into workspace so we can modify it. */
                    jcopy_block_row(&buffer_ptr[x..], core::slice::from_mut(workspace), 1);
                    /* Update DC values */
                    if block_num == master.first_MCU_col[ci] && block_num < last_block_column {
                        DC04 = i32::from(prev_prev_block_row[x + 1][0]);
                        DC05 = DC04;
                        DC09 = i32::from(prev_block_row[x + 1][0]);
                        DC10 = DC09;
                        DC14 = i32::from(buffer_ptr[x + 1][0]);
                        DC15 = DC14;
                        DC19 = i32::from(next_block_row[x + 1][0]);
                        DC20 = DC19;
                        DC24 = i32::from(next_next_block_row[x + 1][0]);
                        DC25 = DC24;
                    }
                    if block_num + 1 < last_block_column {
                        DC05 = i32::from(prev_prev_block_row[x + 2][0]);
                        DC10 = i32::from(prev_block_row[x + 2][0]);
                        DC15 = i32::from(buffer_ptr[x + 2][0]);
                        DC20 = i32::from(next_block_row[x + 2][0]);
                        DC25 = i32::from(next_next_block_row[x + 2][0]);
                    }
                    /* If DC interpolation is enabled, compute coefficient estimates using
                     * a Gaussian-like kernel, keeping the averages of the DC values.
                     *
                     * If DC interpolation is disabled, compute coefficient estimates using
                     * an algorithm similar to the one described in Section K.8 of the JPEG
                     * standard, except applied to a 5x5 window rather than a 3x3 window.
                     *
                     * An estimate is applied only if the coefficient is still zero and is
                     * not known to be fully accurate.
                     */
                    let mut Al: i32;
                    let mut num: JLONG;
                    let mut pred: i32;
                    /* AC01 */
                    Al = coef_bits[1];
                    if Al != 0 && workspace[1] == 0 {
                        num = Q00
                            * JLONG::from(if change_dc {
                                -DC01 - DC02 + DC04 + DC05 - 3 * DC06 + 13 * DC07 - 13 * DC09
                                    + 3 * DC10
                                    - 3 * DC11
                                    + 38 * DC12
                                    - 38 * DC14
                                    + 3 * DC15
                                    - 3 * DC16
                                    + 13 * DC17
                                    - 13 * DC19
                                    + 3 * DC20
                                    - DC21
                                    - DC22
                                    + DC24
                                    + DC25
                            } else {
                                -7 * DC11 + 50 * DC12 - 50 * DC14 + 7 * DC15
                            });
                        if num >= 0 {
                            pred = (((Q01 << 7) + num) / (Q01 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                        } else {
                            pred = (((Q01 << 7) - num) / (Q01 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                            pred = -pred;
                        }
                        workspace[1] = pred as JCOEF;
                    }
                    /* AC10 */
                    Al = coef_bits[2];
                    if Al != 0 && workspace[8] == 0 {
                        num = Q00
                            * JLONG::from(if change_dc {
                                -DC01 - 3 * DC02 - 3 * DC03 - 3 * DC04 - DC05 - DC06
                                    + 13 * DC07
                                    + 38 * DC08
                                    + 13 * DC09
                                    - DC10
                                    + DC16
                                    - 13 * DC17
                                    - 38 * DC18
                                    - 13 * DC19
                                    + DC20
                                    + DC21
                                    + 3 * DC22
                                    + 3 * DC23
                                    + 3 * DC24
                                    + DC25
                            } else {
                                -7 * DC03 + 50 * DC08 - 50 * DC18 + 7 * DC23
                            });
                        if num >= 0 {
                            pred = (((Q10 << 7) + num) / (Q10 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                        } else {
                            pred = (((Q10 << 7) - num) / (Q10 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                            pred = -pred;
                        }
                        workspace[8] = pred as JCOEF;
                    }
                    /* AC20 */
                    Al = coef_bits[3];
                    if Al != 0 && workspace[16] == 0 {
                        num = Q00
                            * JLONG::from(if change_dc {
                                DC03 + 2 * DC07 + 7 * DC08 + 2 * DC09
                                    - 5 * DC12
                                    - 14 * DC13
                                    - 5 * DC14
                                    + 2 * DC17
                                    + 7 * DC18
                                    + 2 * DC19
                                    + DC23
                            } else {
                                -DC03 + 13 * DC08 - 24 * DC13 + 13 * DC18 - DC23
                            });
                        if num >= 0 {
                            pred = (((Q20 << 7) + num) / (Q20 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                        } else {
                            pred = (((Q20 << 7) - num) / (Q20 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                            pred = -pred;
                        }
                        workspace[16] = pred as JCOEF;
                    }
                    /* AC11 */
                    Al = coef_bits[4];
                    if Al != 0 && workspace[9] == 0 {
                        num = Q00
                            * JLONG::from(if change_dc {
                                -DC01 + DC05 + 9 * DC07 - 9 * DC09 - 9 * DC17 + 9 * DC19 + DC21
                                    - DC25
                            } else {
                                DC10 + DC16 - 10 * DC17 + 10 * DC19 - DC02 - DC20 + DC22 - DC24
                                    + DC04
                                    - DC06
                                    + 10 * DC07
                                    - 10 * DC09
                            });
                        if num >= 0 {
                            pred = (((Q11 << 7) + num) / (Q11 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                        } else {
                            pred = (((Q11 << 7) - num) / (Q11 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                            pred = -pred;
                        }
                        workspace[9] = pred as JCOEF;
                    }
                    /* AC02 */
                    Al = coef_bits[5];
                    if Al != 0 && workspace[2] == 0 {
                        num = Q00
                            * JLONG::from(if change_dc {
                                2 * DC07 - 5 * DC08 + 2 * DC09 + DC11 + 7 * DC12 - 14 * DC13
                                    + 7 * DC14
                                    + DC15
                                    + 2 * DC17
                                    - 5 * DC18
                                    + 2 * DC19
                            } else {
                                -DC11 + 13 * DC12 - 24 * DC13 + 13 * DC14 - DC15
                            });
                        if num >= 0 {
                            pred = (((Q02 << 7) + num) / (Q02 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                        } else {
                            pred = (((Q02 << 7) - num) / (Q02 << 8)) as i32;
                            if Al > 0 && pred >= (1 << Al) {
                                pred = (1 << Al) - 1;
                            }
                            pred = -pred;
                        }
                        workspace[2] = pred as JCOEF;
                    }
                    if change_dc {
                        /* AC03 */
                        Al = coef_bits[6];
                        if Al != 0 && workspace[3] == 0 {
                            num =
                                Q00 * JLONG::from(DC07 - DC09 + 2 * DC12 - 2 * DC14 + DC17 - DC19);
                            if num >= 0 {
                                pred = (((Q03 << 7) + num) / (Q03 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                            } else {
                                pred = (((Q03 << 7) - num) / (Q03 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                                pred = -pred;
                            }
                            workspace[3] = pred as JCOEF;
                        }
                        /* AC12 */
                        Al = coef_bits[7];
                        if Al != 0 && workspace[10] == 0 {
                            num =
                                Q00 * JLONG::from(DC07 - 3 * DC08 + DC09 - DC17 + 3 * DC18 - DC19);
                            if num >= 0 {
                                pred = (((Q12 << 7) + num) / (Q12 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                            } else {
                                pred = (((Q12 << 7) - num) / (Q12 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                                pred = -pred;
                            }
                            workspace[10] = pred as JCOEF;
                        }
                        /* AC21 */
                        Al = coef_bits[8];
                        if Al != 0 && workspace[17] == 0 {
                            num =
                                Q00 * JLONG::from(DC07 - DC09 - 3 * DC12 + 3 * DC14 + DC17 - DC19);
                            if num >= 0 {
                                pred = (((Q21 << 7) + num) / (Q21 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                            } else {
                                pred = (((Q21 << 7) - num) / (Q21 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                                pred = -pred;
                            }
                            workspace[17] = pred as JCOEF;
                        }
                        /* AC30 */
                        Al = coef_bits[9];
                        if Al != 0 && workspace[24] == 0 {
                            num =
                                Q00 * JLONG::from(DC07 + 2 * DC08 + DC09 - DC17 - 2 * DC18 - DC19);
                            if num >= 0 {
                                pred = (((Q30 << 7) + num) / (Q30 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                            } else {
                                pred = (((Q30 << 7) - num) / (Q30 << 8)) as i32;
                                if Al > 0 && pred >= (1 << Al) {
                                    pred = (1 << Al) - 1;
                                }
                                pred = -pred;
                            }
                            workspace[24] = pred as JCOEF;
                        }
                        /* coef_bits[0] is non-negative.  Otherwise this function would not
                         * be called.
                         */
                        num = Q00
                            * JLONG::from(
                                -2 * DC01 - 6 * DC02 - 8 * DC03 - 6 * DC04 - 2 * DC05 - 6 * DC06
                                    + 6 * DC07
                                    + 42 * DC08
                                    + 6 * DC09
                                    - 6 * DC10
                                    - 8 * DC11
                                    + 42 * DC12
                                    + 152 * DC13
                                    + 42 * DC14
                                    - 8 * DC15
                                    - 6 * DC16
                                    + 6 * DC17
                                    + 42 * DC18
                                    + 6 * DC19
                                    - 6 * DC20
                                    - 2 * DC21
                                    - 6 * DC22
                                    - 8 * DC23
                                    - 6 * DC24
                                    - 2 * DC25,
                            );
                        if num >= 0 {
                            pred = (((Q00 << 7) + num) / (Q00 << 8)) as i32;
                        } else {
                            pred = (((Q00 << 7) - num) / (Q00 << 8)) as i32;
                            pred = -pred;
                        }
                        workspace[0] = pred as JCOEF;
                    } /* change_dc */

                    /* OK, do the IDCT */
                    inverse_DCT(
                        cinfo,
                        compptr,
                        workspace,
                        output_ptr.offset(block_row as usize * compptr.DCT_scaled_size as usize),
                        output_col,
                    );
                    /* Advance for next column */
                    DC01 = DC02;
                    DC02 = DC03;
                    DC03 = DC04;
                    DC04 = DC05;
                    DC06 = DC07;
                    DC07 = DC08;
                    DC08 = DC09;
                    DC09 = DC10;
                    DC11 = DC12;
                    DC12 = DC13;
                    DC13 = DC14;
                    DC14 = DC15;
                    DC16 = DC17;
                    DC17 = DC18;
                    DC18 = DC19;
                    DC19 = DC20;
                    DC21 = DC22;
                    DC22 = DC23;
                    DC23 = DC24;
                    DC24 = DC25;
                    output_col += compptr.DCT_scaled_size as JDIMENSION;
                }
            }
        }
        Ok(())
    })?;

    cinfo.output_iMCU_row += 1;
    if cinfo.output_iMCU_row < cinfo.total_iMCU_rows {
        return Ok(JPEG_ROW_COMPLETED);
    }
    Ok(JPEG_SCAN_COMPLETED)
}

// upstream: src/jdcoefct.c _jinit_d_coef_controller
/// Initializes the coefficient buffer controller.
///
/// # Errors
///
/// `JERR_BAD_PRECISION` for a data precision other than 8, and what the
/// allocations report.
///
/// # Panics
///
/// When `cinfo.comp_info` is absent.
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    reason = "C's conversions between int, long and JDIMENSION"
)]
pub fn jinit_d_coef_controller(
    cinfo: j_decompress_ptr<'_, '_>,
    need_full_buffer: bool,
) -> Result<(), ErrorExit> {
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    /* C's memset leaves consume_data and decompress_data NULL until they are
     * selected below; the port holds the single-pass routines meanwhile. */
    let coef = alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_my_coef_controller,
        my_coef_controller {
            pub_: jpeg_d_coef_controller {
                start_input_pass,
                consume_data: dummy_consume_data,
                start_output_pass,
                decompress_data: decompress_onepass,
                MCU_ctr: 0,
                MCU_vert_offset: 0,
                MCU_rows_per_iMCU_row: 0,
                coef_arrays: None,
            },
            MCU_ctr: 0,
            MCU_vert_offset: 0,
            MCU_rows_per_iMCU_row: 0,
            MCU_buffer: None,
            workspace: None,
            whole_image: [None; MAX_COMPONENTS],
            coef_bits_latch: None,
        },
    )?;
    cinfo.coef = Some(coef);

    /* Create the coefficient buffer. */
    if need_full_buffer {
        /* Allocate a full-image virtual array for each component, */
        /* padded to a multiple of samp_factor DCT blocks in each direction. */
        /* Note we ask for a pre-zeroed array. */
        for ci in 0..cinfo.num_components as usize {
            let compptr = &cinfo.comp_info()[ci];
            let mut access_rows = compptr.v_samp_factor;
            /* If block smoothing could be used, need a bigger window */
            if cinfo.progressive_mode {
                access_rows *= 5;
            }
            let blocksperrow = jround_up(
                c_long::from(compptr.width_in_blocks),
                c_long::from(compptr.h_samp_factor),
            ) as JDIMENSION;
            let numrows = jround_up(
                c_long::from(compptr.height_in_blocks),
                c_long::from(compptr.v_samp_factor),
            ) as JDIMENSION;
            let request_virt_barray = cinfo.mem().pub_.request_virt_barray;
            let array = request_virt_barray(
                cinfo,
                JPOOL_IMAGE,
                true,
                blocksperrow,
                numrows,
                access_rows as JDIMENSION,
            )?;
            cinfo.coef.as_deref_mut().expect("set above").whole_image[ci] = Some(array);
        }
        let coef = cinfo.coef.as_deref_mut().expect("set above");
        coef.pub_.consume_data = consume_data;
        coef.pub_.decompress_data = decompress_data;
        coef.pub_.coef_arrays = Some(coef.whole_image); /* link to virtual arrays */
    } else {
        /* We only need a single-MCU buffer. */
        let buffer = alloc_large_vec(
            cinfo,
            JPOOL_IMAGE,
            D_MAX_BLOCKS_IN_MCU * SIZEOF_JBLOCK,
            D_MAX_BLOCKS_IN_MCU,
            [0; DCTSIZE2],
        )?;
        let coef = cinfo.coef.as_deref_mut().expect("set above");
        coef.MCU_buffer = Some(
            buffer
                .into_boxed_slice()
                .try_into()
                .expect("D_MAX_BLOCKS_IN_MCU blocks"),
        );
        coef.pub_.consume_data = dummy_consume_data;
        coef.pub_.decompress_data = decompress_onepass;
        coef.pub_.coef_arrays = None; /* flag for no virtual arrays */
    }

    /* Allocate the workspace buffer */
    let workspace = alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        size_of::<JCOEF>() * DCTSIZE2,
        [0; DCTSIZE2],
    )?;
    cinfo.coef.as_deref_mut().expect("set above").workspace = Some(workspace);
    Ok(())
}
