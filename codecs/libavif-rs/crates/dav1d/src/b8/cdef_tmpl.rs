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

// Mechanically translated from upstream/dav1d/src/cdef_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    static dav1d_cdef_directions: [[int8_t; 2]; 12];
}
pub type int8_t = i8;
pub type int16_t = i16;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type ptrdiff_t = isize;
pub type pixel = uint8_t;
pub type CdefEdgeFlags = core::ffi::c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row_2px,
        *const pixel,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        CdefEdgeFlags,
    ) -> (),
>;
pub type cdef_dir_fn = Option<
    unsafe extern "C" fn(
        *const pixel,
        ptrdiff_t,
        *mut core::ffi::c_uint,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub const INT16_MIN: core::ffi::c_int = -(32768 as core::ffi::c_int);
#[inline]
unsafe extern "C" fn clz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
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
#[inline]
unsafe extern "C" fn apply_sign(
    v: core::ffi::c_int,
    s: core::ffi::c_int,
) -> core::ffi::c_int {
    return if s < 0 as core::ffi::c_int { -v } else { v };
}
#[inline]
unsafe extern "C" fn ulog2(v: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ clz(v);
}
#[inline]
unsafe extern "C" fn constrain(
    diff: core::ffi::c_int,
    threshold: core::ffi::c_int,
    shift: core::ffi::c_int,
) -> core::ffi::c_int {
    let adiff: core::ffi::c_int = abs(diff) as core::ffi::c_int;
    return apply_sign(
        imin(adiff, imax(0 as core::ffi::c_int, threshold - (adiff >> shift))),
        diff,
    );
}
#[inline]
unsafe extern "C" fn fill(
    mut tmp: *mut int16_t,
    stride: ptrdiff_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *tmp.offset(x as isize) = INT16_MIN as int16_t;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn padding(
    mut tmp: *mut int16_t,
    tmp_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    mut top: *const pixel,
    mut bottom: *const pixel,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    edges: CdefEdgeFlags,
) {
    let mut x_start: core::ffi::c_int = -(2 as core::ffi::c_int);
    let mut x_end: core::ffi::c_int = w + 2 as core::ffi::c_int;
    let mut y_start: core::ffi::c_int = -(2 as core::ffi::c_int);
    let mut y_end: core::ffi::c_int = h + 2 as core::ffi::c_int;
    if edges as core::ffi::c_uint
        & CDEF_HAVE_TOP as core::ffi::c_int as core::ffi::c_uint == 0
    {
        fill(
            tmp
                .offset(-(2 as core::ffi::c_int as isize))
                .offset(-((2 as ptrdiff_t * tmp_stride) as isize)),
            tmp_stride,
            w + 4 as core::ffi::c_int,
            2 as core::ffi::c_int,
        );
        y_start = 0 as core::ffi::c_int;
    }
    if edges as core::ffi::c_uint
        & CDEF_HAVE_BOTTOM as core::ffi::c_int as core::ffi::c_uint == 0
    {
        fill(
            tmp
                .offset((h as ptrdiff_t * tmp_stride) as isize)
                .offset(-(2 as core::ffi::c_int as isize)),
            tmp_stride,
            w + 4 as core::ffi::c_int,
            2 as core::ffi::c_int,
        );
        y_end -= 2 as core::ffi::c_int;
    }
    if edges as core::ffi::c_uint
        & CDEF_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint == 0
    {
        fill(
            tmp
                .offset((y_start as ptrdiff_t * tmp_stride) as isize)
                .offset(-(2 as core::ffi::c_int as isize)),
            tmp_stride,
            2 as core::ffi::c_int,
            y_end - y_start,
        );
        x_start = 0 as core::ffi::c_int;
    }
    if edges as core::ffi::c_uint
        & CDEF_HAVE_RIGHT as core::ffi::c_int as core::ffi::c_uint == 0
    {
        fill(
            tmp.offset((y_start as ptrdiff_t * tmp_stride) as isize).offset(w as isize),
            tmp_stride,
            2 as core::ffi::c_int,
            y_end - y_start,
        );
        x_end -= 2 as core::ffi::c_int;
    }
    let mut y: core::ffi::c_int = y_start;
    while y < 0 as core::ffi::c_int {
        let mut x: core::ffi::c_int = x_start;
        while x < x_end {
            *tmp.offset((x as ptrdiff_t + y as ptrdiff_t * tmp_stride) as isize) = *top
                .offset(x as isize) as int16_t;
            x += 1;
        }
        top = top.offset(src_stride as isize);
        y += 1;
    }
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < h {
        let mut x_0: core::ffi::c_int = x_start;
        while x_0 < 0 as core::ffi::c_int {
            *tmp.offset((x_0 as ptrdiff_t + y_0 as ptrdiff_t * tmp_stride) as isize) = (*left
                .offset(y_0 as isize))[(2 as core::ffi::c_int + x_0) as usize]
                as int16_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
    let mut y_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_1 < h {
        let mut x_1: core::ffi::c_int = if y_1 < h {
            0 as core::ffi::c_int
        } else {
            x_start
        };
        while x_1 < x_end {
            *tmp.offset(x_1 as isize) = *src.offset(x_1 as isize) as int16_t;
            x_1 += 1;
        }
        src = src.offset(src_stride as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_1 += 1;
    }
    let mut y_2: core::ffi::c_int = h;
    while y_2 < y_end {
        let mut x_2: core::ffi::c_int = x_start;
        while x_2 < x_end {
            *tmp.offset(x_2 as isize) = *bottom.offset(x_2 as isize) as int16_t;
            x_2 += 1;
        }
        bottom = bottom.offset(src_stride as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_2 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn cdef_filter_block_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: core::ffi::c_int,
    sec_strength: core::ffi::c_int,
    dir: core::ffi::c_int,
    damping: core::ffi::c_int,
    w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    edges: CdefEdgeFlags,
) {
    let tmp_stride: ptrdiff_t = 12 as ptrdiff_t;
    if !((w == 4 as core::ffi::c_int || w == 8 as core::ffi::c_int)
        && (h == 4 as core::ffi::c_int || h == 8 as core::ffi::c_int))
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"cdef_filter_block_c\0"))
                .as_ptr(),
            b"cdef_tmpl.c\0" as *const u8 as *const core::ffi::c_char,
            113 as core::ffi::c_int,
            b"(w == 4 || w == 8) && (h == 4 || h == 8)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut tmp_buf: [int16_t; 144] = [0; 144];
    let mut tmp: *mut int16_t = tmp_buf
        .as_mut_ptr()
        .offset((2 as ptrdiff_t * tmp_stride) as isize)
        .offset(2 as core::ffi::c_int as isize);
    let cdef_dirs: *const [int8_t; 2] = &*dav1d_cdef_directions
        .as_ptr()
        .offset(dir as isize) as *const [int8_t; 2];
    padding(tmp, tmp_stride, dst, dst_stride, left, top, bottom, w, h, edges);
    if pri_strength != 0 {
        let bitdepth_min_8: core::ffi::c_int = 8 as core::ffi::c_int
            - 8 as core::ffi::c_int;
        let pri_tap: core::ffi::c_int = 4 as core::ffi::c_int
            - (pri_strength >> bitdepth_min_8 & 1 as core::ffi::c_int);
        let pri_shift: core::ffi::c_int = imax(
            0 as core::ffi::c_int,
            damping - ulog2(pri_strength as core::ffi::c_uint),
        ) as core::ffi::c_int;
        if sec_strength != 0 {
            let sec_shift: core::ffi::c_int = damping
                - ulog2(sec_strength as core::ffi::c_uint) as core::ffi::c_int;
            loop {
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < w {
                    let px: core::ffi::c_int = *dst.offset(x as isize)
                        as core::ffi::c_int;
                    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
                    let mut max: core::ffi::c_int = px;
                    let mut min: core::ffi::c_int = px;
                    let mut pri_tap_k: core::ffi::c_int = pri_tap;
                    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                    while k < 2 as core::ffi::c_int {
                        let off1: core::ffi::c_int = (*cdef_dirs
                            .offset(2 as core::ffi::c_int as isize))[k as usize]
                            as core::ffi::c_int;
                        let p0: core::ffi::c_int = *tmp.offset((x + off1) as isize)
                            as core::ffi::c_int;
                        let p1: core::ffi::c_int = *tmp.offset((x - off1) as isize)
                            as core::ffi::c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 as core::ffi::c_int
                            | 2 as core::ffi::c_int;
                        min = umin(p0 as core::ffi::c_uint, min as core::ffi::c_uint)
                            as core::ffi::c_int;
                        max = imax(p0, max);
                        min = umin(p1 as core::ffi::c_uint, min as core::ffi::c_uint)
                            as core::ffi::c_int;
                        max = imax(p1, max);
                        let off2: core::ffi::c_int = (*cdef_dirs
                            .offset(4 as core::ffi::c_int as isize))[k as usize]
                            as core::ffi::c_int;
                        let off3: core::ffi::c_int = (*cdef_dirs
                            .offset(0 as core::ffi::c_int as isize))[k as usize]
                            as core::ffi::c_int;
                        let s0: core::ffi::c_int = *tmp.offset((x + off2) as isize)
                            as core::ffi::c_int;
                        let s1: core::ffi::c_int = *tmp.offset((x - off2) as isize)
                            as core::ffi::c_int;
                        let s2: core::ffi::c_int = *tmp.offset((x + off3) as isize)
                            as core::ffi::c_int;
                        let s3: core::ffi::c_int = *tmp.offset((x - off3) as isize)
                            as core::ffi::c_int;
                        let sec_tap: core::ffi::c_int = 2 as core::ffi::c_int - k;
                        sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                        min = umin(s0 as core::ffi::c_uint, min as core::ffi::c_uint)
                            as core::ffi::c_int;
                        max = imax(s0, max);
                        min = umin(s1 as core::ffi::c_uint, min as core::ffi::c_uint)
                            as core::ffi::c_int;
                        max = imax(s1, max);
                        min = umin(s2 as core::ffi::c_uint, min as core::ffi::c_uint)
                            as core::ffi::c_int;
                        max = imax(s2, max);
                        min = umin(s3 as core::ffi::c_uint, min as core::ffi::c_uint)
                            as core::ffi::c_int;
                        max = imax(s3, max);
                        k += 1;
                    }
                    *dst.offset(x as isize) = iclip(
                        px
                            + (sum - (sum < 0 as core::ffi::c_int) as core::ffi::c_int
                                + 8 as core::ffi::c_int >> 4 as core::ffi::c_int),
                        min,
                        max,
                    ) as pixel;
                    x += 1;
                }
                dst = dst.offset(dst_stride as isize);
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_0 < w {
                    let px_0: core::ffi::c_int = *dst.offset(x_0 as isize)
                        as core::ffi::c_int;
                    let mut sum_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    let mut pri_tap_k_0: core::ffi::c_int = pri_tap;
                    let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while k_0 < 2 as core::ffi::c_int {
                        let off: core::ffi::c_int = (*cdef_dirs
                            .offset(2 as core::ffi::c_int as isize))[k_0 as usize]
                            as core::ffi::c_int;
                        let p0_0: core::ffi::c_int = *tmp.offset((x_0 + off) as isize)
                            as core::ffi::c_int;
                        let p1_0: core::ffi::c_int = *tmp.offset((x_0 - off) as isize)
                            as core::ffi::c_int;
                        sum_0
                            += pri_tap_k_0
                                * constrain(p0_0 - px_0, pri_strength, pri_shift);
                        sum_0
                            += pri_tap_k_0
                                * constrain(p1_0 - px_0, pri_strength, pri_shift);
                        pri_tap_k_0 = pri_tap_k_0 & 3 as core::ffi::c_int
                            | 2 as core::ffi::c_int;
                        k_0 += 1;
                    }
                    *dst.offset(x_0 as isize) = (px_0
                        + (sum_0 - (sum_0 < 0 as core::ffi::c_int) as core::ffi::c_int
                            + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)) as pixel;
                    x_0 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else {
        if (sec_strength == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"cdef_filter_block_c\0"))
                    .as_ptr(),
                b"cdef_tmpl.c\0" as *const u8 as *const core::ffi::c_char,
                191 as core::ffi::c_int,
                b"sec_strength\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let sec_shift_0: core::ffi::c_int = damping
            - ulog2(sec_strength as core::ffi::c_uint) as core::ffi::c_int;
        loop {
            let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_1 < w {
                let px_1: core::ffi::c_int = *dst.offset(x_1 as isize)
                    as core::ffi::c_int;
                let mut sum_1: core::ffi::c_int = 0 as core::ffi::c_int;
                let mut k_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while k_1 < 2 as core::ffi::c_int {
                    let off1_0: core::ffi::c_int = (*cdef_dirs
                        .offset(4 as core::ffi::c_int as isize))[k_1 as usize]
                        as core::ffi::c_int;
                    let off2_0: core::ffi::c_int = (*cdef_dirs
                        .offset(0 as core::ffi::c_int as isize))[k_1 as usize]
                        as core::ffi::c_int;
                    let s0_0: core::ffi::c_int = *tmp.offset((x_1 + off1_0) as isize)
                        as core::ffi::c_int;
                    let s1_0: core::ffi::c_int = *tmp.offset((x_1 - off1_0) as isize)
                        as core::ffi::c_int;
                    let s2_0: core::ffi::c_int = *tmp.offset((x_1 + off2_0) as isize)
                        as core::ffi::c_int;
                    let s3_0: core::ffi::c_int = *tmp.offset((x_1 - off2_0) as isize)
                        as core::ffi::c_int;
                    let sec_tap_0: core::ffi::c_int = 2 as core::ffi::c_int - k_1;
                    sum_1
                        += sec_tap_0 * constrain(s0_0 - px_1, sec_strength, sec_shift_0);
                    sum_1
                        += sec_tap_0 * constrain(s1_0 - px_1, sec_strength, sec_shift_0);
                    sum_1
                        += sec_tap_0 * constrain(s2_0 - px_1, sec_strength, sec_shift_0);
                    sum_1
                        += sec_tap_0 * constrain(s3_0 - px_1, sec_strength, sec_shift_0);
                    k_1 += 1;
                }
                *dst.offset(x_1 as isize) = (px_1
                    + (sum_1 - (sum_1 < 0 as core::ffi::c_int) as core::ffi::c_int
                        + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)) as pixel;
                x_1 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            tmp = tmp.offset(tmp_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    };
}
unsafe extern "C" fn cdef_filter_block_4x4_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: core::ffi::c_int,
    sec_strength: core::ffi::c_int,
    dir: core::ffi::c_int,
    damping: core::ffi::c_int,
    edges: CdefEdgeFlags,
) {
    cdef_filter_block_c(
        dst,
        stride,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        edges,
    );
}
unsafe extern "C" fn cdef_filter_block_4x8_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: core::ffi::c_int,
    sec_strength: core::ffi::c_int,
    dir: core::ffi::c_int,
    damping: core::ffi::c_int,
    edges: CdefEdgeFlags,
) {
    cdef_filter_block_c(
        dst,
        stride,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        edges,
    );
}
unsafe extern "C" fn cdef_filter_block_8x8_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: core::ffi::c_int,
    sec_strength: core::ffi::c_int,
    dir: core::ffi::c_int,
    damping: core::ffi::c_int,
    edges: CdefEdgeFlags,
) {
    cdef_filter_block_c(
        dst,
        stride,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        edges,
    );
}
unsafe extern "C" fn cdef_find_dir_c(
    mut img: *const pixel,
    stride: ptrdiff_t,
    var: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let bitdepth_min_8: core::ffi::c_int = 8 as core::ffi::c_int - 8 as core::ffi::c_int;
    let mut partial_sum_hv: [[core::ffi::c_int; 8]; 2] = [
        [0 as core::ffi::c_int; 8],
        [0; 8],
    ];
    let mut partial_sum_diag: [[core::ffi::c_int; 15]; 2] = [
        [0 as core::ffi::c_int; 15],
        [0; 15],
    ];
    let mut partial_sum_alt: [[core::ffi::c_int; 11]; 4] = [
        [0 as core::ffi::c_int; 11],
        [0; 11],
        [0; 11],
        [0; 11],
    ];
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < 8 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < 8 as core::ffi::c_int {
            let px: core::ffi::c_int = (*img.offset(x as isize) as core::ffi::c_int
                >> bitdepth_min_8) - 128 as core::ffi::c_int;
            partial_sum_diag[0 as core::ffi::c_int as usize][(y + x) as usize] += px;
            partial_sum_alt[0 as core::ffi::c_int
                as usize][(y + (x >> 1 as core::ffi::c_int)) as usize] += px;
            partial_sum_hv[0 as core::ffi::c_int as usize][y as usize] += px;
            partial_sum_alt[1 as core::ffi::c_int
                as usize][(3 as core::ffi::c_int + y - (x >> 1 as core::ffi::c_int))
                as usize] += px;
            partial_sum_diag[1 as core::ffi::c_int
                as usize][(7 as core::ffi::c_int + y - x) as usize] += px;
            partial_sum_alt[2 as core::ffi::c_int
                as usize][(3 as core::ffi::c_int - (y >> 1 as core::ffi::c_int) + x)
                as usize] += px;
            partial_sum_hv[1 as core::ffi::c_int as usize][x as usize] += px;
            partial_sum_alt[3 as core::ffi::c_int
                as usize][((y >> 1 as core::ffi::c_int) + x) as usize] += px;
            x += 1;
        }
        img = img.offset(stride as isize);
        y += 1;
    }
    let mut cost: [core::ffi::c_uint; 8] = [
        0 as core::ffi::c_int as core::ffi::c_uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < 8 as core::ffi::c_int {
        cost[2 as core::ffi::c_int as usize] = (cost[2 as core::ffi::c_int as usize])
            .wrapping_add(
                (partial_sum_hv[0 as core::ffi::c_int as usize][n as usize]
                    * partial_sum_hv[0 as core::ffi::c_int as usize][n as usize])
                    as core::ffi::c_uint,
            );
        cost[6 as core::ffi::c_int as usize] = (cost[6 as core::ffi::c_int as usize])
            .wrapping_add(
                (partial_sum_hv[1 as core::ffi::c_int as usize][n as usize]
                    * partial_sum_hv[1 as core::ffi::c_int as usize][n as usize])
                    as core::ffi::c_uint,
            );
        n += 1;
    }
    cost[2 as core::ffi::c_int as usize] = (cost[2 as core::ffi::c_int as usize])
        .wrapping_mul(105 as core::ffi::c_uint);
    cost[6 as core::ffi::c_int as usize] = (cost[6 as core::ffi::c_int as usize])
        .wrapping_mul(105 as core::ffi::c_uint);
    static mut div_table: [uint16_t; 7] = [
        840 as core::ffi::c_int as uint16_t,
        420 as core::ffi::c_int as uint16_t,
        280 as core::ffi::c_int as uint16_t,
        210 as core::ffi::c_int as uint16_t,
        168 as core::ffi::c_int as uint16_t,
        140 as core::ffi::c_int as uint16_t,
        120 as core::ffi::c_int as uint16_t,
    ];
    let mut n_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while n_0 < 7 as core::ffi::c_int {
        let d: core::ffi::c_int = div_table[n_0 as usize] as core::ffi::c_int;
        cost[0 as core::ffi::c_int as usize] = (cost[0 as core::ffi::c_int as usize])
            .wrapping_add(
                ((partial_sum_diag[0 as core::ffi::c_int as usize][n_0 as usize]
                    * partial_sum_diag[0 as core::ffi::c_int as usize][n_0 as usize]
                    + partial_sum_diag[0 as core::ffi::c_int
                        as usize][(14 as core::ffi::c_int - n_0) as usize]
                        * partial_sum_diag[0 as core::ffi::c_int
                            as usize][(14 as core::ffi::c_int - n_0) as usize]) * d)
                    as core::ffi::c_uint,
            );
        cost[4 as core::ffi::c_int as usize] = (cost[4 as core::ffi::c_int as usize])
            .wrapping_add(
                ((partial_sum_diag[1 as core::ffi::c_int as usize][n_0 as usize]
                    * partial_sum_diag[1 as core::ffi::c_int as usize][n_0 as usize]
                    + partial_sum_diag[1 as core::ffi::c_int
                        as usize][(14 as core::ffi::c_int - n_0) as usize]
                        * partial_sum_diag[1 as core::ffi::c_int
                            as usize][(14 as core::ffi::c_int - n_0) as usize]) * d)
                    as core::ffi::c_uint,
            );
        n_0 += 1;
    }
    cost[0 as core::ffi::c_int as usize] = (cost[0 as core::ffi::c_int as usize])
        .wrapping_add(
            (partial_sum_diag[0 as core::ffi::c_int
                as usize][7 as core::ffi::c_int as usize]
                * partial_sum_diag[0 as core::ffi::c_int
                    as usize][7 as core::ffi::c_int as usize] * 105 as core::ffi::c_int)
                as core::ffi::c_uint,
        );
    cost[4 as core::ffi::c_int as usize] = (cost[4 as core::ffi::c_int as usize])
        .wrapping_add(
            (partial_sum_diag[1 as core::ffi::c_int
                as usize][7 as core::ffi::c_int as usize]
                * partial_sum_diag[1 as core::ffi::c_int
                    as usize][7 as core::ffi::c_int as usize] * 105 as core::ffi::c_int)
                as core::ffi::c_uint,
        );
    let mut n_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while n_1 < 4 as core::ffi::c_int {
        let cost_ptr: *mut core::ffi::c_uint = &raw mut *cost
            .as_mut_ptr()
            .offset((n_1 * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
            as *mut core::ffi::c_uint;
        let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
        while m < 5 as core::ffi::c_int {
            *cost_ptr = (*cost_ptr)
                .wrapping_add(
                    (partial_sum_alt[n_1 as usize][(3 as core::ffi::c_int + m) as usize]
                        * partial_sum_alt[n_1
                            as usize][(3 as core::ffi::c_int + m) as usize])
                        as core::ffi::c_uint,
                );
            m += 1;
        }
        *cost_ptr = (*cost_ptr).wrapping_mul(105 as core::ffi::c_uint);
        let mut m_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while m_0 < 3 as core::ffi::c_int {
            let d_0: core::ffi::c_int = div_table[(2 as core::ffi::c_int * m_0
                + 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
            *cost_ptr = (*cost_ptr)
                .wrapping_add(
                    ((partial_sum_alt[n_1 as usize][m_0 as usize]
                        * partial_sum_alt[n_1 as usize][m_0 as usize]
                        + partial_sum_alt[n_1
                            as usize][(10 as core::ffi::c_int - m_0) as usize]
                            * partial_sum_alt[n_1
                                as usize][(10 as core::ffi::c_int - m_0) as usize]) * d_0)
                        as core::ffi::c_uint,
                );
            m_0 += 1;
        }
        n_1 += 1;
    }
    let mut best_dir: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut best_cost: core::ffi::c_uint = cost[0 as core::ffi::c_int as usize];
    let mut n_2: core::ffi::c_int = 1 as core::ffi::c_int;
    while n_2 < 8 as core::ffi::c_int {
        if cost[n_2 as usize] > best_cost {
            best_cost = cost[n_2 as usize];
            best_dir = n_2;
        }
        n_2 += 1;
    }
    *var = best_cost.wrapping_sub(cost[(best_dir ^ 4 as core::ffi::c_int) as usize])
        >> 10 as core::ffi::c_int;
    return best_dir;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_cdef_dsp_init_8bpc(c: *mut Dav1dCdefDSPContext) {
    (*c).dir = Some(
        cdef_find_dir_c
            as unsafe extern "C" fn(
                *const pixel,
                ptrdiff_t,
                *mut core::ffi::c_uint,
            ) -> core::ffi::c_int,
    ) as cdef_dir_fn;
    (*c).fb[0 as core::ffi::c_int as usize] = Some(
        cdef_filter_block_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 2],
                *const pixel,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                CdefEdgeFlags,
            ) -> (),
    ) as cdef_fn;
    (*c).fb[1 as core::ffi::c_int as usize] = Some(
        cdef_filter_block_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 2],
                *const pixel,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                CdefEdgeFlags,
            ) -> (),
    ) as cdef_fn;
    (*c).fb[2 as core::ffi::c_int as usize] = Some(
        cdef_filter_block_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 2],
                *const pixel,
                *const pixel,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                CdefEdgeFlags,
            ) -> (),
    ) as cdef_fn;
}
