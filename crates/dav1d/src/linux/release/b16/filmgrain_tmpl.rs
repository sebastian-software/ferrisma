/*
 * Copyright © 2018, Niklas Haas
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

// Mechanically translated from upstream/dav1d/src/filmgrain_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    static dav1d_gaussian_sequence: [int16_t; 2048];
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type pixel = uint16_t;
pub type Dav1dPixelLayout = core::ffi::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainData {
    pub seed: core::ffi::c_uint,
    pub num_y_points: core::ffi::c_int,
    pub y_points: [[uint8_t; 2]; 14],
    pub chroma_scaling_from_luma: core::ffi::c_int,
    pub num_uv_points: [core::ffi::c_int; 2],
    pub uv_points: [[[uint8_t; 2]; 10]; 2],
    pub scaling_shift: core::ffi::c_int,
    pub ar_coeff_lag: core::ffi::c_int,
    pub ar_coeffs_y: [int8_t; 24],
    pub ar_coeffs_uv: [[int8_t; 28]; 2],
    pub ar_coeff_shift: uint64_t,
    pub grain_scale_shift: core::ffi::c_int,
    pub uv_mult: [core::ffi::c_int; 2],
    pub uv_luma_mult: [core::ffi::c_int; 2],
    pub uv_offset: [core::ffi::c_int; 2],
    pub overlap_flag: core::ffi::c_int,
    pub clip_to_restricted_range: core::ffi::c_int,
}
pub type entry = int16_t;
pub type generate_grain_y_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const Dav1dFilmGrainData,
        core::ffi::c_int,
    ) -> (),
>;
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        core::ffi::c_int,
    ) -> (),
>;
pub type fgy_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type fguv_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        core::ffi::c_int,
        core::ffi::c_int,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
#[inline]
unsafe extern "C" fn clz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
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
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 as ptrdiff_t != 0 {
        unreachable!();
    }
    return x >> 1 as core::ffi::c_int;
}
pub const GRAIN_WIDTH: core::ffi::c_int = 82 as core::ffi::c_int;
pub const GRAIN_HEIGHT: core::ffi::c_int = 73 as core::ffi::c_int;
pub const FG_BLOCK_SIZE: core::ffi::c_int = 32 as core::ffi::c_int;
pub const SUB_GRAIN_WIDTH: core::ffi::c_int = 44 as core::ffi::c_int;
pub const SUB_GRAIN_HEIGHT: core::ffi::c_int = 38 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn get_random_number(
    bits: core::ffi::c_int,
    state: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let r: core::ffi::c_int = *state as core::ffi::c_int;
    let mut bit: core::ffi::c_uint = ((r >> 0 as core::ffi::c_int
        ^ r >> 1 as core::ffi::c_int ^ r >> 3 as core::ffi::c_int
        ^ r >> 12 as core::ffi::c_int) & 1 as core::ffi::c_int) as core::ffi::c_uint;
    *state = (r >> 1 as core::ffi::c_int) as core::ffi::c_uint
        | bit << 15 as core::ffi::c_int;
    return (*state >> 16 as core::ffi::c_int - bits
        & (((1 as core::ffi::c_int) << bits) - 1 as core::ffi::c_int)
            as core::ffi::c_uint) as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn round2(x: core::ffi::c_int, shift: uint64_t) -> core::ffi::c_int {
    return x + ((1 as core::ffi::c_int) << shift >> 1 as core::ffi::c_int) >> shift;
}
unsafe extern "C" fn generate_grain_y_c(
    mut buf: *mut [entry; 82],
    data: *const Dav1dFilmGrainData,
    bitdepth_max: core::ffi::c_int,
) {
    let bitdepth_min_8: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int
        - 8 as core::ffi::c_int;
    let mut seed: core::ffi::c_uint = (*data).seed;
    let shift: core::ffi::c_int = 4 as core::ffi::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: core::ffi::c_int = (128 as core::ffi::c_int) << bitdepth_min_8;
    let grain_min: core::ffi::c_int = -grain_ctr;
    let grain_max: core::ffi::c_int = grain_ctr - 1 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < GRAIN_HEIGHT {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < GRAIN_WIDTH {
            let value: core::ffi::c_int = get_random_number(
                11 as core::ffi::c_int,
                &raw mut seed,
            ) as core::ffi::c_int;
            (*buf.offset(y as isize))[x as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as core::ffi::c_int,
                shift as uint64_t,
            ) as entry;
            x += 1;
        }
        y += 1;
    }
    let ar_pad: core::ffi::c_int = 3 as core::ffi::c_int;
    let ar_lag: core::ffi::c_int = (*data).ar_coeff_lag;
    let mut y_0: core::ffi::c_int = ar_pad;
    while y_0 < GRAIN_HEIGHT {
        let mut x_0: core::ffi::c_int = ar_pad;
        while x_0 < GRAIN_WIDTH - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_y).as_ptr();
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut dy: core::ffi::c_int = -ar_lag;
            while dy <= 0 as core::ffi::c_int {
                let mut dx: core::ffi::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh1 = coeff;
                    coeff = coeff.offset(1);
                    sum
                        += *fresh1 as core::ffi::c_int
                            * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                as core::ffi::c_int;
                    dx += 1;
                }
                dy += 1;
            }
            let grain: core::ffi::c_int = (*buf.offset(y_0 as isize))[x_0 as usize]
                as core::ffi::c_int
                + round2(sum, (*data).ar_coeff_shift) as core::ffi::c_int;
            (*buf.offset(y_0 as isize))[x_0 as usize] = iclip(
                grain,
                grain_min,
                grain_max,
            ) as entry;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn generate_grain_uv_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    subx: core::ffi::c_int,
    suby: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let bitdepth_min_8: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int
        - 8 as core::ffi::c_int;
    let mut seed: core::ffi::c_uint = (*data).seed
        ^ (if uv != 0 { 0x49d8 as core::ffi::c_int } else { 0xb524 as core::ffi::c_int })
            as core::ffi::c_uint;
    let shift: core::ffi::c_int = 4 as core::ffi::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: core::ffi::c_int = (128 as core::ffi::c_int) << bitdepth_min_8;
    let grain_min: core::ffi::c_int = -grain_ctr;
    let grain_max: core::ffi::c_int = grain_ctr - 1 as core::ffi::c_int;
    let chromaW: core::ffi::c_int = if subx != 0 {
        SUB_GRAIN_WIDTH
    } else {
        GRAIN_WIDTH
    };
    let chromaH: core::ffi::c_int = if suby != 0 {
        SUB_GRAIN_HEIGHT
    } else {
        GRAIN_HEIGHT
    };
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < chromaH {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < chromaW {
            let value: core::ffi::c_int = get_random_number(
                11 as core::ffi::c_int,
                &raw mut seed,
            ) as core::ffi::c_int;
            (*buf.offset(y as isize))[x as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as core::ffi::c_int,
                shift as uint64_t,
            ) as entry;
            x += 1;
        }
        y += 1;
    }
    let ar_pad: core::ffi::c_int = 3 as core::ffi::c_int;
    let ar_lag: core::ffi::c_int = (*data).ar_coeff_lag;
    let mut y_0: core::ffi::c_int = ar_pad;
    while y_0 < chromaH {
        let mut x_0: core::ffi::c_int = ar_pad;
        while x_0 < chromaW - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_uv[uv as usize]).as_ptr();
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut dy: core::ffi::c_int = -ar_lag;
            while dy <= 0 as core::ffi::c_int {
                let mut dx: core::ffi::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        if (*data).num_y_points == 0 {
                            break;
                        }
                        let mut luma: core::ffi::c_int = 0 as core::ffi::c_int;
                        let lumaX: core::ffi::c_int = (x_0 - ar_pad << subx) + ar_pad;
                        let lumaY: core::ffi::c_int = (y_0 - ar_pad << suby) + ar_pad;
                        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                        while i <= suby {
                            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                            while j <= subx {
                                luma
                                    += (*buf_y
                                        .offset((lumaY + i) as isize))[(lumaX + j) as usize]
                                        as core::ffi::c_int;
                                j += 1;
                            }
                            i += 1;
                        }
                        luma = round2(luma, (subx + suby) as uint64_t);
                        sum += luma * *coeff as core::ffi::c_int;
                        break;
                    } else {
                        let fresh0 = coeff;
                        coeff = coeff.offset(1);
                        sum
                            += *fresh0 as core::ffi::c_int
                                * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                    as core::ffi::c_int;
                        dx += 1;
                    }
                }
                dy += 1;
            }
            let grain: core::ffi::c_int = (*buf.offset(y_0 as isize))[x_0 as usize]
                as core::ffi::c_int
                + round2(sum, (*data).ar_coeff_shift) as core::ffi::c_int;
            (*buf.offset(y_0 as isize))[x_0 as usize] = iclip(
                grain,
                grain_min,
                grain_max,
            ) as entry;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
unsafe extern "C" fn generate_grain_uv_420_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: core::ffi::c_int,
) {
    generate_grain_uv_c(
        buf,
        buf_y,
        data,
        uv,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn generate_grain_uv_422_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: core::ffi::c_int,
) {
    generate_grain_uv_c(
        buf,
        buf_y,
        data,
        uv,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn generate_grain_uv_444_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: core::ffi::c_int,
) {
    generate_grain_uv_c(
        buf,
        buf_y,
        data,
        uv,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bitdepth_max,
    );
}
#[inline]
unsafe extern "C" fn sample_lut(
    mut grain_lut: *const [entry; 82],
    mut offsets: *const [core::ffi::c_int; 2],
    subx: core::ffi::c_int,
    suby: core::ffi::c_int,
    bx: core::ffi::c_int,
    by: core::ffi::c_int,
    x: core::ffi::c_int,
    y: core::ffi::c_int,
) -> entry {
    let randval: core::ffi::c_int = (*offsets.offset(bx as isize))[by as usize];
    let offx: core::ffi::c_int = 3 as core::ffi::c_int
        + (2 as core::ffi::c_int >> subx)
            * (3 as core::ffi::c_int + (randval >> 4 as core::ffi::c_int));
    let offy: core::ffi::c_int = 3 as core::ffi::c_int
        + (2 as core::ffi::c_int >> suby)
            * (3 as core::ffi::c_int + (randval & 0xf as core::ffi::c_int));
    return (*grain_lut
        .offset(
            (offy + y + (FG_BLOCK_SIZE >> suby) * by) as isize,
        ))[(offx + x + (FG_BLOCK_SIZE >> subx) * bx) as usize];
}
unsafe extern "C" fn fgy_32x32xn_c(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: core::ffi::c_int,
    row_num: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let rows: core::ffi::c_int = 1 as core::ffi::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as core::ffi::c_int)
            as core::ffi::c_int;
    let bitdepth_min_8: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int
        - 8 as core::ffi::c_int;
    let grain_ctr: core::ffi::c_int = (128 as core::ffi::c_int) << bitdepth_min_8;
    let grain_min: core::ffi::c_int = -grain_ctr;
    let grain_max: core::ffi::c_int = grain_ctr - 1 as core::ffi::c_int;
    let mut min_value: core::ffi::c_int = 0;
    let mut max_value: core::ffi::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16 as core::ffi::c_int) << bitdepth_min_8;
        max_value = (235 as core::ffi::c_int) << bitdepth_min_8;
    } else {
        min_value = 0 as core::ffi::c_int;
        max_value = bitdepth_max;
    }
    let mut seed: [core::ffi::c_uint; 2] = [0; 2];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 as core::ffi::c_int + 178 as core::ffi::c_int
                & 0xff as core::ffi::c_int) << 8 as core::ffi::c_int)
                as core::ffi::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as core::ffi::c_int + 105 as core::ffi::c_int
                & 0xff as core::ffi::c_int) as core::ffi::c_uint;
        i += 1;
    }
    if !((stride as usize)
        .wrapping_rem(
            (32 as usize).wrapping_mul(::core::mem::size_of::<pixel>() as usize),
        ) == 0 as usize)
    {
        unreachable!();
    }
    let mut offsets: [[core::ffi::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while (bx as size_t) < pw {
        let bw: core::ffi::c_int = imin(
            FG_BLOCK_SIZE,
            (pw as core::ffi::c_int as core::ffi::c_uint).wrapping_sub(bx)
                as core::ffi::c_int,
        ) as core::ffi::c_int;
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < rows {
                offsets[1 as core::ffi::c_int as usize][i_0 as usize] = offsets[0
                    as core::ffi::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < rows {
            offsets[0 as core::ffi::c_int as usize][i_1 as usize] = get_random_number(
                8 as core::ffi::c_int,
                &raw mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let ystart: core::ffi::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            imin(2 as core::ffi::c_int, bh) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        let xstart: core::ffi::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            imin(2 as core::ffi::c_int, bw) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        static mut w: [[core::ffi::c_int; 2]; 2] = [
            [27 as core::ffi::c_int, 17 as core::ffi::c_int],
            [17 as core::ffi::c_int, 27 as core::ffi::c_int],
        ];
        let mut y: core::ffi::c_int = ystart;
        while y < bh {
            let mut x: core::ffi::c_int = xstart;
            while x < bw {
                let mut grain: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x,
                    y,
                ) as core::ffi::c_int;
                let src: *const pixel = src_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let noise: core::ffi::c_int = round2(
                    *scaling.offset(*src as isize) as core::ffi::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst = iclip(*src as core::ffi::c_int + noise, min_value, max_value)
                    as pixel;
                x += 1;
            }
            let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_0 < xstart {
                let mut grain_0: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_0,
                    y,
                ) as core::ffi::c_int;
                let mut old: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_0,
                    y,
                ) as core::ffi::c_int;
                grain_0 = round2(
                    old * w[x_0 as usize][0 as core::ffi::c_int as usize]
                        + grain_0 * w[x_0 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let src_0: *const pixel = src_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let noise_0: core::ffi::c_int = round2(
                    *scaling.offset(*src_0 as isize) as core::ffi::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst_0 = iclip(
                    *src_0 as core::ffi::c_int + noise_0,
                    min_value,
                    max_value,
                ) as pixel;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_0 < ystart {
            let mut x_1: core::ffi::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_1,
                    y_0,
                ) as core::ffi::c_int;
                let mut old_0: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    x_1,
                    y_0,
                ) as core::ffi::c_int;
                grain_1 = round2(
                    old_0 * w[y_0 as usize][0 as core::ffi::c_int as usize]
                        + grain_1 * w[y_0 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let src_1: *const pixel = src_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let noise_1: core::ffi::c_int = round2(
                    *scaling.offset(*src_1 as isize) as core::ffi::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst_1 = iclip(
                    *src_1 as core::ffi::c_int + noise_1,
                    min_value,
                    max_value,
                ) as pixel;
                x_1 += 1;
            }
            let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_2 < xstart {
                let mut top: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                let mut old_1: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                top = round2(
                    old_1 * w[x_2 as usize][0 as core::ffi::c_int as usize]
                        + top * w[x_2 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                old_1 = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                grain_2 = round2(
                    old_1 * w[x_2 as usize][0 as core::ffi::c_int as usize]
                        + grain_2 * w[x_2 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[y_0 as usize][0 as core::ffi::c_int as usize]
                        + grain_2 * w[y_0 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let src_2: *const pixel = src_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let noise_2: core::ffi::c_int = round2(
                    *scaling.offset(*src_2 as isize) as core::ffi::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst_2 = iclip(
                    *src_2 as core::ffi::c_int + noise_2,
                    min_value,
                    max_value,
                ) as pixel;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add(FG_BLOCK_SIZE as core::ffi::c_uint);
    }
}
#[inline(never)]
unsafe extern "C" fn fguv_32x32xn_c(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: core::ffi::c_int,
    row_num: core::ffi::c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: core::ffi::c_int,
    is_id: core::ffi::c_int,
    sx: core::ffi::c_int,
    sy: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    let rows: core::ffi::c_int = 1 as core::ffi::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as core::ffi::c_int)
            as core::ffi::c_int;
    let bitdepth_min_8: core::ffi::c_int = 32 as core::ffi::c_int
        - clz(bitdepth_max as core::ffi::c_uint) as core::ffi::c_int
        - 8 as core::ffi::c_int;
    let grain_ctr: core::ffi::c_int = (128 as core::ffi::c_int) << bitdepth_min_8;
    let grain_min: core::ffi::c_int = -grain_ctr;
    let grain_max: core::ffi::c_int = grain_ctr - 1 as core::ffi::c_int;
    let mut min_value: core::ffi::c_int = 0;
    let mut max_value: core::ffi::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16 as core::ffi::c_int) << bitdepth_min_8;
        max_value = (if is_id != 0 {
            235 as core::ffi::c_int
        } else {
            240 as core::ffi::c_int
        }) << bitdepth_min_8;
    } else {
        min_value = 0 as core::ffi::c_int;
        max_value = bitdepth_max;
    }
    let mut seed: [core::ffi::c_uint; 2] = [0; 2];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 as core::ffi::c_int + 178 as core::ffi::c_int
                & 0xff as core::ffi::c_int) << 8 as core::ffi::c_int)
                as core::ffi::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as core::ffi::c_int + 105 as core::ffi::c_int
                & 0xff as core::ffi::c_int) as core::ffi::c_uint;
        i += 1;
    }
    if !((stride as usize)
        .wrapping_rem(
            (32 as usize).wrapping_mul(::core::mem::size_of::<pixel>() as usize),
        ) == 0 as usize)
    {
        unreachable!();
    }
    let mut offsets: [[core::ffi::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while (bx as size_t) < pw {
        let bw: core::ffi::c_int = imin(
            FG_BLOCK_SIZE >> sx,
            pw.wrapping_sub(bx as size_t) as core::ffi::c_int,
        ) as core::ffi::c_int;
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < rows {
                offsets[1 as core::ffi::c_int as usize][i_0 as usize] = offsets[0
                    as core::ffi::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < rows {
            offsets[0 as core::ffi::c_int as usize][i_1 as usize] = get_random_number(
                8 as core::ffi::c_int,
                &raw mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let ystart: core::ffi::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            imin(2 as core::ffi::c_int >> sy, bh) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        let xstart: core::ffi::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            imin(2 as core::ffi::c_int >> sx, bw) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        static mut w: [[[core::ffi::c_int; 2]; 2]; 2] = [
            [
                [27 as core::ffi::c_int, 17 as core::ffi::c_int],
                [17 as core::ffi::c_int, 27 as core::ffi::c_int],
            ],
            [[23 as core::ffi::c_int, 22 as core::ffi::c_int], [0; 2]],
        ];
        let mut y: core::ffi::c_int = ystart;
        while y < bh {
            let mut x: core::ffi::c_int = xstart;
            while x < bw {
                let mut grain: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x,
                    y,
                ) as core::ffi::c_int;
                let lx: core::ffi::c_int = (bx.wrapping_add(x as core::ffi::c_uint)
                    << sx) as core::ffi::c_int;
                let ly: core::ffi::c_int = y << sy;
                let luma: *const pixel = luma_row
                    .offset((ly as ptrdiff_t * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx as isize);
                let mut avg: pixel = *luma.offset(0 as core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as core::ffi::c_int
                        + *luma.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int + 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as pixel;
                }
                let src: *const pixel = src_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x as core::ffi::c_uint) as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x as core::ffi::c_uint) as isize);
                let mut val: core::ffi::c_int = avg as core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined: core::ffi::c_int = avg as core::ffi::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src as core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = iclip(
                        (combined >> 6 as core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as core::ffi::c_int) << bitdepth_min_8),
                        0 as core::ffi::c_int,
                        bitdepth_max,
                    );
                }
                let noise: core::ffi::c_int = round2(
                    *scaling.offset(val as isize) as core::ffi::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst = iclip(*src as core::ffi::c_int + noise, min_value, max_value)
                    as pixel;
                x += 1;
            }
            let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_0 < xstart {
                let mut grain_0: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_0,
                    y,
                ) as core::ffi::c_int;
                let mut old: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_0,
                    y,
                ) as core::ffi::c_int;
                grain_0 = round2(
                    old * w[sx as usize][x_0 as usize][0 as core::ffi::c_int as usize]
                        + grain_0
                            * w[sx
                                as usize][x_0 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let lx_0: core::ffi::c_int = (bx.wrapping_add(x_0 as core::ffi::c_uint)
                    << sx) as core::ffi::c_int;
                let ly_0: core::ffi::c_int = y << sy;
                let luma_0: *const pixel = luma_row
                    .offset((ly_0 as ptrdiff_t * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_0 as isize);
                let mut avg_0: pixel = *luma_0.offset(0 as core::ffi::c_int as isize);
                if sx != 0 {
                    avg_0 = (avg_0 as core::ffi::c_int
                        + *luma_0.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int + 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as pixel;
                }
                let src_0: *const pixel = src_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_0 as core::ffi::c_uint) as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_0 as core::ffi::c_uint) as isize);
                let mut val_0: core::ffi::c_int = avg_0 as core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_0: core::ffi::c_int = avg_0 as core::ffi::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_0 as core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val_0 = iclip(
                        (combined_0 >> 6 as core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as core::ffi::c_int) << bitdepth_min_8),
                        0 as core::ffi::c_int,
                        bitdepth_max,
                    );
                }
                let noise_0: core::ffi::c_int = round2(
                    *scaling.offset(val_0 as isize) as core::ffi::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst_0 = iclip(
                    *src_0 as core::ffi::c_int + noise_0,
                    min_value,
                    max_value,
                ) as pixel;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_0 < ystart {
            let mut x_1: core::ffi::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_1,
                    y_0,
                ) as core::ffi::c_int;
                let mut old_0: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    x_1,
                    y_0,
                ) as core::ffi::c_int;
                grain_1 = round2(
                    old_0 * w[sy as usize][y_0 as usize][0 as core::ffi::c_int as usize]
                        + grain_1
                            * w[sy
                                as usize][y_0 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let lx_1: core::ffi::c_int = (bx.wrapping_add(x_1 as core::ffi::c_uint)
                    << sx) as core::ffi::c_int;
                let ly_1: core::ffi::c_int = y_0 << sy;
                let luma_1: *const pixel = luma_row
                    .offset((ly_1 as ptrdiff_t * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_1 as isize);
                let mut avg_1: pixel = *luma_1.offset(0 as core::ffi::c_int as isize);
                if sx != 0 {
                    avg_1 = (avg_1 as core::ffi::c_int
                        + *luma_1.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int + 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as pixel;
                }
                let src_1: *const pixel = src_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_1 as core::ffi::c_uint) as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_1 as core::ffi::c_uint) as isize);
                let mut val_1: core::ffi::c_int = avg_1 as core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_1: core::ffi::c_int = avg_1 as core::ffi::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_1 as core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val_1 = iclip(
                        (combined_1 >> 6 as core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as core::ffi::c_int) << bitdepth_min_8),
                        0 as core::ffi::c_int,
                        bitdepth_max,
                    );
                }
                let noise_1: core::ffi::c_int = round2(
                    *scaling.offset(val_1 as isize) as core::ffi::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst_1 = iclip(
                    *src_1 as core::ffi::c_int + noise_1,
                    min_value,
                    max_value,
                ) as pixel;
                x_1 += 1;
            }
            let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_2 < xstart {
                let mut top: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                let mut old_1: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                top = round2(
                    old_1 * w[sx as usize][x_2 as usize][0 as core::ffi::c_int as usize]
                        + top
                            * w[sx
                                as usize][x_2 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2: core::ffi::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                old_1 = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    x_2,
                    y_0,
                ) as core::ffi::c_int;
                grain_2 = round2(
                    old_1 * w[sx as usize][x_2 as usize][0 as core::ffi::c_int as usize]
                        + grain_2
                            * w[sx
                                as usize][x_2 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[sy as usize][y_0 as usize][0 as core::ffi::c_int as usize]
                        + grain_2
                            * w[sy
                                as usize][y_0 as usize][1 as core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let lx_2: core::ffi::c_int = (bx.wrapping_add(x_2 as core::ffi::c_uint)
                    << sx) as core::ffi::c_int;
                let ly_2: core::ffi::c_int = y_0 << sy;
                let luma_2: *const pixel = luma_row
                    .offset((ly_2 as ptrdiff_t * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_2 as isize);
                let mut avg_2: pixel = *luma_2.offset(0 as core::ffi::c_int as isize);
                if sx != 0 {
                    avg_2 = (avg_2 as core::ffi::c_int
                        + *luma_2.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int + 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as pixel;
                }
                let src_2: *const pixel = src_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_2 as core::ffi::c_uint) as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as ptrdiff_t * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_2 as core::ffi::c_uint) as isize);
                let mut val_2: core::ffi::c_int = avg_2 as core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_2: core::ffi::c_int = avg_2 as core::ffi::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_2 as core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val_2 = iclip(
                        (combined_2 >> 6 as core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as core::ffi::c_int) << bitdepth_min_8),
                        0 as core::ffi::c_int,
                        bitdepth_max,
                    );
                }
                let noise_2: core::ffi::c_int = round2(
                    *scaling.offset(val_2 as isize) as core::ffi::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                ) as core::ffi::c_int;
                *dst_2 = iclip(
                    *src_2 as core::ffi::c_int + noise_2,
                    min_value,
                    max_value,
                ) as pixel;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add((FG_BLOCK_SIZE >> sx) as core::ffi::c_uint);
    }
}
unsafe extern "C" fn fguv_32x32xn_420_c(
    mut dst_row: *mut pixel,
    mut src_row: *const pixel,
    mut stride: ptrdiff_t,
    mut data: *const Dav1dFilmGrainData,
    mut pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    mut bh: core::ffi::c_int,
    mut row_num: core::ffi::c_int,
    mut luma_row: *const pixel,
    mut luma_stride: ptrdiff_t,
    mut uv_pl: core::ffi::c_int,
    mut is_id: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    fguv_32x32xn_c(
        dst_row,
        src_row,
        stride,
        data,
        pw,
        scaling,
        grain_lut,
        bh,
        row_num,
        luma_row,
        luma_stride,
        uv_pl,
        is_id,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn fguv_32x32xn_422_c(
    mut dst_row: *mut pixel,
    mut src_row: *const pixel,
    mut stride: ptrdiff_t,
    mut data: *const Dav1dFilmGrainData,
    mut pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    mut bh: core::ffi::c_int,
    mut row_num: core::ffi::c_int,
    mut luma_row: *const pixel,
    mut luma_stride: ptrdiff_t,
    mut uv_pl: core::ffi::c_int,
    mut is_id: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    fguv_32x32xn_c(
        dst_row,
        src_row,
        stride,
        data,
        pw,
        scaling,
        grain_lut,
        bh,
        row_num,
        luma_row,
        luma_stride,
        uv_pl,
        is_id,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn fguv_32x32xn_444_c(
    mut dst_row: *mut pixel,
    mut src_row: *const pixel,
    mut stride: ptrdiff_t,
    mut data: *const Dav1dFilmGrainData,
    mut pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    mut bh: core::ffi::c_int,
    mut row_num: core::ffi::c_int,
    mut luma_row: *const pixel,
    mut luma_stride: ptrdiff_t,
    mut uv_pl: core::ffi::c_int,
    mut is_id: core::ffi::c_int,
    bitdepth_max: core::ffi::c_int,
) {
    fguv_32x32xn_c(
        dst_row,
        src_row,
        stride,
        data,
        pw,
        scaling,
        grain_lut,
        bh,
        row_num,
        luma_row,
        luma_stride,
        uv_pl,
        is_id,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bitdepth_max,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_film_grain_dsp_init_16bpc(
    c: *mut Dav1dFilmGrainDSPContext,
) {
    (*c).generate_grain_y = Some(
        generate_grain_y_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const Dav1dFilmGrainData,
                core::ffi::c_int,
            ) -> (),
    ) as generate_grain_y_fn;
    (*c)
        .generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize] = Some(
        generate_grain_uv_420_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
                core::ffi::c_int,
            ) -> (),
    ) as generate_grain_uv_fn;
    (*c)
        .generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I422 as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize] = Some(
        generate_grain_uv_422_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
                core::ffi::c_int,
            ) -> (),
    ) as generate_grain_uv_fn;
    (*c)
        .generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize] = Some(
        generate_grain_uv_444_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
                core::ffi::c_int,
            ) -> (),
    ) as generate_grain_uv_fn;
    (*c).fgy_32x32xn = Some(
        fgy_32x32xn_c
            as unsafe extern "C" fn(
                *mut pixel,
                *const pixel,
                ptrdiff_t,
                *const Dav1dFilmGrainData,
                size_t,
                *const uint8_t,
                *const [entry; 82],
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as fgy_32x32xn_fn;
    (*c)
        .fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize] = Some(
        fguv_32x32xn_420_c
            as unsafe extern "C" fn(
                *mut pixel,
                *const pixel,
                ptrdiff_t,
                *const Dav1dFilmGrainData,
                size_t,
                *const uint8_t,
                *const [entry; 82],
                core::ffi::c_int,
                core::ffi::c_int,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as fguv_32x32xn_fn;
    (*c)
        .fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize] = Some(
        fguv_32x32xn_422_c
            as unsafe extern "C" fn(
                *mut pixel,
                *const pixel,
                ptrdiff_t,
                *const Dav1dFilmGrainData,
                size_t,
                *const uint8_t,
                *const [entry; 82],
                core::ffi::c_int,
                core::ffi::c_int,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as fguv_32x32xn_fn;
    (*c)
        .fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize] = Some(
        fguv_32x32xn_444_c
            as unsafe extern "C" fn(
                *mut pixel,
                *const pixel,
                ptrdiff_t,
                *const Dav1dFilmGrainData,
                size_t,
                *const uint8_t,
                *const [entry; 82],
                core::ffi::c_int,
                core::ffi::c_int,
                *const pixel,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as fguv_32x32xn_fn;
}
