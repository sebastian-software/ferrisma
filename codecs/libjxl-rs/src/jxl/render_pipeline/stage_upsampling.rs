// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Scalar Min/Max/MulAdd portions derived from Highway hwy/ops/scalar-inl.h:
// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Those portions use Highway's offered BSD alternative in
// licenses/HIGHWAY-BSD-3-Clause.txt. Modified: translated scalar operations.
//! Complete one-lane UpsamplingStage and factory from stage_upsampling.cc.
//! Row descriptors replace pointers. Aligned temporary owners retain the source
//! allocation sizes; u32 views transfer float bits without new unsafe casts.
//! Native storage initializes the source's unused kernel/temp entries. MSan
//! poison annotations and SIMD specializations remain outside this scalar port.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::image_metadata::CustomTransformData;
use crate::jxl::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};

const CHUNK_SIZE: usize = 1024;

// Highway's one-lane MulAdd returns Mul followed by Add. The original stage's
// scalar object retains separate FMUL/FADD on AArch64; do not use fused math.
fn scalar_mul_add(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}

// Highway scalar-inl.h: ignore one NaN, return the second operand for a tie.
// These differ from both std::min/max and Rust's floating-point min/max.
fn scalar_min(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a < b { a } else { b }
}
fn scalar_max(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a > b { a } else { b }
}

pub struct UpsamplingStage {
    settings: Settings,
    temp: [Vec<Option<AlignedMemory>>; 3],
    c: usize,
    kernel: [f32; 64 * 25],
    memory_manager: MemoryManagerPtr,
}
impl Drop for UpsamplingStage {
    /// The implicit destructor: the temp_ array last first, each vector
    /// last first.
    fn drop(&mut self) {
        for temp in self.temp.iter_mut().rev() {
            crate::jxl::base::libcpp_vector::clear(temp);
        }
    }
}
impl UpsamplingStage {
    fn new(
        memory_manager: MemoryManagerPtr,
        factors: &CustomTransformData,
        c: usize,
        shift: usize,
    ) -> Self {
        let mut stage = Self {
            memory_manager,
            settings: Settings::symmetric(shift, 2),
            temp: std::array::from_fn(|_| Vec::new()),
            c,
            kernel: [0.0; 64 * 25],
        };
        let weights: &[f32] = match shift {
            1 => &factors.upsampling2_weights,
            2 => &factors.upsampling4_weights,
            _ => &factors.upsampling8_weights,
        };
        let n = 1usize << shift;
        let h = n / 2;
        for ky in 0..h {
            for kx in 0..h {
                let offset0 = (ky * n + kx) * 25;
                let offset1 = (ky * n + (n - 1 - kx)) * 25;
                let offset2 = ((n - 1 - ky) * n + kx) * 25;
                let offset3 = ((n - 1 - ky) * n + (n - 1 - kx)) * 25;
                for py in 0..5 {
                    for px in 0..5 {
                        let j = 5 * ky + py;
                        let i = 5 * kx + px;
                        let my = i.min(j);
                        let mx = i.max(j);
                        let w =
                            weights[5 * h * my - my.wrapping_mul(my.wrapping_sub(1)) / 2 + mx - my];
                        stage.kernel[offset0 + py * 5 + px] = w;
                        stage.kernel[offset1 + py * 5 + (4 - px)] = w;
                        stage.kernel[offset2 + (4 - py) * 5 + px] = w;
                        stage.kernel[offset3 + (4 - py) * 5 + (4 - px)] = w;
                    }
                }
            }
        }
        stage
    }
    #[expect(
        clippy::needless_range_loop,
        reason = "Mirror the source loop over five parallel spans."
    )]
    fn pre_process_row(
        &mut self,
        input: &RowInfo,
        buffers: &RowBuffers<'_>,
        x0: usize,
        len: usize,
        thread: usize,
    ) {
        let rows: [_; 5] = std::array::from_fn(|i| {
            self.get_input_row(input, self.c, i as i32 - 2)
                .offset(x0 as isize - 2)
        });
        // Resolve the five input spans once per call; values, order and
        // arithmetic are unchanged.
        let spans: [&[f32]; 5] = std::array::from_fn(|i| &buffers.row(rows[i])[..len + 4]);
        for x in 0..len + 4 {
            let v0 = spans[0][x];
            let v1 = spans[1][x];
            let min0 = scalar_min(v0, v1);
            let max0 = scalar_max(v0, v1);
            let v2 = spans[2][x];
            let v3 = spans[3][x];
            let min1 = scalar_min(v2, v3);
            let max1 = scalar_max(v2, v3);
            let v4 = spans[4][x];
            let min2 = scalar_min(min0, min1);
            let max2 = scalar_max(max0, max1);
            self.temp[0][thread].as_mut().unwrap().u32s_mut()[x] = scalar_min(v4, min2).to_bits();
            self.temp[1][thread].as_mut().unwrap().u32s_mut()[x] = scalar_max(v4, max2).to_bits();
        }
        for x in 0..len {
            let col = self.temp[0][thread].as_ref().unwrap().u32s();
            let min0 = scalar_min(f32::from_bits(col[x]), f32::from_bits(col[x + 1]));
            let min1 = scalar_min(f32::from_bits(col[x + 2]), f32::from_bits(col[x + 3]));
            let min2 = scalar_min(min0, min1);
            let min = scalar_min(f32::from_bits(col[x + 4]), min2);
            self.temp[2][thread].as_mut().unwrap().u32s_mut()[x] = min.to_bits();
        }
        for x in 0..len {
            let col = self.temp[1][thread].as_ref().unwrap().u32s();
            let max0 = scalar_max(f32::from_bits(col[x]), f32::from_bits(col[x + 1]));
            let max1 = scalar_max(f32::from_bits(col[x + 2]), f32::from_bits(col[x + 3]));
            let max2 = scalar_max(max0, max1);
            let max = scalar_max(f32::from_bits(col[x + 4]), max2);
            self.temp[0][thread].as_mut().unwrap().u32s_mut()[x] = max.to_bits();
        }
    }
    fn process_row_impl<const N: usize>(
        &self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        x0: usize,
        len: usize,
        thread: usize,
    ) {
        let rows: [_; 25] = std::array::from_fn(|i| {
            self.get_input_row(input, self.c, (i / 5) as i32 - 2)
                .offset(x0 as isize + (i % 5) as isize - 2)
        });
        let mins = self.temp[2][thread].as_ref().unwrap().u32s();
        let maxs = self.temp[0][thread].as_ref().unwrap().u32s();
        // Resolve the 25 input spans once per call and stage the N output
        // rows, which are written back after all samples are computed (input
        // and output rows may share a plane). Values, order and arithmetic
        // are unchanged; only addressing is hoisted.
        let spans: [&[f32]; 25] = std::array::from_fn(|i| &buffers.row(rows[i])[..len]);
        let mut staged: [Vec<f32>; N] = std::array::from_fn(|_| Vec::with_capacity(len * N));
        for x in 0..len {
            for (oy, staged) in staged.iter_mut().enumerate() {
                let mut ups = [0.0; N];
                for (ox, up) in ups.iter_mut().enumerate() {
                    let k = N * oy + ox;
                    let kernel = &self.kernel[k * 25..];
                    let mut acc0 = spans[0][x] * kernel[0];
                    let mut acc1 = spans[1][x] * kernel[1];
                    let mut acc2 = spans[2][x] * kernel[2];
                    for i in (3..24).step_by(3) {
                        acc0 = scalar_mul_add(spans[i][x], kernel[i], acc0);
                        acc1 = scalar_mul_add(spans[i + 1][x], kernel[i + 1], acc1);
                        acc2 = scalar_mul_add(spans[i + 2][x], kernel[i + 2], acc2);
                    }
                    acc0 = scalar_mul_add(spans[24][x], kernel[24], acc0);
                    let result = (acc1 + acc2) + acc0;
                    *up = scalar_min(
                        scalar_max(f32::from_bits(mins[x]), result),
                        f32::from_bits(maxs[x]),
                    );
                }
                staged.extend_from_slice(&ups);
            }
        }
        for (oy, staged) in staged.iter().enumerate() {
            let dst = self
                .get_output_row(output, self.c, oy)
                .offset((x0 * N) as isize);
            buffers.row_mut(dst)[..staged.len()].copy_from_slice(staged);
        }
    }
}
impl RenderPipelineStage for UpsamplingStage {
    fn settings(&self) -> Settings {
        self.settings
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c == self.c {
            Mode::InOut
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "Upsample"
    }
    fn prepare_for_threads(&mut self, num_threads: usize) -> Status {
        let alloc_size = 4 * (CHUNK_SIZE + 4);
        for i in 0..3 {
            self.temp[i].resize_with(num_threads, || None);
            for t in 0..num_threads {
                self.temp[i][t] = Some(AlignedMemory::create(self.memory_manager, alloc_size, 0)?);
            }
        }
        Ok(())
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        thread: usize,
    ) -> Status {
        let n = 1 << self.settings.shift_x;
        // Scalar Lanes(df)==1, so source MSan padding annotations have zero size.
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        for x in (0..width).step_by(CHUNK_SIZE) {
            let end = (x + CHUNK_SIZE).min(width);
            let len = end - x;
            self.pre_process_row(input, buffers, x, len, thread);
            if n == 2 {
                self.process_row_impl::<2>(input, output, buffers, x, len, thread);
            }
            if n == 4 {
                self.process_row_impl::<4>(input, output, buffers, x, len, thread);
            }
            if n == 8 {
                self.process_row_impl::<8>(input, output, buffers, x, len, thread);
            }
        }
        Ok(())
    }
}
pub fn get_upsampling_stage(
    factors: &CustomTransformData,
    c: usize,
    shift: usize,
) -> Option<Box<dyn RenderPipelineStage>> {
    get_upsampling_stage_in(MemoryManagerPtr::NATIVE, factors, c, shift)
}
/// The source GetUpsamplingStage(memory_manager, ups_factors, c, shift).
pub fn get_upsampling_stage_in(
    memory_manager: MemoryManagerPtr,
    factors: &CustomTransformData,
    c: usize,
    shift: usize,
) -> Option<Box<dyn RenderPipelineStage>> {
    if !(1..=3).contains(&shift) {
        return None;
    }
    Some(Box::new(UpsamplingStage::new(
        memory_manager,
        factors,
        c,
        shift,
    )))
}
