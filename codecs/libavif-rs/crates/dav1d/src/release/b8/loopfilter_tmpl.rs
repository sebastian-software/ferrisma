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

// Mechanically translated from upstream/dav1d/src/loopfilter_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ptrdiff_t = isize;
pub type pixel = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1FilterLUT {
    pub __dav1d_align_e: [crate::platform::Align16; 0],

    pub e: [uint8_t; 64],
    pub __dav1d_align_i: [crate::platform::Align16; 0],
    pub i: [uint8_t; 64],
    pub __dav1d_align_sharp: [crate::platform::Align16; 0],
    pub sharp: [uint64_t; 2],
}
pub type loopfilter_sb_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint32_t,
        *const [uint8_t; 4],
        ptrdiff_t,
        *const Av1FilterLUT,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
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
unsafe extern "C" fn iclip_u8(v: core::ffi::c_int) -> core::ffi::c_int {
    return iclip(v, 0 as core::ffi::c_int, 255 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn loop_filter(
    mut dst: *mut pixel,
    mut E: core::ffi::c_int,
    mut I: core::ffi::c_int,
    mut H: core::ffi::c_int,
    stridea: ptrdiff_t,
    strideb: ptrdiff_t,
    wd: core::ffi::c_int,
) {
    let bitdepth_min_8: core::ffi::c_int = 8 as core::ffi::c_int - 8 as core::ffi::c_int;
    let F: core::ffi::c_int = (1 as core::ffi::c_int) << bitdepth_min_8;
    E <<= bitdepth_min_8;
    I <<= bitdepth_min_8;
    H <<= bitdepth_min_8;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        let mut p6: core::ffi::c_int = 0;
        let mut p5: core::ffi::c_int = 0;
        let mut p4: core::ffi::c_int = 0;
        let mut p3: core::ffi::c_int = 0;
        let mut p2: core::ffi::c_int = 0;
        let mut p1: core::ffi::c_int = *dst
            .offset((strideb * -(2 as core::ffi::c_int) as ptrdiff_t) as isize)
            as core::ffi::c_int;
        let mut p0: core::ffi::c_int = *dst
            .offset((strideb * -(1 as core::ffi::c_int) as ptrdiff_t) as isize)
            as core::ffi::c_int;
        let mut q0: core::ffi::c_int = *dst
            .offset((strideb * 0 as core::ffi::c_int as ptrdiff_t) as isize)
            as core::ffi::c_int;
        let mut q1: core::ffi::c_int = *dst
            .offset((strideb * 1 as core::ffi::c_int as ptrdiff_t) as isize)
            as core::ffi::c_int;
        let mut q2: core::ffi::c_int = 0;
        let mut q3: core::ffi::c_int = 0;
        let mut q4: core::ffi::c_int = 0;
        let mut q5: core::ffi::c_int = 0;
        let mut q6: core::ffi::c_int = 0;
        let mut fm: core::ffi::c_int = 0;
        let mut flat8out: core::ffi::c_int = 0;
        let mut flat8in: core::ffi::c_int = 0;
        fm = (abs(p1 - p0) <= I && abs(q1 - q0) <= I
            && abs(p0 - q0) * 2 as core::ffi::c_int
                + (abs(p1 - q1) >> 1 as core::ffi::c_int) <= E) as core::ffi::c_int;
        if wd > 4 as core::ffi::c_int {
            p2 = *dst.offset((strideb * -(3 as core::ffi::c_int) as ptrdiff_t) as isize)
                as core::ffi::c_int;
            q2 = *dst.offset((strideb * 2 as core::ffi::c_int as ptrdiff_t) as isize)
                as core::ffi::c_int;
            fm &= (abs(p2 - p1) <= I && abs(q2 - q1) <= I) as core::ffi::c_int;
            if wd > 6 as core::ffi::c_int {
                p3 = *dst
                    .offset((strideb * -(4 as core::ffi::c_int) as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                q3 = *dst.offset((strideb * 3 as core::ffi::c_int as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                fm &= (abs(p3 - p2) <= I && abs(q3 - q2) <= I) as core::ffi::c_int;
            }
        }
        if !(fm == 0) {
            if wd >= 16 as core::ffi::c_int {
                p6 = *dst
                    .offset((strideb * -(7 as core::ffi::c_int) as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                p5 = *dst
                    .offset((strideb * -(6 as core::ffi::c_int) as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                p4 = *dst
                    .offset((strideb * -(5 as core::ffi::c_int) as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                q4 = *dst.offset((strideb * 4 as core::ffi::c_int as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                q5 = *dst.offset((strideb * 5 as core::ffi::c_int as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                q6 = *dst.offset((strideb * 6 as core::ffi::c_int as ptrdiff_t) as isize)
                    as core::ffi::c_int;
                flat8out = (abs(p6 - p0) <= F && abs(p5 - p0) <= F && abs(p4 - p0) <= F
                    && abs(q4 - q0) <= F && abs(q5 - q0) <= F && abs(q6 - q0) <= F)
                    as core::ffi::c_int;
            }
            if wd >= 6 as core::ffi::c_int {
                flat8in = (abs(p2 - p0) <= F && abs(p1 - p0) <= F && abs(q1 - q0) <= F
                    && abs(q2 - q0) <= F) as core::ffi::c_int;
            }
            if wd >= 8 as core::ffi::c_int {
                flat8in &= (abs(p3 - p0) <= F && abs(q3 - q0) <= F) as core::ffi::c_int;
            }
            if wd >= 16 as core::ffi::c_int && flat8out & flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(6 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p6 + p6 + p6 + p6 + p6 + p6 * 2 as core::ffi::c_int
                    + p5 * 2 as core::ffi::c_int + p4 * 2 as core::ffi::c_int + p3 + p2
                    + p1 + p0 + q0 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * -(5 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p6 + p6 + p6 + p6 + p6 + p5 * 2 as core::ffi::c_int
                    + p4 * 2 as core::ffi::c_int + p3 * 2 as core::ffi::c_int + p2 + p1
                    + p0 + q0 + q1 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * -(4 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p6 + p6 + p6 + p6 + p5 + p4 * 2 as core::ffi::c_int
                    + p3 * 2 as core::ffi::c_int + p2 * 2 as core::ffi::c_int + p1 + p0
                    + q0 + q1 + q2 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * -(3 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p6 + p6 + p6 + p5 + p4 + p3 * 2 as core::ffi::c_int
                    + p2 * 2 as core::ffi::c_int + p1 * 2 as core::ffi::c_int + p0 + q0
                    + q1 + q2 + q3 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * -(2 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p6 + p6 + p5 + p4 + p3 + p2 * 2 as core::ffi::c_int
                    + p1 * 2 as core::ffi::c_int + p0 * 2 as core::ffi::c_int + q0 + q1
                    + q2 + q3 + q4 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p6 + p5 + p4 + p3 + p2 + p1 * 2 as core::ffi::c_int
                    + p0 * 2 as core::ffi::c_int + q0 * 2 as core::ffi::c_int + q1 + q2
                    + q3 + q4 + q5 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst.offset((strideb * 0 as core::ffi::c_int as ptrdiff_t) as isize) = (p5
                    + p4 + p3 + p2 + p1 + p0 * 2 as core::ffi::c_int
                    + q0 * 2 as core::ffi::c_int + q1 * 2 as core::ffi::c_int + q2 + q3
                    + q4 + q5 + q6 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst.offset((strideb * 1 as core::ffi::c_int as ptrdiff_t) as isize) = (p4
                    + p3 + p2 + p1 + p0 + q0 * 2 as core::ffi::c_int
                    + q1 * 2 as core::ffi::c_int + q2 * 2 as core::ffi::c_int + q3 + q4
                    + q5 + q6 + q6 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst.offset((strideb * 2 as core::ffi::c_int as ptrdiff_t) as isize) = (p3
                    + p2 + p1 + p0 + q0 + q1 * 2 as core::ffi::c_int
                    + q2 * 2 as core::ffi::c_int + q3 * 2 as core::ffi::c_int + q4 + q5
                    + q6 + q6 + q6 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst.offset((strideb * 3 as core::ffi::c_int as ptrdiff_t) as isize) = (p2
                    + p1 + p0 + q0 + q1 + q2 * 2 as core::ffi::c_int
                    + q3 * 2 as core::ffi::c_int + q4 * 2 as core::ffi::c_int + q5 + q6
                    + q6 + q6 + q6 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst.offset((strideb * 4 as core::ffi::c_int as ptrdiff_t) as isize) = (p1
                    + p0 + q0 + q1 + q2 + q3 * 2 as core::ffi::c_int
                    + q4 * 2 as core::ffi::c_int + q5 * 2 as core::ffi::c_int + q6 + q6
                    + q6 + q6 + q6 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
                *dst.offset((strideb * 5 as core::ffi::c_int as ptrdiff_t) as isize) = (p0
                    + q0 + q1 + q2 + q3 + q4 * 2 as core::ffi::c_int
                    + q5 * 2 as core::ffi::c_int + q6 * 2 as core::ffi::c_int + q6 + q6
                    + q6 + q6 + q6 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
                    as pixel;
            } else if wd >= 8 as core::ffi::c_int && flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(3 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p3 + p3 + p3 + 2 as core::ffi::c_int * p2 + p1 + p0 + q0
                    + 4 as core::ffi::c_int >> 3 as core::ffi::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(2 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p3 + p3 + p2 + 2 as core::ffi::c_int * p1 + p0 + q0 + q1
                    + 4 as core::ffi::c_int >> 3 as core::ffi::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p3 + p2 + p1 + 2 as core::ffi::c_int * p0 + q0 + q1 + q2
                    + 4 as core::ffi::c_int >> 3 as core::ffi::c_int) as pixel;
                *dst.offset((strideb * 0 as core::ffi::c_int as ptrdiff_t) as isize) = (p2
                    + p1 + p0 + 2 as core::ffi::c_int * q0 + q1 + q2 + q3
                    + 4 as core::ffi::c_int >> 3 as core::ffi::c_int) as pixel;
                *dst.offset((strideb * 1 as core::ffi::c_int as ptrdiff_t) as isize) = (p1
                    + p0 + q0 + 2 as core::ffi::c_int * q1 + q2 + q3 + q3
                    + 4 as core::ffi::c_int >> 3 as core::ffi::c_int) as pixel;
                *dst.offset((strideb * 2 as core::ffi::c_int as ptrdiff_t) as isize) = (p0
                    + q0 + q1 + 2 as core::ffi::c_int * q2 + q3 + q3 + q3
                    + 4 as core::ffi::c_int >> 3 as core::ffi::c_int) as pixel;
            } else if wd == 6 as core::ffi::c_int && flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(2 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p2 + 2 as core::ffi::c_int * p2 + 2 as core::ffi::c_int * p1
                    + 2 as core::ffi::c_int * p0 + q0 + 4 as core::ffi::c_int
                    >> 3 as core::ffi::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as core::ffi::c_int) as ptrdiff_t) as isize,
                    ) = (p2 + 2 as core::ffi::c_int * p1 + 2 as core::ffi::c_int * p0
                    + 2 as core::ffi::c_int * q0 + q1 + 4 as core::ffi::c_int
                    >> 3 as core::ffi::c_int) as pixel;
                *dst.offset((strideb * 0 as core::ffi::c_int as ptrdiff_t) as isize) = (p1
                    + 2 as core::ffi::c_int * p0 + 2 as core::ffi::c_int * q0
                    + 2 as core::ffi::c_int * q1 + q2 + 4 as core::ffi::c_int
                    >> 3 as core::ffi::c_int) as pixel;
                *dst.offset((strideb * 1 as core::ffi::c_int as ptrdiff_t) as isize) = (p0
                    + 2 as core::ffi::c_int * q0 + 2 as core::ffi::c_int * q1
                    + 2 as core::ffi::c_int * q2 + q2 + 4 as core::ffi::c_int
                    >> 3 as core::ffi::c_int) as pixel;
            } else {
                let hev: core::ffi::c_int = (abs(p1 - p0) > H || abs(q1 - q0) > H)
                    as core::ffi::c_int;
                if hev != 0 {
                    let mut f: core::ffi::c_int = iclip(
                        p1 - q1,
                        -(128 as core::ffi::c_int)
                            * ((1 as core::ffi::c_int) << bitdepth_min_8),
                        128 as core::ffi::c_int
                            * ((1 as core::ffi::c_int) << bitdepth_min_8)
                            - 1 as core::ffi::c_int,
                    );
                    let mut f1: core::ffi::c_int = 0;
                    let mut f2: core::ffi::c_int = 0;
                    f = iclip(
                        3 as core::ffi::c_int * (q0 - p0) + f,
                        -(128 as core::ffi::c_int)
                            * ((1 as core::ffi::c_int) << bitdepth_min_8),
                        128 as core::ffi::c_int
                            * ((1 as core::ffi::c_int) << bitdepth_min_8)
                            - 1 as core::ffi::c_int,
                    );
                    f1 = imin(
                        f + 4 as core::ffi::c_int,
                        ((128 as core::ffi::c_int) << bitdepth_min_8)
                            - 1 as core::ffi::c_int,
                    ) >> 3 as core::ffi::c_int;
                    f2 = imin(
                        f + 3 as core::ffi::c_int,
                        ((128 as core::ffi::c_int) << bitdepth_min_8)
                            - 1 as core::ffi::c_int,
                    ) >> 3 as core::ffi::c_int;
                    *dst
                        .offset(
                            (strideb * -(1 as core::ffi::c_int) as ptrdiff_t) as isize,
                        ) = iclip_u8(p0 + f2) as pixel;
                    *dst
                        .offset(
                            (strideb * 0 as core::ffi::c_int as ptrdiff_t) as isize,
                        ) = iclip_u8(q0 - f1) as pixel;
                } else {
                    let mut f_0: core::ffi::c_int = iclip(
                        3 as core::ffi::c_int * (q0 - p0),
                        -(128 as core::ffi::c_int)
                            * ((1 as core::ffi::c_int) << bitdepth_min_8),
                        128 as core::ffi::c_int
                            * ((1 as core::ffi::c_int) << bitdepth_min_8)
                            - 1 as core::ffi::c_int,
                    );
                    let f1_0: core::ffi::c_int = imin(
                        f_0 + 4 as core::ffi::c_int,
                        ((128 as core::ffi::c_int) << bitdepth_min_8)
                            - 1 as core::ffi::c_int,
                    ) as core::ffi::c_int >> 3 as core::ffi::c_int;
                    let f2_0: core::ffi::c_int = imin(
                        f_0 + 3 as core::ffi::c_int,
                        ((128 as core::ffi::c_int) << bitdepth_min_8)
                            - 1 as core::ffi::c_int,
                    ) as core::ffi::c_int >> 3 as core::ffi::c_int;
                    *dst
                        .offset(
                            (strideb * -(1 as core::ffi::c_int) as ptrdiff_t) as isize,
                        ) = iclip_u8(p0 + f2_0) as pixel;
                    *dst
                        .offset(
                            (strideb * 0 as core::ffi::c_int as ptrdiff_t) as isize,
                        ) = iclip_u8(q0 - f1_0) as pixel;
                    f_0 = f1_0 + 1 as core::ffi::c_int >> 1 as core::ffi::c_int;
                    *dst
                        .offset(
                            (strideb * -(2 as core::ffi::c_int) as ptrdiff_t) as isize,
                        ) = iclip_u8(p1 + f_0) as pixel;
                    *dst
                        .offset(
                            (strideb * 1 as core::ffi::c_int as ptrdiff_t) as isize,
                        ) = iclip_u8(q1 - f_0) as pixel;
                }
            }
        }
        i += 1;
        dst = dst.offset(stridea as isize);
    }
}
unsafe extern "C" fn loop_filter_h_sb128y_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    h: core::ffi::c_int,
) {
    let vm: core::ffi::c_uint = *vmask.offset(0 as core::ffi::c_int as isize)
        | *vmask.offset(1 as core::ffi::c_int as isize)
        | *vmask.offset(2 as core::ffi::c_int as isize);
    let mut y: core::ffi::c_uint = 1 as core::ffi::c_uint;
    while vm & !y.wrapping_sub(1 as core::ffi::c_uint) != 0 {
        if vm & y != 0 {
            let L: core::ffi::c_int = if (*l
                .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
            {
                (*l
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] as core::ffi::c_int
            } else {
                (*l
                    .offset(
                        -(1 as core::ffi::c_int) as isize,
                    ))[0 as core::ffi::c_int as usize] as core::ffi::c_int
            };
            if !(L == 0) {
                let H: core::ffi::c_int = L >> 4 as core::ffi::c_int;
                let E: core::ffi::c_int = (*lut).e[L as usize] as core::ffi::c_int;
                let I: core::ffi::c_int = (*lut).i[L as usize] as core::ffi::c_int;
                let idx: core::ffi::c_int = if *vmask
                    .offset(2 as core::ffi::c_int as isize) & y as uint32_t != 0
                {
                    2 as core::ffi::c_int
                } else {
                    (*vmask.offset(1 as core::ffi::c_int as isize) & y as uint32_t != 0)
                        as core::ffi::c_int
                };
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    stride,
                    1 as ptrdiff_t,
                    (4 as core::ffi::c_int) << idx,
                );
            }
        }
        y <<= 1 as core::ffi::c_int;
        dst = dst.offset((4 as ptrdiff_t * stride) as isize);
        l = l.offset(b4_stride as isize);
    }
}
unsafe extern "C" fn loop_filter_v_sb128y_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    w: core::ffi::c_int,
) {
    let vm: core::ffi::c_uint = *vmask.offset(0 as core::ffi::c_int as isize)
        | *vmask.offset(1 as core::ffi::c_int as isize)
        | *vmask.offset(2 as core::ffi::c_int as isize);
    let mut x: core::ffi::c_uint = 1 as core::ffi::c_uint;
    while vm & !x.wrapping_sub(1 as core::ffi::c_uint) != 0 {
        if vm & x != 0 {
            let L: core::ffi::c_int = if (*l
                .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
            {
                (*l
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] as core::ffi::c_int
            } else {
                (*l.offset(-b4_stride as isize))[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int
            };
            if !(L == 0) {
                let H: core::ffi::c_int = L >> 4 as core::ffi::c_int;
                let E: core::ffi::c_int = (*lut).e[L as usize] as core::ffi::c_int;
                let I: core::ffi::c_int = (*lut).i[L as usize] as core::ffi::c_int;
                let idx: core::ffi::c_int = if *vmask
                    .offset(2 as core::ffi::c_int as isize) & x as uint32_t != 0
                {
                    2 as core::ffi::c_int
                } else {
                    (*vmask.offset(1 as core::ffi::c_int as isize) & x as uint32_t != 0)
                        as core::ffi::c_int
                };
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as ptrdiff_t,
                    stride,
                    (4 as core::ffi::c_int) << idx,
                );
            }
        }
        x <<= 1 as core::ffi::c_int;
        dst = dst.offset(4 as core::ffi::c_int as isize);
        l = l.offset(1);
    }
}
unsafe extern "C" fn loop_filter_h_sb128uv_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    h: core::ffi::c_int,
) {
    let vm: core::ffi::c_uint = *vmask.offset(0 as core::ffi::c_int as isize)
        | *vmask.offset(1 as core::ffi::c_int as isize);
    let mut y: core::ffi::c_uint = 1 as core::ffi::c_uint;
    while vm & !y.wrapping_sub(1 as core::ffi::c_uint) != 0 {
        if vm & y != 0 {
            let L: core::ffi::c_int = if (*l
                .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
            {
                (*l
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] as core::ffi::c_int
            } else {
                (*l
                    .offset(
                        -(1 as core::ffi::c_int) as isize,
                    ))[0 as core::ffi::c_int as usize] as core::ffi::c_int
            };
            if !(L == 0) {
                let H: core::ffi::c_int = L >> 4 as core::ffi::c_int;
                let E: core::ffi::c_int = (*lut).e[L as usize] as core::ffi::c_int;
                let I: core::ffi::c_int = (*lut).i[L as usize] as core::ffi::c_int;
                let idx: core::ffi::c_int = (*vmask
                    .offset(1 as core::ffi::c_int as isize) & y as uint32_t != 0)
                    as core::ffi::c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    stride,
                    1 as ptrdiff_t,
                    4 as core::ffi::c_int + 2 as core::ffi::c_int * idx,
                );
            }
        }
        y <<= 1 as core::ffi::c_int;
        dst = dst.offset((4 as ptrdiff_t * stride) as isize);
        l = l.offset(b4_stride as isize);
    }
}
unsafe extern "C" fn loop_filter_v_sb128uv_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    w: core::ffi::c_int,
) {
    let vm: core::ffi::c_uint = *vmask.offset(0 as core::ffi::c_int as isize)
        | *vmask.offset(1 as core::ffi::c_int as isize);
    let mut x: core::ffi::c_uint = 1 as core::ffi::c_uint;
    while vm & !x.wrapping_sub(1 as core::ffi::c_uint) != 0 {
        if vm & x != 0 {
            let L: core::ffi::c_int = if (*l
                .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
            {
                (*l
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] as core::ffi::c_int
            } else {
                (*l.offset(-b4_stride as isize))[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int
            };
            if !(L == 0) {
                let H: core::ffi::c_int = L >> 4 as core::ffi::c_int;
                let E: core::ffi::c_int = (*lut).e[L as usize] as core::ffi::c_int;
                let I: core::ffi::c_int = (*lut).i[L as usize] as core::ffi::c_int;
                let idx: core::ffi::c_int = (*vmask
                    .offset(1 as core::ffi::c_int as isize) & x as uint32_t != 0)
                    as core::ffi::c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as ptrdiff_t,
                    stride,
                    4 as core::ffi::c_int + 2 as core::ffi::c_int * idx,
                );
            }
        }
        x <<= 1 as core::ffi::c_int;
        dst = dst.offset(4 as core::ffi::c_int as isize);
        l = l.offset(1);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_filter_dsp_init_8bpc(
    c: *mut Dav1dLoopFilterDSPContext,
) {
    (*c)
        .loop_filter_sb[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize] = Some(
        loop_filter_h_sb128y_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                core::ffi::c_int,
            ) -> (),
    ) as loopfilter_sb_fn;
    (*c)
        .loop_filter_sb[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] = Some(
        loop_filter_v_sb128y_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                core::ffi::c_int,
            ) -> (),
    ) as loopfilter_sb_fn;
    (*c)
        .loop_filter_sb[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize] = Some(
        loop_filter_h_sb128uv_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                core::ffi::c_int,
            ) -> (),
    ) as loopfilter_sb_fn;
    (*c)
        .loop_filter_sb[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] = Some(
        loop_filter_v_sb128uv_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                core::ffi::c_int,
            ) -> (),
    ) as loopfilter_sb_fn;
}
