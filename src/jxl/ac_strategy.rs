// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AcStrategyType and AcStrategy size/count constants from lib/jxl/ac_strategy.h.
//! AcStrategyImage construction, fills, validity, and counting are also ported
//! from ac_strategy.{h,cc}. Strategy objects, geometry/transform methods, setters,
//! and interpreted AcStrategyRow views remain unported.
use super::base::rect::Rect;
use super::base::status::StatusOr;
use super::frame_dimensions::BLOCK_DIM;
use super::image::Plane;
use super::image_ops::{fill_image, fill_plane};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct AcStrategyType(pub u32);
impl AcStrategyType {
    pub const DCT: Self = Self(0);
    pub const IDENTITY: Self = Self(1);
    pub const DCT2X2: Self = Self(2);
    pub const DCT4X4: Self = Self(3);
    pub const DCT16X16: Self = Self(4);
    pub const DCT32X32: Self = Self(5);
    pub const DCT16X8: Self = Self(6);
    pub const DCT8X16: Self = Self(7);
    pub const DCT32X8: Self = Self(8);
    pub const DCT8X32: Self = Self(9);
    pub const DCT32X16: Self = Self(10);
    pub const DCT16X32: Self = Self(11);
    pub const DCT4X8: Self = Self(12);
    pub const DCT8X4: Self = Self(13);
    pub const AFV0: Self = Self(14);
    pub const AFV1: Self = Self(15);
    pub const AFV2: Self = Self(16);
    pub const AFV3: Self = Self(17);
    pub const DCT64X64: Self = Self(18);
    pub const DCT64X32: Self = Self(19);
    pub const DCT32X64: Self = Self(20);
    pub const DCT128X128: Self = Self(21);
    pub const DCT128X64: Self = Self(22);
    pub const DCT64X128: Self = Self(23);
    pub const DCT256X256: Self = Self(24);
    pub const DCT256X128: Self = Self(25);
    pub const DCT128X256: Self = Self(26);
}
pub const MAX_COEFF_BLOCKS: usize = 32;
pub const MAX_BLOCK_DIM: usize = BLOCK_DIM * MAX_COEFF_BLOCKS;
pub const MAX_COEFF_AREA: usize = MAX_BLOCK_DIM * MAX_BLOCK_DIM;
pub const NUM_VALID_STRATEGIES: u8 = AcStrategyType::DCT128X256.0 as u8 + 1;

#[derive(Default)]
pub struct AcStrategyImage {
    layers: Plane<u8>,
    row_offset: Option<usize>,
    stride: Option<usize>,
}
impl AcStrategyImage {
    pub const INVALID: u8 = 0xff;
    // Retain the source default construction before allocating its layer owner.
    #[allow(clippy::field_reassign_with_default)]
    pub fn create(xsize: usize, ysize: usize) -> StatusOr<Self> {
        let mut image = Self::default();
        image.layers = Plane::create(xsize, ysize)?;
        // The source caches Row(0). Offset zero remains tied to this owner
        // across native moves; no long-lived raw pointer or aliased view exists.
        let _ = image.layers.row(0);
        image.row_offset = Some(0);
        image.stride = Some(image.layers.pixels_per_row() as usize);
        Ok(image)
    }
    pub fn fill_dct8(&mut self, rect: Rect) {
        fill_plane(
            ((AcStrategyType::DCT.0 as u8) << 1) | 1,
            &mut self.layers,
            rect,
        );
    }
    pub fn fill_dct8_full(&mut self) {
        self.fill_dct8(Rect::new(0, 0, self.layers.xsize(), self.layers.ysize()));
    }
    pub fn fill_invalid(&mut self) {
        fill_image(Self::INVALID, &mut self.layers);
    }
    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        let index = self
            .row_offset
            .expect("source row cache must be initialized")
            .wrapping_add(y.wrapping_mul(self.stride.expect("source stride must be initialized")))
            .wrapping_add(x);
        self.layers
            .pixels()
            .expect("source layer allocation must be present")[index]
            != Self::INVALID
    }
    pub fn pixels_per_row(&self) -> usize {
        self.layers.pixels_per_row() as usize
    }
    pub fn xsize(&self) -> usize {
        self.layers.xsize()
    }
    pub fn ysize(&self) -> usize {
        self.layers.ysize()
    }
    pub fn layers(&self) -> &Plane<u8> {
        &self.layers
    }
    pub fn count_blocks(&self, kind: AcStrategyType) -> usize {
        let mut ret = 0usize;
        for y in 0..self.layers.ysize() {
            let row = self.layers.row(y);
            for &value in row.iter().take(self.layers.xsize()) {
                if u32::from(value) == ((u32::from(kind.0 as u8) << 1) | 1) {
                    ret = ret.wrapping_add(1);
                }
            }
        }
        ret
    }
}
