// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Speed-critical functions for Sharp YUV.
//
// Author: Skal (pascal.massimino@gmail.com)

// Mechanically translated from upstream/libwebp/sharpyuv/sharpyuv_dsp.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    static mut SharpYuvGetCPUInfo: VP8CPUInfo;
}
pub type int16_t = i16;
pub type uint16_t = u16;
pub type uint64_t = u64;
pub type CPUFeature = core::ffi::c_uint;
pub const kMSA: CPUFeature = 9;
pub const kMIPSdspR2: CPUFeature = 8;
pub const kMIPS32: CPUFeature = 7;
pub const kNEON: CPUFeature = 6;
pub const kAVX2: CPUFeature = 5;
pub const kAVX: CPUFeature = 4;
pub const kSSE4_1: CPUFeature = 3;
pub const kSlowSSSE3: CPUFeature = 2;
pub const kSSE3: CPUFeature = 1;
pub const kSSE2: CPUFeature = 0;
pub type VP8CPUInfo = Option<unsafe extern "C" fn(CPUFeature) -> core::ffi::c_int>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
unsafe extern "C" fn clip(
    mut v: core::ffi::c_int,
    mut max: core::ffi::c_int,
) -> uint16_t {
    return (if v < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else if v > max {
        max
    } else {
        v as uint16_t as core::ffi::c_int
    }) as uint16_t;
}
unsafe extern "C" fn SharpYuvUpdateY_C(
    mut ref_0: *const uint16_t,
    mut src: *const uint16_t,
    mut dst: *mut uint16_t,
    mut len: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) -> uint64_t {
    let mut diff: uint64_t = 0 as uint64_t;
    let mut i: core::ffi::c_int = 0;
    let max_y: core::ffi::c_int = ((1 as core::ffi::c_int) << bit_depth)
        - 1 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < len {
        let diff_y: core::ffi::c_int = *ref_0.offset(i as isize) as core::ffi::c_int
            - *src.offset(i as isize) as core::ffi::c_int;
        let new_y: core::ffi::c_int = *dst.offset(i as isize) as core::ffi::c_int
            + diff_y;
        *dst.offset(i as isize) = clip(new_y, max_y);
        diff = diff.wrapping_add(abs(diff_y) as uint64_t);
        i += 1;
    }
    return diff;
}
unsafe extern "C" fn SharpYuvUpdateRGB_C(
    mut ref_0: *const int16_t,
    mut src: *const int16_t,
    mut dst: *mut int16_t,
    mut len: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < len {
        let diff_uv: core::ffi::c_int = *ref_0.offset(i as isize) as core::ffi::c_int
            - *src.offset(i as isize) as core::ffi::c_int;
        let ref mut fresh0 = *dst.offset(i as isize);
        *fresh0 = (*fresh0 as core::ffi::c_int + diff_uv) as int16_t;
        i += 1;
    }
}
unsafe extern "C" fn SharpYuvFilterRow_C(
    mut A: *const int16_t,
    mut B: *const int16_t,
    mut len: core::ffi::c_int,
    mut best_y: *const uint16_t,
    mut out: *mut uint16_t,
    mut bit_depth: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let max_y: core::ffi::c_int = ((1 as core::ffi::c_int) << bit_depth)
        - 1 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < len {
        let v0: core::ffi::c_int = *A.offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int * 9 as core::ffi::c_int
            + *A.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *B.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *B.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
        let v1: core::ffi::c_int = *A.offset(1 as core::ffi::c_int as isize)
            as core::ffi::c_int * 9 as core::ffi::c_int
            + *A.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *B.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *B.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
        *out.offset((2 as core::ffi::c_int * i + 0 as core::ffi::c_int) as isize) = clip(
            *best_y.offset((2 as core::ffi::c_int * i + 0 as core::ffi::c_int) as isize)
                as core::ffi::c_int + v0,
            max_y,
        );
        *out.offset((2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as isize) = clip(
            *best_y.offset((2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int + v1,
            max_y,
        );
        i += 1;
        A = A.offset(1);
        B = B.offset(1);
    }
}
#[no_mangle]
pub static mut SharpYuvUpdateY: Option<
    unsafe extern "C" fn(
        *const uint16_t,
        *const uint16_t,
        *mut uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> uint64_t,
> = None;
#[no_mangle]
pub static mut SharpYuvUpdateRGB: Option<
    unsafe extern "C" fn(
        *const int16_t,
        *const int16_t,
        *mut int16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut SharpYuvFilterRow: Option<
    unsafe extern "C" fn(
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        *const uint16_t,
        *mut uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn SharpYuvInitDsp() {
    SharpYuvUpdateY = Some(
        SharpYuvUpdateY_C
            as unsafe extern "C" fn(
                *const uint16_t,
                *const uint16_t,
                *mut uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> uint64_t,
    )
        as Option<
            unsafe extern "C" fn(
                *const uint16_t,
                *const uint16_t,
                *mut uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> uint64_t,
        >;
    SharpYuvUpdateRGB = Some(
        SharpYuvUpdateRGB_C
            as unsafe extern "C" fn(
                *const int16_t,
                *const int16_t,
                *mut int16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const int16_t,
                *const int16_t,
                *mut int16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    SharpYuvFilterRow = Some(
        SharpYuvFilterRow_C
            as unsafe extern "C" fn(
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                *const uint16_t,
                *mut uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                *const uint16_t,
                *mut uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    let _ = core::ptr::read(core::ptr::addr_of!(SharpYuvGetCPUInfo)).is_some();
    if core::ptr::read(core::ptr::addr_of!(SharpYuvUpdateY)).is_none() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"SharpYuvInitDsp\0"))
                .as_ptr(),
            b"sharpyuv_dsp.c\0" as *const u8 as *const core::ffi::c_char,
            102 as core::ffi::c_int,
            b"SharpYuvUpdateY != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if core::ptr::read(core::ptr::addr_of!(SharpYuvUpdateRGB)).is_none() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"SharpYuvInitDsp\0"))
                .as_ptr(),
            b"sharpyuv_dsp.c\0" as *const u8 as *const core::ffi::c_char,
            103 as core::ffi::c_int,
            b"SharpYuvUpdateRGB != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if core::ptr::read(core::ptr::addr_of!(SharpYuvFilterRow)).is_none() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"SharpYuvInitDsp\0"))
                .as_ptr(),
            b"sharpyuv_dsp.c\0" as *const u8 as *const core::ffi::c_char,
            104 as core::ffi::c_int,
            b"SharpYuvFilterRow != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
}
