// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! QuantTable indices and count from lib/jxl/quant_weights.h. Matrix encoding,
//! weights, table conversion, and the remaining quantization API are unported.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum QuantTable {
    Dct = 0,
    Identity,
    Dct2X2,
    Dct4X4,
    Dct16X16,
    Dct32X32,
    Dct8X16,
    Dct8X32,
    Dct16X32,
    Dct4X8,
    Afv0,
    Dct64X64,
    Dct32X64,
    Dct128X128,
    Dct64X128,
    Dct256X256,
    Dct128X256,
}
pub const NUM_QUANT_TABLES: u8 = QuantTable::Dct128X256 as u8 + 1;
