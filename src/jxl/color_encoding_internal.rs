// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Color bundles and non-callback field APIs from
//! `lib/jxl/color_encoding_internal.{h,cc}`, including callback ICC assignment
//! and ColorSpaceTransform. Visitors retain mutable storage synchronization.

use super::base::status::{Status, StatusCode};
use super::cms::color_encoding_cms as cms;
use super::cms::icc::{color_encoding_description, maybe_create_profile};
use super::cms_interface::JxlCmsInterface;
use super::color_encoding::JxlColorEncoding;
pub use super::color_encoding::{
    ColorSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};
pub use super::color_space_transform::ColorSpaceTransform;
use super::field_encodings::{Fields, U32Enc, bits, bits_offset};
use super::fields::{Visitor, bundle, visit_enum};
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct CustomXy {
    storage: cms::CustomXy,
}
impl Default for CustomXy {
    fn default() -> Self {
        let mut value = Self {
            storage: cms::CustomXy::default(),
        };
        bundle::init(&mut value);
        value
    }
}
fn pack_signed(value: i32) -> u32 {
    (value as u32).wrapping_shl(1) ^ if value < 0 { u32::MAX } else { 0 }
}
fn unpack_signed(value: u32) -> i32 {
    ((value >> 1) as i32) ^ -((value & 1) as i32)
}
impl Fields for CustomXy {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let enc = U32Enc::new(
            bits(19),
            bits_offset(19, 524288),
            bits_offset(20, 1048576),
            bits_offset(21, 2097152),
        );
        let mut x = pack_signed(self.storage.x);
        visitor.u32(enc, 0, &mut x)?;
        self.storage.x = unpack_signed(x);
        let mut y = pack_signed(self.storage.y);
        visitor.u32(enc, 0, &mut y)?;
        self.storage.y = unpack_signed(y);
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct CustomTransferFunction {
    pub nonserialized_color_space: ColorSpace,
    storage: cms::CustomTransferFunction,
}
impl Default for CustomTransferFunction {
    fn default() -> Self {
        let mut value = Self {
            nonserialized_color_space: ColorSpace::RGB,
            storage: cms::CustomTransferFunction::default(),
        };
        bundle::init(&mut value);
        value
    }
}
impl CustomTransferFunction {
    pub fn set_implicit(&mut self) -> bool {
        if self.nonserialized_color_space == ColorSpace::XYB {
            return self.storage.set_gamma(1.0 / 3.0).is_ok();
        }
        false
    }
}
impl Fields for CustomTransferFunction {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let implicit = self.set_implicit();
        if visitor.conditional(!implicit) {
            visitor.bool(false, &mut self.storage.have_gamma)?;
            if visitor.conditional(self.storage.have_gamma) {
                visitor.bits(
                    24,
                    cms::CustomTransferFunction::GAMMA_MUL,
                    &mut self.storage.gamma,
                )?;
                if self.storage.gamma > cms::CustomTransferFunction::GAMMA_MUL
                    || u64::from(self.storage.gamma)
                        * u64::from(cms::CustomTransferFunction::MAX_GAMMA)
                        < u64::from(cms::CustomTransferFunction::GAMMA_MUL)
                {
                    return Err(StatusCode::GenericError);
                }
            }
            if visitor.conditional(!self.storage.have_gamma) {
                visit_enum(
                    visitor,
                    TransferFunction::SRGB,
                    &mut self.storage.transfer_function,
                )?;
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct ColorEncoding {
    pub all_default: bool,
    want_icc: bool,
    storage: cms::ColorEncoding,
    white: CustomXy,
    tf: CustomTransferFunction,
    red: CustomXy,
    green: CustomXy,
    blue: CustomXy,
}
impl Default for ColorEncoding {
    fn default() -> Self {
        let mut value = Self {
            all_default: false,
            want_icc: false,
            storage: cms::ColorEncoding::default(),
            white: CustomXy::default(),
            tf: CustomTransferFunction::default(),
            red: CustomXy::default(),
            green: CustomXy::default(),
            blue: CustomXy::default(),
        };
        bundle::init(&mut value);
        value
    }
}
impl ColorEncoding {
    /// Default C++ member move: the ICC vector transfers, scalar/custom fields
    /// remain unchanged in the source. No profile reconstruction takes place.
    pub(crate) fn move_from(&mut self, source: &mut Self) {
        self.all_default = source.all_default;
        self.want_icc = source.want_icc;
        self.storage.white_point = source.storage.white_point;
        self.storage.primaries = source.storage.primaries;
        self.storage.rendering_intent = source.storage.rendering_intent;
        self.storage.have_fields = source.storage.have_fields;
        self.storage.icc = std::mem::take(&mut source.storage.icc);
        self.storage.color_space = source.storage.color_space;
        self.storage.cmyk = source.storage.cmyk;
        self.storage.tf.clone_from(&source.storage.tf);
        self.storage.white.clone_from(&source.storage.white);
        self.storage.red.clone_from(&source.storage.red);
        self.storage.green.clone_from(&source.storage.green);
        self.storage.blue.clone_from(&source.storage.blue);
        self.white.clone_from(&source.white);
        self.tf.clone_from(&source.tf);
        self.red.clone_from(&source.red);
        self.green.clone_from(&source.green);
        self.blue.clone_from(&source.blue);
    }
    fn create_c2(primaries: Primaries, tf: TransferFunction) -> [Self; 2] {
        let mut values = [Self::default(), Self::default()];
        for (i, value) in values.iter_mut().enumerate() {
            value.set_color_space(if i == 0 {
                ColorSpace::RGB
            } else {
                ColorSpace::GRAY
            });
            value.storage.white_point = WhitePoint::D65;
            value.storage.primaries = primaries;
            value.storage.tf.set_transfer_function(tf);
            let status = value.create_icc();
            debug_assert!(status.is_ok());
        }
        values
    }
    pub fn srgb(is_gray: bool) -> &'static Self {
        static VALUES: OnceLock<[ColorEncoding; 2]> = OnceLock::new();
        &VALUES.get_or_init(|| Self::create_c2(Primaries::SRGB, TransferFunction::SRGB))
            [usize::from(is_gray)]
    }
    pub fn linear_srgb(is_gray: bool) -> &'static Self {
        static VALUES: OnceLock<[ColorEncoding; 2]> = OnceLock::new();
        &VALUES.get_or_init(|| Self::create_c2(Primaries::SRGB, TransferFunction::LINEAR))
            [usize::from(is_gray)]
    }
    pub fn create_icc(&mut self) -> Status {
        self.storage.icc.clear();
        let external = self.to_external();
        if maybe_create_profile(&external, &mut self.storage.icc).is_err() {
            self.storage.icc.clear();
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
    pub fn icc(&self) -> &[u8] {
        &self.storage.icc
    }
    /// Assign ICC bytes and populate fields using the original CMS callback.
    ///
    /// # Safety
    /// Any invoked CMS callback and opaque data must be valid for this call,
    /// obey cms_interface.h, fully initialize successful output, and neither
    /// unwind nor reenter this color owner. The input remains owned by the
    /// caller on error; success transfers its byte allocation.
    pub unsafe fn set_icc(&mut self, icc: &mut Vec<u8>, cms: Option<&JxlCmsInterface>) -> Status {
        let cms = cms.ok_or(StatusCode::GenericError)?;
        if icc.is_empty() {
            return Err(StatusCode::GenericError);
        }
        self.storage.have_fields = true;
        // SAFETY: The caller supplies the callback/opaque-data/output contract;
        // the exclusive owner/input borrows prevent aliases during mutation.
        self.want_icc = unsafe { self.storage.set_fields_from_icc(icc, cms) }.is_ok();
        if self.want_icc {
            Ok(())
        } else {
            Err(StatusCode::GenericError)
        }
    }
    pub fn set_icc_raw(&mut self, icc: Vec<u8>) {
        debug_assert!(!icc.is_empty());
        self.storage.icc = icc;
        self.storage.have_fields = false;
        self.want_icc = true;
    }
    pub fn want_icc(&self) -> bool {
        self.want_icc
    }
    pub fn have_fields(&self) -> bool {
        self.storage.have_fields
    }
    pub fn is_gray(&self) -> bool {
        self.storage.color_space == ColorSpace::GRAY
    }
    pub fn is_cmyk(&self) -> bool {
        self.storage.cmyk
    }
    pub fn channels(&self) -> usize {
        self.storage.channels()
    }
    pub fn has_primaries(&self) -> bool {
        self.storage.has_primaries()
    }
    pub fn implicit_white_point(&mut self) -> bool {
        if self.storage.color_space == ColorSpace::XYB {
            self.storage.white_point = WhitePoint::D65;
            return true;
        }
        false
    }
    pub fn is_srgb(&self) -> bool {
        self.storage.have_fields
            && (self.is_gray() || self.storage.color_space == ColorSpace::RGB)
            && self.storage.white_point == WhitePoint::D65
            && self.storage.primaries == Primaries::SRGB
            && self.storage.tf.is_srgb()
    }
    pub fn is_linear_srgb(&self) -> bool {
        self.storage.have_fields
            && (self.is_gray() || self.storage.color_space == ColorSpace::RGB)
            && self.storage.white_point == WhitePoint::D65
            && self.storage.primaries == Primaries::SRGB
            && self.storage.tf.is_linear()
    }
    pub fn set_srgb(&mut self, cs: ColorSpace, ri: RenderingIntent) -> Status {
        self.storage.icc.clear();
        if cs != ColorSpace::GRAY && cs != ColorSpace::RGB {
            return Err(StatusCode::GenericError);
        }
        self.storage.color_space = cs;
        self.storage.white_point = WhitePoint::D65;
        self.storage.primaries = Primaries::SRGB;
        self.storage.tf.transfer_function = TransferFunction::SRGB;
        self.storage.rendering_intent = ri;
        self.create_icc()
    }
    pub fn get_color_space(&self) -> ColorSpace {
        self.storage.color_space
    }
    pub fn set_color_space(&mut self, cs: ColorSpace) {
        self.storage.color_space = cs;
    }
    pub fn get_white_point(&self) -> cms::CieXy {
        self.storage.get_white_point()
    }
    pub fn get_white_point_type(&self) -> WhitePoint {
        self.storage.white_point
    }
    pub fn set_white_point_type(&mut self, wp: WhitePoint) -> Status {
        if !self.storage.have_fields {
            return Err(StatusCode::GenericError);
        }
        self.storage.white_point = wp;
        Ok(())
    }
    pub fn get_primaries(&self, out: &mut cms::PrimariesCieXy) -> Status {
        self.storage.get_primaries(out)
    }
    pub fn get_primaries_type(&self) -> Primaries {
        self.storage.primaries
    }
    pub fn set_primaries_type(&mut self, p: Primaries) -> Status {
        if !self.storage.have_fields || !self.has_primaries() {
            return Err(StatusCode::GenericError);
        }
        self.storage.primaries = p;
        Ok(())
    }
    pub fn tf(&self) -> &cms::CustomTransferFunction {
        &self.storage.tf
    }
    pub fn tf_mut(&mut self) -> &mut cms::CustomTransferFunction {
        &mut self.storage.tf
    }
    pub fn get_rendering_intent(&self) -> RenderingIntent {
        self.storage.rendering_intent
    }
    pub fn set_rendering_intent(&mut self, ri: RenderingIntent) {
        self.storage.rendering_intent = ri;
    }
    pub fn same_color_encoding(&self, other: &Self) -> bool {
        self.storage.same_color_encoding(&other.storage)
    }
    pub fn to_external(&self) -> JxlColorEncoding {
        self.storage.to_external()
    }
    pub fn from_external(&mut self, external: &JxlColorEncoding) -> Status {
        self.storage.from_external(external)?;
        let _ = self.create_icc();
        Ok(())
    }
    pub fn view(&self) -> &cms::ColorEncoding {
        &self.storage
    }
    pub fn description(&self) -> String {
        color_encoding_description(&self.storage.to_external(), true)
    }
}
impl Fields for ColorEncoding {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        visitor.bool(false, &mut self.want_icc)?;
        visit_enum(visitor, ColorSpace::RGB, &mut self.storage.color_space)?;
        if visitor.conditional(!self.want_icc()) {
            let implicit = self.implicit_white_point();
            if visitor.conditional(!implicit) {
                visit_enum(visitor, WhitePoint::D65, &mut self.storage.white_point)?;
                if visitor.conditional(self.storage.white_point == WhitePoint::CUSTOM) {
                    self.white.storage = self.storage.white;
                    visitor.visit_nested(&mut self.white)?;
                    self.storage.white = self.white.storage;
                }
            }
            if visitor.conditional(self.has_primaries()) {
                visit_enum(visitor, Primaries::SRGB, &mut self.storage.primaries)?;
                if visitor.conditional(self.storage.primaries == Primaries::CUSTOM) {
                    self.red.storage = self.storage.red;
                    visitor.visit_nested(&mut self.red)?;
                    self.storage.red = self.red.storage;
                    self.green.storage = self.storage.green;
                    visitor.visit_nested(&mut self.green)?;
                    self.storage.green = self.green.storage;
                    self.blue.storage = self.storage.blue;
                    visitor.visit_nested(&mut self.blue)?;
                    self.storage.blue = self.blue.storage;
                }
            }
            self.tf.nonserialized_color_space = self.storage.color_space;
            self.tf.storage = self.storage.tf;
            visitor.visit_nested(&mut self.tf)?;
            self.storage.tf = self.tf.storage;
            visit_enum(
                visitor,
                RenderingIntent::RELATIVE,
                &mut self.storage.rendering_intent,
            )?;
            if self.storage.color_space == ColorSpace::UNKNOWN || self.storage.tf.is_unknown() {
                return Err(StatusCode::GenericError);
            }
            self.create_icc()?;
        }
        if !(self.want_icc() && visitor.is_reading()) && self.icc().is_empty() {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
}
