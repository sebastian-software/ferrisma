// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Field storage and conversions from `lib/jxl/cms/color_encoding_cms.h`.
//! CMS callback-driven ICC parsing is a separate, unported entry point.
//! Setters preserve the source's partially updated state on errors.

use super::super::base::status::{Status, StatusCode};
use super::super::color_encoding::JxlColorEncoding;
pub use super::super::color_encoding::{
    ColorSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};
pub type IccBytes = Vec<u8>;

pub fn approx_eq(a: f64, b: f64, max_l1: f64) -> bool {
    (a - b).abs() <= max_l1
}
fn approximate(a: f64, b: f64) -> bool {
    approx_eq(a, b, 1e-3)
}
#[derive(Clone, Copy, Debug, Default)]
pub struct CieXy {
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PrimariesCieXy {
    pub r: CieXy,
    pub g: CieXy,
    pub b: CieXy,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct CustomXy {
    pub x: i32,
    pub y: i32,
}
impl CustomXy {
    pub const MUL: u32 = 1_000_000;
    pub const ROUGH_LIMIT: f64 = 4.0;
    pub const MIN: i32 = -0x200000;
    pub const MAX: i32 = 0x1fffff;
    pub fn get_value(&self) -> CieXy {
        CieXy {
            x: f64::from(self.x) * (1.0 / f64::from(Self::MUL)),
            y: f64::from(self.y) * (1.0 / f64::from(Self::MUL)),
        }
    }
    pub fn set_value(&mut self, xy: CieXy) -> Status {
        if !(xy.x.abs() < Self::ROUGH_LIMIT && xy.y.abs() < Self::ROUGH_LIMIT) {
            return Err(StatusCode::GenericError);
        }
        self.x = ((xy.x * f64::from(Self::MUL)) as f32).round() as i32;
        if self.x < Self::MIN || self.x > Self::MAX {
            return Err(StatusCode::GenericError);
        }
        self.y = ((xy.y * f64::from(Self::MUL)) as f32).round() as i32;
        if self.y < Self::MIN || self.y > Self::MAX {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
    pub fn is_same(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
pub fn white_point_from_external(value: WhitePoint, out: &mut WhitePoint) -> Status {
    match value {
        WhitePoint::D65 | WhitePoint::CUSTOM | WhitePoint::E | WhitePoint::DCI => {
            *out = value;
            Ok(())
        }
        _ => Err(StatusCode::GenericError),
    }
}
pub fn primaries_from_external(value: Primaries, out: &mut Primaries) -> Status {
    match value {
        Primaries::SRGB | Primaries::CUSTOM | Primaries::BT2100 | Primaries::P3 => {
            *out = value;
            Ok(())
        }
        _ => Err(StatusCode::GenericError),
    }
}
pub fn rendering_intent_from_external(value: RenderingIntent, out: &mut RenderingIntent) -> Status {
    match value {
        RenderingIntent::PERCEPTUAL
        | RenderingIntent::RELATIVE
        | RenderingIntent::SATURATION
        | RenderingIntent::ABSOLUTE => {
            *out = value;
            Ok(())
        }
        _ => Err(StatusCode::GenericError),
    }
}
pub fn convert_external_to_internal_transfer_function(
    value: TransferFunction,
    out: &mut TransferFunction,
) -> Status {
    match value {
        TransferFunction::BT709
        | TransferFunction::UNKNOWN
        | TransferFunction::LINEAR
        | TransferFunction::SRGB
        | TransferFunction::PQ
        | TransferFunction::DCI
        | TransferFunction::HLG => {
            *out = value;
            Ok(())
        }
        _ => Err(StatusCode::GenericError),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CustomTransferFunction {
    pub have_gamma: bool,
    pub gamma: u32,
    pub transfer_function: TransferFunction,
}
impl Default for CustomTransferFunction {
    fn default() -> Self {
        Self {
            have_gamma: false,
            gamma: 0,
            transfer_function: TransferFunction::SRGB,
        }
    }
}
impl CustomTransferFunction {
    pub const MAX_GAMMA: u32 = 8192;
    pub const GAMMA_MUL: u32 = 10_000_000;
    pub fn get_transfer_function(&self) -> TransferFunction {
        debug_assert!(!self.have_gamma);
        if self.have_gamma {
            TransferFunction::UNKNOWN
        } else {
            self.transfer_function
        }
    }
    pub fn set_transfer_function(&mut self, tf: TransferFunction) {
        self.have_gamma = false;
        self.transfer_function = tf;
    }
    pub fn is_unknown(&self) -> bool {
        !self.have_gamma && self.transfer_function == TransferFunction::UNKNOWN
    }
    pub fn is_srgb(&self) -> bool {
        !self.have_gamma && self.transfer_function == TransferFunction::SRGB
    }
    pub fn is_linear(&self) -> bool {
        !self.have_gamma && self.transfer_function == TransferFunction::LINEAR
    }
    pub fn is_pq(&self) -> bool {
        !self.have_gamma && self.transfer_function == TransferFunction::PQ
    }
    pub fn is_hlg(&self) -> bool {
        !self.have_gamma && self.transfer_function == TransferFunction::HLG
    }
    pub fn is709(&self) -> bool {
        !self.have_gamma && self.transfer_function == TransferFunction::BT709
    }
    pub fn is_dci(&self) -> bool {
        !self.have_gamma && self.transfer_function == TransferFunction::DCI
    }
    pub fn get_gamma(&self) -> f64 {
        debug_assert!(self.have_gamma);
        if !self.have_gamma {
            0.0
        } else {
            f64::from(self.gamma) * (1.0 / f64::from(Self::GAMMA_MUL))
        }
    }
    pub fn set_gamma(&mut self, gamma: f64) -> Status {
        if gamma < 1.0 / f64::from(Self::MAX_GAMMA) || gamma > 1.0 {
            return Err(StatusCode::GenericError);
        }
        self.have_gamma = false;
        if approximate(gamma, 1.0) {
            self.transfer_function = TransferFunction::LINEAR;
            return Ok(());
        }
        if approximate(gamma, 1.0 / 2.6) {
            self.transfer_function = TransferFunction::DCI;
            return Ok(());
        }
        self.have_gamma = true;
        self.gamma = ((gamma * f64::from(Self::GAMMA_MUL)) as f32).round() as u32;
        self.transfer_function = TransferFunction::UNKNOWN;
        Ok(())
    }
    pub fn is_same(&self, other: &Self) -> bool {
        if self.have_gamma != other.have_gamma {
            return false;
        }
        if self.have_gamma {
            self.gamma == other.gamma
        } else {
            self.transfer_function == other.transfer_function
        }
    }
}
#[derive(Clone, Debug)]
pub struct ColorEncoding {
    pub white_point: WhitePoint,
    pub primaries: Primaries,
    pub rendering_intent: RenderingIntent,
    pub have_fields: bool,
    pub icc: IccBytes,
    pub color_space: ColorSpace,
    pub cmyk: bool,
    pub tf: CustomTransferFunction,
    pub white: CustomXy,
    pub red: CustomXy,
    pub green: CustomXy,
    pub blue: CustomXy,
}
impl Default for ColorEncoding {
    fn default() -> Self {
        Self {
            white_point: WhitePoint::D65,
            primaries: Primaries::SRGB,
            rendering_intent: RenderingIntent::RELATIVE,
            have_fields: true,
            icc: Vec::new(),
            color_space: ColorSpace::RGB,
            cmyk: false,
            tf: CustomTransferFunction::default(),
            white: CustomXy::default(),
            red: CustomXy::default(),
            green: CustomXy::default(),
            blue: CustomXy::default(),
        }
    }
}
impl ColorEncoding {
    pub fn has_primaries(&self) -> bool {
        self.color_space != ColorSpace::GRAY && self.color_space != ColorSpace::XYB
    }
    pub fn channels(&self) -> usize {
        if self.color_space == ColorSpace::GRAY {
            1
        } else {
            3
        }
    }
    pub fn get_primaries(&self, xy: &mut PrimariesCieXy) -> Status {
        if !self.have_fields || !self.has_primaries() {
            return Err(StatusCode::GenericError);
        }
        *xy = PrimariesCieXy::default();
        match self.primaries {
            Primaries::CUSTOM => {
                xy.r = self.red.get_value();
                xy.g = self.green.get_value();
                xy.b = self.blue.get_value();
            }
            Primaries::SRGB => {
                xy.r = CieXy {
                    x: 0.639998686,
                    y: 0.330010138,
                };
                xy.g = CieXy {
                    x: 0.300003784,
                    y: 0.600003357,
                };
                xy.b = CieXy {
                    x: 0.150002046,
                    y: 0.059997204,
                };
            }
            Primaries::BT2100 => {
                xy.r = CieXy { x: 0.708, y: 0.292 };
                xy.g = CieXy { x: 0.170, y: 0.797 };
                xy.b = CieXy { x: 0.131, y: 0.046 };
            }
            Primaries::P3 => {
                xy.r = CieXy { x: 0.680, y: 0.320 };
                xy.g = CieXy { x: 0.265, y: 0.690 };
                xy.b = CieXy { x: 0.150, y: 0.060 };
            }
            _ => debug_assert!(false, "invalid internal primaries"),
        }
        Ok(())
    }
    pub fn set_primaries(&mut self, xy: PrimariesCieXy) -> Status {
        if !self.have_fields || !self.has_primaries() {
            return Err(StatusCode::GenericError);
        }
        if xy.r.x == 0.0
            || xy.r.y == 0.0
            || xy.g.x == 0.0
            || xy.g.y == 0.0
            || xy.b.x == 0.0
            || xy.b.y == 0.0
        {
            return Err(StatusCode::GenericError);
        }
        if approximate(xy.r.x, 0.64)
            && approximate(xy.r.y, 0.33)
            && approximate(xy.g.x, 0.30)
            && approximate(xy.g.y, 0.60)
            && approximate(xy.b.x, 0.15)
            && approximate(xy.b.y, 0.06)
        {
            self.primaries = Primaries::SRGB;
            return Ok(());
        }
        if approximate(xy.r.x, 0.708)
            && approximate(xy.r.y, 0.292)
            && approximate(xy.g.x, 0.170)
            && approximate(xy.g.y, 0.797)
            && approximate(xy.b.x, 0.131)
            && approximate(xy.b.y, 0.046)
        {
            self.primaries = Primaries::BT2100;
            return Ok(());
        }
        if approximate(xy.r.x, 0.680)
            && approximate(xy.r.y, 0.320)
            && approximate(xy.g.x, 0.265)
            && approximate(xy.g.y, 0.690)
            && approximate(xy.b.x, 0.150)
            && approximate(xy.b.y, 0.060)
        {
            self.primaries = Primaries::P3;
            return Ok(());
        }
        self.primaries = Primaries::CUSTOM;
        self.red.set_value(xy.r)?;
        self.green.set_value(xy.g)?;
        self.blue.set_value(xy.b)
    }
    pub fn get_white_point(&self) -> CieXy {
        let mut xy = CieXy::default();
        debug_assert!(self.have_fields);
        if !self.have_fields {
            return xy;
        }
        match self.white_point {
            WhitePoint::CUSTOM => xy = self.white.get_value(),
            WhitePoint::D65 => {
                xy = CieXy {
                    x: 0.3127,
                    y: 0.3290,
                }
            }
            WhitePoint::DCI => xy = CieXy { x: 0.314, y: 0.351 },
            WhitePoint::E => {
                xy = CieXy {
                    x: 1.0 / 3.0,
                    y: 1.0 / 3.0,
                }
            }
            _ => debug_assert!(false, "invalid internal white point"),
        }
        xy
    }
    pub fn set_white_point(&mut self, xy: CieXy) -> Status {
        if !self.have_fields || xy.x == 0.0 || xy.y == 0.0 {
            return Err(StatusCode::GenericError);
        }
        if approximate(xy.x, 0.3127) && approximate(xy.y, 0.3290) {
            self.white_point = WhitePoint::D65;
            return Ok(());
        }
        if approximate(xy.x, 1.0 / 3.0) && approximate(xy.y, 1.0 / 3.0) {
            self.white_point = WhitePoint::E;
            return Ok(());
        }
        if approximate(xy.x, 0.314) && approximate(xy.y, 0.351) {
            self.white_point = WhitePoint::DCI;
            return Ok(());
        }
        self.white_point = WhitePoint::CUSTOM;
        self.white.set_value(xy)
    }
    pub fn same_color_space(&self, other: &Self) -> bool {
        if self.color_space != other.color_space || self.white_point != other.white_point {
            return false;
        }
        if self.white_point == WhitePoint::CUSTOM && !self.white.is_same(&other.white) {
            return false;
        }
        if self.has_primaries() != other.has_primaries() {
            return false;
        }
        if self.has_primaries() {
            if self.primaries != other.primaries {
                return false;
            }
            if self.primaries == Primaries::CUSTOM
                && (!self.red.is_same(&other.red)
                    || !self.green.is_same(&other.green)
                    || !self.blue.is_same(&other.blue))
            {
                return false;
            }
        }
        true
    }
    pub fn same_color_encoding(&self, other: &Self) -> bool {
        self.same_color_space(other) && self.tf.is_same(&other.tf) && !self.cmyk && !other.cmyk
    }
    pub fn to_external(&self) -> JxlColorEncoding {
        let mut external = JxlColorEncoding::default();
        let set_error = |out: &mut JxlColorEncoding| {
            out.color_space = ColorSpace::UNKNOWN;
            out.primaries = Primaries::CUSTOM;
            out.rendering_intent = RenderingIntent::PERCEPTUAL;
            out.transfer_function = TransferFunction::UNKNOWN;
            out.white_point = WhitePoint::CUSTOM;
        };
        if !self.have_fields {
            set_error(&mut external);
            return external;
        }
        external.color_space = self.color_space;
        external.white_point = self.white_point;
        let wp = self.get_white_point();
        external.white_point_xy = [wp.x, wp.y];
        if self.color_space == ColorSpace::RGB || self.color_space == ColorSpace::UNKNOWN {
            external.primaries = self.primaries;
            let mut p = PrimariesCieXy::default();
            if self.get_primaries(&mut p).is_err() {
                set_error(&mut external);
                return external;
            }
            external.primaries_red_xy = [p.r.x, p.r.y];
            external.primaries_green_xy = [p.g.x, p.g.y];
            external.primaries_blue_xy = [p.b.x, p.b.y];
        }
        if self.tf.have_gamma {
            external.transfer_function = TransferFunction::GAMMA;
            external.gamma = self.tf.get_gamma();
        } else {
            external.transfer_function = self.tf.get_transfer_function();
            external.gamma = 0.0;
        }
        external.rendering_intent = self.rendering_intent;
        external
    }
    pub fn from_external(&mut self, external: &JxlColorEncoding) -> Status {
        self.color_space = external.color_space;
        white_point_from_external(external.white_point, &mut self.white_point)?;
        if external.white_point == WhitePoint::CUSTOM {
            self.set_white_point(CieXy {
                x: external.white_point_xy[0],
                y: external.white_point_xy[1],
            })?;
        }
        if external.color_space == ColorSpace::RGB || external.color_space == ColorSpace::UNKNOWN {
            primaries_from_external(external.primaries, &mut self.primaries)?;
            if external.primaries == Primaries::CUSTOM {
                self.set_primaries(PrimariesCieXy {
                    r: CieXy {
                        x: external.primaries_red_xy[0],
                        y: external.primaries_red_xy[1],
                    },
                    g: CieXy {
                        x: external.primaries_green_xy[0],
                        y: external.primaries_green_xy[1],
                    },
                    b: CieXy {
                        x: external.primaries_blue_xy[0],
                        y: external.primaries_blue_xy[1],
                    },
                })?;
            }
        }
        let mut tf = CustomTransferFunction::default();
        if external.transfer_function == TransferFunction::GAMMA {
            tf.set_gamma(external.gamma)?;
        } else {
            let mut value = TransferFunction::default();
            convert_external_to_internal_transfer_function(external.transfer_function, &mut value)?;
            tf.set_transfer_function(value);
        }
        self.tf = tf;
        rendering_intent_from_external(external.rendering_intent, &mut self.rendering_intent)?;
        self.icc.clear();
        Ok(())
    }
}
