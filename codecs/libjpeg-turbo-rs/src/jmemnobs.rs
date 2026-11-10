// libjpeg-turbo 3.2.1, src/jmemnobs.c:
//
// /*
//  * jmemnobs.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1992-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2017-2018, 2024, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file provides a really simple implementation of the system-
//  * dependent portion of the JPEG memory manager.  This implementation
//  * assumes that no backing-store files are needed: all required space
//  * can be obtained from malloc().
//  * This is very portable in the sense that it'll compile on almost anything,
//  * but you'd better have lots of main memory (or virtual memory) if you want
//  * to process big images.
//  */
//! Translation of `src/jmemnobs.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! Storage comes from Rust's allocator, owned by the code that requested it
//! (ADR 0005, "Owned storage"): `jpeg_get_small` and `jpeg_get_large` make
//! the caller's reservation where C calls `malloc`, and `jpeg_free_small` and
//! `jpeg_free_large` drop the pool record that C would `free`.

use core::ffi::c_long;

use crate::jerror::{ERREXIT, ErrorExit, J_MESSAGE_CODE};
use crate::jpeglib::{Reservation, j_common_ptr};

// upstream: src/jmemnobs.c jpeg_get_small
/// Obtains a small pool: the reservation of the storage for the request that
/// needs it. `false` where `malloc` returns NULL.
pub fn jpeg_get_small(
    _cinfo: j_common_ptr<'_, '_>,
    _sizeofobject: usize,
    reserve: Reservation<'_>,
) -> bool {
    reserve()
}

// upstream: src/jmemnobs.c jpeg_free_small
/// Releases a small pool or the memory manager's control block.
pub fn jpeg_free_small<T>(_cinfo: j_common_ptr<'_, '_>, object: T, _sizeofobject: usize) {
    drop(object);
}

// upstream: src/jmemnobs.c jpeg_get_large
/// Obtains a large object: the reservation of its storage. `false` where
/// `malloc` returns NULL.
pub fn jpeg_get_large(
    _cinfo: j_common_ptr<'_, '_>,
    _sizeofobject: usize,
    reserve: Reservation<'_>,
) -> bool {
    reserve()
}

// upstream: src/jmemnobs.c jpeg_free_large
/// Releases a large pool.
pub fn jpeg_free_large<T>(_cinfo: j_common_ptr<'_, '_>, object: T, _sizeofobject: usize) {
    drop(object);
}

// upstream: src/jmemnobs.c jpeg_mem_available
/// Computes the total memory space available for allocation.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C casts max_memory_to_use to size_t"
)]
pub fn jpeg_mem_available(
    cinfo: j_common_ptr<'_, '_>,
    _min_bytes_needed: usize,
    max_bytes_needed: usize,
    already_allocated: usize,
) -> usize {
    let max_memory_to_use = cinfo.mem().pub_.max_memory_to_use;
    if max_memory_to_use != 0 {
        if max_memory_to_use as usize > already_allocated {
            (max_memory_to_use as usize).wrapping_sub(already_allocated)
        } else {
            0
        }
    } else {
        /* Here we always say, "we got all you want bud!" */
        max_bytes_needed
    }
}

// upstream: src/jmemnobs.c jpeg_open_backing_store
/// Backing store (temporary file) management: there is none, so this always
/// fails with `JERR_NO_BACKING_STORE` and never returns normally. The
/// backing-store object of the C signature is never opened and has no Rust
/// type (see `jmemsys`).
pub fn jpeg_open_backing_store(
    cinfo: j_common_ptr<'_, '_>,
    _total_bytes_needed: c_long,
) -> ErrorExit {
    ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NO_BACKING_STORE)
}

// upstream: src/jmemnobs.c jpeg_mem_init
/// System-dependent initialization: sets `max_memory_to_use` to 0.
#[must_use]
pub fn jpeg_mem_init(_cinfo: j_common_ptr<'_, '_>) -> c_long {
    0 /* just set max_memory_to_use to 0 */
}

// upstream: src/jmemnobs.c jpeg_mem_term
/// System-dependent cleanup; there is none.
pub fn jpeg_mem_term(_cinfo: j_common_ptr<'_, '_>) {
    /* no work */
}
