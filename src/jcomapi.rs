// libjpeg-turbo 3.2.1, src/jcomapi.c:
//
// /*
//  * jcomapi.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2024-2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains application interface routines that are used for both
//  * compression and decompression.
//  */
//! Translation of `src/jcomapi.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! `jpeg_abort` is translated by work package 2, with
//! `jpeg_CreateDecompress`, because it clears `cinfo->master->marker_list_end`
//! and `cinfo->marker_list`, which that package adds (ADR 0005, "Module
//! interfaces arrive with their modules"). `WITH_PROFILE` is off in the
//! reference configuration, so the profiling report of `jpeg_destroy` is not
//! translated.

use crate::jerror::ErrorExit;
use crate::jmemmgr::alloc_small_box;
use crate::jpeglib::{
    JHUFF_TBL, JPOOL_PERMANENT, JQUANT_TBL, SIZEOF_JHUFF_TBL, SIZEOF_JQUANT_TBL, j_common_ptr,
};

// upstream: src/jcomapi.c jpeg_destroy
/// Destruction of a JPEG object: everything is released except the object
/// itself and its error manager, which belong to the application. The memory
/// manager's `self_destruct` drops every pool and the fields filled from
/// them.
///
/// # Errors
///
/// What `self_destruct` reports; nothing in practice.
pub fn jpeg_destroy(cinfo: j_common_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* We need only tell the memory manager to release everything. */
    /* NB: mem pointer is NULL if memory mgr failed to initialize. */
    if cinfo.mem.is_some() {
        let self_destruct = cinfo.mem().pub_.self_destruct;
        self_destruct(cinfo)?;
    }
    cinfo.mem = None; /* be safe if jpeg_destroy is called twice */
    cinfo.global_state = 0; /* mark it destroyed */
    Ok(())
}

// upstream: src/jcomapi.c jpeg_alloc_quant_table
/// Allocates a quantization table in the permanent pool. Its entries are
/// zero; C leaves them uninitialized.
///
/// # Errors
///
/// What `alloc_small` reports.
pub fn jpeg_alloc_quant_table(cinfo: j_common_ptr<'_, '_>) -> Result<Box<JQUANT_TBL>, ErrorExit> {
    let mut tbl = alloc_small_box(
        cinfo,
        JPOOL_PERMANENT,
        SIZEOF_JQUANT_TBL,
        JQUANT_TBL::default(),
    )?;
    tbl.sent_table = false; /* make sure this is false in any new table */
    Ok(tbl)
}

// upstream: src/jcomapi.c jpeg_alloc_huff_table
/// Allocates a Huffman table in the permanent pool. Its entries are zero; C
/// leaves them uninitialized.
///
/// # Errors
///
/// What `alloc_small` reports.
pub fn jpeg_alloc_huff_table(cinfo: j_common_ptr<'_, '_>) -> Result<Box<JHUFF_TBL>, ErrorExit> {
    let mut tbl = alloc_small_box(
        cinfo,
        JPOOL_PERMANENT,
        SIZEOF_JHUFF_TBL,
        JHUFF_TBL::default(),
    )?;
    tbl.sent_table = false; /* make sure this is false in any new table */
    Ok(tbl)
}
