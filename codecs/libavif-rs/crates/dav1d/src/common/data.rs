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

// Mechanically translated from upstream/dav1d/src/data.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type __sFILEX;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn abort() -> !;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn dav1d_ref_create(size: size_t) -> *mut Dav1dRef;
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
}
pub type int64_t = i64;
pub type uint8_t = u8;
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const uint8_t,
    pub ref_0: *mut Dav1dRef,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDataProps {
    pub timestamp: int64_t,
    pub duration: int64_t,
    pub offset: int64_t,
    pub size: size_t,
    pub user_data: Dav1dUserData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
    pub ref_0: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut core::ffi::c_uchar,
    pub _size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut core::ffi::c_uchar,
    pub _r: core::ffi::c_int,
    pub _w: core::ffi::c_int,
    pub _flags: core::ffi::c_short,
    pub _file: core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: core::ffi::c_int,
    pub _cookie: *mut core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut core::ffi::c_void, fpos_t, core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut core::ffi::c_void,
            *const core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: core::ffi::c_int,
    pub _ubuf: [core::ffi::c_uchar; 3],
    pub _nbuf: [core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = libc::FILE;
pub const memory_order_relaxed: memory_order = 0;
pub type memory_order = core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const INT64_MIN: core::ffi::c_longlong = -INT64_MAX - 1 as core::ffi::c_longlong;
pub const UINTPTR_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const SIZE_MAX: core::ffi::c_ulong = UINTPTR_MAX;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn dav1d_ref_init(
    ref_0: *mut Dav1dRef,
    ptr: *const core::ffi::c_void,
    free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    user_data: *mut core::ffi::c_void,
    free_ref: core::ffi::c_int,
) -> *mut Dav1dRef {
    (*ref_0).data = NULL;
    (*ref_0).const_data = ptr;
    *&raw mut (*ref_0).ref_cnt = 1 as core::ffi::c_int;
    (*ref_0).free_ref = free_ref;
    (*ref_0).free_callback = free_callback;
    (*ref_0).user_data = user_data;
    return ref_0;
}
#[inline]
unsafe extern "C" fn dav1d_ref_inc(ref_0: *mut Dav1dRef) {
    ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::Relaxed}>(
        &raw mut (*ref_0).ref_cnt,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create_internal(
    buf: *mut Dav1dData,
    sz: size_t,
) -> *mut uint8_t {
    if buf.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"buf != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"dav1d_data_create_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    if sz > (SIZE_MAX as size_t).wrapping_div(2 as size_t) {
        return 0 as *mut uint8_t;
    }
    (*buf).ref_0 = dav1d_ref_create(sz) as *mut Dav1dRef;
    if ((*buf).ref_0).is_null() {
        return 0 as *mut uint8_t;
    }
    (*buf).data = (*(*buf).ref_0).const_data as *const uint8_t;
    (*buf).sz = sz;
    dav1d_data_props_set_defaults(&raw mut (*buf).m);
    (*buf).m.size = sz;
    return (*(*buf).ref_0).data as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_internal(
    buf: *mut Dav1dData,
    ptr: *const uint8_t,
    sz: size_t,
    free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    cookie: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    if buf.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"buf != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    if ptr.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"ptr != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    if free_callback.is_none() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"free_callback != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    if sz > (SIZE_MAX as size_t).wrapping_div(2 as size_t) {
        return -(22 as core::ffi::c_int);
    }
    let ref_0: *mut Dav1dRef = malloc(::core::mem::size_of::<Dav1dRef>() as size_t)
        as *mut Dav1dRef;
    if ref_0.is_null() {
        return -(12 as core::ffi::c_int);
    }
    (*buf).ref_0 = dav1d_ref_init(
        ref_0,
        ptr as *const core::ffi::c_void,
        free_callback,
        cookie,
        1 as core::ffi::c_int,
    ) as *mut Dav1dRef;
    (*buf).data = ptr;
    (*buf).sz = sz;
    dav1d_data_props_set_defaults(&raw mut (*buf).m);
    (*buf).m.size = sz;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data_internal(
    buf: *mut Dav1dData,
    user_data: *const uint8_t,
    free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    cookie: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    if buf.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"buf != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"dav1d_data_wrap_user_data_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    if free_callback.is_none() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"free_callback != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"dav1d_data_wrap_user_data_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    let ref_0: *mut Dav1dRef = malloc(::core::mem::size_of::<Dav1dRef>() as size_t)
        as *mut Dav1dRef;
    if ref_0.is_null() {
        return -(12 as core::ffi::c_int);
    }
    (*buf).m.user_data.ref_0 = dav1d_ref_init(
        ref_0,
        user_data as *const core::ffi::c_void,
        free_callback,
        cookie,
        1 as core::ffi::c_int,
    ) as *mut Dav1dRef;
    (*buf).m.user_data.data = user_data;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData) {
    if dst.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"dav1d_data_ref\0"))
                .as_ptr(),
            b"data.c\0" as *const u8 as *const core::ffi::c_char,
            99 as core::ffi::c_int,
            b"dst != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*dst).data).is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"dav1d_data_ref\0"))
                .as_ptr(),
            b"data.c\0" as *const u8 as *const core::ffi::c_char,
            100 as core::ffi::c_int,
            b"dst->data == NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if src.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"dav1d_data_ref\0"))
                .as_ptr(),
            b"data.c\0" as *const u8 as *const core::ffi::c_char,
            101 as core::ffi::c_int,
            b"src != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*src).ref_0).is_null() {
        if ((*src).data).is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"dav1d_data_ref\0"))
                    .as_ptr(),
                b"data.c\0" as *const u8 as *const core::ffi::c_char,
                104 as core::ffi::c_int,
                b"src->data != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dav1d_ref_inc((*src).ref_0 as *mut Dav1dRef);
    }
    if !((*src).m.user_data.ref_0).is_null() {
        dav1d_ref_inc((*src).m.user_data.ref_0 as *mut Dav1dRef);
    }
    *dst = *src;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_copy(
    dst: *mut Dav1dDataProps,
    src: *const Dav1dDataProps,
) {
    if dst.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"dav1d_data_props_copy\0"))
                .as_ptr(),
            b"data.c\0" as *const u8 as *const core::ffi::c_char,
            114 as core::ffi::c_int,
            b"dst != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if src.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"dav1d_data_props_copy\0"))
                .as_ptr(),
            b"data.c\0" as *const u8 as *const core::ffi::c_char,
            115 as core::ffi::c_int,
            b"src != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    dav1d_ref_dec(&raw mut (*dst).user_data.ref_0);
    *dst = *src;
    if !((*dst).user_data.ref_0).is_null() {
        dav1d_ref_inc((*dst).user_data.ref_0 as *mut Dav1dRef);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps) {
    if props.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"dav1d_data_props_set_defaults\0"))
                .as_ptr(),
            b"data.c\0" as *const u8 as *const core::ffi::c_char,
            123 as core::ffi::c_int,
            b"props != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    memset(
        props as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<Dav1dDataProps>() as size_t,
    );
    (*props).timestamp = INT64_MIN as int64_t;
    (*props).offset = -(1 as core::ffi::c_int) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref_internal(props: *mut Dav1dDataProps) {
    if props.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"props != ((void*)0)\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"dav1d_data_props_unref_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    let mut user_data_ref: *mut Dav1dRef = (*props).user_data.ref_0 as *mut Dav1dRef;
    dav1d_data_props_set_defaults(props);
    dav1d_ref_dec(&raw mut user_data_ref);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref_internal(buf: *mut Dav1dData) {
    if buf.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"dav1d_data_unref_internal\0"))
                .as_ptr(),
        );
        abort();
    }
    let mut user_data_ref: *mut Dav1dRef = (*buf).m.user_data.ref_0 as *mut Dav1dRef;
    if !((*buf).ref_0).is_null() {
        if ((*buf).data).is_null() {
            fprintf(
                __stderrp,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const core::ffi::c_char,
                b"buf->data != ((void*)0)\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 26],
                    [core::ffi::c_char; 26],
                >(*b"dav1d_data_unref_internal\0"))
                    .as_ptr(),
            );
            abort();
        }
        dav1d_ref_dec(&raw mut (*buf).ref_0);
    }
    memset(
        buf as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<Dav1dData>() as size_t,
    );
    dav1d_data_props_set_defaults(&raw mut (*buf).m);
    dav1d_ref_dec(&raw mut user_data_ref);
}
