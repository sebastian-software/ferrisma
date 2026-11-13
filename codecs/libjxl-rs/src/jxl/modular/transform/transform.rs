// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Transform storage/fields, channel validation, and PixelAdd from
//! lib/jxl/modular/transform/transform.{h,cc}, including MetaApply/Inverse
//! dispatch to the scalar kernels with the caller's pool. Rust initializes
//! nonserialized max_delta_error storage that has no defined initial C++ value.

use super::super::modular_image::Image;
use super::super::options::Predictor;
use super::palette::{inv_palette, meta_palette};
use super::rct::inv_rct;
use super::squeeze::{inv_squeeze, meta_squeeze};
use super::squeeze_params::SqueezeParams;
use crate::jxl::base::data_parallel::ThreadPool;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::field_encodings::{Fields, U32Enc, bits, bits_offset, val};
use crate::jxl::fields::{Visitor, bundle};
use crate::jxl::modular::encoding::context_predict::weighted;
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TransformId(pub u32);
impl TransformId {
    pub const RCT: Self = Self(0);
    pub const PALETTE: Self = Self(1);
    pub const SQUEEZE: Self = Self(2);
    pub const INVALID: Self = Self(3);
}
#[derive(Clone, Debug)]
pub struct Transform {
    pub id: TransformId,
    pub begin_c: u32,
    pub rct_type: u32,
    pub num_c: u32,
    pub nb_colors: u32,
    pub nb_deltas: u32,
    pub squeezes: Vec<SqueezeParams>,
    pub max_delta_error: i32,
    pub predictor: Predictor,
    pub ordered_palette: bool,
    pub lossy_palette: bool,
}
impl Default for Transform {
    fn default() -> Self {
        Self::new(TransformId::INVALID)
    }
}
impl Transform {
    pub fn new(id: TransformId) -> Self {
        let mut value = Self {
            id: TransformId::RCT,
            begin_c: 0,
            rct_type: 0,
            num_c: 0,
            nb_colors: 0,
            nb_deltas: 0,
            squeezes: Vec::new(),
            max_delta_error: 0,
            predictor: Predictor::ZERO,
            ordered_palette: true,
            lossy_palette: false,
        };
        bundle::init(&mut value);
        value.id = id;
        value
    }
    pub fn inverse(
        &self,
        input: &mut Image,
        wp_header: &weighted::Header,
        pool: Option<&ThreadPool>,
    ) -> Status {
        // public gate: test observation only (decode_trace.rs).
        crate::jxl::decode_trace::note(|| format!("modular inverse transform {}", self.id.0));
        match self.id {
            TransformId::RCT => inv_rct(input, self.begin_c as usize, self.rct_type as usize, pool),
            TransformId::SQUEEZE => inv_squeeze(input, &self.squeezes, pool),
            TransformId::PALETTE => inv_palette(
                input,
                self.begin_c,
                self.nb_colors,
                self.nb_deltas,
                self.predictor,
                wp_header,
                pool,
            ),
            _ => Err(StatusCode::GenericError),
        }
    }
    pub fn meta_apply(&mut self, input: &mut Image) -> Status {
        match self.id {
            TransformId::RCT => {
                check_equal_channels(input, self.begin_c, self.begin_c.wrapping_add(2))
            }
            TransformId::SQUEEZE => meta_squeeze(input, &mut self.squeezes),
            TransformId::PALETTE => meta_palette(
                input,
                self.begin_c,
                self.begin_c.wrapping_add(self.num_c).wrapping_sub(1),
                self.nb_colors,
                self.nb_deltas,
                self.lossy_palette,
            ),
            _ => Err(StatusCode::GenericError),
        }
    }
}
impl Fields for Transform {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.u32(
            U32Enc::new(val(0), val(1), val(2), val(3)),
            0,
            &mut self.id.0,
        )?;
        if self.id == TransformId::INVALID {
            return Err(StatusCode::GenericError);
        }
        if visitor.conditional(self.id == TransformId::RCT || self.id == TransformId::PALETTE) {
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
        }
        if visitor.conditional(self.id == TransformId::RCT) {
            visitor.u32(
                U32Enc::new(val(6), bits(2), bits_offset(4, 2), bits_offset(6, 10)),
                6,
                &mut self.rct_type,
            )?;
            if self.rct_type >= 42 {
                return Err(StatusCode::GenericError);
            }
        }
        if visitor.conditional(self.id == TransformId::PALETTE) {
            visitor.u32(
                U32Enc::new(val(1), val(3), val(4), bits_offset(13, 1)),
                3,
                &mut self.num_c,
            )?;
            visitor.u32(
                U32Enc::new(
                    bits_offset(8, 0),
                    bits_offset(10, 256),
                    bits_offset(12, 1280),
                    bits_offset(16, 5376),
                ),
                256,
                &mut self.nb_colors,
            )?;
            visitor.u32(
                U32Enc::new(
                    val(0),
                    bits_offset(8, 1),
                    bits_offset(10, 257),
                    bits_offset(16, 1281),
                ),
                0,
                &mut self.nb_deltas,
            )?;
            visitor.bits(4, Predictor::ZERO.0, &mut self.predictor.0)?;
            if self.predictor.0 >= Predictor::BEST.0 {
                return Err(StatusCode::GenericError);
            }
        }
        if visitor.conditional(self.id == TransformId::SQUEEZE) {
            let mut count = self.squeezes.len() as u32;
            visitor.u32(
                U32Enc::new(
                    val(0),
                    bits_offset(4, 1),
                    bits_offset(6, 9),
                    bits_offset(8, 41),
                ),
                0,
                &mut count,
            )?;
            if visitor.is_reading() {
                self.squeezes
                    .resize_with(count as usize, SqueezeParams::default);
            }
            for params in &mut self.squeezes[..count as usize] {
                visitor.visit_nested(params)?;
            }
        }
        Ok(())
    }
}
#[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
pub fn check_equal_channels(image: &Image, c1: u32, c2: u32) -> Status {
    let begin = c1 as usize;
    let end = c2 as usize;
    if begin > image.channel.len() || end >= image.channel.len() || c2 < c1 {
        return Err(StatusCode::GenericError);
    }
    if begin < image.nb_meta_channels && end >= image.nb_meta_channels {
        return Err(StatusCode::GenericError);
    }
    let first = &image.channel[begin];
    for c in c1.wrapping_add(1) as usize..=end {
        let other = &image.channel[c];
        if first.w != other.w
            || first.h != other.h
            || first.hshift != other.hshift
            || first.vshift != other.vshift
        {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}
pub const fn pixel_add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}
