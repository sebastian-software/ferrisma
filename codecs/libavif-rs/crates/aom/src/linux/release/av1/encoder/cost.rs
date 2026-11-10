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

// Mechanically translated from upstream/aom/av1/encoder/cost.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type aom_cdf_prob = uint16_t;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const CDF_PROB_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const CDF_PROB_TOP: core::ffi::c_int = (1 as core::ffi::c_int) << CDF_PROB_BITS;
#[inline]
unsafe extern "C-unwind" fn get_prob(
    mut num: core::ffi::c_uint,
    mut den: core::ffi::c_uint,
) -> uint8_t {
    let p: core::ffi::c_int = (num as uint64_t)
        .wrapping_mul(256 as uint64_t)
        .wrapping_add((den >> 1 as core::ffi::c_int) as uint64_t)
        .wrapping_div(den as uint64_t) as core::ffi::c_int;
    let clipped_prob: core::ffi::c_int = p
        | 255 as core::ffi::c_int - p >> 23 as core::ffi::c_int
        | (p == 0 as core::ffi::c_int) as core::ffi::c_int;
    return clipped_prob as uint8_t;
}
pub const EC_MIN_PROB: core::ffi::c_int = 4 as core::ffi::c_int;
pub const AV1_PROB_COST_SHIFT: core::ffi::c_int = 9 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_cost_symbol(mut p15: aom_cdf_prob) -> core::ffi::c_int {
    p15 = clamp(
        p15 as core::ffi::c_int,
        1 as core::ffi::c_int,
        CDF_PROB_TOP - 1 as core::ffi::c_int,
    ) as aom_cdf_prob;
    let shift: core::ffi::c_int = CDF_PROB_BITS - 1 as core::ffi::c_int
        - get_msb(p15 as core::ffi::c_uint) as core::ffi::c_int;
    let prob: core::ffi::c_int = get_prob(
        ((p15 as core::ffi::c_int) << shift) as core::ffi::c_uint,
        CDF_PROB_TOP as core::ffi::c_uint,
    ) as core::ffi::c_int;
    return av1_prob_cost[(prob - 128 as core::ffi::c_int) as usize] as core::ffi::c_int
        + shift * ((1 as core::ffi::c_int) << AV1_PROB_COST_SHIFT);
}
#[no_mangle]
pub static mut av1_prob_cost: [uint16_t; 128] = [
    512 as core::ffi::c_int as uint16_t,
    506 as core::ffi::c_int as uint16_t,
    501 as core::ffi::c_int as uint16_t,
    495 as core::ffi::c_int as uint16_t,
    489 as core::ffi::c_int as uint16_t,
    484 as core::ffi::c_int as uint16_t,
    478 as core::ffi::c_int as uint16_t,
    473 as core::ffi::c_int as uint16_t,
    467 as core::ffi::c_int as uint16_t,
    462 as core::ffi::c_int as uint16_t,
    456 as core::ffi::c_int as uint16_t,
    451 as core::ffi::c_int as uint16_t,
    446 as core::ffi::c_int as uint16_t,
    441 as core::ffi::c_int as uint16_t,
    435 as core::ffi::c_int as uint16_t,
    430 as core::ffi::c_int as uint16_t,
    425 as core::ffi::c_int as uint16_t,
    420 as core::ffi::c_int as uint16_t,
    415 as core::ffi::c_int as uint16_t,
    410 as core::ffi::c_int as uint16_t,
    405 as core::ffi::c_int as uint16_t,
    400 as core::ffi::c_int as uint16_t,
    395 as core::ffi::c_int as uint16_t,
    390 as core::ffi::c_int as uint16_t,
    385 as core::ffi::c_int as uint16_t,
    380 as core::ffi::c_int as uint16_t,
    375 as core::ffi::c_int as uint16_t,
    371 as core::ffi::c_int as uint16_t,
    366 as core::ffi::c_int as uint16_t,
    361 as core::ffi::c_int as uint16_t,
    356 as core::ffi::c_int as uint16_t,
    352 as core::ffi::c_int as uint16_t,
    347 as core::ffi::c_int as uint16_t,
    343 as core::ffi::c_int as uint16_t,
    338 as core::ffi::c_int as uint16_t,
    333 as core::ffi::c_int as uint16_t,
    329 as core::ffi::c_int as uint16_t,
    324 as core::ffi::c_int as uint16_t,
    320 as core::ffi::c_int as uint16_t,
    316 as core::ffi::c_int as uint16_t,
    311 as core::ffi::c_int as uint16_t,
    307 as core::ffi::c_int as uint16_t,
    302 as core::ffi::c_int as uint16_t,
    298 as core::ffi::c_int as uint16_t,
    294 as core::ffi::c_int as uint16_t,
    289 as core::ffi::c_int as uint16_t,
    285 as core::ffi::c_int as uint16_t,
    281 as core::ffi::c_int as uint16_t,
    277 as core::ffi::c_int as uint16_t,
    273 as core::ffi::c_int as uint16_t,
    268 as core::ffi::c_int as uint16_t,
    264 as core::ffi::c_int as uint16_t,
    260 as core::ffi::c_int as uint16_t,
    256 as core::ffi::c_int as uint16_t,
    252 as core::ffi::c_int as uint16_t,
    248 as core::ffi::c_int as uint16_t,
    244 as core::ffi::c_int as uint16_t,
    240 as core::ffi::c_int as uint16_t,
    236 as core::ffi::c_int as uint16_t,
    232 as core::ffi::c_int as uint16_t,
    228 as core::ffi::c_int as uint16_t,
    224 as core::ffi::c_int as uint16_t,
    220 as core::ffi::c_int as uint16_t,
    216 as core::ffi::c_int as uint16_t,
    212 as core::ffi::c_int as uint16_t,
    209 as core::ffi::c_int as uint16_t,
    205 as core::ffi::c_int as uint16_t,
    201 as core::ffi::c_int as uint16_t,
    197 as core::ffi::c_int as uint16_t,
    194 as core::ffi::c_int as uint16_t,
    190 as core::ffi::c_int as uint16_t,
    186 as core::ffi::c_int as uint16_t,
    182 as core::ffi::c_int as uint16_t,
    179 as core::ffi::c_int as uint16_t,
    175 as core::ffi::c_int as uint16_t,
    171 as core::ffi::c_int as uint16_t,
    168 as core::ffi::c_int as uint16_t,
    164 as core::ffi::c_int as uint16_t,
    161 as core::ffi::c_int as uint16_t,
    157 as core::ffi::c_int as uint16_t,
    153 as core::ffi::c_int as uint16_t,
    150 as core::ffi::c_int as uint16_t,
    146 as core::ffi::c_int as uint16_t,
    143 as core::ffi::c_int as uint16_t,
    139 as core::ffi::c_int as uint16_t,
    136 as core::ffi::c_int as uint16_t,
    132 as core::ffi::c_int as uint16_t,
    129 as core::ffi::c_int as uint16_t,
    125 as core::ffi::c_int as uint16_t,
    122 as core::ffi::c_int as uint16_t,
    119 as core::ffi::c_int as uint16_t,
    115 as core::ffi::c_int as uint16_t,
    112 as core::ffi::c_int as uint16_t,
    109 as core::ffi::c_int as uint16_t,
    105 as core::ffi::c_int as uint16_t,
    102 as core::ffi::c_int as uint16_t,
    99 as core::ffi::c_int as uint16_t,
    95 as core::ffi::c_int as uint16_t,
    92 as core::ffi::c_int as uint16_t,
    89 as core::ffi::c_int as uint16_t,
    86 as core::ffi::c_int as uint16_t,
    82 as core::ffi::c_int as uint16_t,
    79 as core::ffi::c_int as uint16_t,
    76 as core::ffi::c_int as uint16_t,
    73 as core::ffi::c_int as uint16_t,
    70 as core::ffi::c_int as uint16_t,
    66 as core::ffi::c_int as uint16_t,
    63 as core::ffi::c_int as uint16_t,
    60 as core::ffi::c_int as uint16_t,
    57 as core::ffi::c_int as uint16_t,
    54 as core::ffi::c_int as uint16_t,
    51 as core::ffi::c_int as uint16_t,
    48 as core::ffi::c_int as uint16_t,
    45 as core::ffi::c_int as uint16_t,
    42 as core::ffi::c_int as uint16_t,
    38 as core::ffi::c_int as uint16_t,
    35 as core::ffi::c_int as uint16_t,
    32 as core::ffi::c_int as uint16_t,
    29 as core::ffi::c_int as uint16_t,
    26 as core::ffi::c_int as uint16_t,
    23 as core::ffi::c_int as uint16_t,
    20 as core::ffi::c_int as uint16_t,
    18 as core::ffi::c_int as uint16_t,
    15 as core::ffi::c_int as uint16_t,
    12 as core::ffi::c_int as uint16_t,
    9 as core::ffi::c_int as uint16_t,
    6 as core::ffi::c_int as uint16_t,
    3 as core::ffi::c_int as uint16_t,
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cost_tokens_from_cdf(
    mut costs: *mut core::ffi::c_int,
    mut cdf: *const aom_cdf_prob,
    mut inv_map: *const core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut prev_cdf: aom_cdf_prob = 0 as aom_cdf_prob;
    i = 0 as core::ffi::c_int;
    loop {
        let mut p15: aom_cdf_prob = (CDF_PROB_TOP
            - *cdf.offset(i as isize) as core::ffi::c_int - prev_cdf as core::ffi::c_int)
            as aom_cdf_prob;
        p15 = (if (p15 as core::ffi::c_int) < EC_MIN_PROB {
            EC_MIN_PROB
        } else {
            p15 as core::ffi::c_int
        }) as aom_cdf_prob;
        prev_cdf = (CDF_PROB_TOP - *cdf.offset(i as isize) as core::ffi::c_int)
            as aom_cdf_prob;
        if !inv_map.is_null() {
            *costs.offset(*inv_map.offset(i as isize) as isize) = av1_cost_symbol(p15);
        } else {
            *costs.offset(i as isize) = av1_cost_symbol(p15);
        }
        if *cdf.offset(i as isize) as core::ffi::c_int
            == CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)
        {
            break;
        }
        i += 1;
    };
}
