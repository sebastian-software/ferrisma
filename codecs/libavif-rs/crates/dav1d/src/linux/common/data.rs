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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn dav1d_ref_create(size: size_t) -> *mut Dav1dRef;
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
}
pub type __uint8_t = u8;
pub type __int64_t = i64;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
pub const memory_order_relaxed: memory_order = 0;
pub type memory_order = core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const INT64_MIN: core::ffi::c_long = -(9223372036854775807 as core::ffi::c_long)
    - 1 as core::ffi::c_long;
pub const SIZE_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
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
            stderr,
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
            stderr,
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
            stderr,
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
            stderr,
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
            stderr,
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
            stderr,
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
    if !dst.is_null() {} else {
        __assert_fail(
            b"dst != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/data.c\0" as *const u8
                as *const core::ffi::c_char,
            99 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 62],
                [core::ffi::c_char; 62],
            >(*b"void dav1d_data_ref(Dav1dData *const, const Dav1dData *const)\0"))
                .as_ptr(),
        );
    };
    if ((*dst).data).is_null() {} else {
        __assert_fail(
            b"dst->data == NULL\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/data.c\0" as *const u8
                as *const core::ffi::c_char,
            100 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 62],
                [core::ffi::c_char; 62],
            >(*b"void dav1d_data_ref(Dav1dData *const, const Dav1dData *const)\0"))
                .as_ptr(),
        );
    };
    if !src.is_null() {} else {
        __assert_fail(
            b"src != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/data.c\0" as *const u8
                as *const core::ffi::c_char,
            101 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 62],
                [core::ffi::c_char; 62],
            >(*b"void dav1d_data_ref(Dav1dData *const, const Dav1dData *const)\0"))
                .as_ptr(),
        );
    };
    if !((*src).ref_0).is_null() {
        if !((*src).data).is_null() {} else {
            __assert_fail(
                b"src->data != NULL\0" as *const u8 as *const core::ffi::c_char,
                b"../dav1d-native-source/src/data.c\0" as *const u8
                    as *const core::ffi::c_char,
                104 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 62],
                    [core::ffi::c_char; 62],
                >(*b"void dav1d_data_ref(Dav1dData *const, const Dav1dData *const)\0"))
                    .as_ptr(),
            );
        };
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
    if !dst.is_null() {} else {
        __assert_fail(
            b"dst != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/data.c\0" as *const u8
                as *const core::ffi::c_char,
            114 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 79],
                [core::ffi::c_char; 79],
            >(
                *b"void dav1d_data_props_copy(Dav1dDataProps *const, const Dav1dDataProps *const)\0",
            ))
                .as_ptr(),
        );
    };
    if !src.is_null() {} else {
        __assert_fail(
            b"src != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/data.c\0" as *const u8
                as *const core::ffi::c_char,
            115 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 79],
                [core::ffi::c_char; 79],
            >(
                *b"void dav1d_data_props_copy(Dav1dDataProps *const, const Dav1dDataProps *const)\0",
            ))
                .as_ptr(),
        );
    };
    dav1d_ref_dec(&raw mut (*dst).user_data.ref_0);
    *dst = *src;
    if !((*dst).user_data.ref_0).is_null() {
        dav1d_ref_inc((*dst).user_data.ref_0 as *mut Dav1dRef);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps) {
    if !props.is_null() {} else {
        __assert_fail(
            b"props != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/data.c\0" as *const u8
                as *const core::ffi::c_char,
            123 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 58],
                [core::ffi::c_char; 58],
            >(*b"void dav1d_data_props_set_defaults(Dav1dDataProps *const)\0"))
                .as_ptr(),
        );
    };
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
            stderr,
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
            stderr,
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
                stderr,
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
