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

// Mechanically translated from upstream/dav1d/src/ipred_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    static dav1d_sm_weights: [uint8_t; 128];
    static dav1d_dr_intra_derivative: [uint16_t; 44];
    static dav1d_filter_intra_taps: [[int8_t; 64]; 5];
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type ptrdiff_t = isize;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type pixel = uint16_t;
pub type Dav1dPixelLayout = core::ffi::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
pub type IntraPredMode = core::ffi::c_uint;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: IntraPredMode = 14;
pub const N_UV_INTRA_PRED_MODES: IntraPredMode = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: IntraPredMode = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;
pub type angular_ipred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
#[inline]
unsafe extern "C" fn ctz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.trailing_zeros() as i32;
}
#[inline]
unsafe extern "C" fn imax(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
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
unsafe extern "C" fn pixel_set(
    dst: *mut pixel,
    val: core::ffi::c_int,
    num: core::ffi::c_int,
) {
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < num {
        *dst.offset(n as isize) = val as pixel;
        n += 1;
    }
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 as ptrdiff_t == 0 {} else {
        __assert_fail(
            b"!(x & 1)\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/include/common/bitdepth.h\0" as *const u8
                as *const core::ffi::c_char,
            80 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"ptrdiff_t PXSTRIDE(const ptrdiff_t)\0"))
                .as_ptr(),
        );
    };
    return x >> 1 as core::ffi::c_int;
}
#[inline(never)]
unsafe extern "C" fn splat_dc(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    dc: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    if dc <= bitdepth_max {} else {
        __assert_fail(
            b"dc <= bitdepth_max\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            61 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 84],
                [core::ffi::c_char; 84],
            >(
                *b"void splat_dc(pixel *, const ptrdiff_t, const int, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let dcN: uint64_t = (dc as core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as core::ffi::c_ulonglong) as uint64_t;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width {
            *(&raw mut *dst.offset(x as isize) as *mut pixel as *mut uint64_t) = dcN;
            x = (x as core::ffi::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<uint64_t>() as usize
                        >> 1 as core::ffi::c_int) as core::ffi::c_ulong,
                ) as core::ffi::c_int as core::ffi::c_int;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn cfl_pred(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    dc: core::ffi::c_int,
    mut ac: *const int16_t,
    alpha: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width {
            let diff: core::ffi::c_int = alpha
                * *ac.offset(x as isize) as core::ffi::c_int;
            *dst.offset(x as isize) = iclip(
                dc
                    + apply_sign(
                        abs(diff) + 32 as core::ffi::c_int >> 6 as core::ffi::c_int,
                        diff,
                    ),
                0 as core::ffi::c_int,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        ac = ac.offset(width as isize);
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn dc_gen_top(
    topleft: *const pixel,
    width: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut dc: core::ffi::c_uint = (width >> 1 as core::ffi::c_int)
        as core::ffi::c_uint;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < width {
        dc = dc
            .wrapping_add(
                *topleft.offset((1 as core::ffi::c_int + i) as isize)
                    as core::ffi::c_uint,
            );
        i += 1;
    }
    return dc >> ctz(width as core::ffi::c_uint);
}
unsafe extern "C" fn ipred_dc_top_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen_top(topleft, width) as core::ffi::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_cfl_top_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut ac: *const int16_t,
    alpha: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    cfl_pred(
        dst,
        stride,
        width,
        height,
        dc_gen_top(topleft, width) as core::ffi::c_int,
        ac,
        alpha,
        bitdepth_max,
    );
}
unsafe extern "C" fn dc_gen_left(
    topleft: *const pixel,
    height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut dc: core::ffi::c_uint = (height >> 1 as core::ffi::c_int)
        as core::ffi::c_uint;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        dc = dc
            .wrapping_add(
                *topleft.offset(-(1 as core::ffi::c_int + i) as isize)
                    as core::ffi::c_uint,
            );
        i += 1;
    }
    return dc >> ctz(height as core::ffi::c_uint);
}
unsafe extern "C" fn ipred_dc_left_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen_left(topleft, height) as core::ffi::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_cfl_left_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut ac: *const int16_t,
    alpha: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let dc: core::ffi::c_uint = dc_gen_left(topleft, height) as core::ffi::c_uint;
    cfl_pred(
        dst,
        stride,
        width,
        height,
        dc as core::ffi::c_int,
        ac,
        alpha,
        bitdepth_max,
    );
}
pub const MULTIPLIER_1x2: core::ffi::c_int = 0xaaab as core::ffi::c_int;
pub const MULTIPLIER_1x4: core::ffi::c_int = 0x6667 as core::ffi::c_int;
pub const BASE_SHIFT: core::ffi::c_int = 17 as core::ffi::c_int;
unsafe extern "C" fn dc_gen(
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut dc: core::ffi::c_uint = (width + height >> 1 as core::ffi::c_int)
        as core::ffi::c_uint;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < width {
        dc = dc
            .wrapping_add(
                *topleft.offset((i + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_uint,
            );
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < height {
        dc = dc
            .wrapping_add(
                *topleft.offset(-(i_0 + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_uint,
            );
        i_0 += 1;
    }
    dc >>= ctz((width + height) as core::ffi::c_uint);
    if width != height {
        dc = dc
            .wrapping_mul(
                (if width > height * 2 as core::ffi::c_int
                    || height > width * 2 as core::ffi::c_int
                {
                    MULTIPLIER_1x4
                } else {
                    MULTIPLIER_1x2
                }) as core::ffi::c_uint,
            );
        dc >>= BASE_SHIFT;
    }
    return dc;
}
unsafe extern "C" fn ipred_dc_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen(topleft, width, height) as core::ffi::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_cfl_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut ac: *const int16_t,
    alpha: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let dc: core::ffi::c_uint = dc_gen(topleft, width, height) as core::ffi::c_uint;
    cfl_pred(
        dst,
        stride,
        width,
        height,
        dc as core::ffi::c_int,
        ac,
        alpha,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_dc_128_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let dc: core::ffi::c_int = bitdepth_max + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int;
    splat_dc(dst, stride, width, height, dc, bitdepth_max);
}
unsafe extern "C" fn ipred_cfl_128_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut ac: *const int16_t,
    alpha: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let dc: core::ffi::c_int = bitdepth_max + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int;
    cfl_pred(dst, stride, width, height, dc, ac, alpha, bitdepth_max);
}
unsafe extern "C" fn ipred_v_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        memcpy(
            dst as *mut core::ffi::c_void,
            topleft.offset(1 as core::ffi::c_int as isize) as *const core::ffi::c_void,
            (width << 1 as core::ffi::c_int) as size_t,
        );
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_h_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        pixel_set(
            dst,
            *topleft.offset(-(1 as core::ffi::c_int + y) as isize) as core::ffi::c_int,
            width,
        );
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_paeth_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    tl_ptr: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let topleft: core::ffi::c_int = *tl_ptr.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let left: core::ffi::c_int = *tl_ptr
            .offset(-(y + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width {
            let top: core::ffi::c_int = *tl_ptr
                .offset((1 as core::ffi::c_int + x) as isize) as core::ffi::c_int;
            let base: core::ffi::c_int = left + top - topleft;
            let ldiff: core::ffi::c_int = abs(left - base) as core::ffi::c_int;
            let tdiff: core::ffi::c_int = abs(top - base) as core::ffi::c_int;
            let tldiff: core::ffi::c_int = abs(topleft - base) as core::ffi::c_int;
            *dst.offset(x as isize) = (if ldiff <= tdiff && ldiff <= tldiff {
                left
            } else if tdiff <= tldiff {
                top
            } else {
                topleft
            }) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let weights_hor: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(width as isize)
        as *const uint8_t;
    let weights_ver: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(height as isize)
        as *const uint8_t;
    let right: core::ffi::c_int = *topleft.offset(width as isize) as core::ffi::c_int;
    let bottom: core::ffi::c_int = *topleft.offset(-height as isize) as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width {
            let pred: core::ffi::c_int = *weights_ver.offset(y as isize)
                as core::ffi::c_int
                * *topleft.offset((1 as core::ffi::c_int + x) as isize)
                    as core::ffi::c_int
                + (256 as core::ffi::c_int
                    - *weights_ver.offset(y as isize) as core::ffi::c_int) * bottom
                + *weights_hor.offset(x as isize) as core::ffi::c_int
                    * *topleft.offset(-(1 as core::ffi::c_int + y) as isize)
                        as core::ffi::c_int
                + (256 as core::ffi::c_int
                    - *weights_hor.offset(x as isize) as core::ffi::c_int) * right;
            *dst.offset(x as isize) = (pred + 256 as core::ffi::c_int
                >> 9 as core::ffi::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_v_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let weights_ver: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(height as isize)
        as *const uint8_t;
    let bottom: core::ffi::c_int = *topleft.offset(-height as isize) as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width {
            let pred: core::ffi::c_int = *weights_ver.offset(y as isize)
                as core::ffi::c_int
                * *topleft.offset((1 as core::ffi::c_int + x) as isize)
                    as core::ffi::c_int
                + (256 as core::ffi::c_int
                    - *weights_ver.offset(y as isize) as core::ffi::c_int) * bottom;
            *dst.offset(x as isize) = (pred + 128 as core::ffi::c_int
                >> 8 as core::ffi::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_h_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    a: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let weights_hor: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(width as isize)
        as *const uint8_t;
    let right: core::ffi::c_int = *topleft.offset(width as isize) as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width {
            let pred: core::ffi::c_int = *weights_hor.offset(x as isize)
                as core::ffi::c_int
                * *topleft.offset(-(y + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int
                + (256 as core::ffi::c_int
                    - *weights_hor.offset(x as isize) as core::ffi::c_int) * right;
            *dst.offset(x as isize) = (pred + 128 as core::ffi::c_int
                >> 8 as core::ffi::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn get_filter_strength(
    wh: core::ffi::c_int,
    angle: core::ffi::c_int,
    is_sm: core::ffi::c_int,
) -> core::ffi::c_int {
    if is_sm != 0 {
        if wh <= 8 as core::ffi::c_int {
            if angle >= 64 as core::ffi::c_int {
                return 2 as core::ffi::c_int;
            }
            if angle >= 40 as core::ffi::c_int {
                return 1 as core::ffi::c_int;
            }
        } else if wh <= 16 as core::ffi::c_int {
            if angle >= 48 as core::ffi::c_int {
                return 2 as core::ffi::c_int;
            }
            if angle >= 20 as core::ffi::c_int {
                return 1 as core::ffi::c_int;
            }
        } else if wh <= 24 as core::ffi::c_int {
            if angle >= 4 as core::ffi::c_int {
                return 3 as core::ffi::c_int;
            }
        } else {
            return 3 as core::ffi::c_int
        }
    } else if wh <= 8 as core::ffi::c_int {
        if angle >= 56 as core::ffi::c_int {
            return 1 as core::ffi::c_int;
        }
    } else if wh <= 16 as core::ffi::c_int {
        if angle >= 40 as core::ffi::c_int {
            return 1 as core::ffi::c_int;
        }
    } else if wh <= 24 as core::ffi::c_int {
        if angle >= 32 as core::ffi::c_int {
            return 3 as core::ffi::c_int;
        }
        if angle >= 16 as core::ffi::c_int {
            return 2 as core::ffi::c_int;
        }
        if angle >= 8 as core::ffi::c_int {
            return 1 as core::ffi::c_int;
        }
    } else if wh <= 32 as core::ffi::c_int {
        if angle >= 32 as core::ffi::c_int {
            return 3 as core::ffi::c_int;
        }
        if angle >= 4 as core::ffi::c_int {
            return 2 as core::ffi::c_int;
        }
        return 1 as core::ffi::c_int;
    } else {
        return 3 as core::ffi::c_int
    }
    return 0 as core::ffi::c_int;
}
#[inline(never)]
unsafe extern "C" fn filter_edge(
    out: *mut pixel,
    sz: core::ffi::c_int,
    lim_from: core::ffi::c_int,
    lim_to: core::ffi::c_int,
    in_0: *const pixel,
    from: core::ffi::c_int,
    to: core::ffi::c_int,
    strength: core::ffi::c_int,
) {
    static mut kernel: [[uint8_t; 5]; 3] = [
        [
            0 as core::ffi::c_int as uint8_t,
            4 as core::ffi::c_int as uint8_t,
            8 as core::ffi::c_int as uint8_t,
            4 as core::ffi::c_int as uint8_t,
            0 as core::ffi::c_int as uint8_t,
        ],
        [
            0 as core::ffi::c_int as uint8_t,
            5 as core::ffi::c_int as uint8_t,
            6 as core::ffi::c_int as uint8_t,
            5 as core::ffi::c_int as uint8_t,
            0 as core::ffi::c_int as uint8_t,
        ],
        [
            2 as core::ffi::c_int as uint8_t,
            4 as core::ffi::c_int as uint8_t,
            4 as core::ffi::c_int as uint8_t,
            4 as core::ffi::c_int as uint8_t,
            2 as core::ffi::c_int as uint8_t,
        ],
    ];
    if strength > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"strength > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            373 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 117],
                [core::ffi::c_char; 117],
            >(
                *b"void filter_edge(pixel *const, const int, const int, const int, const pixel *const, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < imin(sz, lim_from) {
        *out.offset(i as isize) = *in_0
            .offset(iclip(i, from, to - 1 as core::ffi::c_int) as isize);
        i += 1;
    }
    while i < imin(lim_to, sz) {
        let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 5 as core::ffi::c_int {
            s
                += *in_0
                    .offset(
                        iclip(
                            i - 2 as core::ffi::c_int + j,
                            from,
                            to - 1 as core::ffi::c_int,
                        ) as isize,
                    ) as core::ffi::c_int
                    * kernel[(strength - 1 as core::ffi::c_int) as usize][j as usize]
                        as core::ffi::c_int;
            j += 1;
        }
        *out.offset(i as isize) = (s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as pixel;
        i += 1;
    }
    while i < sz {
        *out.offset(i as isize) = *in_0
            .offset(iclip(i, from, to - 1 as core::ffi::c_int) as isize);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn get_upsample(
    wh: core::ffi::c_int,
    angle: core::ffi::c_int,
    is_sm: core::ffi::c_int,
) -> core::ffi::c_int {
    return (angle < 40 as core::ffi::c_int && wh <= 16 as core::ffi::c_int >> is_sm)
        as core::ffi::c_int;
}
#[inline(never)]
unsafe extern "C" fn upsample_edge(
    out: *mut pixel,
    hsz: core::ffi::c_int,
    in_0: *const pixel,
    from: core::ffi::c_int,
    to: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    static mut kernel: [int8_t; 4] = [
        -(1 as core::ffi::c_int) as int8_t,
        9 as core::ffi::c_int as int8_t,
        9 as core::ffi::c_int as int8_t,
        -(1 as core::ffi::c_int) as int8_t,
    ];
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < hsz - 1 as core::ffi::c_int {
        *out.offset((i * 2 as core::ffi::c_int) as isize) = *in_0
            .offset(iclip(i, from, to - 1 as core::ffi::c_int) as isize);
        let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 4 as core::ffi::c_int {
            s
                += *in_0
                    .offset(
                        iclip(
                            i + j - 1 as core::ffi::c_int,
                            from,
                            to - 1 as core::ffi::c_int,
                        ) as isize,
                    ) as core::ffi::c_int * kernel[j as usize] as core::ffi::c_int;
            j += 1;
        }
        *out.offset((i * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = iclip(
            s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int,
            0 as core::ffi::c_int,
            bitdepth_max,
        ) as pixel;
        i += 1;
    }
    *out.offset((i * 2 as core::ffi::c_int) as isize) = *in_0
        .offset(iclip(i, from, to - 1 as core::ffi::c_int) as isize);
}
unsafe extern "C" fn ipred_z1_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut angle: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let is_sm: core::ffi::c_int = angle >> 9 as core::ffi::c_int
        & 0x1 as core::ffi::c_int;
    let enable_intra_edge_filter: core::ffi::c_int = angle >> 10 as core::ffi::c_int;
    angle &= 511 as core::ffi::c_int;
    if angle < 90 as core::ffi::c_int {} else {
        __assert_fail(
            b"angle < 90\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            417 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 122],
                [core::ffi::c_char; 122],
            >(
                *b"void ipred_z1_c(pixel *, const ptrdiff_t, const pixel *const, const int, const int, int, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut dx: core::ffi::c_int = dav1d_dr_intra_derivative[(angle
        >> 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
    let mut top_out: [pixel; 128] = [0; 128];
    let mut top: *const pixel = 0 as *const pixel;
    let mut max_base_x: core::ffi::c_int = 0;
    let upsample_above: core::ffi::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 as core::ffi::c_int - angle, is_sm)
            as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    if upsample_above != 0 {
        upsample_edge(
            top_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(1 as core::ffi::c_int as isize),
            -(1 as core::ffi::c_int),
            width + imin(width, height),
            bitdepth_max,
        );
        top = top_out.as_mut_ptr();
        max_base_x = 2 as core::ffi::c_int * (width + height) - 2 as core::ffi::c_int;
        dx <<= 1 as core::ffi::c_int;
    } else {
        let filter_strength: core::ffi::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90 as core::ffi::c_int - angle, is_sm)
                as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                top_out.as_mut_ptr(),
                width + height,
                0 as core::ffi::c_int,
                width + height,
                &*topleft_in.offset(1 as core::ffi::c_int as isize),
                -(1 as core::ffi::c_int),
                width + imin(width, height),
                filter_strength,
            );
            top = top_out.as_mut_ptr();
            max_base_x = width + height - 1 as core::ffi::c_int;
        } else {
            top = &*topleft_in.offset(1 as core::ffi::c_int as isize) as *const pixel;
            max_base_x = width + imin(width, height) - 1 as core::ffi::c_int;
        }
    }
    let base_inc: core::ffi::c_int = 1 as core::ffi::c_int + upsample_above;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut xpos: core::ffi::c_int = dx;
    while y < height {
        let frac: core::ffi::c_int = xpos & 0x3e as core::ffi::c_int;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut base: core::ffi::c_int = xpos >> 6 as core::ffi::c_int;
        while x < width {
            if base < max_base_x {
                let v: core::ffi::c_int = *top.offset(base as isize) as core::ffi::c_int
                    * (64 as core::ffi::c_int - frac)
                    + *top.offset((base + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * frac;
                *dst.offset(x as isize) = (v + 32 as core::ffi::c_int
                    >> 6 as core::ffi::c_int) as pixel;
                x += 1;
                base += base_inc;
            } else {
                pixel_set(
                    &raw mut *dst.offset(x as isize),
                    *top.offset(max_base_x as isize) as core::ffi::c_int,
                    width - x,
                );
                break;
            }
        }
        y += 1;
        dst = dst.offset(PXSTRIDE(stride) as isize);
        xpos += dx;
    }
}
unsafe extern "C" fn ipred_z2_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut angle: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let is_sm: core::ffi::c_int = angle >> 9 as core::ffi::c_int
        & 0x1 as core::ffi::c_int;
    let enable_intra_edge_filter: core::ffi::c_int = angle >> 10 as core::ffi::c_int;
    angle &= 511 as core::ffi::c_int;
    if angle > 90 as core::ffi::c_int && angle < 180 as core::ffi::c_int {} else {
        __assert_fail(
            b"angle > 90 && angle < 180\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            471 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 122],
                [core::ffi::c_char; 122],
            >(
                *b"void ipred_z2_c(pixel *, const ptrdiff_t, const pixel *const, const int, const int, int, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut dy: core::ffi::c_int = dav1d_dr_intra_derivative[(angle
        - 90 as core::ffi::c_int >> 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
    let mut dx: core::ffi::c_int = dav1d_dr_intra_derivative[(180 as core::ffi::c_int
        - angle >> 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
    let upsample_left: core::ffi::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 180 as core::ffi::c_int - angle, is_sm)
            as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let upsample_above: core::ffi::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 90 as core::ffi::c_int, is_sm)
            as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut edge: [pixel; 129] = [0; 129];
    let topleft: *mut pixel = &raw mut *edge
        .as_mut_ptr()
        .offset(64 as core::ffi::c_int as isize) as *mut pixel;
    if upsample_above != 0 {
        upsample_edge(
            topleft,
            width + 1 as core::ffi::c_int,
            topleft_in,
            0 as core::ffi::c_int,
            width + 1 as core::ffi::c_int,
            bitdepth_max,
        );
        dx <<= 1 as core::ffi::c_int;
    } else {
        let filter_strength: core::ffi::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90 as core::ffi::c_int, is_sm)
                as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                &raw mut *topleft.offset(1 as core::ffi::c_int as isize),
                width,
                0 as core::ffi::c_int,
                max_width,
                &*topleft_in.offset(1 as core::ffi::c_int as isize),
                -(1 as core::ffi::c_int),
                width,
                filter_strength,
            );
        } else {
            memcpy(
                &raw mut *topleft.offset(1 as core::ffi::c_int as isize) as *mut pixel
                    as *mut core::ffi::c_void,
                &*topleft_in.offset(1 as core::ffi::c_int as isize) as *const pixel
                    as *const core::ffi::c_void,
                (width << 1 as core::ffi::c_int) as size_t,
            );
        }
    }
    if upsample_left != 0 {
        upsample_edge(
            &raw mut *topleft.offset((-height * 2 as core::ffi::c_int) as isize),
            height + 1 as core::ffi::c_int,
            &*topleft_in.offset(-height as isize),
            0 as core::ffi::c_int,
            height + 1 as core::ffi::c_int,
            bitdepth_max,
        );
        dy <<= 1 as core::ffi::c_int;
    } else {
        let filter_strength_0: core::ffi::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 as core::ffi::c_int - angle, is_sm)
                as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        if filter_strength_0 != 0 {
            filter_edge(
                &raw mut *topleft.offset(-height as isize),
                height,
                height - max_height,
                height,
                &*topleft_in.offset(-height as isize),
                0 as core::ffi::c_int,
                height + 1 as core::ffi::c_int,
                filter_strength_0,
            );
        } else {
            memcpy(
                &raw mut *topleft.offset(-height as isize) as *mut pixel
                    as *mut core::ffi::c_void,
                &*topleft_in.offset(-height as isize) as *const pixel
                    as *const core::ffi::c_void,
                (height << 1 as core::ffi::c_int) as size_t,
            );
        }
    }
    *topleft = *topleft_in;
    let base_inc_x: core::ffi::c_int = 1 as core::ffi::c_int + upsample_above;
    let left: *const pixel = &raw mut *topleft
        .offset(-(1 as core::ffi::c_int + upsample_left) as isize) as *mut pixel;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut xpos: core::ffi::c_int = (1 as core::ffi::c_int + upsample_above
        << 6 as core::ffi::c_int) - dx;
    while y < height {
        let mut base_x: core::ffi::c_int = xpos >> 6 as core::ffi::c_int;
        let frac_x: core::ffi::c_int = xpos & 0x3e as core::ffi::c_int;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut ypos: core::ffi::c_int = (y << 6 as core::ffi::c_int + upsample_left)
            - dy;
        while x < width {
            let mut v: core::ffi::c_int = 0;
            if base_x >= 0 as core::ffi::c_int {
                v = *topleft.offset(base_x as isize) as core::ffi::c_int
                    * (64 as core::ffi::c_int - frac_x)
                    + *topleft.offset((base_x + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * frac_x;
            } else {
                let base_y: core::ffi::c_int = ypos >> 6 as core::ffi::c_int;
                if base_y >= -(1 as core::ffi::c_int + upsample_left) {} else {
                    __assert_fail(
                        b"base_y >= -(1 + upsample_left)\0" as *const u8
                            as *const core::ffi::c_char,
                        b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                            as *const core::ffi::c_char,
                        532 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 122],
                            [core::ffi::c_char; 122],
                        >(
                            *b"void ipred_z2_c(pixel *, const ptrdiff_t, const pixel *const, const int, const int, int, const int, const int, const int)\0",
                        ))
                            .as_ptr(),
                    );
                };
                let frac_y: core::ffi::c_int = ypos & 0x3e as core::ffi::c_int;
                v = *left.offset(-base_y as isize) as core::ffi::c_int
                    * (64 as core::ffi::c_int - frac_y)
                    + *left.offset(-(base_y + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * frac_y;
            }
            *dst.offset(x as isize) = (v + 32 as core::ffi::c_int
                >> 6 as core::ffi::c_int) as pixel;
            x += 1;
            base_x += base_inc_x;
            ypos -= dy;
        }
        y += 1;
        xpos -= dx;
        dst = dst.offset(PXSTRIDE(stride) as isize);
    }
}
unsafe extern "C" fn ipred_z3_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut angle: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let is_sm: core::ffi::c_int = angle >> 9 as core::ffi::c_int
        & 0x1 as core::ffi::c_int;
    let enable_intra_edge_filter: core::ffi::c_int = angle >> 10 as core::ffi::c_int;
    angle &= 511 as core::ffi::c_int;
    if angle > 180 as core::ffi::c_int {} else {
        __assert_fail(
            b"angle > 180\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            551 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 122],
                [core::ffi::c_char; 122],
            >(
                *b"void ipred_z3_c(pixel *, const ptrdiff_t, const pixel *const, const int, const int, int, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut dy: core::ffi::c_int = dav1d_dr_intra_derivative[(270 as core::ffi::c_int
        - angle >> 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
    let mut left_out: [pixel; 128] = [0; 128];
    let mut left: *const pixel = 0 as *const pixel;
    let mut max_base_y: core::ffi::c_int = 0;
    let upsample_left: core::ffi::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180 as core::ffi::c_int, is_sm)
            as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    if upsample_left != 0 {
        upsample_edge(
            left_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(-(width + height) as isize),
            imax(width - height, 0 as core::ffi::c_int),
            width + height + 1 as core::ffi::c_int,
            bitdepth_max,
        );
        left = &raw mut *left_out
            .as_mut_ptr()
            .offset(
                (2 as core::ffi::c_int * (width + height) - 2 as core::ffi::c_int)
                    as isize,
            ) as *mut pixel;
        max_base_y = 2 as core::ffi::c_int * (width + height) - 2 as core::ffi::c_int;
        dy <<= 1 as core::ffi::c_int;
    } else {
        let filter_strength: core::ffi::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180 as core::ffi::c_int, is_sm)
                as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                left_out.as_mut_ptr(),
                width + height,
                0 as core::ffi::c_int,
                width + height,
                &*topleft_in.offset(-(width + height) as isize),
                imax(width - height, 0 as core::ffi::c_int),
                width + height + 1 as core::ffi::c_int,
                filter_strength,
            );
            left = &raw mut *left_out
                .as_mut_ptr()
                .offset((width + height - 1 as core::ffi::c_int) as isize) as *mut pixel;
            max_base_y = width + height - 1 as core::ffi::c_int;
        } else {
            left = &*topleft_in.offset(-(1 as core::ffi::c_int) as isize)
                as *const pixel;
            max_base_y = height + imin(width, height) - 1 as core::ffi::c_int;
        }
    }
    let base_inc: core::ffi::c_int = 1 as core::ffi::c_int + upsample_left;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ypos: core::ffi::c_int = dy;
    while x < width {
        let frac: core::ffi::c_int = ypos & 0x3e as core::ffi::c_int;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut base: core::ffi::c_int = ypos >> 6 as core::ffi::c_int;
        while y < height {
            if base < max_base_y {
                let v: core::ffi::c_int = *left.offset(-base as isize)
                    as core::ffi::c_int * (64 as core::ffi::c_int - frac)
                    + *left.offset(-(base + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * frac;
                *dst
                    .offset(
                        (y as ptrdiff_t * PXSTRIDE(stride) + x as ptrdiff_t) as isize,
                    ) = (v + 32 as core::ffi::c_int >> 6 as core::ffi::c_int) as pixel;
                y += 1;
                base += base_inc;
            } else {
                loop {
                    *dst
                        .offset(
                            (y as ptrdiff_t * PXSTRIDE(stride) + x as ptrdiff_t) as isize,
                        ) = *left.offset(-max_base_y as isize);
                    y += 1;
                    if !(y < height) {
                        break;
                    }
                }
                break;
            }
        }
        x += 1;
        ypos += dy;
    }
}
pub const FLT_INCR: core::ffi::c_int = 2 as core::ffi::c_int;
unsafe extern "C" fn ipred_filter_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut filt_idx: core::ffi::c_int,
    max_width: core::ffi::c_int,
    max_height: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    filt_idx &= 511 as core::ffi::c_int;
    if filt_idx < 5 as core::ffi::c_int {} else {
        __assert_fail(
            b"filt_idx < 5\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            625 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 126],
                [core::ffi::c_char; 126],
            >(
                *b"void ipred_filter_c(pixel *, const ptrdiff_t, const pixel *const, const int, const int, int, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let filter: *const int8_t = (dav1d_filter_intra_taps[filt_idx as usize]).as_ptr();
    let mut top: *const pixel = &*topleft_in.offset(1 as core::ffi::c_int as isize)
        as *const pixel;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let mut topleft: *const pixel = &*topleft_in.offset(-y as isize) as *const pixel;
        let mut left: *const pixel = &*topleft.offset(-(1 as core::ffi::c_int) as isize)
            as *const pixel;
        let mut left_stride: ptrdiff_t = -(1 as core::ffi::c_int) as ptrdiff_t;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width {
            let p0: core::ffi::c_int = *topleft as core::ffi::c_int;
            let p1: core::ffi::c_int = *top.offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int;
            let p2: core::ffi::c_int = *top.offset(1 as core::ffi::c_int as isize)
                as core::ffi::c_int;
            let p3: core::ffi::c_int = *top.offset(2 as core::ffi::c_int as isize)
                as core::ffi::c_int;
            let p4: core::ffi::c_int = *top.offset(3 as core::ffi::c_int as isize)
                as core::ffi::c_int;
            let p5: core::ffi::c_int = *left
                .offset((0 as ptrdiff_t * left_stride) as isize) as core::ffi::c_int;
            let p6: core::ffi::c_int = *left
                .offset((1 as ptrdiff_t * left_stride) as isize) as core::ffi::c_int;
            let mut ptr: *mut pixel = &raw mut *dst.offset(x as isize) as *mut pixel;
            let mut flt_ptr: *const int8_t = filter;
            let mut yy: core::ffi::c_int = 0 as core::ffi::c_int;
            while yy < 2 as core::ffi::c_int {
                let mut xx: core::ffi::c_int = 0 as core::ffi::c_int;
                while xx < 4 as core::ffi::c_int {
                    let acc: core::ffi::c_int = *flt_ptr
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int * p0
                        + *flt_ptr.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int * p1
                        + *flt_ptr.offset(16 as core::ffi::c_int as isize)
                            as core::ffi::c_int * p2
                        + *flt_ptr.offset(17 as core::ffi::c_int as isize)
                            as core::ffi::c_int * p3
                        + *flt_ptr.offset(32 as core::ffi::c_int as isize)
                            as core::ffi::c_int * p4
                        + *flt_ptr.offset(33 as core::ffi::c_int as isize)
                            as core::ffi::c_int * p5
                        + *flt_ptr.offset(48 as core::ffi::c_int as isize)
                            as core::ffi::c_int * p6;
                    *ptr.offset(xx as isize) = iclip(
                        acc + 8 as core::ffi::c_int >> 4 as core::ffi::c_int,
                        0 as core::ffi::c_int,
                        bitdepth_max,
                    ) as pixel;
                    xx += 1;
                    flt_ptr = flt_ptr.offset(FLT_INCR as isize);
                }
                ptr = ptr.offset(PXSTRIDE(stride) as isize);
                yy += 1;
            }
            left = &raw mut *dst
                .offset((x + 4 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                as *mut pixel;
            left_stride = PXSTRIDE(stride);
            top = top.offset(4 as core::ffi::c_int as isize);
            topleft = &*top.offset(-(1 as core::ffi::c_int) as isize) as *const pixel;
            x += 4 as core::ffi::c_int;
        }
        top = &raw mut *dst
            .offset(
                (PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(stride)
                    as isize,
            ) as *mut pixel;
        dst = &raw mut *dst
            .offset(
                ((PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(stride)
                    * 2 as ptrdiff_t) as isize,
            ) as *mut pixel;
        y += 2 as core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn cfl_ac_c(
    mut ac: *mut int16_t,
    mut ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: core::ffi::c_int,
    h_pad: core::ffi::c_int,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    ss_hor: core::ffi::c_int,
    ss_ver: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let ac_orig: *mut int16_t = ac;
    if w_pad >= 0 as core::ffi::c_int && (w_pad * 4 as core::ffi::c_int) < width
    {} else {
        __assert_fail(
            b"w_pad >= 0 && w_pad * 4 < width\0" as *const u8
                as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            665 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 123],
                [core::ffi::c_char; 123],
            >(
                *b"void cfl_ac_c(int16_t *, const pixel *, const ptrdiff_t, const int, const int, const int, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    if h_pad >= 0 as core::ffi::c_int && (h_pad * 4 as core::ffi::c_int) < height
    {} else {
        __assert_fail(
            b"h_pad >= 0 && h_pad * 4 < height\0" as *const u8
                as *const core::ffi::c_char,
            b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            666 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 123],
                [core::ffi::c_char; 123],
            >(
                *b"void cfl_ac_c(int16_t *, const pixel *, const ptrdiff_t, const int, const int, const int, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    y = 0 as core::ffi::c_int;
    while y < height - 4 as core::ffi::c_int * h_pad {
        x = 0 as core::ffi::c_int;
        while x < width - 4 as core::ffi::c_int * w_pad {
            let mut ac_sum: core::ffi::c_int = *ypx.offset((x << ss_hor) as isize)
                as core::ffi::c_int;
            if ss_hor != 0 {
                ac_sum
                    += *ypx
                        .offset(
                            (x * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int;
            }
            if ss_ver != 0 {
                ac_sum
                    += *ypx
                        .offset(((x << ss_hor) as ptrdiff_t + PXSTRIDE(stride)) as isize)
                        as core::ffi::c_int;
                if ss_hor != 0 {
                    ac_sum
                        += *ypx
                            .offset(
                                ((x * 2 as core::ffi::c_int + 1 as core::ffi::c_int)
                                    as ptrdiff_t + PXSTRIDE(stride)) as isize,
                            ) as core::ffi::c_int;
                }
            }
            *ac.offset(x as isize) = (ac_sum
                << 1 as core::ffi::c_int + (ss_ver == 0) as core::ffi::c_int
                    + (ss_hor == 0) as core::ffi::c_int) as int16_t;
            x += 1;
        }
        while x < width {
            *ac.offset(x as isize) = *ac.offset((x - 1 as core::ffi::c_int) as isize);
            x += 1;
        }
        ac = ac.offset(width as isize);
        ypx = ypx.offset((PXSTRIDE(stride) << ss_ver) as isize);
        y += 1;
    }
    while y < height {
        memcpy(
            ac as *mut core::ffi::c_void,
            &raw mut *ac.offset(-width as isize) as *mut int16_t as *const core::ffi::c_void,
            (width as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        );
        ac = ac.offset(width as isize);
        y += 1;
    }
    let log2sz: core::ffi::c_int = ctz(width as core::ffi::c_uint) as core::ffi::c_int
        + ctz(height as core::ffi::c_uint) as core::ffi::c_int;
    let mut sum: core::ffi::c_int = (1 as core::ffi::c_int) << log2sz
        >> 1 as core::ffi::c_int;
    ac = ac_orig;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            sum += *ac.offset(x as isize) as core::ffi::c_int;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
    sum >>= log2sz;
    ac = ac_orig;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            let fresh1 = &raw mut *ac.offset(x as isize);
            *fresh1 = (*fresh1 as core::ffi::c_int - sum) as int16_t;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
}
unsafe extern "C" fn cfl_ac_420_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: core::ffi::c_int,
    h_pad: core::ffi::c_int,
    cw: core::ffi::c_int,
    ch: core::ffi::c_int,
) {
    cfl_ac_c(
        ac,
        ypx,
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
}
unsafe extern "C" fn cfl_ac_422_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: core::ffi::c_int,
    h_pad: core::ffi::c_int,
    cw: core::ffi::c_int,
    ch: core::ffi::c_int,
) {
    cfl_ac_c(
        ac,
        ypx,
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
unsafe extern "C" fn cfl_ac_444_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: core::ffi::c_int,
    h_pad: core::ffi::c_int,
    cw: core::ffi::c_int,
    ch: core::ffi::c_int,
) {
    cfl_ac_c(
        ac,
        ypx,
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
unsafe extern "C" fn pal_pred_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    pal: *const pixel,
    mut idx: *const uint8_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let fresh0 = idx;
            idx = idx.offset(1);
            let i: core::ffi::c_int = *fresh0 as core::ffi::c_int;
            if i & 0x88 as core::ffi::c_int == 0 {} else {
                __assert_fail(
                    b"!(i & 0x88)\0" as *const u8 as *const core::ffi::c_char,
                    b"../dav1d-native-source/src/ipred_tmpl.c\0" as *const u8
                        as *const core::ffi::c_char,
                    724 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 101],
                        [core::ffi::c_char; 101],
                    >(
                        *b"void pal_pred_c(pixel *, const ptrdiff_t, const pixel *const, const uint8_t *, const int, const int)\0",
                    ))
                        .as_ptr(),
                );
            };
            *dst.offset((x + 0 as core::ffi::c_int) as isize) = *pal
                .offset((i & 7 as core::ffi::c_int) as isize);
            *dst.offset((x + 1 as core::ffi::c_int) as isize) = *pal
                .offset((i >> 4 as core::ffi::c_int) as isize);
            x += 2 as core::ffi::c_int;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_intra_pred_dsp_init_16bpc(
    c: *mut Dav1dIntraPredDSPContext,
) {
    (*c).intra_pred[DC_PRED as core::ffi::c_int as usize] = Some(
        ipred_dc_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[DC_128_PRED as core::ffi::c_int as usize] = Some(
        ipred_dc_128_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[TOP_DC_PRED as core::ffi::c_int as usize] = Some(
        ipred_dc_top_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[LEFT_DC_PRED as core::ffi::c_int as usize] = Some(
        ipred_dc_left_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[HOR_PRED as core::ffi::c_int as usize] = Some(
        ipred_h_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[VERT_PRED as core::ffi::c_int as usize] = Some(
        ipred_v_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[PAETH_PRED as core::ffi::c_int as usize] = Some(
        ipred_paeth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[SMOOTH_PRED as core::ffi::c_int as usize] = Some(
        ipred_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[SMOOTH_V_PRED as core::ffi::c_int as usize] = Some(
        ipred_smooth_v_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[SMOOTH_H_PRED as core::ffi::c_int as usize] = Some(
        ipred_smooth_h_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[Z1_PRED as core::ffi::c_int as usize] = Some(
        ipred_z1_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[Z2_PRED as core::ffi::c_int as usize] = Some(
        ipred_z2_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[Z3_PRED as core::ffi::c_int as usize] = Some(
        ipred_z3_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c).intra_pred[FILTER_PRED as core::ffi::c_int as usize] = Some(
        ipred_filter_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as angular_ipred_fn;
    (*c)
        .cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int - 1 as core::ffi::c_int)
        as usize] = Some(
        cfl_ac_420_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as cfl_ac_fn;
    (*c)
        .cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 as core::ffi::c_int - 1 as core::ffi::c_int)
        as usize] = Some(
        cfl_ac_422_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as cfl_ac_fn;
    (*c)
        .cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int - 1 as core::ffi::c_int)
        as usize] = Some(
        cfl_ac_444_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as cfl_ac_fn;
    (*c).cfl_pred[DC_PRED as core::ffi::c_int as usize] = Some(
        ipred_cfl_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as cfl_pred_fn;
    (*c).cfl_pred[DC_128_PRED as core::ffi::c_int as usize] = Some(
        ipred_cfl_128_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as cfl_pred_fn;
    (*c).cfl_pred[TOP_DC_PRED as core::ffi::c_int as usize] = Some(
        ipred_cfl_top_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as cfl_pred_fn;
    (*c).cfl_pred[LEFT_DC_PRED as core::ffi::c_int as usize] = Some(
        ipred_cfl_left_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as cfl_pred_fn;
    (*c).pal_pred = Some(
        pal_pred_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                *const uint8_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as pal_pred_fn;
}
