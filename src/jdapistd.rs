// libjpeg-turbo 3.2.1, src/jdapistd.c:
//
// /*
//  * jdapistd.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010, 2015-2020, 2022-2026, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains application interface code for the decompression half
//  * of the JPEG library.  These are the "standard" API routines that are
//  * used in the normal full-decompression case.  They are not used by a
//  * transcoding-only application.  Note that if an application links in
//  * jpeg_start_decompress, it will end up linking in the entire decompressor.
//  * We thus must separate this file from jdapimin.c to avoid linking the
//  * whole decompression library into a transcoder.
//  */
//! Translation of `src/jdapistd.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 6 (Control and API).
//!
//! The file is compiled once per sample size; the port is its 8-bit
//! instance, with the names of `jpeglib.h` (`jpeg_read_scanlines` for C's
//! `_jpeg_read_scanlines`). Following ADR 0005, `QUANT_2PASS_SUPPORTED` is
//! treated as undefined, so a dummy pass raises `JERR_NOT_COMPILED` as
//! upstream's `#else` does, and the color quantizer (`cinfo->cquantize`) is
//! absent: `read_and_discard_scanlines` has no quantizer to silence, and
//! `noop_quantize` is not translated. `D_LOSSLESS_SUPPORTED` is treated as
//! undefined as well, and the 12- and 16-bit `process_data` slots are
//! absent, so the dummy-pass branches for other precisions are the
//! `JERR_BAD_PRECISION` errors that upstream raises for a missing slot.
//! `WITH_PROFILE` is off.

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jdcoefct::{my_coef_controller, start_iMCU_row};
use crate::jdmainct::{CTX_PREPARE_FOR_IMCU, my_main_controller, set_wraparound_pointers};
use crate::jdmaster::jinit_master_decompress;
use crate::jdmerge::my_merged_upsampler;
use crate::jdsample::{jinit_upsampler, my_upsampler};
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE, WARNMS};
use crate::jmorecfg::{
    D_MULTISCAN_FILES_SUPPORTED, JDIMENSION, JSAMPLE, QUANT_2PASS_SUPPORTED,
    UPSAMPLE_MERGING_SUPPORTED,
};
use crate::jpegint::{
    DSTATE_BUFIMAGE, DSTATE_BUFPOST, DSTATE_PRELOAD, DSTATE_PRESCAN, DSTATE_RAW_OK, DSTATE_READY,
    DSTATE_SCANNING, jpeg_upsampler_impl,
};
use crate::jpeglib::{
    JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPEG_ROW_COMPLETED, JPEG_SUSPENDED, JSAMPROWS,
    JSAMPROWS_MUT, RowListMut, call_progress_monitor, j_decompress_ptr, jpeg_decompress_struct,
};
use crate::jutils::jdiv_round_up;

// upstream: src/jdapistd.c jpeg_start_decompress
/// Decompression initialization. `jpeg_read_header` must be completed before
/// calling this.
///
/// If a multipass operating mode was selected, this will do all but the
/// last pass, and thus may take a great deal of time.
///
/// Returns `false` if suspended. The return value need be inspected only if
/// a suspending data source is used.
///
/// # Errors
///
/// `JERR_BAD_STATE` at the wrong time, and what master selection, the
/// input controller and the modules report.
///
/// # Panics
///
/// When a module it calls is absent.
#[expect(non_snake_case, reason = "C names")]
pub fn jpeg_start_decompress(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    if cinfo.global_state == DSTATE_READY {
        /* First call: initialize master control, select active modules */
        jinit_master_decompress(cinfo)?;
        if cinfo.buffered_image {
            /* No more work here; expecting jpeg_start_output next */
            cinfo.global_state = DSTATE_BUFIMAGE;
            return Ok(true);
        }
        cinfo.global_state = DSTATE_PRELOAD;
    }
    if cinfo.global_state == DSTATE_PRELOAD {
        /* If file has multiple scans, absorb them all into the coef buffer */
        if cinfo.inputctl().pub_.has_multiple_scans {
            if !D_MULTISCAN_FILES_SUPPORTED {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
            }
            loop {
                /* Call progress monitor hook if present */
                if cinfo.progress.is_some() {
                    call_progress_monitor(cinfo)?;
                }
                /* Absorb some more input */
                let consume_input = cinfo.inputctl().pub_.consume_input;
                let retcode = consume_input(cinfo)?;
                if retcode == JPEG_SUSPENDED {
                    return Ok(false);
                }
                if retcode == JPEG_REACHED_EOI {
                    break;
                }
                /* Advance progress counter if appropriate */
                let total_iMCU_rows = cinfo.total_iMCU_rows;
                if let Some(progress) = cinfo.progress.as_deref_mut()
                    && (retcode == JPEG_ROW_COMPLETED || retcode == JPEG_REACHED_SOS)
                {
                    progress.pass_counter += 1;
                    if progress.pass_counter >= progress.pass_limit {
                        /* jdmaster underestimated number of scans; ratchet up one scan */
                        progress.pass_limit += core::ffi::c_long::from(total_iMCU_rows);
                    }
                }
            }
        }
        cinfo.output_scan_number = cinfo.input_scan_number;
    } else if cinfo.global_state != DSTATE_PRESCAN {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }
    /* Perform any dummy output passes, and set up for the final pass */
    output_pass_setup(cinfo)
}

// upstream: src/jdapistd.c output_pass_setup
/// Sets up for an output pass, and performs any dummy pass(es) needed.
/// Common subroutine for `jpeg_start_decompress` and `jpeg_start_output`.
/// Entry: `global_state = DSTATE_PRESCAN` only if previously suspended.
/// Exit: if done, returns `true` and sets `global_state` for proper output
/// mode. If suspended, returns `false` and sets
/// `global_state = DSTATE_PRESCAN`.
///
/// # Errors
///
/// `JERR_NOT_COMPILED` for a dummy pass (see the module documentation), and
/// what `prepare_for_output_pass` reports.
///
/// # Panics
///
/// When `cinfo.master` or its `prepare_for_output_pass` is absent.
fn output_pass_setup(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    if cinfo.global_state != DSTATE_PRESCAN {
        /* First call: do pass setup */
        let prepare_for_output_pass = cinfo
            .master()
            .pub_
            .prepare_for_output_pass
            .expect("prepare_for_output_pass is NULL");
        prepare_for_output_pass(cinfo)?;
        cinfo.output_scanline = 0;
        cinfo.global_state = DSTATE_PRESCAN;
    }
    /* Loop over any required dummy passes */
    if cinfo.master().pub_.is_dummy_pass && !QUANT_2PASS_SUPPORTED {
        /* Cranking through the dummy pass needs the 2-pass quantizer */
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
    }
    /* Ready for application to drive output pass through
     * jpeg_read_scanlines or jpeg_read_raw_data.
     */
    cinfo.global_state = if cinfo.raw_data_out {
        DSTATE_RAW_OK
    } else {
        DSTATE_SCANNING
    };
    Ok(true)
}

// upstream: src/jdapistd.c _jpeg_crop_scanline
/// Enables partial scanline decompression. Must be called after
/// `jpeg_start_decompress` and before any calls to `jpeg_read_scanlines` or
/// `jpeg_skip_scanlines`. Refer to `libjpeg.txt` for more information.
///
/// `xoffset` and `width` are C's pointers, `None` where C's is null. On
/// return `*xoffset` is aligned down to an iMCU column and `*width` is
/// widened to keep the right edge.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_NOTIMPL` for raw data output,
/// `JERR_BAD_STATE`, `JERR_BAD_CROP_SPEC`, `JERR_WIDTH_OVERFLOW`, and what
/// reinitializing the upsampler reports.
///
/// # Panics
///
/// When `cinfo.master`, `cinfo.comp_info` or the merged upsampler it adjusts
/// is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's conversions between int, long and JDIMENSION"
)]
#[expect(non_snake_case, reason = "C names")]
pub fn jpeg_crop_scanline(
    cinfo: j_decompress_ptr<'_, '_>,
    xoffset: Option<&mut JDIMENSION>,
    width: Option<&mut JDIMENSION>,
) -> Result<(), ErrorExit> {
    let mut reinit_upsampler = false;

    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    if cinfo.master().pub_.lossless || cinfo.raw_data_out {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOTIMPL));
    }

    if (cinfo.global_state != DSTATE_SCANNING && cinfo.global_state != DSTATE_BUFIMAGE)
        || cinfo.output_scanline != 0
    {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }

    let (Some(xoffset), Some(width)) = (xoffset, width) else {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_CROP_SPEC));
    };

    /* xoffset and width must fall within the output image dimensions. */
    if *width == 0 || u64::from(*xoffset) + u64::from(*width) > u64::from(cinfo.output_width) {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW));
    }

    /* No need to do anything if the caller wants the entire width. */
    if *width == cinfo.output_width {
        return Ok(());
    }

    /* Ensuring the proper alignment of xoffset is tricky.  At minimum, it
     * must align with an MCU boundary, because:
     *
     *   (1) The IDCT is performed in blocks, and it is not feasible to modify
     *       the algorithm so that it can transform partial blocks.
     *   (2) Because of the SIMD extensions, any input buffer passed to the
     *       upsampling and color conversion routines must be aligned to the
     *       SIMD word size (for instance, 128-bit in the case of SSE2.)  The
     *       easiest way to accomplish this without copying data is to ensure
     *       that upsampling and color conversion begin at the start of the
     *       first MCU column that will be inverse transformed.
     *
     * In practice, we actually impose a stricter alignment requirement.  We
     * require that xoffset be a multiple of the maximum MCU column width of all
     * of the components (the "iMCU column width.")  This is to simplify the
     * single-pass decompression case, allowing us to use the same MCU column
     * width for all of the components.
     */
    let align: i32 = if cinfo.comps_in_scan == 1 && cinfo.num_components == 1 {
        cinfo.min_DCT_scaled_size
    } else {
        cinfo.min_DCT_scaled_size * cinfo.max_h_samp_factor
    };

    /* Adjust xoffset to the nearest iMCU boundary <= the requested value */
    let input_xoffset = *xoffset;
    *xoffset = (input_xoffset / align as JDIMENSION) * align as JDIMENSION;

    /* Adjust the width so that the right edge of the output image is as
     * requested (only the left edge is altered.)  It is important that calling
     * programs check this value after this function returns, so that they can
     * allocate an output buffer with the appropriate size.
     */
    *width = *width + input_xoffset - *xoffset;
    cinfo.output_width = *width;
    if UPSAMPLE_MERGING_SUPPORTED
        && cinfo.master().using_merged_upsample
        && cinfo.max_v_samp_factor == 2
    {
        let out_row_width = cinfo.output_width * cinfo.out_color_components as JDIMENSION;
        merged_upsampler_mut(cinfo).out_row_width = out_row_width;
    }

    /* Set the first and last iMCU columns that we must decompress.  These values
     * will be used in single-scan decompressions.
     */
    let (output_width, using_merged_upsample) =
        (cinfo.output_width, cinfo.master().using_merged_upsample);
    {
        let master = &mut cinfo.master_mut().pub_;
        master.first_iMCU_col = (i64::from(*xoffset) / i64::from(align)) as JDIMENSION;
        master.last_iMCU_col = (jdiv_round_up(i64::from(*xoffset + output_width), i64::from(align))
            as JDIMENSION)
            .wrapping_sub(1);
    }

    let (comps_in_scan, num_components, max_h_samp_factor, min_DCT_scaled_size) = (
        cinfo.comps_in_scan,
        cinfo.num_components,
        cinfo.max_h_samp_factor,
        cinfo.min_DCT_scaled_size,
    );
    for ci in 0..num_components as usize {
        let compptr = &mut cinfo.comp_info_mut()[ci];
        let hsf = if comps_in_scan == 1 && num_components == 1 {
            1
        } else {
            compptr.h_samp_factor
        };

        /* Set downsampled_width to the new output width. */
        let orig_downsampled_width = compptr.downsampled_width as i32;
        compptr.downsampled_width = jdiv_round_up(
            i64::from(output_width) * i64::from(compptr.h_samp_factor * compptr.DCT_scaled_size),
            i64::from(max_h_samp_factor * min_DCT_scaled_size),
        ) as JDIMENSION;
        if compptr.downsampled_width < 2
            && !(UPSAMPLE_MERGING_SUPPORTED && using_merged_upsample)
            && orig_downsampled_width >= 2
        {
            reinit_upsampler = true;
        }

        /* Set the first and last iMCU columns that we must decompress.  These
         * values will be used in multi-scan decompressions.
         */
        let master = &mut cinfo.master_mut().pub_;
        master.first_MCU_col[ci] =
            (i64::from(xoffset.wrapping_mul(hsf as JDIMENSION)) / i64::from(align)) as JDIMENSION;
        master.last_MCU_col[ci] = (jdiv_round_up(
            i64::from((*xoffset + output_width).wrapping_mul(hsf as JDIMENSION)),
            i64::from(align),
        ) as JDIMENSION)
            .wrapping_sub(1);
    }

    if reinit_upsampler {
        cinfo.master_mut().pub_.jinit_upsampler_no_alloc = true;
        jinit_upsampler(cinfo)?;
        cinfo.master_mut().pub_.jinit_upsampler_no_alloc = false;
    }
    Ok(())
}

// upstream: src/jdapistd.c _jpeg_read_scanlines
/// Reads some scanlines of data from the JPEG decompressor.
///
/// The return value will be the number of lines actually read. This may be
/// less than the number requested in several cases, including bottom of
/// image, data source suspension, and operating modes that emit multiple
/// scanlines at a time.
///
/// Note: we warn about excess calls to `jpeg_read_scanlines` since this
/// likely signals an application programmer error. However, an oversize
/// buffer (`max_lines` > scanlines remaining) is not an error.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_BAD_STATE`, what the warning's
/// `emit_message` and the progress monitor return, and what the main
/// controller reports.
///
/// # Panics
///
/// When `cinfo.main` is absent.
pub fn jpeg_read_scanlines(
    cinfo: j_decompress_ptr<'_, '_>,
    scanlines: JSAMPROWS_MUT<'_, '_>,
    max_lines: JDIMENSION,
) -> Result<JDIMENSION, ErrorExit> {
    /* D_LOSSLESS_SUPPORTED is treated as undefined: only the lossy check */
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    if cinfo.global_state != DSTATE_SCANNING {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }
    if cinfo.output_scanline >= cinfo.output_height {
        WARNMS(cinfo, J_MESSAGE_CODE::JWRN_TOO_MUCH_DATA)?;
        return Ok(0);
    }

    /* Call progress monitor hook if present */
    report_progress(cinfo)?;

    /* Process some data */
    let mut row_ctr: JDIMENSION = 0;
    let Some(process_data) = cinfo.main().process_data else {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    };
    process_data(cinfo, scanlines, &mut row_ctr, max_lines)?;
    cinfo.output_scanline += row_ctr;
    Ok(row_ctr)
}

/// The progress monitor call of `jpeg_read_scanlines` and
/// `jpeg_read_raw_data`: `pass_counter` and `pass_limit` from the output
/// position, then the hook. Not an upstream item.
fn report_progress(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    let (output_scanline, output_height) = (cinfo.output_scanline, cinfo.output_height);
    if let Some(progress) = cinfo.progress.as_deref_mut() {
        progress.pass_counter = core::ffi::c_long::from(output_scanline);
        progress.pass_limit = core::ffi::c_long::from(output_height);
        call_progress_monitor(cinfo)?;
    }
    Ok(())
}

// upstream: src/jdapistd.c noop_convert
/// Dummy color convert function used by `jpeg_skip_scanlines`.
fn noop_convert(
    _cinfo: &jpeg_decompress_struct<'_>,
    _input_buf: &[JSAMPROWS<'_>],
    _input_row: JDIMENSION,
    _output_buf: JSAMPROWS_MUT<'_, '_>,
    _num_rows: i32,
) {
}

// upstream: src/jdapistd.c read_and_discard_scanlines
/// In some cases, it is best to call `jpeg_read_scanlines` and discard the
/// output, rather than skipping the scanlines, because this allows us to
/// maintain the internal state of the context-based upsampler. In these
/// cases, we set up and tear down a dummy color converter in order to avoid
/// valgrind errors and to achieve the best possible performance.
///
/// C reads into the merged upsampler's own spare row; the port reads into a
/// row of the same length, which leaves the spare row as C leaves it (ADR
/// 0005, refinements from work package 6).
///
/// # Errors
///
/// What `jpeg_read_scanlines` reports. As in C, an error leaves the dummy
/// color converter in place.
///
/// # Panics
///
/// When `cinfo.master` or the merged upsampler's spare row is absent.
fn read_and_discard_scanlines(
    cinfo: j_decompress_ptr<'_, '_>,
    num_lines: JDIMENSION,
) -> Result<(), ErrorExit> {
    let using_merged_upsample = UPSAMPLE_MERGING_SUPPORTED && cinfo.master().using_merged_upsample;
    let mut dummy_sample: [JSAMPLE; 1] = [0];
    let mut dummy_row = false; /* scanlines = &dummy_row */
    let mut color_convert = None;

    if let Some(cconvert) = cinfo.cconvert.as_deref_mut()
        && !using_merged_upsample
        && cconvert.pub_.color_convert.is_some()
    {
        color_convert = cconvert.pub_.color_convert;
        cconvert.pub_.color_convert = Some(noop_convert);
        /* This just prevents UBSan from complaining about adding 0 to a NULL
         * pointer.  The pointer isn't actually used.
         */
        dummy_row = true;
    }

    /* cinfo->quantize_colors needs the color quantizer, which is absent */

    let mut spare_row: Option<Vec<JSAMPLE>> = None; /* scanlines = &upsample->spare_row */
    if using_merged_upsample && cinfo.max_v_samp_factor == 2 {
        let len = merged_upsampler_mut(cinfo)
            .spare_row
            .as_ref()
            .expect("spare_row is NULL")
            .len();
        spare_row = Some(vec![0; len]);
    }

    for _ in 0..num_lines {
        if let Some(row) = spare_row.as_mut() {
            jpeg_read_scanlines(cinfo, RowListMut::Separate(&mut [row.as_mut_slice()]), 1)?;
        } else if dummy_row {
            jpeg_read_scanlines(cinfo, RowListMut::Separate(&mut [&mut dummy_sample[..]]), 1)?;
        } else {
            /* C's NULL row list */
            jpeg_read_scanlines(cinfo, RowListMut::Separate(&mut []), 1)?;
        }
    }

    if color_convert.is_some() {
        cinfo
            .cconvert
            .as_deref_mut()
            .expect("checked above")
            .pub_
            .color_convert = color_convert;
    }
    Ok(())
}

/// `(my_upsample_ptr)cinfo->upsample`. Not an upstream item.
///
/// # Panics
///
/// When the slot is empty or holds the merged upsampler, where C's cast
/// would be undefined.
fn upsampler_mut<'a>(cinfo: &'a mut jpeg_decompress_struct<'_>) -> &'a mut my_upsampler {
    match cinfo
        .upsample
        .as_deref_mut()
        .expect("cinfo->upsample is NULL")
    {
        jpeg_upsampler_impl::my_upsampler(upsample) => upsample,
        jpeg_upsampler_impl::my_merged_upsampler(_) => unreachable!("not my_upsampler"),
    }
}

/// `(my_merged_upsample_ptr)cinfo->upsample`. Not an upstream item.
///
/// # Panics
///
/// When the slot is empty or holds the separate upsampler, where C's cast
/// would be undefined.
fn merged_upsampler_mut<'a>(
    cinfo: &'a mut jpeg_decompress_struct<'_>,
) -> &'a mut my_merged_upsampler {
    match cinfo
        .upsample
        .as_deref_mut()
        .expect("cinfo->upsample is NULL")
    {
        jpeg_upsampler_impl::my_merged_upsampler(upsample) => upsample,
        jpeg_upsampler_impl::my_upsampler(_) => unreachable!("not my_merged_upsampler"),
    }
}

/// `(my_main_ptr)cinfo->main`. Not an upstream item.
///
/// # Panics
///
/// When the slot is empty.
fn main_mut<'a>(cinfo: &'a mut jpeg_decompress_struct<'_>) -> &'a mut my_main_controller {
    cinfo.main.as_deref_mut().expect("cinfo->main is NULL")
}

/// `upsample->rows_to_go = cinfo->output_height - cinfo->output_scanline` for
/// whichever upsampler is in use. Not an upstream item.
fn reset_rows_to_go(cinfo: j_decompress_ptr<'_, '_>) {
    let rows_to_go = cinfo.output_height - cinfo.output_scanline;
    if UPSAMPLE_MERGING_SUPPORTED && cinfo.master().using_merged_upsample {
        merged_upsampler_mut(cinfo).rows_to_go = rows_to_go;
    } else {
        upsampler_mut(cinfo).rows_to_go = rows_to_go;
    }
}

// upstream: src/jdapistd.c increment_simple_rowgroup_ctr
/// Called by `jpeg_skip_scanlines`. This partially skips a decompress block
/// by incrementing the rowgroup counter.
///
/// # Errors
///
/// What `read_and_discard_scanlines` reports.
#[expect(clippy::cast_sign_loss, reason = "C's conversions of int factors")]
fn increment_simple_rowgroup_ctr(
    cinfo: j_decompress_ptr<'_, '_>,
    rows: JDIMENSION,
) -> Result<(), ErrorExit> {
    if !cinfo.upsample().need_context_rows && cinfo.max_v_samp_factor != 1 {
        return read_and_discard_scanlines(cinfo, rows);
    }

    /* Increment the counter to the next row group after the skipped rows. */
    let max_v_samp_factor = cinfo.max_v_samp_factor as JDIMENSION;
    main_mut(cinfo).rowgroup_ctr += rows / max_v_samp_factor;

    /* Partially skipping a row group would involve modifying the internal state
     * of the upsampler, so read the remaining rows into a dummy buffer instead.
     */
    let rows_left = rows % max_v_samp_factor;
    cinfo.output_scanline += rows - rows_left;

    read_and_discard_scanlines(cinfo, rows_left)
}

// upstream: src/jdapistd.c _jpeg_skip_scanlines
/// Skips some scanlines of data from the JPEG decompressor.
///
/// The return value will be the number of lines actually skipped. If
/// skipping `num_lines` would move beyond the end of the image, then the
/// actual number of lines remaining in the image is returned. Otherwise, the
/// return value will be equal to `num_lines`.
///
/// Refer to `libjpeg.txt` for more information.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_NOTIMPL`, `JERR_BAD_STATE`, and what reading
/// and decoding the skipped data report.
///
/// # Panics
///
/// When a module it adjusts is absent.
#[expect(
    clippy::cast_sign_loss,
    reason = "C's conversions of int factors to JDIMENSION"
)]
#[expect(non_snake_case, reason = "C names")]
#[expect(clippy::too_many_lines, reason = "C's function")]
pub fn jpeg_skip_scanlines(
    cinfo: j_decompress_ptr<'_, '_>,
    mut num_lines: JDIMENSION,
) -> Result<JDIMENSION, ErrorExit> {
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    if cinfo.master().pub_.lossless {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOTIMPL));
    }

    /* Two-pass color quantization is not supported. */
    if cinfo.quantize_colors && cinfo.two_pass_quantize {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOTIMPL));
    }

    if cinfo.global_state != DSTATE_SCANNING {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }

    /* Do not skip past the bottom of the image. */
    if u64::from(cinfo.output_scanline) + u64::from(num_lines) >= u64::from(cinfo.output_height) {
        num_lines = cinfo.output_height - cinfo.output_scanline;
        cinfo.output_scanline = cinfo.output_height;
        let finish_input_pass = cinfo.inputctl().pub_.finish_input_pass;
        finish_input_pass(cinfo);
        cinfo.inputctl_mut().pub_.eoi_reached = true;
        return Ok(num_lines);
    }

    if num_lines == 0 {
        return Ok(0);
    }

    let lines_per_iMCU_row = (cinfo.min_DCT_scaled_size * cinfo.max_v_samp_factor) as JDIMENSION;
    let lines_left_in_iMCU_row =
        (lines_per_iMCU_row - (cinfo.output_scanline % lines_per_iMCU_row)) % lines_per_iMCU_row;
    /* C's unsigned subtraction wraps when fewer lines are skipped than are
     * left in the iMCU row; the value is unused then.
     */
    let mut lines_after_iMCU_row = num_lines.wrapping_sub(lines_left_in_iMCU_row);

    /* Skip the lines remaining in the current iMCU row.  When upsampling
     * requires context rows, we need the previous and next rows in order to read
     * the current row.  This adds some complexity.
     */
    if cinfo.upsample().need_context_rows {
        /* If the skipped lines would not move us past the current iMCU row, we
         * read the lines and ignore them.  There might be a faster way of doing
         * this, but we are facing increasing complexity for diminishing returns.
         * The increasing complexity would be a by-product of meddling with the
         * state machine used to skip context rows.  Near the end of an iMCU row,
         * the next iMCU row may have already been entropy-decoded.  In this unique
         * case, we will read the next iMCU row if we cannot skip past it as well.
         */
        let buffer_full = main_mut(cinfo).buffer_full;
        if (num_lines < lines_left_in_iMCU_row + 1)
            || (lines_left_in_iMCU_row <= 1
                && buffer_full
                && lines_after_iMCU_row < lines_per_iMCU_row + 1)
        {
            read_and_discard_scanlines(cinfo, num_lines)?;
            return Ok(num_lines);
        }

        /* If the next iMCU row has already been entropy-decoded, make sure that
         * we do not skip too far.
         */
        if lines_left_in_iMCU_row <= 1 && buffer_full {
            cinfo.output_scanline += lines_left_in_iMCU_row + lines_per_iMCU_row;
            lines_after_iMCU_row -= lines_per_iMCU_row;
        } else {
            cinfo.output_scanline += lines_left_in_iMCU_row;
        }

        /* If we have just completed the first block, adjust the buffer pointers */
        let iMCU_row_ctr = main_mut(cinfo).iMCU_row_ctr;
        if iMCU_row_ctr == 0 || (iMCU_row_ctr == 1 && lines_left_in_iMCU_row > 2) {
            let mut main_ptr = cinfo.main.take().expect("cinfo->main is NULL");
            set_wraparound_pointers(cinfo, &mut main_ptr);
            cinfo.main = Some(main_ptr);
        }
        let main_ptr = main_mut(cinfo);
        main_ptr.buffer_full = false;
        main_ptr.rowgroup_ctr = 0;
        main_ptr.context_state = CTX_PREPARE_FOR_IMCU;
        if !cinfo.master().using_merged_upsample {
            let (max_v_samp_factor, rows_to_go) = (
                cinfo.max_v_samp_factor,
                cinfo.output_height - cinfo.output_scanline,
            );
            let upsample = upsampler_mut(cinfo);
            upsample.next_row_out = max_v_samp_factor;
            upsample.rows_to_go = rows_to_go;
        }
    }
    /* Skipping is much simpler when context rows are not required. */
    else if num_lines < lines_left_in_iMCU_row {
        increment_simple_rowgroup_ctr(cinfo, num_lines)?;
        return Ok(num_lines);
    } else {
        cinfo.output_scanline += lines_left_in_iMCU_row;
        let main_ptr = main_mut(cinfo);
        main_ptr.buffer_full = false;
        main_ptr.rowgroup_ctr = 0;
        if UPSAMPLE_MERGING_SUPPORTED && cinfo.master().using_merged_upsample {
            merged_upsampler_mut(cinfo).spare_full = false;
        } else {
            let (max_v_samp_factor, rows_to_go) = (
                cinfo.max_v_samp_factor,
                cinfo.output_height - cinfo.output_scanline,
            );
            let upsample = upsampler_mut(cinfo);
            upsample.next_row_out = max_v_samp_factor;
            upsample.rows_to_go = rows_to_go;
        }
    }

    /* Calculate how many full iMCU rows we can skip. */
    let lines_to_skip = if cinfo.upsample().need_context_rows {
        ((lines_after_iMCU_row - 1) / lines_per_iMCU_row) * lines_per_iMCU_row
    } else {
        (lines_after_iMCU_row / lines_per_iMCU_row) * lines_per_iMCU_row
    };
    /* Calculate the number of lines that remain to be skipped after skipping all
     * of the full iMCU rows that we can.  We will not read these lines unless we
     * have to.
     */
    let lines_to_read = lines_after_iMCU_row - lines_to_skip;

    /* For images requiring multiple scans (progressive, non-interleaved, etc.),
     * all of the entropy decoding occurs in jpeg_start_decompress(), assuming
     * that the input data source is non-suspending.  This makes skipping easy.
     */
    if cinfo.inputctl().pub_.has_multiple_scans || cinfo.buffered_image {
        if cinfo.upsample().need_context_rows {
            cinfo.output_scanline += lines_to_skip;
            cinfo.output_iMCU_row += lines_to_skip / lines_per_iMCU_row;
            main_mut(cinfo).iMCU_row_ctr += lines_to_skip / lines_per_iMCU_row;
            /* It is complex to properly move to the middle of a context block, so
             * read the remaining lines instead of skipping them.
             */
            read_and_discard_scanlines(cinfo, lines_to_read)?;
        } else {
            cinfo.output_scanline += lines_to_skip;
            cinfo.output_iMCU_row += lines_to_skip / lines_per_iMCU_row;
            increment_simple_rowgroup_ctr(cinfo, lines_to_read)?;
        }
        reset_rows_to_go(cinfo);
        return Ok(num_lines);
    }

    /* Skip the iMCU rows that we can safely skip. */
    let mut i: JDIMENSION = 0;
    while i < lines_to_skip {
        let mut y = 0;
        while y < cinfo
            .coef
            .as_deref()
            .expect("cinfo->coef is NULL")
            .MCU_rows_per_iMCU_row
        {
            for _x in 0..cinfo.MCUs_per_row {
                /* Calling decode_mcu() with a NULL pointer causes it to discard the
                 * decoded coefficients.  This is ~5% faster for large subsets, but
                 * it's tough to tell a difference for smaller images.
                 */
                if !cinfo.entropy().insufficient_data {
                    let input_iMCU_row = cinfo.input_iMCU_row;
                    cinfo.master_mut().pub_.last_good_iMCU_row = input_iMCU_row;
                }
                let decode_mcu = cinfo.entropy().decode_mcu.expect("decode_mcu is NULL");
                /* C ignores the result, including a suspension */
                decode_mcu(cinfo, None)?;
            }
            y += 1;
        }
        cinfo.input_iMCU_row += 1;
        cinfo.output_iMCU_row += 1;
        if cinfo.input_iMCU_row < cinfo.total_iMCU_rows {
            let mut coef: Box<my_coef_controller> = cinfo.coef.take().expect("cinfo->coef is NULL");
            start_iMCU_row(cinfo, &mut coef);
            cinfo.coef = Some(coef);
        } else {
            let finish_input_pass = cinfo.inputctl().pub_.finish_input_pass;
            finish_input_pass(cinfo);
        }
        i += lines_per_iMCU_row;
    }
    cinfo.output_scanline += lines_to_skip;

    if cinfo.upsample().need_context_rows {
        /* Context-based upsampling keeps track of iMCU rows. */
        main_mut(cinfo).iMCU_row_ctr += lines_to_skip / lines_per_iMCU_row;

        /* It is complex to properly move to the middle of a context block, so
         * read the remaining lines instead of skipping them.
         */
        read_and_discard_scanlines(cinfo, lines_to_read)?;
    } else {
        increment_simple_rowgroup_ctr(cinfo, lines_to_read)?;
    }

    /* Since skipping lines involves skipping the upsampling step, the value of
     * "rows_to_go" will become invalid unless we set it here.  NOTE: This is a
     * bit odd, since "rows_to_go" seems to be redundantly keeping track of
     * output_scanline.
     */
    reset_rows_to_go(cinfo);

    /* Always skip the requested number of lines. */
    Ok(num_lines)
}

// upstream: src/jdapistd.c _jpeg_read_raw_data
/// Alternate entry point to read raw data. Processes exactly one iMCU row
/// per call, unless suspended. `data` holds a row list per component (C's
/// `JSAMPIMAGE`).
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_NOTIMPL`, `JERR_BAD_STATE`,
/// `JERR_BUFFER_SIZE`, what the warning's `emit_message` and the progress
/// monitor return, and what the coefficient controller reports.
///
/// # Panics
///
/// When `cinfo.coef` is absent.
#[expect(
    clippy::cast_sign_loss,
    reason = "C's conversion of an int product to JDIMENSION"
)]
#[expect(non_snake_case, reason = "C names")]
pub fn jpeg_read_raw_data(
    cinfo: j_decompress_ptr<'_, '_>,
    data: &mut [JSAMPROWS_MUT<'_, '_>],
    max_lines: JDIMENSION,
) -> Result<JDIMENSION, ErrorExit> {
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    if cinfo.master().pub_.lossless {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOTIMPL));
    }

    if cinfo.global_state != DSTATE_RAW_OK {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }
    if cinfo.output_scanline >= cinfo.output_height {
        WARNMS(cinfo, J_MESSAGE_CODE::JWRN_TOO_MUCH_DATA)?;
        return Ok(0);
    }

    /* Call progress monitor hook if present */
    report_progress(cinfo)?;

    /* Verify that at least one iMCU row can be returned. */
    let lines_per_iMCU_row = (cinfo.max_v_samp_factor * cinfo.min_DCT_scaled_size) as JDIMENSION;
    if max_lines < lines_per_iMCU_row {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BUFFER_SIZE));
    }

    /* Decompress directly into user's buffer: the port's decompress_data is
     * never null.
     */
    let decompress_data = cinfo.coef().decompress_data;
    if decompress_data(cinfo, data)? == JPEG_SUSPENDED {
        return Ok(0); /* suspension forced, can do nothing more */
    }

    /* OK, we processed one iMCU row. */
    cinfo.output_scanline += lines_per_iMCU_row;
    Ok(lines_per_iMCU_row)
}

/* Additional entry points for buffered-image mode. */

// upstream: src/jdapistd.c jpeg_start_output
/// Initializes for an output pass in buffered-image mode.
///
/// # Errors
///
/// `JERR_BAD_STATE` at the wrong time, and what `output_pass_setup`
/// reports.
///
/// # Panics
///
/// When `cinfo.inputctl` or `cinfo.master` is absent.
pub fn jpeg_start_output(
    cinfo: j_decompress_ptr<'_, '_>,
    mut scan_number: i32,
) -> Result<bool, ErrorExit> {
    if cinfo.global_state != DSTATE_BUFIMAGE && cinfo.global_state != DSTATE_PRESCAN {
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }
    /* Limit scan number to valid range */
    if scan_number <= 0 {
        scan_number = 1;
    }
    if cinfo.inputctl().pub_.eoi_reached && scan_number > cinfo.input_scan_number {
        scan_number = cinfo.input_scan_number;
    }
    cinfo.output_scan_number = scan_number;
    /* Perform any dummy output passes, and set up for the real pass */
    output_pass_setup(cinfo)
}

// upstream: src/jdapistd.c jpeg_finish_output
/// Finishes up after an output pass in buffered-image mode.
///
/// Returns `false` if suspended. The return value need be inspected only if
/// a suspending data source is used.
///
/// # Errors
///
/// `JERR_BAD_STATE` at the wrong time, and what `finish_output_pass` and
/// the input controller report.
///
/// # Panics
///
/// When `cinfo.master` or `cinfo.inputctl` is absent.
pub fn jpeg_finish_output(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    if (cinfo.global_state == DSTATE_SCANNING || cinfo.global_state == DSTATE_RAW_OK)
        && cinfo.buffered_image
    {
        /* Terminate this pass. */
        /* We do not require the whole pass to have been completed. */
        let finish_output_pass = cinfo
            .master()
            .pub_
            .finish_output_pass
            .expect("finish_output_pass is NULL");
        finish_output_pass(cinfo)?;
        cinfo.global_state = DSTATE_BUFPOST;
    } else if cinfo.global_state != DSTATE_BUFPOST {
        /* BUFPOST = repeat call after a suspension, anything else is error */
        let global_state = cinfo.global_state;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            global_state,
        ));
    }
    /* Read markers looking for SOS or EOI */
    while cinfo.input_scan_number <= cinfo.output_scan_number && !cinfo.inputctl().pub_.eoi_reached
    {
        let consume_input = cinfo.inputctl().pub_.consume_input;
        if consume_input(cinfo)? == JPEG_SUSPENDED {
            return Ok(false); /* Suspend, come back later */
        }
    }
    cinfo.global_state = DSTATE_BUFIMAGE;
    Ok(true)
}
