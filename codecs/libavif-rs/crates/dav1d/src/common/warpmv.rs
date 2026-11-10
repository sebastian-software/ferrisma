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

// Mechanically translated from upstream/dav1d/src/warpmv.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn llabs(_: core::ffi::c_longlong) -> core::ffi::c_longlong;
}
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type Dav1dWarpedMotionType = core::ffi::c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub p: C2RustUnnamed_0,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub y: int16_t,
    pub x: int16_t,
}
pub const INT16_MAX: core::ffi::c_int = 32767 as core::ffi::c_int;
pub const INT16_MIN: core::ffi::c_int = -(32768 as core::ffi::c_int);
#[inline]
unsafe extern "C" fn clz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn clzll(mask: core::ffi::c_ulonglong) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn iclip(
    v: core::ffi::c_int,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) -> core::ffi::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign(
    v: core::ffi::c_int,
    s: core::ffi::c_int,
) -> core::ffi::c_int {
    return if s < 0 as core::ffi::c_int { -v } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign64(v: core::ffi::c_int, s: int64_t) -> core::ffi::c_int {
    return if s < 0 as int64_t { -v } else { v };
}
#[inline]
unsafe extern "C" fn ulog2(v: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ clz(v);
}
#[inline]
unsafe extern "C" fn u64log2(v: uint64_t) -> core::ffi::c_int {
    return 63 as core::ffi::c_int ^ clzll(v as core::ffi::c_ulonglong);
}
static mut div_lut: [uint16_t; 257] = [
    16384 as core::ffi::c_int as uint16_t,
    16320 as core::ffi::c_int as uint16_t,
    16257 as core::ffi::c_int as uint16_t,
    16194 as core::ffi::c_int as uint16_t,
    16132 as core::ffi::c_int as uint16_t,
    16070 as core::ffi::c_int as uint16_t,
    16009 as core::ffi::c_int as uint16_t,
    15948 as core::ffi::c_int as uint16_t,
    15888 as core::ffi::c_int as uint16_t,
    15828 as core::ffi::c_int as uint16_t,
    15768 as core::ffi::c_int as uint16_t,
    15709 as core::ffi::c_int as uint16_t,
    15650 as core::ffi::c_int as uint16_t,
    15592 as core::ffi::c_int as uint16_t,
    15534 as core::ffi::c_int as uint16_t,
    15477 as core::ffi::c_int as uint16_t,
    15420 as core::ffi::c_int as uint16_t,
    15364 as core::ffi::c_int as uint16_t,
    15308 as core::ffi::c_int as uint16_t,
    15252 as core::ffi::c_int as uint16_t,
    15197 as core::ffi::c_int as uint16_t,
    15142 as core::ffi::c_int as uint16_t,
    15087 as core::ffi::c_int as uint16_t,
    15033 as core::ffi::c_int as uint16_t,
    14980 as core::ffi::c_int as uint16_t,
    14926 as core::ffi::c_int as uint16_t,
    14873 as core::ffi::c_int as uint16_t,
    14821 as core::ffi::c_int as uint16_t,
    14769 as core::ffi::c_int as uint16_t,
    14717 as core::ffi::c_int as uint16_t,
    14665 as core::ffi::c_int as uint16_t,
    14614 as core::ffi::c_int as uint16_t,
    14564 as core::ffi::c_int as uint16_t,
    14513 as core::ffi::c_int as uint16_t,
    14463 as core::ffi::c_int as uint16_t,
    14413 as core::ffi::c_int as uint16_t,
    14364 as core::ffi::c_int as uint16_t,
    14315 as core::ffi::c_int as uint16_t,
    14266 as core::ffi::c_int as uint16_t,
    14218 as core::ffi::c_int as uint16_t,
    14170 as core::ffi::c_int as uint16_t,
    14122 as core::ffi::c_int as uint16_t,
    14075 as core::ffi::c_int as uint16_t,
    14028 as core::ffi::c_int as uint16_t,
    13981 as core::ffi::c_int as uint16_t,
    13935 as core::ffi::c_int as uint16_t,
    13888 as core::ffi::c_int as uint16_t,
    13843 as core::ffi::c_int as uint16_t,
    13797 as core::ffi::c_int as uint16_t,
    13752 as core::ffi::c_int as uint16_t,
    13707 as core::ffi::c_int as uint16_t,
    13662 as core::ffi::c_int as uint16_t,
    13618 as core::ffi::c_int as uint16_t,
    13574 as core::ffi::c_int as uint16_t,
    13530 as core::ffi::c_int as uint16_t,
    13487 as core::ffi::c_int as uint16_t,
    13443 as core::ffi::c_int as uint16_t,
    13400 as core::ffi::c_int as uint16_t,
    13358 as core::ffi::c_int as uint16_t,
    13315 as core::ffi::c_int as uint16_t,
    13273 as core::ffi::c_int as uint16_t,
    13231 as core::ffi::c_int as uint16_t,
    13190 as core::ffi::c_int as uint16_t,
    13148 as core::ffi::c_int as uint16_t,
    13107 as core::ffi::c_int as uint16_t,
    13066 as core::ffi::c_int as uint16_t,
    13026 as core::ffi::c_int as uint16_t,
    12985 as core::ffi::c_int as uint16_t,
    12945 as core::ffi::c_int as uint16_t,
    12906 as core::ffi::c_int as uint16_t,
    12866 as core::ffi::c_int as uint16_t,
    12827 as core::ffi::c_int as uint16_t,
    12788 as core::ffi::c_int as uint16_t,
    12749 as core::ffi::c_int as uint16_t,
    12710 as core::ffi::c_int as uint16_t,
    12672 as core::ffi::c_int as uint16_t,
    12633 as core::ffi::c_int as uint16_t,
    12596 as core::ffi::c_int as uint16_t,
    12558 as core::ffi::c_int as uint16_t,
    12520 as core::ffi::c_int as uint16_t,
    12483 as core::ffi::c_int as uint16_t,
    12446 as core::ffi::c_int as uint16_t,
    12409 as core::ffi::c_int as uint16_t,
    12373 as core::ffi::c_int as uint16_t,
    12336 as core::ffi::c_int as uint16_t,
    12300 as core::ffi::c_int as uint16_t,
    12264 as core::ffi::c_int as uint16_t,
    12228 as core::ffi::c_int as uint16_t,
    12193 as core::ffi::c_int as uint16_t,
    12157 as core::ffi::c_int as uint16_t,
    12122 as core::ffi::c_int as uint16_t,
    12087 as core::ffi::c_int as uint16_t,
    12053 as core::ffi::c_int as uint16_t,
    12018 as core::ffi::c_int as uint16_t,
    11984 as core::ffi::c_int as uint16_t,
    11950 as core::ffi::c_int as uint16_t,
    11916 as core::ffi::c_int as uint16_t,
    11882 as core::ffi::c_int as uint16_t,
    11848 as core::ffi::c_int as uint16_t,
    11815 as core::ffi::c_int as uint16_t,
    11782 as core::ffi::c_int as uint16_t,
    11749 as core::ffi::c_int as uint16_t,
    11716 as core::ffi::c_int as uint16_t,
    11683 as core::ffi::c_int as uint16_t,
    11651 as core::ffi::c_int as uint16_t,
    11619 as core::ffi::c_int as uint16_t,
    11586 as core::ffi::c_int as uint16_t,
    11555 as core::ffi::c_int as uint16_t,
    11523 as core::ffi::c_int as uint16_t,
    11491 as core::ffi::c_int as uint16_t,
    11460 as core::ffi::c_int as uint16_t,
    11429 as core::ffi::c_int as uint16_t,
    11398 as core::ffi::c_int as uint16_t,
    11367 as core::ffi::c_int as uint16_t,
    11336 as core::ffi::c_int as uint16_t,
    11305 as core::ffi::c_int as uint16_t,
    11275 as core::ffi::c_int as uint16_t,
    11245 as core::ffi::c_int as uint16_t,
    11215 as core::ffi::c_int as uint16_t,
    11185 as core::ffi::c_int as uint16_t,
    11155 as core::ffi::c_int as uint16_t,
    11125 as core::ffi::c_int as uint16_t,
    11096 as core::ffi::c_int as uint16_t,
    11067 as core::ffi::c_int as uint16_t,
    11038 as core::ffi::c_int as uint16_t,
    11009 as core::ffi::c_int as uint16_t,
    10980 as core::ffi::c_int as uint16_t,
    10951 as core::ffi::c_int as uint16_t,
    10923 as core::ffi::c_int as uint16_t,
    10894 as core::ffi::c_int as uint16_t,
    10866 as core::ffi::c_int as uint16_t,
    10838 as core::ffi::c_int as uint16_t,
    10810 as core::ffi::c_int as uint16_t,
    10782 as core::ffi::c_int as uint16_t,
    10755 as core::ffi::c_int as uint16_t,
    10727 as core::ffi::c_int as uint16_t,
    10700 as core::ffi::c_int as uint16_t,
    10673 as core::ffi::c_int as uint16_t,
    10645 as core::ffi::c_int as uint16_t,
    10618 as core::ffi::c_int as uint16_t,
    10592 as core::ffi::c_int as uint16_t,
    10565 as core::ffi::c_int as uint16_t,
    10538 as core::ffi::c_int as uint16_t,
    10512 as core::ffi::c_int as uint16_t,
    10486 as core::ffi::c_int as uint16_t,
    10460 as core::ffi::c_int as uint16_t,
    10434 as core::ffi::c_int as uint16_t,
    10408 as core::ffi::c_int as uint16_t,
    10382 as core::ffi::c_int as uint16_t,
    10356 as core::ffi::c_int as uint16_t,
    10331 as core::ffi::c_int as uint16_t,
    10305 as core::ffi::c_int as uint16_t,
    10280 as core::ffi::c_int as uint16_t,
    10255 as core::ffi::c_int as uint16_t,
    10230 as core::ffi::c_int as uint16_t,
    10205 as core::ffi::c_int as uint16_t,
    10180 as core::ffi::c_int as uint16_t,
    10156 as core::ffi::c_int as uint16_t,
    10131 as core::ffi::c_int as uint16_t,
    10107 as core::ffi::c_int as uint16_t,
    10082 as core::ffi::c_int as uint16_t,
    10058 as core::ffi::c_int as uint16_t,
    10034 as core::ffi::c_int as uint16_t,
    10010 as core::ffi::c_int as uint16_t,
    9986 as core::ffi::c_int as uint16_t,
    9963 as core::ffi::c_int as uint16_t,
    9939 as core::ffi::c_int as uint16_t,
    9916 as core::ffi::c_int as uint16_t,
    9892 as core::ffi::c_int as uint16_t,
    9869 as core::ffi::c_int as uint16_t,
    9846 as core::ffi::c_int as uint16_t,
    9823 as core::ffi::c_int as uint16_t,
    9800 as core::ffi::c_int as uint16_t,
    9777 as core::ffi::c_int as uint16_t,
    9754 as core::ffi::c_int as uint16_t,
    9732 as core::ffi::c_int as uint16_t,
    9709 as core::ffi::c_int as uint16_t,
    9687 as core::ffi::c_int as uint16_t,
    9664 as core::ffi::c_int as uint16_t,
    9642 as core::ffi::c_int as uint16_t,
    9620 as core::ffi::c_int as uint16_t,
    9598 as core::ffi::c_int as uint16_t,
    9576 as core::ffi::c_int as uint16_t,
    9554 as core::ffi::c_int as uint16_t,
    9533 as core::ffi::c_int as uint16_t,
    9511 as core::ffi::c_int as uint16_t,
    9489 as core::ffi::c_int as uint16_t,
    9468 as core::ffi::c_int as uint16_t,
    9447 as core::ffi::c_int as uint16_t,
    9425 as core::ffi::c_int as uint16_t,
    9404 as core::ffi::c_int as uint16_t,
    9383 as core::ffi::c_int as uint16_t,
    9362 as core::ffi::c_int as uint16_t,
    9341 as core::ffi::c_int as uint16_t,
    9321 as core::ffi::c_int as uint16_t,
    9300 as core::ffi::c_int as uint16_t,
    9279 as core::ffi::c_int as uint16_t,
    9259 as core::ffi::c_int as uint16_t,
    9239 as core::ffi::c_int as uint16_t,
    9218 as core::ffi::c_int as uint16_t,
    9198 as core::ffi::c_int as uint16_t,
    9178 as core::ffi::c_int as uint16_t,
    9158 as core::ffi::c_int as uint16_t,
    9138 as core::ffi::c_int as uint16_t,
    9118 as core::ffi::c_int as uint16_t,
    9098 as core::ffi::c_int as uint16_t,
    9079 as core::ffi::c_int as uint16_t,
    9059 as core::ffi::c_int as uint16_t,
    9039 as core::ffi::c_int as uint16_t,
    9020 as core::ffi::c_int as uint16_t,
    9001 as core::ffi::c_int as uint16_t,
    8981 as core::ffi::c_int as uint16_t,
    8962 as core::ffi::c_int as uint16_t,
    8943 as core::ffi::c_int as uint16_t,
    8924 as core::ffi::c_int as uint16_t,
    8905 as core::ffi::c_int as uint16_t,
    8886 as core::ffi::c_int as uint16_t,
    8867 as core::ffi::c_int as uint16_t,
    8849 as core::ffi::c_int as uint16_t,
    8830 as core::ffi::c_int as uint16_t,
    8812 as core::ffi::c_int as uint16_t,
    8793 as core::ffi::c_int as uint16_t,
    8775 as core::ffi::c_int as uint16_t,
    8756 as core::ffi::c_int as uint16_t,
    8738 as core::ffi::c_int as uint16_t,
    8720 as core::ffi::c_int as uint16_t,
    8702 as core::ffi::c_int as uint16_t,
    8684 as core::ffi::c_int as uint16_t,
    8666 as core::ffi::c_int as uint16_t,
    8648 as core::ffi::c_int as uint16_t,
    8630 as core::ffi::c_int as uint16_t,
    8613 as core::ffi::c_int as uint16_t,
    8595 as core::ffi::c_int as uint16_t,
    8577 as core::ffi::c_int as uint16_t,
    8560 as core::ffi::c_int as uint16_t,
    8542 as core::ffi::c_int as uint16_t,
    8525 as core::ffi::c_int as uint16_t,
    8508 as core::ffi::c_int as uint16_t,
    8490 as core::ffi::c_int as uint16_t,
    8473 as core::ffi::c_int as uint16_t,
    8456 as core::ffi::c_int as uint16_t,
    8439 as core::ffi::c_int as uint16_t,
    8422 as core::ffi::c_int as uint16_t,
    8405 as core::ffi::c_int as uint16_t,
    8389 as core::ffi::c_int as uint16_t,
    8372 as core::ffi::c_int as uint16_t,
    8355 as core::ffi::c_int as uint16_t,
    8339 as core::ffi::c_int as uint16_t,
    8322 as core::ffi::c_int as uint16_t,
    8306 as core::ffi::c_int as uint16_t,
    8289 as core::ffi::c_int as uint16_t,
    8273 as core::ffi::c_int as uint16_t,
    8257 as core::ffi::c_int as uint16_t,
    8240 as core::ffi::c_int as uint16_t,
    8224 as core::ffi::c_int as uint16_t,
    8208 as core::ffi::c_int as uint16_t,
    8192 as core::ffi::c_int as uint16_t,
];
#[inline]
unsafe extern "C" fn iclip_wmp(v: core::ffi::c_int) -> core::ffi::c_int {
    let cv: core::ffi::c_int = iclip(v, INT16_MIN, INT16_MAX) as core::ffi::c_int;
    return apply_sign(abs(cv) + 32 as core::ffi::c_int >> 6 as core::ffi::c_int, cv)
        * ((1 as core::ffi::c_int) << 6 as core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn resolve_divisor_32(
    d: core::ffi::c_uint,
    shift: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    *shift = ulog2(d);
    let e: core::ffi::c_int = d
        .wrapping_sub(((1 as core::ffi::c_int) << *shift) as core::ffi::c_uint)
        as core::ffi::c_int;
    let f: core::ffi::c_int = if *shift > 8 as core::ffi::c_int {
        e + ((1 as core::ffi::c_int) << *shift - 9 as core::ffi::c_int)
            >> *shift - 8 as core::ffi::c_int
    } else {
        e << 8 as core::ffi::c_int - *shift
    };
    if !(f <= 256 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"resolve_divisor_32\0"))
                .as_ptr(),
            b"warpmv.c\0" as *const u8 as *const core::ffi::c_char,
            74 as core::ffi::c_int,
            b"f <= 256\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *shift += 14 as core::ffi::c_int;
    return div_lut[f as usize] as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_shear_params(
    wm: *mut Dav1dWarpedMotionParams,
) -> core::ffi::c_int {
    let mat: *const int32_t = ((*wm).matrix).as_mut_ptr();
    if *mat.offset(2 as core::ffi::c_int as isize) <= 0 as int32_t {
        return 1 as core::ffi::c_int;
    }
    (*wm).u.p.alpha = iclip_wmp(
        *mat.offset(2 as core::ffi::c_int as isize) - 0x10000 as core::ffi::c_int,
    ) as int16_t;
    (*wm).u.p.beta = iclip_wmp(*mat.offset(3 as core::ffi::c_int as isize)) as int16_t;
    let mut shift: core::ffi::c_int = 0;
    let y: core::ffi::c_int = apply_sign(
        resolve_divisor_32(
            abs(*mat.offset(2 as core::ffi::c_int as isize)) as core::ffi::c_uint,
            &raw mut shift,
        ),
        *mat.offset(2 as core::ffi::c_int as isize),
    ) as core::ffi::c_int;
    let v1: int64_t = *mat.offset(4 as core::ffi::c_int as isize) as int64_t
        * 0x10000 as int64_t * y as int64_t;
    let rnd: core::ffi::c_int = (1 as core::ffi::c_int) << shift
        >> 1 as core::ffi::c_int;
    (*wm).u.p.gamma = iclip_wmp(
        apply_sign64(
            (llabs(v1 as core::ffi::c_longlong) + rnd as core::ffi::c_longlong >> shift)
                as core::ffi::c_int,
            v1,
        ),
    ) as int16_t;
    let v2: int64_t = *mat.offset(3 as core::ffi::c_int as isize) as int64_t
        * *mat.offset(4 as core::ffi::c_int as isize) as int64_t * y as int64_t;
    (*wm).u.p.delta = iclip_wmp(
        *mat.offset(5 as core::ffi::c_int as isize)
            - apply_sign64(
                (llabs(v2 as core::ffi::c_longlong) + rnd as core::ffi::c_longlong
                    >> shift) as core::ffi::c_int,
                v2,
            ) - 0x10000 as core::ffi::c_int,
    ) as int16_t;
    return (4 as core::ffi::c_int * abs((*wm).u.p.alpha as core::ffi::c_int)
        + 7 as core::ffi::c_int * abs((*wm).u.p.beta as core::ffi::c_int)
        >= 0x10000 as core::ffi::c_int
        || 4 as core::ffi::c_int * abs((*wm).u.p.gamma as core::ffi::c_int)
            + 4 as core::ffi::c_int * abs((*wm).u.p.delta as core::ffi::c_int)
            >= 0x10000 as core::ffi::c_int) as core::ffi::c_int;
}
unsafe extern "C" fn resolve_divisor_64(
    d: uint64_t,
    shift: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    *shift = u64log2(d);
    let e: int64_t = d.wrapping_sub(((1 as core::ffi::c_longlong) << *shift) as uint64_t)
        as int64_t;
    let f: int64_t = if *shift > 8 as core::ffi::c_int {
        e + ((1 as int64_t) << *shift - 9 as core::ffi::c_int)
            >> *shift - 8 as core::ffi::c_int
    } else {
        e << 8 as core::ffi::c_int - *shift
    };
    if !(f <= 256 as int64_t) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"resolve_divisor_64\0"))
                .as_ptr(),
            b"warpmv.c\0" as *const u8 as *const core::ffi::c_char,
            107 as core::ffi::c_int,
            b"f <= 256\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *shift += 14 as core::ffi::c_int;
    return div_lut[f as usize] as core::ffi::c_int;
}
unsafe extern "C" fn get_mult_shift_ndiag(
    px: int64_t,
    idet: core::ffi::c_int,
    shift: core::ffi::c_int,
) -> core::ffi::c_int {
    let v1: int64_t = px * idet as int64_t;
    let v2: core::ffi::c_int = apply_sign64(
        (llabs(v1 as core::ffi::c_longlong)
            + ((1 as core::ffi::c_longlong) << shift >> 1 as core::ffi::c_int) >> shift)
            as core::ffi::c_int,
        v1,
    ) as core::ffi::c_int;
    return iclip(v2, -(0x1fff as core::ffi::c_int), 0x1fff as core::ffi::c_int);
}
unsafe extern "C" fn get_mult_shift_diag(
    px: int64_t,
    idet: core::ffi::c_int,
    shift: core::ffi::c_int,
) -> core::ffi::c_int {
    let v1: int64_t = px * idet as int64_t;
    let v2: core::ffi::c_int = apply_sign64(
        (llabs(v1 as core::ffi::c_longlong)
            + ((1 as core::ffi::c_longlong) << shift >> 1 as core::ffi::c_int) >> shift)
            as core::ffi::c_int,
        v1,
    ) as core::ffi::c_int;
    return iclip(v2, 0xe001 as core::ffi::c_int, 0x11fff as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_set_affine_mv2d(
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
    mv: mv,
    wm: *mut Dav1dWarpedMotionParams,
    bx4: core::ffi::c_int,
    by4: core::ffi::c_int,
) {
    let mat: *mut int32_t = ((*wm).matrix).as_mut_ptr();
    let rsuy: core::ffi::c_int = 2 as core::ffi::c_int * bh4 - 1 as core::ffi::c_int;
    let rsux: core::ffi::c_int = 2 as core::ffi::c_int * bw4 - 1 as core::ffi::c_int;
    let isuy: core::ffi::c_int = by4 * 4 as core::ffi::c_int + rsuy;
    let isux: core::ffi::c_int = bx4 * 4 as core::ffi::c_int + rsux;
    *mat.offset(0 as core::ffi::c_int as isize) = iclip(
        mv.c2rust_unnamed.x as core::ffi::c_int * 0x2000 as core::ffi::c_int
            - (isux
                * (*mat.offset(2 as core::ffi::c_int as isize)
                    - 0x10000 as core::ffi::c_int)
                + isuy * *mat.offset(3 as core::ffi::c_int as isize)),
        -(0x800000 as core::ffi::c_int),
        0x7fffff as core::ffi::c_int,
    ) as int32_t;
    *mat.offset(1 as core::ffi::c_int as isize) = iclip(
        mv.c2rust_unnamed.y as core::ffi::c_int * 0x2000 as core::ffi::c_int
            - (isux * *mat.offset(4 as core::ffi::c_int as isize)
                + isuy
                    * (*mat.offset(5 as core::ffi::c_int as isize)
                        - 0x10000 as core::ffi::c_int)),
        -(0x800000 as core::ffi::c_int),
        0x7fffff as core::ffi::c_int,
    ) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_find_affine_int(
    mut pts: *const [[core::ffi::c_int; 2]; 2],
    np: core::ffi::c_int,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
    mv: mv,
    wm: *mut Dav1dWarpedMotionParams,
    bx4: core::ffi::c_int,
    by4: core::ffi::c_int,
) -> core::ffi::c_int {
    let mat: *mut int32_t = ((*wm).matrix).as_mut_ptr();
    let mut a: [[core::ffi::c_int; 2]; 2] = [
        [0 as core::ffi::c_int, 0 as core::ffi::c_int],
        [0 as core::ffi::c_int, 0 as core::ffi::c_int],
    ];
    let mut bx: [core::ffi::c_int; 2] = [0 as core::ffi::c_int, 0 as core::ffi::c_int];
    let mut by: [core::ffi::c_int; 2] = [0 as core::ffi::c_int, 0 as core::ffi::c_int];
    let rsuy: core::ffi::c_int = 2 as core::ffi::c_int * bh4 - 1 as core::ffi::c_int;
    let rsux: core::ffi::c_int = 2 as core::ffi::c_int * bw4 - 1 as core::ffi::c_int;
    let suy: core::ffi::c_int = rsuy * 8 as core::ffi::c_int;
    let sux: core::ffi::c_int = rsux * 8 as core::ffi::c_int;
    let duy: core::ffi::c_int = suy + mv.c2rust_unnamed.y as core::ffi::c_int;
    let dux: core::ffi::c_int = sux + mv.c2rust_unnamed.x as core::ffi::c_int;
    let isuy: core::ffi::c_int = by4 * 4 as core::ffi::c_int + rsuy;
    let isux: core::ffi::c_int = bx4 * 4 as core::ffi::c_int + rsux;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < np {
        let dx: core::ffi::c_int = (*pts
            .offset(
                i as isize,
            ))[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] - dux;
        let dy: core::ffi::c_int = (*pts
            .offset(
                i as isize,
            ))[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] - duy;
        let sx: core::ffi::c_int = (*pts
            .offset(
                i as isize,
            ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] - sux;
        let sy: core::ffi::c_int = (*pts
            .offset(
                i as isize,
            ))[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] - suy;
        if abs(sx - dx) < 256 as core::ffi::c_int
            && abs(sy - dy) < 256 as core::ffi::c_int
        {
            a[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                += (sx * sx >> 2 as core::ffi::c_int) + sx * 2 as core::ffi::c_int
                    + 8 as core::ffi::c_int;
            a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                += (sx * sy >> 2 as core::ffi::c_int) + sx + sy + 4 as core::ffi::c_int;
            a[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                += (sy * sy >> 2 as core::ffi::c_int) + sy * 2 as core::ffi::c_int
                    + 8 as core::ffi::c_int;
            bx[0 as core::ffi::c_int as usize]
                += (sx * dx >> 2 as core::ffi::c_int) + sx + dx + 8 as core::ffi::c_int;
            bx[1 as core::ffi::c_int as usize]
                += (sy * dx >> 2 as core::ffi::c_int) + sy + dx + 4 as core::ffi::c_int;
            by[0 as core::ffi::c_int as usize]
                += (sx * dy >> 2 as core::ffi::c_int) + sx + dy + 4 as core::ffi::c_int;
            by[1 as core::ffi::c_int as usize]
                += (sy * dy >> 2 as core::ffi::c_int) + sy + dy + 8 as core::ffi::c_int;
        }
        i += 1;
    }
    let det: int64_t = a[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
        as int64_t
        * a[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
        - a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
            * a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as int64_t;
    if det == 0 as int64_t {
        return 1 as core::ffi::c_int;
    }
    let mut shift: core::ffi::c_int = 0;
    let mut idet: core::ffi::c_int = apply_sign64(
        resolve_divisor_64(llabs(det as core::ffi::c_longlong) as uint64_t, &raw mut shift),
        det,
    );
    shift -= 16 as core::ffi::c_int;
    if shift < 0 as core::ffi::c_int {
        idet <<= -shift;
        shift = 0 as core::ffi::c_int;
    }
    *mat.offset(2 as core::ffi::c_int as isize) = get_mult_shift_diag(
        a[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
            * bx[0 as core::ffi::c_int as usize] as int64_t
            - a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as int64_t * bx[1 as core::ffi::c_int as usize] as int64_t,
        idet,
        shift,
    ) as int32_t;
    *mat.offset(3 as core::ffi::c_int as isize) = get_mult_shift_ndiag(
        a[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] as int64_t
            * bx[1 as core::ffi::c_int as usize] as int64_t
            - a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as int64_t * bx[0 as core::ffi::c_int as usize] as int64_t,
        idet,
        shift,
    ) as int32_t;
    *mat.offset(4 as core::ffi::c_int as isize) = get_mult_shift_ndiag(
        a[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
            * by[0 as core::ffi::c_int as usize] as int64_t
            - a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as int64_t * by[1 as core::ffi::c_int as usize] as int64_t,
        idet,
        shift,
    ) as int32_t;
    *mat.offset(5 as core::ffi::c_int as isize) = get_mult_shift_diag(
        a[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] as int64_t
            * by[1 as core::ffi::c_int as usize] as int64_t
            - a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as int64_t * by[0 as core::ffi::c_int as usize] as int64_t,
        idet,
        shift,
    ) as int32_t;
    *mat.offset(0 as core::ffi::c_int as isize) = iclip(
        mv.c2rust_unnamed.x as core::ffi::c_int * 0x2000 as core::ffi::c_int
            - (isux
                * (*mat.offset(2 as core::ffi::c_int as isize)
                    - 0x10000 as core::ffi::c_int)
                + isuy * *mat.offset(3 as core::ffi::c_int as isize)),
        -(0x800000 as core::ffi::c_int),
        0x7fffff as core::ffi::c_int,
    ) as int32_t;
    *mat.offset(1 as core::ffi::c_int as isize) = iclip(
        mv.c2rust_unnamed.y as core::ffi::c_int * 0x2000 as core::ffi::c_int
            - (isux * *mat.offset(4 as core::ffi::c_int as isize)
                + isuy
                    * (*mat.offset(5 as core::ffi::c_int as isize)
                        - 0x10000 as core::ffi::c_int)),
        -(0x800000 as core::ffi::c_int),
        0x7fffff as core::ffi::c_int,
    ) as int32_t;
    return 0 as core::ffi::c_int;
}
