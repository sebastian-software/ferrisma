// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AcStrategyType and AcStrategy size/count constants from lib/jxl/ac_strategy.h.
//! AcStrategyImage construction, fills, validity, and counting are also ported
//! from ac_strategy.{h,cc}, with strategy objects, geometry and natural orders.
//! Setters and interpreted AcStrategyRow views preserve the packed layer owner.
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::frame_dimensions::BLOCK_DIM;
use super::image::Plane;
use super::image_ops::{fill_image, fill_plane};
use super::memory_manager_internal::MemoryManagerPtr;

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
        Self::create_in(MemoryManagerPtr::NATIVE, xsize, ysize)
    }
    /// The source AcStrategyImage::Create(memory_manager, xsize, ysize).
    #[allow(clippy::field_reassign_with_default)]
    pub fn create_in(
        memory_manager: MemoryManagerPtr,
        xsize: usize,
        ysize: usize,
    ) -> StatusOr<Self> {
        let mut image = Self::default();
        image.layers = Plane::create_in(xsize, ysize, 0, memory_manager)?;
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
    pub fn set(&mut self, x: usize, y: usize, kind: AcStrategyType) -> Status {
        let acs = AcStrategy::from_raw_strategy(kind);
        debug_assert!(y + acs.covered_blocks_y() <= self.layers.ysize());
        debug_assert!(x + acs.covered_blocks_x() <= self.layers.xsize());
        self.set_no_bounds_check(x, y, kind, false)
    }
    pub fn set_no_bounds_check(
        &mut self,
        x: usize,
        y: usize,
        kind: AcStrategyType,
        check: bool,
    ) -> Status {
        let acs = AcStrategy::from_raw_strategy(kind);
        for iy in 0..acs.covered_blocks_y() {
            for ix in 0..acs.covered_blocks_x() {
                let row = self.layers.row_mut(y + iy);
                if check && row[x + ix] != Self::INVALID {
                    return Err(StatusCode::GenericError);
                }
                row[x + ix] = ((kind.0 as u8) << 1) | u8::from((iy | ix) == 0);
            }
        }
        Ok(())
    }
    pub fn const_row(&self, y: usize, x_prefix: usize) -> AcStrategyRow<'_> {
        AcStrategyRow {
            row: &self.layers.row(y)[x_prefix..],
        }
    }
    pub fn const_rect_row(&self, rect: Rect, y: usize) -> AcStrategyRow<'_> {
        self.const_row(rect.y0() + y, rect.x0())
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

/// Original interpreted row; descriptors keep each packed first-block flag.
pub struct AcStrategyRow<'a> {
    row: &'a [u8],
}
impl AcStrategyRow<'_> {
    pub fn get(&self, x: usize) -> AcStrategy {
        let strategy = AcStrategyType(u32::from(self.row[x] >> 1));
        let is_first = self.row[x] & 1 != 0;
        debug_assert!(AcStrategy::from_raw_strategy(strategy).is_multiblock() || is_first);
        AcStrategy { strategy, is_first }
    }
}

/// Original strategy descriptor. Construction admits only defined strategies;
/// non-first-block descriptors are supplied by interpreted rows in the source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcStrategy {
    strategy: AcStrategyType,
    is_first: bool,
}
impl AcStrategy {
    pub const fn type_bit(kind: AcStrategyType) -> u32 {
        1u32 << kind.0
    }
    pub const fn is_raw_strategy_valid(raw: i32) -> bool {
        raw >= 0 && raw < NUM_VALID_STRATEGIES as i32
    }
    pub fn from_raw_strategy(kind: AcStrategyType) -> Self {
        assert!(Self::is_raw_strategy_valid(kind.0 as i32));
        Self {
            strategy: kind,
            is_first: true,
        }
    }
    pub fn is_first_block(&self) -> bool {
        self.is_first
    }
    pub fn raw_strategy(&self) -> u8 {
        self.strategy.0 as u8
    }
    pub fn strategy(&self) -> AcStrategyType {
        self.strategy
    }
    pub fn is_multiblock(&self) -> bool {
        const BITS: u32 = (1 << 4)
            | (1 << 5)
            | (1 << 6)
            | (1 << 7)
            | (1 << 8)
            | (1 << 9)
            | (1 << 10)
            | (1 << 11)
            | (1 << 18)
            | (1 << 19)
            | (1 << 20)
            | (1 << 21)
            | (1 << 22)
            | (1 << 23)
            | (1 << 24)
            | (1 << 25)
            | (1 << 26);
        (Self::type_bit(self.strategy) & BITS) != 0
    }
    pub fn covered_blocks_x(&self) -> usize {
        const LUT: [u8; 27] = [
            1, 1, 1, 1, 2, 4, 1, 2, 1, 4, 2, 4, 1, 1, 1, 1, 1, 1, 8, 4, 8, 16, 8, 16, 32, 16, 32,
        ];
        usize::from(LUT[self.strategy.0 as usize])
    }
    pub fn covered_blocks_y(&self) -> usize {
        const LUT: [u8; 27] = [
            1, 1, 1, 1, 2, 4, 2, 1, 4, 1, 4, 2, 1, 1, 1, 1, 1, 1, 8, 8, 4, 16, 16, 8, 32, 32, 16,
        ];
        usize::from(LUT[self.strategy.0 as usize])
    }
    pub fn log2_covered_blocks(&self) -> usize {
        const LUT: [u8; 27] = [
            0, 0, 0, 0, 2, 4, 1, 1, 2, 2, 3, 3, 0, 0, 0, 0, 0, 0, 6, 5, 5, 8, 7, 7, 10, 9, 9,
        ];
        usize::from(LUT[self.strategy.0 as usize])
    }
    pub fn compute_natural_coeff_order(&self, order: &mut [u32]) {
        self.coeff_order_and_lut(false, order);
    }
    pub fn compute_natural_coeff_order_lut(&self, lut: &mut [u32]) {
        self.coeff_order_and_lut(true, lut);
    }
    fn coeff_order_and_lut(&self, is_lut: bool, out: &mut [u32]) {
        let mut cx = self.covered_blocks_x();
        let mut cy = self.covered_blocks_y();
        super::coeff_order_fwd::coefficient_layout(&mut cy, &mut cx);
        let xs = cx / cy;
        let xsm = xs - 1;
        let xss = xs.ilog2();
        let mut cur = cx * cy;
        for i in 0..cx * BLOCK_DIM {
            for j in 0..=i {
                let (mut x, mut y) = (j, i - j);
                if i % 2 != 0 {
                    std::mem::swap(&mut x, &mut y);
                }
                if (y & xsm) != 0 {
                    continue;
                }
                y >>= xss;
                let val = if x < cx && y < cy {
                    y * cx + x
                } else {
                    let v = cur;
                    cur += 1;
                    v
                };
                if is_lut {
                    out[y * cx * BLOCK_DIM + x] = val as u32;
                } else {
                    out[val] = (y * cx * BLOCK_DIM + x) as u32;
                }
            }
        }
        for ip in (1..cx * BLOCK_DIM).rev() {
            let i = ip - 1;
            for j in 0..=i {
                let (mut x, mut y) = (cx * BLOCK_DIM - 1 - (i - j), cx * BLOCK_DIM - 1 - j);
                if i % 2 != 0 {
                    std::mem::swap(&mut x, &mut y);
                }
                if (y & xsm) != 0 {
                    continue;
                }
                y >>= xss;
                let val = cur;
                cur += 1;
                if is_lut {
                    out[y * cx * BLOCK_DIM + x] = val as u32;
                } else {
                    out[val] = (y * cx * BLOCK_DIM + x) as u32;
                }
            }
        }
    }
}
