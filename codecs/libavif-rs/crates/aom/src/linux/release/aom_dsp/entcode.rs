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

// Mechanically translated from upstream/aom/aom_dsp/entcode.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub const OD_BITRES: core::ffi::c_int = 3 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn od_ec_tell_frac(
    mut nbits_total: uint32_t,
    mut rng: uint32_t,
) -> uint32_t {
    let mut nbits: uint32_t = 0;
    let mut l: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    nbits = nbits_total << OD_BITRES;
    l = 0 as core::ffi::c_int;
    i = OD_BITRES;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 > 0 as core::ffi::c_int) {
            break;
        }
        let mut b: core::ffi::c_int = 0;
        rng = rng.wrapping_mul(rng) >> 15 as core::ffi::c_int;
        b = (rng >> 16 as core::ffi::c_int) as core::ffi::c_int;
        l = l << 1 as core::ffi::c_int | b;
        rng >>= b;
    }
    return nbits.wrapping_sub(l as uint32_t);
}
