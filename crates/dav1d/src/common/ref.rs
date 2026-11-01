/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
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

// Mechanically translated from upstream/dav1d/src/ref.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn free(_: *mut core::ffi::c_void);
    fn posix_memalign(
        __memptr: *mut *mut core::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> core::ffi::c_int;
    fn dav1d_mem_pool_push(pool: *mut Dav1dMemPool, ptr: *mut core::ffi::c_void);
    fn dav1d_mem_pool_pop(
        pool: *mut Dav1dMemPool,
        size: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRef {
    pub data: *mut core::ffi::c_void,
    pub const_data: *const core::ffi::c_void,
    pub ref_cnt: atomic_int,
    pub free_ref: core::ffi::c_int,
    pub free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    pub user_data: *mut core::ffi::c_void,
}
pub type atomic_int = core::ffi::c_int;
pub type pthread_mutex_t = libc::pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPoolBuffer {
    pub next: *mut Dav1dMemPoolBuffer,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPool {
    pub lock: pthread_mutex_t,
    pub buf: *mut Dav1dMemPoolBuffer,
    pub ref_cnt: core::ffi::c_int,
    pub end: core::ffi::c_int,
}
pub const __ATOMIC_SEQ_CST: core::ffi::c_int = 5 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn dav1d_alloc_aligned_internal(
    sz: size_t,
    align: size_t,
) -> *mut core::ffi::c_void {
    if (align & align.wrapping_sub(1 as size_t) != 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"dav1d_alloc_aligned_internal\0"))
                .as_ptr(),
            b"mem.h\0" as *const u8 as *const core::ffi::c_char,
            90 as core::ffi::c_int,
            b"!(align & (align - 1))\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
unsafe extern "C" fn default_free_callback(
    data: *const uint8_t,
    user_data: *mut core::ffi::c_void,
) {
    if !(data == user_data as *const uint8_t) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"default_free_callback\0"))
                .as_ptr(),
            b"ref.c\0" as *const u8 as *const core::ffi::c_char,
            33 as core::ffi::c_int,
            b"data == user_data\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    dav1d_free_aligned_internal(user_data);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_ref_create(mut size: size_t) -> *mut Dav1dRef {
    size = size
        .wrapping_add(::core::mem::size_of::<*mut core::ffi::c_void>() as size_t)
        .wrapping_sub(1 as size_t)
        & !(::core::mem::size_of::<*mut core::ffi::c_void>() as size_t)
            .wrapping_sub(1 as size_t);
    let data: *mut uint8_t = dav1d_alloc_aligned_internal(
        size.wrapping_add(::core::mem::size_of::<Dav1dRef>() as size_t),
        64 as size_t,
    ) as *mut uint8_t;
    if data.is_null() {
        return 0 as *mut Dav1dRef;
    }
    let res: *mut Dav1dRef = data.offset(size as isize) as *mut Dav1dRef;
    (*res).data = data as *mut core::ffi::c_void;
    (*res).user_data = (*res).data;
    (*res).const_data = (*res).user_data;
    *&raw mut (*res).ref_cnt = 1 as core::ffi::c_int;
    (*res).free_ref = 0 as core::ffi::c_int;
    (*res).free_callback = Some(
        default_free_callback
            as unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    ) as Option<unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> ()>;
    return res;
}
unsafe extern "C" fn pool_free_callback(
    data: *const uint8_t,
    user_data: *mut core::ffi::c_void,
) {
    dav1d_mem_pool_push(data as *mut Dav1dMemPool, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_ref_create_using_pool(
    pool: *mut Dav1dMemPool,
    mut size: size_t,
) -> *mut Dav1dRef {
    let buf: *mut core::ffi::c_void = dav1d_mem_pool_pop(pool, size)
        as *mut core::ffi::c_void;
    if buf.is_null() {
        return 0 as *mut Dav1dRef;
    }
    if !((::core::mem::size_of::<Dav1dMemPoolBuffer>() as usize)
        .wrapping_add(::core::mem::size_of::<Dav1dRef>() as usize) <= 64 as usize)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"dav1d_ref_create_using_pool\0"))
                .as_ptr(),
            b"ref.c\0" as *const u8 as *const core::ffi::c_char,
            61 as core::ffi::c_int,
            b"sizeof(Dav1dMemPoolBuffer) + sizeof(Dav1dRef) <= 64\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let res: *mut Dav1dRef = &raw mut *(buf as *mut Dav1dRef)
        .offset(-(1 as core::ffi::c_int) as isize) as *mut Dav1dRef;
    (*res).data = buf;
    (*res).const_data = pool as *const core::ffi::c_void;
    *&raw mut (*res).ref_cnt = 1 as core::ffi::c_int;
    (*res).free_ref = 0 as core::ffi::c_int;
    (*res).free_callback = Some(
        pool_free_callback
            as unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    ) as Option<unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> ()>;
    (*res).user_data = buf;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_ref_dec(pref: *mut *mut Dav1dRef) {
    if pref.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"dav1d_ref_dec\0"))
                .as_ptr(),
            b"ref.c\0" as *const u8 as *const core::ffi::c_char,
            74 as core::ffi::c_int,
            b"pref != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let ref_0: *mut Dav1dRef = *pref;
    if ref_0.is_null() {
        return;
    }
    *pref = 0 as *mut Dav1dRef;
    if ::core::intrinsics::atomic_xsub::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
        &raw mut (*ref_0).ref_cnt as *mut atomic_int,
        1 as core::ffi::c_int,
    ) == 1 as core::ffi::c_int
    {
        let free_ref: core::ffi::c_int = (*ref_0).free_ref;
        ((*ref_0).free_callback)
            .expect(
                "non-null function pointer",
            )((*ref_0).const_data as *const uint8_t, (*ref_0).user_data);
        if free_ref != 0 {
            free(ref_0 as *mut core::ffi::c_void);
        }
    }
}
