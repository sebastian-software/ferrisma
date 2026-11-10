// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Parameter bundle from lib/jxl/modular/transform/squeeze_params.{h,cc}.
//! The source constructor is located in transform.cc.

use crate::jxl::base::status::Status;
use crate::jxl::field_encodings::{Fields, U32Enc, bits, bits_offset, val};
use crate::jxl::fields::{Visitor, bundle};
#[derive(Clone, Debug)]
pub struct SqueezeParams {
    pub horizontal: bool,
    pub in_place: bool,
    pub begin_c: u32,
    pub num_c: u32,
}
impl Default for SqueezeParams {
    fn default() -> Self {
        let mut value = Self {
            horizontal: false,
            in_place: false,
            begin_c: 0,
            num_c: 0,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for SqueezeParams {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.bool(false, &mut self.horizontal)?;
        visitor.bool(false, &mut self.in_place)?;
        visitor.u32(
            U32Enc::new(
                bits(3),
                bits_offset(6, 8),
                bits_offset(10, 72),
                bits_offset(13, 1096),
            ),
            0,
            &mut self.begin_c,
        )?;
        visitor.u32(
            U32Enc::new(val(1), val(2), val(3), bits_offset(4, 4)),
            2,
            &mut self.num_c,
        )
    }
}
