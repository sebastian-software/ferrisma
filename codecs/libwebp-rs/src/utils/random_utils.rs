// libwebp v1.6.0, src/utils/random_utils.c:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Pseudo-random utilities
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/random_utils.h:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Pseudo-random utilities
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/utils/random_utils.c` and `src/utils/random_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).

// upstream: src/utils/random_utils.h VP8_RANDOM_DITHER_FIX
/// Fixed-point precision of the dithering amplitude.
pub const VP8_RANDOM_DITHER_FIX: i32 = 8;
// upstream: src/utils/random_utils.h VP8_RANDOM_TABLE_SIZE
/// Number of entries of the Knuth generator table.
pub const VP8_RANDOM_TABLE_SIZE: usize = 55;

// upstream: src/utils/random_utils.c kRandomTable
/// Initial table of the generator, 31-bit values.
pub const RANDOM_TABLE: [u32; VP8_RANDOM_TABLE_SIZE] = [
    0x0de1_5230,
    0x03b3_1886,
    0x775f_accb,
    0x1c88_626a,
    0x6838_5c55,
    0x14b3_b828,
    0x4a85_fef8,
    0x49dd_b84b,
    0x64fc_f397,
    0x5c55_0289,
    0x4a29_0000,
    0x0d7e_c1da,
    0x5940_b7ab,
    0x5492_577d,
    0x4e19_ca72,
    0x38d3_8c69,
    0x0c01_ee65,
    0x32a1_755f,
    0x5437_f652,
    0x5abb_2c32,
    0x0faa_57b1,
    0x73f5_33e7,
    0x685f_eeda,
    0x7563_cce2,
    0x6e99_0e83,
    0x4730_a7ed,
    0x4fc0_d9c6,
    0x496b_153c,
    0x4f14_03fa,
    0x541a_fb0c,
    0x7399_0b32,
    0x26d7_cb1c,
    0x6fcc_3706,
    0x2cbb_77d8,
    0x7576_2f2a,
    0x6425_ccdd,
    0x24b3_5461,
    0x0a7d_8715,
    0x2204_14a8,
    0x141e_bf67,
    0x56b4_1583,
    0x73e5_02e3,
    0x44ca_b16f,
    0x2826_4d42,
    0x73ba_aefb,
    0x0a50_ebed,
    0x1d6a_b6fb,
    0x0d3a_d40b,
    0x35db_3b68,
    0x2b08_1e83,
    0x77ce_6b95,
    0x5181_e5f0,
    0x7885_3bbc,
    0x009f_9494,
    0x27e5_ed3c,
];

// upstream: src/utils/random_utils.h VP8Random
/// State of the dithering generator (C: `VP8Random`).
///
/// `index1` and `index2` are positions in `tab`; C keeps them as `int`, and they
/// are only ever in `0..VP8_RANDOM_TABLE_SIZE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VP8Random {
    pub index1: usize,
    pub index2: usize,
    pub tab: [u32; VP8_RANDOM_TABLE_SIZE],
    pub amp: i32,
}

// upstream: src/utils/random_utils.c VP8InitRandom
/// Initializes the generator with a dithering amplitude in `[0, 1]`; values
/// outside that range are clamped, and the amplitude is kept in fixed point.
pub fn vp8_init_random(rg: &mut VP8Random, dithering: f32) {
    rg.tab = RANDOM_TABLE;
    rg.index1 = 0;
    rg.index2 = 31;
    rg.amp = if dithering < 0.0 {
        0
    } else if dithering > 1.0 {
        1 << VP8_RANDOM_DITHER_FIX
    } else {
        scaled_amplitude(dithering)
    };
}

/// `(uint32_t)((1 << VP8_RANDOM_DITHER_FIX) * dithering)` for a value in `[0, 1]`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the value is in [0, 256] and the C conversion truncates toward zero"
)]
#[must_use]
fn scaled_amplitude(dithering: f32) -> i32 {
    ((256.0_f32 * dithering) as u32).cast_signed()
}

// upstream: src/utils/random_utils.h VP8RandomBits2
/// Returns a centered pseudo-random number with `num_bits` of amplitude, scaled
/// by `amp` in `VP8_RANDOM_DITHER_FIX` fixed point (Knuth's difference generator).
///
/// `num_bits + VP8_RANDOM_DITHER_FIX` must not exceed 31.
#[must_use]
pub fn vp8_random_bits2(rg: &mut VP8Random, num_bits: i32, amp: i32) -> i32 {
    debug_assert!(num_bits + VP8_RANDOM_DITHER_FIX <= 31);
    let mut diff = rg.tab[rg.index1]
        .wrapping_sub(rg.tab[rg.index2])
        .cast_signed();
    if diff < 0 {
        diff = diff.cast_unsigned().wrapping_add(1 << 31).cast_signed();
    }
    rg.tab[rg.index1] = diff.cast_unsigned();
    rg.index1 += 1;
    if rg.index1 == VP8_RANDOM_TABLE_SIZE {
        rg.index1 = 0;
    }
    rg.index2 += 1;
    if rg.index2 == VP8_RANDOM_TABLE_SIZE {
        rg.index2 = 0;
    }
    // sign-extend, 0-center
    diff = (diff.cast_unsigned() << 1).cast_signed() >> (32 - num_bits);
    diff = (diff * amp) >> VP8_RANDOM_DITHER_FIX; // restrict range
    diff += 1 << (num_bits - 1); // shift back to 0.5-center
    diff
}

// upstream: src/utils/random_utils.h VP8RandomBits
/// Returns a centered pseudo-random number with `num_bits` of amplitude, scaled
/// by the amplitude that the generator was initialized with.
#[must_use]
pub fn vp8_random_bits(rg: &mut VP8Random, num_bits: i32) -> i32 {
    let amp = rg.amp;
    vp8_random_bits2(rg, num_bits, amp)
}
