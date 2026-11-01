/*
 * Copyright © 2020, VideoLAN and dav1d authors
 * Copyright © 2020, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Mechanically translated from upstream/dav1d/src/mem.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn posix_memalign(
        __memptr: *mut *mut core::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type uintptr_t = usize;
pub type size_t = usize;
pub type pthread_mutex_t = libc::pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPool {
    pub lock: pthread_mutex_t,
    pub buf: *mut Dav1dMemPoolBuffer,
    pub ref_cnt: core::ffi::c_int,
    pub end: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPoolBuffer {
    pub next: *mut Dav1dMemPoolBuffer,
    pub size: size_t,
}
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn dav1d_alloc_aligned_internal(
    sz: size_t,
    align: size_t,
) -> *mut core::ffi::c_void {
    if align & align.wrapping_sub(1 as size_t) != 0 {
        unreachable!();
    }
    let mut ptr: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    if posix_memalign(&raw mut ptr, align, sz) != 0 {
        return NULL;
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn dav1d_free_aligned_internal(mut ptr: *mut core::ffi::c_void) {
    free(ptr);
}
#[cold]
unsafe extern "C" fn mem_pool_destroy(pool: *mut Dav1dMemPool) {
    pthread_mutex_destroy(&raw mut (*pool).lock);
    free(pool as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_mem_pool_push(
    pool: *mut Dav1dMemPool,
    ptr: *mut core::ffi::c_void,
) {
    pthread_mutex_lock(&raw mut (*pool).lock);
    let buf: *mut Dav1dMemPoolBuffer = (ptr as uintptr_t).wrapping_sub(64 as uintptr_t)
        as *mut Dav1dMemPoolBuffer;
    (*pool).ref_cnt -= 1;
    let ref_cnt: core::ffi::c_int = (*pool).ref_cnt;
    if (*pool).end == 0 {
        (*buf).next = (*pool).buf as *mut Dav1dMemPoolBuffer;
        (*pool).buf = buf;
        pthread_mutex_unlock(&raw mut (*pool).lock);
        if !(ref_cnt > 0 as core::ffi::c_int) {
            unreachable!();
        }
    } else {
        pthread_mutex_unlock(&raw mut (*pool).lock);
        dav1d_free_aligned_internal(buf as *mut core::ffi::c_void);
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_mem_pool_pop(
    pool: *mut Dav1dMemPool,
    size: size_t,
) -> *mut core::ffi::c_void {
    pthread_mutex_lock(&raw mut (*pool).lock);
    let mut buf: *mut Dav1dMemPoolBuffer = (*pool).buf;
    (*pool).ref_cnt += 1;
    let mut current_block_14: u64;
    if !buf.is_null() {
        (*pool).buf = (*buf).next as *mut Dav1dMemPoolBuffer;
        pthread_mutex_unlock(&raw mut (*pool).lock);
        if (*buf).size != size {
            dav1d_free_aligned_internal(buf as *mut core::ffi::c_void);
            current_block_14 = 0;
        } else {
            current_block_14 = 1;
        }
    } else {
        pthread_mutex_unlock(&raw mut (*pool).lock);
        current_block_14 = 0;
    }
    match current_block_14 {
        0 => {
            buf = dav1d_alloc_aligned_internal(
                size.wrapping_add(64 as size_t),
                64 as size_t,
            ) as *mut Dav1dMemPoolBuffer;
            if buf.is_null() {
                pthread_mutex_lock(&raw mut (*pool).lock);
                (*pool).ref_cnt -= 1;
                let ref_cnt: core::ffi::c_int = (*pool).ref_cnt;
                pthread_mutex_unlock(&raw mut (*pool).lock);
                if ref_cnt == 0 {
                    mem_pool_destroy(pool);
                }
                return NULL;
            }
            (*buf).size = size;
        }
        _ => {}
    }
    return (buf as uintptr_t).wrapping_add(64 as uintptr_t) as *mut core::ffi::c_void;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_mem_pool_init(
    ppool: *mut *mut Dav1dMemPool,
) -> core::ffi::c_int {
    let pool: *mut Dav1dMemPool = malloc(
        ::core::mem::size_of::<Dav1dMemPool>() as size_t,
    ) as *mut Dav1dMemPool;
    if !pool.is_null() {
        if pthread_mutex_init(&raw mut (*pool).lock, 0 as *const pthread_mutexattr_t) == 0 {
            (*pool).buf = 0 as *mut Dav1dMemPoolBuffer;
            (*pool).ref_cnt = 1 as core::ffi::c_int;
            (*pool).end = 0 as core::ffi::c_int;
            *ppool = pool;
            return 0 as core::ffi::c_int;
        }
        free(pool as *mut core::ffi::c_void);
    }
    *ppool = 0 as *mut Dav1dMemPool;
    return -(12 as core::ffi::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_mem_pool_end(pool: *mut Dav1dMemPool) {
    if !pool.is_null() {
        pthread_mutex_lock(&raw mut (*pool).lock);
        let mut buf: *mut Dav1dMemPoolBuffer = (*pool).buf;
        (*pool).ref_cnt -= 1;
        let ref_cnt: core::ffi::c_int = (*pool).ref_cnt;
        (*pool).buf = 0 as *mut Dav1dMemPoolBuffer;
        (*pool).end = 1 as core::ffi::c_int;
        pthread_mutex_unlock(&raw mut (*pool).lock);
        while !buf.is_null() {
            let ptr: *mut core::ffi::c_void = buf as *mut core::ffi::c_void;
            buf = (*buf).next as *mut Dav1dMemPoolBuffer;
            dav1d_free_aligned_internal(ptr);
        }
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
    }
}
