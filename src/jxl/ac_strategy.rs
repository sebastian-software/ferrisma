// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AcStrategyType and AcStrategy size/count constants from lib/jxl/ac_strategy.h.
//! Strategy objects, geometry/transform methods, and AcStrategyImage are unported.
use super::frame_dimensions::BLOCK_DIM;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AcStrategyType {
    Dct = 0,
    Identity,
    Dct2X2,
    Dct4X4,
    Dct16X16,
    Dct32X32,
    Dct16X8,
    Dct8X16,
    Dct32X8,
    Dct8X32,
    Dct32X16,
    Dct16X32,
    Dct4X8,
    Dct8X4,
    Afv0,
    Afv1,
    Afv2,
    Afv3,
    Dct64X64,
    Dct64X32,
    Dct32X64,
    Dct128X128,
    Dct128X64,
    Dct64X128,
    Dct256X256,
    Dct256X128,
    Dct128X256,
}
pub const MAX_COEFF_BLOCKS: usize = 32;
pub const MAX_BLOCK_DIM: usize = BLOCK_DIM * MAX_COEFF_BLOCKS;
pub const MAX_COEFF_AREA: usize = MAX_BLOCK_DIM * MAX_BLOCK_DIM;
pub const NUM_VALID_STRATEGIES: u8 = AcStrategyType::Dct128X256 as u8 + 1;
