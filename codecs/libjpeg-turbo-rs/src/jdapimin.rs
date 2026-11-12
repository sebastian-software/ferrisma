// libjpeg-turbo 3.2.1, src/jdapimin.c:
//
// /*
//  * jdapimin.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1998, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2016, 2022, 2024-2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains application interface code for the decompression half
//  * of the JPEG library.  These are the "minimum" API routines that may be
//  * needed in either the normal full-decompression case or the
//  * transcoding-only case.
//  *
//  * Most of the routines intended to be called directly by an application
//  * are in this file or in jdapistd.c.  But also see jcomapi.c for routines
//  * shared by compression and decompression, and jdtrans.c for the transcoding
//  * case.
//  */
//! Translation of `src/jdapimin.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
//!
//! `jpeg_finish_decompress` calls the master control module's
//! `finish_output_pass`, so work package 6 adds it with `jdmaster.c` (ADR
//! 0005, decisions of 2026-11-07). With `WITH_SIMD` the master marks the
//! SIMD support as undetected (ADR 0007); `WITH_SIMDE` is undefined.
//! `D_LOSSLESS_SUPPORTED` and `QUANT_2PASS_SUPPORTED` are treated as
//! undefined (ADR 0005).

use crate::jcomapi::{jpeg_abort, jpeg_destroy};
use crate::jconfig::{BITS_IN_JSAMPLE, JPEG_LIB_VERSION};
use crate::jdinput::jinit_input_controller;
use crate::jdmarker::jinit_marker_reader;
use crate::jdmaster::{SIZEOF_my_decomp_master, my_decomp_master};
use crate::jerror::{ERREXIT, ERREXIT1, ERREXIT2, ErrorExit, J_MESSAGE_CODE, TRACEMS3, WARNMS1};
use crate::jmemmgr::{alloc_small_box, jinit_memory_mgr};
use crate::jpegint::{
    DSTATE_BUFIMAGE, DSTATE_BUFPOST, DSTATE_INHEADER, DSTATE_PRELOAD, DSTATE_PRESCAN,
    DSTATE_RAW_OK, DSTATE_READY, DSTATE_SCANNING, DSTATE_START, DSTATE_STOPPING,
};
use crate::jpeglib::{
    J_COLOR_SPACE, J_DITHER_MODE, JDCT_DEFAULT, JPEG_HEADER_OK, JPEG_HEADER_TABLES_ONLY,
    JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPEG_SUSPENDED, JPOOL_PERMANENT, NUM_HUFF_TBLS,
    NUM_QUANT_TBLS, SIZEOF_jpeg_decompress_struct, call_init_source, call_term_source,
    j_decompress_ptr, jpeg_decompress_struct, jpeg_error_mgr,
};

// upstream: src/jdapimin.c jpeg_CreateDecompress
/// Initializes a JPEG decompression object. The error manager must already
/// be set up, in case the memory manager fails. Every other field is reset,
/// as C's `memset` does (the port keeps no `client_data`).
///
/// # Errors
///
/// `JERR_BAD_LIB_VERSION` or `JERR_BAD_STRUCT_SIZE` for a caller built
/// against another library, or what the memory manager reports.
#[expect(non_snake_case, reason = "C function name")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "the C casts of the struct sizes to int"
)]
#[allow(clippy::needless_range_loop, reason = "the C index loops")]
pub fn jpeg_CreateDecompress(
    cinfo: j_decompress_ptr<'_, '_>,
    version: i32,
    structsize: usize,
) -> Result<(), ErrorExit> {
    /* Guard against version mismatches between library and caller. */
    cinfo.mem = None; /* so jpeg_destroy knows mem mgr not called */
    if version != JPEG_LIB_VERSION {
        return Err(ERREXIT2(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_LIB_VERSION,
            JPEG_LIB_VERSION,
            version,
        ));
    }
    if structsize != SIZEOF_jpeg_decompress_struct {
        return Err(ERREXIT2(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STRUCT_SIZE,
            SIZEOF_jpeg_decompress_struct as i32,
            structsize as i32,
        ));
    }

    /* For debugging purposes, we zero the whole master structure.
     * But the application has already set the err pointer, and may have set
     * client_data, so we have to save and restore those fields.
     */
    {
        let err = core::mem::replace(&mut cinfo.err, Box::new(jpeg_error_mgr::default()));
        *cinfo = jpeg_decompress_struct::new(err);
    }
    cinfo.is_decompressor = true;

    /* Initialize a memory manager instance for this object */
    jinit_memory_mgr(cinfo)?;

    /* Zero out pointers to permanent structures. */
    cinfo.progress = None;
    cinfo.src = None;

    for i in 0..NUM_QUANT_TBLS {
        cinfo.quant_tbl_ptrs[i] = None;
    }

    for i in 0..NUM_HUFF_TBLS {
        cinfo.dc_huff_tbl_ptrs[i] = None;
        cinfo.ac_huff_tbl_ptrs[i] = None;
    }

    /* Initialize marker processor so application can override methods
     * for COM, APPn markers before calling jpeg_read_header.
     */
    cinfo.marker_list = None;
    jinit_marker_reader(cinfo)?;

    /* And initialize the overall input controller. */
    jinit_input_controller(cinfo)?;

    cinfo.data_precision = BITS_IN_JSAMPLE;

    /* OK, I'm ready */
    cinfo.global_state = DSTATE_START;

    /* The master struct is used to store extension parameters, so we allocate it
     * here.
     */
    cinfo.master = Some(alloc_small_box(
        cinfo,
        JPOOL_PERMANENT,
        SIZEOF_my_decomp_master,
        my_decomp_master::default(),
    )?);
    #[cfg(with_simd)]
    {
        cinfo.master_mut().pub_.simd_support = crate::jsimd::JSIMD_UNDEFINED;
        /* WITH_SIMDE is undefined */
        cinfo.master_mut().pub_.simd_huffman = 1;
    }
    Ok(())
}

// upstream: src/jpeglib.h jpeg_create_decompress
/// `jpeg_CreateDecompress` with this library's version and struct size.
///
/// # Errors
///
/// What `jpeg_CreateDecompress` reports.
pub fn jpeg_create_decompress(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    jpeg_CreateDecompress(cinfo, JPEG_LIB_VERSION, SIZEOF_jpeg_decompress_struct)
}

// upstream: src/jdapimin.c jpeg_destroy_decompress
/// Destruction of a JPEG decompression object.
///
/// # Errors
///
/// What `jpeg_destroy` reports; nothing in practice.
pub fn jpeg_destroy_decompress(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    jpeg_destroy(cinfo) /* use common routine */
}

// upstream: src/jdapimin.c jpeg_abort_decompress
/// Aborts processing of a JPEG decompression operation, but doesn't destroy
/// the object itself.
///
/// # Errors
///
/// What `jpeg_abort` reports; nothing in practice.
pub fn jpeg_abort_decompress(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    jpeg_abort(cinfo) /* use common routine */
}

// upstream: src/jdapimin.c default_decompress_parms
/// Sets default decompression parameters: guesses the input colorspace from
/// the markers and component IDs, and sets the output colorspace and the
/// other parameters accordingly.
fn default_decompress_parms(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Guess the input colorspace, and set output colorspace accordingly. */
    /* (Wish JPEG committee had provided a real way to specify this...) */
    /* Note application may override our guesses. */
    match cinfo.num_components {
        1 => {
            cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_GRAYSCALE;
            cinfo.out_color_space = J_COLOR_SPACE::JCS_GRAYSCALE;
        }

        3 => {
            if cinfo.saw_JFIF_marker {
                cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_YCbCr; /* JFIF implies YCbCr */
            } else if cinfo.saw_Adobe_marker {
                match cinfo.Adobe_transform {
                    0 => cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_RGB,
                    1 => cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_YCbCr,
                    _ => {
                        WARNMS1(
                            cinfo,
                            J_MESSAGE_CODE::JWRN_ADOBE_XFORM,
                            i32::from(cinfo.Adobe_transform),
                        )?;
                        cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_YCbCr; /* assume it's YCbCr */
                    }
                }
            } else {
                /* Saw no special markers, try to guess from the component IDs */
                let comp_info = cinfo.comp_info();
                let cid0 = comp_info[0].component_id;
                let cid1 = comp_info[1].component_id;
                let cid2 = comp_info[2].component_id;

                if cid0 == 1 && cid1 == 2 && cid2 == 3 {
                    cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_YCbCr; /* assume JFIF w/out marker */
                } else if cid0 == 82 && cid1 == 71 && cid2 == 66 {
                    cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_RGB; /* ASCII 'R', 'G', 'B' */
                } else {
                    TRACEMS3(cinfo, 1, J_MESSAGE_CODE::JTRC_UNKNOWN_IDS, cid0, cid1, cid2)?;
                    cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_YCbCr; /* assume it's YCbCr */
                }
            }
            /* Always guess RGB is proper output colorspace. */
            cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
        }

        4 => {
            if cinfo.saw_Adobe_marker {
                match cinfo.Adobe_transform {
                    0 => cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_CMYK,
                    2 => cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_YCCK,
                    _ => {
                        WARNMS1(
                            cinfo,
                            J_MESSAGE_CODE::JWRN_ADOBE_XFORM,
                            i32::from(cinfo.Adobe_transform),
                        )?;
                        cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_YCCK; /* assume it's YCCK */
                    }
                }
            } else {
                /* No special markers, assume straight CMYK. */
                cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_CMYK;
            }
            cinfo.out_color_space = J_COLOR_SPACE::JCS_CMYK;
        }

        _ => {
            cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_UNKNOWN;
            cinfo.out_color_space = J_COLOR_SPACE::JCS_UNKNOWN;
        }
    }

    /* Set defaults for other decompression parameters. */
    cinfo.scale_num = 1; /* 1:1 scaling */
    cinfo.scale_denom = 1;
    cinfo.output_gamma = 1.0;
    cinfo.buffered_image = false;
    cinfo.raw_data_out = false;
    cinfo.dct_method = JDCT_DEFAULT;
    cinfo.do_fancy_upsampling = true;
    cinfo.do_block_smoothing = true;
    cinfo.quantize_colors = false;
    /* We set these in case application only sets quantize_colors. */
    cinfo.dither_mode = J_DITHER_MODE::JDITHER_FS;
    cinfo.two_pass_quantize = false;
    cinfo.desired_number_of_colors = 256;
    // cinfo->colormap = NULL: the colormap arrives with work package 6.
    /* Initialize for no mode change in buffered-image mode. */
    cinfo.enable_1pass_quant = false;
    cinfo.enable_external_quant = false;
    cinfo.enable_2pass_quant = false;
    Ok(())
}

// upstream: src/jdapimin.c jpeg_read_header
/// Decompression startup: reads the start of the JPEG datastream, as far as
/// the first SOS marker, and saves all tables and parameters in the object.
/// Returns `JPEG_HEADER_OK`, `JPEG_HEADER_TABLES_ONLY` for an abbreviated
/// (tables-only) datastream, or `JPEG_SUSPENDED`.
///
/// # Errors
///
/// `JERR_BAD_STATE`, `JERR_NO_IMAGE` when `require_image` and the datastream
/// has no image, or what the marker reader reports.
pub fn jpeg_read_header(
    cinfo: j_decompress_ptr<'_, '_>,
    require_image: bool,
) -> Result<i32, ErrorExit> {
    if cinfo.global_state != DSTATE_START && cinfo.global_state != DSTATE_INHEADER {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            cinfo.global_state,
        ));
    }

    let mut retcode = jpeg_consume_input(cinfo)?;

    match retcode {
        JPEG_REACHED_SOS => retcode = JPEG_HEADER_OK,
        JPEG_REACHED_EOI => {
            if require_image {
                /* Complain if application wanted an image */
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NO_IMAGE));
            }
            /* Reset to start state; it would be safer to require the application to
             * call jpeg_abort, but we can't change it now for compatibility reasons.
             * A side effect is to free any temporary memory (there shouldn't be any).
             */
            jpeg_abort(cinfo)?; /* sets state = DSTATE_START */
            retcode = JPEG_HEADER_TABLES_ONLY;
        }
        _ => {} /* JPEG_SUSPENDED: no work */
    }

    Ok(retcode)
}

// upstream: src/jdapimin.c jpeg_consume_input
/// Consumes data in advance of what the decompressor requires: the state
/// machine of the transitions from datastream start to header scanning to
/// ready-for-start_decompress. All the actual input is done by the input
/// controller's `consume_input`.
///
/// # Errors
///
/// `JERR_BAD_STATE`, or what the input controller or the source reports.
pub fn jpeg_consume_input(cinfo: j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit> {
    /* NB: every possible DSTATE value should be listed in this switch */
    let retcode = match cinfo.global_state {
        DSTATE_START | DSTATE_INHEADER => {
            if cinfo.global_state == DSTATE_START {
                /* Start-of-datastream actions: reset appropriate modules */
                let reset_input_controller = cinfo.inputctl().pub_.reset_input_controller;
                reset_input_controller(cinfo);
                /* Initialize application's data source module */
                call_init_source(cinfo)?;
                cinfo.global_state = DSTATE_INHEADER;
            }
            /* FALLTHROUGH */
            let consume_input = cinfo.inputctl().pub_.consume_input;
            let retcode = consume_input(cinfo)?;
            if retcode == JPEG_REACHED_SOS {
                /* Found SOS, prepare to decompress */
                /* Set up default parameters based on header data */
                default_decompress_parms(cinfo)?;
                /* Set global state: ready for start_decompress */
                cinfo.global_state = DSTATE_READY;
            }
            retcode
        }
        DSTATE_READY => {
            /* Can't advance past first SOS until start_decompress is called */
            JPEG_REACHED_SOS
        }
        DSTATE_PRELOAD | DSTATE_PRESCAN | DSTATE_SCANNING | DSTATE_RAW_OK | DSTATE_BUFIMAGE
        | DSTATE_BUFPOST | DSTATE_STOPPING => {
            let consume_input = cinfo.inputctl().pub_.consume_input;
            consume_input(cinfo)?
        }
        _ => {
            return Err(ERREXIT1(
                cinfo,
                J_MESSAGE_CODE::JERR_BAD_STATE,
                cinfo.global_state,
            ));
        }
    };
    Ok(retcode)
}

// upstream: src/jdapimin.c jpeg_input_complete
/// Have we finished reading the input file?
///
/// # Errors
///
/// `JERR_BAD_STATE` for an object that is not created or destroyed.
pub fn jpeg_input_complete(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    /* Check for valid jpeg object */
    if cinfo.global_state < DSTATE_START || cinfo.global_state > DSTATE_STOPPING {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            cinfo.global_state,
        ));
    }
    Ok(cinfo.inputctl().pub_.eoi_reached)
}

// upstream: src/jdapimin.c jpeg_has_multiple_scans
/// Is there more than one scan? Only valid after `jpeg_read_header`.
///
/// # Errors
///
/// `JERR_BAD_STATE` before `jpeg_read_header` completes.
pub fn jpeg_has_multiple_scans(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    /* Only valid after jpeg_read_header completes */
    if cinfo.global_state < DSTATE_READY || cinfo.global_state > DSTATE_STOPPING {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            cinfo.global_state,
        ));
    }
    Ok(cinfo.inputctl().pub_.has_multiple_scans)
}

// upstream: src/jdapimin.c jpeg_finish_decompress
/// Finishes JPEG decompression.
///
/// This will normally just verify the file trailer and release temp
/// storage.
///
/// Returns `false` if suspended. The return value need be inspected only if
/// a suspending data source is used.
///
/// # Errors
///
/// `JERR_TOO_LITTLE_DATA` before the last scanline is read,
/// `JERR_BAD_STATE` at the wrong time, and what `finish_output_pass`, the
/// input controller and the source manager report.
///
/// # Panics
///
/// When `cinfo.master`, `cinfo.inputctl` or the source manager is absent.
pub fn jpeg_finish_decompress(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    if (cinfo.global_state == DSTATE_SCANNING || cinfo.global_state == DSTATE_RAW_OK)
        && !cinfo.buffered_image
    {
        /* Terminate final pass of non-buffered mode */
        if cinfo.output_scanline < cinfo.output_height {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_TOO_LITTLE_DATA));
        }
        let finish_output_pass = cinfo
            .master()
            .pub_
            .finish_output_pass
            .expect("finish_output_pass is NULL");
        finish_output_pass(cinfo)?;
        cinfo.global_state = DSTATE_STOPPING;
    } else if cinfo.global_state == DSTATE_BUFIMAGE {
        /* Finishing after a buffered-image operation */
        cinfo.global_state = DSTATE_STOPPING;
    } else if cinfo.global_state != DSTATE_STOPPING {
        /* STOPPING = repeat call after a suspension, anything else is error */
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            cinfo.global_state,
        ));
    }
    /* Read until EOI */
    while !cinfo.inputctl().pub_.eoi_reached {
        let consume_input = cinfo.inputctl().pub_.consume_input;
        if consume_input(cinfo)? == JPEG_SUSPENDED {
            return Ok(false); /* Suspend, come back later */
        }
    }
    /* Do final cleanup */
    call_term_source(cinfo)?;
    /* We can use jpeg_abort to release memory and reset global_state */
    jpeg_abort(cinfo)?;
    Ok(true)
}
