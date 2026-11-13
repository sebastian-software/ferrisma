// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD `Random3Planes`/`RandomImage`/`BitsToFloat` of lib/jxl/dec_noise.cc
//! for the NEON and AVX2 targets (ADR 0004), written once against
//! [`SimdF32`]. The random bits come from the same Xorshift128Plus as the
//! scalar path (its vector `Fill` computes the same 64-bit words lane by
//! lane). What the target changes is the store width: `BitsToFloat` stores
//! whole vectors, so the final span of each row is written up to
//! `RoundUpTo(remaining, Lanes)` samples, into the plane's row padding.
use super::base::rect::Rect;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::Plane;
use super::xorshift128plus::Xorshift128Plus;

/// The original's `memcpy(batch32, batch64, sizeof(batch32))`, in host byte
/// order.
fn next_batch(rng: &mut Xorshift128Plus) -> [u32; 2 * Xorshift128Plus::N] {
    let words = rng.fill();
    std::array::from_fn(|i| {
        let bytes = words[i / 2].to_ne_bytes();
        u32::from_ne_bytes(bytes[4 * (i % 2)..4 * (i % 2) + 4].try_into().unwrap())
    })
}

/// `BitsToFloat(random_bits, floats)`: one vector of `[1, 2)` floats.
#[inline(always)]
fn bits_to_float<D: SimdF32>(d: D, random_bits: &[u32], floats: &mut [f32]) {
    let bits = d.bitcast_to_i32(d.load_bits(random_bits));
    // 1.0 + 23 random mantissa bits = [1, 2)
    let rand12 = d.or(
        d.bitcast_to_f32(d.shift_right_u32::<9>(bits)),
        d.load_bits(&[0x3F80_0000; 16]),
    );
    d.store(rand12, floats);
}

struct RandomImage<'a> {
    rng: &'a mut Xorshift128Plus,
    rect: Rect,
    noise: &'a mut Plane<f32>,
}
impl Kernel for RandomImage<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        const FLOATS_PER_BATCH: usize = 2 * Xorshift128Plus::N;
        let n = D::LANES;
        let xsize = self.rect.xsize();
        for y in 0..self.rect.ysize() {
            let row = &mut self.noise.row_mut(self.rect.y0() + y)[self.rect.x0()..];
            let mut x = 0;
            // Only entire batches (avoids exceeding the image padding).
            while x + FLOATS_PER_BATCH < xsize {
                let batch32 = next_batch(self.rng);
                let mut i = 0;
                while i < FLOATS_PER_BATCH {
                    bits_to_float(d, &batch32[i..], &mut row[x + i..]);
                    i += n;
                }
                x += FLOATS_PER_BATCH;
            }
            // Any remaining pixels, rounded up to vectors (safe due to padding).
            let batch32 = next_batch(self.rng);
            let mut batch_pos = 0; // < kFloatsPerBatch
            while x < xsize {
                bits_to_float(d, &batch32[batch_pos..], &mut row[x..]);
                x += n;
                batch_pos += n;
            }
        }
    }
}

/// `Random3Planes` instantiated for one SIMD target.
pub fn random_three_planes_simd(
    target: Target,
    visible_frame_index: usize,
    nonvisible_frame_index: usize,
    x0: usize,
    y0: usize,
    planes: [(&mut Plane<f32>, Rect); 3],
) {
    let mut rng = Xorshift128Plus::from_seeds(
        visible_frame_index as u32,
        nonvisible_frame_index as u32,
        x0 as u32,
        y0 as u32,
    );
    for (noise, rect) in planes {
        random_image_simd(target, &mut rng, rect, noise);
    }
}

/// `RandomImage` instantiated for one SIMD target.
pub fn random_image_simd(
    target: Target,
    rng: &mut Xorshift128Plus,
    rect: Rect,
    noise: &mut Plane<f32>,
) {
    dispatch(target, RandomImage { rng, rect, noise });
}
