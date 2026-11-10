// libjpeg-turbo 3.2.1, src/jdmainct.h:
//
// /*
//  * jdmainct.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdmainct.c:
//
// /*
//  * jdmainct.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010, 2016, 2022, 2024, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the main buffer controller for decompression.
//  * The main buffer lies between the JPEG decompressor proper and the
//  * post-processor; it holds downsampled data in the JPEG colorspace.
//  *
//  * Note that this code is bypassed in raw-data mode, since the application
//  * supplies the equivalent of the main buffer in that case.
//  */
//! Translation of `src/jdmainct.h` and `src/jdmainct.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
//!
//! The workspace of each component is one sample array. The "funny" pointer
//! lists of the context case are lists of row offsets into that array: C
//! allocates both lists of a component in one block of `2 * rgroup * (M +
//! 4)` pointers, and `xbuffer[0][ci]` and `xbuffer[1][ci]` point into it,
//! `rgroup` and `rgroup * (M + 5)` entries deep, so that each list has one
//! row group at negative offsets. The port keeps the block (`xbuf`) and the
//! two positions (`xbuffer`), and hands the lists to the coefficient
//! controller as row lists from those positions and to the postprocessor as
//! [`JSAMPROWS_AT`], which can reach the row group before them. With
//! `QUANT_2PASS_SUPPORTED` undefined, `process_data_crank_post` and the
//! `JBUF_CRANK_DEST` case are not compiled. The methods take the module out
//! of `cinfo.main`; the coefficient controller and the postprocessor they
//! call never reach it.

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemmgr::alloc_small_box;
use crate::jmorecfg::{
    C_BOOLEAN, C_INT, C_JDIMENSION, C_POINTER, CLayout, JDIMENSION, MAX_COMPONENTS,
};
use crate::jpegint::{J_BUF_MODE, jpeg_d_main_controller};
use crate::jpeglib::{
    JPOOL_IMAGE, JSAMPROWS_AT, JSAMPROWS_MUT, RowListAt, RowListMut, SampleArray, j_decompress_ptr,
    jpeg_decompress_struct,
};

// upstream: src/jdmainct.h my_main_controller
/// Private buffer controller object.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Debug)]
pub struct my_main_controller {
    /// Public fields.
    pub pub_: jpeg_d_main_controller,

    /// Allocated workspace (M or M+2 row groups), one array per component.
    pub buffer: Vec<SampleArray>,

    /// Have we gotten an iMCU row from decoder?
    pub buffer_full: bool,
    /// Counts row groups output to postprocessor.
    pub rowgroup_ctr: JDIMENSION,

    /* Remaining fields are only used in the context case. */
    /// The block of both pointer lists of each component: row offsets into
    /// `buffer[ci]`.
    pub xbuf: Vec<Vec<usize>>,
    /// These are the master pointers to the funny-order pointer lists: the
    /// position of row 0 of list `w` in `xbuf[ci]`; empty until
    /// `alloc_funny_pointers`.
    pub xbuffer: [Vec<usize>; 2],

    /// Indicates which pointer set is now in use.
    pub whichptr: i32,
    /// `process_data` state machine status.
    pub context_state: i32,
    /// Row groups available to postprocessor.
    pub rowgroups_avail: JDIMENSION,
    /// Counts iMCU rows to detect image top/bot.
    pub iMCU_row_ctr: JDIMENSION,
}

/// `sizeof(my_main_controller)` on the target, with the `process_data_12`
/// and `process_data_16` pointers of `struct jpeg_d_main_controller`.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_main_controller: usize = CLayout::structure(&[
    // struct jpeg_d_main_controller
    CLayout::structure(&[C_POINTER.array(4)]),
    C_POINTER.array(MAX_COMPONENTS),
    C_BOOLEAN,
    C_JDIMENSION,
    C_POINTER.array(2),
    C_INT,
    C_INT,
    C_JDIMENSION,
    C_JDIMENSION,
])
.size;

/* context_state values: */
// upstream: src/jdmainct.h CTX_PREPARE_FOR_IMCU
/// Need to prepare for MCU row.
pub const CTX_PREPARE_FOR_IMCU: i32 = 0;
// upstream: src/jdmainct.h CTX_PROCESS_IMCU
/// Feeding iMCU to postprocessor.
pub const CTX_PROCESS_IMCU: i32 = 1;
// upstream: src/jdmainct.h CTX_POSTPONED_ROW
/// Feeding postponed row group.
pub const CTX_POSTPONED_ROW: i32 = 2;

/// The height of a row group of `compptr`, as C computes it in every
/// function of this file. Not an upstream item.
fn rgroup_of(cinfo: &jpeg_decompress_struct<'_>, ci: usize) -> i32 {
    let compptr = &cinfo.comp_info()[ci];
    (compptr.v_samp_factor * compptr.DCT_scaled_size) / cinfo.min_DCT_scaled_size
}

/// The index in `xbuf[ci]` of entry `i` of list `w`, which may be negative
/// down to `-rgroup`. Not an upstream item.
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's signed subscripts of a pointer into the list"
)]
fn xindex(main_ptr: &my_main_controller, w: usize, ci: usize, i: i32) -> usize {
    (main_ptr.xbuffer[w][ci] as isize + i as isize) as usize
}

// upstream: src/jdmainct.h set_wraparound_pointers
/// Sets up the "wraparound" pointers at top and bottom of the pointer lists.
/// This changes the pointer list state from top-of-image to the normal
/// state.
#[expect(non_snake_case, reason = "C names")]
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
pub fn set_wraparound_pointers(
    cinfo: &jpeg_decompress_struct<'_>,
    main_ptr: &mut my_main_controller,
) {
    let M = cinfo.min_DCT_scaled_size;

    for ci in 0..cinfo.num_components as usize {
        let rgroup = rgroup_of(cinfo, ci); /* height of a row group of component */
        for i in 0..rgroup {
            let (a0, b0) = (
                xindex(main_ptr, 0, ci, i - rgroup),
                xindex(main_ptr, 0, ci, rgroup * (M + 1) + i),
            );
            main_ptr.xbuf[ci][a0] = main_ptr.xbuf[ci][b0];
            let (a1, b1) = (
                xindex(main_ptr, 1, ci, i - rgroup),
                xindex(main_ptr, 1, ci, rgroup * (M + 1) + i),
            );
            main_ptr.xbuf[ci][a1] = main_ptr.xbuf[ci][b1];
            let (c0, d0) = (
                xindex(main_ptr, 0, ci, rgroup * (M + 2) + i),
                xindex(main_ptr, 0, ci, i),
            );
            main_ptr.xbuf[ci][c0] = main_ptr.xbuf[ci][d0];
            let (c1, d1) = (
                xindex(main_ptr, 1, ci, rgroup * (M + 2) + i),
                xindex(main_ptr, 1, ci, i),
            );
            main_ptr.xbuf[ci][c1] = main_ptr.xbuf[ci][d1];
        }
    }
}

/// Runs `body` with the main controller taken out of `cinfo.main` and puts
/// it back on every path (ADR 0005, "A method's own state"). Not an
/// upstream item.
fn with_main<R>(
    cinfo: j_decompress_ptr<'_, '_>,
    body: impl FnOnce(j_decompress_ptr<'_, '_>, &mut my_main_controller) -> Result<R, ErrorExit>,
) -> Result<R, ErrorExit> {
    let mut main_ptr = cinfo.main.take().expect("cinfo->main is NULL");
    let result = body(cinfo, &mut main_ptr);
    cinfo.main = Some(main_ptr);
    result
}

// upstream: src/jdmainct.c alloc_funny_pointers
/// Allocates space for the funny pointer lists. This is done only once, not
/// once per pass.
///
/// # Errors
///
/// What the allocations report.
#[expect(non_snake_case, reason = "C names")]
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
fn alloc_funny_pointers(
    cinfo: j_decompress_ptr<'_, '_>,
    main_ptr: &mut my_main_controller,
) -> Result<(), ErrorExit> {
    let M = cinfo.min_DCT_scaled_size;
    let num_components = cinfo.num_components as usize;

    /* Get top-level space for component array pointers.
     * We alloc both arrays with one call to save a few cycles.
     */
    let alloc_small = cinfo.mem().pub_.alloc_small;
    alloc_small(
        cinfo,
        JPOOL_IMAGE,
        num_components * 2 * C_POINTER.size,
        &mut || true,
    )?;
    main_ptr.xbuffer = [vec![0; num_components], vec![0; num_components]];

    for ci in 0..num_components {
        let rgroup = rgroup_of(cinfo, ci) as usize; /* height of a row group of component */
        /* Get space for pointer lists --- M+4 row groups in each list.
         * We alloc both pointer lists with one call to save a few cycles.
         */
        let len = 2 * (rgroup * (M as usize + 4));
        let mut xbuf: Vec<usize> = Vec::new();
        alloc_small(cinfo, JPOOL_IMAGE, len * C_POINTER.size, &mut || {
            xbuf.try_reserve_exact(len).is_ok()
        })?;
        xbuf.resize(len, 0);
        main_ptr.xbuf.push(xbuf);
        /* want one row group at negative offsets */
        main_ptr.xbuffer[0][ci] = rgroup;
        main_ptr.xbuffer[1][ci] = rgroup + rgroup * (M as usize + 4);
    }
    Ok(())
}

// upstream: src/jdmainct.c make_funny_pointers
/// Creates the funny pointer lists discussed in the comments above. The
/// actual workspace is already allocated (in `main->buffer`), and the space
/// for the pointer lists is allocated too. This routine just fills in the
/// curiously ordered lists. This will be repeated at the beginning of each
/// pass.
#[expect(non_snake_case, reason = "C names")]
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
fn make_funny_pointers(cinfo: &jpeg_decompress_struct<'_>, main_ptr: &mut my_main_controller) {
    let M = cinfo.min_DCT_scaled_size;

    for ci in 0..cinfo.num_components as usize {
        let rgroup = rgroup_of(cinfo, ci); /* height of a row group of component */
        /* First copy the workspace pointers as-is */
        for i in 0..rgroup * (M + 2) {
            let row = main_ptr.buffer[ci].rows[i as usize];
            let (a, b) = (xindex(main_ptr, 0, ci, i), xindex(main_ptr, 1, ci, i));
            main_ptr.xbuf[ci][b] = row;
            main_ptr.xbuf[ci][a] = row;
        }
        /* In the second list, put the last four row groups in swapped order */
        for i in 0..rgroup * 2 {
            let a = xindex(main_ptr, 1, ci, rgroup * (M - 2) + i);
            main_ptr.xbuf[ci][a] = main_ptr.buffer[ci].rows[(rgroup * M + i) as usize];
            let b = xindex(main_ptr, 1, ci, rgroup * M + i);
            main_ptr.xbuf[ci][b] = main_ptr.buffer[ci].rows[(rgroup * (M - 2) + i) as usize];
        }
        /* The wraparound pointers at top and bottom will be filled later
         * (see set_wraparound_pointers, below).  Initially we want the "above"
         * pointers to duplicate the first actual data line.  This only needs
         * to happen in xbuffer[0].
         */
        for i in 0..rgroup {
            let (a, b) = (
                xindex(main_ptr, 0, ci, i - rgroup),
                xindex(main_ptr, 0, ci, 0),
            );
            main_ptr.xbuf[ci][a] = main_ptr.xbuf[ci][b];
        }
    }
}

// upstream: src/jdmainct.c set_bottom_pointers
/// Changes the pointer lists to duplicate the last sample row at the bottom
/// of the image. `whichptr` indicates which xbuffer holds the final iMCU
/// row. Also sets `rowgroups_avail` to indicate number of nondummy row
/// groups in row.
#[expect(non_snake_case, reason = "C names")]
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's conversions between int and JDIMENSION"
)]
fn set_bottom_pointers(cinfo: &jpeg_decompress_struct<'_>, main_ptr: &mut my_main_controller) {
    for ci in 0..cinfo.num_components as usize {
        let compptr = &cinfo.comp_info()[ci];
        /* Count sample rows in one iMCU row and in one row group */
        let iMCUheight = compptr.v_samp_factor * compptr.DCT_scaled_size;
        let rgroup = iMCUheight / cinfo.min_DCT_scaled_size;
        /* Count nondummy sample rows remaining for this component */
        let mut rows_left = (compptr.downsampled_height % iMCUheight as JDIMENSION) as i32;
        if rows_left == 0 {
            rows_left = iMCUheight;
        }
        /* Count nondummy row groups.  Should get same answer for each component,
         * so we need only do it once.
         */
        if ci == 0 {
            main_ptr.rowgroups_avail = ((rows_left - 1) / rgroup + 1) as JDIMENSION;
        }
        /* Duplicate the last real sample row rgroup*2 times; this pads out the
         * last partial rowgroup and ensures at least one full rowgroup of context.
         */
        let w = main_ptr.whichptr as usize;
        for i in 0..rgroup * 2 {
            let (a, b) = (
                xindex(main_ptr, w, ci, rows_left + i),
                xindex(main_ptr, w, ci, rows_left - 1),
            );
            main_ptr.xbuf[ci][a] = main_ptr.xbuf[ci][b];
        }
    }
}

// upstream: src/jdmainct.c start_pass_main
/// Initializes for a processing pass.
///
/// # Errors
///
/// `JERR_BAD_BUFFER_MODE` for any mode but `JBUF_PASS_THRU`.
///
/// # Panics
///
/// When `cinfo.main` or `cinfo.upsample` is absent.
pub fn start_pass_main(
    cinfo: j_decompress_ptr<'_, '_>,
    pass_mode: J_BUF_MODE,
) -> Result<(), ErrorExit> {
    with_main(cinfo, |cinfo, main_ptr| {
        match pass_mode {
            J_BUF_MODE::JBUF_PASS_THRU => {
                if cinfo.upsample().need_context_rows {
                    main_ptr.pub_.process_data = Some(process_data_context_main);
                    make_funny_pointers(cinfo, main_ptr); /* Create the xbuffer[] lists */
                    main_ptr.whichptr = 0; /* Read first iMCU row into xbuffer[0] */
                    main_ptr.context_state = CTX_PREPARE_FOR_IMCU;
                    main_ptr.iMCU_row_ctr = 0;
                } else {
                    /* Simple case with no context needed */
                    main_ptr.pub_.process_data = Some(process_data_simple_main);
                }
                main_ptr.buffer_full = false; /* Mark buffer empty */
                main_ptr.rowgroup_ctr = 0;
            }
            _ => return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_BUFFER_MODE)),
        }
        Ok(())
    })
}

/// The workspace as the coefficient controller's output: list `w`, or the
/// arrays themselves when `w` is `None`. Not an upstream item.
fn decompress_target(
    main_ptr: &mut my_main_controller,
    w: Option<usize>,
) -> Vec<JSAMPROWS_MUT<'_, '_>> {
    let xbuf = &main_ptr.xbuf;
    let xbuffer = &main_ptr.xbuffer;
    main_ptr
        .buffer
        .iter_mut()
        .enumerate()
        .map(|(ci, array)| match w {
            None => array.as_rows_mut(),
            Some(w) => RowListMut::Shared {
                storage: &mut array.buffer,
                rows: &xbuf[ci][xbuffer[w][ci]..],
            },
        })
        .collect()
}

/// The workspace as the postprocessor's input: list `w`, or the arrays
/// themselves when `w` is `None`. Not an upstream item.
fn post_input<'a>(
    buffer: &'a [SampleArray],
    xbuf: &'a [Vec<usize>],
    xbuffer: &'a [Vec<usize>; 2],
    w: Option<usize>,
) -> Vec<JSAMPROWS_AT<'a>> {
    buffer
        .iter()
        .enumerate()
        .map(|(ci, array)| match w {
            None => RowListAt {
                storage: &array.buffer,
                rows: &array.rows,
                origin: 0,
            },
            Some(w) => RowListAt {
                storage: &array.buffer,
                rows: &xbuf[ci],
                origin: xbuffer[w][ci],
            },
        })
        .collect()
}

// upstream: src/jdmainct.c process_data_simple_main
/// Processes some data. This handles the simple case where no context is
/// required.
///
/// # Errors
///
/// What the coefficient controller reports.
///
/// # Panics
///
/// When a module or method is absent.
#[expect(clippy::cast_sign_loss, reason = "C's (JDIMENSION)min_DCT_scaled_size")]
pub fn process_data_simple_main(
    cinfo: j_decompress_ptr<'_, '_>,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    out_row_ctr: &mut JDIMENSION,
    out_rows_avail: JDIMENSION,
) -> Result<(), ErrorExit> {
    with_main(cinfo, |cinfo, main_ptr| {
        /* Read input data if we haven't filled the main buffer yet */
        if !main_ptr.buffer_full {
            let decompress_data = cinfo.coef().decompress_data;
            let mut target = decompress_target(main_ptr, None);
            if decompress_data(cinfo, &mut target)? == 0 {
                return Ok(()); /* suspension forced, can do nothing more */
            }
            main_ptr.buffer_full = true; /* OK, we have an iMCU row to work with */
        }

        /* There are always min_DCT_scaled_size row groups in an iMCU row. */
        let rowgroups_avail = cinfo.min_DCT_scaled_size as JDIMENSION;
        /* Note: at the bottom of the image, we may pass extra garbage row groups
         * to the postprocessor.  The postprocessor has to check for bottom
         * of image anyway (at row resolution), so no point in us doing it too.
         */

        /* Feed the postprocessor */
        let post_process_data = cinfo
            .post()
            .post_process_data
            .expect("post_process_data is NULL");
        let input = post_input(&main_ptr.buffer, &main_ptr.xbuf, &main_ptr.xbuffer, None);
        post_process_data(
            cinfo,
            &input,
            &mut main_ptr.rowgroup_ctr,
            rowgroups_avail,
            output_buf,
            out_row_ctr,
            out_rows_avail,
        );
        drop(input);

        /* Has postprocessor consumed all the data yet? If so, mark buffer empty */
        if main_ptr.rowgroup_ctr >= rowgroups_avail {
            main_ptr.buffer_full = false;
            main_ptr.rowgroup_ctr = 0;
        }
        Ok(())
    })
}

// upstream: src/jdmainct.c process_data_context_main
/// Processes some data. This handles the case where context rows must be
/// provided.
///
/// # Errors
///
/// What the coefficient controller reports.
///
/// # Panics
///
/// When a module or method is absent.
#[expect(
    clippy::cast_sign_loss,
    reason = "C's conversions of min_DCT_scaled_size and whichptr"
)]
pub fn process_data_context_main(
    cinfo: j_decompress_ptr<'_, '_>,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    out_row_ctr: &mut JDIMENSION,
    out_rows_avail: JDIMENSION,
) -> Result<(), ErrorExit> {
    with_main(cinfo, |cinfo, main_ptr| {
        /* Read input data if we haven't filled the main buffer yet */
        if !main_ptr.buffer_full {
            let decompress_data = cinfo.coef().decompress_data;
            let w = main_ptr.whichptr as usize;
            let mut target = decompress_target(main_ptr, Some(w));
            if decompress_data(cinfo, &mut target)? == 0 {
                return Ok(()); /* suspension forced, can do nothing more */
            }
            main_ptr.buffer_full = true; /* OK, we have an iMCU row to work with */
            main_ptr.iMCU_row_ctr += 1; /* count rows received */
        }

        /* Postprocessor typically will not swallow all the input data it is handed
         * in one call (due to filling the output buffer first).  Must be prepared
         * to exit and restart.  This switch lets us keep track of how far we got.
         * Note that each case falls through to the next on successful completion.
         */
        let post_process_data = cinfo
            .post()
            .post_process_data
            .expect("post_process_data is NULL");
        if main_ptr.context_state == CTX_POSTPONED_ROW {
            /* Call postprocessor using previously set pointers for postponed row */
            let w = main_ptr.whichptr as usize;
            let input = post_input(&main_ptr.buffer, &main_ptr.xbuf, &main_ptr.xbuffer, Some(w));
            post_process_data(
                cinfo,
                &input,
                &mut main_ptr.rowgroup_ctr,
                main_ptr.rowgroups_avail,
                output_buf.reborrow(),
                out_row_ctr,
                out_rows_avail,
            );
            drop(input);
            if main_ptr.rowgroup_ctr < main_ptr.rowgroups_avail {
                return Ok(()); /* Need to suspend */
            }
            main_ptr.context_state = CTX_PREPARE_FOR_IMCU;
            if *out_row_ctr >= out_rows_avail {
                return Ok(()); /* Postprocessor exactly filled output buf */
            }
        }
        if main_ptr.context_state == CTX_PREPARE_FOR_IMCU {
            /* Prepare to process first M-1 row groups of this iMCU row */
            main_ptr.rowgroup_ctr = 0;
            main_ptr.rowgroups_avail = (cinfo.min_DCT_scaled_size - 1) as JDIMENSION;
            /* Check for bottom of image: if so, tweak pointers to "duplicate"
             * the last sample row, and adjust rowgroups_avail to ignore padding rows.
             */
            if main_ptr.iMCU_row_ctr == cinfo.total_iMCU_rows {
                set_bottom_pointers(cinfo, main_ptr);
            }
            main_ptr.context_state = CTX_PROCESS_IMCU;
        }
        if main_ptr.context_state == CTX_PROCESS_IMCU {
            /* Call postprocessor using previously set pointers */
            let w = main_ptr.whichptr as usize;
            let input = post_input(&main_ptr.buffer, &main_ptr.xbuf, &main_ptr.xbuffer, Some(w));
            post_process_data(
                cinfo,
                &input,
                &mut main_ptr.rowgroup_ctr,
                main_ptr.rowgroups_avail,
                output_buf.reborrow(),
                out_row_ctr,
                out_rows_avail,
            );
            drop(input);
            if main_ptr.rowgroup_ctr < main_ptr.rowgroups_avail {
                return Ok(()); /* Need to suspend */
            }
            /* After the first iMCU, change wraparound pointers to normal state */
            if main_ptr.iMCU_row_ctr == 1 {
                set_wraparound_pointers(cinfo, main_ptr);
            }
            /* Prepare to load new iMCU row using other xbuffer list */
            main_ptr.whichptr ^= 1; /* 0=>1 or 1=>0 */
            main_ptr.buffer_full = false;
            /* Still need to process last row group of this iMCU row, */
            /* which is saved at index M+1 of the other xbuffer */
            main_ptr.rowgroup_ctr = (cinfo.min_DCT_scaled_size + 1) as JDIMENSION;
            main_ptr.rowgroups_avail = (cinfo.min_DCT_scaled_size + 2) as JDIMENSION;
            main_ptr.context_state = CTX_POSTPONED_ROW;
        }
        Ok(())
    })
}

// upstream: src/jdmainct.c _jinit_d_main_controller
/// Initializes the main buffer controller.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_BAD_BUFFER_MODE`, `JERR_NOTIMPL` and what the
/// allocations report.
///
/// # Panics
///
/// When `cinfo.upsample` or `cinfo.comp_info` is absent.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
pub fn jinit_d_main_controller(
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

    let main_ptr = alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_my_main_controller,
        my_main_controller {
            pub_: jpeg_d_main_controller {
                start_pass: start_pass_main,
                process_data: None,
            },
            buffer: Vec::new(),
            buffer_full: false,
            rowgroup_ctr: 0,
            xbuf: Vec::new(),
            xbuffer: [Vec::new(), Vec::new()],
            whichptr: 0,
            context_state: 0,
            rowgroups_avail: 0,
            iMCU_row_ctr: 0,
        },
    )?;
    cinfo.main = Some(main_ptr);

    if need_full_buffer {
        /* shouldn't happen */
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_BUFFER_MODE));
    }

    with_main(cinfo, |cinfo, main_ptr| {
        /* Allocate the workspace.
         * ngroups is the number of row groups we need.
         */
        let ngroups = if cinfo.upsample().need_context_rows {
            if cinfo.min_DCT_scaled_size < 2 {
                /* unsupported, see comments above */
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOTIMPL));
            }
            alloc_funny_pointers(cinfo, main_ptr)?; /* Alloc space for xbuffer[] lists */
            cinfo.min_DCT_scaled_size + 2
        } else {
            cinfo.min_DCT_scaled_size
        };

        for ci in 0..cinfo.num_components as usize {
            let rgroup = rgroup_of(cinfo, ci); /* height of a row group of component */
            let compptr = &cinfo.comp_info()[ci];
            let samplesperrow = compptr.width_in_blocks * compptr.DCT_scaled_size as JDIMENSION;
            let alloc_sarray = cinfo.mem().pub_.alloc_sarray;
            let array = alloc_sarray(
                cinfo,
                JPOOL_IMAGE,
                samplesperrow,
                (rgroup * ngroups) as JDIMENSION,
            )?;
            main_ptr.buffer.push(array);
        }
        Ok(())
    })
}
