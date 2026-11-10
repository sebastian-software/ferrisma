/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_mem/aom_mem.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type size_t = usize;
pub type uintptr_t = usize;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_MAX_ALLOCABLE_MEMORY: core::ffi::c_long = 8589934592 as core::ffi::c_long;
pub const ADDRESS_STORAGE_SIZE: usize = ::core::mem::size_of::<size_t>();
pub const DEFAULT_ALIGNMENT: usize = (2 as usize)
    .wrapping_mul(::core::mem::size_of::<*mut core::ffi::c_void>() as usize);
unsafe extern "C-unwind" fn GetAllocationPaddingSize(mut align: size_t) -> size_t {
    return align.wrapping_sub(1 as size_t).wrapping_add(ADDRESS_STORAGE_SIZE);
}
unsafe extern "C-unwind" fn check_size_argument_overflow(
    mut nmemb: size_t,
    mut size: size_t,
    mut align: size_t,
) -> core::ffi::c_int {
    if nmemb == 0 as size_t {
        return 1 as core::ffi::c_int;
    }
    let alloc_padding: size_t = GetAllocationPaddingSize(align) as size_t;
    if size
        > (AOM_MAX_ALLOCABLE_MEMORY as size_t)
            .wrapping_sub(alloc_padding)
            .wrapping_div(nmemb)
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn GetMallocAddressLocation(
    mem: *mut core::ffi::c_void,
) -> *mut size_t {
    return (mem as *mut size_t).offset(-(1 as core::ffi::c_int as isize));
}
unsafe extern "C-unwind" fn SetActualMallocAddress(
    mem: *mut core::ffi::c_void,
    malloc_addr: *const core::ffi::c_void,
) {
    let malloc_addr_location: *mut size_t = GetMallocAddressLocation(mem) as *mut size_t;
    *malloc_addr_location = malloc_addr as size_t;
}
unsafe extern "C-unwind" fn GetActualMallocAddress(
    mem: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let malloc_addr_location: *const size_t = GetMallocAddressLocation(mem);
    return *malloc_addr_location as *mut core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_memalign(
    mut align: size_t,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    let mut x: *mut core::ffi::c_void = NULL;
    if check_size_argument_overflow(1 as size_t, size, align) == 0 {
        return NULL;
    }
    let aligned_size: size_t = size
        .wrapping_add(GetAllocationPaddingSize(align) as size_t);
    let addr: *mut core::ffi::c_void = malloc(aligned_size) as *mut core::ffi::c_void;
    if !addr.is_null() {
        x = (((addr as *mut core::ffi::c_uchar)
            .offset(::core::mem::size_of::<size_t>() as usize as isize) as uintptr_t)
            .wrapping_add((align as uintptr_t).wrapping_sub(1 as uintptr_t))
            & !(align.wrapping_sub(1 as size_t) as uintptr_t)) as *mut core::ffi::c_void;
        SetActualMallocAddress(x, addr);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_malloc(mut size: size_t) -> *mut core::ffi::c_void {
    return aom_memalign(DEFAULT_ALIGNMENT, size);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_calloc(
    mut num: size_t,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    if check_size_argument_overflow(num, size, DEFAULT_ALIGNMENT) == 0 {
        return NULL;
    }
    let total_size: size_t = num.wrapping_mul(size);
    let x: *mut core::ffi::c_void = aom_malloc(total_size) as *mut core::ffi::c_void;
    if !x.is_null() {
        memset(x, 0 as core::ffi::c_int, total_size);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_free(mut memblk: *mut core::ffi::c_void) {
    if !memblk.is_null() {
        let mut addr: *mut core::ffi::c_void = GetActualMallocAddress(memblk);
        free(addr);
    }
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
}
