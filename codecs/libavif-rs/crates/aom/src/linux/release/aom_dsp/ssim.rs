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

// Mechanically translated from upstream/aom/aom_dsp/ssim.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ssim_parms_8x8_c(
    mut s: *const uint8_t,
    mut sp: core::ffi::c_int,
    mut r: *const uint8_t,
    mut rp: core::ffi::c_int,
    mut sum_s: *mut uint32_t,
    mut sum_r: *mut uint32_t,
    mut sum_sq_s: *mut uint32_t,
    mut sum_sq_r: *mut uint32_t,
    mut sum_sxr: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            *sum_s = (*sum_s).wrapping_add(*s.offset(j as isize) as uint32_t);
            *sum_r = (*sum_r).wrapping_add(*r.offset(j as isize) as uint32_t);
            *sum_sq_s = (*sum_sq_s)
                .wrapping_add(
                    (*s.offset(j as isize) as core::ffi::c_int
                        * *s.offset(j as isize) as core::ffi::c_int) as uint32_t,
                );
            *sum_sq_r = (*sum_sq_r)
                .wrapping_add(
                    (*r.offset(j as isize) as core::ffi::c_int
                        * *r.offset(j as isize) as core::ffi::c_int) as uint32_t,
                );
            *sum_sxr = (*sum_sxr)
                .wrapping_add(
                    (*s.offset(j as isize) as core::ffi::c_int
                        * *r.offset(j as isize) as core::ffi::c_int) as uint32_t,
                );
            j += 1;
        }
        i += 1;
        s = s.offset(sp as isize);
        r = r.offset(rp as isize);
    }
}
static mut cc1: int64_t = 26634 as int64_t;
static mut cc2: int64_t = 239708 as int64_t;
static mut cc1_10: int64_t = 428658 as int64_t;
static mut cc2_10: int64_t = 3857925 as int64_t;
static mut cc1_12: int64_t = 6868593 as int64_t;
static mut cc2_12: int64_t = 61817334 as int64_t;
unsafe extern "C-unwind" fn similarity(
    mut sum_s: uint32_t,
    mut sum_r: uint32_t,
    mut sum_sq_s: uint32_t,
    mut sum_sq_r: uint32_t,
    mut sum_sxr: uint32_t,
    mut count: core::ffi::c_int,
    mut bd: uint32_t,
) -> core::ffi::c_double {
    let mut ssim_n: core::ffi::c_double = 0.;
    let mut ssim_d: core::ffi::c_double = 0.;
    let mut c1: int64_t = 0 as int64_t;
    let mut c2: int64_t = 0 as int64_t;
    if bd == 8 as uint32_t {
        c1 = cc1 * count as int64_t * count as int64_t >> 12 as core::ffi::c_int;
        c2 = cc2 * count as int64_t * count as int64_t >> 12 as core::ffi::c_int;
    } else if bd == 10 as uint32_t {
        c1 = cc1_10 * count as int64_t * count as int64_t >> 12 as core::ffi::c_int;
        c2 = cc2_10 * count as int64_t * count as int64_t >> 12 as core::ffi::c_int;
    } else if bd == 12 as uint32_t {
        c1 = cc1_12 * count as int64_t * count as int64_t >> 12 as core::ffi::c_int;
        c2 = cc2_12 * count as int64_t * count as int64_t >> 12 as core::ffi::c_int;
    } else {
        return 0 as core::ffi::c_int as core::ffi::c_double
    }
    ssim_n = (2.0f64 * sum_s as core::ffi::c_double * sum_r as core::ffi::c_double
        + c1 as core::ffi::c_double)
        * (2.0f64 * count as core::ffi::c_double * sum_sxr as core::ffi::c_double
            - 2.0f64 * sum_s as core::ffi::c_double * sum_r as core::ffi::c_double
            + c2 as core::ffi::c_double);
    ssim_d = (sum_s as core::ffi::c_double * sum_s as core::ffi::c_double
        + sum_r as core::ffi::c_double * sum_r as core::ffi::c_double
        + c1 as core::ffi::c_double)
        * (count as core::ffi::c_double * sum_sq_s as core::ffi::c_double
            - sum_s as core::ffi::c_double * sum_s as core::ffi::c_double
            + count as core::ffi::c_double * sum_sq_r as core::ffi::c_double
            - sum_r as core::ffi::c_double * sum_r as core::ffi::c_double
            + c2 as core::ffi::c_double);
    return ssim_n / ssim_d;
}
unsafe extern "C-unwind" fn ssim_8x8(
    mut s: *const uint8_t,
    mut sp: core::ffi::c_int,
    mut r: *const uint8_t,
    mut rp: core::ffi::c_int,
) -> core::ffi::c_double {
    let mut sum_s: uint32_t = 0 as uint32_t;
    let mut sum_r: uint32_t = 0 as uint32_t;
    let mut sum_sq_s: uint32_t = 0 as uint32_t;
    let mut sum_sq_r: uint32_t = 0 as uint32_t;
    let mut sum_sxr: uint32_t = 0 as uint32_t;
    aom_ssim_parms_8x8_c(
        s,
        sp,
        r,
        rp,
        &raw mut sum_s,
        &raw mut sum_r,
        &raw mut sum_sq_s,
        &raw mut sum_sq_r,
        &raw mut sum_sxr,
    );
    return similarity(
        sum_s,
        sum_r,
        sum_sq_s,
        sum_sq_r,
        sum_sxr,
        64 as core::ffi::c_int,
        8 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ssim2(
    mut img1: *const uint8_t,
    mut img2: *const uint8_t,
    mut stride_img1: core::ffi::c_int,
    mut stride_img2: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_double {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut samples: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ssim_total: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    i = 0 as core::ffi::c_int;
    while i <= height - 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j <= width - 8 as core::ffi::c_int {
            let mut v: core::ffi::c_double = ssim_8x8(
                img1.offset(j as isize),
                stride_img1,
                img2.offset(j as isize),
                stride_img2,
            );
            ssim_total += v;
            samples += 1;
            j += 4 as core::ffi::c_int;
        }
        i += 4 as core::ffi::c_int;
        img1 = img1.offset((stride_img1 * 4 as core::ffi::c_int) as isize);
        img2 = img2.offset((stride_img2 * 4 as core::ffi::c_int) as isize);
    }
    ssim_total /= samples as core::ffi::c_double;
    return ssim_total;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_ssim_parms_8x8_c(
    mut s: *const uint16_t,
    mut sp: core::ffi::c_int,
    mut r: *const uint16_t,
    mut rp: core::ffi::c_int,
    mut sum_s: *mut uint32_t,
    mut sum_r: *mut uint32_t,
    mut sum_sq_s: *mut uint32_t,
    mut sum_sq_r: *mut uint32_t,
    mut sum_sxr: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            *sum_s = (*sum_s).wrapping_add(*s.offset(j as isize) as uint32_t);
            *sum_r = (*sum_r).wrapping_add(*r.offset(j as isize) as uint32_t);
            *sum_sq_s = (*sum_sq_s)
                .wrapping_add(
                    (*s.offset(j as isize) as core::ffi::c_int
                        * *s.offset(j as isize) as core::ffi::c_int) as uint32_t,
                );
            *sum_sq_r = (*sum_sq_r)
                .wrapping_add(
                    (*r.offset(j as isize) as core::ffi::c_int
                        * *r.offset(j as isize) as core::ffi::c_int) as uint32_t,
                );
            *sum_sxr = (*sum_sxr)
                .wrapping_add(
                    (*s.offset(j as isize) as core::ffi::c_int
                        * *r.offset(j as isize) as core::ffi::c_int) as uint32_t,
                );
            j += 1;
        }
        i += 1;
        s = s.offset(sp as isize);
        r = r.offset(rp as isize);
    }
}
unsafe extern "C-unwind" fn highbd_ssim_8x8(
    mut s: *const uint16_t,
    mut sp: core::ffi::c_int,
    mut r: *const uint16_t,
    mut rp: core::ffi::c_int,
    mut bd: uint32_t,
    mut shift: uint32_t,
) -> core::ffi::c_double {
    let mut sum_s: uint32_t = 0 as uint32_t;
    let mut sum_r: uint32_t = 0 as uint32_t;
    let mut sum_sq_s: uint32_t = 0 as uint32_t;
    let mut sum_sq_r: uint32_t = 0 as uint32_t;
    let mut sum_sxr: uint32_t = 0 as uint32_t;
    aom_highbd_ssim_parms_8x8_c(
        s,
        sp,
        r,
        rp,
        &raw mut sum_s,
        &raw mut sum_r,
        &raw mut sum_sq_s,
        &raw mut sum_sq_r,
        &raw mut sum_sxr,
    );
    return similarity(
        sum_s >> shift,
        sum_r >> shift,
        sum_sq_s >> (2 as uint32_t).wrapping_mul(shift),
        sum_sq_r >> (2 as uint32_t).wrapping_mul(shift),
        sum_sxr >> (2 as uint32_t).wrapping_mul(shift),
        64 as core::ffi::c_int,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_ssim2(
    mut img1: *const uint8_t,
    mut img2: *const uint8_t,
    mut stride_img1: core::ffi::c_int,
    mut stride_img2: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut bd: uint32_t,
    mut shift: uint32_t,
) -> core::ffi::c_double {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut samples: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ssim_total: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    i = 0 as core::ffi::c_int;
    while i <= height - 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j <= width - 8 as core::ffi::c_int {
            let mut v: core::ffi::c_double = highbd_ssim_8x8(
                ((img1.offset(j as isize) as uintptr_t) << 1 as core::ffi::c_int)
                    as *mut uint16_t,
                stride_img1,
                ((img2.offset(j as isize) as uintptr_t) << 1 as core::ffi::c_int)
                    as *mut uint16_t,
                stride_img2,
                bd,
                shift,
            );
            ssim_total += v;
            samples += 1;
            j += 4 as core::ffi::c_int;
        }
        i += 4 as core::ffi::c_int;
        img1 = img1.offset((stride_img1 * 4 as core::ffi::c_int) as isize);
        img2 = img2.offset((stride_img2 * 4 as core::ffi::c_int) as isize);
    }
    ssim_total /= samples as core::ffi::c_double;
    return ssim_total;
}
