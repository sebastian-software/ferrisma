// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! PassDefinition, ProgressiveMode, and splitter configuration/InitPasses from
//! lib/jxl/enc_progressive_split.h, and SplitACCoefficients<int32_t> from
//! enc_progressive_split.cc. Preserve unsigned narrowing and untouched reused
//! Passes array entries.

use super::ac_strategy::AcStrategy;
use super::base::status::{Status, StatusCode};
use super::coeff_order_fwd::coefficient_layout;
use super::common::MAX_NUM_PASSES;
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE};
use super::frame_header::Passes;

pub const NO_DOWNSAMPLING_FACTOR: usize = usize::MAX;
#[derive(Clone, Copy, Debug, Default)]
pub struct PassDefinition {
    pub num_coefficients: usize,
    pub shift: usize,
    pub suitable_for_downsampling_of_at_least: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct ProgressiveMode {
    pub num_passes: usize,
    pub passes: [PassDefinition; MAX_NUM_PASSES],
}
impl Default for ProgressiveMode {
    fn default() -> Self {
        let mut passes = [PassDefinition::default(); MAX_NUM_PASSES];
        passes[0] = PassDefinition {
            num_coefficients: 8,
            shift: 0,
            suitable_for_downsampling_of_at_least: 1,
        };
        Self {
            num_passes: 1,
            passes,
        }
    }
}
impl ProgressiveMode {
    pub fn new<const N: usize>(definitions: &[PassDefinition; N]) -> Self {
        const {
            assert!(N <= MAX_NUM_PASSES);
        }
        let mut mode = Self {
            num_passes: N,
            ..Self::default()
        };
        // The source's previous-pass bookkeeping is used only by disabled
        // release debug assertions; each active entry is assigned in order.
        for (i, &pass) in definitions.iter().enumerate() {
            mode.passes[i] = pass;
        }
        mode
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ProgressiveSplitter {
    mode: ProgressiveMode,
}
impl ProgressiveSplitter {
    pub fn set_progressive_mode(&mut self, mode: ProgressiveMode) {
        self.mode = mode;
    }
    pub fn get_num_passes(&self) -> usize {
        self.mode.num_passes
    }
    /// SplitACCoefficients<int32_t> from enc_progressive_split.cc. `output`
    /// holds one destination per pass, each at least the block's size.
    #[allow(
        clippy::needless_range_loop,
        reason = "Keep the source pass index loop."
    )]
    pub fn split_ac_coefficients(
        &self,
        block: &[i32],
        acs: AcStrategy,
        _bx: usize,
        _by: usize,
        output: &mut [&mut [i32]],
    ) {
        let size = acs.covered_blocks_x() * acs.covered_blocks_y() * DCT_BLOCK_SIZE;
        let shift_right_round0 = |v: i32, shift: i32| -> i32 {
            let one_if_negative = ((v as u32) >> 31) as i32;
            let add = (one_if_negative << shift).wrapping_sub(one_if_negative);
            v.wrapping_add(add) >> shift
        };
        // Early quit for the simple case of only one pass.
        if self.mode.num_passes == 1 {
            output[0][..size].copy_from_slice(&block[..size]);
            return;
        }
        let mut ncoeffs_all_done_from_earlier_passes = 1usize;

        let mut previous_pass_shift = 0i32;
        for num_pass in 0..self.mode.num_passes {
            // Zero out output block.
            output[num_pass][..size].fill(0);
            let pass_shift = self.mode.passes[num_pass].shift as i32;
            let frame_ncoeffs = self.mode.passes[num_pass].num_coefficients;
            let mut xsize = acs.covered_blocks_x();
            let mut ysize = acs.covered_blocks_y();
            coefficient_layout(&mut ysize, &mut xsize);
            for y in 0..ysize * frame_ncoeffs {
                for x in 0..xsize * frame_ncoeffs {
                    let pos = y * xsize * BLOCK_DIM + x;
                    if x < xsize * ncoeffs_all_done_from_earlier_passes
                        && y < ysize * ncoeffs_all_done_from_earlier_passes
                    {
                        // This coefficient was already included in an earlier pass,
                        // which included a genuinely smaller set of coefficients.
                        continue;
                    }
                    let mut v = block[pos];
                    // Previous pass discarded some bits: do not encode them again.
                    if previous_pass_shift != 0 {
                        let previous_v = shift_right_round0(v, previous_pass_shift)
                            .wrapping_mul(1 << previous_pass_shift);
                        v = v.wrapping_sub(previous_v);
                    }
                    output[num_pass][pos] = shift_right_round0(v, pass_shift);
                }
            }
            // We just finished a pass.
            // Hence, we are now guaranteed to have included all coeffs up to
            // frame_ncoeffs in every block, unless the current pass is shifted.
            if self.mode.passes[num_pass].shift == 0 {
                ncoeffs_all_done_from_earlier_passes = frame_ncoeffs;
            }
            previous_pass_shift = self.mode.passes[num_pass].shift as i32;
        }
    }
    pub fn init_passes(&self, passes: &mut Passes) -> Status {
        passes.num_passes = self.get_num_passes() as u32;
        passes.num_downsample = 0;
        if passes.num_passes == 0 {
            return Err(StatusCode::GenericError);
        }
        passes.shift[passes.num_passes as usize - 1] = 0;
        if passes.num_passes == 1 {
            return Ok(());
        }
        for i in 0..self.mode.num_passes - 1 {
            let min_factor = self.mode.passes[i].suitable_for_downsampling_of_at_least;
            passes.shift[i] = self.mode.passes[i].shift as u32;
            if 1 < min_factor && min_factor != NO_DOWNSAMPLING_FACTOR {
                passes.downsample[passes.num_downsample as usize] = min_factor as u32;
                passes.last_pass[passes.num_downsample as usize] = i as u32;
                if self.mode.passes[i + 1].suitable_for_downsampling_of_at_least < min_factor {
                    passes.num_downsample += 1;
                }
            }
        }
        Ok(())
    }
}
