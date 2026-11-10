// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/pack_signed.h`, including pointer-width UnpackSigned.

pub const fn pack_signed(value: i32) -> u32 {
    (value as u32).wrapping_shl(1) ^ (((!value) as u32 >> 31).wrapping_sub(1))
}
pub const fn unpack_signed(value: usize) -> i32 {
    ((value >> 1) ^ ((!value & 1).wrapping_sub(1))) as i32
}
