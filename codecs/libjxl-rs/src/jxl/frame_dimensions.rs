// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/frame_dimensions.h`. Keep source rounding, padding,
//! group/block units, and unsigned arithmetic explicit.

use super::base::rect::Rect;
pub const BLOCK_DIM: usize = 8;
pub const DCT_BLOCK_SIZE: usize = BLOCK_DIM * BLOCK_DIM;
pub const GROUP_DIM: usize = 256;
pub const GROUP_DIM_IN_BLOCKS: usize = GROUP_DIM / BLOCK_DIM;
fn div_ceil(value: usize, divisor: usize) -> usize {
    value.wrapping_add(divisor - 1) / divisor
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameDimensions {
    pub xsize: usize,
    pub ysize: usize,
    pub xsize_upsampled: usize,
    pub ysize_upsampled: usize,
    pub xsize_upsampled_padded: usize,
    pub ysize_upsampled_padded: usize,
    pub xsize_padded: usize,
    pub ysize_padded: usize,
    pub xsize_blocks: usize,
    pub ysize_blocks: usize,
    pub xsize_groups: usize,
    pub ysize_groups: usize,
    pub xsize_dc_groups: usize,
    pub ysize_dc_groups: usize,
    pub num_groups: usize,
    pub num_dc_groups: usize,
    pub group_dim: usize,
    pub dc_group_dim: usize,
}
impl FrameDimensions {
    #[expect(
        clippy::too_many_arguments,
        reason = "Preserve the source frame-dimension parameter order and units."
    )]
    pub fn set(
        &mut self,
        x: usize,
        y: usize,
        group_shift: usize,
        hshift: usize,
        vshift: usize,
        modular: bool,
        upsampling: usize,
    ) {
        self.group_dim = (GROUP_DIM >> 1) << group_shift;
        self.dc_group_dim = self.group_dim.wrapping_mul(BLOCK_DIM);
        self.xsize_upsampled = x;
        self.ysize_upsampled = y;
        self.xsize = div_ceil(x, upsampling);
        self.ysize = div_ceil(y, upsampling);
        self.xsize_blocks = div_ceil(self.xsize, BLOCK_DIM << hshift) << hshift;
        self.ysize_blocks = div_ceil(self.ysize, BLOCK_DIM << vshift) << vshift;
        self.xsize_padded = self.xsize_blocks.wrapping_mul(BLOCK_DIM);
        self.ysize_padded = self.ysize_blocks.wrapping_mul(BLOCK_DIM);
        if modular {
            self.xsize_padded = self.xsize;
            self.ysize_padded = self.ysize;
        }
        self.xsize_upsampled_padded = self.xsize_padded.wrapping_mul(upsampling);
        self.ysize_upsampled_padded = self.ysize_padded.wrapping_mul(upsampling);
        self.xsize_groups = div_ceil(self.xsize, self.group_dim);
        self.ysize_groups = div_ceil(self.ysize, self.group_dim);
        self.xsize_dc_groups = div_ceil(self.xsize_blocks, self.group_dim);
        self.ysize_dc_groups = div_ceil(self.ysize_blocks, self.group_dim);
        self.num_groups = self.xsize_groups.wrapping_mul(self.ysize_groups);
        self.num_dc_groups = self.xsize_dc_groups.wrapping_mul(self.ysize_dc_groups);
    }
    pub fn group_rect(&self, index: usize) -> Rect {
        Rect::new_clamped(
            (index % self.xsize_groups).wrapping_mul(self.group_dim),
            (index / self.xsize_groups).wrapping_mul(self.group_dim),
            self.group_dim,
            self.group_dim,
            self.xsize,
            self.ysize,
        )
    }
    pub fn block_group_rect(&self, index: usize) -> Rect {
        Rect::new_clamped(
            (index % self.xsize_groups).wrapping_mul(self.group_dim >> 3),
            (index / self.xsize_groups).wrapping_mul(self.group_dim >> 3),
            self.group_dim >> 3,
            self.group_dim >> 3,
            self.xsize_blocks,
            self.ysize_blocks,
        )
    }
    pub fn dc_group_rect(&self, index: usize) -> Rect {
        Rect::new_clamped(
            (index % self.xsize_dc_groups).wrapping_mul(self.group_dim),
            (index / self.xsize_dc_groups).wrapping_mul(self.group_dim),
            self.group_dim,
            self.group_dim,
            self.xsize_blocks,
            self.ysize_blocks,
        )
    }
}
