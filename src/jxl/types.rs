// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Endianness and bit-depth descriptors from `lib/include/jxl/types.h`.
//! Other public C API types remain unported.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum JxlEndianness {
    Native = 0,
    Little = 1,
    Big = 2,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlBitDepthType(pub u32);
impl JxlBitDepthType {
    pub const FROM_PIXEL_FORMAT: Self = Self(0);
    pub const FROM_CODESTREAM: Self = Self(1);
    pub const CUSTOM: Self = Self(2);
}
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct JxlBitDepth {
    pub depth_type: JxlBitDepthType,
    pub bits_per_sample: u32,
    pub exponent_bits_per_sample: u32,
}
