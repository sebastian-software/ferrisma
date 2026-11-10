// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/mem.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn avifAlloc(mut size: size_t) -> *mut core::ffi::c_void {
    if size == 0 as size_t {
        return NULL;
    }
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn avifCalloc(
    mut count: size_t,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    return calloc(count, size);
}
#[no_mangle]
pub unsafe extern "C" fn avifFree(mut p: *mut core::ffi::c_void) {
    free(p);
}
