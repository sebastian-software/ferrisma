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

// Mechanically translated from upstream/dav1d/src/mc_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    static dav1d_mc_subpel_filters: [[[int8_t; 8]; 15]; 6];
    static dav1d_mc_warp_filter: [[int8_t; 8]; 193];
    static dav1d_resize_filter: [[int8_t; 8]; 64];
    static dav1d_obmc_masks: [uint8_t; 64];
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type ptrdiff_t = isize;
pub type uint8_t = __uint8_t;
pub type intptr_t = isize;
pub type pixel = uint8_t;
pub type Dav1dFilterMode = core::ffi::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
pub type Filter2d = core::ffi::c_uint;
pub const N_2D_FILTERS: Filter2d = 10;
pub const FILTER_2D_BILINEAR: Filter2d = 9;
pub const FILTER_2D_8TAP_SMOOTH_SHARP: Filter2d = 8;
pub const FILTER_2D_8TAP_SMOOTH: Filter2d = 7;
pub const FILTER_2D_8TAP_SMOOTH_REGULAR: Filter2d = 6;
pub const FILTER_2D_8TAP_SHARP: Filter2d = 5;
pub const FILTER_2D_8TAP_SHARP_SMOOTH: Filter2d = 4;
pub const FILTER_2D_8TAP_SHARP_REGULAR: Filter2d = 3;
pub const FILTER_2D_8TAP_REGULAR_SHARP: Filter2d = 2;
pub const FILTER_2D_8TAP_REGULAR_SMOOTH: Filter2d = 1;
pub const FILTER_2D_8TAP_REGULAR: Filter2d = 0;
pub type mc_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type mct_fn = Option<
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
pub type mct_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type warp8x8t_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type avg_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type w_avg_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type mask_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
    ) -> (),
>;
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type emu_edge_fn = Option<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
    ) -> (),
>;
pub type resize_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
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
unsafe extern "C" fn iclip_u8(v: core::ffi::c_int) -> core::ffi::c_int {
    return iclip(v, 0 as core::ffi::c_int, 255 as core::ffi::c_int);
}
pub const PREP_BIAS: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline(never)]
unsafe extern "C" fn put_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    loop {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            w as size_t,
        );
        dst = dst.offset(dst_stride as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn prep_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *tmp.offset(x as isize) = (((*src.offset(x as isize) as core::ffi::c_int)
                << intermediate_bits) - PREP_BIAS) as int16_t;
            x += 1;
        }
        tmp = tmp.offset(w as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn put_8tap_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    filter_type: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let intermediate_rnd: core::ffi::c_int = 32 as core::ffi::c_int
        + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int - intermediate_bits
            >> 1 as core::ffi::c_int);
    let fh: *const int8_t = if mx == 0 {
        0 as *const int8_t
    } else if w > 4 as core::ffi::c_int {
        (dav1d_mc_subpel_filters[(filter_type & 3 as core::ffi::c_int)
            as usize][(mx - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
            + (filter_type & 1 as core::ffi::c_int))
            as usize][(mx - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    };
    let fv: *const int8_t = if my == 0 {
        0 as *const int8_t
    } else if h > 4 as core::ffi::c_int {
        (dav1d_mc_subpel_filters[(filter_type >> 2 as core::ffi::c_int)
            as usize][(my - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
            + (filter_type >> 2 as core::ffi::c_int & 1 as core::ffi::c_int))
            as usize][(my - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    };
    dst_stride = dst_stride;
    src_stride = src_stride;
    if !fh.is_null() {
        if !fv.is_null() {
            let mut tmp_h: core::ffi::c_int = h + 7 as core::ffi::c_int;
            let mut mid: [int16_t; 17280] = [0; 17280];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            src = src.offset(-((src_stride * 3 as ptrdiff_t) as isize));
            loop {
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < w {
                    *mid_ptr.offset(x as isize) = (*fh
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x + -(3 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                        + *fh.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + -(2 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + -(1 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 0 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 1 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 2 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 3 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int)
                            << 6 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 6 as core::ffi::c_int - intermediate_bits) as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid
                .as_mut_ptr()
                .offset((128 as core::ffi::c_int * 3 as core::ffi::c_int) as isize);
            loop {
                let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_0 < w {
                    *dst.offset(x_0 as isize) = iclip_u8(
                        *fv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(3 as core::ffi::c_int) * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                            + *fv.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + -(2 as core::ffi::c_int) * 128 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fv.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + -(1 as core::ffi::c_int) * 128 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fv.offset(3 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 0 as core::ffi::c_int * 128 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fv.offset(4 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 1 as core::ffi::c_int * 128 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fv.offset(5 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 2 as core::ffi::c_int * 128 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fv.offset(6 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 3 as core::ffi::c_int * 128 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fv.offset(7 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 4 as core::ffi::c_int * 128 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + ((1 as core::ffi::c_int)
                                << 6 as core::ffi::c_int + intermediate_bits
                                >> 1 as core::ffi::c_int)
                            >> 6 as core::ffi::c_int + intermediate_bits,
                    ) as pixel;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                dst = dst.offset(dst_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_1 < w {
                    *dst.offset(x_1 as isize) = iclip_u8(
                        *fh.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + -(3 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                            + *fh.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *src
                                    .offset(
                                        (x_1 + -(2 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fh.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *src
                                    .offset(
                                        (x_1 + -(1 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fh.offset(3 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *src
                                    .offset(
                                        (x_1 + 0 as core::ffi::c_int * 1 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fh.offset(4 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *src
                                    .offset(
                                        (x_1 + 1 as core::ffi::c_int * 1 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fh.offset(5 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *src
                                    .offset(
                                        (x_1 + 2 as core::ffi::c_int * 1 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fh.offset(6 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *src
                                    .offset(
                                        (x_1 + 3 as core::ffi::c_int * 1 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int
                            + *fh.offset(7 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *src
                                    .offset(
                                        (x_1 + 4 as core::ffi::c_int * 1 as core::ffi::c_int)
                                            as isize,
                                    ) as core::ffi::c_int + intermediate_rnd
                            >> 6 as core::ffi::c_int,
                    ) as pixel;
                    x_1 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if !fv.is_null() {
        loop {
            let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_2 < w {
                *dst.offset(x_2 as isize) = iclip_u8(
                    *fv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + -(3 as core::ffi::c_int) as ptrdiff_t * src_stride)
                                    as isize,
                            ) as core::ffi::c_int
                        + *fv.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_2 as ptrdiff_t
                                        + -(2 as core::ffi::c_int) as ptrdiff_t * src_stride)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_2 as ptrdiff_t
                                        + -(1 as core::ffi::c_int) as ptrdiff_t * src_stride)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_2 as ptrdiff_t
                                        + 0 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_2 as ptrdiff_t
                                        + 1 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_2 as ptrdiff_t
                                        + 2 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_2 as ptrdiff_t
                                        + 3 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_2 as ptrdiff_t
                                        + 4 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                                ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int,
                ) as pixel;
                x_2 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        put_c(dst, dst_stride, src, src_stride, w, h);
    };
}
#[inline(never)]
unsafe extern "C" fn put_8tap_scaled_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    mut my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
    filter_type: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let intermediate_rnd: core::ffi::c_int = (1 as core::ffi::c_int) << intermediate_bits
        >> 1 as core::ffi::c_int;
    let mut mid: [int16_t; 1024] = [0; 1024];
    let mut mid_ptrs: [*mut int16_t; 8] = [0 as *mut int16_t; 8];
    let mut in_y: core::ffi::c_int = -(8 as core::ffi::c_int);
    src_stride = src_stride;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        mid_ptrs[i as usize] = &raw mut *mid
            .as_mut_ptr()
            .offset((128 as core::ffi::c_int * i) as isize) as *mut int16_t;
        i += 1;
    }
    src = src.offset(-((src_stride * 3 as ptrdiff_t) as isize));
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0;
        let mut src_y: core::ffi::c_int = my >> 10 as core::ffi::c_int;
        let fv: *const int8_t = if (my & 0x3ff as core::ffi::c_int)
            >> 6 as core::ffi::c_int == 0
        {
            0 as *const int8_t
        } else if h > 4 as core::ffi::c_int {
            (dav1d_mc_subpel_filters[(filter_type >> 2 as core::ffi::c_int)
                as usize][(((my & 0x3ff as core::ffi::c_int) >> 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as usize])
                .as_ptr()
        } else {
            (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
                + (filter_type >> 2 as core::ffi::c_int & 1 as core::ffi::c_int))
                as usize][(((my & 0x3ff as core::ffi::c_int) >> 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as usize])
                .as_ptr()
        };
        while in_y < src_y {
            let mut imx: core::ffi::c_int = mx;
            let mut ioff: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut mid_ptr: *mut int16_t = mid_ptrs[0 as core::ffi::c_int as usize];
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < 7 as core::ffi::c_int {
                mid_ptrs[i_0 as usize] = mid_ptrs[(i_0 + 1 as core::ffi::c_int)
                    as usize];
                i_0 += 1;
            }
            mid_ptrs[7 as core::ffi::c_int as usize] = mid_ptr;
            x = 0 as core::ffi::c_int;
            while x < w {
                let fh: *const int8_t = if imx >> 6 as core::ffi::c_int == 0 {
                    0 as *const int8_t
                } else if w > 4 as core::ffi::c_int {
                    (dav1d_mc_subpel_filters[(filter_type & 3 as core::ffi::c_int)
                        as usize][((imx >> 6 as core::ffi::c_int)
                        - 1 as core::ffi::c_int) as usize])
                        .as_ptr()
                } else {
                    (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
                        + (filter_type & 1 as core::ffi::c_int))
                        as usize][((imx >> 6 as core::ffi::c_int)
                        - 1 as core::ffi::c_int) as usize])
                        .as_ptr()
                };
                *mid_ptr.offset(x as isize) = (if !fh.is_null() {
                    *fh.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (ioff + -(3 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                        + *fh.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + -(2 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + -(1 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 0 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 1 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 2 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 3 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 4 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int)
                            << 6 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 6 as core::ffi::c_int - intermediate_bits
                } else {
                    (*src.offset(ioff as isize) as core::ffi::c_int) << intermediate_bits
                }) as int16_t;
                imx += dx;
                ioff += imx >> 10 as core::ffi::c_int;
                imx &= 0x3ff as core::ffi::c_int;
                x += 1;
            }
            src = src.offset(src_stride as isize);
            in_y += 1;
        }
        x = 0 as core::ffi::c_int;
        while x < w {
            *dst.offset(x as isize) = (if !fv.is_null() {
                iclip_u8(
                    *fv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[0 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                        + *fv.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *(mid_ptrs[1 as core::ffi::c_int as usize])
                                .offset(x as isize) as core::ffi::c_int
                        + *fv.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *(mid_ptrs[2 as core::ffi::c_int as usize])
                                .offset(x as isize) as core::ffi::c_int
                        + *fv.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *(mid_ptrs[3 as core::ffi::c_int as usize])
                                .offset(x as isize) as core::ffi::c_int
                        + *fv.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *(mid_ptrs[4 as core::ffi::c_int as usize])
                                .offset(x as isize) as core::ffi::c_int
                        + *fv.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *(mid_ptrs[5 as core::ffi::c_int as usize])
                                .offset(x as isize) as core::ffi::c_int
                        + *fv.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *(mid_ptrs[6 as core::ffi::c_int as usize])
                                .offset(x as isize) as core::ffi::c_int
                        + *fv.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *(mid_ptrs[7 as core::ffi::c_int as usize])
                                .offset(x as isize) as core::ffi::c_int
                        + ((1 as core::ffi::c_int)
                            << 6 as core::ffi::c_int + intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 6 as core::ffi::c_int + intermediate_bits,
                )
            } else {
                iclip_u8(
                    *(mid_ptrs[3 as core::ffi::c_int as usize]).offset(x as isize)
                        as core::ffi::c_int + intermediate_rnd >> intermediate_bits,
                )
            }) as pixel;
            x += 1;
        }
        my += dy;
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn prep_8tap_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    filter_type: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let fh: *const int8_t = if mx == 0 {
        0 as *const int8_t
    } else if w > 4 as core::ffi::c_int {
        (dav1d_mc_subpel_filters[(filter_type & 3 as core::ffi::c_int)
            as usize][(mx - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
            + (filter_type & 1 as core::ffi::c_int))
            as usize][(mx - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    };
    let fv: *const int8_t = if my == 0 {
        0 as *const int8_t
    } else if h > 4 as core::ffi::c_int {
        (dav1d_mc_subpel_filters[(filter_type >> 2 as core::ffi::c_int)
            as usize][(my - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
            + (filter_type >> 2 as core::ffi::c_int & 1 as core::ffi::c_int))
            as usize][(my - 1 as core::ffi::c_int) as usize])
            .as_ptr()
    };
    src_stride = src_stride;
    if !fh.is_null() {
        if !fv.is_null() {
            let mut tmp_h: core::ffi::c_int = h + 7 as core::ffi::c_int;
            let mut mid: [int16_t; 17280] = [0; 17280];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            src = src.offset(-((src_stride * 3 as ptrdiff_t) as isize));
            loop {
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < w {
                    *mid_ptr.offset(x as isize) = (*fh
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x + -(3 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                        + *fh.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + -(2 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + -(1 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 0 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 1 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 2 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 3 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x + 4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int)
                            << 6 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 6 as core::ffi::c_int - intermediate_bits) as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid
                .as_mut_ptr()
                .offset((128 as core::ffi::c_int * 3 as core::ffi::c_int) as isize);
            loop {
                let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_0 < w {
                    let mut t: core::ffi::c_int = (*fv
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + -(3 as core::ffi::c_int) * 128 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                        + *fv.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(2 as core::ffi::c_int) * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(1 as core::ffi::c_int) * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 0 as core::ffi::c_int * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 1 as core::ffi::c_int * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 2 as core::ffi::c_int * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 3 as core::ffi::c_int * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fv.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 4 as core::ffi::c_int * 128 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int)
                        - PREP_BIAS;
                    if t >= -(32767 as core::ffi::c_int) - 1 as core::ffi::c_int
                        && t <= 32767 as core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"t >= INT16_MIN && t <= INT16_MAX\0" as *const u8
                                as *const core::ffi::c_char,
                            b"../dav1d-native-source/src/mc_tmpl.c\0" as *const u8
                                as *const core::ffi::c_char,
                            275 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 103],
                                [core::ffi::c_char; 103],
                            >(
                                *b"void prep_8tap_c(int16_t *, const pixel *, ptrdiff_t, const int, int, const int, const int, const int)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    *tmp.offset(x_0 as isize) = t as int16_t;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                tmp = tmp.offset(w as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_1 < w {
                    *tmp.offset(x_1 as isize) = ((*fh
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_1 + -(3 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                        + *fh.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + -(2 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + -(1 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + 0 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + 1 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + 2 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + 3 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (x_1 + 4 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int)
                            << 6 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 6 as core::ffi::c_int - intermediate_bits) - PREP_BIAS)
                        as int16_t;
                    x_1 += 1;
                }
                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if !fv.is_null() {
        loop {
            let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_2 < w {
                *tmp.offset(x_2 as isize) = ((*fv.offset(0 as core::ffi::c_int as isize)
                    as core::ffi::c_int
                    * *src
                        .offset(
                            (x_2 as ptrdiff_t
                                + -(3 as core::ffi::c_int) as ptrdiff_t * src_stride)
                                as isize,
                        ) as core::ffi::c_int
                    + *fv.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + -(2 as core::ffi::c_int) as ptrdiff_t * src_stride)
                                    as isize,
                            ) as core::ffi::c_int
                    + *fv.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + -(1 as core::ffi::c_int) as ptrdiff_t * src_stride)
                                    as isize,
                            ) as core::ffi::c_int
                    + *fv.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + 0 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                            ) as core::ffi::c_int
                    + *fv.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + 1 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                            ) as core::ffi::c_int
                    + *fv.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + 2 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                            ) as core::ffi::c_int
                    + *fv.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + 3 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                            ) as core::ffi::c_int
                    + *fv.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (x_2 as ptrdiff_t
                                    + 4 as core::ffi::c_int as ptrdiff_t * src_stride) as isize,
                            ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int)
                        << 6 as core::ffi::c_int - intermediate_bits
                        >> 1 as core::ffi::c_int)
                    >> 6 as core::ffi::c_int - intermediate_bits) - PREP_BIAS)
                    as int16_t;
                x_2 += 1;
            }
            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        prep_c(tmp, src, src_stride, w, h);
    };
}
#[inline(never)]
unsafe extern "C" fn prep_8tap_scaled_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    mut my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
    filter_type: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut mid: [int16_t; 1024] = [0; 1024];
    let mut mid_ptrs: [*mut int16_t; 8] = [0 as *mut int16_t; 8];
    let mut in_y: core::ffi::c_int = -(8 as core::ffi::c_int);
    src_stride = src_stride;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        mid_ptrs[i as usize] = &raw mut *mid
            .as_mut_ptr()
            .offset((128 as core::ffi::c_int * i) as isize) as *mut int16_t;
        i += 1;
    }
    src = src.offset(-((src_stride * 3 as ptrdiff_t) as isize));
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0;
        let mut src_y: core::ffi::c_int = my >> 10 as core::ffi::c_int;
        let fv: *const int8_t = if (my & 0x3ff as core::ffi::c_int)
            >> 6 as core::ffi::c_int == 0
        {
            0 as *const int8_t
        } else if h > 4 as core::ffi::c_int {
            (dav1d_mc_subpel_filters[(filter_type >> 2 as core::ffi::c_int)
                as usize][(((my & 0x3ff as core::ffi::c_int) >> 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as usize])
                .as_ptr()
        } else {
            (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
                + (filter_type >> 2 as core::ffi::c_int & 1 as core::ffi::c_int))
                as usize][(((my & 0x3ff as core::ffi::c_int) >> 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as usize])
                .as_ptr()
        };
        while in_y < src_y {
            let mut imx: core::ffi::c_int = mx;
            let mut ioff: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut mid_ptr: *mut int16_t = mid_ptrs[0 as core::ffi::c_int as usize];
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < 7 as core::ffi::c_int {
                mid_ptrs[i_0 as usize] = mid_ptrs[(i_0 + 1 as core::ffi::c_int)
                    as usize];
                i_0 += 1;
            }
            mid_ptrs[7 as core::ffi::c_int as usize] = mid_ptr;
            x = 0 as core::ffi::c_int;
            while x < w {
                let fh: *const int8_t = if imx >> 6 as core::ffi::c_int == 0 {
                    0 as *const int8_t
                } else if w > 4 as core::ffi::c_int {
                    (dav1d_mc_subpel_filters[(filter_type & 3 as core::ffi::c_int)
                        as usize][((imx >> 6 as core::ffi::c_int)
                        - 1 as core::ffi::c_int) as usize])
                        .as_ptr()
                } else {
                    (dav1d_mc_subpel_filters[(3 as core::ffi::c_int
                        + (filter_type & 1 as core::ffi::c_int))
                        as usize][((imx >> 6 as core::ffi::c_int)
                        - 1 as core::ffi::c_int) as usize])
                        .as_ptr()
                };
                *mid_ptr.offset(x as isize) = (if !fh.is_null() {
                    *fh.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (ioff + -(3 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                        + *fh.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + -(2 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + -(1 as core::ffi::c_int) * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 0 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 1 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 2 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 3 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + *fh.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                            * *src
                                .offset(
                                    (ioff + 4 as core::ffi::c_int * 1 as core::ffi::c_int)
                                        as isize,
                                ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int)
                            << 6 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 6 as core::ffi::c_int - intermediate_bits
                } else {
                    (*src.offset(ioff as isize) as core::ffi::c_int) << intermediate_bits
                }) as int16_t;
                imx += dx;
                ioff += imx >> 10 as core::ffi::c_int;
                imx &= 0x3ff as core::ffi::c_int;
                x += 1;
            }
            src = src.offset(src_stride as isize);
            in_y += 1;
        }
        x = 0 as core::ffi::c_int;
        while x < w {
            *tmp.offset(x as isize) = ((if !fv.is_null() {
                *fv.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *(mid_ptrs[0 as core::ffi::c_int as usize]).offset(x as isize)
                        as core::ffi::c_int
                    + *fv.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[1 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                    + *fv.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[2 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                    + *fv.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[3 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                    + *fv.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[4 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                    + *fv.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[5 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                    + *fv.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[6 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                    + *fv.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *(mid_ptrs[7 as core::ffi::c_int as usize]).offset(x as isize)
                            as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int
            } else {
                *(mid_ptrs[3 as core::ffi::c_int as usize]).offset(x as isize)
                    as core::ffi::c_int
            }) - PREP_BIAS) as int16_t;
            x += 1;
        }
        my += dy;
        tmp = tmp.offset(w as isize);
        y += 1;
    }
}
unsafe extern "C" fn put_8tap_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_regular_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_regular_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_regular_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_regular_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_regular_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_regular_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_regular_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_regular_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_smooth_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_smooth_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_smooth_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_smooth_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_smooth_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_smooth_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_smooth_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_smooth_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_sharp_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_sharp_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_sharp_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_sharp_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_sharp_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_8tap_sharp_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_sharp_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn prep_8tap_sharp_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as core::ffi::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as core::ffi::c_int) << 2 as core::ffi::c_int,
    );
}
unsafe extern "C" fn put_bilin_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let intermediate_rnd: core::ffi::c_int = (1 as core::ffi::c_int) << intermediate_bits
        >> 1 as core::ffi::c_int;
    dst_stride = dst_stride;
    src_stride = src_stride;
    if mx != 0 {
        if my != 0 {
            let mut mid: [int16_t; 16512] = [0; 16512];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            let mut tmp_h: core::ffi::c_int = h + 1 as core::ffi::c_int;
            loop {
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < w {
                    *mid_ptr.offset(x as isize) = (16 as core::ffi::c_int
                        * *src.offset(x as isize) as core::ffi::c_int
                        + mx
                            * (*src.offset((x + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int
                                - *src.offset(x as isize) as core::ffi::c_int)
                        + ((1 as core::ffi::c_int)
                            << 4 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 4 as core::ffi::c_int - intermediate_bits) as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr();
            loop {
                let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_0 < w {
                    *dst.offset(x_0 as isize) = iclip_u8(
                        16 as core::ffi::c_int
                            * *mid_ptr.offset(x_0 as isize) as core::ffi::c_int
                            + my
                                * (*mid_ptr.offset((x_0 + 128 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int
                                    - *mid_ptr.offset(x_0 as isize) as core::ffi::c_int)
                            + ((1 as core::ffi::c_int)
                                << 4 as core::ffi::c_int + intermediate_bits
                                >> 1 as core::ffi::c_int)
                            >> 4 as core::ffi::c_int + intermediate_bits,
                    ) as pixel;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                dst = dst.offset(dst_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_1 < w {
                    let px: core::ffi::c_int = 16 as core::ffi::c_int
                        * *src.offset(x_1 as isize) as core::ffi::c_int
                        + mx
                            * (*src.offset((x_1 + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int
                                - *src.offset(x_1 as isize) as core::ffi::c_int)
                        + ((1 as core::ffi::c_int)
                            << 4 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 4 as core::ffi::c_int - intermediate_bits;
                    *dst.offset(x_1 as isize) = iclip_u8(
                        px + intermediate_rnd >> intermediate_bits,
                    ) as pixel;
                    x_1 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if my != 0 {
        loop {
            let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_2 < w {
                *dst.offset(x_2 as isize) = iclip_u8(
                    16 as core::ffi::c_int
                        * *src.offset(x_2 as isize) as core::ffi::c_int
                        + my
                            * (*src.offset((x_2 as ptrdiff_t + src_stride) as isize)
                                as core::ffi::c_int
                                - *src.offset(x_2 as isize) as core::ffi::c_int)
                        + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 4 as core::ffi::c_int,
                ) as pixel;
                x_2 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        put_c(dst, dst_stride, src, src_stride, w, h);
    };
}
unsafe extern "C" fn put_bilin_scaled_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    mut my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut mid: [int16_t; 256] = [0; 256];
    let mut in_y: core::ffi::c_int = -(2 as core::ffi::c_int);
    loop {
        let mut x: core::ffi::c_int = 0;
        let mut y: core::ffi::c_int = my >> 10 as core::ffi::c_int;
        let mut mid1: *mut int16_t = &raw mut *mid
            .as_mut_ptr()
            .offset(((y & 1 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize)
            as *mut int16_t;
        let mut mid2: *mut int16_t = &raw mut *mid
            .as_mut_ptr()
            .offset(
                ((y + 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                    * 128 as core::ffi::c_int) as isize,
            ) as *mut int16_t;
        let mut dmy: core::ffi::c_int = my & 0x3ff as core::ffi::c_int;
        while in_y < y {
            let mut imx: core::ffi::c_int = mx;
            let mut ioff: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut mid_ptr: *mut int16_t = &raw mut *mid
                .as_mut_ptr()
                .offset(
                    ((in_y & 1 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize,
                ) as *mut int16_t;
            x = 0 as core::ffi::c_int;
            while x < w {
                *mid_ptr.offset(x as isize) = (16 as core::ffi::c_int
                    * *src.offset(ioff as isize) as core::ffi::c_int
                    + (imx >> 6 as core::ffi::c_int)
                        * (*src.offset((ioff + 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int
                            - *src.offset(ioff as isize) as core::ffi::c_int)
                    + ((1 as core::ffi::c_int)
                        << 4 as core::ffi::c_int - intermediate_bits
                        >> 1 as core::ffi::c_int)
                    >> 4 as core::ffi::c_int - intermediate_bits) as int16_t;
                imx += dx;
                ioff += imx >> 10 as core::ffi::c_int;
                imx &= 0x3ff as core::ffi::c_int;
                x += 1;
            }
            src = src.offset(src_stride as isize);
            in_y += 1;
        }
        x = 0 as core::ffi::c_int;
        while x < w {
            *dst.offset(x as isize) = iclip_u8(
                16 as core::ffi::c_int * *mid1.offset(x as isize) as core::ffi::c_int
                    + (dmy >> 6 as core::ffi::c_int)
                        * (*mid2.offset(x as isize) as core::ffi::c_int
                            - *mid1.offset(x as isize) as core::ffi::c_int)
                    + ((1 as core::ffi::c_int)
                        << 4 as core::ffi::c_int + intermediate_bits
                        >> 1 as core::ffi::c_int)
                    >> 4 as core::ffi::c_int + intermediate_bits,
            ) as pixel;
            x += 1;
        }
        my += dy;
        dst = dst.offset(dst_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn prep_bilin_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    my: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    src_stride = src_stride;
    if mx != 0 {
        if my != 0 {
            let mut mid: [int16_t; 16512] = [0; 16512];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            let mut tmp_h: core::ffi::c_int = h + 1 as core::ffi::c_int;
            loop {
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < w {
                    *mid_ptr.offset(x as isize) = (16 as core::ffi::c_int
                        * *src.offset(x as isize) as core::ffi::c_int
                        + mx
                            * (*src.offset((x + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int
                                - *src.offset(x as isize) as core::ffi::c_int)
                        + ((1 as core::ffi::c_int)
                            << 4 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 4 as core::ffi::c_int - intermediate_bits) as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr();
            loop {
                let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_0 < w {
                    *tmp.offset(x_0 as isize) = ((16 as core::ffi::c_int
                        * *mid_ptr.offset(x_0 as isize) as core::ffi::c_int
                        + my
                            * (*mid_ptr.offset((x_0 + 128 as core::ffi::c_int) as isize)
                                as core::ffi::c_int
                                - *mid_ptr.offset(x_0 as isize) as core::ffi::c_int)
                        + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 4 as core::ffi::c_int)
                        - PREP_BIAS) as int16_t;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128 as core::ffi::c_int as isize);
                tmp = tmp.offset(w as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_1 < w {
                    *tmp.offset(x_1 as isize) = ((16 as core::ffi::c_int
                        * *src.offset(x_1 as isize) as core::ffi::c_int
                        + mx
                            * (*src.offset((x_1 + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int
                                - *src.offset(x_1 as isize) as core::ffi::c_int)
                        + ((1 as core::ffi::c_int)
                            << 4 as core::ffi::c_int - intermediate_bits
                            >> 1 as core::ffi::c_int)
                        >> 4 as core::ffi::c_int - intermediate_bits) - PREP_BIAS)
                        as int16_t;
                    x_1 += 1;
                }
                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if my != 0 {
        loop {
            let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_2 < w {
                *tmp.offset(x_2 as isize) = ((16 as core::ffi::c_int
                    * *src.offset(x_2 as isize) as core::ffi::c_int
                    + my
                        * (*src.offset((x_2 as ptrdiff_t + src_stride) as isize)
                            as core::ffi::c_int
                            - *src.offset(x_2 as isize) as core::ffi::c_int)
                    + ((1 as core::ffi::c_int)
                        << 4 as core::ffi::c_int - intermediate_bits
                        >> 1 as core::ffi::c_int)
                    >> 4 as core::ffi::c_int - intermediate_bits) - PREP_BIAS)
                    as int16_t;
                x_2 += 1;
            }
            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        prep_c(tmp, src, src_stride, w, h);
    };
}
unsafe extern "C" fn prep_bilin_scaled_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mx: core::ffi::c_int,
    mut my: core::ffi::c_int,
    dx: core::ffi::c_int,
    dy: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut mid: [int16_t; 256] = [0; 256];
    let mut in_y: core::ffi::c_int = -(2 as core::ffi::c_int);
    loop {
        let mut x: core::ffi::c_int = 0;
        let mut y: core::ffi::c_int = my >> 10 as core::ffi::c_int;
        let mut mid1: *mut int16_t = &raw mut *mid
            .as_mut_ptr()
            .offset(((y & 1 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize)
            as *mut int16_t;
        let mut mid2: *mut int16_t = &raw mut *mid
            .as_mut_ptr()
            .offset(
                ((y + 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                    * 128 as core::ffi::c_int) as isize,
            ) as *mut int16_t;
        let mut dmy: core::ffi::c_int = my & 0x3ff as core::ffi::c_int;
        while in_y < y {
            let mut imx: core::ffi::c_int = mx;
            let mut ioff: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut mid_ptr: *mut int16_t = &raw mut *mid
                .as_mut_ptr()
                .offset(
                    ((in_y & 1 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize,
                ) as *mut int16_t;
            x = 0 as core::ffi::c_int;
            while x < w {
                *mid_ptr.offset(x as isize) = (16 as core::ffi::c_int
                    * *src.offset(ioff as isize) as core::ffi::c_int
                    + (imx >> 6 as core::ffi::c_int)
                        * (*src.offset((ioff + 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int
                            - *src.offset(ioff as isize) as core::ffi::c_int)
                    + ((1 as core::ffi::c_int)
                        << 4 as core::ffi::c_int - intermediate_bits
                        >> 1 as core::ffi::c_int)
                    >> 4 as core::ffi::c_int - intermediate_bits) as int16_t;
                imx += dx;
                ioff += imx >> 10 as core::ffi::c_int;
                imx &= 0x3ff as core::ffi::c_int;
                x += 1;
            }
            src = src.offset(src_stride as isize);
            in_y += 1;
        }
        x = 0 as core::ffi::c_int;
        while x < w {
            *tmp.offset(x as isize) = ((16 as core::ffi::c_int
                * *mid1.offset(x as isize) as core::ffi::c_int
                + (dmy >> 6 as core::ffi::c_int)
                    * (*mid2.offset(x as isize) as core::ffi::c_int
                        - *mid1.offset(x as isize) as core::ffi::c_int)
                + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 4 as core::ffi::c_int) - PREP_BIAS)
                as int16_t;
            x += 1;
        }
        my += dy;
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn avg_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let sh: core::ffi::c_int = intermediate_bits + 1 as core::ffi::c_int;
    let rnd: core::ffi::c_int = ((1 as core::ffi::c_int) << intermediate_bits)
        + PREP_BIAS * 2 as core::ffi::c_int;
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *dst.offset(x as isize) = iclip_u8(
                *tmp1.offset(x as isize) as core::ffi::c_int
                    + *tmp2.offset(x as isize) as core::ffi::c_int + rnd >> sh,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn w_avg_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    weight: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let sh: core::ffi::c_int = intermediate_bits + 4 as core::ffi::c_int;
    let rnd: core::ffi::c_int = ((8 as core::ffi::c_int) << intermediate_bits)
        + PREP_BIAS * 16 as core::ffi::c_int;
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *dst.offset(x as isize) = iclip_u8(
                *tmp1.offset(x as isize) as core::ffi::c_int * weight
                    + *tmp2.offset(x as isize) as core::ffi::c_int
                        * (16 as core::ffi::c_int - weight) + rnd >> sh,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn mask_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut mask: *const uint8_t,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let sh: core::ffi::c_int = intermediate_bits + 6 as core::ffi::c_int;
    let rnd: core::ffi::c_int = ((32 as core::ffi::c_int) << intermediate_bits)
        + PREP_BIAS * 64 as core::ffi::c_int;
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *dst.offset(x as isize) = iclip_u8(
                *tmp1.offset(x as isize) as core::ffi::c_int
                    * *mask.offset(x as isize) as core::ffi::c_int
                    + *tmp2.offset(x as isize) as core::ffi::c_int
                        * (64 as core::ffi::c_int
                            - *mask.offset(x as isize) as core::ffi::c_int) + rnd >> sh,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        mask = mask.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn blend_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut mask: *const uint8_t,
) {
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *dst.offset(x as isize) = (*dst.offset(x as isize) as core::ffi::c_int
                * (64 as core::ffi::c_int - *mask.offset(x as isize) as core::ffi::c_int)
                + *tmp.offset(x as isize) as core::ffi::c_int
                    * *mask.offset(x as isize) as core::ffi::c_int
                + 32 as core::ffi::c_int >> 6 as core::ffi::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(dst_stride as isize);
        tmp = tmp.offset(w as isize);
        mask = mask.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn blend_v_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mask: *const uint8_t = &*dav1d_obmc_masks.as_ptr().offset(w as isize)
        as *const uint8_t;
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w * 3 as core::ffi::c_int >> 2 as core::ffi::c_int {
            *dst.offset(x as isize) = (*dst.offset(x as isize) as core::ffi::c_int
                * (64 as core::ffi::c_int - *mask.offset(x as isize) as core::ffi::c_int)
                + *tmp.offset(x as isize) as core::ffi::c_int
                    * *mask.offset(x as isize) as core::ffi::c_int
                + 32 as core::ffi::c_int >> 6 as core::ffi::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(dst_stride as isize);
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn blend_h_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut mask: *const uint8_t = &*dav1d_obmc_masks.as_ptr().offset(h as isize)
        as *const uint8_t;
    h = h * 3 as core::ffi::c_int >> 2 as core::ffi::c_int;
    loop {
        let fresh0 = mask;
        mask = mask.offset(1);
        let m: core::ffi::c_int = *fresh0 as core::ffi::c_int;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *dst.offset(x as isize) = (*dst.offset(x as isize) as core::ffi::c_int
                * (64 as core::ffi::c_int - m)
                + *tmp.offset(x as isize) as core::ffi::c_int * m
                + 32 as core::ffi::c_int >> 6 as core::ffi::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(dst_stride as isize);
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn w_mask_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut mask: *mut uint8_t,
    sign: core::ffi::c_int,
    ss_hor: core::ffi::c_int,
    ss_ver: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let bitdepth: core::ffi::c_int = 8 as core::ffi::c_int;
    let sh: core::ffi::c_int = intermediate_bits + 6 as core::ffi::c_int;
    let rnd: core::ffi::c_int = ((32 as core::ffi::c_int) << intermediate_bits)
        + PREP_BIAS * 64 as core::ffi::c_int;
    let mask_sh: core::ffi::c_int = bitdepth + intermediate_bits - 4 as core::ffi::c_int;
    let mask_rnd: core::ffi::c_int = (1 as core::ffi::c_int)
        << mask_sh - 5 as core::ffi::c_int;
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let tmpdiff: core::ffi::c_int = *tmp1.offset(x as isize) as core::ffi::c_int
                - *tmp2.offset(x as isize) as core::ffi::c_int;
            let m: core::ffi::c_int = imin(
                38 as core::ffi::c_int + (abs(tmpdiff) + mask_rnd >> mask_sh),
                64 as core::ffi::c_int,
            ) as core::ffi::c_int;
            *dst.offset(x as isize) = iclip_u8(
                tmpdiff * m
                    + *tmp2.offset(x as isize) as core::ffi::c_int
                        * 64 as core::ffi::c_int + rnd >> sh,
            ) as pixel;
            if ss_hor != 0 {
                x += 1;
                let tmpdiff_0: core::ffi::c_int = *tmp1.offset(x as isize)
                    as core::ffi::c_int - *tmp2.offset(x as isize) as core::ffi::c_int;
                let n: core::ffi::c_int = imin(
                    38 as core::ffi::c_int + (abs(tmpdiff_0) + mask_rnd >> mask_sh),
                    64 as core::ffi::c_int,
                ) as core::ffi::c_int;
                *dst.offset(x as isize) = iclip_u8(
                    tmpdiff_0 * n
                        + *tmp2.offset(x as isize) as core::ffi::c_int
                            * 64 as core::ffi::c_int + rnd >> sh,
                ) as pixel;
                if h & ss_ver != 0 {
                    *mask.offset((x >> 1 as core::ffi::c_int) as isize) = (m + n
                        + *mask.offset((x >> 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int + 2 as core::ffi::c_int - sign
                        >> 2 as core::ffi::c_int) as uint8_t;
                } else if ss_ver != 0 {
                    *mask.offset((x >> 1 as core::ffi::c_int) as isize) = (m + n)
                        as uint8_t;
                } else {
                    *mask.offset((x >> 1 as core::ffi::c_int) as isize) = (m + n
                        + 1 as core::ffi::c_int - sign >> 1 as core::ffi::c_int)
                        as uint8_t;
                }
            } else {
                *mask.offset(x as isize) = m as uint8_t;
            }
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
        if ss_ver == 0 || h & 1 as core::ffi::c_int != 0 {
            mask = mask.offset((w >> ss_hor) as isize);
        }
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn w_mask_444_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mut mask: *mut uint8_t,
    sign: core::ffi::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
unsafe extern "C" fn w_mask_422_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mut mask: *mut uint8_t,
    sign: core::ffi::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
unsafe extern "C" fn w_mask_420_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mut mask: *mut uint8_t,
    sign: core::ffi::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
}
unsafe extern "C" fn warp_affine_8x8_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mut mx: core::ffi::c_int,
    mut my: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut mid: [int16_t; 120] = [0; 120];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src = src.offset(-((3 as ptrdiff_t * src_stride) as isize));
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < 15 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut tmx: core::ffi::c_int = mx;
        while x < 8 as core::ffi::c_int {
            let filter: *const int8_t = (dav1d_mc_warp_filter[(64 as core::ffi::c_int
                + (tmx + 512 as core::ffi::c_int >> 10 as core::ffi::c_int)) as usize])
                .as_ptr();
            *mid_ptr.offset(x as isize) = (*filter.offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int
                * *src
                    .offset((x - 3 as core::ffi::c_int * 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int
                + *filter.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x - 2 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x - 1 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 0 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 1 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 2 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 3 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int - intermediate_bits
                    >> 1 as core::ffi::c_int)
                >> 7 as core::ffi::c_int - intermediate_bits) as int16_t;
            x += 1;
            tmx += *abcd.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
        }
        src = src.offset(src_stride as isize);
        mid_ptr = mid_ptr.offset(8 as core::ffi::c_int as isize);
        y += 1;
        mx += *abcd.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int;
    }
    mid_ptr = &raw mut *mid
        .as_mut_ptr()
        .offset((3 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
        as *mut int16_t;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < 8 as core::ffi::c_int {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut tmy: core::ffi::c_int = my;
        while x_0 < 8 as core::ffi::c_int {
            let filter_0: *const int8_t = (dav1d_mc_warp_filter[(64 as core::ffi::c_int
                + (tmy + 512 as core::ffi::c_int >> 10 as core::ffi::c_int)) as usize])
                .as_ptr();
            *dst.offset(x_0 as isize) = iclip_u8(
                *filter_0.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 - 3 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                    + *filter_0.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 - 2 as core::ffi::c_int * 8 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                    + *filter_0.offset(2 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 - 1 as core::ffi::c_int * 8 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                    + *filter_0.offset(3 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 0 as core::ffi::c_int * 8 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                    + *filter_0.offset(4 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 1 as core::ffi::c_int * 8 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                    + *filter_0.offset(5 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 2 as core::ffi::c_int * 8 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                    + *filter_0.offset(6 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 3 as core::ffi::c_int * 8 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                    + *filter_0.offset(7 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 4 as core::ffi::c_int * 8 as core::ffi::c_int)
                                    as isize,
                            ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int)
                        << 7 as core::ffi::c_int + intermediate_bits
                        >> 1 as core::ffi::c_int)
                    >> 7 as core::ffi::c_int + intermediate_bits,
            ) as pixel;
            x_0 += 1;
            tmy += *abcd.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int;
        }
        mid_ptr = mid_ptr.offset(8 as core::ffi::c_int as isize);
        dst = dst.offset(dst_stride as isize);
        y_0 += 1;
        my += *abcd.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int;
    }
}
unsafe extern "C" fn warp_affine_8x8t_c(
    mut tmp: *mut int16_t,
    tmp_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mut mx: core::ffi::c_int,
    mut my: core::ffi::c_int,
) {
    let intermediate_bits: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut mid: [int16_t; 120] = [0; 120];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src = src.offset(-((3 as ptrdiff_t * src_stride) as isize));
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < 15 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut tmx: core::ffi::c_int = mx;
        while x < 8 as core::ffi::c_int {
            let filter: *const int8_t = (dav1d_mc_warp_filter[(64 as core::ffi::c_int
                + (tmx + 512 as core::ffi::c_int >> 10 as core::ffi::c_int)) as usize])
                .as_ptr();
            *mid_ptr.offset(x as isize) = (*filter.offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int
                * *src
                    .offset((x - 3 as core::ffi::c_int * 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int
                + *filter.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x - 2 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x - 1 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 0 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 1 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 2 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 3 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + *filter.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            (x + 4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int - intermediate_bits
                    >> 1 as core::ffi::c_int)
                >> 7 as core::ffi::c_int - intermediate_bits) as int16_t;
            x += 1;
            tmx += *abcd.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
        }
        src = src.offset(src_stride as isize);
        mid_ptr = mid_ptr.offset(8 as core::ffi::c_int as isize);
        y += 1;
        mx += *abcd.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int;
    }
    mid_ptr = &raw mut *mid
        .as_mut_ptr()
        .offset((3 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
        as *mut int16_t;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < 8 as core::ffi::c_int {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut tmy: core::ffi::c_int = my;
        while x_0 < 8 as core::ffi::c_int {
            let filter_0: *const int8_t = (dav1d_mc_warp_filter[(64 as core::ffi::c_int
                + (tmy + 512 as core::ffi::c_int >> 10 as core::ffi::c_int)) as usize])
                .as_ptr();
            *tmp.offset(x_0 as isize) = ((*filter_0
                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                * *mid_ptr
                    .offset(
                        (x_0 - 3 as core::ffi::c_int * 8 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int
                + *filter_0.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 - 2 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                + *filter_0.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 - 1 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                + *filter_0.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 + 0 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                + *filter_0.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 + 1 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                + *filter_0.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 + 2 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                + *filter_0.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 + 3 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                + *filter_0.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 + 4 as core::ffi::c_int * 8 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int) - PREP_BIAS)
                as int16_t;
            x_0 += 1;
            tmy += *abcd.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int;
        }
        mid_ptr = mid_ptr.offset(8 as core::ffi::c_int as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_0 += 1;
        my += *abcd.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int;
    }
}
unsafe extern "C" fn emu_edge_c(
    bw: intptr_t,
    bh: intptr_t,
    iw: intptr_t,
    ih: intptr_t,
    x: intptr_t,
    y: intptr_t,
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut ref_0: *const pixel,
    ref_stride: ptrdiff_t,
) {
    ref_0 = ref_0
        .offset(
            (iclip(
                y as core::ffi::c_int,
                0 as core::ffi::c_int,
                ih as core::ffi::c_int - 1 as core::ffi::c_int,
            ) as ptrdiff_t * ref_stride
                + iclip(
                    x as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    iw as core::ffi::c_int - 1 as core::ffi::c_int,
                ) as ptrdiff_t) as isize,
        );
    let left_ext: core::ffi::c_int = iclip(
        -x as core::ffi::c_int,
        0 as core::ffi::c_int,
        bw as core::ffi::c_int - 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let right_ext: core::ffi::c_int = iclip(
        (x + bw - iw) as core::ffi::c_int,
        0 as core::ffi::c_int,
        bw as core::ffi::c_int - 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if ((left_ext + right_ext) as intptr_t) < bw {} else {
        __assert_fail(
            b"left_ext + right_ext < bw\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/mc_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            881 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 170],
                [core::ffi::c_char; 170],
            >(
                *b"void emu_edge_c(const intptr_t, const intptr_t, const intptr_t, const intptr_t, const intptr_t, const intptr_t, pixel *, const ptrdiff_t, const pixel *, const ptrdiff_t)\0",
            ))
                .as_ptr(),
        );
    };
    let top_ext: core::ffi::c_int = iclip(
        -y as core::ffi::c_int,
        0 as core::ffi::c_int,
        bh as core::ffi::c_int - 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let bottom_ext: core::ffi::c_int = iclip(
        (y + bh - ih) as core::ffi::c_int,
        0 as core::ffi::c_int,
        bh as core::ffi::c_int - 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if ((top_ext + bottom_ext) as intptr_t) < bh {} else {
        __assert_fail(
            b"top_ext + bottom_ext < bh\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/mc_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            884 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 170],
                [core::ffi::c_char; 170],
            >(
                *b"void emu_edge_c(const intptr_t, const intptr_t, const intptr_t, const intptr_t, const intptr_t, const intptr_t, pixel *, const ptrdiff_t, const pixel *, const ptrdiff_t)\0",
            ))
                .as_ptr(),
        );
    };
    let mut blk: *mut pixel = dst.offset((top_ext as ptrdiff_t * dst_stride) as isize);
    let center_w: core::ffi::c_int = (bw - left_ext as intptr_t - right_ext as intptr_t)
        as core::ffi::c_int;
    let center_h: core::ffi::c_int = (bh - top_ext as intptr_t - bottom_ext as intptr_t)
        as core::ffi::c_int;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < center_h {
        memcpy(
            blk.offset(left_ext as isize) as *mut core::ffi::c_void,
            ref_0 as *const core::ffi::c_void,
            center_w as size_t,
        );
        if left_ext != 0 {
            memset(
                blk as *mut core::ffi::c_void,
                *blk.offset(left_ext as isize) as core::ffi::c_int,
                left_ext as size_t,
            );
        }
        if right_ext != 0 {
            memset(
                blk.offset(left_ext as isize).offset(center_w as isize)
                    as *mut core::ffi::c_void,
                *blk.offset((left_ext + center_w - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                right_ext as size_t,
            );
        }
        ref_0 = ref_0.offset(ref_stride as isize);
        blk = blk.offset(dst_stride as isize);
        y_0 += 1;
    }
    blk = dst.offset((top_ext as ptrdiff_t * dst_stride) as isize);
    let mut y_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_1 < top_ext {
        memcpy(
            dst as *mut core::ffi::c_void,
            blk as *const core::ffi::c_void,
            bw as size_t,
        );
        dst = dst.offset(dst_stride as isize);
        y_1 += 1;
    }
    dst = dst.offset((center_h as ptrdiff_t * dst_stride) as isize);
    let mut y_2: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_2 < bottom_ext {
        memcpy(
            dst as *mut core::ffi::c_void,
            &raw mut *dst.offset(-dst_stride as isize) as *mut pixel
                as *const core::ffi::c_void,
            bw as size_t,
        );
        dst = dst.offset(dst_stride as isize);
        y_2 += 1;
    }
}
unsafe extern "C" fn resize_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    dst_w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    src_w: core::ffi::c_int,
    dx: core::ffi::c_int,
    mx0: core::ffi::c_int,
) {
    loop {
        let mut mx: core::ffi::c_int = mx0;
        let mut src_x: core::ffi::c_int = -(1 as core::ffi::c_int);
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < dst_w {
            let F: *const int8_t = (dav1d_resize_filter[(mx >> 8 as core::ffi::c_int)
                as usize])
                .as_ptr();
            *dst.offset(x as isize) = iclip_u8(
                -(*F.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *src
                        .offset(
                            iclip(
                                src_x - 3 as core::ffi::c_int,
                                0 as core::ffi::c_int,
                                src_w - 1 as core::ffi::c_int,
                            ) as isize,
                        ) as core::ffi::c_int
                    + *F.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x - 2 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                    src_w - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int
                    + *F.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x - 1 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                    src_w - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int
                    + *F.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 0 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                    src_w - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int
                    + *F.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 1 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                    src_w - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int
                    + *F.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 2 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                    src_w - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int
                    + *F.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 3 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                    src_w - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int
                    + *F.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 4 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                    src_w - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int) + 64 as core::ffi::c_int
                    >> 7 as core::ffi::c_int,
            ) as pixel;
            mx += dx;
            src_x += mx >> 14 as core::ffi::c_int;
            mx &= 0x3fff as core::ffi::c_int;
            x += 1;
        }
        dst = dst.offset(dst_stride as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_mc_dsp_init_8bpc(c: *mut Dav1dMCDSPContext) {
    (*c).mc[FILTER_2D_8TAP_REGULAR as core::ffi::c_int as usize] = Some(
        put_8tap_regular_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as core::ffi::c_int as usize] = Some(
        put_8tap_regular_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_REGULAR as core::ffi::c_int as usize] = Some(
        prep_8tap_regular_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as core::ffi::c_int as usize] = Some(
        prep_8tap_regular_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as core::ffi::c_int as usize] = Some(
        put_8tap_regular_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as core::ffi::c_int as usize] = Some(
        put_8tap_regular_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as core::ffi::c_int as usize] = Some(
        prep_8tap_regular_smooth_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as core::ffi::c_int as usize] = Some(
        prep_8tap_regular_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as core::ffi::c_int as usize] = Some(
        put_8tap_regular_sharp_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as core::ffi::c_int as usize] = Some(
        put_8tap_regular_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as core::ffi::c_int as usize] = Some(
        prep_8tap_regular_sharp_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as core::ffi::c_int as usize] = Some(
        prep_8tap_regular_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as core::ffi::c_int as usize] = Some(
        put_8tap_sharp_regular_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as core::ffi::c_int as usize] = Some(
        put_8tap_sharp_regular_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as core::ffi::c_int as usize] = Some(
        prep_8tap_sharp_regular_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as core::ffi::c_int as usize] = Some(
        prep_8tap_sharp_regular_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as core::ffi::c_int as usize] = Some(
        put_8tap_sharp_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as core::ffi::c_int as usize] = Some(
        put_8tap_sharp_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as core::ffi::c_int as usize] = Some(
        prep_8tap_sharp_smooth_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as core::ffi::c_int as usize] = Some(
        prep_8tap_sharp_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_SHARP as core::ffi::c_int as usize] = Some(
        put_8tap_sharp_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as core::ffi::c_int as usize] = Some(
        put_8tap_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_SHARP as core::ffi::c_int as usize] = Some(
        prep_8tap_sharp_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as core::ffi::c_int as usize] = Some(
        prep_8tap_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as core::ffi::c_int as usize] = Some(
        put_8tap_smooth_regular_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as core::ffi::c_int as usize] = Some(
        put_8tap_smooth_regular_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as core::ffi::c_int as usize] = Some(
        prep_8tap_smooth_regular_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as core::ffi::c_int as usize] = Some(
        prep_8tap_smooth_regular_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_SMOOTH as core::ffi::c_int as usize] = Some(
        put_8tap_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as core::ffi::c_int as usize] = Some(
        put_8tap_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_SMOOTH as core::ffi::c_int as usize] = Some(
        prep_8tap_smooth_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as core::ffi::c_int as usize] = Some(
        prep_8tap_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as core::ffi::c_int as usize] = Some(
        put_8tap_smooth_sharp_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as core::ffi::c_int as usize] = Some(
        put_8tap_smooth_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as core::ffi::c_int as usize] = Some(
        prep_8tap_smooth_sharp_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as core::ffi::c_int as usize] = Some(
        prep_8tap_smooth_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).mc[FILTER_2D_BILINEAR as core::ffi::c_int as usize] = Some(
        put_bilin_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_fn;
    (*c).mc_scaled[FILTER_2D_BILINEAR as core::ffi::c_int as usize] = Some(
        put_bilin_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mc_scaled_fn;
    (*c).mct[FILTER_2D_BILINEAR as core::ffi::c_int as usize] = Some(
        prep_bilin_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_fn;
    (*c).mct_scaled[FILTER_2D_BILINEAR as core::ffi::c_int as usize] = Some(
        prep_bilin_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as mct_scaled_fn;
    (*c).avg = Some(
        avg_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as avg_fn;
    (*c).w_avg = Some(
        w_avg_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as w_avg_fn;
    (*c).mask = Some(
        mask_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint8_t,
            ) -> (),
    ) as mask_fn;
    (*c).blend = Some(
        blend_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint8_t,
            ) -> (),
    ) as blend_fn;
    (*c).blend_v = Some(
        blend_v_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as blend_dir_fn;
    (*c).blend_h = Some(
        blend_h_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as blend_dir_fn;
    (*c).w_mask[0 as core::ffi::c_int as usize] = Some(
        w_mask_444_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *mut uint8_t,
                core::ffi::c_int,
            ) -> (),
    ) as w_mask_fn;
    (*c).w_mask[1 as core::ffi::c_int as usize] = Some(
        w_mask_422_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *mut uint8_t,
                core::ffi::c_int,
            ) -> (),
    ) as w_mask_fn;
    (*c).w_mask[2 as core::ffi::c_int as usize] = Some(
        w_mask_420_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *mut uint8_t,
                core::ffi::c_int,
            ) -> (),
    ) as w_mask_fn;
    (*c).warp8x8 = Some(
        warp_affine_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as warp8x8_fn;
    (*c).warp8x8t = Some(
        warp_affine_8x8t_c
            as unsafe extern "C" fn(
                *mut int16_t,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                *const int16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as warp8x8t_fn;
    (*c).emu_edge = Some(
        emu_edge_c
            as unsafe extern "C" fn(
                intptr_t,
                intptr_t,
                intptr_t,
                intptr_t,
                intptr_t,
                intptr_t,
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
            ) -> (),
    ) as emu_edge_fn;
    (*c).resize = Some(
        resize_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as resize_fn;
}
