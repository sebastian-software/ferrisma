/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/intrapred.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[inline]
unsafe extern "C-unwind" fn aom_memset16(
    mut dest: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
    mut length: size_t,
) -> *mut core::ffi::c_void {
    let mut i: size_t = 0;
    let mut dest16: *mut uint16_t = dest as *mut uint16_t;
    i = 0 as size_t;
    while i < length {
        let fresh0 = dest16;
        dest16 = dest16.offset(1);
        *fresh0 = val as uint16_t;
        i = i.wrapping_add(1);
    }
    return dest;
}
pub const SMOOTH_WEIGHT_LOG2_SCALE: core::ffi::c_int = 8 as core::ffi::c_int;
static mut smooth_weights: crate::platform::Aligned8<[uint8_t; 124]> = crate::platform::Aligned8([
    255 as core::ffi::c_int as uint8_t,
    149 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    197 as core::ffi::c_int as uint8_t,
    146 as core::ffi::c_int as uint8_t,
    105 as core::ffi::c_int as uint8_t,
    73 as core::ffi::c_int as uint8_t,
    50 as core::ffi::c_int as uint8_t,
    37 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    225 as core::ffi::c_int as uint8_t,
    196 as core::ffi::c_int as uint8_t,
    170 as core::ffi::c_int as uint8_t,
    145 as core::ffi::c_int as uint8_t,
    123 as core::ffi::c_int as uint8_t,
    102 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    68 as core::ffi::c_int as uint8_t,
    54 as core::ffi::c_int as uint8_t,
    43 as core::ffi::c_int as uint8_t,
    33 as core::ffi::c_int as uint8_t,
    26 as core::ffi::c_int as uint8_t,
    20 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    240 as core::ffi::c_int as uint8_t,
    225 as core::ffi::c_int as uint8_t,
    210 as core::ffi::c_int as uint8_t,
    196 as core::ffi::c_int as uint8_t,
    182 as core::ffi::c_int as uint8_t,
    169 as core::ffi::c_int as uint8_t,
    157 as core::ffi::c_int as uint8_t,
    145 as core::ffi::c_int as uint8_t,
    133 as core::ffi::c_int as uint8_t,
    122 as core::ffi::c_int as uint8_t,
    111 as core::ffi::c_int as uint8_t,
    101 as core::ffi::c_int as uint8_t,
    92 as core::ffi::c_int as uint8_t,
    83 as core::ffi::c_int as uint8_t,
    74 as core::ffi::c_int as uint8_t,
    66 as core::ffi::c_int as uint8_t,
    59 as core::ffi::c_int as uint8_t,
    52 as core::ffi::c_int as uint8_t,
    45 as core::ffi::c_int as uint8_t,
    39 as core::ffi::c_int as uint8_t,
    34 as core::ffi::c_int as uint8_t,
    29 as core::ffi::c_int as uint8_t,
    25 as core::ffi::c_int as uint8_t,
    21 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    14 as core::ffi::c_int as uint8_t,
    12 as core::ffi::c_int as uint8_t,
    10 as core::ffi::c_int as uint8_t,
    9 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    248 as core::ffi::c_int as uint8_t,
    240 as core::ffi::c_int as uint8_t,
    233 as core::ffi::c_int as uint8_t,
    225 as core::ffi::c_int as uint8_t,
    218 as core::ffi::c_int as uint8_t,
    210 as core::ffi::c_int as uint8_t,
    203 as core::ffi::c_int as uint8_t,
    196 as core::ffi::c_int as uint8_t,
    189 as core::ffi::c_int as uint8_t,
    182 as core::ffi::c_int as uint8_t,
    176 as core::ffi::c_int as uint8_t,
    169 as core::ffi::c_int as uint8_t,
    163 as core::ffi::c_int as uint8_t,
    156 as core::ffi::c_int as uint8_t,
    150 as core::ffi::c_int as uint8_t,
    144 as core::ffi::c_int as uint8_t,
    138 as core::ffi::c_int as uint8_t,
    133 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    121 as core::ffi::c_int as uint8_t,
    116 as core::ffi::c_int as uint8_t,
    111 as core::ffi::c_int as uint8_t,
    106 as core::ffi::c_int as uint8_t,
    101 as core::ffi::c_int as uint8_t,
    96 as core::ffi::c_int as uint8_t,
    91 as core::ffi::c_int as uint8_t,
    86 as core::ffi::c_int as uint8_t,
    82 as core::ffi::c_int as uint8_t,
    77 as core::ffi::c_int as uint8_t,
    73 as core::ffi::c_int as uint8_t,
    69 as core::ffi::c_int as uint8_t,
    65 as core::ffi::c_int as uint8_t,
    61 as core::ffi::c_int as uint8_t,
    57 as core::ffi::c_int as uint8_t,
    54 as core::ffi::c_int as uint8_t,
    50 as core::ffi::c_int as uint8_t,
    47 as core::ffi::c_int as uint8_t,
    44 as core::ffi::c_int as uint8_t,
    41 as core::ffi::c_int as uint8_t,
    38 as core::ffi::c_int as uint8_t,
    35 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    29 as core::ffi::c_int as uint8_t,
    27 as core::ffi::c_int as uint8_t,
    25 as core::ffi::c_int as uint8_t,
    22 as core::ffi::c_int as uint8_t,
    20 as core::ffi::c_int as uint8_t,
    18 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    15 as core::ffi::c_int as uint8_t,
    13 as core::ffi::c_int as uint8_t,
    12 as core::ffi::c_int as uint8_t,
    10 as core::ffi::c_int as uint8_t,
    9 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    7 as core::ffi::c_int as uint8_t,
    6 as core::ffi::c_int as uint8_t,
    6 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
]);
#[inline]
unsafe extern "C-unwind" fn v_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        memcpy(
            dst as *mut core::ffi::c_void,
            above as *const core::ffi::c_void,
            bw as size_t,
        );
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn h_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        memset(
            dst as *mut core::ffi::c_void,
            *left.offset(r as isize) as core::ffi::c_int,
            bw as size_t,
        );
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn abs_diff(
    mut a: core::ffi::c_int,
    mut b: core::ffi::c_int,
) -> core::ffi::c_int {
    return if a > b { a - b } else { b - a };
}
#[inline]
unsafe extern "C-unwind" fn paeth_predictor_single(
    mut left: uint16_t,
    mut top: uint16_t,
    mut top_left: uint16_t,
) -> uint16_t {
    let base: core::ffi::c_int = top as core::ffi::c_int + left as core::ffi::c_int
        - top_left as core::ffi::c_int;
    let p_left: core::ffi::c_int = abs_diff(base, left as core::ffi::c_int)
        as core::ffi::c_int;
    let p_top: core::ffi::c_int = abs_diff(base, top as core::ffi::c_int)
        as core::ffi::c_int;
    let p_top_left: core::ffi::c_int = abs_diff(base, top_left as core::ffi::c_int)
        as core::ffi::c_int;
    return (if p_left <= p_top && p_left <= p_top_left {
        left as core::ffi::c_int
    } else if p_top <= p_top_left {
        top as core::ffi::c_int
    } else {
        top_left as core::ffi::c_int
    }) as uint16_t;
}
#[inline]
unsafe extern "C-unwind" fn paeth_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let ytop_left: uint8_t = *above.offset(-(1 as core::ffi::c_int) as isize);
    r = 0 as core::ffi::c_int;
    while r < bh {
        c = 0 as core::ffi::c_int;
        while c < bw {
            *dst.offset(c as isize) = paeth_predictor_single(
                *left.offset(r as isize) as uint16_t,
                *above.offset(c as isize) as uint16_t,
                ytop_left as uint16_t,
            ) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn smooth_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let below_pred: uint8_t = *left.offset((bh - 1 as core::ffi::c_int) as isize);
    let right_pred: uint8_t = *above.offset((bw - 1 as core::ffi::c_int) as isize);
    let sm_weights_w: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bw as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let sm_weights_h: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bh as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let log2_scale: core::ffi::c_int = 1 as core::ffi::c_int + SMOOTH_WEIGHT_LOG2_SCALE;
    let scale: uint16_t = ((1 as core::ffi::c_int) << SMOOTH_WEIGHT_LOG2_SCALE)
        as uint16_t;
    if !((*sm_weights_w.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            95 as core::ffi::c_int,
            b"sm_weights_w[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*sm_weights_h.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            95 as core::ffi::c_int,
            b"sm_weights_h[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights_w.offset((bw - 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < scale as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            95 as core::ffi::c_int,
            b"scale - sm_weights_w[bw - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights_h.offset((bh - 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < scale as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            95 as core::ffi::c_int,
            b"scale - sm_weights_h[bh - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((log2_scale as usize).wrapping_add(::core::mem::size_of::<uint8_t>() as usize)
        < 31 as usize) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            95 as core::ffi::c_int,
            b"log2_scale + sizeof(*dst) < 31\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0;
        c = 0 as core::ffi::c_int;
        while c < bw {
            let pixels: [uint8_t; 4] = [
                *above.offset(c as isize),
                below_pred,
                *left.offset(r as isize),
                right_pred,
            ];
            let weights: [uint8_t; 4] = [
                *sm_weights_h.offset(r as isize),
                (scale as core::ffi::c_int
                    - *sm_weights_h.offset(r as isize) as core::ffi::c_int) as uint8_t,
                *sm_weights_w.offset(c as isize),
                (scale as core::ffi::c_int
                    - *sm_weights_w.offset(c as isize) as core::ffi::c_int) as uint8_t,
            ];
            let mut this_pred: uint32_t = 0 as uint32_t;
            let mut i: core::ffi::c_int = 0;
            if !(scale as core::ffi::c_int
                >= *sm_weights_h.offset(r as isize) as core::ffi::c_int
                && scale as core::ffi::c_int
                    >= *sm_weights_w.offset(c as isize) as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"smooth_predictor\0"))
                        .as_ptr(),
                    b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
                    105 as core::ffi::c_int,
                    b"scale >= sm_weights_h[r] && scale >= sm_weights_w[c]\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            i = 0 as core::ffi::c_int;
            while i < 4 as core::ffi::c_int {
                this_pred = this_pred
                    .wrapping_add(
                        (weights[i as usize] as core::ffi::c_int
                            * pixels[i as usize] as core::ffi::c_int) as uint32_t,
                    );
                i += 1;
            }
            *dst.offset(c as isize) = (this_pred
                .wrapping_add(
                    ((1 as core::ffi::c_int) << log2_scale - 1 as core::ffi::c_int)
                        as uint32_t,
                ) >> log2_scale) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn smooth_v_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let below_pred: uint8_t = *left.offset((bh - 1 as core::ffi::c_int) as isize);
    let sm_weights: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bh as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let log2_scale: core::ffi::c_int = SMOOTH_WEIGHT_LOG2_SCALE;
    let scale: uint16_t = ((1 as core::ffi::c_int) << SMOOTH_WEIGHT_LOG2_SCALE)
        as uint16_t;
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            124 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            124 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bw - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            124 as core::ffi::c_int,
            b"scale - sm_weights[bw - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bh - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            124 as core::ffi::c_int,
            b"scale - sm_weights[bh - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((log2_scale as usize).wrapping_add(::core::mem::size_of::<uint8_t>() as usize)
        < 31 as usize) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            124 as core::ffi::c_int,
            b"log2_scale + sizeof(*dst) < 31\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0;
        c = 0 as core::ffi::c_int;
        while c < bw {
            let pixels: [uint8_t; 2] = [*above.offset(c as isize), below_pred];
            let weights: [uint8_t; 2] = [
                *sm_weights.offset(r as isize),
                (scale as core::ffi::c_int
                    - *sm_weights.offset(r as isize) as core::ffi::c_int) as uint8_t,
            ];
            let mut this_pred: uint32_t = 0 as uint32_t;
            if !(scale as core::ffi::c_int
                >= *sm_weights.offset(r as isize) as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"smooth_v_predictor\0"))
                        .as_ptr(),
                    b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
                    133 as core::ffi::c_int,
                    b"scale >= sm_weights[r]\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut i: core::ffi::c_int = 0;
            i = 0 as core::ffi::c_int;
            while i < 2 as core::ffi::c_int {
                this_pred = this_pred
                    .wrapping_add(
                        (weights[i as usize] as core::ffi::c_int
                            * pixels[i as usize] as core::ffi::c_int) as uint32_t,
                    );
                i += 1;
            }
            *dst.offset(c as isize) = (this_pred
                .wrapping_add(
                    ((1 as core::ffi::c_int) << log2_scale - 1 as core::ffi::c_int)
                        as uint32_t,
                ) >> log2_scale) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn smooth_h_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let right_pred: uint8_t = *above.offset((bw - 1 as core::ffi::c_int) as isize);
    let sm_weights: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bw as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let log2_scale: core::ffi::c_int = SMOOTH_WEIGHT_LOG2_SCALE;
    let scale: uint16_t = ((1 as core::ffi::c_int) << SMOOTH_WEIGHT_LOG2_SCALE)
        as uint16_t;
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            153 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            153 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bw - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            153 as core::ffi::c_int,
            b"scale - sm_weights[bw - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bh - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            153 as core::ffi::c_int,
            b"scale - sm_weights[bh - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((log2_scale as usize).wrapping_add(::core::mem::size_of::<uint8_t>() as usize)
        < 31 as usize) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            153 as core::ffi::c_int,
            b"log2_scale + sizeof(*dst) < 31\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0;
        c = 0 as core::ffi::c_int;
        while c < bw {
            let pixels: [uint8_t; 2] = [*left.offset(r as isize), right_pred];
            let weights: [uint8_t; 2] = [
                *sm_weights.offset(c as isize),
                (scale as core::ffi::c_int
                    - *sm_weights.offset(c as isize) as core::ffi::c_int) as uint8_t,
            ];
            let mut this_pred: uint32_t = 0 as uint32_t;
            if !(scale as core::ffi::c_int
                >= *sm_weights.offset(c as isize) as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"smooth_h_predictor\0"))
                        .as_ptr(),
                    b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
                    162 as core::ffi::c_int,
                    b"scale >= sm_weights[c]\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut i: core::ffi::c_int = 0;
            i = 0 as core::ffi::c_int;
            while i < 2 as core::ffi::c_int {
                this_pred = this_pred
                    .wrapping_add(
                        (weights[i as usize] as core::ffi::c_int
                            * pixels[i as usize] as core::ffi::c_int) as uint32_t,
                    );
                i += 1;
            }
            *dst.offset(c as isize) = (this_pred
                .wrapping_add(
                    ((1 as core::ffi::c_int) << log2_scale - 1 as core::ffi::c_int)
                        as uint32_t,
                ) >> log2_scale) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn dc_128_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        memset(dst as *mut core::ffi::c_void, 128 as core::ffi::c_int, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn dc_left_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut expected_dc: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < bh {
        sum += *left.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    expected_dc = (sum + (bh >> 1 as core::ffi::c_int)) / bh;
    r = 0 as core::ffi::c_int;
    while r < bh {
        memset(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn dc_top_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut expected_dc: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < bw {
        sum += *above.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    expected_dc = (sum + (bw >> 1 as core::ffi::c_int)) / bw;
    r = 0 as core::ffi::c_int;
    while r < bh {
        memset(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn dc_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut expected_dc: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let count: core::ffi::c_int = bw + bh;
    i = 0 as core::ffi::c_int;
    while i < bw {
        sum += *above.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < bh {
        sum += *left.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    expected_dc = (sum + (count >> 1 as core::ffi::c_int)) / count;
    r = 0 as core::ffi::c_int;
    while r < bh {
        memset(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn divide_using_multiply_shift(
    mut num: core::ffi::c_int,
    mut shift1: core::ffi::c_int,
    mut multiplier: core::ffi::c_int,
    mut shift2: core::ffi::c_int,
) -> core::ffi::c_int {
    let interm: core::ffi::c_int = num >> shift1;
    return interm * multiplier >> shift2;
}
pub const DC_MULTIPLIER_1X2: core::ffi::c_int = 0x5556 as core::ffi::c_int;
pub const DC_MULTIPLIER_1X4: core::ffi::c_int = 0x3334 as core::ffi::c_int;
pub const DC_SHIFT2: core::ffi::c_int = 16 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn dc_predictor_rect(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
    mut shift1: core::ffi::c_int,
    mut multiplier: core::ffi::c_int,
) {
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < bw {
        sum += *above.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < bh {
        sum += *left.offset(i_0 as isize) as core::ffi::c_int;
        i_0 += 1;
    }
    let expected_dc: core::ffi::c_int = divide_using_multiply_shift(
        sum + (bw + bh >> 1 as core::ffi::c_int),
        shift1,
        multiplier,
        DC_SHIFT2,
    ) as core::ffi::c_int;
    if !(expected_dc < (1 as core::ffi::c_int) << 8 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"dc_predictor_rect\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            280 as core::ffi::c_int,
            b"expected_dc < (1 << 8)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < bh {
        memset(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        2 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        2 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        2 as core::ffi::c_int,
        DC_MULTIPLIER_1X4,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        2 as core::ffi::c_int,
        DC_MULTIPLIER_1X4,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        3 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        3 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        3 as core::ffi::c_int,
        DC_MULTIPLIER_1X4,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        3 as core::ffi::c_int,
        DC_MULTIPLIER_1X4,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        4 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        4 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        4 as core::ffi::c_int,
        DC_MULTIPLIER_1X4,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        4 as core::ffi::c_int,
        DC_MULTIPLIER_1X4,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        5 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor_rect(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        5 as core::ffi::c_int,
        DC_MULTIPLIER_1X2,
    );
}
#[inline]
unsafe extern "C-unwind" fn highbd_v_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        memcpy(
            dst as *mut core::ffi::c_void,
            above as *const core::ffi::c_void,
            (bw as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_h_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        aom_memset16(
            dst as *mut core::ffi::c_void,
            *left.offset(r as isize) as core::ffi::c_int,
            bw as size_t,
        );
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_paeth_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let ytop_left: uint16_t = *above.offset(-(1 as core::ffi::c_int) as isize);
    r = 0 as core::ffi::c_int;
    while r < bh {
        c = 0 as core::ffi::c_int;
        while c < bw {
            *dst.offset(c as isize) = paeth_predictor_single(
                *left.offset(r as isize),
                *above.offset(c as isize),
                ytop_left,
            );
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_smooth_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let below_pred: uint16_t = *left.offset((bh - 1 as core::ffi::c_int) as isize);
    let right_pred: uint16_t = *above.offset((bw - 1 as core::ffi::c_int) as isize);
    let sm_weights_w: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bw as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let sm_weights_h: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bh as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let log2_scale: core::ffi::c_int = 1 as core::ffi::c_int + SMOOTH_WEIGHT_LOG2_SCALE;
    let scale: uint16_t = ((1 as core::ffi::c_int) << SMOOTH_WEIGHT_LOG2_SCALE)
        as uint16_t;
    if !((*sm_weights_w.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"highbd_smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            422 as core::ffi::c_int,
            b"sm_weights_w[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*sm_weights_h.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"highbd_smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            422 as core::ffi::c_int,
            b"sm_weights_h[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights_w.offset((bw - 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < scale as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"highbd_smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            422 as core::ffi::c_int,
            b"scale - sm_weights_w[bw - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights_h.offset((bh - 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < scale as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"highbd_smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            422 as core::ffi::c_int,
            b"scale - sm_weights_h[bh - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((log2_scale as usize).wrapping_add(::core::mem::size_of::<uint16_t>() as usize)
        < 31 as usize) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"highbd_smooth_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            422 as core::ffi::c_int,
            b"log2_scale + sizeof(*dst) < 31\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0;
        c = 0 as core::ffi::c_int;
        while c < bw {
            let pixels: [uint16_t; 4] = [
                *above.offset(c as isize),
                below_pred,
                *left.offset(r as isize),
                right_pred,
            ];
            let weights: [uint8_t; 4] = [
                *sm_weights_h.offset(r as isize),
                (scale as core::ffi::c_int
                    - *sm_weights_h.offset(r as isize) as core::ffi::c_int) as uint8_t,
                *sm_weights_w.offset(c as isize),
                (scale as core::ffi::c_int
                    - *sm_weights_w.offset(c as isize) as core::ffi::c_int) as uint8_t,
            ];
            let mut this_pred: uint32_t = 0 as uint32_t;
            let mut i: core::ffi::c_int = 0;
            if !(scale as core::ffi::c_int
                >= *sm_weights_h.offset(r as isize) as core::ffi::c_int
                && scale as core::ffi::c_int
                    >= *sm_weights_w.offset(c as isize) as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"highbd_smooth_predictor\0"))
                        .as_ptr(),
                    b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
                    432 as core::ffi::c_int,
                    b"scale >= sm_weights_h[r] && scale >= sm_weights_w[c]\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            i = 0 as core::ffi::c_int;
            while i < 4 as core::ffi::c_int {
                this_pred = this_pred
                    .wrapping_add(
                        (weights[i as usize] as core::ffi::c_int
                            * pixels[i as usize] as core::ffi::c_int) as uint32_t,
                    );
                i += 1;
            }
            *dst.offset(c as isize) = (this_pred
                .wrapping_add(
                    ((1 as core::ffi::c_int) << log2_scale - 1 as core::ffi::c_int)
                        as uint32_t,
                ) >> log2_scale) as uint16_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_smooth_v_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let below_pred: uint16_t = *left.offset((bh - 1 as core::ffi::c_int) as isize);
    let sm_weights: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bh as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let log2_scale: core::ffi::c_int = SMOOTH_WEIGHT_LOG2_SCALE;
    let scale: uint16_t = ((1 as core::ffi::c_int) << SMOOTH_WEIGHT_LOG2_SCALE)
        as uint16_t;
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            453 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            453 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bw - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            453 as core::ffi::c_int,
            b"scale - sm_weights[bw - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bh - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            453 as core::ffi::c_int,
            b"scale - sm_weights[bh - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((log2_scale as usize).wrapping_add(::core::mem::size_of::<uint16_t>() as usize)
        < 31 as usize) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_v_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            453 as core::ffi::c_int,
            b"log2_scale + sizeof(*dst) < 31\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0;
        c = 0 as core::ffi::c_int;
        while c < bw {
            let pixels: [uint16_t; 2] = [*above.offset(c as isize), below_pred];
            let weights: [uint8_t; 2] = [
                *sm_weights.offset(r as isize),
                (scale as core::ffi::c_int
                    - *sm_weights.offset(r as isize) as core::ffi::c_int) as uint8_t,
            ];
            let mut this_pred: uint32_t = 0 as uint32_t;
            if !(scale as core::ffi::c_int
                >= *sm_weights.offset(r as isize) as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 26],
                        [core::ffi::c_char; 26],
                    >(*b"highbd_smooth_v_predictor\0"))
                        .as_ptr(),
                    b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
                    462 as core::ffi::c_int,
                    b"scale >= sm_weights[r]\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut i: core::ffi::c_int = 0;
            i = 0 as core::ffi::c_int;
            while i < 2 as core::ffi::c_int {
                this_pred = this_pred
                    .wrapping_add(
                        (weights[i as usize] as core::ffi::c_int
                            * pixels[i as usize] as core::ffi::c_int) as uint32_t,
                    );
                i += 1;
            }
            *dst.offset(c as isize) = (this_pred
                .wrapping_add(
                    ((1 as core::ffi::c_int) << log2_scale - 1 as core::ffi::c_int)
                        as uint32_t,
                ) >> log2_scale) as uint16_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_smooth_h_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let right_pred: uint16_t = *above.offset((bw - 1 as core::ffi::c_int) as isize);
    let sm_weights: *const uint8_t = core::ptr::addr_of!(smooth_weights.0).cast::<uint8_t>()
        .offset(bw as isize)
        .offset(-(4 as core::ffi::c_int as isize));
    let log2_scale: core::ffi::c_int = SMOOTH_WEIGHT_LOG2_SCALE;
    let scale: uint16_t = ((1 as core::ffi::c_int) << SMOOTH_WEIGHT_LOG2_SCALE)
        as uint16_t;
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            484 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*sm_weights.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            484 as core::ffi::c_int,
            b"sm_weights[0] < scale\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bw - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            484 as core::ffi::c_int,
            b"scale - sm_weights[bw - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((scale as core::ffi::c_int
        - *sm_weights.offset((bh - 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < scale as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            484 as core::ffi::c_int,
            b"scale - sm_weights[bh - 1] < scale\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((log2_scale as usize).wrapping_add(::core::mem::size_of::<uint16_t>() as usize)
        < 31 as usize) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"highbd_smooth_h_predictor\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            484 as core::ffi::c_int,
            b"log2_scale + sizeof(*dst) < 31\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0;
        c = 0 as core::ffi::c_int;
        while c < bw {
            let pixels: [uint16_t; 2] = [*left.offset(r as isize), right_pred];
            let weights: [uint8_t; 2] = [
                *sm_weights.offset(c as isize),
                (scale as core::ffi::c_int
                    - *sm_weights.offset(c as isize) as core::ffi::c_int) as uint8_t,
            ];
            let mut this_pred: uint32_t = 0 as uint32_t;
            if !(scale as core::ffi::c_int
                >= *sm_weights.offset(c as isize) as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 26],
                        [core::ffi::c_char; 26],
                    >(*b"highbd_smooth_h_predictor\0"))
                        .as_ptr(),
                    b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
                    493 as core::ffi::c_int,
                    b"scale >= sm_weights[c]\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut i: core::ffi::c_int = 0;
            i = 0 as core::ffi::c_int;
            while i < 2 as core::ffi::c_int {
                this_pred = this_pred
                    .wrapping_add(
                        (weights[i as usize] as core::ffi::c_int
                            * pixels[i as usize] as core::ffi::c_int) as uint32_t,
                    );
                i += 1;
            }
            *dst.offset(c as isize) = (this_pred
                .wrapping_add(
                    ((1 as core::ffi::c_int) << log2_scale - 1 as core::ffi::c_int)
                        as uint32_t,
                ) >> log2_scale) as uint16_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_dc_128_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < bh {
        aom_memset16(
            dst as *mut core::ffi::c_void,
            (128 as core::ffi::c_int) << bd - 8 as core::ffi::c_int,
            bw as size_t,
        );
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_dc_left_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut expected_dc: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < bh {
        sum += *left.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    expected_dc = (sum + (bh >> 1 as core::ffi::c_int)) / bh;
    r = 0 as core::ffi::c_int;
    while r < bh {
        aom_memset16(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_dc_top_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut expected_dc: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < bw {
        sum += *above.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    expected_dc = (sum + (bw >> 1 as core::ffi::c_int)) / bw;
    r = 0 as core::ffi::c_int;
    while r < bh {
        aom_memset16(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_dc_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut expected_dc: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let count: core::ffi::c_int = bw + bh;
    i = 0 as core::ffi::c_int;
    while i < bw {
        sum += *above.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < bh {
        sum += *left.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    expected_dc = (sum + (count >> 1 as core::ffi::c_int)) / count;
    r = 0 as core::ffi::c_int;
    while r < bh {
        aom_memset16(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
pub const HIGHBD_DC_MULTIPLIER_1X2: core::ffi::c_int = 0xaaab as core::ffi::c_int;
pub const HIGHBD_DC_MULTIPLIER_1X4: core::ffi::c_int = 0x6667 as core::ffi::c_int;
pub const HIGHBD_DC_SHIFT2: core::ffi::c_int = 17 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn highbd_dc_predictor_rect(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
    mut shift1: core::ffi::c_int,
    mut multiplier: uint32_t,
) {
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < bw {
        sum += *above.offset(i as isize) as core::ffi::c_int;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < bh {
        sum += *left.offset(i_0 as isize) as core::ffi::c_int;
        i_0 += 1;
    }
    let expected_dc: core::ffi::c_int = divide_using_multiply_shift(
        sum + (bw + bh >> 1 as core::ffi::c_int),
        shift1,
        multiplier as core::ffi::c_int,
        HIGHBD_DC_SHIFT2,
    ) as core::ffi::c_int;
    if !(expected_dc < (1 as core::ffi::c_int) << bd) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"highbd_dc_predictor_rect\0"))
                .as_ptr(),
            b"intrapred.c\0" as *const u8 as *const core::ffi::c_char,
            605 as core::ffi::c_int,
            b"expected_dc < (1 << bd)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < bh {
        aom_memset16(dst as *mut core::ffi::c_void, expected_dc, bw as size_t);
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
        2 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
        2 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
        2 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X4 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
        2 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X4 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
        3 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
        3 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
        3 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X4 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
        3 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X4 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
        4 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
        4 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
        4 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X4 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
        4 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X4 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
        5 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor_rect(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
        5 as core::ffi::c_int,
        HIGHBD_DC_MULTIPLIER_1X2 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 8 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 16 as core::ffi::c_int, 4 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 16 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 32 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 4 as core::ffi::c_int, 16 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 4 as core::ffi::c_int, 4 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 4 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 8 as core::ffi::c_int, 16 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 8 as core::ffi::c_int, 32 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_v_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    v_predictor(dst, stride, 8 as core::ffi::c_int, 4 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 16 as core::ffi::c_int, 4 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 8 as core::ffi::c_int, 16 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 8 as core::ffi::c_int, 32 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 8 as core::ffi::c_int, 4 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 8 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 16 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 32 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 4 as core::ffi::c_int, 16 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 4 as core::ffi::c_int, 4 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_h_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    h_predictor(dst, stride, 4 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_v_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_v_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_v_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_smooth_h_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_smooth_h_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_paeth_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_paeth_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    paeth_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_128_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_128_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_left_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_left_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_32x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_32x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_32x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_4x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_4x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_64x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_64x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_8x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_8x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_8x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_4x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_4x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_top_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor(dst, stride, 8 as core::ffi::c_int, 8 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_32x32_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_16x16_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor(
        dst,
        stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x64_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_64x64_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor(
        dst,
        stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor(dst, stride, 4 as core::ffi::c_int, 4 as core::ffi::c_int, above, left);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    dc_predictor(
        dst,
        stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        above,
        left,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_4x4_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor(
        dst,
        stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_8x8_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut bd: core::ffi::c_int,
) {
    highbd_dc_predictor(
        dst,
        stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        above,
        left,
        bd,
    );
}

extern "C" {
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
