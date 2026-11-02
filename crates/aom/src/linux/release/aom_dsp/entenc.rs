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

// Mechanically translated from upstream/aom/aom_dsp/entenc.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn od_ec_tell_frac(nbits_total: uint32_t, rng: uint32_t) -> uint32_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const EC_PROB_SHIFT: core::ffi::c_int = 6 as core::ffi::c_int;
pub const EC_MIN_PROB: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn BSwap64(mut x: uint64_t) -> uint64_t {
    return x.swap_bytes();
}
pub const CDF_PROB_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const CDF_PROB_TOP: core::ffi::c_int = (1 as core::ffi::c_int) << CDF_PROB_BITS;
#[inline]
unsafe extern "C-unwind" fn propagate_carry_bwd(
    mut buf: *mut core::ffi::c_uchar,
    mut offs: uint32_t,
) {
    let mut sum: uint16_t = 0;
    let mut carry: uint16_t = 1 as uint16_t;
    loop {
        sum = (*buf.offset(offs as isize) as uint16_t as core::ffi::c_int
            + 1 as core::ffi::c_int) as uint16_t;
        let fresh0 = offs;
        offs = offs.wrapping_sub(1);
        *buf.offset(fresh0 as isize) = sum as core::ffi::c_uchar;
        carry = (sum as core::ffi::c_int >> 8 as core::ffi::c_int) as uint16_t;
        if !(carry != 0) {
            break;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn write_enc_data_to_out_buf(
    mut out: *mut core::ffi::c_uchar,
    mut offs: uint32_t,
    mut output: uint64_t,
    mut carry: uint64_t,
    mut enc_offs: *mut uint32_t,
    mut num_bytes_ready: uint8_t,
) {
    let reg: uint64_t = BSwap64(
        output
            << ((8 as core::ffi::c_int - num_bytes_ready as core::ffi::c_int)
                << 3 as core::ffi::c_int),
    ) as uint64_t;
    memcpy(
        &raw mut *out.offset(offs as isize) as *mut core::ffi::c_uchar
            as *mut core::ffi::c_void,
        &reg as *const uint64_t as *const core::ffi::c_void,
        8 as size_t,
    );
    if carry != 0 {
        propagate_carry_bwd(out, offs.wrapping_sub(1 as uint32_t));
    }
    *enc_offs = offs.wrapping_add(num_bytes_ready as uint32_t);
}
unsafe extern "C-unwind" fn od_ec_enc_normalize(
    mut enc: *mut od_ec_enc,
    mut low: od_ec_enc_window,
    mut rng: core::ffi::c_uint,
) {
    let mut d: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut s: core::ffi::c_int = 0;
    if (*enc).error != 0 {
        return;
    }
    c = (*enc).cnt as core::ffi::c_int;
    d = 16 as core::ffi::c_int - (1 as core::ffi::c_int + get_msb(rng));
    s = c + d;
    if s >= 40 as core::ffi::c_int {
        let mut out: *mut core::ffi::c_uchar = (*enc).buf;
        let mut storage: uint32_t = (*enc).storage;
        let mut offs: uint32_t = (*enc).offs;
        if offs.wrapping_add(8 as uint32_t) > storage {
            storage = (2 as uint32_t).wrapping_mul(storage).wrapping_add(8 as uint32_t);
            out = realloc(
                out as *mut core::ffi::c_void,
                (::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
                    .wrapping_mul(storage as size_t),
            ) as *mut core::ffi::c_uchar;
            if out.is_null() {
                (*enc).error = -(1 as core::ffi::c_int);
                return;
            }
            (*enc).buf = out;
            (*enc).storage = storage;
        }
        let mut num_bytes_ready: uint8_t = ((s >> 3 as core::ffi::c_int)
            + 1 as core::ffi::c_int) as uint8_t;
        c
            += 24 as core::ffi::c_int
                - ((num_bytes_ready as core::ffi::c_int) << 3 as core::ffi::c_int);
        let mut output: uint64_t = low as uint64_t >> c;
        low = low
            & ((1 as core::ffi::c_int as od_ec_enc_window) << c)
                .wrapping_sub(1 as od_ec_enc_window);
        let mut mask: uint64_t = (1 as core::ffi::c_int as uint64_t)
            << ((num_bytes_ready as core::ffi::c_int) << 3 as core::ffi::c_int);
        let mut carry: uint64_t = output & mask;
        mask = mask.wrapping_sub(0x1 as uint64_t);
        output = output & mask;
        write_enc_data_to_out_buf(
            out,
            offs,
            output,
            carry,
            &raw mut (*enc).offs,
            num_bytes_ready,
        );
        s = c + d - 24 as core::ffi::c_int;
    }
    (*enc).low = low << d;
    (*enc).rng = (rng << d) as uint16_t;
    (*enc).cnt = s as int16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_enc_init(mut enc: *mut od_ec_enc, mut size: uint32_t) {
    od_ec_enc_reset(enc);
    (*enc).buf = malloc(
        (::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
            .wrapping_mul(size as size_t),
    ) as *mut core::ffi::c_uchar;
    (*enc).storage = size;
    if size > 0 as uint32_t && ((*enc).buf).is_null() {
        (*enc).storage = 0 as uint32_t;
        (*enc).error = -(1 as core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_enc_reset(mut enc: *mut od_ec_enc) {
    (*enc).offs = 0 as uint32_t;
    (*enc).low = 0 as od_ec_enc_window;
    (*enc).rng = 0x8000 as uint16_t;
    (*enc).cnt = -(9 as core::ffi::c_int) as int16_t;
    (*enc).error = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_enc_clear(mut enc: *mut od_ec_enc) {
    free((*enc).buf as *mut core::ffi::c_void);
}
unsafe extern "C-unwind" fn od_ec_encode_q15(
    mut enc: *mut od_ec_enc,
    mut fl: core::ffi::c_uint,
    mut fh: core::ffi::c_uint,
    mut s: core::ffi::c_int,
    mut nsyms: core::ffi::c_int,
) {
    let mut l: od_ec_enc_window = 0;
    let mut r: core::ffi::c_uint = 0;
    let mut u: core::ffi::c_uint = 0;
    let mut v: core::ffi::c_uint = 0;
    l = (*enc).low;
    r = (*enc).rng as core::ffi::c_uint;
    let N: core::ffi::c_int = nsyms - 1 as core::ffi::c_int;
    if fl < CDF_PROB_TOP as core::ffi::c_uint {
        u = ((r as uint32_t >> 8 as core::ffi::c_int)
            .wrapping_mul((fl >> EC_PROB_SHIFT) as uint32_t)
            >> 7 as core::ffi::c_int - EC_PROB_SHIFT)
            .wrapping_add((EC_MIN_PROB * (N - (s - 1 as core::ffi::c_int))) as uint32_t)
            as core::ffi::c_uint;
        v = ((r as uint32_t >> 8 as core::ffi::c_int)
            .wrapping_mul((fh >> EC_PROB_SHIFT) as uint32_t)
            >> 7 as core::ffi::c_int - EC_PROB_SHIFT)
            .wrapping_add((EC_MIN_PROB * (N - (s + 0 as core::ffi::c_int))) as uint32_t)
            as core::ffi::c_uint;
        l = l.wrapping_add(r.wrapping_sub(u) as od_ec_enc_window);
        r = u.wrapping_sub(v);
    } else {
        r = r
            .wrapping_sub(
                ((r as uint32_t >> 8 as core::ffi::c_int)
                    .wrapping_mul((fh >> EC_PROB_SHIFT) as uint32_t)
                    >> 7 as core::ffi::c_int - EC_PROB_SHIFT)
                    .wrapping_add(
                        (EC_MIN_PROB * (N - (s + 0 as core::ffi::c_int))) as uint32_t,
                    ) as core::ffi::c_uint,
            );
    }
    od_ec_enc_normalize(enc, l, r);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_encode_bool_q15(
    mut enc: *mut od_ec_enc,
    mut val: core::ffi::c_int,
    mut f: core::ffi::c_uint,
) {
    let mut l: od_ec_enc_window = 0;
    let mut r: core::ffi::c_uint = 0;
    let mut v: core::ffi::c_uint = 0;
    l = (*enc).low;
    r = (*enc).rng as core::ffi::c_uint;
    v = ((r as uint32_t >> 8 as core::ffi::c_int)
        .wrapping_mul((f >> EC_PROB_SHIFT) as uint32_t)
        >> 7 as core::ffi::c_int - EC_PROB_SHIFT) as core::ffi::c_uint;
    v = v.wrapping_add(EC_MIN_PROB as core::ffi::c_uint);
    if val != 0 {
        l = l.wrapping_add(r.wrapping_sub(v) as od_ec_enc_window);
    }
    r = if val != 0 { v } else { r.wrapping_sub(v) };
    od_ec_enc_normalize(enc, l, r);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_encode_cdf_q15(
    mut enc: *mut od_ec_enc,
    mut s: core::ffi::c_int,
    mut icdf: *const uint16_t,
    mut nsyms: core::ffi::c_int,
) {
    od_ec_encode_q15(
        enc,
        (if s > 0 as core::ffi::c_int {
            *icdf.offset((s - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        } else {
            CDF_PROB_TOP - 0 as core::ffi::c_int
        }) as core::ffi::c_uint,
        *icdf.offset(s as isize) as core::ffi::c_uint,
        s,
        nsyms,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_enc_done(
    mut enc: *mut od_ec_enc,
    mut nbytes: *mut uint32_t,
) -> *mut core::ffi::c_uchar {
    let mut out: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    let mut storage: uint32_t = 0;
    let mut offs: uint32_t = 0;
    let mut m: od_ec_enc_window = 0;
    let mut e: od_ec_enc_window = 0;
    let mut l: od_ec_enc_window = 0;
    let mut c: core::ffi::c_int = 0;
    let mut s: core::ffi::c_int = 0;
    if (*enc).error != 0 {
        return 0 as *mut core::ffi::c_uchar;
    }
    l = (*enc).low;
    c = (*enc).cnt as core::ffi::c_int;
    s = 10 as core::ffi::c_int;
    m = 0x3fff as od_ec_enc_window;
    e = l.wrapping_add(m) & !m | m.wrapping_add(1 as od_ec_enc_window);
    s += c;
    offs = (*enc).offs;
    out = (*enc).buf;
    storage = (*enc).storage;
    let s_bits: core::ffi::c_int = s + 7 as core::ffi::c_int >> 3 as core::ffi::c_int;
    let mut b: core::ffi::c_int = if s_bits > 0 as core::ffi::c_int {
        s_bits
    } else {
        0 as core::ffi::c_int
    };
    if offs.wrapping_add(b as uint32_t) > storage {
        storage = offs.wrapping_add(b as uint32_t);
        out = realloc(
            out as *mut core::ffi::c_void,
            (::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
                .wrapping_mul(storage as size_t),
        ) as *mut core::ffi::c_uchar;
        if out.is_null() {
            (*enc).error = -(1 as core::ffi::c_int);
            return 0 as *mut core::ffi::c_uchar;
        }
        (*enc).buf = out;
        (*enc).storage = storage;
    }
    if s > 0 as core::ffi::c_int {
        let mut n: uint64_t = 0;
        n = ((1 as core::ffi::c_int as uint64_t) << c + 16 as core::ffi::c_int)
            .wrapping_sub(1 as uint64_t);
        loop {
            let mut val: uint16_t = (e >> c + 16 as core::ffi::c_int) as uint16_t;
            *out.offset(offs as isize) = (val as core::ffi::c_int
                & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
            if val as core::ffi::c_int & 0x100 as core::ffi::c_int != 0 {
                propagate_carry_bwd(out, offs.wrapping_sub(1 as uint32_t));
            }
            offs = offs.wrapping_add(1);
            e &= n;
            s -= 8 as core::ffi::c_int;
            c -= 8 as core::ffi::c_int;
            n >>= 8 as core::ffi::c_int;
            if !(s > 0 as core::ffi::c_int) {
                break;
            }
        }
    }
    *nbytes = offs;
    return out;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_enc_tell(mut enc: *const od_ec_enc) -> core::ffi::c_int {
    return (((*enc).cnt as core::ffi::c_int + 10 as core::ffi::c_int) as uint32_t)
        .wrapping_add(((*enc).offs).wrapping_mul(8 as uint32_t)) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_enc_tell_frac(mut enc: *const od_ec_enc) -> uint32_t {
    return od_ec_tell_frac(od_ec_enc_tell(enc) as uint32_t, (*enc).rng as uint32_t);
}

extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
