// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/fields.{h,cc}`.
//! `VisitorBase` is embedded rather than inherited. The const-cast visitor is
//! represented by a mutable bundle borrow. Generic enum helpers are unported.

use super::base::status::{Status, StatusCode, StatusOr};
use super::dec_bit_reader::BitReader;
use super::field_encodings::{Fields, U32Enc};

pub mod bits_coder {
    use super::*;
    pub use crate::jxl::enc_fields::bits_coder::write;

    pub const fn max_encoded_bits(bits: usize) -> usize {
        bits
    }

    pub fn can_encode(bits: usize, value: u32, encoded_bits: &mut usize) -> Status {
        assert!(bits <= 32);
        *encoded_bits = bits;
        if u64::from(value) >= (1u64 << bits) {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }

    pub fn read(bits: usize, reader: &mut BitReader<'_>) -> u32 {
        assert!(bits <= 32);
        reader.read_bits(bits) as u32
    }
}

pub mod u32_coder {
    use super::*;
    pub use crate::jxl::enc_fields::u32_coder::write;

    pub fn max_encoded_bits(enc: U32Enc) -> usize {
        let mut extra_bits = 0;
        for selector in 0..4 {
            let d = enc.get_distr(selector);
            if d.is_direct() {
                continue;
            } else {
                extra_bits = extra_bits.max(d.extra_bits());
            }
        }
        2 + extra_bits
    }

    pub fn can_encode(enc: U32Enc, value: u32, encoded_bits: &mut usize) -> Status {
        match choose_selector(enc, value) {
            Ok((_, total_bits)) => {
                *encoded_bits = total_bits;
                Ok(())
            }
            Err(code) => {
                *encoded_bits = 0;
                Err(code)
            }
        }
    }

    pub fn read(enc: U32Enc, reader: &mut BitReader<'_>) -> u32 {
        let selector = reader.read_fixed_bits::<2>() as u32;
        let d = enc.get_distr(selector);
        if d.is_direct() {
            d.direct()
        } else {
            (reader.read_bits(d.extra_bits()) + u64::from(d.offset())) as u32
        }
    }

    pub fn choose_selector(enc: U32Enc, value: u32) -> StatusOr<(u32, usize)> {
        let bits_required = 32 - value.leading_zeros();
        debug_assert!(bits_required <= 32);
        let mut selector = 0;
        let mut total_bits = 64;
        for s in 0..4 {
            let d = enc.get_distr(s);
            if d.is_direct() {
                if d.direct() == value {
                    return Ok((s, 2));
                }
                continue;
            }
            let extra_bits = d.extra_bits();
            let offset = d.offset();
            if value < offset || u64::from(value) >= u64::from(offset) + (1u64 << extra_bits) {
                continue;
            }
            if 2 + extra_bits < total_bits {
                selector = s;
                total_bits = 2 + extra_bits;
            }
        }
        if total_bits == 64 {
            return Err(StatusCode::GenericError);
        }
        Ok((selector, total_bits))
    }
}

pub mod u64_coder {
    use super::*;
    pub use crate::jxl::enc_fields::u64_coder::write;

    pub const fn max_encoded_bits() -> usize {
        2 + 12 + 6 * (8 + 1) + (4 + 1)
    }

    pub fn read(reader: &mut BitReader<'_>) -> u64 {
        let selector = reader.read_fixed_bits::<2>();
        if selector == 0 {
            return 0;
        }
        if selector == 1 {
            return 1 + reader.read_fixed_bits::<4>();
        }
        if selector == 2 {
            return 17 + reader.read_fixed_bits::<8>();
        }
        let mut result = reader.read_fixed_bits::<12>();
        let mut shift = 12;
        while reader.read_fixed_bits::<1>() != 0 {
            if shift == 60 {
                result |= reader.read_fixed_bits::<4>() << shift;
                break;
            }
            result |= reader.read_fixed_bits::<8>() << shift;
            shift += 8;
        }
        result
    }

    pub fn can_encode(mut value: u64, encoded_bits: &mut usize) -> Status {
        if value == 0 {
            *encoded_bits = 2;
        } else if value <= 16 {
            *encoded_bits = 2 + 4;
        } else if value <= 272 {
            *encoded_bits = 2 + 8;
        } else {
            *encoded_bits = 2 + 12;
            value >>= 12;
            let mut shift = 12;
            while value > 0 && shift < 60 {
                *encoded_bits += 1 + 8;
                value >>= 8;
                shift += 8;
            }
            if value > 0 {
                *encoded_bits += 1 + 4;
            } else {
                *encoded_bits += 1;
            }
        }
        Ok(())
    }
}

pub mod f16_coder {
    use super::*;
    pub use crate::jxl::enc_fields::f16_coder::write;

    pub const fn max_encoded_bits() -> usize {
        16
    }

    pub fn read(reader: &mut BitReader<'_>, value: &mut f32) -> Status {
        let bits16 = reader.read_fixed_bits::<16>() as u32;
        let sign = bits16 >> 15;
        let biased_exp = (bits16 >> 10) & 0x1F;
        let mantissa = bits16 & 0x3FF;
        if biased_exp == 31 {
            return Err(StatusCode::GenericError);
        }
        if biased_exp == 0 {
            *value = (1.0f32 / 16384.0) * (mantissa as f32 * (1.0f32 / 1024.0));
            if sign != 0 {
                *value = -*value;
            }
            return Ok(());
        }
        let biased_exp32 = biased_exp + (127 - 15);
        let mantissa32 = mantissa << (23 - 10);
        let bits32 = (sign << 31) | (biased_exp32 << 23) | mantissa32;
        *value = f32::from_bits(bits32);
        Ok(())
    }

    pub fn can_encode(value: f32, encoded_bits: &mut usize) -> Status {
        *encoded_bits = max_encoded_bits();
        if value.is_nan() || value.is_infinite() || value.abs() > 65504.0 {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
}

pub trait Visitor {
    fn visit(&mut self, fields: &mut dyn Fields) -> Status;
    fn base(&mut self) -> &mut VisitorBase;
    fn bits(&mut self, bits: usize, default_value: u32, value: &mut u32) -> Status;
    fn u32(&mut self, enc: U32Enc, default_value: u32, value: &mut u32) -> Status;
    fn u64(&mut self, default_value: u64, value: &mut u64) -> Status;
    fn f16(&mut self, default_value: f32, value: &mut f32) -> Status;

    fn bool(&mut self, default_value: bool, value: &mut bool) -> Status {
        let mut bits = u32::from(*value);
        self.bits(1, u32::from(default_value), &mut bits)?;
        debug_assert!(bits <= 1);
        *value = bits == 1;
        Ok(())
    }

    fn conditional(&mut self, condition: bool) -> bool {
        condition
    }

    fn all_default(&mut self, _fields: &mut dyn Fields, all_default: &mut bool) -> bool {
        // The source uses this Status as an if-condition. A failed Bool read
        // is false, not an early return: the bundle continues visiting fields.
        // Preserve the resulting partial state and consumed-bit counter.
        if self.bool(true, all_default).is_err() {
            return false;
        }
        *all_default
    }

    fn set_default(&mut self, _fields: &mut dyn Fields) {}

    fn visit_nested(&mut self, fields: &mut dyn Fields) -> Status {
        self.visit(fields)
    }

    fn is_reading(&self) -> bool {
        false
    }

    fn begin_extensions(&mut self, extensions: &mut u64) -> Status {
        self.u64(0, extensions)?;
        self.base().extension_states.begin();
        Ok(())
    }

    fn end_extensions(&mut self) -> Status {
        self.base().extension_states.end();
        Ok(())
    }

    fn check_has_enough_bits(&mut self, _bits: usize) -> Status {
        // The source returns false for non-reading visitors.
        Err(StatusCode::GenericError)
    }
}

#[derive(Default)]
struct ExtensionStates {
    begun: u64,
    ended: u64,
}

impl ExtensionStates {
    fn push(&mut self) {
        self.begun <<= 1;
        self.ended <<= 1;
    }

    fn pop(&mut self) {
        self.begun >>= 1;
        self.ended >>= 1;
    }

    fn is_begun(&self) -> bool {
        self.begun & 1 != 0
    }

    fn is_ended(&self) -> bool {
        self.ended & 1 != 0
    }

    fn begin(&mut self) {
        debug_assert!(!self.is_begun() && !self.is_ended());
        self.begun += 1;
    }

    fn end(&mut self) {
        debug_assert!(self.is_begun() && !self.is_ended());
        self.ended += 1;
    }
}

#[derive(Default)]
pub struct VisitorBase {
    depth: usize,
    extension_states: ExtensionStates,
}

impl VisitorBase {
    pub(crate) fn extension_states_begin(&mut self) {
        self.extension_states.begin();
    }

    pub(crate) fn visit<V: Visitor>(visitor: &mut V, fields: &mut dyn Fields) -> Status {
        if visitor.base().depth >= bundle::MAX_EXTENSIONS {
            return Err(StatusCode::GenericError);
        }
        visitor.base().depth += 1;
        visitor.base().extension_states.push();
        let ok = fields.visit_fields(visitor);
        if ok.is_ok() {
            let states = &visitor.base().extension_states;
            debug_assert!(!states.is_begun() || states.is_ended());
        }
        visitor.base().extension_states.pop();
        debug_assert_ne!(visitor.base().depth, 0);
        visitor.base().depth -= 1;
        ok
    }
}

impl Drop for VisitorBase {
    fn drop(&mut self) {
        debug_assert!(self.depth == 0 || std::thread::panicking());
    }
}

// C++ inheritance shares Visit and the base state. This macro only supplies
// that same delegation; each visitor's field behavior stays explicit below.
macro_rules! visitor_base {
    () => {
        fn base(&mut self) -> &mut VisitorBase {
            &mut self.base
        }
        fn visit(&mut self, fields: &mut dyn Fields) -> Status {
            VisitorBase::visit(self, fields)
        }
    };
}
pub(crate) use visitor_base;

#[derive(Default)]
struct InitVisitor {
    base: VisitorBase,
}

impl Visitor for InitVisitor {
    visitor_base!();

    fn bits(&mut self, _: usize, default_value: u32, value: &mut u32) -> Status {
        *value = default_value;
        Ok(())
    }

    fn u32(&mut self, _: U32Enc, default_value: u32, value: &mut u32) -> Status {
        *value = default_value;
        Ok(())
    }

    fn u64(&mut self, default_value: u64, value: &mut u64) -> Status {
        *value = default_value;
        Ok(())
    }

    fn bool(&mut self, default_value: bool, value: &mut bool) -> Status {
        *value = default_value;
        Ok(())
    }

    fn f16(&mut self, default_value: f32, value: &mut f32) -> Status {
        *value = default_value;
        Ok(())
    }

    fn conditional(&mut self, _: bool) -> bool {
        true
    }

    fn all_default(&mut self, _: &mut dyn Fields, all_default: &mut bool) -> bool {
        self.bool(true, all_default)
            .expect("initialization cannot fail");
        false
    }

    fn visit_nested(&mut self, _: &mut dyn Fields) -> Status {
        // Nested constructors already initialize their own fields.
        Ok(())
    }
}

#[derive(Default)]
struct SetDefaultVisitor {
    base: VisitorBase,
}

impl Visitor for SetDefaultVisitor {
    visitor_base!();

    fn bits(&mut self, _: usize, default_value: u32, value: &mut u32) -> Status {
        *value = default_value;
        Ok(())
    }

    fn u32(&mut self, _: U32Enc, default_value: u32, value: &mut u32) -> Status {
        *value = default_value;
        Ok(())
    }

    fn u64(&mut self, default_value: u64, value: &mut u64) -> Status {
        *value = default_value;
        Ok(())
    }

    fn bool(&mut self, default_value: bool, value: &mut bool) -> Status {
        *value = default_value;
        Ok(())
    }

    fn f16(&mut self, default_value: f32, value: &mut f32) -> Status {
        *value = default_value;
        Ok(())
    }

    fn conditional(&mut self, _: bool) -> bool {
        true
    }

    fn all_default(&mut self, _: &mut dyn Fields, all_default: &mut bool) -> bool {
        self.bool(true, all_default)
            .expect("initialization cannot fail");
        false
    }
}

struct AllDefaultVisitor {
    base: VisitorBase,
    all_default: bool,
}

impl Visitor for AllDefaultVisitor {
    visitor_base!();

    fn bits(&mut self, _: usize, default_value: u32, value: &mut u32) -> Status {
        self.all_default &= *value == default_value;
        Ok(())
    }

    fn u32(&mut self, _: U32Enc, default_value: u32, value: &mut u32) -> Status {
        self.all_default &= *value == default_value;
        Ok(())
    }

    fn u64(&mut self, default_value: u64, value: &mut u64) -> Status {
        self.all_default &= *value == default_value;
        Ok(())
    }

    fn f16(&mut self, default_value: f32, value: &mut f32) -> Status {
        self.all_default &= (*value - default_value).abs() < 1E-6f32;
        Ok(())
    }

    fn all_default(&mut self, _: &mut dyn Fields, _: &mut bool) -> bool {
        false
    }
}

struct ReadVisitor<'r, 'a> {
    base: VisitorBase,
    ok: bool,
    enough_bytes: bool,
    reader: &'r mut BitReader<'a>,
    extension_bits: [u64; bundle::MAX_EXTENSIONS],
    total_extension_bits: u64,
    pos_after_ext_size: usize,
}

impl<'r, 'a> ReadVisitor<'r, 'a> {
    fn new(reader: &'r mut BitReader<'a>) -> Self {
        Self {
            base: VisitorBase::default(),
            ok: true,
            enough_bytes: true,
            reader,
            extension_bits: [0; bundle::MAX_EXTENSIONS],
            total_extension_bits: 0,
            pos_after_ext_size: 0,
        }
    }

    fn check_bounds(&mut self) -> Status {
        self.reader
            .all_reads_within_bounds()
            .map_err(|_| StatusCode::NotEnoughBytes)
    }
}

impl Visitor for ReadVisitor<'_, '_> {
    visitor_base!();

    fn bits(&mut self, bits: usize, _: u32, value: &mut u32) -> Status {
        *value = bits_coder::read(bits, self.reader);
        self.check_bounds()
    }

    fn u32(&mut self, enc: U32Enc, _: u32, value: &mut u32) -> Status {
        *value = u32_coder::read(enc, self.reader);
        self.check_bounds()
    }

    fn u64(&mut self, _: u64, value: &mut u64) -> Status {
        *value = u64_coder::read(self.reader);
        self.check_bounds()
    }

    fn f16(&mut self, _: f32, value: &mut f32) -> Status {
        self.ok &= f16_coder::read(self.reader, value).is_ok();
        self.check_bounds()
    }

    fn set_default(&mut self, fields: &mut dyn Fields) {
        bundle::set_default(fields);
    }

    fn is_reading(&self) -> bool {
        true
    }

    fn begin_extensions(&mut self, extensions: &mut u64) -> Status {
        self.u64(0, extensions)?;
        self.base.extension_states.begin();
        if *extensions == 0 {
            return Ok(());
        }
        let mut remaining_extensions = *extensions;
        while remaining_extensions != 0 {
            let idx = remaining_extensions.trailing_zeros() as usize;
            // Avoid two simultaneous mutable borrows of this visitor. Preserve
            // the field value even on a truncated read, as in the source.
            let mut value = self.extension_bits[idx];
            let status = self.u64(0, &mut value);
            self.extension_bits[idx] = value;
            status?;
            self.total_extension_bits = self
                .total_extension_bits
                .checked_add(value)
                .ok_or(StatusCode::GenericError)?;
            remaining_extensions &= remaining_extensions - 1;
        }
        self.pos_after_ext_size = self.reader.total_bits_consumed();
        if self.pos_after_ext_size == 0 {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }

    fn end_extensions(&mut self) -> Status {
        self.base.extension_states.end();
        if self.pos_after_ext_size == 0 {
            return Ok(());
        }
        if !self.enough_bytes {
            return Ok(());
        }
        let bits_read = self.reader.total_bits_consumed();
        let end = (self.pos_after_ext_size as u64)
            .checked_add(self.total_extension_bits)
            .ok_or(StatusCode::GenericError)?;
        if bits_read as u64 > end {
            return Err(StatusCode::GenericError);
        }
        let remaining_bits = (end - bits_read as u64) as usize;
        if remaining_bits != 0 {
            self.reader.skip_bits(remaining_bits);
            self.check_bounds()?;
        }
        Ok(())
    }

    fn check_has_enough_bits(&mut self, bits: usize) -> Status {
        let have_bits = self.reader.total_bytes().wrapping_mul(8);
        let want_bits = bits.wrapping_add(self.reader.total_bits_consumed());
        if have_bits < want_bits {
            return Err(StatusCode::NotEnoughBytes);
        }
        Ok(())
    }
}

struct CanEncodeVisitor {
    base: VisitorBase,
    ok: bool,
    encoded_bits: usize,
    extensions: u64,
    pos_after_ext: u64,
}

impl CanEncodeVisitor {
    fn new() -> Self {
        Self {
            base: VisitorBase::default(),
            ok: true,
            encoded_bits: 0,
            extensions: 0,
            pos_after_ext: 0,
        }
    }

    fn get_sizes(&mut self) -> StatusOr<(usize, usize)> {
        if !self.ok {
            return Err(StatusCode::GenericError);
        }
        let mut extension_bits = 0;
        let mut total_bits = self.encoded_bits;
        if self.pos_after_ext != 0 {
            debug_assert!(self.encoded_bits >= self.pos_after_ext as usize);
            extension_bits = self.encoded_bits - self.pos_after_ext as usize;
            let mut encoded_bits = 0;
            self.ok &= u64_coder::can_encode(extension_bits as u64, &mut encoded_bits).is_ok();
            total_bits += encoded_bits;
            for _ in 1..self.extensions.count_ones() {
                encoded_bits = 0;
                self.ok &= u64_coder::can_encode(0, &mut encoded_bits).is_ok();
                total_bits += encoded_bits;
            }
        }
        Ok((extension_bits, total_bits))
    }
}

impl Visitor for CanEncodeVisitor {
    visitor_base!();

    fn bits(&mut self, bits: usize, _: u32, value: &mut u32) -> Status {
        let mut encoded_bits = 0;
        self.ok &= bits_coder::can_encode(bits, *value, &mut encoded_bits).is_ok();
        self.encoded_bits += encoded_bits;
        Ok(())
    }

    fn u32(&mut self, enc: U32Enc, _: u32, value: &mut u32) -> Status {
        let mut encoded_bits = 0;
        self.ok &= u32_coder::can_encode(enc, *value, &mut encoded_bits).is_ok();
        self.encoded_bits += encoded_bits;
        Ok(())
    }

    fn u64(&mut self, _: u64, value: &mut u64) -> Status {
        let mut encoded_bits = 0;
        self.ok &= u64_coder::can_encode(*value, &mut encoded_bits).is_ok();
        self.encoded_bits += encoded_bits;
        Ok(())
    }

    fn f16(&mut self, _: f32, value: &mut f32) -> Status {
        let mut encoded_bits = 0;
        self.ok &= f16_coder::can_encode(*value, &mut encoded_bits).is_ok();
        self.encoded_bits += encoded_bits;
        Ok(())
    }

    fn all_default(&mut self, fields: &mut dyn Fields, all_default: &mut bool) -> bool {
        *all_default = bundle::all_default(fields);
        if self.bool(true, all_default).is_err() {
            return false;
        }
        *all_default
    }

    fn begin_extensions(&mut self, extensions: &mut u64) -> Status {
        self.u64(0, extensions)?;
        self.base.extension_states.begin();
        self.extensions = *extensions;
        if *extensions != 0 {
            debug_assert_eq!(self.pos_after_ext, 0);
            self.pos_after_ext = self.encoded_bits as u64;
            debug_assert_ne!(self.pos_after_ext, 0);
        }
        Ok(())
    }
}

pub mod bundle {
    use super::*;
    pub use crate::jxl::enc_fields::bundle::write;

    pub const MAX_EXTENSIONS: usize = 64;

    pub fn init(fields: &mut dyn Fields) {
        let mut visitor = InitVisitor::default();
        visitor.visit(fields).expect("Init should never fail");
    }

    pub fn set_default(fields: &mut dyn Fields) {
        let mut visitor = SetDefaultVisitor::default();
        visitor.visit(fields).expect("SetDefault should never fail");
    }

    pub fn all_default(fields: &mut dyn Fields) -> bool {
        let mut visitor = AllDefaultVisitor {
            base: VisitorBase::default(),
            all_default: true,
        };
        // The release reference ignores a failed visit after JXL_DEBUG_ABORT,
        // then returns the accumulated predicate, including partial state,
        // to the enclosing visitor.
        let _status = visitor.visit(fields);
        visitor.all_default
    }

    pub fn can_encode(fields: &mut dyn Fields) -> StatusOr<(usize, usize)> {
        let mut visitor = CanEncodeVisitor::new();
        visitor.visit(fields)?;
        visitor.get_sizes()
    }

    pub fn read(reader: &mut BitReader<'_>, fields: &mut dyn Fields) -> Status {
        let mut visitor = ReadVisitor::new(reader);
        visitor.visit(fields)?;
        if visitor.ok {
            Ok(())
        } else {
            Err(StatusCode::GenericError)
        }
    }

    pub fn can_read(reader: &mut BitReader<'_>, fields: &mut dyn Fields) -> bool {
        let mut visitor = ReadVisitor::new(reader);
        visitor.visit(fields) != Err(StatusCode::NotEnoughBytes)
    }
}

pub fn check_has_enough_bits(visitor: &mut dyn Visitor, bits: usize) -> Status {
    visitor.check_has_enough_bits(bits)
}

/// The source's templated Visitor::Enum helper, kept outside the object-safe
/// Visitor trait. A decoded raw value is assigned before enum validation.
pub fn visit_enum<E: super::field_encodings::FieldEnum>(
    visitor: &mut dyn Visitor,
    default_value: E,
    value: &mut E,
) -> Status {
    use super::field_encodings::{U32Enc, bits_offset, enum_valid, val};
    let mut u32_value = value.raw();
    visitor.u32(
        U32Enc::new(val(0), val(1), bits_offset(4, 2), bits_offset(6, 18)),
        default_value.raw(),
        &mut u32_value,
    )?;
    *value = E::from_raw(u32_value);
    enum_valid(*value)
}
