/*
 * Copyright © 2024, VideoLAN and dav1d authors
 * Copyright © 2024, Two Orioles, LLC
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

// Mechanically translated from upstream/dav1d/src/ctx.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias64 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias32 {
    pub u32_0: uint32_t,
    pub u8_0: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias8 {
    pub u8_0: uint8_t,
}
pub type dav1d_memset_pow2_fn = Option<
    unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
>;
unsafe extern "C" fn memset_w1(ptr: *mut core::ffi::c_void, value: core::ffi::c_int) {
    (*(&raw mut *(ptr as *mut uint8_t).offset(0 as core::ffi::c_int as isize) as *mut uint8_t
        as *mut alias8))
        .u8_0 = (value * 0x1 as core::ffi::c_int) as uint8_t;
}
unsafe extern "C" fn memset_w2(ptr: *mut core::ffi::c_void, value: core::ffi::c_int) {
    (*(&raw mut *(ptr as *mut uint8_t).offset(0 as core::ffi::c_int as isize) as *mut uint8_t
        as *mut alias16))
        .u16_0 = (value * 0x101 as core::ffi::c_int) as uint16_t;
}
unsafe extern "C" fn memset_w4(ptr: *mut core::ffi::c_void, value: core::ffi::c_int) {
    (*(&raw mut *(ptr as *mut uint8_t).offset(0 as core::ffi::c_int as isize) as *mut uint8_t
        as *mut alias32))
        .u32_0 = (value as core::ffi::c_uint)
        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
}
unsafe extern "C" fn memset_w8(ptr: *mut core::ffi::c_void, value: core::ffi::c_int) {
    (*(&raw mut *(ptr as *mut uint8_t).offset(0 as core::ffi::c_int as isize) as *mut uint8_t
        as *mut alias64))
        .u64_0 = (value as core::ffi::c_ulonglong)
        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong) as uint64_t;
}
unsafe extern "C" fn memset_w16(ptr: *mut core::ffi::c_void, value: core::ffi::c_int) {
    memset(
        &raw mut *(ptr as *mut uint8_t).offset(0 as core::ffi::c_int as isize)
            as *mut uint8_t as *mut core::ffi::c_void,
        value,
        16 as size_t,
    );
}
unsafe extern "C" fn memset_w32(ptr: *mut core::ffi::c_void, value: core::ffi::c_int) {
    memset(
        &raw mut *(ptr as *mut uint8_t).offset(0 as core::ffi::c_int as isize)
            as *mut uint8_t as *mut core::ffi::c_void,
        value,
        32 as size_t,
    );
}
#[no_mangle]
pub static mut dav1d_memset_pow2: [dav1d_memset_pow2_fn; 6] = unsafe {
    [
        Some(
            memset_w1
                as unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
        ),
        Some(
            memset_w2
                as unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
        ),
        Some(
            memset_w4
                as unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
        ),
        Some(
            memset_w8
                as unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
        ),
        Some(
            memset_w16
                as unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
        ),
        Some(
            memset_w32
                as unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
        ),
    ]
};
