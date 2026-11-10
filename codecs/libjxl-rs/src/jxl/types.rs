// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Pixel, endianness, and bit-depth descriptors from `lib/include/jxl/types.h`.
//! Other public C API types remain unported.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlEndianness(pub u32);
impl JxlEndianness {
    pub const NATIVE: Self = Self(0);
    pub const LITTLE: Self = Self(1);
    pub const BIG: Self = Self(2);
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlDataType(pub u32);
impl JxlDataType {
    pub const FLOAT: Self = Self(0);
    pub const UINT8: Self = Self(2);
    pub const UINT16: Self = Self(3);
    pub const FLOAT16: Self = Self(5);
}
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct JxlPixelFormat {
    pub num_channels: u32,
    pub data_type: JxlDataType,
    pub endianness: JxlEndianness,
    pub align: usize,
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
