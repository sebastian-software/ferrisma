// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/frame_header.{h,cc}`. Metadata pointers become optional
//! shared borrows. Byte names, field order, conditional state, and partial
//! writes remain explicit. Diagnostic string formatting remains unported.

use super::base::status::{Status, StatusCode};
use super::common::MAX_NUM_PASSES;
use super::dec_bit_reader::BitReader;
use super::field_encodings::{Fields, U32Enc, bits, bits_offset, val};
use super::fields::{Visitor, bundle};
use super::frame_dimensions::FrameDimensions;
use super::image_metadata::CodecMetadata;
use super::loop_filter::LoopFilter;
use super::pack_signed::{pack_signed, unpack_signed};

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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FrameEncoding(pub u32);
impl FrameEncoding {
    pub const VAR_DCT: Self = Self(0);
    pub const MODULAR: Self = Self(1);
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColorTransform(pub u32);
impl ColorTransform {
    pub const XYB: Self = Self(0);
    pub const NONE: Self = Self(1);
    pub const YCBCR: Self = Self(2);
}
pub fn jpeg_order(transform: ColorTransform, is_gray: bool) -> [i32; 3] {
    if is_gray {
        [0, 0, 0]
    } else if transform == ColorTransform::YCBCR {
        [1, 0, 2]
    } else {
        debug_assert!(
            transform == ColorTransform::NONE,
            "invalid transform for JPEG order"
        );
        [0, 1, 2]
    }
}
#[derive(Clone, Debug)]
pub struct YCbCrChromaSubsampling {
    channel_mode: [u32; 3],
    maxhs: u8,
    maxvs: u8,
}
impl Default for YCbCrChromaSubsampling {
    fn default() -> Self {
        let mut value = Self {
            channel_mode: [0; 3],
            maxhs: 0,
            maxvs: 0,
        };
        bundle::init(&mut value);
        value
    }
}
impl YCbCrChromaSubsampling {
    const HSHIFT: [u8; 4] = [0, 1, 1, 0];
    const VSHIFT: [u8; 4] = [0, 1, 0, 1];
    fn recompute(&mut self) {
        self.maxhs = 0;
        self.maxvs = 0;
        for &ch in &self.channel_mode {
            self.maxhs = self.maxhs.max(Self::HSHIFT[ch as usize]);
            self.maxvs = self.maxvs.max(Self::VSHIFT[ch as usize]);
        }
    }
    pub fn h_shift(&self, c: usize) -> usize {
        (i32::from(self.maxhs) - i32::from(Self::HSHIFT[self.channel_mode[c] as usize])) as usize
    }
    pub fn v_shift(&self, c: usize) -> usize {
        (i32::from(self.maxvs) - i32::from(Self::VSHIFT[self.channel_mode[c] as usize])) as usize
    }
    pub fn max_h_shift(&self) -> u8 {
        self.maxhs
    }
    pub fn max_v_shift(&self) -> u8 {
        self.maxvs
    }
    pub fn raw_h_shift(&self, c: usize) -> u8 {
        Self::HSHIFT[self.channel_mode[c] as usize]
    }
    pub fn raw_v_shift(&self, c: usize) -> u8 {
        Self::VSHIFT[self.channel_mode[c] as usize]
    }
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    pub fn set(&mut self, hsample: &[u8; 3], vsample: &[u8; 3]) -> Status {
        for c in 0..3 {
            let jpeg = if c < 2 { c ^ 1 } else { c };
            let mut i = 0;
            while i < 4 {
                if (1u8 << Self::HSHIFT[i]) == hsample[jpeg]
                    && (1u8 << Self::VSHIFT[i]) == vsample[jpeg]
                {
                    self.channel_mode[c] = i as u32;
                    break;
                }
                i += 1;
            }
            if i == 4 {
                return Err(StatusCode::GenericError);
            }
        }
        self.recompute();
        Ok(())
    }
    pub fn is_444(&self) -> bool {
        [0, 2, 1]
            .iter()
            .all(|&c| self.h_shift(c) == 0 && self.v_shift(c) == 0)
    }
    pub fn is_420(&self) -> bool {
        self.h_shift(0) == 1
            && self.v_shift(0) == 1
            && self.h_shift(2) == 1
            && self.v_shift(2) == 1
            && self.h_shift(1) == 0
            && self.v_shift(1) == 0
    }
    pub fn is_422(&self) -> bool {
        self.h_shift(0) == 1
            && self.v_shift(0) == 0
            && self.h_shift(2) == 1
            && self.v_shift(2) == 0
            && self.h_shift(1) == 0
            && self.v_shift(1) == 0
    }
    pub fn is_440(&self) -> bool {
        self.h_shift(0) == 0
            && self.v_shift(0) == 1
            && self.h_shift(2) == 0
            && self.v_shift(2) == 1
            && self.h_shift(1) == 0
            && self.v_shift(1) == 0
    }
}
impl Fields for YCbCrChromaSubsampling {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        for ch in &mut self.channel_mode {
            visitor.bits(2, 0, ch)?;
        }
        self.recompute();
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BlendMode(pub u32);
impl BlendMode {
    pub const REPLACE: Self = Self(0);
    pub const ADD: Self = Self(1);
    pub const BLEND: Self = Self(2);
    pub const ALPHA_WEIGHTED_ADD: Self = Self(3);
    pub const MUL: Self = Self(4);
}
fn visit_blend_mode(
    visitor: &mut dyn Visitor,
    default: BlendMode,
    value: &mut BlendMode,
) -> Status {
    let mut encoded = value.0;
    visitor.u32(
        U32Enc::new(val(0), val(1), val(2), bits_offset(2, 3)),
        default.0,
        &mut encoded,
    )?;
    if encoded > BlendMode::MUL.0 {
        return Err(StatusCode::GenericError);
    }
    *value = BlendMode(encoded);
    Ok(())
}
#[derive(Clone, Debug)]
pub struct BlendingInfo {
    pub mode: BlendMode,
    pub alpha_channel: u32,
    pub clamp: bool,
    pub source: u32,
    pub nonserialized_num_extra_channels: usize,
    pub nonserialized_is_partial_frame: bool,
}
impl Default for BlendingInfo {
    fn default() -> Self {
        let mut value = Self {
            mode: BlendMode::REPLACE,
            alpha_channel: 0,
            clamp: false,
            source: 0,
            nonserialized_num_extra_channels: 0,
            nonserialized_is_partial_frame: false,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for BlendingInfo {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visit_blend_mode(visitor, BlendMode::REPLACE, &mut self.mode)?;
        if visitor.conditional(
            self.nonserialized_num_extra_channels > 0
                && (self.mode == BlendMode::BLEND || self.mode == BlendMode::ALPHA_WEIGHTED_ADD),
        ) {
            visitor.u32(
                U32Enc::new(val(0), val(1), val(2), bits_offset(3, 3)),
                0,
                &mut self.alpha_channel,
            )?;
            if visitor.is_reading()
                && self.alpha_channel as usize >= self.nonserialized_num_extra_channels
            {
                return Err(StatusCode::GenericError);
            }
        }
        if visitor.conditional(
            (self.nonserialized_num_extra_channels > 0
                && (self.mode == BlendMode::BLEND || self.mode == BlendMode::ALPHA_WEIGHTED_ADD))
                || self.mode == BlendMode::MUL,
        ) {
            visitor.bool(false, &mut self.clamp)?;
        }
        if visitor
            .conditional(self.mode != BlendMode::REPLACE || self.nonserialized_is_partial_frame)
        {
            visitor.u32(
                U32Enc::new(val(0), val(1), val(2), val(3)),
                0,
                &mut self.source,
            )?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameOrigin {
    pub x0: i32,
    pub y0: i32,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameSize {
    pub xsize: u32,
    pub ysize: u32,
}
#[derive(Clone)]
pub struct AnimationFrame<'a> {
    pub duration: u32,
    pub timecode: u32,
    pub nonserialized_metadata: Option<&'a CodecMetadata>,
}
impl<'a> AnimationFrame<'a> {
    pub fn new(metadata: Option<&'a CodecMetadata>) -> Self {
        let mut value = Self {
            duration: 0,
            timecode: 0,
            nonserialized_metadata: metadata,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for AnimationFrame<'_> {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        if visitor.conditional(
            self.nonserialized_metadata
                .is_some_and(|m| m.m.have_animation),
        ) {
            visitor.u32(
                U32Enc::new(val(0), val(1), bits(8), bits(32)),
                0,
                &mut self.duration,
            )?;
        }
        if visitor.conditional(
            self.nonserialized_metadata
                .is_some_and(|m| m.m.animation.have_timecodes),
        ) {
            visitor.bits(32, 0, &mut self.timecode)?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct Passes {
    pub num_passes: u32,
    pub num_downsample: u32,
    pub downsample: [u32; MAX_NUM_PASSES],
    pub last_pass: [u32; MAX_NUM_PASSES],
    pub shift: [u32; MAX_NUM_PASSES],
}
impl Default for Passes {
    fn default() -> Self {
        let mut value = Self {
            num_passes: 0,
            num_downsample: 0,
            downsample: [0; MAX_NUM_PASSES],
            last_pass: [0; MAX_NUM_PASSES],
            shift: [0; MAX_NUM_PASSES],
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for Passes {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.u32(
            U32Enc::new(val(1), val(2), val(3), bits_offset(3, 4)),
            1,
            &mut self.num_passes,
        )?;
        if self.num_passes as usize > MAX_NUM_PASSES {
            return Err(StatusCode::GenericError);
        }
        if visitor.conditional(self.num_passes != 1) {
            visitor.u32(
                U32Enc::new(val(0), val(1), val(2), bits_offset(1, 3)),
                0,
                &mut self.num_downsample,
            )?;
            if self.num_downsample > 4 || self.num_downsample > self.num_passes {
                return Err(StatusCode::GenericError);
            }
            for i in 0..self.num_passes as usize - 1 {
                visitor.bits(2, 0, &mut self.shift[i])?;
            }
            self.shift[self.num_passes as usize - 1] = 0;
            for i in 0..self.num_downsample as usize {
                visitor.u32(
                    U32Enc::new(val(1), val(2), val(4), val(8)),
                    1,
                    &mut self.downsample[i],
                )?;
                if i > 0 && self.downsample[i] >= self.downsample[i - 1] {
                    return Err(StatusCode::GenericError);
                }
            }
            for i in 0..self.num_downsample as usize {
                visitor.u32(
                    U32Enc::new(val(0), val(1), val(2), bits(3)),
                    0,
                    &mut self.last_pass[i],
                )?;
                if i > 0 && self.last_pass[i] <= self.last_pass[i - 1] {
                    return Err(StatusCode::GenericError);
                }
                if self.last_pass[i] >= self.num_passes {
                    return Err(StatusCode::GenericError);
                }
            }
        }
        Ok(())
    }
}
impl Passes {
    pub fn get_downsampling_bracket(&self, pass: usize) -> (i32, i32) {
        let mut max_shift = 2;
        let mut min_shift = 3;
        for i in 0..=pass {
            for j in 0..self.num_downsample as usize {
                if i == self.last_pass[j] as usize {
                    match self.downsample[j] {
                        8 => min_shift = 3,
                        4 => min_shift = 2,
                        2 => min_shift = 1,
                        1 => min_shift = 0,
                        _ => {}
                    }
                }
            }
            if i == self.num_passes as usize - 1 {
                min_shift = 0;
            }
            if i == pass {
                return (min_shift, max_shift);
            }
            max_shift = min_shift - 1;
        }
        unreachable!()
    }
    pub fn get_downsampling_target_for_completed_passes(&self, num: u32) -> u32 {
        if num >= self.num_passes {
            return 1;
        }
        let mut result = 8;
        for i in 0..self.num_downsample as usize {
            if num > self.last_pass[i] {
                result = result.min(self.downsample[i]);
            }
        }
        result
    }
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FrameType(pub u32);
impl FrameType {
    pub const REGULAR: Self = Self(0);
    pub const DC: Self = Self(1);
    pub const REFERENCE_ONLY: Self = Self(2);
    pub const SKIP_PROGRESSIVE: Self = Self(3);
}
fn visit_frame_type(
    visitor: &mut dyn Visitor,
    default: FrameType,
    value: &mut FrameType,
) -> Status {
    let mut encoded = value.0;
    visitor.u32(
        U32Enc::new(val(0), val(1), val(2), val(3)),
        default.0,
        &mut encoded,
    )?;
    *value = FrameType(encoded);
    Ok(())
}
pub const FLAG_NOISE: u64 = 1;
pub const FLAG_PATCHES: u64 = 2;
pub const FLAG_SPLINES: u64 = 16;
pub const FLAG_USE_DC_FRAME: u64 = 32;
pub const FLAG_SKIP_ADAPTIVE_DC_SMOOTHING: u64 = 128;
#[derive(Clone)]
pub struct FrameHeader<'a> {
    pub all_default: bool,
    pub encoding: FrameEncoding,
    pub frame_type: FrameType,
    pub flags: u64,
    pub color_transform: ColorTransform,
    pub chroma_subsampling: YCbCrChromaSubsampling,
    pub group_size_shift: u32,
    pub x_qm_scale: u32,
    pub b_qm_scale: u32,
    pub name: Vec<u8>,
    pub passes: Passes,
    pub custom_size_or_origin: bool,
    pub frame_size: FrameSize,
    pub upsampling: u32,
    pub extra_channel_upsampling: Vec<u32>,
    pub frame_origin: FrameOrigin,
    pub blending_info: BlendingInfo,
    pub extra_channel_blending_info: Vec<BlendingInfo>,
    pub animation_frame: AnimationFrame<'a>,
    pub is_last: bool,
    pub save_as_reference: u32,
    pub save_before_color_transform: bool,
    pub dc_level: u32,
    pub nonserialized_metadata: Option<&'a CodecMetadata>,
    pub loop_filter: LoopFilter,
    pub nonserialized_is_preview: bool,
    pub extensions: u64,
}
impl<'a> FrameHeader<'a> {
    pub fn new(metadata: Option<&'a CodecMetadata>) -> Self {
        let mut value = Self {
            all_default: false,
            encoding: FrameEncoding::MODULAR,
            frame_type: FrameType::REGULAR,
            flags: 0,
            color_transform: ColorTransform::XYB,
            chroma_subsampling: YCbCrChromaSubsampling::default(),
            group_size_shift: 0,
            x_qm_scale: 0,
            b_qm_scale: 0,
            name: Vec::new(),
            passes: Passes::default(),
            custom_size_or_origin: false,
            frame_size: FrameSize::default(),
            upsampling: 0,
            extra_channel_upsampling: Vec::new(),
            frame_origin: FrameOrigin::default(),
            blending_info: BlendingInfo::default(),
            extra_channel_blending_info: Vec::new(),
            animation_frame: AnimationFrame::new(metadata),
            is_last: false,
            save_as_reference: 0,
            save_before_color_transform: false,
            dc_level: 0,
            nonserialized_metadata: metadata,
            loop_filter: LoopFilter::default(),
            nonserialized_is_preview: false,
            extensions: 0,
        };
        bundle::init(&mut value);
        value
    }
    pub fn update_flag(&mut self, condition: bool, flag: u64) {
        if condition {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }
    pub fn can_be_referenced(&self) -> bool {
        !self.is_last
            && self.frame_type != FrameType::DC
            && (self.animation_frame.duration == 0 || self.save_as_reference != 0)
    }
    pub fn default_xsize(&self) -> usize {
        self.nonserialized_metadata.map_or(0, |m| {
            if self.nonserialized_is_preview {
                m.m.preview_size.xsize()
            } else {
                m.xsize()
            }
        })
    }
    pub fn default_ysize(&self) -> usize {
        self.nonserialized_metadata.map_or(0, |m| {
            if self.nonserialized_is_preview {
                m.m.preview_size.ysize()
            } else {
                m.ysize()
            }
        })
    }
    pub fn to_frame_dimensions(&self) -> FrameDimensions {
        let mut x = self.default_xsize();
        let mut y = self.default_ysize();
        if self.frame_size.xsize != 0 {
            x = self.frame_size.xsize as usize;
        }
        if self.frame_size.ysize != 0 {
            y = self.frame_size.ysize as usize;
        }
        if self.dc_level != 0 {
            let d = 1usize << (3 * self.dc_level);
            x = x.wrapping_add(d - 1) / d;
            y = y.wrapping_add(d - 1) / d;
        }
        let mut result = FrameDimensions::default();
        result.set(
            x,
            y,
            self.group_size_shift as usize,
            self.chroma_subsampling.max_h_shift() as usize,
            self.chroma_subsampling.max_v_shift() as usize,
            self.encoding == FrameEncoding::MODULAR,
            self.upsampling as usize,
        );
        result
    }
    pub fn needs_color_transform(&self) -> bool {
        !self.save_before_color_transform
            || self.frame_type == FrameType::REGULAR
            || self.frame_type == FrameType::SKIP_PROGRESSIVE
    }
}
impl Fields for FrameHeader<'_> {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        visit_frame_type(visitor, FrameType::REGULAR, &mut self.frame_type)?;
        if visitor.is_reading()
            && self.nonserialized_is_preview
            && self.frame_type != FrameType::REGULAR
        {
            return Err(StatusCode::GenericError);
        }
        let mut modular = self.encoding == FrameEncoding::MODULAR;
        visitor.bool(false, &mut modular)?;
        self.encoding = if modular {
            FrameEncoding::MODULAR
        } else {
            FrameEncoding::VAR_DCT
        };
        visitor.u64(0, &mut self.flags)?;
        let xyb = self.nonserialized_metadata.is_none_or(|m| m.m.xyb_encoded);
        if xyb {
            self.color_transform = ColorTransform::XYB;
        } else {
            let mut alternate = self.color_transform == ColorTransform::YCBCR;
            visitor.bool(false, &mut alternate)?;
            self.color_transform = if alternate {
                ColorTransform::YCBCR
            } else {
                ColorTransform::NONE
            };
        }
        if visitor.conditional(
            self.color_transform == ColorTransform::YCBCR && self.flags & FLAG_USE_DC_FRAME == 0,
        ) {
            visitor.visit_nested(&mut self.chroma_subsampling)?;
        }
        let num_extra = self
            .nonserialized_metadata
            .map_or(0, |m| m.m.extra_channel_info.len());
        if visitor.conditional(self.flags & FLAG_USE_DC_FRAME == 0) {
            visitor.u32(
                U32Enc::new(val(1), val(2), val(4), val(8)),
                1,
                &mut self.upsampling,
            )?;
            if let Some(metadata) = self
                .nonserialized_metadata
                .filter(|_| visitor.conditional(num_extra != 0))
            {
                self.extra_channel_upsampling
                    .resize(metadata.m.extra_channel_info.len(), 1);
                for (i, info) in metadata.m.extra_channel_info.iter().enumerate() {
                    let upsampling = &mut self.extra_channel_upsampling[i];
                    *upsampling >>= info.dim_shift;
                    visitor.u32(U32Enc::new(val(1), val(2), val(4), val(8)), 1, upsampling)?;
                    *upsampling <<= info.dim_shift;
                    if *upsampling < self.upsampling || *upsampling > 8 {
                        return Err(StatusCode::GenericError);
                    }
                }
            } else {
                self.extra_channel_upsampling.clear();
            }
        }
        if visitor.conditional(self.encoding == FrameEncoding::MODULAR) {
            visitor.bits(2, 1, &mut self.group_size_shift)?;
        }
        if visitor.conditional(
            self.encoding == FrameEncoding::VAR_DCT && self.color_transform == ColorTransform::XYB,
        ) {
            visitor.bits(3, 3, &mut self.x_qm_scale)?;
            visitor.bits(3, 2, &mut self.b_qm_scale)?;
        } else {
            self.x_qm_scale = 2;
            self.b_qm_scale = 2;
        }
        if visitor.conditional(self.frame_type != FrameType::REFERENCE_ONLY) {
            visitor.visit_nested(&mut self.passes)?;
        }
        if visitor.conditional(self.frame_type == FrameType::DC) {
            visitor.u32(
                U32Enc::new(val(1), val(2), val(3), val(4)),
                1,
                &mut self.dc_level,
            )?;
        }
        if self.frame_type != FrameType::DC {
            self.dc_level = 0;
        }
        let mut partial = false;
        if visitor.conditional(self.frame_type != FrameType::DC) {
            visitor.bool(false, &mut self.custom_size_or_origin)?;
            if visitor.conditional(self.custom_size_or_origin) {
                let enc = U32Enc::new(
                    bits(8),
                    bits_offset(11, 256),
                    bits_offset(14, 2304),
                    bits_offset(30, 18688),
                );
                if visitor.conditional(
                    self.frame_type == FrameType::REGULAR
                        || self.frame_type == FrameType::SKIP_PROGRESSIVE,
                ) {
                    let mut x = pack_signed(self.frame_origin.x0);
                    let mut y = pack_signed(self.frame_origin.y0);
                    visitor.u32(enc, 0, &mut x)?;
                    visitor.u32(enc, 0, &mut y)?;
                    self.frame_origin.x0 = unpack_signed(x as usize);
                    self.frame_origin.y0 = unpack_signed(y as usize);
                }
                visitor.u32(enc, 0, &mut self.frame_size.xsize)?;
                visitor.u32(enc, 0, &mut self.frame_size.ysize)?;
                if self.custom_size_or_origin
                    && (self.frame_size.xsize == 0 || self.frame_size.ysize == 0)
                {
                    return Err(StatusCode::GenericError);
                }
                let image_x = self.default_xsize() as i32;
                let image_y = self.default_ysize() as i32;
                if self.frame_type == FrameType::REGULAR
                    || self.frame_type == FrameType::SKIP_PROGRESSIVE
                {
                    partial |= self.frame_origin.x0 > 0;
                    partial |= self.frame_origin.y0 > 0;
                    // Valid source dimensions avoid signed overflow here.
                    partial |=
                        (self.frame_size.xsize as i32).wrapping_add(self.frame_origin.x0) < image_x;
                    partial |=
                        (self.frame_size.ysize as i32).wrapping_add(self.frame_origin.y0) < image_y;
                }
            }
        }
        if visitor.conditional(
            self.frame_type == FrameType::REGULAR || self.frame_type == FrameType::SKIP_PROGRESSIVE,
        ) {
            self.blending_info.nonserialized_num_extra_channels = num_extra;
            self.blending_info.nonserialized_is_partial_frame = partial;
            visitor.visit_nested(&mut self.blending_info)?;
            let mut replace_all = self.blending_info.mode == BlendMode::REPLACE;
            self.extra_channel_blending_info
                .resize_with(num_extra, BlendingInfo::default);
            for info in &mut self.extra_channel_blending_info {
                info.nonserialized_is_partial_frame = partial;
                info.nonserialized_num_extra_channels = num_extra;
                visitor.visit_nested(info)?;
                replace_all &= info.mode == BlendMode::REPLACE;
            }
            if visitor.is_reading()
                && self.nonserialized_is_preview
                && (!replace_all || self.custom_size_or_origin)
            {
                return Err(StatusCode::GenericError);
            }
            if visitor.conditional(
                self.nonserialized_metadata
                    .is_some_and(|m| m.m.have_animation),
            ) {
                self.animation_frame.nonserialized_metadata = self.nonserialized_metadata;
                visitor.visit_nested(&mut self.animation_frame)?;
            }
            visitor.bool(true, &mut self.is_last)?;
        } else {
            self.is_last = false;
        }
        if visitor.conditional(self.frame_type != FrameType::DC && !self.is_last) {
            visitor.u32(
                U32Enc::new(val(0), val(1), val(2), val(3)),
                0,
                &mut self.save_as_reference,
            )?;
        }
        if self.frame_type != FrameType::DC {
            if visitor.conditional(
                self.can_be_referenced()
                    && self.blending_info.mode == BlendMode::REPLACE
                    && !partial
                    && (self.frame_type == FrameType::REGULAR
                        || self.frame_type == FrameType::SKIP_PROGRESSIVE),
            ) {
                visitor.bool(false, &mut self.save_before_color_transform)?;
            } else if visitor.conditional(self.frame_type == FrameType::REFERENCE_ONLY) {
                visitor.bool(true, &mut self.save_before_color_transform)?;
                let x = if self.custom_size_or_origin {
                    self.frame_size.xsize as usize
                } else {
                    self.nonserialized_metadata
                        .expect("reference frame metadata")
                        .xsize()
                };
                let y = if self.custom_size_or_origin {
                    self.frame_size.ysize as usize
                } else {
                    self.nonserialized_metadata
                        .expect("reference frame metadata")
                        .ysize()
                };
                if !self.save_before_color_transform {
                    let metadata = self
                        .nonserialized_metadata
                        .expect("reference frame metadata");
                    if x < metadata.xsize()
                        || y < metadata.ysize()
                        || self.frame_origin.x0 != 0
                        || self.frame_origin.y0 != 0
                    {
                        return Err(StatusCode::GenericError);
                    }
                }
            }
        } else {
            self.save_before_color_transform = true;
        }
        visit_name_string(visitor, &mut self.name)?;
        self.loop_filter.nonserialized_is_modular = modular;
        visitor.visit_nested(&mut self.loop_filter)?;
        visitor.begin_extensions(&mut self.extensions)?;
        visitor.end_extensions()
    }
}
pub const ORDER_ENC: U32Enc = U32Enc::new(val(0x5f), val(0x13), val(0), bits(13));
pub fn read_frame_header(reader: &mut BitReader<'_>, frame: &mut FrameHeader<'_>) -> Status {
    bundle::read(reader, frame)
}
