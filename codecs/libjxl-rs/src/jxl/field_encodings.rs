// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/field_encodings.h`.
//! Raw-value wrappers implement FieldEnum to retain invalid wire values safely.

use super::base::status::{Status, StatusCode};

pub trait Fields {
    fn visit_fields(&mut self, visitor: &mut dyn super::fields::Visitor) -> Status;
}

#[derive(Clone, Copy, Debug)]
pub struct U32Distr {
    pub d: u32,
}

impl U32Distr {
    pub const DIRECT: u32 = 0x8000_0000;

    pub const fn is_direct(self) -> bool {
        self.d & Self::DIRECT != 0
    }

    pub const fn direct(self) -> u32 {
        self.d & (Self::DIRECT - 1)
    }

    pub const fn extra_bits(self) -> usize {
        ((self.d & 0x1F) + 1) as usize
    }

    pub const fn offset(self) -> u32 {
        (self.d >> 5) & 0x03FF_FFFF
    }
}

pub const fn val(value: u32) -> U32Distr {
    U32Distr {
        d: value | U32Distr::DIRECT,
    }
}

pub const fn bits_offset(bits: u32, offset: u32) -> U32Distr {
    U32Distr {
        d: ((bits.wrapping_sub(1)) & 0x1F) + ((offset & 0x03FF_FFFF) << 5),
    }
}

pub const fn bits(bits: u32) -> U32Distr {
    bits_offset(bits, 0)
}

#[derive(Clone, Copy, Debug)]
pub struct U32Enc {
    d: [U32Distr; 4],
}

impl U32Enc {
    pub const fn new(d0: U32Distr, d1: U32Distr, d2: U32Distr, d3: U32Distr) -> Self {
        Self {
            d: [d0, d1, d2, d3],
        }
    }

    pub fn get_distr(self, selector: u32) -> U32Distr {
        self.d[selector as usize]
    }
}

/// A source enum's raw representation, including invalid decoded values.
/// Implement with a u32 wrapper rather than constructing an invalid Rust enum.
pub trait FieldEnum: Copy {
    const BITS: u64;
    const NAME: &'static str;
    fn from_raw(value: u32) -> Self;
    fn raw(self) -> u32;
}

pub const fn make_bit(index: u32) -> u64 {
    assert!(index < 64);
    1u64 << index
}

pub fn values<E: FieldEnum>() -> Vec<E> {
    let mut bits = E::BITS;
    let mut values = Vec::with_capacity(bits.count_ones() as usize);
    while bits != 0 {
        values.push(E::from_raw(bits.trailing_zeros()));
        bits &= bits - 1;
    }
    values
}

pub fn enum_valid<E: FieldEnum>(value: E) -> Status {
    if value.raw() >= 64 {
        return Err(StatusCode::GenericError);
    }
    if E::BITS & make_bit(value.raw()) == 0 {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
