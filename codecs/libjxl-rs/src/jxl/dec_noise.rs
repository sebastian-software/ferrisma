// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Scalar noise decoding and generation from lib/jxl/dec_noise.cc; the
//! NEON/AVX2 generation is dec_noise_simd.rs.
//! Owner-relative pipeline buffers replace the original non-owning plane/Rect
//! pairs. The caller retains the original GetInputBuffers/Done lifecycle.
use super::base::rect::Rect;
use super::base::status::Status;
use super::dec_bit_reader::BitReader;
use super::frame_dimensions::FrameDimensions;
use super::frame_header::FrameHeader;
use super::image::Plane;
use super::noise::{NOISE_PRECISION, NoiseParams};
use super::render_pipeline::simple_render_pipeline::RenderPipelineInput;
use super::xorshift128plus::Xorshift128Plus;

pub fn decode_float_param(precision: f32, val: &mut f32, br: &mut BitReader<'_>) {
    let absval_quant = br.read_fixed_bits::<10>() as i32;
    *val = absval_quant as f32 / precision;
}
pub fn decode_noise(br: &mut BitReader<'_>, noise_params: &mut NoiseParams) -> Status {
    for value in &mut noise_params.lut {
        decode_float_param(NOISE_PRECISION, value, br);
    }
    Ok(())
}
pub fn bits_to_float(random_bits: u32) -> f32 {
    f32::from_bits((random_bits >> 9) | 0x3f800000)
}
fn next_batch(rng: &mut Xorshift128Plus) -> [u32; 2 * Xorshift128Plus::N] {
    let words = rng.fill();
    // Original memcpy preserves host byte order rather than a fixed word order.
    std::array::from_fn(|i| {
        let bytes = words[i / 2].to_ne_bytes();
        u32::from_ne_bytes(bytes[4 * (i % 2)..4 * (i % 2) + 4].try_into().unwrap())
    })
}
pub fn random_image(rng: &mut Xorshift128Plus, rect: Rect, noise: &mut Plane<f32>) {
    const BATCH: usize = 2 * Xorshift128Plus::N;
    for y in 0..rect.ysize() {
        let row = &mut noise.row_mut(rect.y0() + y)[rect.x0()..];
        let mut x = 0;
        while x + BATCH < rect.xsize() {
            let batch = next_batch(rng);
            for i in 0..BATCH {
                row[x + i] = bits_to_float(batch[i]);
            }
            x += BATCH;
        }
        // The source fills a batch even when the final span is empty.
        let batch = next_batch(rng);
        let mut batch_pos = 0;
        while x < rect.xsize() {
            row[x] = bits_to_float(batch[batch_pos]);
            x += 1;
            batch_pos += 1;
        }
    }
}
pub fn random_three_planes(
    visible_frame_index: usize,
    nonvisible_frame_index: usize,
    x0: usize,
    y0: usize,
    planes: [(&mut Plane<f32>, Rect); 3],
) {
    // HWY_DYNAMIC_DISPATCH(Random3Planes) (port ADR 0004).
    if let Some(target) = super::hwy::decoder_target() {
        return super::dec_noise_simd::random_three_planes_simd(
            target,
            visible_frame_index,
            nonvisible_frame_index,
            x0,
            y0,
            planes,
        );
    }
    let mut rng = Xorshift128Plus::from_seeds(
        visible_frame_index as u32,
        nonvisible_frame_index as u32,
        x0 as u32,
        y0 as u32,
    );
    for (plane, rect) in planes {
        random_image(&mut rng, rect, plane);
    }
}
/// Original PrepareNoiseInput body with the input handle supplied explicitly.
/// Acquiring that handle does not mutate decoder state in the simple pipeline;
/// separating it avoids retaining an alias to the pipeline through its owner.
pub fn prepare_noise_input(
    visible_frame_index: usize,
    nonvisible_frame_index: usize,
    frame_dim: &FrameDimensions,
    frame_header: &FrameHeader<'_>,
    group_index: usize,
    input: &mut RenderPipelineInput<'_, '_>,
) {
    let group_dim = frame_dim.group_dim;
    let gx = group_index % frame_dim.xsize_groups;
    let gy = group_index / frame_dim.xsize_groups;
    let noise_c_start = 3 + frame_header
        .nonserialized_metadata
        .unwrap()
        .m
        .num_extra_channels as usize;
    // HWY_DYNAMIC_DISPATCH(Random3Planes) selects the same target for
    // every call (port ADR 0004).
    let target = super::hwy::decoder_target();
    for iy in 0..frame_header.upsampling as usize {
        for ix in 0..frame_header.upsampling as usize {
            let mut rng = Xorshift128Plus::from_seeds(
                visible_frame_index as u32,
                nonvisible_frame_index as u32,
                ((gx * frame_header.upsampling as usize + ix) * group_dim) as u32,
                ((gy * frame_header.upsampling as usize + iy) * group_dim) as u32,
            );
            for c in 0..3 {
                let (plane, r) = input.get_buffer_mut(noise_c_start + c);
                let rect = Rect::new_clamped(
                    r.x0() + ix * group_dim,
                    r.y0() + iy * group_dim,
                    group_dim,
                    group_dim,
                    r.x1(),
                    r.y1(),
                );
                match target {
                    Some(target) => {
                        super::dec_noise_simd::random_image_simd(target, &mut rng, rect, plane)
                    }
                    None => random_image(&mut rng, rect, plane),
                }
            }
        }
    }
}
