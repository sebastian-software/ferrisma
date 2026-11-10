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

// Mechanically translated from upstream/aom/aom_dsp/bitwriter_buffer.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_write_bit_buffer {
    pub bit_buffer: *mut uint8_t,
    pub bit_offset: uint32_t,
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_is_byte_aligned(
    mut wb: *const aom_write_bit_buffer,
) -> core::ffi::c_int {
    return (((*wb).bit_offset).wrapping_rem(CHAR_BIT as uint32_t) == 0 as uint32_t)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_bytes_written(
    mut wb: *const aom_write_bit_buffer,
) -> uint32_t {
    return ((*wb).bit_offset)
        .wrapping_div(CHAR_BIT as uint32_t)
        .wrapping_add(
            (((*wb).bit_offset).wrapping_rem(CHAR_BIT as uint32_t) > 0 as uint32_t)
                as core::ffi::c_int as uint32_t,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_write_bit(
    mut wb: *mut aom_write_bit_buffer,
    mut bit: core::ffi::c_int,
) {
    let off: core::ffi::c_int = (*wb).bit_offset as core::ffi::c_int;
    let p: core::ffi::c_int = off / CHAR_BIT;
    let q: core::ffi::c_int = CHAR_BIT - 1 as core::ffi::c_int - off % CHAR_BIT;
    if q == CHAR_BIT - 1 as core::ffi::c_int {
        *((*wb).bit_buffer).offset(p as isize) = (bit << q) as uint8_t;
    } else {
        let fresh0 = &raw mut *((*wb).bit_buffer).offset(p as isize);
        *fresh0 = (*fresh0 as core::ffi::c_int & !((1 as core::ffi::c_int) << q))
            as uint8_t;
        let fresh1 = &raw mut *((*wb).bit_buffer).offset(p as isize);
        *fresh1 = (*fresh1 as core::ffi::c_int | bit << q) as uint8_t;
    }
    (*wb).bit_offset = (off + 1 as core::ffi::c_int) as uint32_t;
}
unsafe extern "C-unwind" fn overwrite_bit(
    mut wb: *mut aom_write_bit_buffer,
    mut bit: core::ffi::c_int,
) {
    let off: core::ffi::c_int = (*wb).bit_offset as core::ffi::c_int;
    let p: core::ffi::c_int = off / CHAR_BIT;
    let q: core::ffi::c_int = CHAR_BIT - 1 as core::ffi::c_int - off % CHAR_BIT;
    let fresh2 = &raw mut *((*wb).bit_buffer).offset(p as isize);
    *fresh2 = (*fresh2 as core::ffi::c_int & !((1 as core::ffi::c_int) << q)) as uint8_t;
    let fresh3 = &raw mut *((*wb).bit_buffer).offset(p as isize);
    *fresh3 = (*fresh3 as core::ffi::c_int | bit << q) as uint8_t;
    (*wb).bit_offset = (off + 1 as core::ffi::c_int) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_write_literal(
    mut wb: *mut aom_write_bit_buffer,
    mut data: core::ffi::c_int,
    mut bits: core::ffi::c_int,
) {
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        aom_wb_write_bit(wb, data >> bit & 1 as core::ffi::c_int);
        bit -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_write_unsigned_literal(
    mut wb: *mut aom_write_bit_buffer,
    mut data: uint32_t,
    mut bits: core::ffi::c_int,
) {
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        aom_wb_write_bit(wb, (data >> bit & 1 as uint32_t) as core::ffi::c_int);
        bit -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_overwrite_literal(
    mut wb: *mut aom_write_bit_buffer,
    mut data: core::ffi::c_int,
    mut bits: core::ffi::c_int,
) {
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        overwrite_bit(wb, data >> bit & 1 as core::ffi::c_int);
        bit -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_write_inv_signed_literal(
    mut wb: *mut aom_write_bit_buffer,
    mut data: core::ffi::c_int,
    mut bits: core::ffi::c_int,
) {
    aom_wb_write_literal(wb, data, bits + 1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_write_uvlc(
    mut wb: *mut aom_write_bit_buffer,
    mut v: uint32_t,
) {
    v = v.wrapping_add(1);
    let leading_zeroes: core::ffi::c_int = get_msb(v as core::ffi::c_uint)
        as core::ffi::c_int;
    aom_wb_write_literal(wb, 0 as core::ffi::c_int, leading_zeroes);
    aom_wb_write_unsigned_literal(wb, v, leading_zeroes + 1 as core::ffi::c_int);
}
unsafe extern "C-unwind" fn wb_write_primitive_quniform(
    mut wb: *mut aom_write_bit_buffer,
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
        aom_wb_write_literal(wb, v as core::ffi::c_int, l - 1 as core::ffi::c_int);
    } else {
        aom_wb_write_literal(
            wb,
            m + (v as core::ffi::c_int - m >> 1 as core::ffi::c_int),
            l - 1 as core::ffi::c_int,
        );
        aom_wb_write_bit(wb, v as core::ffi::c_int - m & 1 as core::ffi::c_int);
    };
}
unsafe extern "C-unwind" fn wb_write_primitive_subexpfin(
    mut wb: *mut aom_write_bit_buffer,
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
            wb_write_primitive_quniform(
                wb,
                (n as core::ffi::c_int - mk) as uint16_t,
                (v as core::ffi::c_int - mk) as uint16_t,
            );
            break;
        } else {
            let mut t: core::ffi::c_int = (v as core::ffi::c_int >= mk + a)
                as core::ffi::c_int;
            aom_wb_write_bit(wb, t);
            if t != 0 {
                i = i + 1 as core::ffi::c_int;
                mk += a;
            } else {
                aom_wb_write_literal(wb, v as core::ffi::c_int - mk, b);
                break;
            }
        }
    };
}
unsafe extern "C-unwind" fn wb_write_primitive_refsubexpfin(
    mut wb: *mut aom_write_bit_buffer,
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: uint16_t,
    mut v: uint16_t,
) {
    wb_write_primitive_subexpfin(wb, n, k, recenter_finite_nonneg(n, ref_0, v));
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wb_write_signed_primitive_refsubexpfin(
    mut wb: *mut aom_write_bit_buffer,
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: int16_t,
    mut v: int16_t,
) {
    ref_0 = (ref_0 as core::ffi::c_int + (n as core::ffi::c_int - 1 as core::ffi::c_int))
        as int16_t;
    v = (v as core::ffi::c_int + (n as core::ffi::c_int - 1 as core::ffi::c_int))
        as int16_t;
    let scaled_n: uint16_t = (((n as core::ffi::c_int) << 1 as core::ffi::c_int)
        - 1 as core::ffi::c_int) as uint16_t;
    wb_write_primitive_refsubexpfin(wb, scaled_n, k, ref_0 as uint16_t, v as uint16_t);
}
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const CHAR_BIT: core::ffi::c_int = __CHAR_BIT__;
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
pub const __CHAR_BIT__: core::ffi::c_int = 8 as core::ffi::c_int;
