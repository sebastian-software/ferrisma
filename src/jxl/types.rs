// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The endianness definition from `lib/include/jxl/types.h`.
//! Other public C API types remain unported.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum JxlEndianness {
    Native = 0,
    Little = 1,
    Big = 2,
}
