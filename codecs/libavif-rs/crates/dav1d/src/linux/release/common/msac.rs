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

// Mechanically translated from upstream/dav1d/src/msac.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = usize;
pub type ec_win = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MsacContext {
    pub buf_pos: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub dif: ec_win,
    pub rng: core::ffi::c_uint,
    pub cnt: core::ffi::c_int,
    pub allow_update_cdf: core::ffi::c_int,
}
#[inline]
unsafe extern "C" fn clz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
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
#[inline]
unsafe extern "C" fn dav1d_msac_decode_bools(
    s: *mut MsacContext,
    mut n: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut v: core::ffi::c_uint = 0 as core::ffi::c_uint;
    loop {
        let fresh5 = n;
        n = n.wrapping_sub(1);
        if !(fresh5 != 0) {
            break;
        }
        v = v << 1 as core::ffi::c_int | dav1d_msac_decode_bool_equi_c(s);
    }
    return v;
}
pub const EC_PROB_SHIFT: core::ffi::c_int = 6 as core::ffi::c_int;
pub const EC_MIN_PROB: core::ffi::c_int = 4 as core::ffi::c_int;
pub const EC_WIN_SIZE: usize = (::core::mem::size_of::<ec_win>() as usize)
    << 3 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ctx_refill(s: *mut MsacContext) {
    let mut buf_pos: *const uint8_t = (*s).buf_pos;
    let mut buf_end: *const uint8_t = (*s).buf_end;
    let mut c: core::ffi::c_int = EC_WIN_SIZE
        .wrapping_sub((*s).cnt as usize)
        .wrapping_sub(24 as usize) as core::ffi::c_int;
    let mut dif: ec_win = (*s).dif;
    loop {
        if buf_pos >= buf_end {
            dif |= !(!(0xff as core::ffi::c_int as ec_win) << c);
            break;
        } else {
            let fresh0 = buf_pos;
            buf_pos = buf_pos.offset(1);
            dif
                |= ((*fresh0 as core::ffi::c_int ^ 0xff as core::ffi::c_int) as ec_win)
                    << c;
            c -= 8 as core::ffi::c_int;
            if !(c >= 0 as core::ffi::c_int) {
                break;
            }
        }
    }
    (*s).dif = dif;
    (*s).cnt = EC_WIN_SIZE.wrapping_sub(c as usize).wrapping_sub(24 as usize)
        as core::ffi::c_int;
    (*s).buf_pos = buf_pos;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_subexp(
    s: *mut MsacContext,
    ref_0: core::ffi::c_int,
    n: core::ffi::c_int,
    mut k: core::ffi::c_uint,
) -> core::ffi::c_int {
    if !(n >> k == 8 as core::ffi::c_int) {
        unreachable!();
    }
    let mut a: core::ffi::c_uint = 0 as core::ffi::c_uint;
    if dav1d_msac_decode_bool_equi_c(s) != 0 {
        if dav1d_msac_decode_bool_equi_c(s) != 0 {
            k = k
                .wrapping_add(
                    (dav1d_msac_decode_bool_equi_c(s))
                        .wrapping_add(1 as core::ffi::c_uint),
                );
        }
        a = ((1 as core::ffi::c_int) << k) as core::ffi::c_uint;
    }
    let v: core::ffi::c_uint = (dav1d_msac_decode_bools(s, k) as core::ffi::c_uint)
        .wrapping_add(a);
    return (if ref_0 * 2 as core::ffi::c_int <= n {
        inv_recenter(ref_0 as core::ffi::c_uint, v)
    } else {
        ((n - 1 as core::ffi::c_int) as core::ffi::c_uint)
            .wrapping_sub(
                inv_recenter((n - 1 as core::ffi::c_int - ref_0) as core::ffi::c_uint, v),
            )
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ctx_norm(s: *mut MsacContext, dif: ec_win, rng: core::ffi::c_uint) {
    let d: core::ffi::c_int = 15 as core::ffi::c_int
        ^ (31 as core::ffi::c_int ^ clz(rng) as core::ffi::c_int);
    let cnt: core::ffi::c_int = (*s).cnt;
    if !(rng <= 65535 as core::ffi::c_uint) {
        unreachable!();
    }
    (*s).dif = dif << d;
    (*s).rng = rng << d;
    (*s).cnt = cnt - d;
    if (cnt as core::ffi::c_uint) < d as core::ffi::c_uint {
        ctx_refill(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_equi_c(
    s: *mut MsacContext,
) -> core::ffi::c_uint {
    let r: core::ffi::c_uint = (*s).rng;
    let mut dif: ec_win = (*s).dif;
    if !(dif
        >> ((::core::mem::size_of::<ec_win>() as usize) << 3 as core::ffi::c_int)
            .wrapping_sub(16 as usize) < r as ec_win)
    {
        unreachable!();
    }
    let mut v: core::ffi::c_uint = ((r >> 8 as core::ffi::c_int)
        << 7 as core::ffi::c_int)
        .wrapping_add(EC_MIN_PROB as core::ffi::c_uint);
    let vw: ec_win = (v as ec_win) << EC_WIN_SIZE.wrapping_sub(16 as usize);
    let ret: core::ffi::c_uint = (dif >= vw) as core::ffi::c_int as core::ffi::c_uint;
    dif = dif.wrapping_sub((ret as ec_win).wrapping_mul(vw));
    v = v
        .wrapping_add(
            ret.wrapping_mul(r.wrapping_sub((2 as core::ffi::c_uint).wrapping_mul(v))),
        );
    ctx_norm(s, dif, v);
    return (ret == 0) as core::ffi::c_int as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_c(
    s: *mut MsacContext,
    f: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let r: core::ffi::c_uint = (*s).rng;
    let mut dif: ec_win = (*s).dif;
    if !(dif
        >> ((::core::mem::size_of::<ec_win>() as usize) << 3 as core::ffi::c_int)
            .wrapping_sub(16 as usize) < r as ec_win)
    {
        unreachable!();
    }
    let mut v: core::ffi::c_uint = ((r >> 8 as core::ffi::c_int)
        .wrapping_mul(f >> EC_PROB_SHIFT) >> 7 as core::ffi::c_int - EC_PROB_SHIFT)
        .wrapping_add(EC_MIN_PROB as core::ffi::c_uint);
    let vw: ec_win = (v as ec_win) << EC_WIN_SIZE.wrapping_sub(16 as usize);
    let ret: core::ffi::c_uint = (dif >= vw) as core::ffi::c_int as core::ffi::c_uint;
    dif = dif.wrapping_sub((ret as ec_win).wrapping_mul(vw));
    v = v
        .wrapping_add(
            ret.wrapping_mul(r.wrapping_sub((2 as core::ffi::c_uint).wrapping_mul(v))),
        );
    ctx_norm(s, dif, v);
    return (ret == 0) as core::ffi::c_int as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt_c(
    s: *mut MsacContext,
    cdf: *mut uint16_t,
    n_symbols: size_t,
) -> core::ffi::c_uint {
    let c: core::ffi::c_uint = ((*s).dif >> EC_WIN_SIZE.wrapping_sub(16 as usize))
        as core::ffi::c_uint;
    let r: core::ffi::c_uint = (*s).rng >> 8 as core::ffi::c_int;
    let mut u: core::ffi::c_uint = 0;
    let mut v: core::ffi::c_uint = (*s).rng;
    let mut val: core::ffi::c_uint = -(1 as core::ffi::c_int) as core::ffi::c_uint;
    if !(n_symbols <= 15 as size_t) {
        unreachable!();
    }
    if !(*cdf.offset(n_symbols as isize) as core::ffi::c_int <= 32 as core::ffi::c_int) {
        unreachable!();
    }
    loop {
        val = val.wrapping_add(1);
        u = v;
        v = r
            .wrapping_mul(
                (*cdf.offset(val as isize) as core::ffi::c_int >> EC_PROB_SHIFT)
                    as core::ffi::c_uint,
            );
        v >>= 7 as core::ffi::c_int - EC_PROB_SHIFT;
        v = v
            .wrapping_add(
                (EC_MIN_PROB as core::ffi::c_uint)
                    .wrapping_mul((n_symbols as core::ffi::c_uint).wrapping_sub(val)),
            );
        if !(c < v) {
            break;
        }
    }
    if !(u <= (*s).rng) {
        unreachable!();
    }
    ctx_norm(
        s,
        ((*s).dif).wrapping_sub((v as ec_win) << EC_WIN_SIZE.wrapping_sub(16 as usize)),
        u.wrapping_sub(v),
    );
    if (*s).allow_update_cdf != 0 {
        let count: core::ffi::c_uint = *cdf.offset(n_symbols as isize)
            as core::ffi::c_uint;
        let rate: core::ffi::c_uint = (4 as core::ffi::c_uint)
            .wrapping_add(count >> 4 as core::ffi::c_int)
            .wrapping_add(
                (n_symbols > 2 as size_t) as core::ffi::c_int as core::ffi::c_uint,
            );
        let mut i: core::ffi::c_uint = 0;
        i = 0 as core::ffi::c_uint;
        while i < val {
            let fresh1 = &raw mut *cdf.offset(i as isize);
            *fresh1 = (*fresh1 as core::ffi::c_int
                + (32768 as core::ffi::c_int
                    - *cdf.offset(i as isize) as core::ffi::c_int >> rate)) as uint16_t;
            i = i.wrapping_add(1);
        }
        while (i as size_t) < n_symbols {
            let fresh2 = &raw mut *cdf.offset(i as isize);
            *fresh2 = (*fresh2 as core::ffi::c_int
                - (*cdf.offset(i as isize) as core::ffi::c_int >> rate)) as uint16_t;
            i = i.wrapping_add(1);
        }
        *cdf.offset(n_symbols as isize) = count
            .wrapping_add(
                (count < 32 as core::ffi::c_uint) as core::ffi::c_int
                    as core::ffi::c_uint,
            ) as uint16_t;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_adapt_c(
    s: *mut MsacContext,
    cdf: *mut uint16_t,
) -> core::ffi::c_uint {
    let bit: core::ffi::c_uint = dav1d_msac_decode_bool_c(s, *cdf as core::ffi::c_uint)
        as core::ffi::c_uint;
    if (*s).allow_update_cdf != 0 {
        let count: core::ffi::c_uint = *cdf.offset(1 as core::ffi::c_int as isize)
            as core::ffi::c_uint;
        let rate: core::ffi::c_int = (4 as core::ffi::c_uint)
            .wrapping_add(count >> 4 as core::ffi::c_int) as core::ffi::c_int;
        if bit != 0 {
            let fresh3 = &raw mut *cdf.offset(0 as core::ffi::c_int as isize);
            *fresh3 = (*fresh3 as core::ffi::c_int
                + (32768 as core::ffi::c_int
                    - *cdf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    >> rate)) as uint16_t;
        } else {
            let fresh4 = &raw mut *cdf.offset(0 as core::ffi::c_int as isize);
            *fresh4 = (*fresh4 as core::ffi::c_int
                - (*cdf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    >> rate)) as uint16_t;
        }
        *cdf.offset(1 as core::ffi::c_int as isize) = count
            .wrapping_add(
                (count < 32 as core::ffi::c_uint) as core::ffi::c_int
                    as core::ffi::c_uint,
            ) as uint16_t;
    }
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_hi_tok_c(
    s: *mut MsacContext,
    cdf: *mut uint16_t,
) -> core::ffi::c_uint {
    let mut tok_br: core::ffi::c_uint = dav1d_msac_decode_symbol_adapt_c(
        s,
        cdf,
        3 as size_t,
    );
    let mut tok: core::ffi::c_uint = (3 as core::ffi::c_uint).wrapping_add(tok_br);
    if tok_br == 3 as core::ffi::c_uint {
        tok_br = dav1d_msac_decode_symbol_adapt_c(s, cdf, 3 as size_t);
        tok = (6 as core::ffi::c_uint).wrapping_add(tok_br);
        if tok_br == 3 as core::ffi::c_uint {
            tok_br = dav1d_msac_decode_symbol_adapt_c(s, cdf, 3 as size_t);
            tok = (9 as core::ffi::c_uint).wrapping_add(tok_br);
            if tok_br == 3 as core::ffi::c_uint {
                tok = (12 as core::ffi::c_uint)
                    .wrapping_add(dav1d_msac_decode_symbol_adapt_c(s, cdf, 3 as size_t));
            }
        }
    }
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_init(
    s: *mut MsacContext,
    data: *const uint8_t,
    sz: size_t,
    disable_cdf_update_flag: core::ffi::c_int,
) {
    (*s).buf_pos = data;
    (*s).buf_end = data.offset(sz as isize);
    (*s).dif = 0 as ec_win;
    (*s).rng = 0x8000 as core::ffi::c_uint;
    (*s).cnt = -(15 as core::ffi::c_int);
    (*s).allow_update_cdf = (disable_cdf_update_flag == 0) as core::ffi::c_int;
    ctx_refill(s);
}
