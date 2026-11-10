// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Sharp RGB to YUV conversion.
//
// Author: Skal (pascal.massimino@gmail.com)

// Mechanically translated from upstream/libwebp/sharpyuv/sharpyuv.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    #[cfg_attr(sharpyuv_test_allocator, link_name = "sharp_test_malloc")]
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    #[cfg_attr(sharpyuv_test_allocator, link_name = "sharp_test_free")]
    fn free(_: *mut core::ffi::c_void);
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    static mut SharpYuvUpdateY: Option<
        unsafe extern "C" fn(
            *const uint16_t,
            *const uint16_t,
            *mut uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> uint64_t,
    >;
    static mut SharpYuvUpdateRGB: Option<
        unsafe extern "C" fn(
            *const int16_t,
            *const int16_t,
            *mut int16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut SharpYuvFilterRow: Option<
        unsafe extern "C" fn(
            *const int16_t,
            *const int16_t,
            core::ffi::c_int,
            *const uint16_t,
            *mut uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    fn SharpYuvInitDsp();
    fn SharpYuvInitGammaTables();
    fn SharpYuvGammaToLinear(
        v: uint16_t,
        bit_depth: core::ffi::c_int,
        transfer_type: SharpYuvTransferFunctionType,
    ) -> uint32_t;
    fn SharpYuvLinearToGamma(
        value: uint32_t,
        bit_depth: core::ffi::c_int,
        transfer_type: SharpYuvTransferFunctionType,
    ) -> uint16_t;
    static mut SharpYuvGetCPUInfo: VP8CPUInfo;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [core::ffi::c_int; 4],
    pub rgb_to_u: [core::ffi::c_int; 4],
    pub rgb_to_v: [core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvOptions {
    pub yuv_matrix: *const SharpYuvConversionMatrix,
    pub transfer_type: SharpYuvTransferFunctionType,
}
pub type SharpYuvTransferFunctionType = core::ffi::c_uint;
pub const kSharpYuvTransferFunctionNum: SharpYuvTransferFunctionType = 19;
pub const kSharpYuvTransferFunctionHlg: SharpYuvTransferFunctionType = 18;
pub const kSharpYuvTransferFunctionSmpte428: SharpYuvTransferFunctionType = 17;
pub const kSharpYuvTransferFunctionSmpte2084: SharpYuvTransferFunctionType = 16;
pub const kSharpYuvTransferFunctionBt2020_12Bit: SharpYuvTransferFunctionType = 15;
pub const kSharpYuvTransferFunctionBt2020_10Bit: SharpYuvTransferFunctionType = 14;
pub const kSharpYuvTransferFunctionSrgb: SharpYuvTransferFunctionType = 13;
pub const kSharpYuvTransferFunctionBt1361: SharpYuvTransferFunctionType = 12;
pub const kSharpYuvTransferFunctionIec61966: SharpYuvTransferFunctionType = 11;
pub const kSharpYuvTransferFunctionLog100_Sqrt10: SharpYuvTransferFunctionType = 10;
pub const kSharpYuvTransferFunctionLog100: SharpYuvTransferFunctionType = 9;
pub const kSharpYuvTransferFunctionLinear: SharpYuvTransferFunctionType = 8;
pub const kSharpYuvTransferFunctionSmpte240: SharpYuvTransferFunctionType = 7;
pub const kSharpYuvTransferFunctionBt601: SharpYuvTransferFunctionType = 6;
pub const kSharpYuvTransferFunctionBt470Bg: SharpYuvTransferFunctionType = 5;
pub const kSharpYuvTransferFunctionBt470M: SharpYuvTransferFunctionType = 4;
pub const kSharpYuvTransferFunctionBt709: SharpYuvTransferFunctionType = 1;
pub type uint8_t = u8;
pub type fixed_y_t = uint16_t;
pub type uint16_t = u16;
pub type uint64_t = u64;
pub type size_t = usize;
pub type fixed_t = int16_t;
pub type int16_t = i16;
pub type int64_t = i64;
pub type uint32_t = u32;
pub type VP8CPUInfo = *const core::ffi::c_void;
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
pub type pthread_mutex_t = libc::pthread_mutex_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub const SHARPYUV_VERSION_MAJOR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SHARPYUV_VERSION_MINOR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SHARPYUV_VERSION: core::ffi::c_int = (0 as core::ffi::c_int)
    << 24 as core::ffi::c_int | (4 as core::ffi::c_int) << 16 as core::ffi::c_int
    | 2 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const _PTHREAD_MUTEX_SIG_init: core::ffi::c_int = 0x32aaaba7 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn SharpYuvGetVersion() -> core::ffi::c_int {
    return SHARPYUV_VERSION;
}
static mut kNumIterations: core::ffi::c_int = 4 as core::ffi::c_int;
pub const YUV_FIX: core::ffi::c_int = 16 as core::ffi::c_int;
static mut kYuvHalf: core::ffi::c_int = (1 as core::ffi::c_int)
    << YUV_FIX - 1 as core::ffi::c_int;
static mut kMaxBitDepth: core::ffi::c_int = 14 as core::ffi::c_int;
unsafe extern "C" fn GetPrecisionShift(
    mut rgb_bit_depth: core::ffi::c_int,
) -> core::ffi::c_int {
    return if rgb_bit_depth + 2 as core::ffi::c_int <= kMaxBitDepth {
        2 as core::ffi::c_int
    } else {
        kMaxBitDepth - rgb_bit_depth
    };
}
unsafe extern "C" fn clip_8b(mut v: fixed_t) -> uint8_t {
    return (if v as core::ffi::c_int & !(0xff as core::ffi::c_int) == 0 {
        v as uint8_t as core::ffi::c_uint
    } else if (v as core::ffi::c_int) < 0 as core::ffi::c_int {
        0 as core::ffi::c_uint
    } else {
        255 as core::ffi::c_uint
    }) as uint8_t;
}
unsafe extern "C" fn clip(mut v: fixed_t, mut max: core::ffi::c_int) -> uint16_t {
    return (if (v as core::ffi::c_int) < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else if v as core::ffi::c_int > max {
        max
    } else {
        v as uint16_t as core::ffi::c_int
    }) as uint16_t;
}
unsafe extern "C" fn clip_bit_depth(
    mut y: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) -> fixed_y_t {
    let max: core::ffi::c_int = ((1 as core::ffi::c_int) << bit_depth)
        - 1 as core::ffi::c_int;
    return (if y & !max == 0 {
        y as fixed_y_t as core::ffi::c_int
    } else if y < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        max
    }) as fixed_y_t;
}
unsafe extern "C" fn RGBToGray(
    mut r: int64_t,
    mut g: int64_t,
    mut b: int64_t,
) -> core::ffi::c_int {
    let luma: int64_t = 13933 as int64_t * r + 46871 as int64_t * g + 4732 as int64_t * b
        + kYuvHalf as int64_t;
    return (luma >> YUV_FIX) as core::ffi::c_int;
}
unsafe extern "C" fn ScaleDown(
    mut a: uint16_t,
    mut b: uint16_t,
    mut c: uint16_t,
    mut d: uint16_t,
    mut rgb_bit_depth: core::ffi::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) -> uint32_t {
    let bit_depth: core::ffi::c_int = rgb_bit_depth
        + GetPrecisionShift(rgb_bit_depth) as core::ffi::c_int;
    let A: uint32_t = SharpYuvGammaToLinear(a, bit_depth, transfer_type) as uint32_t;
    let B: uint32_t = SharpYuvGammaToLinear(b, bit_depth, transfer_type) as uint32_t;
    let C: uint32_t = SharpYuvGammaToLinear(c, bit_depth, transfer_type) as uint32_t;
    let D: uint32_t = SharpYuvGammaToLinear(d, bit_depth, transfer_type) as uint32_t;
    return SharpYuvLinearToGamma(
        A.wrapping_add(B).wrapping_add(C).wrapping_add(D).wrapping_add(2 as uint32_t)
            >> 2 as core::ffi::c_int,
        bit_depth,
        transfer_type,
    ) as uint32_t;
}
#[inline]
unsafe extern "C" fn UpdateW(
    mut src: *const fixed_y_t,
    mut dst: *mut fixed_y_t,
    mut w: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) {
    let bit_depth: core::ffi::c_int = rgb_bit_depth
        + GetPrecisionShift(rgb_bit_depth) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let R: uint32_t = SharpYuvGammaToLinear(
            *src.offset((0 as core::ffi::c_int * w + i) as isize),
            bit_depth,
            transfer_type,
        ) as uint32_t;
        let G: uint32_t = SharpYuvGammaToLinear(
            *src.offset((1 as core::ffi::c_int * w + i) as isize),
            bit_depth,
            transfer_type,
        ) as uint32_t;
        let B: uint32_t = SharpYuvGammaToLinear(
            *src.offset((2 as core::ffi::c_int * w + i) as isize),
            bit_depth,
            transfer_type,
        ) as uint32_t;
        let Y: uint32_t = RGBToGray(R as int64_t, G as int64_t, B as int64_t)
            as uint32_t;
        *dst.offset(i as isize) = SharpYuvLinearToGamma(Y, bit_depth, transfer_type);
        i += 1;
        if !(i < w) {
            break;
        }
    };
}
unsafe extern "C" fn UpdateChroma(
    mut src1: *const fixed_y_t,
    mut src2: *const fixed_y_t,
    mut dst: *mut fixed_t,
    mut uv_w: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let r: core::ffi::c_int = ScaleDown(
            *src1
                .offset((0 as core::ffi::c_int * uv_w + 0 as core::ffi::c_int) as isize),
            *src1
                .offset((0 as core::ffi::c_int * uv_w + 1 as core::ffi::c_int) as isize),
            *src2
                .offset((0 as core::ffi::c_int * uv_w + 0 as core::ffi::c_int) as isize),
            *src2
                .offset((0 as core::ffi::c_int * uv_w + 1 as core::ffi::c_int) as isize),
            rgb_bit_depth,
            transfer_type,
        ) as core::ffi::c_int;
        let g: core::ffi::c_int = ScaleDown(
            *src1
                .offset((2 as core::ffi::c_int * uv_w + 0 as core::ffi::c_int) as isize),
            *src1
                .offset((2 as core::ffi::c_int * uv_w + 1 as core::ffi::c_int) as isize),
            *src2
                .offset((2 as core::ffi::c_int * uv_w + 0 as core::ffi::c_int) as isize),
            *src2
                .offset((2 as core::ffi::c_int * uv_w + 1 as core::ffi::c_int) as isize),
            rgb_bit_depth,
            transfer_type,
        ) as core::ffi::c_int;
        let b: core::ffi::c_int = ScaleDown(
            *src1
                .offset((4 as core::ffi::c_int * uv_w + 0 as core::ffi::c_int) as isize),
            *src1
                .offset((4 as core::ffi::c_int * uv_w + 1 as core::ffi::c_int) as isize),
            *src2
                .offset((4 as core::ffi::c_int * uv_w + 0 as core::ffi::c_int) as isize),
            *src2
                .offset((4 as core::ffi::c_int * uv_w + 1 as core::ffi::c_int) as isize),
            rgb_bit_depth,
            transfer_type,
        ) as core::ffi::c_int;
        let W: core::ffi::c_int = RGBToGray(r as int64_t, g as int64_t, b as int64_t)
            as core::ffi::c_int;
        *dst.offset((0 as core::ffi::c_int * uv_w) as isize) = (r - W) as fixed_t;
        *dst.offset((1 as core::ffi::c_int * uv_w) as isize) = (g - W) as fixed_t;
        *dst.offset((2 as core::ffi::c_int * uv_w) as isize) = (b - W) as fixed_t;
        dst = dst.offset(1 as core::ffi::c_int as isize);
        src1 = src1.offset(2 as core::ffi::c_int as isize);
        src2 = src2.offset(2 as core::ffi::c_int as isize);
        i += 1;
        if !(i < uv_w) {
            break;
        }
    };
}
unsafe extern "C" fn StoreGray(
    mut rgb: *const fixed_y_t,
    mut y: *mut fixed_y_t,
    mut w: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    if !(w > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 10],
                [core::ffi::c_char; 10],
            >(*b"StoreGray\0"))
                .as_ptr(),
            b"sharpyuv.c\0" as *const u8 as *const core::ffi::c_char,
            132 as core::ffi::c_int,
            b"w > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    loop {
        *y.offset(i as isize) = RGBToGray(
            *rgb.offset((0 as core::ffi::c_int * w + i) as isize) as int64_t,
            *rgb.offset((1 as core::ffi::c_int * w + i) as isize) as int64_t,
            *rgb.offset((2 as core::ffi::c_int * w + i) as isize) as int64_t,
        ) as fixed_y_t;
        i += 1;
        if !(i < w) {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn Filter2(
    mut A: core::ffi::c_int,
    mut B: core::ffi::c_int,
    mut W0: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) -> fixed_y_t {
    let v0: core::ffi::c_int = A * 3 as core::ffi::c_int + B + 2 as core::ffi::c_int
        >> 2 as core::ffi::c_int;
    return clip_bit_depth(v0 + W0, bit_depth);
}
#[inline]
unsafe extern "C" fn Shift(
    mut v: core::ffi::c_int,
    mut shift: core::ffi::c_int,
) -> core::ffi::c_int {
    return if shift >= 0 as core::ffi::c_int { v << shift } else { v >> -shift };
}
unsafe extern "C" fn ImportOneRow(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    mut rgb_step: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut pic_width: core::ffi::c_int,
    dst: *mut fixed_y_t,
) {
    let step: core::ffi::c_int = if rgb_bit_depth > 8 as core::ffi::c_int {
        rgb_step / 2 as core::ffi::c_int
    } else {
        rgb_step
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let w: core::ffi::c_int = pic_width + 1 as core::ffi::c_int
        & !(1 as core::ffi::c_int);
    loop {
        let off: core::ffi::c_int = i * step;
        let shift: core::ffi::c_int = GetPrecisionShift(rgb_bit_depth)
            as core::ffi::c_int;
        if rgb_bit_depth == 8 as core::ffi::c_int {
            *dst.offset((i + 0 as core::ffi::c_int * w) as isize) = Shift(
                *r_ptr.offset(off as isize) as core::ffi::c_int,
                shift,
            ) as fixed_y_t;
            *dst.offset((i + 1 as core::ffi::c_int * w) as isize) = Shift(
                *g_ptr.offset(off as isize) as core::ffi::c_int,
                shift,
            ) as fixed_y_t;
            *dst.offset((i + 2 as core::ffi::c_int * w) as isize) = Shift(
                *b_ptr.offset(off as isize) as core::ffi::c_int,
                shift,
            ) as fixed_y_t;
        } else {
            *dst.offset((i + 0 as core::ffi::c_int * w) as isize) = Shift(
                *(r_ptr as *mut uint16_t).offset(off as isize) as core::ffi::c_int,
                shift,
            ) as fixed_y_t;
            *dst.offset((i + 1 as core::ffi::c_int * w) as isize) = Shift(
                *(g_ptr as *mut uint16_t).offset(off as isize) as core::ffi::c_int,
                shift,
            ) as fixed_y_t;
            *dst.offset((i + 2 as core::ffi::c_int * w) as isize) = Shift(
                *(b_ptr as *mut uint16_t).offset(off as isize) as core::ffi::c_int,
                shift,
            ) as fixed_y_t;
        }
        i += 1;
        if !(i < pic_width) {
            break;
        }
    }
    if pic_width & 1 as core::ffi::c_int != 0 {
        *dst.offset((pic_width + 0 as core::ffi::c_int * w) as isize) = *dst
            .offset(
                (pic_width + 0 as core::ffi::c_int * w - 1 as core::ffi::c_int) as isize,
            );
        *dst.offset((pic_width + 1 as core::ffi::c_int * w) as isize) = *dst
            .offset(
                (pic_width + 1 as core::ffi::c_int * w - 1 as core::ffi::c_int) as isize,
            );
        *dst.offset((pic_width + 2 as core::ffi::c_int * w) as isize) = *dst
            .offset(
                (pic_width + 2 as core::ffi::c_int * w - 1 as core::ffi::c_int) as isize,
            );
    }
}
unsafe extern "C" fn InterpolateTwoRows(
    best_y: *const fixed_y_t,
    mut prev_uv: *const fixed_t,
    mut cur_uv: *const fixed_t,
    mut next_uv: *const fixed_t,
    mut w: core::ffi::c_int,
    mut out1: *mut fixed_y_t,
    mut out2: *mut fixed_y_t,
    mut rgb_bit_depth: core::ffi::c_int,
) {
    let uv_w: core::ffi::c_int = w >> 1 as core::ffi::c_int;
    let len: core::ffi::c_int = w - 1 as core::ffi::c_int >> 1 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 3 as core::ffi::c_int;
    let bit_depth: core::ffi::c_int = rgb_bit_depth
        + GetPrecisionShift(rgb_bit_depth) as core::ffi::c_int;
    loop {
        let fresh0 = k;
        k = k - 1;
        if !(fresh0 > 0 as core::ffi::c_int) {
            break;
        }
        *out1.offset(0 as core::ffi::c_int as isize) = Filter2(
            *cur_uv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            *prev_uv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            *best_y.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            bit_depth,
        );
        *out2.offset(0 as core::ffi::c_int as isize) = Filter2(
            *cur_uv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            *next_uv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            *best_y.offset(w as isize) as core::ffi::c_int,
            bit_depth,
        );
        SharpYuvFilterRow
            .expect(
                "non-null function pointer",
            )(
            cur_uv as *const int16_t,
            prev_uv as *const int16_t,
            len,
            best_y
                .offset(0 as core::ffi::c_int as isize)
                .offset(1 as core::ffi::c_int as isize),
            out1.offset(1 as core::ffi::c_int as isize),
            bit_depth,
        );
        SharpYuvFilterRow
            .expect(
                "non-null function pointer",
            )(
            cur_uv as *const int16_t,
            next_uv as *const int16_t,
            len,
            best_y.offset(w as isize).offset(1 as core::ffi::c_int as isize),
            out2.offset(1 as core::ffi::c_int as isize),
            bit_depth,
        );
        if w & 1 as core::ffi::c_int == 0 {
            *out1.offset((w - 1 as core::ffi::c_int) as isize) = Filter2(
                *cur_uv.offset((uv_w - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                *prev_uv.offset((uv_w - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                *best_y
                    .offset((w - 1 as core::ffi::c_int + 0 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                bit_depth,
            );
            *out2.offset((w - 1 as core::ffi::c_int) as isize) = Filter2(
                *cur_uv.offset((uv_w - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                *next_uv.offset((uv_w - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                *best_y.offset((w - 1 as core::ffi::c_int + w) as isize)
                    as core::ffi::c_int,
                bit_depth,
            );
        }
        out1 = out1.offset(w as isize);
        out2 = out2.offset(w as isize);
        prev_uv = prev_uv.offset(uv_w as isize);
        cur_uv = cur_uv.offset(uv_w as isize);
        next_uv = next_uv.offset(uv_w as isize);
    };
}
#[inline]
unsafe extern "C" fn RGBToYUVComponent(
    mut r: core::ffi::c_int,
    mut g: core::ffi::c_int,
    mut b: core::ffi::c_int,
    mut coeffs: *const core::ffi::c_int,
    mut sfix: core::ffi::c_int,
) -> core::ffi::c_int {
    let srounder: core::ffi::c_int = (1 as core::ffi::c_int)
        << YUV_FIX + sfix - 1 as core::ffi::c_int;
    let luma: core::ffi::c_int = *coeffs.offset(0 as core::ffi::c_int as isize) * r
        + *coeffs.offset(1 as core::ffi::c_int as isize) * g
        + *coeffs.offset(2 as core::ffi::c_int as isize) * b
        + *coeffs.offset(3 as core::ffi::c_int as isize) + srounder;
    return luma >> YUV_FIX + sfix;
}
unsafe extern "C" fn ConvertWRGBToYUV(
    mut best_y: *const fixed_y_t,
    mut best_uv: *const fixed_t,
    mut y_ptr: *mut uint8_t,
    mut y_stride: core::ffi::c_int,
    mut u_ptr: *mut uint8_t,
    mut u_stride: core::ffi::c_int,
    mut v_ptr: *mut uint8_t,
    mut v_stride: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut yuv_bit_depth: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut yuv_matrix: *const SharpYuvConversionMatrix,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let best_uv_base: *const fixed_t = best_uv;
    let w: core::ffi::c_int = width + 1 as core::ffi::c_int & !(1 as core::ffi::c_int);
    let h: core::ffi::c_int = height + 1 as core::ffi::c_int & !(1 as core::ffi::c_int);
    let uv_w: core::ffi::c_int = w >> 1 as core::ffi::c_int;
    let uv_h: core::ffi::c_int = h >> 1 as core::ffi::c_int;
    let sfix: core::ffi::c_int = GetPrecisionShift(rgb_bit_depth) as core::ffi::c_int;
    let yuv_max: core::ffi::c_int = ((1 as core::ffi::c_int) << yuv_bit_depth)
        - 1 as core::ffi::c_int;
    best_uv = best_uv_base;
    j = 0 as core::ffi::c_int;
    loop {
        i = 0 as core::ffi::c_int;
        loop {
            let off: core::ffi::c_int = i >> 1 as core::ffi::c_int;
            let W: core::ffi::c_int = *best_y.offset(i as isize) as core::ffi::c_int;
            let r: core::ffi::c_int = *best_uv
                .offset((off + 0 as core::ffi::c_int * uv_w) as isize)
                as core::ffi::c_int + W;
            let g: core::ffi::c_int = *best_uv
                .offset((off + 1 as core::ffi::c_int * uv_w) as isize)
                as core::ffi::c_int + W;
            let b: core::ffi::c_int = *best_uv
                .offset((off + 2 as core::ffi::c_int * uv_w) as isize)
                as core::ffi::c_int + W;
            let y: core::ffi::c_int = RGBToYUVComponent(
                r,
                g,
                b,
                ((*yuv_matrix).rgb_to_y).as_ptr(),
                sfix,
            ) as core::ffi::c_int;
            if yuv_bit_depth <= 8 as core::ffi::c_int {
                *y_ptr.offset(i as isize) = clip_8b(y as fixed_t);
            } else {
                *(y_ptr as *mut uint16_t).offset(i as isize) = clip(
                    y as fixed_t,
                    yuv_max,
                );
            }
            i += 1;
            if !(i < width) {
                break;
            }
        }
        best_y = best_y.offset(w as isize);
        best_uv = best_uv
            .offset(
                ((j & 1 as core::ffi::c_int) * 3 as core::ffi::c_int * uv_w) as isize,
            );
        y_ptr = y_ptr.offset(y_stride as isize);
        j += 1;
        if !(j < height) {
            break;
        }
    }
    best_uv = best_uv_base;
    j = 0 as core::ffi::c_int;
    loop {
        i = 0 as core::ffi::c_int;
        loop {
            let r_0: core::ffi::c_int = *best_uv
                .offset((i + 0 as core::ffi::c_int * uv_w) as isize) as core::ffi::c_int;
            let g_0: core::ffi::c_int = *best_uv
                .offset((i + 1 as core::ffi::c_int * uv_w) as isize) as core::ffi::c_int;
            let b_0: core::ffi::c_int = *best_uv
                .offset((i + 2 as core::ffi::c_int * uv_w) as isize) as core::ffi::c_int;
            let u: core::ffi::c_int = RGBToYUVComponent(
                r_0,
                g_0,
                b_0,
                ((*yuv_matrix).rgb_to_u).as_ptr(),
                sfix,
            ) as core::ffi::c_int;
            let v: core::ffi::c_int = RGBToYUVComponent(
                r_0,
                g_0,
                b_0,
                ((*yuv_matrix).rgb_to_v).as_ptr(),
                sfix,
            ) as core::ffi::c_int;
            if yuv_bit_depth <= 8 as core::ffi::c_int {
                *u_ptr.offset(i as isize) = clip_8b(u as fixed_t);
                *v_ptr.offset(i as isize) = clip_8b(v as fixed_t);
            } else {
                *(u_ptr as *mut uint16_t).offset(i as isize) = clip(
                    u as fixed_t,
                    yuv_max,
                );
                *(v_ptr as *mut uint16_t).offset(i as isize) = clip(
                    v as fixed_t,
                    yuv_max,
                );
            }
            i += 1;
            if !(i < uv_w) {
                break;
            }
        }
        best_uv = best_uv.offset((3 as core::ffi::c_int * uv_w) as isize);
        u_ptr = u_ptr.offset(u_stride as isize);
        v_ptr = v_ptr.offset(v_stride as isize);
        j += 1;
        if !(j < uv_h) {
            break;
        }
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C" fn SafeMalloc(
    mut nmemb: uint64_t,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    let total_size: uint64_t = nmemb.wrapping_mul(size as uint64_t);
    if total_size != total_size as size_t as uint64_t {
        return NULL;
    }
    return malloc(total_size as size_t);
}
unsafe extern "C" fn DoSharpArgbToYuv(
    mut r_ptr: *const uint8_t,
    mut g_ptr: *const uint8_t,
    mut b_ptr: *const uint8_t,
    mut rgb_step: core::ffi::c_int,
    mut rgb_stride: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut y_ptr: *mut uint8_t,
    mut y_stride: core::ffi::c_int,
    mut u_ptr: *mut uint8_t,
    mut u_stride: core::ffi::c_int,
    mut v_ptr: *mut uint8_t,
    mut v_stride: core::ffi::c_int,
    mut yuv_bit_depth: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut yuv_matrix: *const SharpYuvConversionMatrix,
    mut transfer_type: SharpYuvTransferFunctionType,
) -> core::ffi::c_int {
    let w: core::ffi::c_int = width + 1 as core::ffi::c_int & !(1 as core::ffi::c_int);
    let h: core::ffi::c_int = height + 1 as core::ffi::c_int & !(1 as core::ffi::c_int);
    let uv_w: core::ffi::c_int = w >> 1 as core::ffi::c_int;
    let uv_h: core::ffi::c_int = h >> 1 as core::ffi::c_int;
    let y_bit_depth: core::ffi::c_int = rgb_bit_depth
        + GetPrecisionShift(rgb_bit_depth) as core::ffi::c_int;
    let mut prev_diff_y_sum: uint64_t = !(0 as core::ffi::c_int) as uint64_t;
    let mut j: core::ffi::c_int = 0;
    let mut iter: core::ffi::c_int = 0;
    let tmp_buffer: *mut fixed_y_t = SafeMalloc(
        ((w * 3 as core::ffi::c_int) as uint64_t).wrapping_mul(2 as uint64_t),
        ::core::mem::size_of::<fixed_y_t>() as size_t,
    ) as *mut fixed_y_t;
    let best_y_base: *mut fixed_y_t = SafeMalloc(
        (w as uint64_t).wrapping_mul(h as uint64_t),
        ::core::mem::size_of::<fixed_y_t>() as size_t,
    ) as *mut fixed_y_t;
    let target_y_base: *mut fixed_y_t = SafeMalloc(
        (w as uint64_t).wrapping_mul(h as uint64_t),
        ::core::mem::size_of::<fixed_y_t>() as size_t,
    ) as *mut fixed_y_t;
    let best_rgb_y: *mut fixed_y_t = SafeMalloc(
        (w as uint64_t).wrapping_mul(2 as uint64_t),
        ::core::mem::size_of::<fixed_y_t>() as size_t,
    ) as *mut fixed_y_t;
    let best_uv_base: *mut fixed_t = SafeMalloc(
        ((uv_w * 3 as core::ffi::c_int) as uint64_t).wrapping_mul(uv_h as uint64_t),
        ::core::mem::size_of::<fixed_t>() as size_t,
    ) as *mut fixed_t;
    let target_uv_base: *mut fixed_t = SafeMalloc(
        ((uv_w * 3 as core::ffi::c_int) as uint64_t).wrapping_mul(uv_h as uint64_t),
        ::core::mem::size_of::<fixed_t>() as size_t,
    ) as *mut fixed_t;
    let best_rgb_uv: *mut fixed_t = SafeMalloc(
        ((uv_w * 3 as core::ffi::c_int) as uint64_t).wrapping_mul(1 as uint64_t),
        ::core::mem::size_of::<fixed_t>() as size_t,
    ) as *mut fixed_t;
    let mut best_y: *mut fixed_y_t = best_y_base;
    let mut target_y: *mut fixed_y_t = target_y_base;
    let mut best_uv: *mut fixed_t = best_uv_base;
    let mut target_uv: *mut fixed_t = target_uv_base;
    let diff_y_threshold: uint64_t = (3.0f64 * w as core::ffi::c_double
        * h as core::ffi::c_double) as uint64_t;
    let mut ok: core::ffi::c_int = 0;
    if !(w > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"DoSharpArgbToYuv\0"))
                .as_ptr(),
            b"sharpyuv.c\0" as *const u8 as *const core::ffi::c_char,
            335 as core::ffi::c_int,
            b"w > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(h > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"DoSharpArgbToYuv\0"))
                .as_ptr(),
            b"sharpyuv.c\0" as *const u8 as *const core::ffi::c_char,
            336 as core::ffi::c_int,
            b"h > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if best_y_base.is_null() || best_uv_base.is_null() || target_y_base.is_null()
        || target_uv_base.is_null() || best_rgb_y.is_null() || best_rgb_uv.is_null()
        || tmp_buffer.is_null()
    {
        ok = 0 as core::ffi::c_int;
    } else {
        j = 0 as core::ffi::c_int;
        while j < height {
            let is_last_row: core::ffi::c_int = (j == height - 1 as core::ffi::c_int)
                as core::ffi::c_int;
            let src1: *mut fixed_y_t = tmp_buffer
                .offset((0 as core::ffi::c_int * w) as isize);
            let src2: *mut fixed_y_t = tmp_buffer
                .offset((3 as core::ffi::c_int * w) as isize);
            ImportOneRow(r_ptr, g_ptr, b_ptr, rgb_step, rgb_bit_depth, width, src1);
            if is_last_row == 0 {
                ImportOneRow(
                    r_ptr.offset(rgb_stride as isize),
                    g_ptr.offset(rgb_stride as isize),
                    b_ptr.offset(rgb_stride as isize),
                    rgb_step,
                    rgb_bit_depth,
                    width,
                    src2,
                );
            } else {
                memcpy(
                    src2 as *mut core::ffi::c_void,
                    src1 as *const core::ffi::c_void,
                    ((3 as core::ffi::c_int * w) as size_t)
                        .wrapping_mul(::core::mem::size_of::<fixed_y_t>() as size_t),
                );
            }
            StoreGray(src1, best_y.offset(0 as core::ffi::c_int as isize), w);
            StoreGray(src2, best_y.offset(w as isize), w);
            UpdateW(src1, target_y, w, rgb_bit_depth, transfer_type);
            UpdateW(src2, target_y.offset(w as isize), w, rgb_bit_depth, transfer_type);
            UpdateChroma(src1, src2, target_uv, uv_w, rgb_bit_depth, transfer_type);
            memcpy(
                best_uv as *mut core::ffi::c_void,
                target_uv as *const core::ffi::c_void,
                ((3 as core::ffi::c_int * uv_w) as size_t)
                    .wrapping_mul(::core::mem::size_of::<fixed_t>() as size_t),
            );
            best_y = best_y.offset((2 as core::ffi::c_int * w) as isize);
            best_uv = best_uv.offset((3 as core::ffi::c_int * uv_w) as isize);
            target_y = target_y.offset((2 as core::ffi::c_int * w) as isize);
            target_uv = target_uv.offset((3 as core::ffi::c_int * uv_w) as isize);
            r_ptr = r_ptr.offset((2 as core::ffi::c_int * rgb_stride) as isize);
            g_ptr = g_ptr.offset((2 as core::ffi::c_int * rgb_stride) as isize);
            b_ptr = b_ptr.offset((2 as core::ffi::c_int * rgb_stride) as isize);
            j += 2 as core::ffi::c_int;
        }
        iter = 0 as core::ffi::c_int;
        while iter < kNumIterations {
            let mut cur_uv: *const fixed_t = best_uv_base;
            let mut prev_uv: *const fixed_t = best_uv_base;
            let mut diff_y_sum: uint64_t = 0 as uint64_t;
            best_y = best_y_base;
            best_uv = best_uv_base;
            target_y = target_y_base;
            target_uv = target_uv_base;
            j = 0 as core::ffi::c_int;
            loop {
                let src1_0: *mut fixed_y_t = tmp_buffer
                    .offset((0 as core::ffi::c_int * w) as isize);
                let src2_0: *mut fixed_y_t = tmp_buffer
                    .offset((3 as core::ffi::c_int * w) as isize);
                let next_uv: *const fixed_t = cur_uv
                    .offset(
                        (if j < h - 2 as core::ffi::c_int {
                            3 as core::ffi::c_int * uv_w
                        } else {
                            0 as core::ffi::c_int
                        }) as isize,
                    );
                InterpolateTwoRows(
                    best_y,
                    prev_uv,
                    cur_uv,
                    next_uv,
                    w,
                    src1_0,
                    src2_0,
                    rgb_bit_depth,
                );
                prev_uv = cur_uv;
                cur_uv = next_uv;
                UpdateW(
                    src1_0,
                    best_rgb_y.offset((0 as core::ffi::c_int * w) as isize),
                    w,
                    rgb_bit_depth,
                    transfer_type,
                );
                UpdateW(
                    src2_0,
                    best_rgb_y.offset((1 as core::ffi::c_int * w) as isize),
                    w,
                    rgb_bit_depth,
                    transfer_type,
                );
                UpdateChroma(
                    src1_0,
                    src2_0,
                    best_rgb_uv,
                    uv_w,
                    rgb_bit_depth,
                    transfer_type,
                );
                diff_y_sum = diff_y_sum
                    .wrapping_add(
                        SharpYuvUpdateY
                            .expect(
                                "non-null function pointer",
                            )(
                            target_y,
                            best_rgb_y,
                            best_y as *mut uint16_t,
                            2 as core::ffi::c_int * w,
                            y_bit_depth,
                        ),
                    );
                SharpYuvUpdateRGB
                    .expect(
                        "non-null function pointer",
                    )(
                    target_uv,
                    best_rgb_uv,
                    best_uv as *mut int16_t,
                    3 as core::ffi::c_int * uv_w,
                );
                best_y = best_y.offset((2 as core::ffi::c_int * w) as isize);
                best_uv = best_uv.offset((3 as core::ffi::c_int * uv_w) as isize);
                target_y = target_y.offset((2 as core::ffi::c_int * w) as isize);
                target_uv = target_uv.offset((3 as core::ffi::c_int * uv_w) as isize);
                j += 2 as core::ffi::c_int;
                if !(j < h) {
                    break;
                }
            }
            if iter > 0 as core::ffi::c_int {
                if diff_y_sum < diff_y_threshold {
                    break;
                }
                if diff_y_sum > prev_diff_y_sum {
                    break;
                }
            }
            prev_diff_y_sum = diff_y_sum;
            iter += 1;
        }
        ok = ConvertWRGBToYUV(
            best_y_base,
            best_uv_base,
            y_ptr,
            y_stride,
            u_ptr,
            u_stride,
            v_ptr,
            v_stride,
            rgb_bit_depth,
            yuv_bit_depth,
            width,
            height,
            yuv_matrix,
        );
    }
    free(best_y_base as *mut core::ffi::c_void);
    free(best_uv_base as *mut core::ffi::c_void);
    free(target_y_base as *mut core::ffi::c_void);
    free(target_uv_base as *mut core::ffi::c_void);
    free(best_rgb_y as *mut core::ffi::c_void);
    free(best_rgb_uv as *mut core::ffi::c_void);
    free(tmp_buffer as *mut core::ffi::c_void);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvInit(mut cpu_info_func: VP8CPUInfo) {
    static mut sharpyuv_last_cpuinfo_used: VP8CPUInfo = unsafe {
        ::core::mem::transmute::<
            *mut VP8CPUInfo,
            VP8CPUInfo,
        >(core::ptr::addr_of!(sharpyuv_last_cpuinfo_used) as *const VP8CPUInfo as *mut VP8CPUInfo)
    };
    static mut sharpyuv_lock: pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;
    if pthread_mutex_lock(core::ptr::addr_of_mut!(sharpyuv_lock)) != 0 {
        return;
    }
    if cpu_info_func
        != ::core::mem::transmute::<
            *mut VP8CPUInfo,
            VP8CPUInfo,
        >(core::ptr::addr_of_mut!(SharpYuvGetCPUInfo) as *mut VP8CPUInfo)
    {
        SharpYuvGetCPUInfo = cpu_info_func;
    }
    if sharpyuv_last_cpuinfo_used == SharpYuvGetCPUInfo {
        pthread_mutex_unlock(core::ptr::addr_of_mut!(sharpyuv_lock));
        return;
    }
    SharpYuvInitDsp();
    SharpYuvInitGammaTables();
    ::core::ptr::write_volatile(
        core::ptr::addr_of_mut!(sharpyuv_last_cpuinfo_used) as *mut VP8CPUInfo,
        SharpYuvGetCPUInfo,
    );
    pthread_mutex_unlock(core::ptr::addr_of_mut!(sharpyuv_lock));
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvConvert(
    mut r_ptr: *const core::ffi::c_void,
    mut g_ptr: *const core::ffi::c_void,
    mut b_ptr: *const core::ffi::c_void,
    mut rgb_step: core::ffi::c_int,
    mut rgb_stride: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut y_ptr: *mut core::ffi::c_void,
    mut y_stride: core::ffi::c_int,
    mut u_ptr: *mut core::ffi::c_void,
    mut u_stride: core::ffi::c_int,
    mut v_ptr: *mut core::ffi::c_void,
    mut v_stride: core::ffi::c_int,
    mut yuv_bit_depth: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut yuv_matrix: *const SharpYuvConversionMatrix,
) -> core::ffi::c_int {
    let mut options: SharpYuvOptions = SharpYuvOptions {
        yuv_matrix: 0 as *const SharpYuvConversionMatrix,
        transfer_type: 0 as SharpYuvTransferFunctionType,
    };
    options.yuv_matrix = yuv_matrix;
    options.transfer_type = kSharpYuvTransferFunctionSrgb;
    return SharpYuvConvertWithOptions(
        r_ptr,
        g_ptr,
        b_ptr,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_stride,
        u_ptr,
        u_stride,
        v_ptr,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &mut options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvOptionsInitInternal(
    mut yuv_matrix: *const SharpYuvConversionMatrix,
    mut options: *mut SharpYuvOptions,
    mut version: core::ffi::c_int,
) -> core::ffi::c_int {
    let major: core::ffi::c_int = version >> 24 as core::ffi::c_int;
    let minor: core::ffi::c_int = version >> 16 as core::ffi::c_int
        & 0xff as core::ffi::c_int;
    if options.is_null() || yuv_matrix.is_null()
        || major == SHARPYUV_VERSION_MAJOR && major == 0 as core::ffi::c_int
            && minor != SHARPYUV_VERSION_MINOR || major != SHARPYUV_VERSION_MAJOR
    {
        return 0 as core::ffi::c_int;
    }
    (*options).yuv_matrix = yuv_matrix;
    (*options).transfer_type = kSharpYuvTransferFunctionSrgb;
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvConvertWithOptions(
    mut r_ptr: *const core::ffi::c_void,
    mut g_ptr: *const core::ffi::c_void,
    mut b_ptr: *const core::ffi::c_void,
    mut rgb_step: core::ffi::c_int,
    mut rgb_stride: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut y_ptr: *mut core::ffi::c_void,
    mut y_stride: core::ffi::c_int,
    mut u_ptr: *mut core::ffi::c_void,
    mut u_stride: core::ffi::c_int,
    mut v_ptr: *mut core::ffi::c_void,
    mut v_stride: core::ffi::c_int,
    mut yuv_bit_depth: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut options: *const SharpYuvOptions,
) -> core::ffi::c_int {
    let mut yuv_matrix: *const SharpYuvConversionMatrix = (*options).yuv_matrix;
    let mut transfer_type: SharpYuvTransferFunctionType = (*options).transfer_type;
    let mut scaled_matrix: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
        rgb_to_y: [0; 4],
        rgb_to_u: [0; 4],
        rgb_to_v: [0; 4],
    };
    let rgb_max: core::ffi::c_int = ((1 as core::ffi::c_int) << rgb_bit_depth)
        - 1 as core::ffi::c_int;
    let rgb_round: core::ffi::c_int = (1 as core::ffi::c_int)
        << rgb_bit_depth - 1 as core::ffi::c_int;
    let yuv_max: core::ffi::c_int = ((1 as core::ffi::c_int) << yuv_bit_depth)
        - 1 as core::ffi::c_int;
    let sfix: core::ffi::c_int = GetPrecisionShift(rgb_bit_depth) as core::ffi::c_int;
    if width < 1 as core::ffi::c_int || height < 1 as core::ffi::c_int
        || width == INT_MAX || height == INT_MAX || r_ptr.is_null() || g_ptr.is_null()
        || b_ptr.is_null() || y_ptr.is_null() || u_ptr.is_null() || v_ptr.is_null()
    {
        return 0 as core::ffi::c_int;
    }
    if rgb_bit_depth != 8 as core::ffi::c_int && rgb_bit_depth != 10 as core::ffi::c_int
        && rgb_bit_depth != 12 as core::ffi::c_int
        && rgb_bit_depth != 16 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if yuv_bit_depth != 8 as core::ffi::c_int && yuv_bit_depth != 10 as core::ffi::c_int
        && yuv_bit_depth != 12 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if rgb_bit_depth > 8 as core::ffi::c_int
        && (rgb_step % 2 as core::ffi::c_int != 0 as core::ffi::c_int
            || rgb_stride % 2 as core::ffi::c_int != 0 as core::ffi::c_int)
    {
        return 0 as core::ffi::c_int;
    }
    if yuv_bit_depth > 8 as core::ffi::c_int
        && (y_stride % 2 as core::ffi::c_int != 0 as core::ffi::c_int
            || u_stride % 2 as core::ffi::c_int != 0 as core::ffi::c_int
            || v_stride % 2 as core::ffi::c_int != 0 as core::ffi::c_int)
    {
        return 0 as core::ffi::c_int;
    }
    SharpYuvInit(
        ::core::mem::transmute::<
            *mut VP8CPUInfo,
            VP8CPUInfo,
        >(core::ptr::addr_of_mut!(SharpYuvGetCPUInfo) as *mut VP8CPUInfo),
    );
    if rgb_bit_depth == yuv_bit_depth {
        memcpy(
            &mut scaled_matrix as *mut SharpYuvConversionMatrix
                as *mut core::ffi::c_void,
            yuv_matrix as *const core::ffi::c_void,
            ::core::mem::size_of::<SharpYuvConversionMatrix>() as size_t,
        );
    } else {
        let mut i: core::ffi::c_int = 0;
        i = 0 as core::ffi::c_int;
        while i < 3 as core::ffi::c_int {
            scaled_matrix.rgb_to_y[i as usize] = ((*yuv_matrix).rgb_to_y[i as usize]
                * yuv_max + rgb_round) / rgb_max;
            scaled_matrix.rgb_to_u[i as usize] = ((*yuv_matrix).rgb_to_u[i as usize]
                * yuv_max + rgb_round) / rgb_max;
            scaled_matrix.rgb_to_v[i as usize] = ((*yuv_matrix).rgb_to_v[i as usize]
                * yuv_max + rgb_round) / rgb_max;
            i += 1;
        }
    }
    scaled_matrix.rgb_to_y[3 as core::ffi::c_int as usize] = Shift(
        (*yuv_matrix).rgb_to_y[3 as core::ffi::c_int as usize],
        sfix,
    );
    scaled_matrix.rgb_to_u[3 as core::ffi::c_int as usize] = Shift(
        (*yuv_matrix).rgb_to_u[3 as core::ffi::c_int as usize],
        sfix,
    );
    scaled_matrix.rgb_to_v[3 as core::ffi::c_int as usize] = Shift(
        (*yuv_matrix).rgb_to_v[3 as core::ffi::c_int as usize],
        sfix,
    );
    return DoSharpArgbToYuv(
        r_ptr as *const uint8_t,
        g_ptr as *const uint8_t,
        b_ptr as *const uint8_t,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr as *mut uint8_t,
        y_stride,
        u_ptr as *mut uint8_t,
        u_stride,
        v_ptr as *mut uint8_t,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &mut scaled_matrix,
        transfer_type,
    );
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
