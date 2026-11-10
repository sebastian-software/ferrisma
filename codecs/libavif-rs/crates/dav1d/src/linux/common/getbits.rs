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

// Mechanically translated from upstream/dav1d/src/getbits.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
pub type __uint8_t = u8;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBits {
    pub state: uint64_t,
    pub bits_left: core::ffi::c_int,
    pub error: core::ffi::c_int,
    pub ptr: *const uint8_t,
    pub ptr_start: *const uint8_t,
    pub ptr_end: *const uint8_t,
}
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
#[inline]
unsafe extern "C" fn clz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn ulog2(v: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ clz(v);
}
#[inline]
unsafe extern "C" fn inv_recenter(
    r: core::ffi::c_uint,
    v: core::ffi::c_uint,
) -> core::ffi::c_uint {
    if v > r << 1 as core::ffi::c_int {
        return v
    } else if v & 1 as core::ffi::c_uint == 0 as core::ffi::c_uint {
        return (v >> 1 as core::ffi::c_int).wrapping_add(r)
    } else {
        return r
            .wrapping_sub(
                v.wrapping_add(1 as core::ffi::c_uint) >> 1 as core::ffi::c_int,
            )
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_init_get_bits(
    c: *mut GetBits,
    data: *const uint8_t,
    sz: size_t,
) {
    if sz != 0 {} else {
        __assert_fail(
            b"sz\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/getbits.c\0" as *const u8
                as *const core::ffi::c_char,
            39 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 77],
                [core::ffi::c_char; 77],
            >(
                *b"void dav1d_init_get_bits(GetBits *const, const uint8_t *const, const size_t)\0",
            ))
                .as_ptr(),
        );
    };
    (*c).ptr_start = data;
    (*c).ptr = (*c).ptr_start;
    (*c).ptr_end = &*((*c).ptr_start).offset(sz as isize) as *const uint8_t;
    (*c).state = 0 as uint64_t;
    (*c).bits_left = 0 as core::ffi::c_int;
    (*c).error = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_bit(c: *mut GetBits) -> core::ffi::c_uint {
    if (*c).bits_left == 0 {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1 as core::ffi::c_int;
        } else {
            let fresh0 = (*c).ptr;
            (*c).ptr = ((*c).ptr).offset(1);
            let state: core::ffi::c_uint = *fresh0 as core::ffi::c_uint;
            (*c).bits_left = 7 as core::ffi::c_int;
            (*c).state = (state as uint64_t) << 57 as core::ffi::c_int;
            return state >> 7 as core::ffi::c_int;
        }
    }
    let state_0: uint64_t = (*c).state;
    (*c).bits_left -= 1;
    (*c).state = state_0 << 1 as core::ffi::c_int;
    return (state_0 >> 63 as core::ffi::c_int) as core::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn refill(c: *mut GetBits, n: core::ffi::c_int) {
    if (*c).bits_left >= 0 as core::ffi::c_int && (*c).bits_left < 32 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"c->bits_left >= 0 && c->bits_left < 32\0" as *const u8
                as *const core::ffi::c_char,
            b"../dav1d-native-source/src/getbits.c\0" as *const u8
                as *const core::ffi::c_char,
            66 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"void refill(GetBits *const, const int)\0"))
                .as_ptr(),
        );
    };
    let mut state: core::ffi::c_uint = 0 as core::ffi::c_uint;
    loop {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1 as core::ffi::c_int;
            if state != 0 {
                break;
            }
            return;
        } else {
            let fresh1 = (*c).ptr;
            (*c).ptr = ((*c).ptr).offset(1);
            state = state << 8 as core::ffi::c_int | *fresh1 as core::ffi::c_uint;
            (*c).bits_left += 8 as core::ffi::c_int;
            if !(n > (*c).bits_left) {
                break;
            }
        }
    }
    (*c).state |= (state as uint64_t) << 64 as core::ffi::c_int - (*c).bits_left;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_bits(
    c: *mut GetBits,
    n: core::ffi::c_int,
) -> core::ffi::c_uint {
    if n > 0 as core::ffi::c_int && n <= 32 as core::ffi::c_int {} else {
        __assert_fail(
            b"n > 0 && n <= 32\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/getbits.c\0" as *const u8
                as *const core::ffi::c_char,
            92 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 55],
                [core::ffi::c_char; 55],
            >(*b"unsigned int dav1d_get_bits(GetBits *const, const int)\0"))
                .as_ptr(),
        );
    };
    if n as core::ffi::c_uint > (*c).bits_left as core::ffi::c_uint {
        refill(c, n);
    }
    let state: uint64_t = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    return (state >> 64 as core::ffi::c_int - n) as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_sbits(
    c: *mut GetBits,
    n: core::ffi::c_int,
) -> core::ffi::c_int {
    if n > 0 as core::ffi::c_int && n <= 32 as core::ffi::c_int {} else {
        __assert_fail(
            b"n > 0 && n <= 32\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/getbits.c\0" as *const u8
                as *const core::ffi::c_char,
            93 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 47],
                [core::ffi::c_char; 47],
            >(*b"int dav1d_get_sbits(GetBits *const, const int)\0"))
                .as_ptr(),
        );
    };
    if n as core::ffi::c_uint > (*c).bits_left as core::ffi::c_uint {
        refill(c, n);
    }
    let state: uint64_t = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    return (state as int64_t >> 64 as core::ffi::c_int - n) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_uleb128(c: *mut GetBits) -> core::ffi::c_uint {
    let mut val: uint64_t = 0 as uint64_t;
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut more: core::ffi::c_uint = 0;
    loop {
        let v: core::ffi::c_int = dav1d_get_bits(c, 8 as core::ffi::c_int)
            as core::ffi::c_int;
        more = (v & 0x80 as core::ffi::c_int) as core::ffi::c_uint;
        val |= ((v & 0x7f as core::ffi::c_int) as uint64_t) << i;
        i = i.wrapping_add(7 as core::ffi::c_uint);
        if !(more != 0 && i < 56 as core::ffi::c_uint) {
            break;
        }
    }
    if val > UINT32_MAX as uint64_t || more != 0 {
        (*c).error = 1 as core::ffi::c_int;
        return 0 as core::ffi::c_uint;
    }
    return val as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_uniform(
    c: *mut GetBits,
    max: core::ffi::c_uint,
) -> core::ffi::c_uint {
    if max > 1 as core::ffi::c_uint {} else {
        __assert_fail(
            b"max > 1\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/getbits.c\0" as *const u8
                as *const core::ffi::c_char,
            117 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 67],
                [core::ffi::c_char; 67],
            >(*b"unsigned int dav1d_get_uniform(GetBits *const, const unsigned int)\0"))
                .as_ptr(),
        );
    };
    let l: core::ffi::c_int = ulog2(max) as core::ffi::c_int + 1 as core::ffi::c_int;
    if l > 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"l > 1\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/getbits.c\0" as *const u8
                as *const core::ffi::c_char,
            119 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 67],
                [core::ffi::c_char; 67],
            >(*b"unsigned int dav1d_get_uniform(GetBits *const, const unsigned int)\0"))
                .as_ptr(),
        );
    };
    let m: core::ffi::c_uint = ((1 as core::ffi::c_uint) << l).wrapping_sub(max);
    let v: core::ffi::c_uint = dav1d_get_bits(c, l - 1 as core::ffi::c_int)
        as core::ffi::c_uint;
    return if v < m {
        v
    } else {
        (v << 1 as core::ffi::c_int).wrapping_sub(m).wrapping_add(dav1d_get_bit(c))
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_vlc(c: *mut GetBits) -> core::ffi::c_uint {
    if dav1d_get_bit(c) != 0 {
        return 0 as core::ffi::c_uint;
    }
    let mut n_bits: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        n_bits += 1;
        if n_bits == 32 as core::ffi::c_int {
            return UINT32_MAX;
        }
        if !(dav1d_get_bit(c) == 0) {
            break;
        }
    }
    return ((1 as core::ffi::c_uint) << n_bits)
        .wrapping_sub(1 as core::ffi::c_uint)
        .wrapping_add(dav1d_get_bits(c, n_bits));
}
unsafe extern "C" fn get_bits_subexp_u(
    c: *mut GetBits,
    ref_0: core::ffi::c_uint,
    n: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut v: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let b: core::ffi::c_int = if i != 0 {
            3 as core::ffi::c_int + i - 1 as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
        if n
            < v
                .wrapping_add(
                    (3 as core::ffi::c_int * ((1 as core::ffi::c_int) << b))
                        as core::ffi::c_uint,
                )
        {
            v = v
                .wrapping_add(
                    dav1d_get_uniform(
                        c,
                        n.wrapping_sub(v).wrapping_add(1 as core::ffi::c_uint),
                    ),
                );
            break;
        } else if dav1d_get_bit(c) == 0 {
            v = v.wrapping_add(dav1d_get_bits(c, b));
            break;
        } else {
            v = v.wrapping_add(((1 as core::ffi::c_int) << b) as core::ffi::c_uint);
            i += 1;
        }
    }
    return if ref_0.wrapping_mul(2 as core::ffi::c_uint) <= n {
        inv_recenter(ref_0, v)
    } else {
        n.wrapping_sub(inv_recenter(n.wrapping_sub(ref_0), v))
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_bits_subexp(
    c: *mut GetBits,
    ref_0: core::ffi::c_int,
    n: core::ffi::c_uint,
) -> core::ffi::c_int {
    return get_bits_subexp_u(
        c,
        (ref_0 + ((1 as core::ffi::c_int) << n)) as core::ffi::c_uint,
        ((2 as core::ffi::c_int) << n) as core::ffi::c_uint,
    ) as core::ffi::c_int - ((1 as core::ffi::c_int) << n);
}
