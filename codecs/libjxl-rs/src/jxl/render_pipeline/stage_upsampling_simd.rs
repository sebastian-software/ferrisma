// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD UpsamplingStage of render_pipeline/stage_upsampling.cc (2x, 4x
//! and 8x, with simd_util-inl.h's `StoreInterleaved`) for the NEON and AVX2
//! targets (ADR 0004), written once against [`SimdF32`]. Compare it only
//! with the original's SIMD build at the same target.
//!
//! As the original, the stage reads and writes whole vectors: the column
//! minima and maxima to `RoundUpTo(len + 4, Lanes(d))`, the output to
//! `RoundUpTo(len, Lanes(d)) * N` samples per output row. On AVX2, when
//! `len % 8` is 1 to 4, the minima/maxima lanes from `RoundUpTo(len, 8) - 4`
//! on read column extrema the chunk never wrote (the original's per-thread
//! scratch from an earlier row, or never initialized); those lanes only
//! reach output samples past `xsize * N`, which libjxl treats as row padding.
//! The port keeps the scratch per thread like the original.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings, Span,
};
use super::simd_rows::with_spans;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use crate::jxl::image_metadata::CustomTransformData;
use crate::jxl::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};

const CHUNK_SIZE: usize = 1024;

fn round_up_to(v: usize, m: usize) -> usize {
    v.div_ceil(m) * m
}

/// `PreProcessRowImpl`: `rows` are the five input rows from `x0 - 2`.
struct PreProcess<'a> {
    rows: [&'a [f32]; 5],
    len: usize,
    /// `temp_[0]` (col_min, then maxs), `temp_[1]` (col_max), `temp_[2]` (mins).
    temp: [&'a mut [f32]; 3],
}
impl Kernel for PreProcess<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) {
        let rows = self.rows;
        let len = self.len;
        let [t0, t1, t2] = self.temp;
        let col_min = t0;
        let col_max = t1;
        let mut x = 0;
        while x < len + 4 {
            let v0 = df.load(&rows[0][x..]);
            let v1 = df.load(&rows[1][x..]);
            let min0 = df.min(v0, v1);
            let max0 = df.max(v0, v1);
            let v2 = df.load(&rows[2][x..]);
            let v3 = df.load(&rows[3][x..]);
            let min1 = df.min(v2, v3);
            let max1 = df.max(v2, v3);
            let v4 = df.load(&rows[4][x..]);
            let min2 = df.min(min0, min1);
            let max2 = df.max(max0, max1);
            let min = df.min(v4, min2);
            let max = df.max(v4, max2);
            df.store(min, &mut col_min[x..]);
            df.store(max, &mut col_max[x..]);
            x += D::LANES;
        }

        let mins = t2;
        let mut x = 0;
        while x < len {
            let v0 = df.load(&col_min[x..]);
            let v1 = df.load(&col_min[x + 1..]);
            let min0 = df.min(v0, v1);
            let v2 = df.load(&col_min[x + 2..]);
            let v3 = df.load(&col_min[x + 3..]);
            let min1 = df.min(v2, v3);
            let v4 = df.load(&col_min[x + 4..]);
            let min2 = df.min(min0, min1);
            let min = df.min(v4, min2);
            df.store(min, &mut mins[x..]);
            x += D::LANES;
        }

        // col_mins will be overwritten
        let maxs = col_min;
        let mut x = 0;
        while x < len {
            let v0 = df.load(&col_max[x..]);
            let v1 = df.load(&col_max[x + 1..]);
            let max0 = df.max(v0, v1);
            let v2 = df.load(&col_max[x + 2..]);
            let v3 = df.load(&col_max[x + 3..]);
            let max1 = df.max(v2, v3);
            let v4 = df.load(&col_max[x + 4..]);
            let max2 = df.max(max0, max1);
            let max = df.max(v4, max2);
            df.store(max, &mut maxs[x..]);
            x += D::LANES;
        }
    }
}

/// `ProcessRowImpl<N>`: `rows` are the five input rows from `x0 - 2`, so
/// `input[5 * (iy + 2) + (ix + 2)] + x` is `rows[iy + 2][ix + 2 + x]`; `out`
/// receives the `N` output rows from `x0 * N`.
struct Process<'a, const N: usize> {
    rows: [&'a [f32]; 5],
    len: usize,
    kernel: &'a [f32; 64 * 25],
    mins: &'a [f32],
    maxs: &'a [f32],
    out: [&'a mut [f32]; N],
}
impl<const N: usize> Kernel for Process<'_, N> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) {
        let input = |i: usize, x: usize| &self.rows[i / 5][i % 5 + x..];
        let kernel = |k: usize, i: usize| self.kernel[k * 25 + i];
        let mut out = self.out;
        let mut x = 0;
        while x < self.len {
            for (oy, out_row) in out.iter_mut().enumerate() {
                let mut ups = [df.zero(); N];
                for (ox, up) in ups.iter_mut().enumerate() {
                    let k = N * oy + ox;
                    let mut acc0 = df.mul(df.load(input(0, x)), df.set(kernel(k, 0)));
                    let mut acc1 = df.mul(df.load(input(1, x)), df.set(kernel(k, 1)));
                    let mut acc2 = df.mul(df.load(input(2, x)), df.set(kernel(k, 2)));
                    let mut i = 3;
                    while i < 24 {
                        acc0 = df.mul_add(df.load(input(i, x)), df.set(kernel(k, i)), acc0);
                        acc1 = df.mul_add(df.load(input(i + 1, x)), df.set(kernel(k, i + 1)), acc1);
                        acc2 = df.mul_add(df.load(input(i + 2, x)), df.set(kernel(k, i + 2)), acc2);
                        i += 3;
                    }
                    acc0 = df.mul_add(df.load(input(24, x)), df.set(kernel(k, 24)), acc0);
                    let result = df.add(df.add(acc1, acc2), acc0);
                    *up = df.clamp(result, df.load(&self.mins[x..]), df.load(&self.maxs[x..]));
                }
                store_interleaved(df, &ups, &mut out_row[x * N..]);
            }
            x += D::LANES;
        }
    }
}

/// simd_util-inl.h `StoreInterleaved(df, v0, ..., v{N-1}, mem)` for N = 2, 4
/// and 8 on NEON and AVX2: `v0[0], v1[0], ..., v{N-1}[0], v0[1], ...`. The
/// original's interleave/concat sequences only permute lanes into that
/// order; they perform no arithmetic.
#[inline(always)]
fn store_interleaved<D: SimdF32, const N: usize>(df: D, v: &[D::V; N], mem: &mut [f32]) {
    let mem = &mut mem[..N * D::LANES];
    let mut lanes = [0.0f32; 8];
    for (j, v) in v.iter().enumerate() {
        df.store(*v, &mut lanes[..]);
        for (i, &lane) in lanes[..D::LANES].iter().enumerate() {
            mem[i * N + j] = lane;
        }
    }
}

pub struct UpsamplingStageSimd {
    target: Target,
    settings: Settings,
    temp: [Vec<Option<AlignedMemory>>; 3],
    c: usize,
    kernel: [f32; 64 * 25],
    memory_manager: MemoryManagerPtr,
}
impl Drop for UpsamplingStageSimd {
    /// The implicit destructor: the temp_ array last first, each vector
    /// last first.
    fn drop(&mut self) {
        for temp in self.temp.iter_mut().rev() {
            crate::jxl::base::libcpp_vector::clear(temp);
        }
    }
}
impl UpsamplingStageSimd {
    fn new(
        target: Target,
        memory_manager: MemoryManagerPtr,
        factors: &CustomTransformData,
        c: usize,
        shift: usize,
    ) -> Self {
        let mut stage = Self {
            target,
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

    fn process_chunk<const N: usize>(
        &mut self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        x0: usize,
        len: usize,
        thread: usize,
    ) {
        let lanes = self.target.lanes();
        let c = self.c;
        // PreProcessRowImpl reads RoundUpTo(len + 4, Lanes) samples from
        // x0 - 2; ProcessRowImpl reads RoundUpTo(len, Lanes) + 4 from there.
        let read_len = round_up_to(len + 4, lanes).max(round_up_to(len, lanes) + 4);
        // stage_upsampling.cc: the input rows and the N output rows are the
        // pipeline's rows, loaded and stored in place.
        let reads: [Span; 5] = std::array::from_fn(|i| Span {
            row: self.get_input_row(input, c, i as i32 - 2),
            from: x0 as isize - 2,
            len: read_len,
        });
        let out_len = round_up_to(len, lanes) * N;
        let writes: [Span; N] = std::array::from_fn(|oy| Span {
            row: self.get_output_row(output, c, oy),
            from: (x0 * N) as isize,
            len: out_len,
        });
        let target = self.target;
        let kernel = &self.kernel;
        let [t0, t1, t2] = &mut self.temp;
        with_spans(buffers, reads, writes, |rows, out| {
            let temp = [
                t0[thread].as_mut().unwrap().f32s_mut(),
                t1[thread].as_mut().unwrap().f32s_mut(),
                t2[thread].as_mut().unwrap().f32s_mut(),
            ];
            dispatch(target, PreProcess { rows, len, temp });
            let maxs = t0[thread].as_mut().unwrap().f32s_mut();
            let mins = t2[thread].as_mut().unwrap().f32s_mut();
            dispatch(
                target,
                Process::<N> {
                    rows,
                    len,
                    kernel,
                    mins,
                    maxs,
                    out,
                },
            );
        });
    }
}

impl RenderPipelineStage for UpsamplingStageSimd {
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
        // sizeof(float) * (kChunkSize + std::max(size_t{4}, Lanes(df)))
        let alloc_size = 4 * (CHUNK_SIZE + self.target.lanes().max(4));
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
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        for x in (0..width).step_by(CHUNK_SIZE) {
            let end = (x + CHUNK_SIZE).min(width);
            let len = end - x;
            if n == 2 {
                self.process_chunk::<2>(input, output, buffers, x, len, thread);
            }
            if n == 4 {
                self.process_chunk::<4>(input, output, buffers, x, len, thread);
            }
            if n == 8 {
                self.process_chunk::<8>(input, output, buffers, x, len, thread);
            }
        }
        Ok(())
    }
}

/// `GetUpsamplingStage(memory_manager, ups_factors, c, shift)` instantiated
/// for one SIMD target; the caller checks `shift` as the dispatching
/// wrapper does.
pub fn get_upsampling_stage_simd(
    memory_manager: MemoryManagerPtr,
    factors: &CustomTransformData,
    c: usize,
    shift: usize,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    Box::new(UpsamplingStageSimd::new(
        target,
        memory_manager,
        factors,
        c,
        shift,
    ))
}
