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

// Mechanically translated from upstream/aom/aom_dsp/bitreader_buffer.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type int16_t = i16;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type size_t = usize;
pub type aom_rb_error_handler = Option<
    unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_read_bit_buffer {
    pub bit_buffer: *const uint8_t,
    pub bit_buffer_end: *const uint8_t,
    pub bit_offset: uint32_t,
    pub error_handler_data: *mut core::ffi::c_void,
    pub error_handler: aom_rb_error_handler,
}
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
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
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    if !(n != 0 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<[u8; 8], [core::ffi::c_char; 8]>(*b"get_msb\0"))
                .as_ptr(),
            b"bitops.h\0" as *const u8 as *const core::ffi::c_char,
            43 as core::ffi::c_int,
            b"n != 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_rb_bytes_read(
    mut rb: *const aom_read_bit_buffer,
) -> size_t {
    return (((*rb).bit_offset).wrapping_add(7 as uint32_t) >> 3 as core::ffi::c_int)
        as size_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_rb_read_bit(
    mut rb: *mut aom_read_bit_buffer,
) -> core::ffi::c_int {
    let off: uint32_t = (*rb).bit_offset;
    let p: uint32_t = off >> 3 as core::ffi::c_int;
    let q: core::ffi::c_int = 7 as core::ffi::c_int
        - (off & 0x7 as uint32_t) as core::ffi::c_int;
    if ((*rb).bit_buffer).offset(p as isize) < (*rb).bit_buffer_end {
        let bit: core::ffi::c_int = *((*rb).bit_buffer).offset(p as isize)
            as core::ffi::c_int >> q & 1 as core::ffi::c_int;
        (*rb).bit_offset = off.wrapping_add(1 as uint32_t);
        return bit;
    } else {
        if ((*rb).error_handler).is_some() {
            ((*rb).error_handler)
                .expect("non-null function pointer")((*rb).error_handler_data);
        }
        return 0 as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_rb_read_literal(
    mut rb: *mut aom_read_bit_buffer,
    mut bits: core::ffi::c_int,
) -> core::ffi::c_int {
    if !(bits <= 31 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"aom_rb_read_literal\0"))
                .as_ptr(),
            b"bitreader_buffer.c\0" as *const u8 as *const core::ffi::c_char,
            40 as core::ffi::c_int,
            b"bits <= 31\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut value: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        value |= aom_rb_read_bit(rb) << bit;
        bit -= 1;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_rb_read_unsigned_literal(
    mut rb: *mut aom_read_bit_buffer,
    mut bits: core::ffi::c_int,
) -> uint32_t {
    if !(bits <= 32 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"aom_rb_read_unsigned_literal\0"))
                .as_ptr(),
            b"bitreader_buffer.c\0" as *const u8 as *const core::ffi::c_char,
            48 as core::ffi::c_int,
            b"bits <= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut value: uint32_t = 0 as uint32_t;
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        value |= (aom_rb_read_bit(rb) as uint32_t) << bit;
        bit -= 1;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_rb_read_inv_signed_literal(
    mut rb: *mut aom_read_bit_buffer,
    mut bits: core::ffi::c_int,
) -> core::ffi::c_int {
    let nbits: core::ffi::c_int = (::core::mem::size_of::<core::ffi::c_uint>() as usize)
        .wrapping_mul(8 as usize)
        .wrapping_sub(bits as usize)
        .wrapping_sub(1 as usize) as core::ffi::c_int;
    let value: core::ffi::c_uint = (aom_rb_read_literal(rb, bits + 1 as core::ffi::c_int)
        as core::ffi::c_uint) << nbits;
    return value as core::ffi::c_int >> nbits;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_rb_read_uvlc(mut rb: *mut aom_read_bit_buffer) -> uint32_t {
    let mut leading_zeros: core::ffi::c_int = 0 as core::ffi::c_int;
    while leading_zeros < 32 as core::ffi::c_int && aom_rb_read_bit(rb) == 0 {
        leading_zeros += 1;
    }
    if leading_zeros == 32 as core::ffi::c_int {
        return UINT32_MAX as uint32_t;
    }
    let base: uint32_t = ((1 as uint32_t) << leading_zeros).wrapping_sub(1 as uint32_t);
    let value: uint32_t = aom_rb_read_literal(rb, leading_zeros) as uint32_t;
    return base.wrapping_add(value);
}
unsafe extern "C-unwind" fn aom_rb_read_primitive_quniform(
    mut rb: *mut aom_read_bit_buffer,
    mut n: uint16_t,
) -> uint16_t {
    if n as core::ffi::c_int <= 1 as core::ffi::c_int {
        return 0 as uint16_t;
    }
    let l: core::ffi::c_int = get_msb(n as core::ffi::c_uint) as core::ffi::c_int
        + 1 as core::ffi::c_int;
    let m: core::ffi::c_int = ((1 as core::ffi::c_int) << l) - n as core::ffi::c_int;
    let v: core::ffi::c_int = aom_rb_read_literal(rb, l - 1 as core::ffi::c_int)
        as core::ffi::c_int;
    return (if v < m {
        v
    } else {
        (v << 1 as core::ffi::c_int) - m + aom_rb_read_bit(rb)
    }) as uint16_t;
}
unsafe extern "C-unwind" fn aom_rb_read_primitive_subexpfin(
    mut rb: *mut aom_read_bit_buffer,
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
            return (aom_rb_read_primitive_quniform(
                rb,
                (n as core::ffi::c_int - mk) as uint16_t,
            ) as core::ffi::c_int + mk) as uint16_t;
        }
        if aom_rb_read_bit(rb) == 0 {
            return (aom_rb_read_literal(rb, b) + mk) as uint16_t;
        }
        i = i + 1 as core::ffi::c_int;
        mk += a;
    };
}
unsafe extern "C-unwind" fn aom_rb_read_primitive_refsubexpfin(
    mut rb: *mut aom_read_bit_buffer,
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: uint16_t,
) -> uint16_t {
    return inv_recenter_finite_nonneg(
        n,
        ref_0,
        aom_rb_read_primitive_subexpfin(rb, n, k),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_rb_read_signed_primitive_refsubexpfin(
    mut rb: *mut aom_read_bit_buffer,
    mut n: uint16_t,
    mut k: uint16_t,
    mut ref_0: int16_t,
) -> int16_t {
    ref_0 = (ref_0 as core::ffi::c_int + (n as core::ffi::c_int - 1 as core::ffi::c_int))
        as int16_t;
    let scaled_n: uint16_t = (((n as core::ffi::c_int) << 1 as core::ffi::c_int)
        - 1 as core::ffi::c_int) as uint16_t;
    return (aom_rb_read_primitive_refsubexpfin(rb, scaled_n, k, ref_0 as uint16_t)
        as core::ffi::c_int - n as core::ffi::c_int + 1 as core::ffi::c_int) as int16_t;
}

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_get_msb(n: core::ffi::c_uint) -> core::ffi::c_int {
    get_msb(n)
}
