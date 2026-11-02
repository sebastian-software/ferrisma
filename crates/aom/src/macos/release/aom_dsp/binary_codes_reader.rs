/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/binary_codes_reader.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn od_ec_decode_bool_q15(
        dec: *mut od_ec_dec,
        f: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_reader {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub ec: od_ec_dec,
    pub allow_update_cdf: uint8_t,
}
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_(
    mut r: *mut aom_reader,
    mut prob: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut p: core::ffi::c_int = 0x7fffff as core::ffi::c_int
        - (prob << 15 as core::ffi::c_int) + prob >> 8 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = od_ec_decode_bool_q15(
        &raw mut (*r).ec,
        p as core::ffi::c_uint,
    );
    return bit;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_bit_(mut r: *mut aom_reader) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    ret = aom_read_(r, 128 as core::ffi::c_int);
    return ret;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_literal_(
    mut r: *mut aom_reader,
    mut bits: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut literal: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        literal |= aom_read_bit_(r) << bit;
        bit -= 1;
    }
    return literal;
}
#[inline]
unsafe extern "C-unwind" fn inv_recenter_nonneg(mut r: uint16_t, mut v: uint16_t) -> uint16_t {
    if v as core::ffi::c_int > (r as core::ffi::c_int) << 1 as core::ffi::c_int {
        return v
    } else if v as core::ffi::c_int & 1 as core::ffi::c_int == 0 as core::ffi::c_int {
        return ((v as core::ffi::c_int >> 1 as core::ffi::c_int) + r as core::ffi::c_int)
            as uint16_t
    } else {
        return (r as core::ffi::c_int
            - (v as core::ffi::c_int + 1 as core::ffi::c_int >> 1 as core::ffi::c_int))
            as uint16_t
    };
}
#[inline]
unsafe extern "C-unwind" fn inv_recenter_finite_nonneg(
    mut n: uint16_t,
    mut r: uint16_t,
    mut v: uint16_t,
) -> uint16_t {
    if (r as core::ffi::c_int) << 1 as core::ffi::c_int <= n as core::ffi::c_int {
        return inv_recenter_nonneg(r, v)
    } else {
        return (n as core::ffi::c_int - 1 as core::ffi::c_int
            - inv_recenter_nonneg(
                (n as core::ffi::c_int - 1 as core::ffi::c_int - r as core::ffi::c_int)
                    as uint16_t,
                v,
            ) as core::ffi::c_int) as uint16_t
    };
}
unsafe extern "C-unwind" fn read_primitive_quniform_(
    mut r: *mut aom_reader,
    mut n: uint16_t,
) -> uint16_t {
    if n as core::ffi::c_int <= 1 as core::ffi::c_int {
        return 0 as uint16_t;
    }
    let l: core::ffi::c_int = get_msb(n as core::ffi::c_uint) as core::ffi::c_int
        + 1 as core::ffi::c_int;
    let m: core::ffi::c_int = ((1 as core::ffi::c_int) << l) - n as core::ffi::c_int;
    let v: core::ffi::c_int = aom_read_literal_(r, l - 1 as core::ffi::c_int)
        as core::ffi::c_int;
    return (if v < m { v } else { (v << 1 as core::ffi::c_int) - m + aom_read_bit_(r) })
        as uint16_t;
}
unsafe extern "C-unwind" fn read_primitive_subexpfin_(
    mut r: *mut aom_reader,
    mut n: uint16_t,
    mut k: uint16_t,
) -> uint16_t {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mk: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let mut b: core::ffi::c_int = if i != 0 {
            k as core::ffi::c_int + i - 1 as core::ffi::c_int
        } else {
            k as core::ffi::c_int
        };
        let mut a: core::ffi::c_int = (1 as core::ffi::c_int) << b;
        if n as core::ffi::c_int <= mk + 3 as core::ffi::c_int * a {
            return (read_primitive_quniform_(r, (n as core::ffi::c_int - mk) as uint16_t)
                as core::ffi::c_int + mk) as uint16_t;
        }
        if aom_read_bit_(r) == 0 {
            return (aom_read_literal_(r, b) + mk) as uint16_t;
        }
        i = i + 1 as core::ffi::c_int;
        mk += a;
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_read_primitive_refsubexpfin_(
    mut r: *mut aom_reader,
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: uint16_t,
) -> uint16_t {
    return inv_recenter_finite_nonneg(n, ref_0, read_primitive_subexpfin_(r, n, k));
}

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_aom_read_(
    r: *mut aom_reader,
    prob: core::ffi::c_int,
) -> core::ffi::c_int {
    aom_read_(r, prob)
}
