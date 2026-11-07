// libwebp v1.6.0, src/webp/types.h:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  Common types + memory wrappers
//
// Author: Skal (pascal.massimino@gmail.com)

//! C ABI export of `src/webp/types.h` from libwebp v1.6.0.
//!
//! Mirrors the header's structs with `repr(C)` and converts them to the
//! safe representations at the boundary; see
//! `docs/adr/0004-c-abi-boundary.md`.
//!
//! Work package: WP12 (decoder integration); exports `WebPMalloc` and `WebPFree`, which the decoder tools need first.
//!
//! Memory that C callers release with `WebPFree()` or `free()` comes from the
//! platform `malloc`, which this module declares (ADR 0004, memory). The other
//! ABI modules hand decoded samples to C through [`malloc_copy`].

use core::ffi::c_void;

use crate::utils::utils::webp_safe_malloc;

// The platform allocator of the C library that Rust's standard library links
// already (ADR 0004).
unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

/// Allocates `size` bytes with the platform `malloc`; null when it fails.
pub(crate) fn platform_malloc(size: usize) -> *mut u8 {
    // SAFETY: `malloc` accepts every size and returns either null or a block
    // of at least `size` bytes; it has no other precondition.
    unsafe { malloc(size) }.cast()
}

/// Releases a block of the platform `malloc`.
///
/// # Safety
///
/// `ptr` is null or a block of the platform `malloc` that is not used again.
pub(crate) unsafe fn platform_free(ptr: *mut u8) {
    // SAFETY: the caller passes null or a live `malloc` block that it gives up.
    unsafe { free(ptr.cast()) }
}

/// Copies `bytes` into a new block of the platform `malloc`, which C releases
/// with `free()` or `WebPFree()`; null when the allocation fails.
pub(crate) fn malloc_copy(bytes: &[u8]) -> *mut u8 {
    let block = platform_malloc(bytes.len());
    if !block.is_null() {
        // SAFETY: `block` holds at least `bytes.len()` bytes and is a new
        // allocation, so it does not overlap `bytes`.
        unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), block, bytes.len()) };
    }
    block
}

// upstream: src/utils/utils.c WebPMalloc
/// Allocates `size` bytes of memory. Returns null upon error. Memory must be
/// deallocated by calling `WebPFree()`. This function is made available by the
/// core `libwebp` library. C applies the size test of
/// `WebPSafeMalloc(1, size)` and then calls `malloc`; the test is the Rust
/// translation's, and the block comes from the platform `malloc`.
#[unsafe(export_name = "WebPMalloc")]
#[must_use]
pub extern "C" fn webp_malloc(size: usize) -> *mut c_void {
    // The size test of WebPSafeMalloc(1, size): the reservation of one byte
    // that the helper makes is released at once.
    if webp_safe_malloc::<u8>(1, size).is_none() {
        return core::ptr::null_mut();
    }
    platform_malloc(size).cast()
}

// upstream: src/utils/utils.c WebPFree
/// Releases memory returned by the `WebPDecode*()` functions (from
/// `decode.h`) and by `WebPMalloc()`.
///
/// # Safety
///
/// `ptr` is null or a block that this library returned and that is not used
/// again.
#[unsafe(export_name = "WebPFree")]
pub unsafe extern "C" fn webp_free(ptr: *mut c_void) {
    // C: WebPSafeFree(ptr), which is free() without PRINT_MEM_INFO.
    // SAFETY: the caller passes null or a live block of this library, which
    // comes from the platform `malloc`.
    unsafe { platform_free(ptr.cast()) }
}
