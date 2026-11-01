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

// Mechanically translated from upstream/dav1d/src/looprestoration_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static dav1d_sgr_x_by_x: [uint8_t; 256];
}
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type ptrdiff_t = isize;
pub type pixel = uint16_t;
pub type coef = int32_t;
pub type LrEdgeFlags = core::ffi::c_uint;
pub const LR_HAVE_BOTTOM: LrEdgeFlags = 8;
pub const LR_HAVE_TOP: LrEdgeFlags = 4;
pub const LR_HAVE_RIGHT: LrEdgeFlags = 2;
pub const LR_HAVE_LEFT: LrEdgeFlags = 1;
pub type const_left_pixel_row = *const [pixel; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub union LooprestorationParams {
    pub __dav1d_align_filter: [crate::platform::Align16; 0],

    pub filter: [[int16_t; 8]; 2],
    pub sgr: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}
pub type looprestorationfilter_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        *const LooprestorationParams,
        LrEdgeFlags,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn clz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 as ptrdiff_t != 0 {
        unreachable!();
    }
    return x >> 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn imax(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn umin(
    a: core::ffi::c_uint,
    b: core::ffi::c_uint,
) -> core::ffi::c_uint {
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
pub const REST_UNIT_STRIDE: core::ffi::c_int = 390 as core::ffi::c_int;
unsafe extern "C" fn wiener_filter_h(
    mut dst: *mut uint16_t,
    mut left: *const [pixel; 4],
    mut src: *const pixel,
    mut fh: *const int16_t,
    w: core::ffi::c_int,
    edges: LrEdgeFlags,
    bitdepth_max: core::ffi::c_int,
) {
    let bitdepth: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int;
    let round_bits_h: core::ffi::c_int = 3 as core::ffi::c_int
        + (bitdepth == 12 as core::ffi::c_int) as core::ffi::c_int
            * 2 as core::ffi::c_int;
    let rounding_off_h: core::ffi::c_int = (1 as core::ffi::c_int)
        << round_bits_h - 1 as core::ffi::c_int;
    let clip_limit: core::ffi::c_int = (1 as core::ffi::c_int)
        << bitdepth + 1 as core::ffi::c_int + 7 as core::ffi::c_int - round_bits_h;
    if w < 6 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut sum: core::ffi::c_int = (1 as core::ffi::c_int)
                << bitdepth + 6 as core::ffi::c_int;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < 7 as core::ffi::c_int {
                let mut idx: core::ffi::c_int = x + i - 3 as core::ffi::c_int;
                if idx < 0 as core::ffi::c_int {
                    if edges as core::ffi::c_uint
                        & LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint == 0
                    {
                        sum
                            += *src.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                * *fh.offset(i as isize) as core::ffi::c_int;
                    } else if !left.is_null() {
                        sum
                            += (*left
                                .offset(
                                    0 as core::ffi::c_int as isize,
                                ))[(4 as core::ffi::c_int + idx) as usize]
                                as core::ffi::c_int
                                * *fh.offset(i as isize) as core::ffi::c_int;
                    } else {
                        sum
                            += *src.offset(idx as isize) as core::ffi::c_int
                                * *fh.offset(i as isize) as core::ffi::c_int;
                    }
                } else if idx >= w
                    && edges as core::ffi::c_uint
                        & LR_HAVE_RIGHT as core::ffi::c_int as core::ffi::c_uint == 0
                {
                    sum
                        += *src.offset((w - 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int
                            * *fh.offset(i as isize) as core::ffi::c_int;
                } else {
                    sum
                        += *src.offset(idx as isize) as core::ffi::c_int
                            * *fh.offset(i as isize) as core::ffi::c_int;
                }
                i += 1;
            }
            sum = iclip(
                sum + rounding_off_h >> round_bits_h,
                0 as core::ffi::c_int,
                clip_limit - 1 as core::ffi::c_int,
            );
            *dst.offset(x as isize) = sum as uint16_t;
            x += 1;
        }
        return;
    }
    let mut start: core::ffi::c_int = 3 as core::ffi::c_int;
    if edges as core::ffi::c_uint & LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint
        == 0
    {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < 3 as core::ffi::c_int {
            let mut sum_0: core::ffi::c_int = (1 as core::ffi::c_int)
                << bitdepth + 6 as core::ffi::c_int;
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < 7 as core::ffi::c_int {
                let mut idx_0: core::ffi::c_int = x_0 + i_0 - 3 as core::ffi::c_int;
                if idx_0 < 0 as core::ffi::c_int {
                    sum_0
                        += *src.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int
                            * *fh.offset(i_0 as isize) as core::ffi::c_int;
                } else {
                    sum_0
                        += *src.offset(idx_0 as isize) as core::ffi::c_int
                            * *fh.offset(i_0 as isize) as core::ffi::c_int;
                }
                i_0 += 1;
            }
            sum_0 = iclip(
                sum_0 + rounding_off_h >> round_bits_h,
                0 as core::ffi::c_int,
                clip_limit - 1 as core::ffi::c_int,
            );
            *dst.offset(x_0 as isize) = sum_0 as uint16_t;
            x_0 += 1;
        }
    } else if !left.is_null() {
        let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_1 < 3 as core::ffi::c_int {
            let mut sum_1: core::ffi::c_int = (1 as core::ffi::c_int)
                << bitdepth + 6 as core::ffi::c_int;
            let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_1 < 7 as core::ffi::c_int {
                let mut idx_1: core::ffi::c_int = x_1 + i_1 - 3 as core::ffi::c_int;
                if idx_1 < 0 as core::ffi::c_int {
                    sum_1
                        += (*left
                            .offset(
                                0 as core::ffi::c_int as isize,
                            ))[(4 as core::ffi::c_int + idx_1) as usize]
                            as core::ffi::c_int
                            * *fh.offset(i_1 as isize) as core::ffi::c_int;
                } else {
                    sum_1
                        += *src.offset(idx_1 as isize) as core::ffi::c_int
                            * *fh.offset(i_1 as isize) as core::ffi::c_int;
                }
                i_1 += 1;
            }
            sum_1 = iclip(
                sum_1 + rounding_off_h >> round_bits_h,
                0 as core::ffi::c_int,
                clip_limit - 1 as core::ffi::c_int,
            );
            *dst.offset(x_1 as isize) = sum_1 as uint16_t;
            x_1 += 1;
        }
    } else {
        start = 0 as core::ffi::c_int;
    }
    let mut end: core::ffi::c_int = w - 3 as core::ffi::c_int;
    if edges as core::ffi::c_uint
        & LR_HAVE_RIGHT as core::ffi::c_int as core::ffi::c_uint != 0
    {
        end = w;
    }
    let mut x_2: core::ffi::c_int = start;
    while x_2 < end {
        let mut sum_2: core::ffi::c_int = (1 as core::ffi::c_int)
            << bitdepth + 6 as core::ffi::c_int;
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < 7 as core::ffi::c_int {
            let mut idx_2: core::ffi::c_int = x_2 + i_2 - 3 as core::ffi::c_int;
            sum_2
                += *src.offset(idx_2 as isize) as core::ffi::c_int
                    * *fh.offset(i_2 as isize) as core::ffi::c_int;
            i_2 += 1;
        }
        sum_2 = iclip(
            sum_2 + rounding_off_h >> round_bits_h,
            0 as core::ffi::c_int,
            clip_limit - 1 as core::ffi::c_int,
        );
        *dst.offset(x_2 as isize) = sum_2 as uint16_t;
        x_2 += 1;
    }
    let mut x_3: core::ffi::c_int = end;
    while x_3 < w {
        let mut sum_3: core::ffi::c_int = (1 as core::ffi::c_int)
            << bitdepth + 6 as core::ffi::c_int;
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_3 < 7 as core::ffi::c_int {
            let mut idx_3: core::ffi::c_int = x_3 + i_3 - 3 as core::ffi::c_int;
            if idx_3 >= w {
                sum_3
                    += *src.offset((w - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int
                        * *fh.offset(i_3 as isize) as core::ffi::c_int;
            } else {
                sum_3
                    += *src.offset(idx_3 as isize) as core::ffi::c_int
                        * *fh.offset(i_3 as isize) as core::ffi::c_int;
            }
            i_3 += 1;
        }
        sum_3 = iclip(
            sum_3 + rounding_off_h >> round_bits_h,
            0 as core::ffi::c_int,
            clip_limit - 1 as core::ffi::c_int,
        );
        *dst.offset(x_3 as isize) = sum_3 as uint16_t;
        x_3 += 1;
    }
}
unsafe extern "C" fn wiener_filter_v(
    mut p: *mut pixel,
    mut ptrs: *mut *mut uint16_t,
    mut fv: *const int16_t,
    w: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let bitdepth: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int;
    let round_bits_v: core::ffi::c_int = 11 as core::ffi::c_int
        - (bitdepth == 12 as core::ffi::c_int) as core::ffi::c_int
            * 2 as core::ffi::c_int;
    let rounding_off_v: core::ffi::c_int = (1 as core::ffi::c_int)
        << round_bits_v - 1 as core::ffi::c_int;
    let round_offset: core::ffi::c_int = (1 as core::ffi::c_int)
        << bitdepth + (round_bits_v - 1 as core::ffi::c_int);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < w {
        let mut sum: core::ffi::c_int = -round_offset;
        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
        while k < 6 as core::ffi::c_int {
            sum
                += *(*ptrs.offset(k as isize)).offset(i as isize) as core::ffi::c_int
                    * *fv.offset(k as isize) as core::ffi::c_int;
            k += 1;
        }
        sum
            += *(*ptrs.offset(5 as core::ffi::c_int as isize)).offset(i as isize)
                as core::ffi::c_int
                * *fv.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int;
        *p.offset(i as isize) = iclip(
            sum + rounding_off_v >> round_bits_v,
            0 as core::ffi::c_int,
            bitdepth_max,
        ) as pixel;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 5 as core::ffi::c_int {
        let fresh8 = &raw mut *ptrs.offset(i_0 as isize);
        *fresh8 = *ptrs.offset((i_0 + 1 as core::ffi::c_int) as isize);
        i_0 += 1;
    }
}
unsafe extern "C" fn wiener_filter_hv(
    mut p: *mut pixel,
    mut ptrs: *mut *mut uint16_t,
    mut left: *const [pixel; 4],
    mut src: *const pixel,
    mut filter: *const [int16_t; 8],
    w: core::ffi::c_int,
    edges: LrEdgeFlags,
    bitdepth_max: core::ffi::c_int,
) {
    let bitdepth: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int;
    let round_bits_v: core::ffi::c_int = 11 as core::ffi::c_int
        - (bitdepth == 12 as core::ffi::c_int) as core::ffi::c_int
            * 2 as core::ffi::c_int;
    let rounding_off_v: core::ffi::c_int = (1 as core::ffi::c_int)
        << round_bits_v - 1 as core::ffi::c_int;
    let round_offset: core::ffi::c_int = (1 as core::ffi::c_int)
        << bitdepth + (round_bits_v - 1 as core::ffi::c_int);
    let mut fh: *const int16_t = (*filter.offset(0 as core::ffi::c_int as isize))
        .as_ptr();
    let mut fv: *const int16_t = (*filter.offset(1 as core::ffi::c_int as isize))
        .as_ptr();
    let mut tmp: [uint16_t; 390] = [0; 390];
    wiener_filter_h(
        tmp.as_mut_ptr(),
        left,
        src,
        fh as *const int16_t,
        w,
        edges,
        bitdepth_max,
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < w {
        let mut sum: core::ffi::c_int = -round_offset;
        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
        while k < 6 as core::ffi::c_int {
            sum
                += *(*ptrs.offset(k as isize)).offset(i as isize) as core::ffi::c_int
                    * *fv.offset(k as isize) as core::ffi::c_int;
            k += 1;
        }
        sum
            += tmp[i as usize] as core::ffi::c_int
                * *fv.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int;
        *p.offset(i as isize) = iclip(
            sum + rounding_off_v >> round_bits_v,
            0 as core::ffi::c_int,
            bitdepth_max,
        ) as pixel;
        i += 1;
    }
    memcpy(
        *ptrs.offset(6 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
        tmp.as_mut_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<uint16_t>() as size_t)
            .wrapping_mul(REST_UNIT_STRIDE as size_t),
    );
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 6 as core::ffi::c_int {
        let fresh9 = &raw mut *ptrs.offset(i_0 as isize);
        *fresh9 = *ptrs.offset((i_0 + 1 as core::ffi::c_int) as isize);
        i_0 += 1;
    }
    let fresh10 = &raw mut *ptrs.offset(6 as core::ffi::c_int as isize);
    *fresh10 = *ptrs.offset(0 as core::ffi::c_int as isize);
}
unsafe extern "C" fn wiener_c(
    mut p: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut hor: [uint16_t; 2340] = [0; 2340];
    let mut ptrs: [*mut uint16_t; 7] = [0 as *mut uint16_t; 7];
    let mut rows: [*mut uint16_t; 6] = [0 as *mut uint16_t; 6];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 6 as core::ffi::c_int {
        rows[i as usize] = &raw mut *hor.as_mut_ptr().offset((i * REST_UNIT_STRIDE) as isize)
            as *mut uint16_t;
        i += 1;
    }
    let filter: *const [int16_t; 8] = ((*params).filter).as_ptr() as *const [int16_t; 8];
    let mut fh: *const int16_t = ((*params).filter[0 as core::ffi::c_int as usize])
        .as_ptr();
    let mut fv: *const int16_t = ((*params).filter[1 as core::ffi::c_int as usize])
        .as_ptr();
    let mut lpf_bottom: *const pixel = lpf
        .offset((6 as ptrdiff_t * PXSTRIDE(stride)) as isize);
    let mut src: *const pixel = p;
    if edges as core::ffi::c_uint & LR_HAVE_TOP as core::ffi::c_int as core::ffi::c_uint
        != 0
    {
        ptrs[0 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        ptrs[1 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        ptrs[2 as core::ffi::c_int as usize] = rows[1 as core::ffi::c_int as usize];
        ptrs[3 as core::ffi::c_int as usize] = rows[2 as core::ffi::c_int as usize];
        ptrs[4 as core::ffi::c_int as usize] = rows[2 as core::ffi::c_int as usize];
        ptrs[5 as core::ffi::c_int as usize] = rows[2 as core::ffi::c_int as usize];
        wiener_filter_h(
            rows[0 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            fh as *const int16_t,
            w,
            edges,
            bitdepth_max,
        );
        lpf = lpf.offset(PXSTRIDE(stride) as isize);
        wiener_filter_h(
            rows[1 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            fh as *const int16_t,
            w,
            edges,
            bitdepth_max,
        );
        wiener_filter_h(
            rows[2 as core::ffi::c_int as usize],
            left,
            src,
            fh as *const int16_t,
            w,
            edges,
            bitdepth_max,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 0;
        } else {
            ptrs[5 as core::ffi::c_int as usize] = rows[3 as core::ffi::c_int as usize];
            ptrs[4 as core::ffi::c_int as usize] = ptrs[5 as core::ffi::c_int as usize];
            wiener_filter_h(
                rows[3 as core::ffi::c_int as usize],
                left,
                src,
                fh as *const int16_t,
                w,
                edges,
                bitdepth_max,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 1;
            } else {
                ptrs[5 as core::ffi::c_int as usize] = rows[4 as core::ffi::c_int
                    as usize];
                wiener_filter_h(
                    rows[4 as core::ffi::c_int as usize],
                    left,
                    src,
                    fh as *const int16_t,
                    w,
                    edges,
                    bitdepth_max,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                h -= 1;
                if h <= 0 as core::ffi::c_int {
                    current_block = 2;
                } else {
                    current_block = 3;
                }
            }
        }
    } else {
        ptrs[0 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        ptrs[1 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        ptrs[2 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        ptrs[3 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        ptrs[4 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        ptrs[5 as core::ffi::c_int as usize] = rows[0 as core::ffi::c_int as usize];
        wiener_filter_h(
            rows[0 as core::ffi::c_int as usize],
            left,
            src,
            fh as *const int16_t,
            w,
            edges,
            bitdepth_max,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 0;
        } else {
            ptrs[5 as core::ffi::c_int as usize] = rows[1 as core::ffi::c_int as usize];
            ptrs[4 as core::ffi::c_int as usize] = ptrs[5 as core::ffi::c_int as usize];
            wiener_filter_h(
                rows[1 as core::ffi::c_int as usize],
                left,
                src,
                fh as *const int16_t,
                w,
                edges,
                bitdepth_max,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 1;
            } else {
                ptrs[5 as core::ffi::c_int as usize] = rows[2 as core::ffi::c_int
                    as usize];
                wiener_filter_h(
                    rows[2 as core::ffi::c_int as usize],
                    left,
                    src,
                    fh as *const int16_t,
                    w,
                    edges,
                    bitdepth_max,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                h -= 1;
                if h <= 0 as core::ffi::c_int {
                    current_block = 2;
                } else {
                    ptrs[6 as core::ffi::c_int as usize] = rows[3 as core::ffi::c_int
                        as usize];
                    wiener_filter_hv(
                        p,
                        ptrs.as_mut_ptr(),
                        left,
                        src,
                        filter as *const [int16_t; 8],
                        w,
                        edges,
                        bitdepth_max,
                    );
                    left = left.offset(1);
                    src = src.offset(PXSTRIDE(stride) as isize);
                    p = p.offset(PXSTRIDE(stride) as isize);
                    h -= 1;
                    if h <= 0 as core::ffi::c_int {
                        current_block = 2;
                    } else {
                        ptrs[6 as core::ffi::c_int as usize] = rows[4 as core::ffi::c_int
                            as usize];
                        wiener_filter_hv(
                            p,
                            ptrs.as_mut_ptr(),
                            left,
                            src,
                            filter as *const [int16_t; 8],
                            w,
                            edges,
                            bitdepth_max,
                        );
                        left = left.offset(1);
                        src = src.offset(PXSTRIDE(stride) as isize);
                        p = p.offset(PXSTRIDE(stride) as isize);
                        h -= 1;
                        if h <= 0 as core::ffi::c_int {
                            current_block = 2;
                        } else {
                            current_block = 3;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        3 => {
            ptrs[6 as core::ffi::c_int as usize] = (ptrs[5 as core::ffi::c_int as usize])
                .offset(REST_UNIT_STRIDE as isize);
            loop {
                wiener_filter_hv(
                    p,
                    ptrs.as_mut_ptr(),
                    left,
                    src,
                    filter as *const [int16_t; 8],
                    w,
                    edges,
                    bitdepth_max,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                p = p.offset(PXSTRIDE(stride) as isize);
                h -= 1;
                if !(h > 0 as core::ffi::c_int) {
                    break;
                }
            }
            if edges as core::ffi::c_uint
                & LR_HAVE_BOTTOM as core::ffi::c_int as core::ffi::c_uint == 0
            {
                current_block = 2;
            } else {
                wiener_filter_hv(
                    p,
                    ptrs.as_mut_ptr(),
                    0 as *const [pixel; 4],
                    lpf_bottom,
                    filter as *const [int16_t; 8],
                    w,
                    edges,
                    bitdepth_max,
                );
                lpf_bottom = lpf_bottom.offset(PXSTRIDE(stride) as isize);
                p = p.offset(PXSTRIDE(stride) as isize);
                wiener_filter_hv(
                    p,
                    ptrs.as_mut_ptr(),
                    0 as *const [pixel; 4],
                    lpf_bottom,
                    filter as *const [int16_t; 8],
                    w,
                    edges,
                    bitdepth_max,
                );
                p = p.offset(PXSTRIDE(stride) as isize);
                current_block = 0;
            }
        }
        _ => {}
    }
    match current_block {
        2 => {
            wiener_filter_v(p, ptrs.as_mut_ptr(), fv as *const int16_t, w, bitdepth_max);
            p = p.offset(PXSTRIDE(stride) as isize);
            current_block = 1;
        }
        _ => {}
    }
    match current_block {
        1 => {
            wiener_filter_v(p, ptrs.as_mut_ptr(), fv as *const int16_t, w, bitdepth_max);
            p = p.offset(PXSTRIDE(stride) as isize);
        }
        _ => {}
    }
    wiener_filter_v(p, ptrs.as_mut_ptr(), fv as *const int16_t, w, bitdepth_max);
}
#[inline(never)]
unsafe extern "C" fn rotate(
    mut sumsq_ptrs: *mut *mut int32_t,
    mut sum_ptrs: *mut *mut coef,
    mut n: core::ffi::c_int,
) {
    let mut tmp32: *mut int32_t = *sumsq_ptrs.offset(0 as core::ffi::c_int as isize);
    let mut tmpc: *mut coef = *sum_ptrs.offset(0 as core::ffi::c_int as isize);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n - 1 as core::ffi::c_int {
        let fresh0 = &raw mut *sumsq_ptrs.offset(i as isize);
        *fresh0 = *sumsq_ptrs.offset((i + 1 as core::ffi::c_int) as isize);
        let fresh1 = &raw mut *sum_ptrs.offset(i as isize);
        *fresh1 = *sum_ptrs.offset((i + 1 as core::ffi::c_int) as isize);
        i += 1;
    }
    let fresh2 = &raw mut *sumsq_ptrs.offset((n - 1 as core::ffi::c_int) as isize);
    *fresh2 = tmp32;
    let fresh3 = &raw mut *sum_ptrs.offset((n - 1 as core::ffi::c_int) as isize);
    *fresh3 = tmpc;
}
#[inline(never)]
unsafe extern "C" fn rotate5_x2(
    mut sumsq_ptrs: *mut *mut int32_t,
    mut sum_ptrs: *mut *mut coef,
) {
    let mut tmp32: [*mut int32_t; 2] = [0 as *mut int32_t; 2];
    let mut tmpc: [*mut coef; 2] = [0 as *mut coef; 2];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        tmp32[i as usize] = *sumsq_ptrs.offset(i as isize);
        tmpc[i as usize] = *sum_ptrs.offset(i as isize);
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 3 as core::ffi::c_int {
        let fresh4 = &raw mut *sumsq_ptrs.offset(i_0 as isize);
        *fresh4 = *sumsq_ptrs.offset((i_0 + 2 as core::ffi::c_int) as isize);
        let fresh5 = &raw mut *sum_ptrs.offset(i_0 as isize);
        *fresh5 = *sum_ptrs.offset((i_0 + 2 as core::ffi::c_int) as isize);
        i_0 += 1;
    }
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < 2 as core::ffi::c_int {
        let fresh6 = &raw mut *sumsq_ptrs.offset((3 as core::ffi::c_int + i_1) as isize);
        *fresh6 = tmp32[i_1 as usize];
        let fresh7 = &raw mut *sum_ptrs.offset((3 as core::ffi::c_int + i_1) as isize);
        *fresh7 = tmpc[i_1 as usize];
        i_1 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn sgr_box3_row_h(
    mut sumsq: *mut int32_t,
    mut sum: *mut coef,
    mut left: *const [pixel; 4],
    mut src: *const pixel,
    w: core::ffi::c_int,
    edges: LrEdgeFlags,
) {
    sumsq = sumsq.offset(1);
    sum = sum.offset(1);
    let mut a: core::ffi::c_int = if edges as core::ffi::c_uint
        & LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint != 0
    {
        if !left.is_null() {
            (*left
                .offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
                as core::ffi::c_int
        } else {
            *src.offset(-(2 as core::ffi::c_int) as isize) as core::ffi::c_int
        }
    } else {
        *src.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
    };
    let mut b: core::ffi::c_int = if edges as core::ffi::c_uint
        & LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint != 0
    {
        if !left.is_null() {
            (*left
                .offset(0 as core::ffi::c_int as isize))[3 as core::ffi::c_int as usize]
                as core::ffi::c_int
        } else {
            *src.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
        }
    } else {
        *src.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
    };
    let mut x: core::ffi::c_int = -(1 as core::ffi::c_int);
    while x < w + 1 as core::ffi::c_int {
        let mut c: core::ffi::c_int = if (x + 1 as core::ffi::c_int) < w
            || edges as core::ffi::c_uint
                & LR_HAVE_RIGHT as core::ffi::c_int as core::ffi::c_uint != 0
        {
            *src.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        } else {
            *src.offset((w - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        };
        *sum.offset(x as isize) = (a + b + c) as coef;
        *sumsq.offset(x as isize) = (a * a + b * b + c * c) as int32_t;
        a = b;
        b = c;
        x += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn sgr_box5_row_h(
    mut sumsq: *mut int32_t,
    mut sum: *mut coef,
    mut left: *const [pixel; 4],
    mut src: *const pixel,
    w: core::ffi::c_int,
    edges: LrEdgeFlags,
) {
    sumsq = sumsq.offset(1);
    sum = sum.offset(1);
    let mut a: core::ffi::c_int = if edges as core::ffi::c_uint
        & LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint != 0
    {
        if !left.is_null() {
            (*left
                .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
                as core::ffi::c_int
        } else {
            *src.offset(-(3 as core::ffi::c_int) as isize) as core::ffi::c_int
        }
    } else {
        *src.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
    };
    let mut b: core::ffi::c_int = if edges as core::ffi::c_uint
        & LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint != 0
    {
        if !left.is_null() {
            (*left
                .offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
                as core::ffi::c_int
        } else {
            *src.offset(-(2 as core::ffi::c_int) as isize) as core::ffi::c_int
        }
    } else {
        *src.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
    };
    let mut c: core::ffi::c_int = if edges as core::ffi::c_uint
        & LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint != 0
    {
        if !left.is_null() {
            (*left
                .offset(0 as core::ffi::c_int as isize))[3 as core::ffi::c_int as usize]
                as core::ffi::c_int
        } else {
            *src.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
        }
    } else {
        *src.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
    };
    let mut d: core::ffi::c_int = *src.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let mut x: core::ffi::c_int = -(1 as core::ffi::c_int);
    while x < w + 1 as core::ffi::c_int {
        let mut e: core::ffi::c_int = if (x + 2 as core::ffi::c_int) < w
            || edges as core::ffi::c_uint
                & LR_HAVE_RIGHT as core::ffi::c_int as core::ffi::c_uint != 0
        {
            *src.offset((x + 2 as core::ffi::c_int) as isize) as core::ffi::c_int
        } else {
            *src.offset((w - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        };
        *sum.offset(x as isize) = (a + b + c + d + e) as coef;
        *sumsq.offset(x as isize) = (a * a + b * b + c * c + d * d + e * e) as int32_t;
        a = b;
        b = c;
        c = d;
        d = e;
        x += 1;
    }
}
unsafe extern "C" fn sgr_box35_row_h(
    mut sumsq3: *mut int32_t,
    mut sum3: *mut coef,
    mut sumsq5: *mut int32_t,
    mut sum5: *mut coef,
    mut left: *const [pixel; 4],
    mut src: *const pixel,
    w: core::ffi::c_int,
    edges: LrEdgeFlags,
) {
    sgr_box3_row_h(sumsq3, sum3, left, src, w, edges);
    sgr_box5_row_h(sumsq5, sum5, left, src, w, edges);
}
#[inline(never)]
unsafe extern "C" fn sgr_box3_row_v(
    mut sumsq: *mut *mut int32_t,
    mut sum: *mut *mut coef,
    mut sumsq_out: *mut int32_t,
    mut sum_out: *mut coef,
    w: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < w + 2 as core::ffi::c_int {
        let mut sq_a: core::ffi::c_int = *(*sumsq.offset(0 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut sq_b: core::ffi::c_int = *(*sumsq.offset(1 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut sq_c: core::ffi::c_int = *(*sumsq.offset(2 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_a: core::ffi::c_int = *(*sum.offset(0 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_b: core::ffi::c_int = *(*sum.offset(1 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_c: core::ffi::c_int = *(*sum.offset(2 as core::ffi::c_int as isize))
            .offset(x as isize);
        *sumsq_out.offset(x as isize) = (sq_a + sq_b + sq_c) as int32_t;
        *sum_out.offset(x as isize) = (s_a + s_b + s_c) as coef;
        x += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn sgr_box5_row_v(
    mut sumsq: *mut *mut int32_t,
    mut sum: *mut *mut coef,
    mut sumsq_out: *mut int32_t,
    mut sum_out: *mut coef,
    w: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < w + 2 as core::ffi::c_int {
        let mut sq_a: core::ffi::c_int = *(*sumsq.offset(0 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut sq_b: core::ffi::c_int = *(*sumsq.offset(1 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut sq_c: core::ffi::c_int = *(*sumsq.offset(2 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut sq_d: core::ffi::c_int = *(*sumsq.offset(3 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut sq_e: core::ffi::c_int = *(*sumsq.offset(4 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_a: core::ffi::c_int = *(*sum.offset(0 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_b: core::ffi::c_int = *(*sum.offset(1 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_c: core::ffi::c_int = *(*sum.offset(2 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_d: core::ffi::c_int = *(*sum.offset(3 as core::ffi::c_int as isize))
            .offset(x as isize);
        let mut s_e: core::ffi::c_int = *(*sum.offset(4 as core::ffi::c_int as isize))
            .offset(x as isize);
        *sumsq_out.offset(x as isize) = (sq_a + sq_b + sq_c + sq_d + sq_e) as int32_t;
        *sum_out.offset(x as isize) = (s_a + s_b + s_c + s_d + s_e) as coef;
        x += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn sgr_calc_row_ab(
    mut AA: *mut int32_t,
    mut BB: *mut coef,
    mut w: core::ffi::c_int,
    mut s: core::ffi::c_int,
    mut bitdepth_max: core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut sgr_one_by_x: core::ffi::c_int,
) {
    let bitdepth_min_8: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int
        - 8 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < w + 2 as core::ffi::c_int {
        let a: core::ffi::c_int = *AA.offset(i as isize)
            + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int * bitdepth_min_8
                >> 1 as core::ffi::c_int) >> 2 as core::ffi::c_int * bitdepth_min_8;
        let b: core::ffi::c_int = *BB.offset(i as isize)
            + ((1 as core::ffi::c_int) << bitdepth_min_8 >> 1 as core::ffi::c_int)
            >> bitdepth_min_8;
        let p: core::ffi::c_uint = imax(a * n - b * b, 0 as core::ffi::c_int)
            as core::ffi::c_uint;
        let z: core::ffi::c_uint = p
            .wrapping_mul(s as core::ffi::c_uint)
            .wrapping_add(
                ((1 as core::ffi::c_int) << 19 as core::ffi::c_int) as core::ffi::c_uint,
            ) >> 20 as core::ffi::c_int;
        let x: core::ffi::c_uint = dav1d_sgr_x_by_x[umin(z, 255 as core::ffi::c_uint)
            as usize] as core::ffi::c_uint;
        *AA.offset(i as isize) = (x
            .wrapping_mul(*BB.offset(i as isize) as core::ffi::c_uint)
            .wrapping_mul(sgr_one_by_x as core::ffi::c_uint)
            .wrapping_add(
                ((1 as core::ffi::c_int) << 11 as core::ffi::c_int) as core::ffi::c_uint,
            ) >> 12 as core::ffi::c_int) as int32_t;
        *BB.offset(i as isize) = x as coef;
        i += 1;
    }
}
unsafe extern "C" fn sgr_box3_vert(
    mut sumsq: *mut *mut int32_t,
    mut sum: *mut *mut coef,
    mut sumsq_out: *mut int32_t,
    mut sum_out: *mut coef,
    w: core::ffi::c_int,
    s: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    sgr_box3_row_v(sumsq, sum, sumsq_out, sum_out, w);
    sgr_calc_row_ab(
        sumsq_out,
        sum_out,
        w,
        s,
        bitdepth_max,
        9 as core::ffi::c_int,
        455 as core::ffi::c_int,
    );
    rotate(sumsq, sum, 3 as core::ffi::c_int);
}
unsafe extern "C" fn sgr_box5_vert(
    mut sumsq: *mut *mut int32_t,
    mut sum: *mut *mut coef,
    mut sumsq_out: *mut int32_t,
    mut sum_out: *mut coef,
    w: core::ffi::c_int,
    s: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    sgr_box5_row_v(sumsq, sum, sumsq_out, sum_out, w);
    sgr_calc_row_ab(
        sumsq_out,
        sum_out,
        w,
        s,
        bitdepth_max,
        25 as core::ffi::c_int,
        164 as core::ffi::c_int,
    );
    rotate5_x2(sumsq, sum);
}
unsafe extern "C" fn sgr_box3_hv(
    mut sumsq: *mut *mut int32_t,
    mut sum: *mut *mut coef,
    mut AA: *mut int32_t,
    mut BB: *mut coef,
    mut left: *const [pixel; 4],
    mut src: *const pixel,
    w: core::ffi::c_int,
    s: core::ffi::c_int,
    edges: LrEdgeFlags,
    bitdepth_max: core::ffi::c_int,
) {
    sgr_box3_row_h(
        *sumsq.offset(2 as core::ffi::c_int as isize),
        *sum.offset(2 as core::ffi::c_int as isize),
        left,
        src,
        w,
        edges,
    );
    sgr_box3_vert(sumsq, sum, AA, BB, w, s, bitdepth_max);
}
#[inline(never)]
unsafe extern "C" fn sgr_finish_filter_row1(
    mut tmp: *mut coef,
    mut src: *const pixel,
    mut A_ptrs: *mut *mut int32_t,
    mut B_ptrs: *mut *mut coef,
    w: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < w {
        let a: core::ffi::c_int = (*(*B_ptrs.offset(1 as core::ffi::c_int as isize))
            .offset((i + 1 as core::ffi::c_int) as isize)
            + *(*B_ptrs.offset(1 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            + *(*B_ptrs.offset(1 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
            + *(*B_ptrs.offset(0 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int) as isize)
            + *(*B_ptrs.offset(2 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int) as isize)) * 4 as core::ffi::c_int
            + (*(*B_ptrs.offset(0 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*B_ptrs.offset(2 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*B_ptrs.offset(0 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                + *(*B_ptrs.offset(2 as core::ffi::c_int as isize))
                    .offset(
                        (i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    )) * 3 as core::ffi::c_int;
        let b: core::ffi::c_int = (*(*A_ptrs.offset(1 as core::ffi::c_int as isize))
            .offset((i + 1 as core::ffi::c_int) as isize)
            + *(*A_ptrs.offset(1 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            + *(*A_ptrs.offset(1 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
            + *(*A_ptrs.offset(0 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int) as isize)
            + *(*A_ptrs.offset(2 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int) as isize)) * 4 as core::ffi::c_int
            + (*(*A_ptrs.offset(0 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*A_ptrs.offset(2 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*A_ptrs.offset(0 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                + *(*A_ptrs.offset(2 as core::ffi::c_int as isize))
                    .offset(
                        (i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    )) * 3 as core::ffi::c_int;
        *tmp.offset(i as isize) = (b - a * *src.offset(i as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
            >> 9 as core::ffi::c_int) as coef;
        i += 1;
    }
}
pub const FILTER_OUT_STRIDE: core::ffi::c_int = 384 as core::ffi::c_int;
#[inline(never)]
unsafe extern "C" fn sgr_finish_filter2(
    mut tmp: *mut coef,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    mut A_ptrs: *mut *mut int32_t,
    mut B_ptrs: *mut *mut coef,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < w {
        let a: core::ffi::c_int = (*(*B_ptrs.offset(0 as core::ffi::c_int as isize))
            .offset((i + 1 as core::ffi::c_int) as isize)
            + *(*B_ptrs.offset(1 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int) as isize)) * 6 as core::ffi::c_int
            + (*(*B_ptrs.offset(0 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*B_ptrs.offset(1 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*B_ptrs.offset(0 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                + *(*B_ptrs.offset(1 as core::ffi::c_int as isize))
                    .offset(
                        (i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    )) * 5 as core::ffi::c_int;
        let b: core::ffi::c_int = (*(*A_ptrs.offset(0 as core::ffi::c_int as isize))
            .offset((i + 1 as core::ffi::c_int) as isize)
            + *(*A_ptrs.offset(1 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int) as isize)) * 6 as core::ffi::c_int
            + (*(*A_ptrs.offset(0 as core::ffi::c_int as isize))
                .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*A_ptrs.offset(1 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                + *(*A_ptrs.offset(0 as core::ffi::c_int as isize))
                    .offset((i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                + *(*A_ptrs.offset(1 as core::ffi::c_int as isize))
                    .offset(
                        (i + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    )) * 5 as core::ffi::c_int;
        *tmp.offset(i as isize) = (b - a * *src.offset(i as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
            >> 9 as core::ffi::c_int) as coef;
        i += 1;
    }
    if h <= 1 as core::ffi::c_int {
        return;
    }
    tmp = tmp.offset(FILTER_OUT_STRIDE as isize);
    src = src.offset(PXSTRIDE(src_stride) as isize);
    let mut A: *const int32_t = &raw mut *(*A_ptrs.offset(1 as core::ffi::c_int as isize))
        .offset(1 as core::ffi::c_int as isize) as *mut int32_t;
    let mut B: *const coef = &raw mut *(*B_ptrs.offset(1 as core::ffi::c_int as isize))
        .offset(1 as core::ffi::c_int as isize) as *mut coef;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < w {
        let a_0: core::ffi::c_int = *B.offset(i_0 as isize) * 6 as core::ffi::c_int
            + (*B.offset((i_0 - 1 as core::ffi::c_int) as isize)
                + *B.offset((i_0 + 1 as core::ffi::c_int) as isize))
                * 5 as core::ffi::c_int;
        let b_0: core::ffi::c_int = *A.offset(i_0 as isize) * 6 as core::ffi::c_int
            + (*A.offset((i_0 - 1 as core::ffi::c_int) as isize)
                + *A.offset((i_0 + 1 as core::ffi::c_int) as isize))
                * 5 as core::ffi::c_int;
        *tmp.offset(i_0 as isize) = (b_0
            - a_0 * *src.offset(i_0 as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
            >> 8 as core::ffi::c_int) as coef;
        i_0 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn sgr_weighted_row1(
    mut dst: *mut pixel,
    mut t1: *const coef,
    w: core::ffi::c_int,
    w1: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < w {
        let v: core::ffi::c_int = w1 * *t1.offset(i as isize);
        *dst.offset(i as isize) = iclip(
            *dst.offset(i as isize) as core::ffi::c_int
                + (v + ((1 as core::ffi::c_int) << 10 as core::ffi::c_int)
                    >> 11 as core::ffi::c_int),
            0 as core::ffi::c_int,
            bitdepth_max,
        ) as pixel;
        i += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn sgr_weighted2(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut t1: *const coef,
    mut t2: *const coef,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    w0: core::ffi::c_int,
    w1: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < h {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < w {
            let v: core::ffi::c_int = w0 * *t1.offset(i as isize)
                + w1 * *t2.offset(i as isize);
            *dst.offset(i as isize) = iclip(
                *dst.offset(i as isize) as core::ffi::c_int
                    + (v + ((1 as core::ffi::c_int) << 10 as core::ffi::c_int)
                        >> 11 as core::ffi::c_int),
                0 as core::ffi::c_int,
                bitdepth_max,
            ) as pixel;
            i += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        t1 = t1.offset(FILTER_OUT_STRIDE as isize);
        t2 = t2.offset(FILTER_OUT_STRIDE as isize);
        j += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn sgr_finish1(
    mut dst: *mut *mut pixel,
    stride: ptrdiff_t,
    mut A_ptrs: *mut *mut int32_t,
    mut B_ptrs: *mut *mut coef,
    w: core::ffi::c_int,
    w1: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut tmp: crate::platform::Aligned16<[coef; 384]> = crate::platform::Aligned16([0; 384]);
    sgr_finish_filter_row1(tmp.0.as_mut_ptr(), *dst, A_ptrs, B_ptrs, w);
    sgr_weighted_row1(*dst, tmp.0.as_mut_ptr(), w, w1, bitdepth_max);
    *dst = (*dst).offset(PXSTRIDE(stride) as isize);
    rotate(A_ptrs, B_ptrs, 3 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn sgr_finish2(
    mut dst: *mut *mut pixel,
    stride: ptrdiff_t,
    mut A_ptrs: *mut *mut int32_t,
    mut B_ptrs: *mut *mut coef,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    w1: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut tmp: crate::platform::Aligned16<[coef; 768]> = crate::platform::Aligned16([0; 768]);
    sgr_finish_filter2(tmp.0.as_mut_ptr(), *dst, stride, A_ptrs, B_ptrs, w, h);
    sgr_weighted_row1(*dst, tmp.0.as_mut_ptr(), w, w1, bitdepth_max);
    *dst = (*dst).offset(PXSTRIDE(stride) as isize);
    if h > 1 as core::ffi::c_int {
        sgr_weighted_row1(
            *dst,
            tmp.0.as_mut_ptr().offset(FILTER_OUT_STRIDE as isize),
            w,
            w1,
            bitdepth_max,
        );
        *dst = (*dst).offset(PXSTRIDE(stride) as isize);
    }
    rotate(A_ptrs, B_ptrs, 2 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn sgr_finish_mix(
    mut dst: *mut *mut pixel,
    stride: ptrdiff_t,
    mut A5_ptrs: *mut *mut int32_t,
    mut B5_ptrs: *mut *mut coef,
    mut A3_ptrs: *mut *mut int32_t,
    mut B3_ptrs: *mut *mut coef,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    w0: core::ffi::c_int,
    w1: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let mut tmp5: crate::platform::Aligned16<[coef; 768]> = crate::platform::Aligned16([0; 768]);
    let mut tmp3: crate::platform::Aligned16<[coef; 768]> = crate::platform::Aligned16([0; 768]);
    sgr_finish_filter2(tmp5.0.as_mut_ptr(), *dst, stride, A5_ptrs, B5_ptrs, w, h);
    sgr_finish_filter_row1(tmp3.0.as_mut_ptr(), *dst, A3_ptrs, B3_ptrs, w);
    if h > 1 as core::ffi::c_int {
        sgr_finish_filter_row1(
            tmp3.0.as_mut_ptr().offset(FILTER_OUT_STRIDE as isize),
            (*dst).offset(PXSTRIDE(stride) as isize),
            &raw mut *A3_ptrs.offset(1 as core::ffi::c_int as isize),
            &raw mut *B3_ptrs.offset(1 as core::ffi::c_int as isize),
            w,
        );
    }
    sgr_weighted2(
        *dst,
        stride,
        tmp5.0.as_mut_ptr(),
        tmp3.0.as_mut_ptr(),
        w,
        h,
        w0,
        w1,
        bitdepth_max,
    );
    *dst = (*dst).offset((h as ptrdiff_t * PXSTRIDE(stride)) as isize);
    rotate(A5_ptrs, B5_ptrs, 2 as core::ffi::c_int);
    rotate(A3_ptrs, B3_ptrs, 4 as core::ffi::c_int);
}
unsafe extern "C" fn sgr_3x3_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut sumsq_buf: crate::platform::Aligned16<[int32_t; 1216]> = crate::platform::Aligned16([0; 1216]);
    let mut sum_buf: crate::platform::Aligned16<[coef; 1216]> = crate::platform::Aligned16([0; 1216]);
    let mut sumsq_ptrs: [*mut int32_t; 3] = [0 as *mut int32_t; 3];
    let mut sumsq_rows: [*mut int32_t; 3] = [0 as *mut int32_t; 3];
    let mut sum_ptrs: [*mut coef; 3] = [0 as *mut coef; 3];
    let mut sum_rows: [*mut coef; 3] = [0 as *mut coef; 3];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        sumsq_rows[i as usize] = &raw mut *sumsq_buf.0
            .as_mut_ptr()
            .offset((i * BUF_STRIDE) as isize) as *mut int32_t;
        sum_rows[i as usize] = &raw mut *sum_buf.0
            .as_mut_ptr()
            .offset((i * BUF_STRIDE) as isize) as *mut coef;
        i += 1;
    }
    let mut A_buf: crate::platform::Aligned16<[int32_t; 1216]> = crate::platform::Aligned16([0; 1216]);
    let mut B_buf: crate::platform::Aligned16<[coef; 1216]> = crate::platform::Aligned16([0; 1216]);
    let mut A_ptrs: [*mut int32_t; 3] = [0 as *mut int32_t; 3];
    let mut B_ptrs: [*mut coef; 3] = [0 as *mut coef; 3];
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 3 as core::ffi::c_int {
        A_ptrs[i_0 as usize] = &raw mut *A_buf.0
            .as_mut_ptr()
            .offset((i_0 * BUF_STRIDE) as isize) as *mut int32_t;
        B_ptrs[i_0 as usize] = &raw mut *B_buf.0
            .as_mut_ptr()
            .offset((i_0 * BUF_STRIDE) as isize) as *mut coef;
        i_0 += 1;
    }
    let mut src: *const pixel = dst;
    let mut lpf_bottom: *const pixel = lpf
        .offset((6 as ptrdiff_t * PXSTRIDE(stride)) as isize);
    if edges as core::ffi::c_uint & LR_HAVE_TOP as core::ffi::c_int as core::ffi::c_uint
        != 0
    {
        sumsq_ptrs[0 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[1 as core::ffi::c_int as usize] = sumsq_rows[1 as core::ffi::c_int
            as usize];
        sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_rows[2 as core::ffi::c_int
            as usize];
        sum_ptrs[0 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[1 as core::ffi::c_int as usize] = sum_rows[1 as core::ffi::c_int
            as usize];
        sum_ptrs[2 as core::ffi::c_int as usize] = sum_rows[2 as core::ffi::c_int
            as usize];
        sgr_box3_row_h(
            sumsq_rows[0 as core::ffi::c_int as usize],
            sum_rows[0 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            w,
            edges,
        );
        lpf = lpf.offset(PXSTRIDE(stride) as isize);
        sgr_box3_row_h(
            sumsq_rows[1 as core::ffi::c_int as usize],
            sum_rows[1 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            w,
            edges,
        );
        sgr_box3_hv(
            sumsq_ptrs.as_mut_ptr(),
            sum_ptrs.as_mut_ptr(),
            A_ptrs[2 as core::ffi::c_int as usize],
            B_ptrs[2 as core::ffi::c_int as usize],
            left,
            src,
            w,
            (*params).sgr.s1 as core::ffi::c_int,
            edges,
            bitdepth_max,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 3 as core::ffi::c_int);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 4;
        } else {
            sgr_box3_hv(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[2 as core::ffi::c_int as usize],
                B_ptrs[2 as core::ffi::c_int as usize],
                left,
                src,
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                edges,
                bitdepth_max,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 3 as core::ffi::c_int);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 5;
            } else {
                current_block = 6;
            }
        }
    } else {
        sumsq_ptrs[0 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[1 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[0 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[1 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[2 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sgr_box3_row_h(
            sumsq_rows[0 as core::ffi::c_int as usize],
            sum_rows[0 as core::ffi::c_int as usize],
            left,
            src,
            w,
            edges,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        sgr_box3_vert(
            sumsq_ptrs.as_mut_ptr(),
            sum_ptrs.as_mut_ptr(),
            A_ptrs[2 as core::ffi::c_int as usize],
            B_ptrs[2 as core::ffi::c_int as usize],
            w,
            (*params).sgr.s1 as core::ffi::c_int,
            bitdepth_max,
        );
        rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 3 as core::ffi::c_int);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 4;
        } else {
            sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_rows[1 as core::ffi::c_int
                as usize];
            sum_ptrs[2 as core::ffi::c_int as usize] = sum_rows[1 as core::ffi::c_int
                as usize];
            sgr_box3_hv(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[2 as core::ffi::c_int as usize],
                B_ptrs[2 as core::ffi::c_int as usize],
                left,
                src,
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                edges,
                bitdepth_max,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 3 as core::ffi::c_int);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 5;
            } else {
                sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_rows[2
                    as core::ffi::c_int as usize];
                sum_ptrs[2 as core::ffi::c_int as usize] = sum_rows[2 as core::ffi::c_int
                    as usize];
                current_block = 6;
            }
        }
    }
    match current_block {
        6 => {
            loop {
                sgr_box3_hv(
                    sumsq_ptrs.as_mut_ptr(),
                    sum_ptrs.as_mut_ptr(),
                    A_ptrs[2 as core::ffi::c_int as usize],
                    B_ptrs[2 as core::ffi::c_int as usize],
                    left,
                    src,
                    w,
                    (*params).sgr.s1 as core::ffi::c_int,
                    edges,
                    bitdepth_max,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                sgr_finish1(
                    &raw mut dst,
                    stride,
                    A_ptrs.as_mut_ptr(),
                    B_ptrs.as_mut_ptr(),
                    w,
                    (*params).sgr.w1 as core::ffi::c_int,
                    bitdepth_max,
                );
                h -= 1;
                if !(h > 0 as core::ffi::c_int) {
                    break;
                }
            }
            if edges as core::ffi::c_uint
                & LR_HAVE_BOTTOM as core::ffi::c_int as core::ffi::c_uint == 0
            {
                current_block = 5;
            } else {
                sgr_box3_hv(
                    sumsq_ptrs.as_mut_ptr(),
                    sum_ptrs.as_mut_ptr(),
                    A_ptrs[2 as core::ffi::c_int as usize],
                    B_ptrs[2 as core::ffi::c_int as usize],
                    0 as *const [pixel; 4],
                    lpf_bottom,
                    w,
                    (*params).sgr.s1 as core::ffi::c_int,
                    edges,
                    bitdepth_max,
                );
                lpf_bottom = lpf_bottom.offset(PXSTRIDE(stride) as isize);
                sgr_finish1(
                    &raw mut dst,
                    stride,
                    A_ptrs.as_mut_ptr(),
                    B_ptrs.as_mut_ptr(),
                    w,
                    (*params).sgr.w1 as core::ffi::c_int,
                    bitdepth_max,
                );
                sgr_box3_hv(
                    sumsq_ptrs.as_mut_ptr(),
                    sum_ptrs.as_mut_ptr(),
                    A_ptrs[2 as core::ffi::c_int as usize],
                    B_ptrs[2 as core::ffi::c_int as usize],
                    0 as *const [pixel; 4],
                    lpf_bottom,
                    w,
                    (*params).sgr.s1 as core::ffi::c_int,
                    edges,
                    bitdepth_max,
                );
                sgr_finish1(
                    &raw mut dst,
                    stride,
                    A_ptrs.as_mut_ptr(),
                    B_ptrs.as_mut_ptr(),
                    w,
                    (*params).sgr.w1 as core::ffi::c_int,
                    bitdepth_max,
                );
                return;
            }
        }
        4 => {
            sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_ptrs[1 as core::ffi::c_int
                as usize];
            sum_ptrs[2 as core::ffi::c_int as usize] = sum_ptrs[1 as core::ffi::c_int
                as usize];
            sgr_box3_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[2 as core::ffi::c_int as usize],
                B_ptrs[2 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 3 as core::ffi::c_int);
            current_block = 7;
        }
        _ => {}
    }
    match current_block {
        5 => {
            sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_ptrs[1 as core::ffi::c_int
                as usize];
            sum_ptrs[2 as core::ffi::c_int as usize] = sum_ptrs[1 as core::ffi::c_int
                as usize];
            sgr_box3_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[2 as core::ffi::c_int as usize],
                B_ptrs[2 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_finish1(
                &raw mut dst,
                stride,
                A_ptrs.as_mut_ptr(),
                B_ptrs.as_mut_ptr(),
                w,
                (*params).sgr.w1 as core::ffi::c_int,
                bitdepth_max,
            );
        }
        _ => {}
    }
    sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_ptrs[1 as core::ffi::c_int
        as usize];
    sum_ptrs[2 as core::ffi::c_int as usize] = sum_ptrs[1 as core::ffi::c_int as usize];
    sgr_box3_vert(
        sumsq_ptrs.as_mut_ptr(),
        sum_ptrs.as_mut_ptr(),
        A_ptrs[2 as core::ffi::c_int as usize],
        B_ptrs[2 as core::ffi::c_int as usize],
        w,
        (*params).sgr.s1 as core::ffi::c_int,
        bitdepth_max,
    );
    sgr_finish1(
        &raw mut dst,
        stride,
        A_ptrs.as_mut_ptr(),
        B_ptrs.as_mut_ptr(),
        w,
        (*params).sgr.w1 as core::ffi::c_int,
        bitdepth_max,
    );
}
pub const BUF_STRIDE: core::ffi::c_int = 384 as core::ffi::c_int
    + 16 as core::ffi::c_int;
unsafe extern "C" fn sgr_5x5_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut sumsq_buf: crate::platform::Aligned16<[int32_t; 2016]> = crate::platform::Aligned16([0; 2016]);
    let mut sum_buf: crate::platform::Aligned16<[coef; 2016]> = crate::platform::Aligned16([0; 2016]);
    let mut sumsq_ptrs: [*mut int32_t; 5] = [0 as *mut int32_t; 5];
    let mut sumsq_rows: [*mut int32_t; 5] = [0 as *mut int32_t; 5];
    let mut sum_ptrs: [*mut coef; 5] = [0 as *mut coef; 5];
    let mut sum_rows: [*mut coef; 5] = [0 as *mut coef; 5];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 5 as core::ffi::c_int {
        sumsq_rows[i as usize] = &raw mut *sumsq_buf.0
            .as_mut_ptr()
            .offset((i * BUF_STRIDE) as isize) as *mut int32_t;
        sum_rows[i as usize] = &raw mut *sum_buf.0
            .as_mut_ptr()
            .offset((i * BUF_STRIDE) as isize) as *mut coef;
        i += 1;
    }
    let mut A_buf: crate::platform::Aligned16<[int32_t; 816]> = crate::platform::Aligned16([0; 816]);
    let mut B_buf: crate::platform::Aligned16<[coef; 816]> = crate::platform::Aligned16([0; 816]);
    let mut A_ptrs: [*mut int32_t; 2] = [0 as *mut int32_t; 2];
    let mut B_ptrs: [*mut coef; 2] = [0 as *mut coef; 2];
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 2 as core::ffi::c_int {
        A_ptrs[i_0 as usize] = &raw mut *A_buf.0
            .as_mut_ptr()
            .offset((i_0 * BUF_STRIDE) as isize) as *mut int32_t;
        B_ptrs[i_0 as usize] = &raw mut *B_buf.0
            .as_mut_ptr()
            .offset((i_0 * BUF_STRIDE) as isize) as *mut coef;
        i_0 += 1;
    }
    let mut src: *const pixel = dst;
    let mut lpf_bottom: *const pixel = lpf
        .offset((6 as ptrdiff_t * PXSTRIDE(stride)) as isize);
    if edges as core::ffi::c_uint & LR_HAVE_TOP as core::ffi::c_int as core::ffi::c_uint
        != 0
    {
        sumsq_ptrs[0 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[1 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_rows[1 as core::ffi::c_int
            as usize];
        sumsq_ptrs[3 as core::ffi::c_int as usize] = sumsq_rows[2 as core::ffi::c_int
            as usize];
        sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_rows[3 as core::ffi::c_int
            as usize];
        sum_ptrs[0 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[1 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[2 as core::ffi::c_int as usize] = sum_rows[1 as core::ffi::c_int
            as usize];
        sum_ptrs[3 as core::ffi::c_int as usize] = sum_rows[2 as core::ffi::c_int
            as usize];
        sum_ptrs[4 as core::ffi::c_int as usize] = sum_rows[3 as core::ffi::c_int
            as usize];
        sgr_box5_row_h(
            sumsq_rows[0 as core::ffi::c_int as usize],
            sum_rows[0 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            w,
            edges,
        );
        lpf = lpf.offset(PXSTRIDE(stride) as isize);
        sgr_box5_row_h(
            sumsq_rows[1 as core::ffi::c_int as usize],
            sum_rows[1 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            w,
            edges,
        );
        sgr_box5_row_h(
            sumsq_rows[2 as core::ffi::c_int as usize],
            sum_rows[2 as core::ffi::c_int as usize],
            left,
            src,
            w,
            edges,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 8;
        } else {
            sgr_box5_row_h(
                sumsq_rows[3 as core::ffi::c_int as usize],
                sum_rows[3 as core::ffi::c_int as usize],
                left,
                src,
                w,
                edges,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            sgr_box5_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[1 as core::ffi::c_int as usize],
                B_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 2 as core::ffi::c_int);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 9;
            } else {
                sumsq_ptrs[3 as core::ffi::c_int as usize] = sumsq_rows[4
                    as core::ffi::c_int as usize];
                sum_ptrs[3 as core::ffi::c_int as usize] = sum_rows[4 as core::ffi::c_int
                    as usize];
                current_block = 10;
            }
        }
    } else {
        sumsq_ptrs[0 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[1 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[2 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[3 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[0 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[1 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[2 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[3 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sum_ptrs[4 as core::ffi::c_int as usize] = sum_rows[0 as core::ffi::c_int
            as usize];
        sgr_box5_row_h(
            sumsq_rows[0 as core::ffi::c_int as usize],
            sum_rows[0 as core::ffi::c_int as usize],
            left,
            src,
            w,
            edges,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 8;
        } else {
            sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_rows[1 as core::ffi::c_int
                as usize];
            sum_ptrs[4 as core::ffi::c_int as usize] = sum_rows[1 as core::ffi::c_int
                as usize];
            sgr_box5_row_h(
                sumsq_rows[1 as core::ffi::c_int as usize],
                sum_rows[1 as core::ffi::c_int as usize],
                left,
                src,
                w,
                edges,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            sgr_box5_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[1 as core::ffi::c_int as usize],
                B_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 2 as core::ffi::c_int);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 9;
            } else {
                sumsq_ptrs[3 as core::ffi::c_int as usize] = sumsq_rows[2
                    as core::ffi::c_int as usize];
                sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_rows[3
                    as core::ffi::c_int as usize];
                sum_ptrs[3 as core::ffi::c_int as usize] = sum_rows[2 as core::ffi::c_int
                    as usize];
                sum_ptrs[4 as core::ffi::c_int as usize] = sum_rows[3 as core::ffi::c_int
                    as usize];
                sgr_box5_row_h(
                    sumsq_rows[2 as core::ffi::c_int as usize],
                    sum_rows[2 as core::ffi::c_int as usize],
                    left,
                    src,
                    w,
                    edges,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                h -= 1;
                if h <= 0 as core::ffi::c_int {
                    current_block = 11;
                } else {
                    sgr_box5_row_h(
                        sumsq_rows[3 as core::ffi::c_int as usize],
                        sum_rows[3 as core::ffi::c_int as usize],
                        left,
                        src,
                        w,
                        edges,
                    );
                    left = left.offset(1);
                    src = src.offset(PXSTRIDE(stride) as isize);
                    sgr_box5_vert(
                        sumsq_ptrs.as_mut_ptr(),
                        sum_ptrs.as_mut_ptr(),
                        A_ptrs[1 as core::ffi::c_int as usize],
                        B_ptrs[1 as core::ffi::c_int as usize],
                        w,
                        (*params).sgr.s0 as core::ffi::c_int,
                        bitdepth_max,
                    );
                    sgr_finish2(
                        &raw mut dst,
                        stride,
                        A_ptrs.as_mut_ptr(),
                        B_ptrs.as_mut_ptr(),
                        w,
                        2 as core::ffi::c_int,
                        (*params).sgr.w0 as core::ffi::c_int,
                        bitdepth_max,
                    );
                    h -= 1;
                    if h <= 0 as core::ffi::c_int {
                        current_block = 9;
                    } else {
                        sumsq_ptrs[3 as core::ffi::c_int as usize] = sumsq_rows[4
                            as core::ffi::c_int as usize];
                        sum_ptrs[3 as core::ffi::c_int as usize] = sum_rows[4
                            as core::ffi::c_int as usize];
                        current_block = 10;
                    }
                }
            }
        }
    }
    match current_block {
        10 => {
            loop {
                sgr_box5_row_h(
                    sumsq_ptrs[3 as core::ffi::c_int as usize],
                    sum_ptrs[3 as core::ffi::c_int as usize],
                    left,
                    src,
                    w,
                    edges,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                h -= 1;
                if h <= 0 as core::ffi::c_int {
                    current_block = 11;
                    break;
                }
                sgr_box5_row_h(
                    sumsq_ptrs[4 as core::ffi::c_int as usize],
                    sum_ptrs[4 as core::ffi::c_int as usize],
                    left,
                    src,
                    w,
                    edges,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                sgr_box5_vert(
                    sumsq_ptrs.as_mut_ptr(),
                    sum_ptrs.as_mut_ptr(),
                    A_ptrs[1 as core::ffi::c_int as usize],
                    B_ptrs[1 as core::ffi::c_int as usize],
                    w,
                    (*params).sgr.s0 as core::ffi::c_int,
                    bitdepth_max,
                );
                sgr_finish2(
                    &raw mut dst,
                    stride,
                    A_ptrs.as_mut_ptr(),
                    B_ptrs.as_mut_ptr(),
                    w,
                    2 as core::ffi::c_int,
                    (*params).sgr.w0 as core::ffi::c_int,
                    bitdepth_max,
                );
                h -= 1;
                if !(h > 0 as core::ffi::c_int) {
                    current_block = 12;
                    break;
                }
            }
            match current_block {
                11 => {}
                _ => {
                    if edges as core::ffi::c_uint
                        & LR_HAVE_BOTTOM as core::ffi::c_int as core::ffi::c_uint == 0
                    {
                        current_block = 9;
                    } else {
                        sgr_box5_row_h(
                            sumsq_ptrs[3 as core::ffi::c_int as usize],
                            sum_ptrs[3 as core::ffi::c_int as usize],
                            0 as *const [pixel; 4],
                            lpf_bottom,
                            w,
                            edges,
                        );
                        lpf_bottom = lpf_bottom.offset(PXSTRIDE(stride) as isize);
                        sgr_box5_row_h(
                            sumsq_ptrs[4 as core::ffi::c_int as usize],
                            sum_ptrs[4 as core::ffi::c_int as usize],
                            0 as *const [pixel; 4],
                            lpf_bottom,
                            w,
                            edges,
                        );
                        current_block = 13;
                    }
                }
            }
        }
        8 => {
            sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_ptrs[3 as core::ffi::c_int
                as usize];
            sum_ptrs[4 as core::ffi::c_int as usize] = sum_ptrs[3 as core::ffi::c_int
                as usize];
            sgr_box5_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[1 as core::ffi::c_int as usize],
                B_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A_ptrs.as_mut_ptr(), B_ptrs.as_mut_ptr(), 2 as core::ffi::c_int);
            current_block = 14;
        }
        _ => {}
    }
    match current_block {
        11 => {
            sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_ptrs[3 as core::ffi::c_int
                as usize];
            sum_ptrs[4 as core::ffi::c_int as usize] = sum_ptrs[3 as core::ffi::c_int
                as usize];
            sgr_box5_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[1 as core::ffi::c_int as usize],
                B_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_finish2(
                &raw mut dst,
                stride,
                A_ptrs.as_mut_ptr(),
                B_ptrs.as_mut_ptr(),
                w,
                2 as core::ffi::c_int,
                (*params).sgr.w0 as core::ffi::c_int,
                bitdepth_max,
            );
            current_block = 14;
        }
        9 => {
            sumsq_ptrs[3 as core::ffi::c_int as usize] = sumsq_ptrs[2 as core::ffi::c_int
                as usize];
            sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_ptrs[2 as core::ffi::c_int
                as usize];
            sum_ptrs[3 as core::ffi::c_int as usize] = sum_ptrs[2 as core::ffi::c_int
                as usize];
            sum_ptrs[4 as core::ffi::c_int as usize] = sum_ptrs[2 as core::ffi::c_int
                as usize];
            current_block = 13;
        }
        _ => {}
    }
    match current_block {
        14 => {
            sumsq_ptrs[3 as core::ffi::c_int as usize] = sumsq_ptrs[2 as core::ffi::c_int
                as usize];
            sumsq_ptrs[4 as core::ffi::c_int as usize] = sumsq_ptrs[2 as core::ffi::c_int
                as usize];
            sum_ptrs[3 as core::ffi::c_int as usize] = sum_ptrs[2 as core::ffi::c_int
                as usize];
            sum_ptrs[4 as core::ffi::c_int as usize] = sum_ptrs[2 as core::ffi::c_int
                as usize];
            sgr_box5_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[1 as core::ffi::c_int as usize],
                B_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_finish2(
                &raw mut dst,
                stride,
                A_ptrs.as_mut_ptr(),
                B_ptrs.as_mut_ptr(),
                w,
                1 as core::ffi::c_int,
                (*params).sgr.w0 as core::ffi::c_int,
                bitdepth_max,
            );
            return;
        }
        _ => {
            sgr_box5_vert(
                sumsq_ptrs.as_mut_ptr(),
                sum_ptrs.as_mut_ptr(),
                A_ptrs[1 as core::ffi::c_int as usize],
                B_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_finish2(
                &raw mut dst,
                stride,
                A_ptrs.as_mut_ptr(),
                B_ptrs.as_mut_ptr(),
                w,
                2 as core::ffi::c_int,
                (*params).sgr.w0 as core::ffi::c_int,
                bitdepth_max,
            );
            return;
        }
    };
}
unsafe extern "C" fn sgr_mix_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut sumsq5_buf: crate::platform::Aligned16<[int32_t; 2016]> = crate::platform::Aligned16([0; 2016]);
    let mut sum5_buf: crate::platform::Aligned16<[coef; 2016]> = crate::platform::Aligned16([0; 2016]);
    let mut sumsq5_ptrs: [*mut int32_t; 5] = [0 as *mut int32_t; 5];
    let mut sumsq5_rows: [*mut int32_t; 5] = [0 as *mut int32_t; 5];
    let mut sum5_ptrs: [*mut coef; 5] = [0 as *mut coef; 5];
    let mut sum5_rows: [*mut coef; 5] = [0 as *mut coef; 5];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 5 as core::ffi::c_int {
        sumsq5_rows[i as usize] = &raw mut *sumsq5_buf.0
            .as_mut_ptr()
            .offset((i * BUF_STRIDE) as isize) as *mut int32_t;
        sum5_rows[i as usize] = &raw mut *sum5_buf.0
            .as_mut_ptr()
            .offset((i * BUF_STRIDE) as isize) as *mut coef;
        i += 1;
    }
    let mut sumsq3_buf: crate::platform::Aligned16<[int32_t; 1216]> = crate::platform::Aligned16([0; 1216]);
    let mut sum3_buf: crate::platform::Aligned16<[coef; 1216]> = crate::platform::Aligned16([0; 1216]);
    let mut sumsq3_ptrs: [*mut int32_t; 3] = [0 as *mut int32_t; 3];
    let mut sumsq3_rows: [*mut int32_t; 3] = [0 as *mut int32_t; 3];
    let mut sum3_ptrs: [*mut coef; 3] = [0 as *mut coef; 3];
    let mut sum3_rows: [*mut coef; 3] = [0 as *mut coef; 3];
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 3 as core::ffi::c_int {
        sumsq3_rows[i_0 as usize] = &raw mut *sumsq3_buf.0
            .as_mut_ptr()
            .offset((i_0 * BUF_STRIDE) as isize) as *mut int32_t;
        sum3_rows[i_0 as usize] = &raw mut *sum3_buf.0
            .as_mut_ptr()
            .offset((i_0 * BUF_STRIDE) as isize) as *mut coef;
        i_0 += 1;
    }
    let mut A5_buf: crate::platform::Aligned16<[int32_t; 816]> = crate::platform::Aligned16([0; 816]);
    let mut B5_buf: crate::platform::Aligned16<[coef; 816]> = crate::platform::Aligned16([0; 816]);
    let mut A5_ptrs: [*mut int32_t; 2] = [0 as *mut int32_t; 2];
    let mut B5_ptrs: [*mut coef; 2] = [0 as *mut coef; 2];
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < 2 as core::ffi::c_int {
        A5_ptrs[i_1 as usize] = &raw mut *A5_buf.0
            .as_mut_ptr()
            .offset((i_1 * BUF_STRIDE) as isize) as *mut int32_t;
        B5_ptrs[i_1 as usize] = &raw mut *B5_buf.0
            .as_mut_ptr()
            .offset((i_1 * BUF_STRIDE) as isize) as *mut coef;
        i_1 += 1;
    }
    let mut A3_buf: crate::platform::Aligned16<[int32_t; 1616]> = crate::platform::Aligned16([0; 1616]);
    let mut B3_buf: crate::platform::Aligned16<[coef; 1616]> = crate::platform::Aligned16([0; 1616]);
    let mut A3_ptrs: [*mut int32_t; 4] = [0 as *mut int32_t; 4];
    let mut B3_ptrs: [*mut coef; 4] = [0 as *mut coef; 4];
    let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_2 < 4 as core::ffi::c_int {
        A3_ptrs[i_2 as usize] = &raw mut *A3_buf.0
            .as_mut_ptr()
            .offset((i_2 * BUF_STRIDE) as isize) as *mut int32_t;
        B3_ptrs[i_2 as usize] = &raw mut *B3_buf.0
            .as_mut_ptr()
            .offset((i_2 * BUF_STRIDE) as isize) as *mut coef;
        i_2 += 1;
    }
    let mut src: *const pixel = dst;
    let mut lpf_bottom: *const pixel = lpf
        .offset((6 as ptrdiff_t * PXSTRIDE(stride)) as isize);
    if edges as core::ffi::c_uint & LR_HAVE_TOP as core::ffi::c_int as core::ffi::c_uint
        != 0
    {
        sumsq5_ptrs[0 as core::ffi::c_int as usize] = sumsq5_rows[0 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[1 as core::ffi::c_int as usize] = sumsq5_rows[0 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[2 as core::ffi::c_int as usize] = sumsq5_rows[1 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[3 as core::ffi::c_int as usize] = sumsq5_rows[2 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_rows[3 as core::ffi::c_int
            as usize];
        sum5_ptrs[0 as core::ffi::c_int as usize] = sum5_rows[0 as core::ffi::c_int
            as usize];
        sum5_ptrs[1 as core::ffi::c_int as usize] = sum5_rows[0 as core::ffi::c_int
            as usize];
        sum5_ptrs[2 as core::ffi::c_int as usize] = sum5_rows[1 as core::ffi::c_int
            as usize];
        sum5_ptrs[3 as core::ffi::c_int as usize] = sum5_rows[2 as core::ffi::c_int
            as usize];
        sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_rows[3 as core::ffi::c_int
            as usize];
        sumsq3_ptrs[0 as core::ffi::c_int as usize] = sumsq3_rows[0 as core::ffi::c_int
            as usize];
        sumsq3_ptrs[1 as core::ffi::c_int as usize] = sumsq3_rows[1 as core::ffi::c_int
            as usize];
        sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_rows[2 as core::ffi::c_int
            as usize];
        sum3_ptrs[0 as core::ffi::c_int as usize] = sum3_rows[0 as core::ffi::c_int
            as usize];
        sum3_ptrs[1 as core::ffi::c_int as usize] = sum3_rows[1 as core::ffi::c_int
            as usize];
        sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_rows[2 as core::ffi::c_int
            as usize];
        sgr_box35_row_h(
            sumsq3_rows[0 as core::ffi::c_int as usize],
            sum3_rows[0 as core::ffi::c_int as usize],
            sumsq5_rows[0 as core::ffi::c_int as usize],
            sum5_rows[0 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            w,
            edges,
        );
        lpf = lpf.offset(PXSTRIDE(stride) as isize);
        sgr_box35_row_h(
            sumsq3_rows[1 as core::ffi::c_int as usize],
            sum3_rows[1 as core::ffi::c_int as usize],
            sumsq5_rows[1 as core::ffi::c_int as usize],
            sum5_rows[1 as core::ffi::c_int as usize],
            0 as *const [pixel; 4],
            lpf,
            w,
            edges,
        );
        sgr_box35_row_h(
            sumsq3_rows[2 as core::ffi::c_int as usize],
            sum3_rows[2 as core::ffi::c_int as usize],
            sumsq5_rows[2 as core::ffi::c_int as usize],
            sum5_rows[2 as core::ffi::c_int as usize],
            left,
            src,
            w,
            edges,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        sgr_box3_vert(
            sumsq3_ptrs.as_mut_ptr(),
            sum3_ptrs.as_mut_ptr(),
            A3_ptrs[3 as core::ffi::c_int as usize],
            B3_ptrs[3 as core::ffi::c_int as usize],
            w,
            (*params).sgr.s1 as core::ffi::c_int,
            bitdepth_max,
        );
        rotate(A3_ptrs.as_mut_ptr(), B3_ptrs.as_mut_ptr(), 4 as core::ffi::c_int);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 15;
        } else {
            sgr_box35_row_h(
                sumsq3_ptrs[2 as core::ffi::c_int as usize],
                sum3_ptrs[2 as core::ffi::c_int as usize],
                sumsq5_rows[3 as core::ffi::c_int as usize],
                sum5_rows[3 as core::ffi::c_int as usize],
                left,
                src,
                w,
                edges,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            sgr_box5_vert(
                sumsq5_ptrs.as_mut_ptr(),
                sum5_ptrs.as_mut_ptr(),
                A5_ptrs[1 as core::ffi::c_int as usize],
                B5_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A5_ptrs.as_mut_ptr(), B5_ptrs.as_mut_ptr(), 2 as core::ffi::c_int);
            sgr_box3_vert(
                sumsq3_ptrs.as_mut_ptr(),
                sum3_ptrs.as_mut_ptr(),
                A3_ptrs[3 as core::ffi::c_int as usize],
                B3_ptrs[3 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A3_ptrs.as_mut_ptr(), B3_ptrs.as_mut_ptr(), 4 as core::ffi::c_int);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 16;
            } else {
                sumsq5_ptrs[3 as core::ffi::c_int as usize] = sumsq5_rows[4
                    as core::ffi::c_int as usize];
                sum5_ptrs[3 as core::ffi::c_int as usize] = sum5_rows[4
                    as core::ffi::c_int as usize];
                current_block = 17;
            }
        }
    } else {
        sumsq5_ptrs[0 as core::ffi::c_int as usize] = sumsq5_rows[0 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[1 as core::ffi::c_int as usize] = sumsq5_rows[0 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[2 as core::ffi::c_int as usize] = sumsq5_rows[0 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[3 as core::ffi::c_int as usize] = sumsq5_rows[0 as core::ffi::c_int
            as usize];
        sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_rows[0 as core::ffi::c_int
            as usize];
        sum5_ptrs[0 as core::ffi::c_int as usize] = sum5_rows[0 as core::ffi::c_int
            as usize];
        sum5_ptrs[1 as core::ffi::c_int as usize] = sum5_rows[0 as core::ffi::c_int
            as usize];
        sum5_ptrs[2 as core::ffi::c_int as usize] = sum5_rows[0 as core::ffi::c_int
            as usize];
        sum5_ptrs[3 as core::ffi::c_int as usize] = sum5_rows[0 as core::ffi::c_int
            as usize];
        sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_rows[0 as core::ffi::c_int
            as usize];
        sumsq3_ptrs[0 as core::ffi::c_int as usize] = sumsq3_rows[0 as core::ffi::c_int
            as usize];
        sumsq3_ptrs[1 as core::ffi::c_int as usize] = sumsq3_rows[0 as core::ffi::c_int
            as usize];
        sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_rows[0 as core::ffi::c_int
            as usize];
        sum3_ptrs[0 as core::ffi::c_int as usize] = sum3_rows[0 as core::ffi::c_int
            as usize];
        sum3_ptrs[1 as core::ffi::c_int as usize] = sum3_rows[0 as core::ffi::c_int
            as usize];
        sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_rows[0 as core::ffi::c_int
            as usize];
        sgr_box35_row_h(
            sumsq3_rows[0 as core::ffi::c_int as usize],
            sum3_rows[0 as core::ffi::c_int as usize],
            sumsq5_rows[0 as core::ffi::c_int as usize],
            sum5_rows[0 as core::ffi::c_int as usize],
            left,
            src,
            w,
            edges,
        );
        left = left.offset(1);
        src = src.offset(PXSTRIDE(stride) as isize);
        sgr_box3_vert(
            sumsq3_ptrs.as_mut_ptr(),
            sum3_ptrs.as_mut_ptr(),
            A3_ptrs[3 as core::ffi::c_int as usize],
            B3_ptrs[3 as core::ffi::c_int as usize],
            w,
            (*params).sgr.s1 as core::ffi::c_int,
            bitdepth_max,
        );
        rotate(A3_ptrs.as_mut_ptr(), B3_ptrs.as_mut_ptr(), 4 as core::ffi::c_int);
        h -= 1;
        if h <= 0 as core::ffi::c_int {
            current_block = 15;
        } else {
            sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_rows[1
                as core::ffi::c_int as usize];
            sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_rows[1 as core::ffi::c_int
                as usize];
            sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_rows[1
                as core::ffi::c_int as usize];
            sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_rows[1 as core::ffi::c_int
                as usize];
            sgr_box35_row_h(
                sumsq3_rows[1 as core::ffi::c_int as usize],
                sum3_rows[1 as core::ffi::c_int as usize],
                sumsq5_rows[1 as core::ffi::c_int as usize],
                sum5_rows[1 as core::ffi::c_int as usize],
                left,
                src,
                w,
                edges,
            );
            left = left.offset(1);
            src = src.offset(PXSTRIDE(stride) as isize);
            sgr_box5_vert(
                sumsq5_ptrs.as_mut_ptr(),
                sum5_ptrs.as_mut_ptr(),
                A5_ptrs[1 as core::ffi::c_int as usize],
                B5_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A5_ptrs.as_mut_ptr(), B5_ptrs.as_mut_ptr(), 2 as core::ffi::c_int);
            sgr_box3_vert(
                sumsq3_ptrs.as_mut_ptr(),
                sum3_ptrs.as_mut_ptr(),
                A3_ptrs[3 as core::ffi::c_int as usize],
                B3_ptrs[3 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A3_ptrs.as_mut_ptr(), B3_ptrs.as_mut_ptr(), 4 as core::ffi::c_int);
            h -= 1;
            if h <= 0 as core::ffi::c_int {
                current_block = 16;
            } else {
                sumsq5_ptrs[3 as core::ffi::c_int as usize] = sumsq5_rows[2
                    as core::ffi::c_int as usize];
                sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_rows[3
                    as core::ffi::c_int as usize];
                sum5_ptrs[3 as core::ffi::c_int as usize] = sum5_rows[2
                    as core::ffi::c_int as usize];
                sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_rows[3
                    as core::ffi::c_int as usize];
                sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_rows[2
                    as core::ffi::c_int as usize];
                sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_rows[2
                    as core::ffi::c_int as usize];
                sgr_box35_row_h(
                    sumsq3_rows[2 as core::ffi::c_int as usize],
                    sum3_rows[2 as core::ffi::c_int as usize],
                    sumsq5_rows[2 as core::ffi::c_int as usize],
                    sum5_rows[2 as core::ffi::c_int as usize],
                    left,
                    src,
                    w,
                    edges,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                sgr_box3_vert(
                    sumsq3_ptrs.as_mut_ptr(),
                    sum3_ptrs.as_mut_ptr(),
                    A3_ptrs[3 as core::ffi::c_int as usize],
                    B3_ptrs[3 as core::ffi::c_int as usize],
                    w,
                    (*params).sgr.s1 as core::ffi::c_int,
                    bitdepth_max,
                );
                rotate(
                    A3_ptrs.as_mut_ptr(),
                    B3_ptrs.as_mut_ptr(),
                    4 as core::ffi::c_int,
                );
                h -= 1;
                if h <= 0 as core::ffi::c_int {
                    current_block = 18;
                } else {
                    sgr_box35_row_h(
                        sumsq3_ptrs[2 as core::ffi::c_int as usize],
                        sum3_ptrs[2 as core::ffi::c_int as usize],
                        sumsq5_rows[3 as core::ffi::c_int as usize],
                        sum5_rows[3 as core::ffi::c_int as usize],
                        left,
                        src,
                        w,
                        edges,
                    );
                    left = left.offset(1);
                    src = src.offset(PXSTRIDE(stride) as isize);
                    sgr_box5_vert(
                        sumsq5_ptrs.as_mut_ptr(),
                        sum5_ptrs.as_mut_ptr(),
                        A5_ptrs[1 as core::ffi::c_int as usize],
                        B5_ptrs[1 as core::ffi::c_int as usize],
                        w,
                        (*params).sgr.s0 as core::ffi::c_int,
                        bitdepth_max,
                    );
                    sgr_box3_vert(
                        sumsq3_ptrs.as_mut_ptr(),
                        sum3_ptrs.as_mut_ptr(),
                        A3_ptrs[3 as core::ffi::c_int as usize],
                        B3_ptrs[3 as core::ffi::c_int as usize],
                        w,
                        (*params).sgr.s1 as core::ffi::c_int,
                        bitdepth_max,
                    );
                    sgr_finish_mix(
                        &raw mut dst,
                        stride,
                        A5_ptrs.as_mut_ptr(),
                        B5_ptrs.as_mut_ptr(),
                        A3_ptrs.as_mut_ptr(),
                        B3_ptrs.as_mut_ptr(),
                        w,
                        2 as core::ffi::c_int,
                        (*params).sgr.w0 as core::ffi::c_int,
                        (*params).sgr.w1 as core::ffi::c_int,
                        bitdepth_max,
                    );
                    h -= 1;
                    if h <= 0 as core::ffi::c_int {
                        current_block = 16;
                    } else {
                        sumsq5_ptrs[3 as core::ffi::c_int as usize] = sumsq5_rows[4
                            as core::ffi::c_int as usize];
                        sum5_ptrs[3 as core::ffi::c_int as usize] = sum5_rows[4
                            as core::ffi::c_int as usize];
                        current_block = 17;
                    }
                }
            }
        }
    }
    match current_block {
        17 => {
            loop {
                sgr_box35_row_h(
                    sumsq3_ptrs[2 as core::ffi::c_int as usize],
                    sum3_ptrs[2 as core::ffi::c_int as usize],
                    sumsq5_ptrs[3 as core::ffi::c_int as usize],
                    sum5_ptrs[3 as core::ffi::c_int as usize],
                    left,
                    src,
                    w,
                    edges,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                sgr_box3_vert(
                    sumsq3_ptrs.as_mut_ptr(),
                    sum3_ptrs.as_mut_ptr(),
                    A3_ptrs[3 as core::ffi::c_int as usize],
                    B3_ptrs[3 as core::ffi::c_int as usize],
                    w,
                    (*params).sgr.s1 as core::ffi::c_int,
                    bitdepth_max,
                );
                rotate(
                    A3_ptrs.as_mut_ptr(),
                    B3_ptrs.as_mut_ptr(),
                    4 as core::ffi::c_int,
                );
                h -= 1;
                if h <= 0 as core::ffi::c_int {
                    current_block = 18;
                    break;
                }
                sgr_box35_row_h(
                    sumsq3_ptrs[2 as core::ffi::c_int as usize],
                    sum3_ptrs[2 as core::ffi::c_int as usize],
                    sumsq5_ptrs[4 as core::ffi::c_int as usize],
                    sum5_ptrs[4 as core::ffi::c_int as usize],
                    left,
                    src,
                    w,
                    edges,
                );
                left = left.offset(1);
                src = src.offset(PXSTRIDE(stride) as isize);
                sgr_box5_vert(
                    sumsq5_ptrs.as_mut_ptr(),
                    sum5_ptrs.as_mut_ptr(),
                    A5_ptrs[1 as core::ffi::c_int as usize],
                    B5_ptrs[1 as core::ffi::c_int as usize],
                    w,
                    (*params).sgr.s0 as core::ffi::c_int,
                    bitdepth_max,
                );
                sgr_box3_vert(
                    sumsq3_ptrs.as_mut_ptr(),
                    sum3_ptrs.as_mut_ptr(),
                    A3_ptrs[3 as core::ffi::c_int as usize],
                    B3_ptrs[3 as core::ffi::c_int as usize],
                    w,
                    (*params).sgr.s1 as core::ffi::c_int,
                    bitdepth_max,
                );
                sgr_finish_mix(
                    &raw mut dst,
                    stride,
                    A5_ptrs.as_mut_ptr(),
                    B5_ptrs.as_mut_ptr(),
                    A3_ptrs.as_mut_ptr(),
                    B3_ptrs.as_mut_ptr(),
                    w,
                    2 as core::ffi::c_int,
                    (*params).sgr.w0 as core::ffi::c_int,
                    (*params).sgr.w1 as core::ffi::c_int,
                    bitdepth_max,
                );
                h -= 1;
                if !(h > 0 as core::ffi::c_int) {
                    current_block = 19;
                    break;
                }
            }
            match current_block {
                18 => {}
                _ => {
                    if edges as core::ffi::c_uint
                        & LR_HAVE_BOTTOM as core::ffi::c_int as core::ffi::c_uint == 0
                    {
                        current_block = 16;
                    } else {
                        sgr_box35_row_h(
                            sumsq3_ptrs[2 as core::ffi::c_int as usize],
                            sum3_ptrs[2 as core::ffi::c_int as usize],
                            sumsq5_ptrs[3 as core::ffi::c_int as usize],
                            sum5_ptrs[3 as core::ffi::c_int as usize],
                            0 as *const [pixel; 4],
                            lpf_bottom,
                            w,
                            edges,
                        );
                        lpf_bottom = lpf_bottom.offset(PXSTRIDE(stride) as isize);
                        sgr_box3_vert(
                            sumsq3_ptrs.as_mut_ptr(),
                            sum3_ptrs.as_mut_ptr(),
                            A3_ptrs[3 as core::ffi::c_int as usize],
                            B3_ptrs[3 as core::ffi::c_int as usize],
                            w,
                            (*params).sgr.s1 as core::ffi::c_int,
                            bitdepth_max,
                        );
                        rotate(
                            A3_ptrs.as_mut_ptr(),
                            B3_ptrs.as_mut_ptr(),
                            4 as core::ffi::c_int,
                        );
                        sgr_box35_row_h(
                            sumsq3_ptrs[2 as core::ffi::c_int as usize],
                            sum3_ptrs[2 as core::ffi::c_int as usize],
                            sumsq5_ptrs[4 as core::ffi::c_int as usize],
                            sum5_ptrs[4 as core::ffi::c_int as usize],
                            0 as *const [pixel; 4],
                            lpf_bottom,
                            w,
                            edges,
                        );
                        current_block = 20;
                    }
                }
            }
        }
        15 => {
            sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_ptrs[3
                as core::ffi::c_int as usize];
            sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_ptrs[3 as core::ffi::c_int
                as usize];
            sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_ptrs[1
                as core::ffi::c_int as usize];
            sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_ptrs[1 as core::ffi::c_int
                as usize];
            sgr_box5_vert(
                sumsq5_ptrs.as_mut_ptr(),
                sum5_ptrs.as_mut_ptr(),
                A5_ptrs[1 as core::ffi::c_int as usize],
                B5_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A5_ptrs.as_mut_ptr(), B5_ptrs.as_mut_ptr(), 2 as core::ffi::c_int);
            sgr_box3_vert(
                sumsq3_ptrs.as_mut_ptr(),
                sum3_ptrs.as_mut_ptr(),
                A3_ptrs[3 as core::ffi::c_int as usize],
                B3_ptrs[3 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A3_ptrs.as_mut_ptr(), B3_ptrs.as_mut_ptr(), 4 as core::ffi::c_int);
            current_block = 21;
        }
        _ => {}
    }
    match current_block {
        18 => {
            sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_ptrs[3
                as core::ffi::c_int as usize];
            sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_ptrs[3 as core::ffi::c_int
                as usize];
            sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_ptrs[1
                as core::ffi::c_int as usize];
            sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_ptrs[1 as core::ffi::c_int
                as usize];
            sgr_box5_vert(
                sumsq5_ptrs.as_mut_ptr(),
                sum5_ptrs.as_mut_ptr(),
                A5_ptrs[1 as core::ffi::c_int as usize],
                B5_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_box3_vert(
                sumsq3_ptrs.as_mut_ptr(),
                sum3_ptrs.as_mut_ptr(),
                A3_ptrs[3 as core::ffi::c_int as usize],
                B3_ptrs[3 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_finish_mix(
                &raw mut dst,
                stride,
                A5_ptrs.as_mut_ptr(),
                B5_ptrs.as_mut_ptr(),
                A3_ptrs.as_mut_ptr(),
                B3_ptrs.as_mut_ptr(),
                w,
                2 as core::ffi::c_int,
                (*params).sgr.w0 as core::ffi::c_int,
                (*params).sgr.w1 as core::ffi::c_int,
                bitdepth_max,
            );
            current_block = 21;
        }
        16 => {
            sumsq5_ptrs[3 as core::ffi::c_int as usize] = sumsq5_ptrs[2
                as core::ffi::c_int as usize];
            sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_ptrs[2
                as core::ffi::c_int as usize];
            sum5_ptrs[3 as core::ffi::c_int as usize] = sum5_ptrs[2 as core::ffi::c_int
                as usize];
            sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_ptrs[2 as core::ffi::c_int
                as usize];
            sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_ptrs[1
                as core::ffi::c_int as usize];
            sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_ptrs[1 as core::ffi::c_int
                as usize];
            sgr_box3_vert(
                sumsq3_ptrs.as_mut_ptr(),
                sum3_ptrs.as_mut_ptr(),
                A3_ptrs[3 as core::ffi::c_int as usize],
                B3_ptrs[3 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A3_ptrs.as_mut_ptr(), B3_ptrs.as_mut_ptr(), 4 as core::ffi::c_int);
            sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_ptrs[1
                as core::ffi::c_int as usize];
            sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_ptrs[1 as core::ffi::c_int
                as usize];
            current_block = 20;
        }
        _ => {}
    }
    match current_block {
        21 => {
            sumsq5_ptrs[3 as core::ffi::c_int as usize] = sumsq5_ptrs[2
                as core::ffi::c_int as usize];
            sumsq5_ptrs[4 as core::ffi::c_int as usize] = sumsq5_ptrs[2
                as core::ffi::c_int as usize];
            sum5_ptrs[3 as core::ffi::c_int as usize] = sum5_ptrs[2 as core::ffi::c_int
                as usize];
            sum5_ptrs[4 as core::ffi::c_int as usize] = sum5_ptrs[2 as core::ffi::c_int
                as usize];
            sumsq3_ptrs[2 as core::ffi::c_int as usize] = sumsq3_ptrs[1
                as core::ffi::c_int as usize];
            sum3_ptrs[2 as core::ffi::c_int as usize] = sum3_ptrs[1 as core::ffi::c_int
                as usize];
            sgr_box5_vert(
                sumsq5_ptrs.as_mut_ptr(),
                sum5_ptrs.as_mut_ptr(),
                A5_ptrs[1 as core::ffi::c_int as usize],
                B5_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_box3_vert(
                sumsq3_ptrs.as_mut_ptr(),
                sum3_ptrs.as_mut_ptr(),
                A3_ptrs[3 as core::ffi::c_int as usize],
                B3_ptrs[3 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            rotate(A3_ptrs.as_mut_ptr(), B3_ptrs.as_mut_ptr(), 4 as core::ffi::c_int);
            sgr_finish_mix(
                &raw mut dst,
                stride,
                A5_ptrs.as_mut_ptr(),
                B5_ptrs.as_mut_ptr(),
                A3_ptrs.as_mut_ptr(),
                B3_ptrs.as_mut_ptr(),
                w,
                1 as core::ffi::c_int,
                (*params).sgr.w0 as core::ffi::c_int,
                (*params).sgr.w1 as core::ffi::c_int,
                bitdepth_max,
            );
            return;
        }
        _ => {
            sgr_box5_vert(
                sumsq5_ptrs.as_mut_ptr(),
                sum5_ptrs.as_mut_ptr(),
                A5_ptrs[1 as core::ffi::c_int as usize],
                B5_ptrs[1 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s0 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_box3_vert(
                sumsq3_ptrs.as_mut_ptr(),
                sum3_ptrs.as_mut_ptr(),
                A3_ptrs[3 as core::ffi::c_int as usize],
                B3_ptrs[3 as core::ffi::c_int as usize],
                w,
                (*params).sgr.s1 as core::ffi::c_int,
                bitdepth_max,
            );
            sgr_finish_mix(
                &raw mut dst,
                stride,
                A5_ptrs.as_mut_ptr(),
                B5_ptrs.as_mut_ptr(),
                A3_ptrs.as_mut_ptr(),
                B3_ptrs.as_mut_ptr(),
                w,
                2 as core::ffi::c_int,
                (*params).sgr.w0 as core::ffi::c_int,
                (*params).sgr.w1 as core::ffi::c_int,
                bitdepth_max,
            );
            return;
        }
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_restoration_dsp_init_16bpc(
    c: *mut Dav1dLoopRestorationDSPContext,
    bpc: core::ffi::c_int,
) {
    (*c).wiener[1 as core::ffi::c_int as usize] = Some(
        wiener_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                core::ffi::c_int,
            ) -> (),
    ) as looprestorationfilter_fn;
    (*c).wiener[0 as core::ffi::c_int as usize] = (*c)
        .wiener[1 as core::ffi::c_int as usize];
    (*c).sgr[0 as core::ffi::c_int as usize] = Some(
        sgr_5x5_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                core::ffi::c_int,
            ) -> (),
    ) as looprestorationfilter_fn;
    (*c).sgr[1 as core::ffi::c_int as usize] = Some(
        sgr_3x3_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                core::ffi::c_int,
            ) -> (),
    ) as looprestorationfilter_fn;
    (*c).sgr[2 as core::ffi::c_int as usize] = Some(
        sgr_mix_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                core::ffi::c_int,
            ) -> (),
    ) as looprestorationfilter_fn;
}
