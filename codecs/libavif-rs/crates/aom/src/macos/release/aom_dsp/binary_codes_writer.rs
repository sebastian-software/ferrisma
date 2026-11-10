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

// Mechanically translated from upstream/aom/aom_dsp/binary_codes_writer.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn od_ec_encode_bool_q15(
        enc: *mut od_ec_enc,
        val: core::ffi::c_int,
        f_q15: core::ffi::c_uint,
    );
}
pub type size_t = usize;
pub type int16_t = i16;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type od_ec_enc_window = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_enc {
    pub buf: *mut core::ffi::c_uchar,
    pub storage: uint32_t,
    pub offs: uint32_t,
    pub low: od_ec_enc_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
    pub error: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_writer {
    pub pos: core::ffi::c_uint,
    pub buffer: *mut uint8_t,
    pub size: size_t,
    pub ec: od_ec_enc,
    pub allow_update_cdf: uint8_t,
}
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C-unwind" fn aom_write(
    mut w: *mut aom_writer,
    mut bit: core::ffi::c_int,
    mut probability: core::ffi::c_int,
) {
    let mut p: core::ffi::c_int = 0x7fffff as core::ffi::c_int
        - (probability << 15 as core::ffi::c_int) + probability >> 8 as core::ffi::c_int;
    od_ec_encode_bool_q15(&raw mut (*w).ec, bit, p as core::ffi::c_uint);
}
#[inline]
unsafe extern "C-unwind" fn aom_write_bit(mut w: *mut aom_writer, mut bit: core::ffi::c_int) {
    aom_write(w, bit, 128 as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn aom_write_literal(
    mut w: *mut aom_writer,
    mut data: core::ffi::c_int,
    mut bits: core::ffi::c_int,
) {
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        aom_write_bit(w, 1 as core::ffi::c_int & data >> bit);
        bit -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn recenter_nonneg(mut r: uint16_t, mut v: uint16_t) -> uint16_t {
    if v as core::ffi::c_int > (r as core::ffi::c_int) << 1 as core::ffi::c_int {
        return v
    } else if v as core::ffi::c_int >= r as core::ffi::c_int {
        return ((v as core::ffi::c_int - r as core::ffi::c_int) << 1 as core::ffi::c_int)
            as uint16_t
    } else {
        return (((r as core::ffi::c_int - v as core::ffi::c_int)
            << 1 as core::ffi::c_int) - 1 as core::ffi::c_int) as uint16_t
    };
}
#[inline]
unsafe extern "C-unwind" fn recenter_finite_nonneg(
    mut n: uint16_t,
    mut r: uint16_t,
    mut v: uint16_t,
) -> uint16_t {
    if (r as core::ffi::c_int) << 1 as core::ffi::c_int <= n as core::ffi::c_int {
        return recenter_nonneg(r, v)
    } else {
        return recenter_nonneg(
            (n as core::ffi::c_int - 1 as core::ffi::c_int - r as core::ffi::c_int)
                as uint16_t,
            (n as core::ffi::c_int - 1 as core::ffi::c_int - v as core::ffi::c_int)
                as uint16_t,
        )
    };
}
unsafe extern "C-unwind" fn write_primitive_quniform(
    mut w: *mut aom_writer,
    mut n: uint16_t,
    mut v: uint16_t,
) {
    if n as core::ffi::c_int <= 1 as core::ffi::c_int {
        return;
    }
    let l: core::ffi::c_int = get_msb(n as core::ffi::c_uint) as core::ffi::c_int
        + 1 as core::ffi::c_int;
    let m: core::ffi::c_int = ((1 as core::ffi::c_int) << l) - n as core::ffi::c_int;
    if (v as core::ffi::c_int) < m {
        aom_write_literal(w, v as core::ffi::c_int, l - 1 as core::ffi::c_int);
    } else {
        aom_write_literal(
            w,
            m + (v as core::ffi::c_int - m >> 1 as core::ffi::c_int),
            l - 1 as core::ffi::c_int,
        );
        aom_write_bit(w, v as core::ffi::c_int - m & 1 as core::ffi::c_int);
    };
}
unsafe extern "C-unwind" fn count_primitive_quniform(
    mut n: uint16_t,
    mut v: uint16_t,
) -> core::ffi::c_int {
    if n as core::ffi::c_int <= 1 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let l: core::ffi::c_int = get_msb(n as core::ffi::c_uint) as core::ffi::c_int
        + 1 as core::ffi::c_int;
    let m: core::ffi::c_int = ((1 as core::ffi::c_int) << l) - n as core::ffi::c_int;
    return if (v as core::ffi::c_int) < m { l - 1 as core::ffi::c_int } else { l };
}
unsafe extern "C-unwind" fn write_primitive_subexpfin(
    mut w: *mut aom_writer,
    mut n: uint16_t,
    mut k: uint16_t,
    mut v: uint16_t,
) {
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
            write_primitive_quniform(
                w,
                (n as core::ffi::c_int - mk) as uint16_t,
                (v as core::ffi::c_int - mk) as uint16_t,
            );
            break;
        } else {
            let mut t: core::ffi::c_int = (v as core::ffi::c_int >= mk + a)
                as core::ffi::c_int;
            aom_write_bit(w, t);
            if t != 0 {
                i = i + 1 as core::ffi::c_int;
                mk += a;
            } else {
                aom_write_literal(w, v as core::ffi::c_int - mk, b);
                break;
            }
        }
    };
}
unsafe extern "C-unwind" fn count_primitive_subexpfin(
    mut n: uint16_t,
    mut k: uint16_t,
    mut v: uint16_t,
) -> core::ffi::c_int {
    let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
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
            count
                += count_primitive_quniform(
                    (n as core::ffi::c_int - mk) as uint16_t,
                    (v as core::ffi::c_int - mk) as uint16_t,
                );
            break;
        } else {
            let mut t: core::ffi::c_int = (v as core::ffi::c_int >= mk + a)
                as core::ffi::c_int;
            count += 1;
            if t != 0 {
                i = i + 1 as core::ffi::c_int;
                mk += a;
            } else {
                count += b;
                break;
            }
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_write_primitive_refsubexpfin(
    mut w: *mut aom_writer,
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: uint16_t,
    mut v: uint16_t,
) {
    write_primitive_subexpfin(w, n, k, recenter_finite_nonneg(n, ref_0, v));
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_count_primitive_refsubexpfin(
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: uint16_t,
    mut v: uint16_t,
) -> core::ffi::c_int {
    return count_primitive_subexpfin(n, k, recenter_finite_nonneg(n, ref_0, v));
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_count_signed_primitive_refsubexpfin(
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: int16_t,
    mut v: int16_t,
) -> core::ffi::c_int {
    ref_0 = (ref_0 as core::ffi::c_int + (n as core::ffi::c_int - 1 as core::ffi::c_int))
        as int16_t;
    v = (v as core::ffi::c_int + (n as core::ffi::c_int - 1 as core::ffi::c_int))
        as int16_t;
    let scaled_n: uint16_t = (((n as core::ffi::c_int) << 1 as core::ffi::c_int)
        - 1 as core::ffi::c_int) as uint16_t;
    return aom_count_primitive_refsubexpfin(
        scaled_n,
        k,
        ref_0 as uint16_t,
        v as uint16_t,
    );
}

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_aom_write(
    w: *mut aom_writer,
    bit: core::ffi::c_int,
    probability: core::ffi::c_int,
) {
    aom_write(w, bit, probability)
}
