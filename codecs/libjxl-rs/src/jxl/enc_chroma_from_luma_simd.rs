// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! InitDCStorage of lib/jxl/enc_chroma_from_luma.cc for the NEON and AVX2
//! targets (port ADR 0004, "Encoder path"), written once against
//! [`SimdF32`] with `HWY_FULL(float)` (four NEON, eight AVX2 lanes). Compare
//! it only with the original's SIMD build at the same target; the scalar
//! (one-lane) translation stays in `enc_chroma_from_luma`.
use super::base::status::{Status, StatusCode};
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::ImageF;
use super::memory_manager_internal::MemoryManagerPtr;

/// InitDCStorage: four rows of `RoundUpTo(num_blocks, Lanes(df))` samples,
/// the last `Lanes(df)` of each zeroed.
pub fn init_dc_storage_d<D: SimdF32>(
    _d: D,
    memory_manager: MemoryManagerPtr,
    num_blocks: usize,
    dc_values: &mut ImageF,
) -> Status {
    // First row: Y channel
    // Second row: X channel
    // Third row: Y channel
    // Fourth row: B channel
    *dc_values = ImageF::create_in(
        num_blocks.div_ceil(D::LANES) * D::LANES,
        4,
        0,
        memory_manager,
    )?;

    if dc_values.xsize() == 0 {
        return Err(StatusCode::GenericError);
    }
    // Zero-fill the last lanes
    let xsize = dc_values.xsize();
    for y in 0..4 {
        for x in xsize - D::LANES..xsize {
            dc_values.row_mut(y)[x] = 0.0;
        }
    }
    Ok(())
}

struct InitDc<'a> {
    memory_manager: MemoryManagerPtr,
    num_blocks: usize,
    dc_values: &'a mut ImageF,
}
impl Kernel for InitDc<'_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        init_dc_storage_d(d, self.memory_manager, self.num_blocks, self.dc_values)
    }
}

/// `InitDCStorage` instantiated for one SIMD target.
pub fn init_dc_storage_simd(
    target: Target,
    memory_manager: MemoryManagerPtr,
    num_blocks: usize,
    dc_values: &mut ImageF,
) -> Status {
    dispatch(
        target,
        InitDc {
            memory_manager,
            num_blocks,
            dc_values,
        },
    )
}
