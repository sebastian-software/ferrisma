/*
 * Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/entdec.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn od_ec_tell_frac(nbits_total: uint32_t, rng: uint32_t) -> uint32_t;
}
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type od_ec_window = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_dec {
    pub buf: *const core::ffi::c_uchar,
    pub tell_offs: int32_t,
    pub end: *const core::ffi::c_uchar,
    pub bptr: *const core::ffi::c_uchar,
    pub dif: od_ec_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
}
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const EC_PROB_SHIFT: core::ffi::c_int = 6 as core::ffi::c_int;
pub const EC_MIN_PROB: core::ffi::c_int = 4 as core::ffi::c_int;
pub const OD_EC_WINDOW_SIZE: core::ffi::c_int = ::core::mem::size_of::<od_ec_window>()
    as core::ffi::c_int * CHAR_BIT;
pub const OD_EC_LOTS_OF_BITS: core::ffi::c_int = 0x4000 as core::ffi::c_int;
unsafe extern "C-unwind" fn od_ec_dec_refill(mut dec: *mut od_ec_dec) {
    let mut s: core::ffi::c_int = 0;
    let mut dif: od_ec_window = 0;
    let mut cnt: int16_t = 0;
    let mut bptr: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    let mut end: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    dif = (*dec).dif;
    cnt = (*dec).cnt;
    bptr = (*dec).bptr;
    end = (*dec).end;
    s = OD_EC_WINDOW_SIZE - 9 as core::ffi::c_int
        - (cnt as core::ffi::c_int + 15 as core::ffi::c_int);
    while s >= 0 as core::ffi::c_int && bptr < end {
        dif ^= (*bptr.offset(0 as core::ffi::c_int as isize) as od_ec_window) << s;
        cnt = (cnt as core::ffi::c_int + 8 as core::ffi::c_int) as int16_t;
        s -= 8 as core::ffi::c_int;
        bptr = bptr.offset(1);
    }
    if bptr >= end {
        (*dec).tell_offs = ((*dec).tell_offs as core::ffi::c_int
            + (OD_EC_LOTS_OF_BITS - cnt as core::ffi::c_int)) as int32_t;
        cnt = OD_EC_LOTS_OF_BITS as int16_t;
    }
    (*dec).dif = dif;
    (*dec).cnt = cnt;
    (*dec).bptr = bptr;
}
unsafe extern "C-unwind" fn od_ec_dec_normalize(
    mut dec: *mut od_ec_dec,
    mut dif: od_ec_window,
    mut rng: core::ffi::c_uint,
    mut ret: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut d: core::ffi::c_int = 0;
    d = 16 as core::ffi::c_int - (1 as core::ffi::c_int + get_msb(rng));
    (*dec).cnt = ((*dec).cnt as core::ffi::c_int - d) as int16_t;
    (*dec).dif = (dif.wrapping_add(1 as od_ec_window) << d)
        .wrapping_sub(1 as od_ec_window);
    (*dec).rng = (rng << d) as uint16_t;
    if ((*dec).cnt as core::ffi::c_int) < 0 as core::ffi::c_int {
        od_ec_dec_refill(dec);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_dec_init(
    mut dec: *mut od_ec_dec,
    mut buf: *const core::ffi::c_uchar,
    mut storage: uint32_t,
) {
    (*dec).buf = buf;
    (*dec).tell_offs = (10 as core::ffi::c_int
        - (OD_EC_WINDOW_SIZE - 8 as core::ffi::c_int)) as int32_t;
    (*dec).end = buf.offset(storage as isize);
    (*dec).bptr = buf;
    (*dec).dif = ((1 as core::ffi::c_int as od_ec_window)
        << OD_EC_WINDOW_SIZE - 1 as core::ffi::c_int)
        .wrapping_sub(1 as od_ec_window);
    (*dec).rng = 0x8000 as uint16_t;
    (*dec).cnt = -(15 as core::ffi::c_int) as int16_t;
    od_ec_dec_refill(dec);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_decode_bool_q15(
    mut dec: *mut od_ec_dec,
    mut f: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut dif: od_ec_window = 0;
    let mut vw: od_ec_window = 0;
    let mut r: core::ffi::c_uint = 0;
    let mut r_new: core::ffi::c_uint = 0;
    let mut v: core::ffi::c_uint = 0;
    let mut ret: core::ffi::c_int = 0;
    dif = (*dec).dif;
    r = (*dec).rng as core::ffi::c_uint;
    v = ((r as uint32_t >> 8 as core::ffi::c_int)
        .wrapping_mul((f >> EC_PROB_SHIFT) as uint32_t)
        >> 7 as core::ffi::c_int - EC_PROB_SHIFT) as core::ffi::c_uint;
    v = v.wrapping_add(EC_MIN_PROB as core::ffi::c_uint);
    vw = (v as od_ec_window) << OD_EC_WINDOW_SIZE - 16 as core::ffi::c_int;
    ret = 1 as core::ffi::c_int;
    r_new = v;
    if dif >= vw {
        r_new = r.wrapping_sub(v);
        dif = dif.wrapping_sub(vw);
        ret = 0 as core::ffi::c_int;
    }
    return od_ec_dec_normalize(dec, dif, r_new, ret);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_decode_cdf_q15(
    mut dec: *mut od_ec_dec,
    mut icdf: *const uint16_t,
    mut nsyms: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut dif: od_ec_window = 0;
    let mut r: core::ffi::c_uint = 0;
    let mut c: core::ffi::c_uint = 0;
    let mut u: core::ffi::c_uint = 0;
    let mut v: core::ffi::c_uint = 0;
    let mut ret: core::ffi::c_int = 0;
    dif = (*dec).dif;
    r = (*dec).rng as core::ffi::c_uint;
    let N: core::ffi::c_int = nsyms - 1 as core::ffi::c_int;
    c = (dif >> OD_EC_WINDOW_SIZE - 16 as core::ffi::c_int) as core::ffi::c_uint;
    v = r;
    ret = -(1 as core::ffi::c_int);
    loop {
        u = v;
        ret += 1;
        v = ((r as uint32_t >> 8 as core::ffi::c_int)
            .wrapping_mul(
                (*icdf.offset(ret as isize) as core::ffi::c_int >> EC_PROB_SHIFT)
                    as uint32_t,
            ) >> 7 as core::ffi::c_int - EC_PROB_SHIFT) as core::ffi::c_uint;
        v = v.wrapping_add((EC_MIN_PROB * (N - ret)) as core::ffi::c_uint);
        if !(c < v) {
            break;
        }
    }
    r = u.wrapping_sub(v);
    dif = dif
        .wrapping_sub((v as od_ec_window) << OD_EC_WINDOW_SIZE - 16 as core::ffi::c_int);
    return od_ec_dec_normalize(dec, dif, r, ret);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_dec_tell(mut dec: *const od_ec_dec) -> core::ffi::c_int {
    return (((*dec).bptr).offset_from((*dec).buf) as core::ffi::c_long
        * 8 as core::ffi::c_long - (*dec).cnt as core::ffi::c_long
        + (*dec).tell_offs as core::ffi::c_long) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_dec_tell_frac(mut dec: *const od_ec_dec) -> uint32_t {
    return od_ec_tell_frac(od_ec_dec_tell(dec) as uint32_t, (*dec).rng as uint32_t);
}
pub const __CHAR_BIT__: core::ffi::c_int = 8 as core::ffi::c_int;
pub const CHAR_BIT: core::ffi::c_int = __CHAR_BIT__;
