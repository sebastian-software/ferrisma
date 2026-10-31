// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Name-string visitor from `lib/jxl/frame_header.h`, also used by image
//! metadata. Frame types, fields, and frame parsing remain unported.
//! C++ strings become byte vectors to preserve arbitrary non-UTF-8 input.

use super::base::status::Status;
use super::field_encodings::{U32Enc, bits, bits_offset, val};
use super::fields::Visitor;

pub fn visit_name_string(visitor: &mut dyn Visitor, name: &mut Vec<u8>) -> Status {
    let mut length = name.len() as u32;
    visitor.u32(
        U32Enc::new(val(0), bits(4), bits_offset(5, 16), bits_offset(10, 48)),
        0,
        &mut length,
    )?;
    if visitor.is_reading() {
        name.resize(length as usize, 0);
    }
    for byte in &mut name[..length as usize] {
        let mut value = u32::from(*byte);
        visitor.bits(8, 0, &mut value)?;
        *byte = value as u8;
    }
    Ok(())
}
