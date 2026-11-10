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

// Mechanically translated from upstream/aom/aom_dsp/loopfilter.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[inline]
unsafe extern "C-unwind" fn signed_char_clamp(mut t: core::ffi::c_int) -> int8_t {
    return clamp(t, -(128 as core::ffi::c_int), 127 as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn signed_char_clamp_high(
    mut t: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> int16_t {
    match bd {
        10 => {
            return clamp(
                t,
                -(128 as core::ffi::c_int) * 4 as core::ffi::c_int,
                128 as core::ffi::c_int * 4 as core::ffi::c_int - 1 as core::ffi::c_int,
            ) as int16_t;
        }
        12 => {
            return clamp(
                t,
                -(128 as core::ffi::c_int) * 16 as core::ffi::c_int,
                128 as core::ffi::c_int * 16 as core::ffi::c_int - 1 as core::ffi::c_int,
            ) as int16_t;
        }
        8 | _ => {
            return clamp(
                t,
                -(128 as core::ffi::c_int),
                128 as core::ffi::c_int - 1 as core::ffi::c_int,
            ) as int16_t;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn filter_mask2(
    mut limit: uint8_t,
    mut blimit: uint8_t,
    mut p1: uint8_t,
    mut p0: uint8_t,
    mut q0: uint8_t,
    mut q1: uint8_t,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p0 as core::ffi::c_int - q0 as core::ffi::c_int) * 2 as core::ffi::c_int
            + abs(p1 as core::ffi::c_int - q1 as core::ffi::c_int)
                / 2 as core::ffi::c_int > blimit as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn filter_mask(
    mut limit: uint8_t,
    mut blimit: uint8_t,
    mut p3: uint8_t,
    mut p2: uint8_t,
    mut p1: uint8_t,
    mut p0: uint8_t,
    mut q0: uint8_t,
    mut q1: uint8_t,
    mut q2: uint8_t,
    mut q3: uint8_t,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p3 as core::ffi::c_int - p2 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p1 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q1 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q3 as core::ffi::c_int - q2 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p0 as core::ffi::c_int - q0 as core::ffi::c_int) * 2 as core::ffi::c_int
            + abs(p1 as core::ffi::c_int - q1 as core::ffi::c_int)
                / 2 as core::ffi::c_int > blimit as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn filter_mask3_chroma(
    mut limit: uint8_t,
    mut blimit: uint8_t,
    mut p2: uint8_t,
    mut p1: uint8_t,
    mut p0: uint8_t,
    mut q0: uint8_t,
    mut q1: uint8_t,
    mut q2: uint8_t,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p1 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q1 as core::ffi::c_int)
            > limit as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p0 as core::ffi::c_int - q0 as core::ffi::c_int) * 2 as core::ffi::c_int
            + abs(p1 as core::ffi::c_int - q1 as core::ffi::c_int)
                / 2 as core::ffi::c_int > blimit as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn flat_mask3_chroma(
    mut thresh: uint8_t,
    mut p2: uint8_t,
    mut p1: uint8_t,
    mut p0: uint8_t,
    mut q0: uint8_t,
    mut q1: uint8_t,
    mut q2: uint8_t,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn flat_mask4(
    mut thresh: uint8_t,
    mut p3: uint8_t,
    mut p2: uint8_t,
    mut p1: uint8_t,
    mut p0: uint8_t,
    mut q0: uint8_t,
    mut q1: uint8_t,
    mut q2: uint8_t,
    mut q3: uint8_t,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p3 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q3 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn hev_mask(
    mut thresh: uint8_t,
    mut p1: uint8_t,
    mut p0: uint8_t,
    mut q0: uint8_t,
    mut q1: uint8_t,
) -> int8_t {
    let mut hev: int8_t = 0 as int8_t;
    hev = (hev as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    hev = (hev as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh as core::ffi::c_int) as core::ffi::c_int * -(1 as core::ffi::c_int))
        as int8_t;
    return hev;
}
#[inline]
unsafe extern "C-unwind" fn filter4(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut op1: *mut uint8_t,
    mut op0: *mut uint8_t,
    mut oq0: *mut uint8_t,
    mut oq1: *mut uint8_t,
) {
    let mut filter1: int8_t = 0;
    let mut filter2: int8_t = 0;
    let ps1: int8_t = (*op1 as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as int8_t;
    let ps0: int8_t = (*op0 as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as int8_t;
    let qs0: int8_t = (*oq0 as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as int8_t;
    let qs1: int8_t = (*oq1 as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as int8_t;
    let hev: int8_t = hev_mask(thresh, *op1, *op0, *oq0, *oq1) as int8_t;
    let mut filter: int8_t = (signed_char_clamp(
        ps1 as core::ffi::c_int - qs1 as core::ffi::c_int,
    ) as core::ffi::c_int & hev as core::ffi::c_int) as int8_t;
    filter = (signed_char_clamp(
        filter as core::ffi::c_int
            + 3 as core::ffi::c_int * (qs0 as core::ffi::c_int - ps0 as core::ffi::c_int),
    ) as core::ffi::c_int & mask as core::ffi::c_int) as int8_t;
    filter1 = (signed_char_clamp(filter as core::ffi::c_int + 4 as core::ffi::c_int)
        as core::ffi::c_int >> 3 as core::ffi::c_int) as int8_t;
    filter2 = (signed_char_clamp(filter as core::ffi::c_int + 3 as core::ffi::c_int)
        as core::ffi::c_int >> 3 as core::ffi::c_int) as int8_t;
    *oq0 = (signed_char_clamp(qs0 as core::ffi::c_int - filter1 as core::ffi::c_int)
        as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as uint8_t;
    *op0 = (signed_char_clamp(ps0 as core::ffi::c_int + filter2 as core::ffi::c_int)
        as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as uint8_t;
    filter = (filter1 as core::ffi::c_int
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int & !(hev as core::ffi::c_int)) as int8_t;
    *oq1 = (signed_char_clamp(qs1 as core::ffi::c_int - filter as core::ffi::c_int)
        as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as uint8_t;
    *op1 = (signed_char_clamp(ps1 as core::ffi::c_int + filter as core::ffi::c_int)
        as core::ffi::c_int ^ 0x80 as core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p1: uint8_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint8_t = *s.offset(-p as isize);
        let q0: uint8_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint8_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let mask: int8_t = filter_mask2(*limit, *blimit, p1, p0, q0, q1) as int8_t;
        filter4(
            mask,
            *thresh,
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_dual_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    aom_lpf_horizontal_4_c(s, p, blimit0, limit0, thresh0);
    aom_lpf_horizontal_4_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_quad_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    aom_lpf_horizontal_4_c(s, p, blimit0, limit0, thresh0);
    aom_lpf_horizontal_4_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_horizontal_4_c(
        s.offset(8 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_horizontal_4_c(
        s.offset(12 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p1: uint8_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint8_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint8_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint8_t = *s.offset(1 as core::ffi::c_int as isize);
        let mask: int8_t = filter_mask2(*limit, *blimit, p1, p0, q0, q1) as int8_t;
        filter4(
            mask,
            *thresh,
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
        );
        s = s.offset(pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_dual_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    aom_lpf_vertical_4_c(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_4_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_quad_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    aom_lpf_vertical_4_c(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_4_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_vertical_4_c(
        s.offset((8 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_vertical_4_c(
        s.offset((12 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
}
#[inline]
unsafe extern "C-unwind" fn filter6(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut flat: int8_t,
    mut op2: *mut uint8_t,
    mut op1: *mut uint8_t,
    mut op0: *mut uint8_t,
    mut oq0: *mut uint8_t,
    mut oq1: *mut uint8_t,
    mut oq2: *mut uint8_t,
) {
    if flat as core::ffi::c_int != 0 && mask as core::ffi::c_int != 0 {
        let p2: uint8_t = *op2;
        let p1: uint8_t = *op1;
        let p0: uint8_t = *op0;
        let q0: uint8_t = *oq0;
        let q1: uint8_t = *oq1;
        let q2: uint8_t = *oq2;
        *op1 = (p2 as core::ffi::c_int * 3 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int + q0 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *op0 = (p2 as core::ffi::c_int + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int + q1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *oq0 = (p1 as core::ffi::c_int + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int + q2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *oq1 = (p0 as core::ffi::c_int + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int
            + q2 as core::ffi::c_int * 3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
    } else {
        filter4(mask, thresh, op1, op0, oq0, oq1);
    };
}
#[inline]
unsafe extern "C-unwind" fn filter8(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut flat: int8_t,
    mut op3: *mut uint8_t,
    mut op2: *mut uint8_t,
    mut op1: *mut uint8_t,
    mut op0: *mut uint8_t,
    mut oq0: *mut uint8_t,
    mut oq1: *mut uint8_t,
    mut oq2: *mut uint8_t,
    mut oq3: *mut uint8_t,
) {
    if flat as core::ffi::c_int != 0 && mask as core::ffi::c_int != 0 {
        let p3: uint8_t = *op3;
        let p2: uint8_t = *op2;
        let p1: uint8_t = *op1;
        let p0: uint8_t = *op0;
        let q0: uint8_t = *oq0;
        let q1: uint8_t = *oq1;
        let q2: uint8_t = *oq2;
        let q3: uint8_t = *oq3;
        *op2 = (p3 as core::ffi::c_int + p3 as core::ffi::c_int + p3 as core::ffi::c_int
            + 2 as core::ffi::c_int * p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *op1 = (p3 as core::ffi::c_int + p3 as core::ffi::c_int + p2 as core::ffi::c_int
            + 2 as core::ffi::c_int * p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *op0 = (p3 as core::ffi::c_int + p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + 2 as core::ffi::c_int * p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *oq0 = (p2 as core::ffi::c_int + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + 2 as core::ffi::c_int * q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *oq1 = (p1 as core::ffi::c_int + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + 2 as core::ffi::c_int * q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
        *oq2 = (p0 as core::ffi::c_int + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + 2 as core::ffi::c_int * q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q3 as core::ffi::c_int + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint8_t;
    } else {
        filter4(mask, thresh, op1, op0, oq0, oq1);
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p2: uint8_t = *s.offset((-(3 as core::ffi::c_int) * p) as isize);
        let p1: uint8_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint8_t = *s.offset(-p as isize);
        let q0: uint8_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint8_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let q2: uint8_t = *s.offset((2 as core::ffi::c_int * p) as isize);
        let mask: int8_t = filter_mask3_chroma(*limit, *blimit, p2, p1, p0, q0, q1, q2)
            as int8_t;
        let flat: int8_t = flat_mask3_chroma(1 as uint8_t, p2, p1, p0, q0, q1, q2)
            as int8_t;
        filter6(
            mask,
            *thresh,
            flat,
            s.offset(-((3 as core::ffi::c_int * p) as isize)),
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
            s.offset((2 as core::ffi::c_int * p) as isize),
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_dual_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    aom_lpf_horizontal_6_c(s, p, blimit0, limit0, thresh0);
    aom_lpf_horizontal_6_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_quad_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    aom_lpf_horizontal_6_c(s, p, blimit0, limit0, thresh0);
    aom_lpf_horizontal_6_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_horizontal_6_c(
        s.offset(8 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_horizontal_6_c(
        s.offset(12 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p3: uint8_t = *s.offset((-(4 as core::ffi::c_int) * p) as isize);
        let p2: uint8_t = *s.offset((-(3 as core::ffi::c_int) * p) as isize);
        let p1: uint8_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint8_t = *s.offset(-p as isize);
        let q0: uint8_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint8_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let q2: uint8_t = *s.offset((2 as core::ffi::c_int * p) as isize);
        let q3: uint8_t = *s.offset((3 as core::ffi::c_int * p) as isize);
        let mask: int8_t = filter_mask(*limit, *blimit, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        let flat: int8_t = flat_mask4(1 as uint8_t, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        filter8(
            mask,
            *thresh,
            flat,
            s.offset(-((4 as core::ffi::c_int * p) as isize)),
            s.offset(-((3 as core::ffi::c_int * p) as isize)),
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
            s.offset((2 as core::ffi::c_int * p) as isize),
            s.offset((3 as core::ffi::c_int * p) as isize),
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_dual_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    aom_lpf_horizontal_8_c(s, p, blimit0, limit0, thresh0);
    aom_lpf_horizontal_8_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_quad_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    aom_lpf_horizontal_8_c(s, p, blimit0, limit0, thresh0);
    aom_lpf_horizontal_8_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_horizontal_8_c(
        s.offset(8 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_horizontal_8_c(
        s.offset(12 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p2: uint8_t = *s.offset(-(3 as core::ffi::c_int) as isize);
        let p1: uint8_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint8_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint8_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint8_t = *s.offset(1 as core::ffi::c_int as isize);
        let q2: uint8_t = *s.offset(2 as core::ffi::c_int as isize);
        let mask: int8_t = filter_mask3_chroma(*limit, *blimit, p2, p1, p0, q0, q1, q2)
            as int8_t;
        let flat: int8_t = flat_mask3_chroma(1 as uint8_t, p2, p1, p0, q0, q1, q2)
            as int8_t;
        filter6(
            mask,
            *thresh,
            flat,
            s.offset(-(3 as core::ffi::c_int as isize)),
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
            s.offset(2 as core::ffi::c_int as isize),
        );
        s = s.offset(pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_dual_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    aom_lpf_vertical_6_c(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_6_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_quad_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    aom_lpf_vertical_6_c(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_6_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_vertical_6_c(
        s.offset((8 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_vertical_6_c(
        s.offset((12 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p3: uint8_t = *s.offset(-(4 as core::ffi::c_int) as isize);
        let p2: uint8_t = *s.offset(-(3 as core::ffi::c_int) as isize);
        let p1: uint8_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint8_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint8_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint8_t = *s.offset(1 as core::ffi::c_int as isize);
        let q2: uint8_t = *s.offset(2 as core::ffi::c_int as isize);
        let q3: uint8_t = *s.offset(3 as core::ffi::c_int as isize);
        let mask: int8_t = filter_mask(*limit, *blimit, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        let flat: int8_t = flat_mask4(1 as uint8_t, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        filter8(
            mask,
            *thresh,
            flat,
            s.offset(-(4 as core::ffi::c_int as isize)),
            s.offset(-(3 as core::ffi::c_int as isize)),
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
            s.offset(2 as core::ffi::c_int as isize),
            s.offset(3 as core::ffi::c_int as isize),
        );
        s = s.offset(pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_dual_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    aom_lpf_vertical_8_c(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_8_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_quad_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    aom_lpf_vertical_8_c(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_8_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_vertical_8_c(
        s.offset((8 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
    aom_lpf_vertical_8_c(
        s.offset((12 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
    );
}
#[inline]
unsafe extern "C-unwind" fn filter14(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut flat: int8_t,
    mut flat2: int8_t,
    mut op6: *mut uint8_t,
    mut op5: *mut uint8_t,
    mut op4: *mut uint8_t,
    mut op3: *mut uint8_t,
    mut op2: *mut uint8_t,
    mut op1: *mut uint8_t,
    mut op0: *mut uint8_t,
    mut oq0: *mut uint8_t,
    mut oq1: *mut uint8_t,
    mut oq2: *mut uint8_t,
    mut oq3: *mut uint8_t,
    mut oq4: *mut uint8_t,
    mut oq5: *mut uint8_t,
    mut oq6: *mut uint8_t,
) {
    if flat2 as core::ffi::c_int != 0 && flat as core::ffi::c_int != 0
        && mask as core::ffi::c_int != 0
    {
        let p6: uint8_t = *op6;
        let p5: uint8_t = *op5;
        let p4: uint8_t = *op4;
        let p3: uint8_t = *op3;
        let p2: uint8_t = *op2;
        let p1: uint8_t = *op1;
        let p0: uint8_t = *op0;
        let q0: uint8_t = *oq0;
        let q1: uint8_t = *oq1;
        let q2: uint8_t = *oq2;
        let q3: uint8_t = *oq3;
        let q4: uint8_t = *oq4;
        let q5: uint8_t = *oq5;
        let q6: uint8_t = *oq6;
        *op5 = (p6 as core::ffi::c_int * 7 as core::ffi::c_int
            + p5 as core::ffi::c_int * 2 as core::ffi::c_int
            + p4 as core::ffi::c_int * 2 as core::ffi::c_int + p3 as core::ffi::c_int
            + p2 as core::ffi::c_int + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *op4 = (p6 as core::ffi::c_int * 5 as core::ffi::c_int
            + p5 as core::ffi::c_int * 2 as core::ffi::c_int
            + p4 as core::ffi::c_int * 2 as core::ffi::c_int
            + p3 as core::ffi::c_int * 2 as core::ffi::c_int + p2 as core::ffi::c_int
            + p1 as core::ffi::c_int + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *op3 = (p6 as core::ffi::c_int * 4 as core::ffi::c_int + p5 as core::ffi::c_int
            + p4 as core::ffi::c_int * 2 as core::ffi::c_int
            + p3 as core::ffi::c_int * 2 as core::ffi::c_int
            + p2 as core::ffi::c_int * 2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *op2 = (p6 as core::ffi::c_int * 3 as core::ffi::c_int + p5 as core::ffi::c_int
            + p4 as core::ffi::c_int + p3 as core::ffi::c_int * 2 as core::ffi::c_int
            + p2 as core::ffi::c_int * 2 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int + q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *op1 = (p6 as core::ffi::c_int * 2 as core::ffi::c_int + p5 as core::ffi::c_int
            + p4 as core::ffi::c_int + p3 as core::ffi::c_int
            + p2 as core::ffi::c_int * 2 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int + q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q4 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *op0 = (p6 as core::ffi::c_int + p5 as core::ffi::c_int + p4 as core::ffi::c_int
            + p3 as core::ffi::c_int + p2 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int + q3 as core::ffi::c_int + q4 as core::ffi::c_int
            + q5 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *oq0 = (p5 as core::ffi::c_int + p4 as core::ffi::c_int + p3 as core::ffi::c_int
            + p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int + q4 as core::ffi::c_int + q5 as core::ffi::c_int
            + q6 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *oq1 = (p4 as core::ffi::c_int + p3 as core::ffi::c_int + p2 as core::ffi::c_int
            + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int
            + q2 as core::ffi::c_int * 2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q4 as core::ffi::c_int + q5 as core::ffi::c_int
            + q6 as core::ffi::c_int * 2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *oq2 = (p3 as core::ffi::c_int + p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int
            + q2 as core::ffi::c_int * 2 as core::ffi::c_int
            + q3 as core::ffi::c_int * 2 as core::ffi::c_int + q4 as core::ffi::c_int
            + q5 as core::ffi::c_int + q6 as core::ffi::c_int * 3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *oq3 = (p2 as core::ffi::c_int + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int * 2 as core::ffi::c_int
            + q3 as core::ffi::c_int * 2 as core::ffi::c_int
            + q4 as core::ffi::c_int * 2 as core::ffi::c_int + q5 as core::ffi::c_int
            + q6 as core::ffi::c_int * 4 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *oq4 = (p1 as core::ffi::c_int + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int * 2 as core::ffi::c_int
            + q4 as core::ffi::c_int * 2 as core::ffi::c_int
            + q5 as core::ffi::c_int * 2 as core::ffi::c_int
            + q6 as core::ffi::c_int * 5 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
        *oq5 = (p0 as core::ffi::c_int + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q4 as core::ffi::c_int * 2 as core::ffi::c_int
            + q5 as core::ffi::c_int * 2 as core::ffi::c_int
            + q6 as core::ffi::c_int * 7 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint8_t;
    } else {
        filter8(mask, thresh, flat, op3, op2, op1, op0, oq0, oq1, oq2, oq3);
    };
}
unsafe extern "C-unwind" fn mb_lpf_horizontal_edge_w(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut count: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut step: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < step * count {
        let p6: uint8_t = *s.offset((-(7 as core::ffi::c_int) * p) as isize);
        let p5: uint8_t = *s.offset((-(6 as core::ffi::c_int) * p) as isize);
        let p4: uint8_t = *s.offset((-(5 as core::ffi::c_int) * p) as isize);
        let p3: uint8_t = *s.offset((-(4 as core::ffi::c_int) * p) as isize);
        let p2: uint8_t = *s.offset((-(3 as core::ffi::c_int) * p) as isize);
        let p1: uint8_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint8_t = *s.offset(-p as isize);
        let q0: uint8_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint8_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let q2: uint8_t = *s.offset((2 as core::ffi::c_int * p) as isize);
        let q3: uint8_t = *s.offset((3 as core::ffi::c_int * p) as isize);
        let q4: uint8_t = *s.offset((4 as core::ffi::c_int * p) as isize);
        let q5: uint8_t = *s.offset((5 as core::ffi::c_int * p) as isize);
        let q6: uint8_t = *s.offset((6 as core::ffi::c_int * p) as isize);
        let mask: int8_t = filter_mask(*limit, *blimit, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        let flat: int8_t = flat_mask4(1 as uint8_t, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        let flat2: int8_t = flat_mask4(1 as uint8_t, p6, p5, p4, p0, q0, q4, q5, q6)
            as int8_t;
        filter14(
            mask,
            *thresh,
            flat,
            flat2,
            s.offset(-((7 as core::ffi::c_int * p) as isize)),
            s.offset(-((6 as core::ffi::c_int * p) as isize)),
            s.offset(-((5 as core::ffi::c_int * p) as isize)),
            s.offset(-((4 as core::ffi::c_int * p) as isize)),
            s.offset(-((3 as core::ffi::c_int * p) as isize)),
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
            s.offset((2 as core::ffi::c_int * p) as isize),
            s.offset((3 as core::ffi::c_int * p) as isize),
            s.offset((4 as core::ffi::c_int * p) as isize),
            s.offset((5 as core::ffi::c_int * p) as isize),
            s.offset((6 as core::ffi::c_int * p) as isize),
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    mb_lpf_horizontal_edge_w(s, p, blimit, limit, thresh, 1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_dual_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    mb_lpf_horizontal_edge_w(s, p, blimit0, limit0, thresh0, 1 as core::ffi::c_int);
    mb_lpf_horizontal_edge_w(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_quad_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    mb_lpf_horizontal_edge_w(s, p, blimit0, limit0, thresh0, 1 as core::ffi::c_int);
    mb_lpf_horizontal_edge_w(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
        1 as core::ffi::c_int,
    );
    mb_lpf_horizontal_edge_w(
        s.offset(8 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
        1 as core::ffi::c_int,
    );
    mb_lpf_horizontal_edge_w(
        s.offset(12 as core::ffi::c_int as isize),
        p,
        blimit0,
        limit0,
        thresh0,
        1 as core::ffi::c_int,
    );
}
unsafe extern "C-unwind" fn mb_lpf_vertical_edge_w(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut count: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p6: uint8_t = *s.offset(-(7 as core::ffi::c_int) as isize);
        let p5: uint8_t = *s.offset(-(6 as core::ffi::c_int) as isize);
        let p4: uint8_t = *s.offset(-(5 as core::ffi::c_int) as isize);
        let p3: uint8_t = *s.offset(-(4 as core::ffi::c_int) as isize);
        let p2: uint8_t = *s.offset(-(3 as core::ffi::c_int) as isize);
        let p1: uint8_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint8_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint8_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint8_t = *s.offset(1 as core::ffi::c_int as isize);
        let q2: uint8_t = *s.offset(2 as core::ffi::c_int as isize);
        let q3: uint8_t = *s.offset(3 as core::ffi::c_int as isize);
        let q4: uint8_t = *s.offset(4 as core::ffi::c_int as isize);
        let q5: uint8_t = *s.offset(5 as core::ffi::c_int as isize);
        let q6: uint8_t = *s.offset(6 as core::ffi::c_int as isize);
        let mask: int8_t = filter_mask(*limit, *blimit, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        let flat: int8_t = flat_mask4(1 as uint8_t, p3, p2, p1, p0, q0, q1, q2, q3)
            as int8_t;
        let flat2: int8_t = flat_mask4(1 as uint8_t, p6, p5, p4, p0, q0, q4, q5, q6)
            as int8_t;
        filter14(
            mask,
            *thresh,
            flat,
            flat2,
            s.offset(-(7 as core::ffi::c_int as isize)),
            s.offset(-(6 as core::ffi::c_int as isize)),
            s.offset(-(5 as core::ffi::c_int as isize)),
            s.offset(-(4 as core::ffi::c_int as isize)),
            s.offset(-(3 as core::ffi::c_int as isize)),
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
            s.offset(2 as core::ffi::c_int as isize),
            s.offset(3 as core::ffi::c_int as isize),
            s.offset(4 as core::ffi::c_int as isize),
            s.offset(5 as core::ffi::c_int as isize),
            s.offset(6 as core::ffi::c_int as isize),
        );
        s = s.offset(p as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_c(
    mut s: *mut uint8_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
) {
    mb_lpf_vertical_edge_w(s, p, blimit, limit, thresh, 4 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_dual_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
) {
    mb_lpf_vertical_edge_w(s, pitch, blimit0, limit0, thresh0, 4 as core::ffi::c_int);
    mb_lpf_vertical_edge_w(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_quad_c(
    mut s: *mut uint8_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
) {
    mb_lpf_vertical_edge_w(s, pitch, blimit0, limit0, thresh0, 4 as core::ffi::c_int);
    mb_lpf_vertical_edge_w(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
        4 as core::ffi::c_int,
    );
    mb_lpf_vertical_edge_w(
        s.offset((8 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
        4 as core::ffi::c_int,
    );
    mb_lpf_vertical_edge_w(
        s.offset((12 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit0,
        limit0,
        thresh0,
        4 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn highbd_filter_mask2(
    mut limit: uint8_t,
    mut blimit: uint8_t,
    mut p1: uint16_t,
    mut p0: uint16_t,
    mut q0: uint16_t,
    mut q1: uint16_t,
    mut bd: core::ffi::c_int,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    let mut limit16: int16_t = ((limit as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    let mut blimit16: int16_t = ((blimit as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p0 as core::ffi::c_int - q0 as core::ffi::c_int) * 2 as core::ffi::c_int
            + abs(p1 as core::ffi::c_int - q1 as core::ffi::c_int)
                / 2 as core::ffi::c_int > blimit16 as core::ffi::c_int)
            as core::ffi::c_int * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn highbd_filter_mask(
    mut limit: uint8_t,
    mut blimit: uint8_t,
    mut p3: uint16_t,
    mut p2: uint16_t,
    mut p1: uint16_t,
    mut p0: uint16_t,
    mut q0: uint16_t,
    mut q1: uint16_t,
    mut q2: uint16_t,
    mut q3: uint16_t,
    mut bd: core::ffi::c_int,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    let mut limit16: int16_t = ((limit as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    let mut blimit16: int16_t = ((blimit as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    mask = (mask as core::ffi::c_int
        | (abs(p3 as core::ffi::c_int - p2 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p1 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q1 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q3 as core::ffi::c_int - q2 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p0 as core::ffi::c_int - q0 as core::ffi::c_int) * 2 as core::ffi::c_int
            + abs(p1 as core::ffi::c_int - q1 as core::ffi::c_int)
                / 2 as core::ffi::c_int > blimit16 as core::ffi::c_int)
            as core::ffi::c_int * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn highbd_filter_mask3_chroma(
    mut limit: uint8_t,
    mut blimit: uint8_t,
    mut p2: uint16_t,
    mut p1: uint16_t,
    mut p0: uint16_t,
    mut q0: uint16_t,
    mut q1: uint16_t,
    mut q2: uint16_t,
    mut bd: core::ffi::c_int,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    let mut limit16: int16_t = ((limit as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    let mut blimit16: int16_t = ((blimit as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p1 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q1 as core::ffi::c_int)
            > limit16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p0 as core::ffi::c_int - q0 as core::ffi::c_int) * 2 as core::ffi::c_int
            + abs(p1 as core::ffi::c_int - q1 as core::ffi::c_int)
                / 2 as core::ffi::c_int > blimit16 as core::ffi::c_int)
            as core::ffi::c_int * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn highbd_flat_mask3_chroma(
    mut thresh: uint8_t,
    mut p2: uint16_t,
    mut p1: uint16_t,
    mut p0: uint16_t,
    mut q0: uint16_t,
    mut q1: uint16_t,
    mut q2: uint16_t,
    mut bd: core::ffi::c_int,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    let mut thresh16: int16_t = ((thresh as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn highbd_flat_mask4(
    mut thresh: uint8_t,
    mut p3: uint16_t,
    mut p2: uint16_t,
    mut p1: uint16_t,
    mut p0: uint16_t,
    mut q0: uint16_t,
    mut q1: uint16_t,
    mut q2: uint16_t,
    mut q3: uint16_t,
    mut bd: core::ffi::c_int,
) -> int8_t {
    let mut mask: int8_t = 0 as int8_t;
    let mut thresh16: int16_t = ((thresh as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    mask = (mask as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p2 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q2 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(p3 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    mask = (mask as core::ffi::c_int
        | (abs(q3 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int8_t;
    return !(mask as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn highbd_hev_mask(
    mut thresh: uint8_t,
    mut p1: uint16_t,
    mut p0: uint16_t,
    mut q0: uint16_t,
    mut q1: uint16_t,
    mut bd: core::ffi::c_int,
) -> int16_t {
    let mut hev: int16_t = 0 as int16_t;
    let mut thresh16: int16_t = ((thresh as uint16_t as core::ffi::c_int)
        << bd - 8 as core::ffi::c_int) as int16_t;
    hev = (hev as core::ffi::c_int
        | (abs(p1 as core::ffi::c_int - p0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int16_t;
    hev = (hev as core::ffi::c_int
        | (abs(q1 as core::ffi::c_int - q0 as core::ffi::c_int)
            > thresh16 as core::ffi::c_int) as core::ffi::c_int
            * -(1 as core::ffi::c_int)) as int16_t;
    return hev;
}
#[inline]
unsafe extern "C-unwind" fn highbd_filter4(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut op1: *mut uint16_t,
    mut op0: *mut uint16_t,
    mut oq0: *mut uint16_t,
    mut oq1: *mut uint16_t,
    mut bd: core::ffi::c_int,
) {
    let mut filter1: int16_t = 0;
    let mut filter2: int16_t = 0;
    let mut shift: core::ffi::c_int = bd - 8 as core::ffi::c_int;
    let ps1: int16_t = (*op1 as int16_t as core::ffi::c_int
        - ((0x80 as core::ffi::c_int) << shift)) as int16_t;
    let ps0: int16_t = (*op0 as int16_t as core::ffi::c_int
        - ((0x80 as core::ffi::c_int) << shift)) as int16_t;
    let qs0: int16_t = (*oq0 as int16_t as core::ffi::c_int
        - ((0x80 as core::ffi::c_int) << shift)) as int16_t;
    let qs1: int16_t = (*oq1 as int16_t as core::ffi::c_int
        - ((0x80 as core::ffi::c_int) << shift)) as int16_t;
    let hev: int16_t = highbd_hev_mask(thresh, *op1, *op0, *oq0, *oq1, bd) as int16_t;
    let mut filter: int16_t = (signed_char_clamp_high(
        ps1 as core::ffi::c_int - qs1 as core::ffi::c_int,
        bd,
    ) as core::ffi::c_int & hev as core::ffi::c_int) as int16_t;
    filter = (signed_char_clamp_high(
        filter as core::ffi::c_int
            + 3 as core::ffi::c_int
                * (qs0 as core::ffi::c_int - ps0 as core::ffi::c_int),
        bd,
    ) as core::ffi::c_int & mask as core::ffi::c_int) as int16_t;
    filter1 = (signed_char_clamp_high(
        filter as core::ffi::c_int + 4 as core::ffi::c_int,
        bd,
    ) as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t;
    filter2 = (signed_char_clamp_high(
        filter as core::ffi::c_int + 3 as core::ffi::c_int,
        bd,
    ) as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t;
    *oq0 = (signed_char_clamp_high(
        qs0 as core::ffi::c_int - filter1 as core::ffi::c_int,
        bd,
    ) as core::ffi::c_int + ((0x80 as core::ffi::c_int) << shift)) as uint16_t;
    *op0 = (signed_char_clamp_high(
        ps0 as core::ffi::c_int + filter2 as core::ffi::c_int,
        bd,
    ) as core::ffi::c_int + ((0x80 as core::ffi::c_int) << shift)) as uint16_t;
    filter = (filter1 as core::ffi::c_int
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int & !(hev as core::ffi::c_int)) as int16_t;
    *oq1 = (signed_char_clamp_high(
        qs1 as core::ffi::c_int - filter as core::ffi::c_int,
        bd,
    ) as core::ffi::c_int + ((0x80 as core::ffi::c_int) << shift)) as uint16_t;
    *op1 = (signed_char_clamp_high(
        ps1 as core::ffi::c_int + filter as core::ffi::c_int,
        bd,
    ) as core::ffi::c_int + ((0x80 as core::ffi::c_int) << shift)) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_4_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p1: uint16_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint16_t = *s.offset(-p as isize);
        let q0: uint16_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint16_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let mask: int8_t = highbd_filter_mask2(*limit, *blimit, p1, p0, q0, q1, bd)
            as int8_t;
        highbd_filter4(
            mask,
            *thresh,
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
            bd,
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_4_dual_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    aom_highbd_lpf_horizontal_4_c(s, p, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_horizontal_4_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_4_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p1: uint16_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint16_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint16_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint16_t = *s.offset(1 as core::ffi::c_int as isize);
        let mask: int8_t = highbd_filter_mask2(*limit, *blimit, p1, p0, q0, q1, bd)
            as int8_t;
        highbd_filter4(
            mask,
            *thresh,
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
            bd,
        );
        s = s.offset(pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_4_dual_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    aom_highbd_lpf_vertical_4_c(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_vertical_4_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
}
#[inline]
unsafe extern "C-unwind" fn highbd_filter6(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut flat: int8_t,
    mut op2: *mut uint16_t,
    mut op1: *mut uint16_t,
    mut op0: *mut uint16_t,
    mut oq0: *mut uint16_t,
    mut oq1: *mut uint16_t,
    mut oq2: *mut uint16_t,
    mut bd: core::ffi::c_int,
) {
    if flat as core::ffi::c_int != 0 && mask as core::ffi::c_int != 0 {
        let p2: uint16_t = *op2;
        let p1: uint16_t = *op1;
        let p0: uint16_t = *op0;
        let q0: uint16_t = *oq0;
        let q1: uint16_t = *oq1;
        let q2: uint16_t = *oq2;
        *op1 = (p2 as core::ffi::c_int * 3 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int + q0 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *op0 = (p2 as core::ffi::c_int + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int + q1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *oq0 = (p1 as core::ffi::c_int + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int + q2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *oq1 = (p0 as core::ffi::c_int + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int
            + q2 as core::ffi::c_int * 3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
    } else {
        highbd_filter4(mask, thresh, op1, op0, oq0, oq1, bd);
    };
}
#[inline]
unsafe extern "C-unwind" fn highbd_filter8(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut flat: int8_t,
    mut op3: *mut uint16_t,
    mut op2: *mut uint16_t,
    mut op1: *mut uint16_t,
    mut op0: *mut uint16_t,
    mut oq0: *mut uint16_t,
    mut oq1: *mut uint16_t,
    mut oq2: *mut uint16_t,
    mut oq3: *mut uint16_t,
    mut bd: core::ffi::c_int,
) {
    if flat as core::ffi::c_int != 0 && mask as core::ffi::c_int != 0 {
        let p3: uint16_t = *op3;
        let p2: uint16_t = *op2;
        let p1: uint16_t = *op1;
        let p0: uint16_t = *op0;
        let q0: uint16_t = *oq0;
        let q1: uint16_t = *oq1;
        let q2: uint16_t = *oq2;
        let q3: uint16_t = *oq3;
        *op2 = (p3 as core::ffi::c_int + p3 as core::ffi::c_int + p3 as core::ffi::c_int
            + 2 as core::ffi::c_int * p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *op1 = (p3 as core::ffi::c_int + p3 as core::ffi::c_int + p2 as core::ffi::c_int
            + 2 as core::ffi::c_int * p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *op0 = (p3 as core::ffi::c_int + p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + 2 as core::ffi::c_int * p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *oq0 = (p2 as core::ffi::c_int + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + 2 as core::ffi::c_int * q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *oq1 = (p1 as core::ffi::c_int + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + 2 as core::ffi::c_int * q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
        *oq2 = (p0 as core::ffi::c_int + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + 2 as core::ffi::c_int * q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q3 as core::ffi::c_int + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 3 as core::ffi::c_int) as uint16_t;
    } else {
        highbd_filter4(mask, thresh, op1, op0, oq0, oq1, bd);
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_8_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p3: uint16_t = *s.offset((-(4 as core::ffi::c_int) * p) as isize);
        let p2: uint16_t = *s.offset((-(3 as core::ffi::c_int) * p) as isize);
        let p1: uint16_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint16_t = *s.offset(-p as isize);
        let q0: uint16_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint16_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let q2: uint16_t = *s.offset((2 as core::ffi::c_int * p) as isize);
        let q3: uint16_t = *s.offset((3 as core::ffi::c_int * p) as isize);
        let mask: int8_t = highbd_filter_mask(
            *limit,
            *blimit,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        let flat: int8_t = highbd_flat_mask4(
            1 as uint8_t,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        highbd_filter8(
            mask,
            *thresh,
            flat,
            s.offset(-((4 as core::ffi::c_int * p) as isize)),
            s.offset(-((3 as core::ffi::c_int * p) as isize)),
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
            s.offset((2 as core::ffi::c_int * p) as isize),
            s.offset((3 as core::ffi::c_int * p) as isize),
            bd,
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_6_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p2: uint16_t = *s.offset((-(3 as core::ffi::c_int) * p) as isize);
        let p1: uint16_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint16_t = *s.offset(-p as isize);
        let q0: uint16_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint16_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let q2: uint16_t = *s.offset((2 as core::ffi::c_int * p) as isize);
        let mask: int8_t = highbd_filter_mask3_chroma(
            *limit,
            *blimit,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            bd,
        ) as int8_t;
        let flat: int8_t = highbd_flat_mask3_chroma(
            1 as uint8_t,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            bd,
        ) as int8_t;
        highbd_filter6(
            mask,
            *thresh,
            flat,
            s.offset(-((3 as core::ffi::c_int * p) as isize)),
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
            s.offset((2 as core::ffi::c_int * p) as isize),
            bd,
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_6_dual_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    aom_highbd_lpf_horizontal_6_c(s, p, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_horizontal_6_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_8_dual_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    aom_highbd_lpf_horizontal_8_c(s, p, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_horizontal_8_c(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_6_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p2: uint16_t = *s.offset(-(3 as core::ffi::c_int) as isize);
        let p1: uint16_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint16_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint16_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint16_t = *s.offset(1 as core::ffi::c_int as isize);
        let q2: uint16_t = *s.offset(2 as core::ffi::c_int as isize);
        let mask: int8_t = highbd_filter_mask3_chroma(
            *limit,
            *blimit,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            bd,
        ) as int8_t;
        let flat: int8_t = highbd_flat_mask3_chroma(
            1 as uint8_t,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            bd,
        ) as int8_t;
        highbd_filter6(
            mask,
            *thresh,
            flat,
            s.offset(-(3 as core::ffi::c_int as isize)),
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
            s.offset(2 as core::ffi::c_int as isize),
            bd,
        );
        s = s.offset(pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_6_dual_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    aom_highbd_lpf_vertical_6_c(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_vertical_6_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_8_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p3: uint16_t = *s.offset(-(4 as core::ffi::c_int) as isize);
        let p2: uint16_t = *s.offset(-(3 as core::ffi::c_int) as isize);
        let p1: uint16_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint16_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint16_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint16_t = *s.offset(1 as core::ffi::c_int as isize);
        let q2: uint16_t = *s.offset(2 as core::ffi::c_int as isize);
        let q3: uint16_t = *s.offset(3 as core::ffi::c_int as isize);
        let mask: int8_t = highbd_filter_mask(
            *limit,
            *blimit,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        let flat: int8_t = highbd_flat_mask4(
            1 as uint8_t,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        highbd_filter8(
            mask,
            *thresh,
            flat,
            s.offset(-(4 as core::ffi::c_int as isize)),
            s.offset(-(3 as core::ffi::c_int as isize)),
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
            s.offset(2 as core::ffi::c_int as isize),
            s.offset(3 as core::ffi::c_int as isize),
            bd,
        );
        s = s.offset(pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_8_dual_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    aom_highbd_lpf_vertical_8_c(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_vertical_8_c(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
}
#[inline]
unsafe extern "C-unwind" fn highbd_filter14(
    mut mask: int8_t,
    mut thresh: uint8_t,
    mut flat: int8_t,
    mut flat2: int8_t,
    mut op6: *mut uint16_t,
    mut op5: *mut uint16_t,
    mut op4: *mut uint16_t,
    mut op3: *mut uint16_t,
    mut op2: *mut uint16_t,
    mut op1: *mut uint16_t,
    mut op0: *mut uint16_t,
    mut oq0: *mut uint16_t,
    mut oq1: *mut uint16_t,
    mut oq2: *mut uint16_t,
    mut oq3: *mut uint16_t,
    mut oq4: *mut uint16_t,
    mut oq5: *mut uint16_t,
    mut oq6: *mut uint16_t,
    mut bd: core::ffi::c_int,
) {
    if flat2 as core::ffi::c_int != 0 && flat as core::ffi::c_int != 0
        && mask as core::ffi::c_int != 0
    {
        let p6: uint16_t = *op6;
        let p5: uint16_t = *op5;
        let p4: uint16_t = *op4;
        let p3: uint16_t = *op3;
        let p2: uint16_t = *op2;
        let p1: uint16_t = *op1;
        let p0: uint16_t = *op0;
        let q0: uint16_t = *oq0;
        let q1: uint16_t = *oq1;
        let q2: uint16_t = *oq2;
        let q3: uint16_t = *oq3;
        let q4: uint16_t = *oq4;
        let q5: uint16_t = *oq5;
        let q6: uint16_t = *oq6;
        *op5 = (p6 as core::ffi::c_int * 7 as core::ffi::c_int
            + p5 as core::ffi::c_int * 2 as core::ffi::c_int
            + p4 as core::ffi::c_int * 2 as core::ffi::c_int + p3 as core::ffi::c_int
            + p2 as core::ffi::c_int + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *op4 = (p6 as core::ffi::c_int * 5 as core::ffi::c_int
            + p5 as core::ffi::c_int * 2 as core::ffi::c_int
            + p4 as core::ffi::c_int * 2 as core::ffi::c_int
            + p3 as core::ffi::c_int * 2 as core::ffi::c_int + p2 as core::ffi::c_int
            + p1 as core::ffi::c_int + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *op3 = (p6 as core::ffi::c_int * 4 as core::ffi::c_int + p5 as core::ffi::c_int
            + p4 as core::ffi::c_int * 2 as core::ffi::c_int
            + p3 as core::ffi::c_int * 2 as core::ffi::c_int
            + p2 as core::ffi::c_int * 2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *op2 = (p6 as core::ffi::c_int * 3 as core::ffi::c_int + p5 as core::ffi::c_int
            + p4 as core::ffi::c_int + p3 as core::ffi::c_int * 2 as core::ffi::c_int
            + p2 as core::ffi::c_int * 2 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int + q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *op1 = (p6 as core::ffi::c_int * 2 as core::ffi::c_int + p5 as core::ffi::c_int
            + p4 as core::ffi::c_int + p3 as core::ffi::c_int
            + p2 as core::ffi::c_int * 2 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int + q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q4 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *op0 = (p6 as core::ffi::c_int + p5 as core::ffi::c_int + p4 as core::ffi::c_int
            + p3 as core::ffi::c_int + p2 as core::ffi::c_int
            + p1 as core::ffi::c_int * 2 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int + q3 as core::ffi::c_int + q4 as core::ffi::c_int
            + q5 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *oq0 = (p5 as core::ffi::c_int + p4 as core::ffi::c_int + p3 as core::ffi::c_int
            + p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int + q4 as core::ffi::c_int + q5 as core::ffi::c_int
            + q6 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *oq1 = (p4 as core::ffi::c_int + p3 as core::ffi::c_int + p2 as core::ffi::c_int
            + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int * 2 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int
            + q2 as core::ffi::c_int * 2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q4 as core::ffi::c_int + q5 as core::ffi::c_int
            + q6 as core::ffi::c_int * 2 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *oq2 = (p3 as core::ffi::c_int + p2 as core::ffi::c_int + p1 as core::ffi::c_int
            + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int * 2 as core::ffi::c_int
            + q2 as core::ffi::c_int * 2 as core::ffi::c_int
            + q3 as core::ffi::c_int * 2 as core::ffi::c_int + q4 as core::ffi::c_int
            + q5 as core::ffi::c_int + q6 as core::ffi::c_int * 3 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *oq3 = (p2 as core::ffi::c_int + p1 as core::ffi::c_int + p0 as core::ffi::c_int
            + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int * 2 as core::ffi::c_int
            + q3 as core::ffi::c_int * 2 as core::ffi::c_int
            + q4 as core::ffi::c_int * 2 as core::ffi::c_int + q5 as core::ffi::c_int
            + q6 as core::ffi::c_int * 4 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *oq4 = (p1 as core::ffi::c_int + p0 as core::ffi::c_int + q0 as core::ffi::c_int
            + q1 as core::ffi::c_int + q2 as core::ffi::c_int
            + q3 as core::ffi::c_int * 2 as core::ffi::c_int
            + q4 as core::ffi::c_int * 2 as core::ffi::c_int
            + q5 as core::ffi::c_int * 2 as core::ffi::c_int
            + q6 as core::ffi::c_int * 5 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
        *oq5 = (p0 as core::ffi::c_int + q0 as core::ffi::c_int + q1 as core::ffi::c_int
            + q2 as core::ffi::c_int + q3 as core::ffi::c_int
            + q4 as core::ffi::c_int * 2 as core::ffi::c_int
            + q5 as core::ffi::c_int * 2 as core::ffi::c_int
            + q6 as core::ffi::c_int * 7 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 4 as core::ffi::c_int) as uint16_t;
    } else {
        highbd_filter8(mask, thresh, flat, op3, op2, op1, op0, oq0, oq1, oq2, oq3, bd);
    };
}
unsafe extern "C-unwind" fn highbd_mb_lpf_horizontal_edge_w(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut count: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut step: core::ffi::c_int = 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < step * count {
        let p3: uint16_t = *s.offset((-(4 as core::ffi::c_int) * p) as isize);
        let p2: uint16_t = *s.offset((-(3 as core::ffi::c_int) * p) as isize);
        let p1: uint16_t = *s.offset((-(2 as core::ffi::c_int) * p) as isize);
        let p0: uint16_t = *s.offset(-p as isize);
        let q0: uint16_t = *s.offset((0 as core::ffi::c_int * p) as isize);
        let q1: uint16_t = *s.offset((1 as core::ffi::c_int * p) as isize);
        let q2: uint16_t = *s.offset((2 as core::ffi::c_int * p) as isize);
        let q3: uint16_t = *s.offset((3 as core::ffi::c_int * p) as isize);
        let mask: int8_t = highbd_filter_mask(
            *limit,
            *blimit,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        let flat: int8_t = highbd_flat_mask4(
            1 as uint8_t,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        let flat2: int8_t = highbd_flat_mask4(
            1 as uint8_t,
            *s.offset((-(7 as core::ffi::c_int) * p) as isize),
            *s.offset((-(6 as core::ffi::c_int) * p) as isize),
            *s.offset((-(5 as core::ffi::c_int) * p) as isize),
            p0,
            q0,
            *s.offset((4 as core::ffi::c_int * p) as isize),
            *s.offset((5 as core::ffi::c_int * p) as isize),
            *s.offset((6 as core::ffi::c_int * p) as isize),
            bd,
        ) as int8_t;
        highbd_filter14(
            mask,
            *thresh,
            flat,
            flat2,
            s.offset(-((7 as core::ffi::c_int * p) as isize)),
            s.offset(-((6 as core::ffi::c_int * p) as isize)),
            s.offset(-((5 as core::ffi::c_int * p) as isize)),
            s.offset(-((4 as core::ffi::c_int * p) as isize)),
            s.offset(-((3 as core::ffi::c_int * p) as isize)),
            s.offset(-((2 as core::ffi::c_int * p) as isize)),
            s.offset(-((1 as core::ffi::c_int * p) as isize)),
            s,
            s.offset((1 as core::ffi::c_int * p) as isize),
            s.offset((2 as core::ffi::c_int * p) as isize),
            s.offset((3 as core::ffi::c_int * p) as isize),
            s.offset((4 as core::ffi::c_int * p) as isize),
            s.offset((5 as core::ffi::c_int * p) as isize),
            s.offset((6 as core::ffi::c_int * p) as isize),
            bd,
        );
        s = s.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_14_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    highbd_mb_lpf_horizontal_edge_w(
        s,
        pitch,
        blimit,
        limit,
        thresh,
        1 as core::ffi::c_int,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_14_dual_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    highbd_mb_lpf_horizontal_edge_w(
        s,
        p,
        blimit0,
        limit0,
        thresh0,
        1 as core::ffi::c_int,
        bd,
    );
    highbd_mb_lpf_horizontal_edge_w(
        s.offset(4 as core::ffi::c_int as isize),
        p,
        blimit1,
        limit1,
        thresh1,
        1 as core::ffi::c_int,
        bd,
    );
}
unsafe extern "C-unwind" fn highbd_mb_lpf_vertical_edge_w(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut count: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < count {
        let p3: uint16_t = *s.offset(-(4 as core::ffi::c_int) as isize);
        let p2: uint16_t = *s.offset(-(3 as core::ffi::c_int) as isize);
        let p1: uint16_t = *s.offset(-(2 as core::ffi::c_int) as isize);
        let p0: uint16_t = *s.offset(-(1 as core::ffi::c_int) as isize);
        let q0: uint16_t = *s.offset(0 as core::ffi::c_int as isize);
        let q1: uint16_t = *s.offset(1 as core::ffi::c_int as isize);
        let q2: uint16_t = *s.offset(2 as core::ffi::c_int as isize);
        let q3: uint16_t = *s.offset(3 as core::ffi::c_int as isize);
        let mask: int8_t = highbd_filter_mask(
            *limit,
            *blimit,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        let flat: int8_t = highbd_flat_mask4(
            1 as uint8_t,
            p3,
            p2,
            p1,
            p0,
            q0,
            q1,
            q2,
            q3,
            bd,
        ) as int8_t;
        let flat2: int8_t = highbd_flat_mask4(
            1 as uint8_t,
            *s.offset(-(7 as core::ffi::c_int) as isize),
            *s.offset(-(6 as core::ffi::c_int) as isize),
            *s.offset(-(5 as core::ffi::c_int) as isize),
            p0,
            q0,
            *s.offset(4 as core::ffi::c_int as isize),
            *s.offset(5 as core::ffi::c_int as isize),
            *s.offset(6 as core::ffi::c_int as isize),
            bd,
        ) as int8_t;
        highbd_filter14(
            mask,
            *thresh,
            flat,
            flat2,
            s.offset(-(7 as core::ffi::c_int as isize)),
            s.offset(-(6 as core::ffi::c_int as isize)),
            s.offset(-(5 as core::ffi::c_int as isize)),
            s.offset(-(4 as core::ffi::c_int as isize)),
            s.offset(-(3 as core::ffi::c_int as isize)),
            s.offset(-(2 as core::ffi::c_int as isize)),
            s.offset(-(1 as core::ffi::c_int as isize)),
            s,
            s.offset(1 as core::ffi::c_int as isize),
            s.offset(2 as core::ffi::c_int as isize),
            s.offset(3 as core::ffi::c_int as isize),
            s.offset(4 as core::ffi::c_int as isize),
            s.offset(5 as core::ffi::c_int as isize),
            s.offset(6 as core::ffi::c_int as isize),
            bd,
        );
        s = s.offset(p as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_14_c(
    mut s: *mut uint16_t,
    mut p: core::ffi::c_int,
    mut blimit: *const uint8_t,
    mut limit: *const uint8_t,
    mut thresh: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    highbd_mb_lpf_vertical_edge_w(
        s,
        p,
        blimit,
        limit,
        thresh,
        4 as core::ffi::c_int,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_14_dual_c(
    mut s: *mut uint16_t,
    mut pitch: core::ffi::c_int,
    mut blimit0: *const uint8_t,
    mut limit0: *const uint8_t,
    mut thresh0: *const uint8_t,
    mut blimit1: *const uint8_t,
    mut limit1: *const uint8_t,
    mut thresh1: *const uint8_t,
    mut bd: core::ffi::c_int,
) {
    highbd_mb_lpf_vertical_edge_w(
        s,
        pitch,
        blimit0,
        limit0,
        thresh0,
        4 as core::ffi::c_int,
        bd,
    );
    highbd_mb_lpf_vertical_edge_w(
        s.offset((4 as core::ffi::c_int * pitch) as isize),
        pitch,
        blimit1,
        limit1,
        thresh1,
        4 as core::ffi::c_int,
        bd,
    );
}

extern "C" {
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
}
