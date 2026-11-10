// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// OldBundle and NewBundle are ported fixtures from lib/jxl/fields_test.cc.

use jxl::jxl::base::status::Status;
use jxl::jxl::field_encodings::{Fields, U32Enc, bits, val};
use jxl::jxl::fields::{Visitor, bundle};
use jxl::jxl::headers::SizeHeader;

pub struct OldBundle {
    pub old_small: u32,
    pub old_f: f32,
    pub old_large: u32,
    pub extensions: u64,
}

impl Default for OldBundle {
    fn default() -> Self {
        let mut fields = Self {
            old_small: 0,
            old_f: 0.0,
            old_large: 0,
            extensions: 0,
        };
        bundle::init(&mut fields);
        fields
    }
}

impl Fields for OldBundle {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.u32(
            U32Enc::new(val(1), bits(2), bits(3), bits(4)),
            1,
            &mut self.old_small,
        )?;
        visitor.f16(1.125, &mut self.old_f)?;
        visitor.u32(
            U32Enc::new(bits(7), bits(12), bits(16), bits(32)),
            0,
            &mut self.old_large,
        )?;
        visitor.begin_extensions(&mut self.extensions)?;
        visitor.end_extensions()
    }
}

pub struct NewBundle {
    pub old_small: u32,
    pub old_f: f32,
    pub old_large: u32,
    pub extensions: u64,
    pub new_small: u32,
    pub new_f: f32,
    pub new_large: u32,
}

impl Default for NewBundle {
    fn default() -> Self {
        let mut fields = Self {
            old_small: 0,
            old_f: 0.0,
            old_large: 0,
            extensions: 0,
            new_small: 2,
            new_f: -2.0,
            new_large: 0,
        };
        bundle::init(&mut fields);
        fields
    }
}

impl Fields for NewBundle {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.u32(
            U32Enc::new(val(1), bits(2), bits(3), bits(4)),
            1,
            &mut self.old_small,
        )?;
        visitor.f16(1.125, &mut self.old_f)?;
        visitor.u32(
            U32Enc::new(bits(7), bits(12), bits(16), bits(32)),
            0,
            &mut self.old_large,
        )?;
        visitor.begin_extensions(&mut self.extensions)?;
        if visitor.conditional(self.extensions & 1 != 0) {
            visitor.u32(
                U32Enc::new(val(2), bits(2), bits(3), bits(4)),
                2,
                &mut self.new_small,
            )?;
            visitor.f16(-2.0, &mut self.new_f)?;
        }
        if visitor.conditional(self.extensions & 2 != 0) {
            visitor.u32(
                U32Enc::new(bits(9), bits(12), bits(16), bits(32)),
                0,
                &mut self.new_large,
            )?;
        }
        visitor.end_extensions()
    }
}

// Exercises the AllDefault/SetDefault/VisitNested pattern used by upstream
// bundles. This fixture is local; the visitor algorithms are the ported code.
pub struct DefaultedBundle {
    pub all_default: bool,
    pub value: u32,
    pub size: SizeHeader,
}

impl Default for DefaultedBundle {
    fn default() -> Self {
        let mut fields = Self {
            all_default: false,
            value: 0,
            size: SizeHeader::default(),
        };
        bundle::init(&mut fields);
        fields
    }
}

impl Fields for DefaultedBundle {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        visitor.bits(4, 7, &mut self.value)?;
        visitor.visit_nested(&mut self.size)
    }
}
