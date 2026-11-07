// libjpeg-turbo 3.2.1, src/jdatasrc.c:
//
// /*
//  * jdatasrc.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * Modified 2009-2011 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2013, 2016, 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains decompression data source routines for the case of
//  * reading JPEG data from memory or from a file (or any stdio stream).
//  * While these routines are sufficient for most applications,
//  * some will want to use a different source manager.
//  * IMPORTANT: we assume that fread() will correctly transcribe an array of
//  * JOCTETs from 8-bit-wide elements on external storage.  If char is wider
//  * than 8 bits on your machine, you may need to do some tweaking.
//  */
//! Translation of `src/jdatasrc.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
//!
//! Not translated: `init_source`, `fill_input_buffer` and `jpeg_stdio_src`, because the stdio source is out of scope; only `jpeg_mem_src` is translated (ADR 0001).
//!
//! The memory source's methods are [`MemSourceMethods`]; its input is a
//! borrowed slice, so a null `inbuffer` cannot occur and `insize` is the
//! slice's length.

use core::ffi::c_long;

use crate::jerror::{ERREXIT, ErrorExit, J_MESSAGE_CODE, WARNMS};
use crate::jmemmgr::alloc_small_box;
use crate::jmorecfg::JOCTET;
use crate::jpeglib::{
    InputBuffer, JPEG_EOI, JPOOL_PERMANENT, SIZEOF_jpeg_source_mgr, jpeg_decompress_struct,
    jpeg_source_methods, jpeg_source_mgr,
};

/// The fake EOI marker that `fill_mem_input_buffer` inserts.
#[expect(clippy::cast_possible_truncation, reason = "JPEG_EOI is a byte")]
static MYBUFFER: [JOCTET; 4] = [0xFF, JPEG_EOI as JOCTET, 0, 0];

/// The methods that `jpeg_mem_src` installs. Not an upstream item.
#[derive(Clone, Copy, Debug, Default)]
pub struct MemSourceMethods;

impl jpeg_source_methods for MemSourceMethods {
    fn init_source(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        init_mem_source(cinfo);
        Ok(())
    }

    fn fill_input_buffer(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
    ) -> Result<bool, ErrorExit> {
        fill_mem_input_buffer(cinfo)
    }

    fn skip_input_data(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        num_bytes: c_long,
    ) -> Result<(), ErrorExit> {
        skip_input_data(self, cinfo, num_bytes)
    }

    fn resync_to_restart(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        desired: i32,
    ) -> Result<bool, ErrorExit> {
        crate::jdmarker::jpeg_resync_to_restart(cinfo, desired) /* use default method */
    }

    fn term_source(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        term_source(cinfo);
        Ok(())
    }

    fn is_mem_source(&self) -> bool {
        true
    }
}

// upstream: src/jdatasrc.c init_mem_source
/// Initializes the memory source; no work necessary here.
pub fn init_mem_source(_cinfo: &mut jpeg_decompress_struct<'_>) {
    /* no work necessary here */
}

// upstream: src/jdatasrc.c fill_mem_input_buffer
/// The whole JPEG data is expected to reside in the supplied memory buffer,
/// so any request for more data beyond the given buffer size is treated as
/// an error: warns `JWRN_JPEG_EOF` and inserts a fake EOI marker.
///
/// # Errors
///
/// What `emit_message` returns for the warning.
pub fn fill_mem_input_buffer(cinfo: &mut jpeg_decompress_struct<'_>) -> Result<bool, ErrorExit> {
    WARNMS(cinfo, J_MESSAGE_CODE::JWRN_JPEG_EOF)?;

    /* Insert a fake EOI marker */

    let src = cinfo.src_mut();
    src.buffer = InputBuffer::Borrowed(&MYBUFFER);
    src.next_input_byte = 0;
    src.bytes_in_buffer = 2;

    Ok(true)
}

// upstream: src/jdatasrc.c skip_input_data
/// Skips data, refilling the buffer through the source's own
/// `fill_input_buffer` as needed, which it assumes never suspends.
///
/// # Errors
///
/// What `fill_input_buffer` reports.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between long and size_t"
)]
pub fn skip_input_data<M: jpeg_source_methods + ?Sized>(
    methods: &mut M,
    cinfo: &mut jpeg_decompress_struct<'_>,
    mut num_bytes: c_long,
) -> Result<(), ErrorExit> {
    /* Just a dumb implementation for now.  Could use fseek() except
     * it doesn't work on pipes.  Not clear that being smart is worth
     * any trouble anyway --- large skips are infrequent.
     */
    if num_bytes > 0 {
        while num_bytes > cinfo.src().bytes_in_buffer as c_long {
            num_bytes -= cinfo.src().bytes_in_buffer as c_long;
            let _ = methods.fill_input_buffer(cinfo)?;
            /* note we assume that fill_input_buffer will never return FALSE,
             * so suspension need not be handled.
             */
        }
        let src = cinfo.src_mut();
        src.next_input_byte += num_bytes as usize;
        src.bytes_in_buffer -= num_bytes as usize;
    }
    Ok(())
}

// upstream: src/jdatasrc.c term_source
/// Terminates the source; no work necessary here.
pub fn term_source(_cinfo: &mut jpeg_decompress_struct<'_>) {
    /* no work necessary here */
}

// upstream: src/jdatasrc.c jpeg_mem_src
/// Prepares for input from a supplied memory buffer. The source object is
/// made permanent, so that a series of JPEG images can be read from the same
/// buffer by calling `jpeg_mem_src` only before the first one.
///
/// # Errors
///
/// `JERR_INPUT_EMPTY` for empty input; `JERR_BUFFER_SIZE` when the object
/// already has a source manager other than the memory source's.
pub fn jpeg_mem_src<'src>(
    cinfo: &mut jpeg_decompress_struct<'src>,
    inbuffer: &'src [u8],
) -> Result<(), ErrorExit> {
    if inbuffer.is_empty() {
        /* Treat empty input as fatal error */
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_INPUT_EMPTY));
    }

    /* The source object is made permanent so that a series of JPEG images
     * can be read from the same buffer by calling jpeg_mem_src only before
     * the first one.
     */
    match cinfo.src.as_deref() {
        None => {
            /* first time for this JPEG object? */
            let src = jpeg_source_mgr {
                next_input_byte: 0,
                bytes_in_buffer: 0,
                buffer: InputBuffer::Borrowed(&[]),
                methods: None,
            };
            cinfo.src = Some(alloc_small_box(
                cinfo,
                JPOOL_PERMANENT,
                SIZEOF_jpeg_source_mgr,
                src,
            )?);
        }
        Some(src)
            if !src
                .methods
                .as_deref()
                .is_some_and(jpeg_source_methods::is_mem_source) =>
        {
            /* It is unsafe to reuse the existing source manager unless it was created
             * by this function.
             */
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BUFFER_SIZE));
        }
        Some(_) => {}
    }

    let src = cinfo.src_mut();
    src.methods = Some(Box::new(MemSourceMethods));
    src.bytes_in_buffer = inbuffer.len();
    src.buffer = InputBuffer::Borrowed(inbuffer);
    src.next_input_byte = 0;
    Ok(())
}
