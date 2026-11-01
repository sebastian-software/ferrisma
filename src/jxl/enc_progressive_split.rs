// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! PassDefinition, ProgressiveMode, and splitter configuration/InitPasses from
//! lib/jxl/enc_progressive_split.h. Coefficient splitting remains unported.
//! Preserve unsigned narrowing and untouched reused Passes array entries.

use super::base::status::{Status, StatusCode};
use super::common::MAX_NUM_PASSES;
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
