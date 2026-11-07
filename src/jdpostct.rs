// libjpeg-turbo 3.2.1, src/jdpostct.c:
//
// /*
//  * jdpostct.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022-2024, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the decompression postprocessing controller.
//  * This controller manages the upsampling, color conversion, and color
//  * quantization/reduction steps; specifically, it controls the buffering
//  * between upsample/color conversion and color quantization/reduction.
//  *
//  * If no color quantization/reduction is required, then this module has no
//  * work to do, and it just hands off to the upsample/color conversion code.
//  * An integrated upsample/convert/quantize process would replace this module
//  * entirely.
//  */
//! Translation of `src/jdpostct.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
//!
//! Color quantization is out of scope (ADR 0001), and with
//! `QUANT_2PASS_SUPPORTED` undefined, `post_process_prepass` and
//! `post_process_2pass` are not compiled and the full-image buffer fails
//! with `JERR_BAD_BUFFER_MODE`, as upstream's `#else` does (ADR 0005).
//! `post_process_1pass`, which upstream compiles but which calls the color
//! quantizer, is not translated: `start_pass_dpost` raises
//! `JERR_NOT_COMPILED` where it would select it, the error that
//! `jdmaster.c` raises for a quantizer that is compiled out. Without virtual
//! sample arrays (ADR 0005, "Virtual arrays") `whole_image` is absent; it is
//! always C's null pointer here.

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemmgr::alloc_small_box;
use crate::jmorecfg::{C_JDIMENSION, C_POINTER, CLayout, JDIMENSION};
use crate::jpegint::{J_BUF_MODE, jpeg_d_post_controller};
use crate::jpeglib::{JPOOL_IMAGE, SampleArray, j_decompress_ptr};

// upstream: src/jdpostct.c my_post_controller
/// Private buffer controller object.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug)]
pub struct my_post_controller {
    /// Public fields.
    pub pub_: jpeg_d_post_controller,

    /* Color quantization source buffer: this holds output data from
     * the upsample/color conversion step to be passed to the quantizer.
     * For two-pass color quantization, we need a full-image buffer;
     * for one-pass operation, a strip buffer is sufficient.
     */
    /// Strip buffer, or `None`.
    pub buffer: Option<SampleArray>,
    /// Buffer size in rows.
    pub strip_height: JDIMENSION,
    /* for two-pass mode only: */
    /// Row # of first row in current strip.
    pub starting_row: JDIMENSION,
    /// Index of next row to fill/empty in strip.
    pub next_row: JDIMENSION,
}

/// `sizeof(my_post_controller)` on the target, with `whole_image` and the
/// `post_process_data_12` and `post_process_data_16` pointers of `struct
/// jpeg_d_post_controller`.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_post_controller: usize = CLayout::structure(&[
    // struct jpeg_d_post_controller
    CLayout::structure(&[C_POINTER.array(4)]),
    C_POINTER,
    C_POINTER,
    C_JDIMENSION,
    C_JDIMENSION,
    C_JDIMENSION,
])
.size;

// upstream: src/jdpostct.c start_pass_dpost
/// Initializes for a processing pass.
///
/// # Errors
///
/// `JERR_NOT_COMPILED` with color quantization (see the module
/// documentation) and `JERR_BAD_BUFFER_MODE` for any mode but
/// `JBUF_PASS_THRU`.
///
/// # Panics
///
/// When `cinfo.post` or `cinfo.upsample` is absent.
pub fn start_pass_dpost(
    cinfo: j_decompress_ptr<'_, '_>,
    pass_mode: J_BUF_MODE,
) -> Result<(), ErrorExit> {
    match pass_mode {
        J_BUF_MODE::JBUF_PASS_THRU => {
            if cinfo.quantize_colors {
                /* Single-pass processing with color quantization: out of scope */
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED));
            }
            /* For single-pass processing without color quantization,
             * I have no work to do; just call the upsampler directly.
             */
            let upsample = cinfo.upsample().upsample;
            cinfo
                .post
                .as_deref_mut()
                .expect("cinfo->post is NULL")
                .pub_
                .post_process_data = Some(upsample);
        }
        _ => return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_BUFFER_MODE)),
    }
    let post = cinfo.post.as_deref_mut().expect("cinfo->post is NULL");
    post.next_row = 0;
    post.starting_row = 0;
    Ok(())
}

// upstream: src/jdpostct.c _jinit_d_post_controller
/// Initializes the postprocessing controller.
///
/// # Errors
///
/// `JERR_BAD_PRECISION`, `JERR_BAD_BUFFER_MODE` for two-pass quantization,
/// and what the allocations report.
///
/// # Panics
///
/// Only on a translation error: the module is set before it is used.
#[expect(clippy::cast_sign_loss, reason = "C's (JDIMENSION)max_v_samp_factor")]
pub fn jinit_d_post_controller(
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

    let post = alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_my_post_controller,
        my_post_controller {
            pub_: jpeg_d_post_controller {
                start_pass: start_pass_dpost,
                post_process_data: None,
            },
            buffer: None, /* flag for no strip buffer */
            strip_height: 0,
            starting_row: 0,
            next_row: 0,
        },
    )?;
    cinfo.post = Some(post);

    /* Create the quantization buffer, if needed */
    if cinfo.quantize_colors {
        /* The buffer strip height is max_v_samp_factor, which is typically
         * an efficient number of rows for upsampling to return.
         * (In the presence of output rescaling, we might want to be smarter?)
         */
        let strip_height = cinfo.max_v_samp_factor as JDIMENSION;
        cinfo.post.as_deref_mut().expect("set above").strip_height = strip_height;
        if need_full_buffer {
            /* Two-pass color quantization: need full-image storage. */
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_BUFFER_MODE));
        }
        /* One-pass color quantization: just make a strip buffer. */
        let samplesperrow = cinfo.output_width * cinfo.out_color_components as JDIMENSION;
        let alloc_sarray = cinfo.mem().pub_.alloc_sarray;
        let buffer = alloc_sarray(cinfo, JPOOL_IMAGE, samplesperrow, strip_height)?;
        cinfo.post.as_deref_mut().expect("set above").buffer = Some(buffer);
    }
    Ok(())
}
