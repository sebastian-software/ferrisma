// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The vector conversions of WriteToOutputStage (render_pipeline/stage_write.cc)
//! for the NEON and AVX2 targets (ADR 0004), written once against
//! [`SimdF32`]: `MakeUnsigned` with `StoreUnsignedRow`, `StoreFloat16Row`
//! and `UnpremulAlpha`. `stage_write.rs` keeps the stage, its buffers and
//! callbacks and calls these when the decoder selected a SIMD target.
//!
//! The source loops run whole vectors to `RoundUpTo(len, Lanes(d))`; the
//! lanes past `len` read row padding or scratch memory and are never copied
//! to an output, so these functions take inputs padded with zeros to whole
//! vectors and return the first `len` results. Every conversion is
//! lane-wise. The interleaving stores (`StoreInterleaved2/3/4`) and the
//! 16-bit byte swap are data movement and stay with the caller's loops, as
//! does `StoreFloatRow`, which only moves words.
use super::stage_write_dither::DITHER;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};

/// Copy `len` samples into a zero-padded buffer of whole vectors.
pub fn padded(target: Target, len: usize, sample: impl Fn(usize) -> f32) -> Vec<f32> {
    let lanes = target.lanes();
    let mut values = vec![0.0; len.div_ceil(lanes) * lanes];
    for (x, v) in values.iter_mut().enumerate().take(len) {
        *v = sample(x);
    }
    values
}

/// `MakeUnsigned<T>(LoadU(d, &input[c][i]), xstart + i, ypos, mul, c)` for
/// every vector of the row, as `StoreUnsignedRow` calls it per channel.
struct UnsignedRow<'a> {
    input: &'a [f32],
    bits: usize,
    byte: bool,
    xstart: usize,
    ypos: usize,
    c: usize,
    out: &'a mut [u16],
}
impl Kernel for UnsignedRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        assert!(self.bits < 32, "source 1u shift precondition");
        let mul = d.set(((1u32 << self.bits) - 1) as f32);
        // DemoteTo(Rebind<T>, BitCast(du32, ni)) saturates the unsigned
        // value on both targets (vqmovn; Min with 0x7fffffff then packus).
        let max = if self.byte {
            u32::from(u8::MAX)
        } else {
            u32::from(u16::MAX)
        };
        let mut ni = vec![0i32; D::LANES];
        for i in (0..self.input.len()).step_by(D::LANES) {
            let mut v = d.load(&self.input[i..]);
            v = d.mul(v, mul);
            if self.byte {
                let x0 = self.xstart.wrapping_add(i);
                let x_off = x0.wrapping_add(self.c.wrapping_mul(23)) % 32;
                let y_off = self.ypos.wrapping_add(self.c.wrapping_mul(13)) % 32;
                let pos = y_off * 48 + x_off;
                let dither = d.load(&DITHER[pos..]);
                v = d.add(v, dither);
            }
            v = d.clamp(d.zero(), v, mul);
            d.store_i32(d.nearest_int(v), &mut ni);
            for (lane, &n) in ni.iter().enumerate() {
                self.out[i + lane] = (n as u32).min(max) as u16;
            }
        }
    }
}

/// The converted samples of one channel for `StoreUnsignedRow<uint8_t>`
/// (`byte`, with ordered dithering) or `<uint16_t>`; `input` is padded.
#[allow(
    clippy::too_many_arguments,
    reason = "the source MakeUnsigned arguments"
)]
pub fn unsigned_row(
    target: Target,
    input: &[f32],
    len: usize,
    bits: usize,
    byte: bool,
    xstart: usize,
    ypos: usize,
    c: usize,
) -> Vec<u16> {
    let mut out = vec![0u16; input.len()];
    dispatch(
        target,
        UnsignedRow {
            input,
            bits,
            byte,
            xstart,
            ypos,
            c,
            out: &mut out,
        },
    );
    out.truncate(len);
    out
}

/// `BitCast(du, DemoteTo(df16, LoadU(d, &input[c][i])))` per vector.
struct Float16Row<'a> {
    input: &'a [f32],
    out: &'a mut [u16],
}
impl Kernel for Float16Row<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        for i in (0..self.input.len()).step_by(D::LANES) {
            d.store_demote_to_f16(d.load(&self.input[i..]), &mut self.out[i..]);
        }
    }
}

/// The binary16 words of one channel for `StoreFloat16Row`; `input` is padded.
pub fn float16_row(target: Target, input: &[f32], len: usize) -> Vec<u16> {
    let mut out = vec![0u16; input.len()];
    dispatch(
        target,
        Float16Row {
            input,
            out: &mut out,
        },
    );
    out.truncate(len);
    out
}

/// The vector loop of `UnpremulAlpha` on the copied rows: `colors` are the
/// `num_color_` color copies, updated in place; all inputs are padded.
struct Unpremul<'a> {
    colors: &'a mut [Vec<f32>],
    alpha: &'a [f32],
    small_alpha: f32,
}
impl Kernel for Unpremul<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let one = d.set(1.0);
        let small_alpha = d.set(self.small_alpha);
        for ix in (0..self.alpha.len()).step_by(D::LANES) {
            let alpha = d.load(&self.alpha[ix..]);
            let mul = d.div(one, d.max(small_alpha, alpha));
            for color in self.colors.iter_mut() {
                let val = d.load(&color[ix..]);
                d.store(d.mul(val, mul), &mut color[ix..]);
            }
        }
    }
}

/// `UnpremulAlpha`'s arithmetic on padded copies.
pub fn unpremul(target: Target, colors: &mut [Vec<f32>], alpha: &[f32], small_alpha: f32) {
    dispatch(
        target,
        Unpremul {
            colors,
            alpha,
            small_alpha,
        },
    );
}
