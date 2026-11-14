// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar DequantLane/DequantBlock and DecodeGroupImpl from
//! `lib/jxl/dec_group.cc`, including original quantized JPEG reconstruction.

use super::ac_strategy::AcStrategyType;
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::chroma_from_luma::COLOR_TILE_DIM_IN_BLOCKS;
use super::dct_util::{AcImage, AcPtrMut, AcType, ConstAcPtr, GroupCoefficients};
use super::dec_cache::GroupDecBuffers;
use super::dec_group::{DrawMode, GetBlock, GroupDecoderShared, with_group_shared};
use super::dec_transforms::{lowest_frequencies_from_dc, transform_to_pixels};
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE};
use super::frame_header::FrameHeader;
use super::passes_state::PassesSharedState;
use super::quant_weights::DequantMatrices;
use super::quantizer::Quantizer;
use super::quantizer_inl::adjust_quant_bias;
use super::render_pipeline::simple_render_pipeline::RenderPipelineInput;

fn coefficient(block: &ConstAcPtr<'_>, k: usize) -> i32 {
    match block {
        ConstAcPtr::I16(values) => i32::from(values[k]),
        ConstAcPtr::I32(values) => values[k],
    }
}

// The scalar addition of the dequantized residual and the CfL prediction. Its
// operand order is that of the AArch64 mapping on every target; the order
// only selects between competing NaN payloads, which are not gated (D8).
#[inline]
fn add_dequant(first: f32, second: f32) -> f32 {
    first + second
}

/// One scalar Highway lane; the three matrix views borrow the same owner.
#[allow(clippy::too_many_arguments)]
pub fn dequant_lane(
    scaled_dequant: [f32; 3],
    dequant_matrices: [&[f32]; 3],
    size: usize,
    k: usize,
    x_cc_mul: f32,
    b_cc_mul: f32,
    biases: &[f32; 4],
    qblock: &[ConstAcPtr<'_>; 3],
    block: &mut [f32],
) {
    let x_mul = dequant_matrices[0][k] * scaled_dequant[0];
    let y_mul = dequant_matrices[1][k] * scaled_dequant[1];
    let b_mul = dequant_matrices[2][k] * scaled_dequant[2];
    let quantized_x_int = coefficient(&qblock[0], k);
    let quantized_y_int = coefficient(&qblock[1], k);
    let quantized_b_int = coefficient(&qblock[2], k);
    let dequant_x_cc = adjust_quant_bias(0, quantized_x_int, biases) * x_mul;
    let dequant_y = adjust_quant_bias(1, quantized_y_int, biases) * y_mul;
    let dequant_b_cc = adjust_quant_bias(2, quantized_b_int, biases) * b_mul;
    let (dequant_x, dequant_b) = (
        add_dequant(dequant_x_cc, x_cc_mul * dequant_y),
        b_cc_mul * dequant_y + dequant_b_cc,
    );
    block[k] = dequant_x;
    block[size + k] = dequant_y;
    block[2 * size + k] = dequant_b;
}

/// Original scalar block dequantization followed by LLF replacement from DC.
#[allow(clippy::too_many_arguments)]
pub fn dequant_block(
    inv_global_scale: f32,
    quant: i32,
    x_dm_multiplier: f32,
    b_dm_multiplier: f32,
    x_cc_mul: f32,
    b_cc_mul: f32,
    kind: AcStrategyType,
    size: usize,
    quantizer: &Quantizer,
    matrices: &DequantMatrices,
    covered_blocks: usize,
    sbx: [usize; 3],
    dc_rows: [&[f32]; 3],
    dc_stride: usize,
    biases: &[f32; 4],
    qblock: &[ConstAcPtr<'_>; 3],
    block: &mut [f32],
    scratch: &mut [f32],
) {
    let scaled_dequant_s = inv_global_scale / quant as f32;
    let scaled_dequant = [
        scaled_dequant_s * x_dm_multiplier,
        scaled_dequant_s,
        scaled_dequant_s * b_dm_multiplier,
    ];
    let dequant_matrices = std::array::from_fn(|c| quantizer.dequant_matrix(matrices, kind, c));
    for k in 0..covered_blocks * DCT_BLOCK_SIZE {
        dequant_lane(
            scaled_dequant,
            dequant_matrices,
            size,
            k,
            x_cc_mul,
            b_cc_mul,
            biases,
            qblock,
            block,
        );
    }
    for c in 0..3 {
        lowest_frequencies_from_dc(
            kind,
            &dc_rows[c][sbx[c]..],
            dc_stride,
            &mut block[c * size..],
            scratch,
        );
    }
}

fn const_blocks<'a>(blocks: &'a [AcPtrMut<'_>; 3]) -> [ConstAcPtr<'a>; 3] {
    blocks.each_ref().map(|block| match block {
        AcPtrMut::I16(values) => ConstAcPtr::I16(values),
        AcPtrMut::I32(values) => ConstAcPtr::I32(values),
    })
}

/// Complete source traversal for pixel or original quantized JPEG output.
/// The caller retains valid strategy geometry, initialized DC/matrix/correlation
/// owners, and prepared pipeline buffers. JPEG requires i32 coefficient storage;
/// RAW table divisions and float conversions retain source definedness domains.
#[allow(clippy::too_many_arguments)]
pub fn decode_group_impl(
    frame_header: &FrameHeader<'_>,
    get_block: &mut impl GetBlock,
    buffers: GroupDecBuffers<'_>,
    shared: &PassesSharedState<'_>,
    coefficients: &mut AcImage,
    x_dm_multiplier: f32,
    b_dm_multiplier: f32,
    biases: &[f32; 4],
    group_idx: usize,
    render_pipeline_input: &mut RenderPipelineInput<'_, '_>,
    jpeg_data: Option<&mut super::jpeg::jpeg_data::JpegData>,
    draw: DrawMode,
) -> Status {
    with_group_shared(shared, |shared| {
        decode_group_impl_shared(
            frame_header,
            get_block,
            buffers,
            shared,
            coefficients.group_rows(group_idx),
            x_dm_multiplier,
            b_dm_multiplier,
            biases,
            group_idx,
            render_pipeline_input,
            jpeg_data,
            draw,
        )
    })
}

/// DecodeGroupImpl through the shared view (port ADR 0003, "Concurrent
/// decoder tasks"); see `decode_group_impl`.
#[allow(clippy::too_many_arguments)]
pub fn decode_group_impl_shared(
    frame_header: &FrameHeader<'_>,
    get_block: &mut impl GetBlock,
    buffers: GroupDecBuffers<'_>,
    shared: &GroupDecoderShared<'_>,
    mut coefficients: GroupCoefficients<'_>,
    x_dm_multiplier: f32,
    b_dm_multiplier: f32,
    biases: &[f32; 4],
    group_idx: usize,
    render_pipeline_input: &mut RenderPipelineInput<'_, '_>,
    mut jpeg_data: Option<&mut super::jpeg::jpeg_data::JpegData>,
    draw: DrawMode,
) -> Status {
    // vardct kernels: HWY_DYNAMIC_DISPATCH(DecodeGroupImpl) selects the
    // target once per group (port ADR 0004, "Decoder path").
    let simd = super::hwy::decoder_target();
    let block_rect = shared.frame_dim.block_group_rect(group_idx);
    let xsize_blocks = block_rect.xsize();
    let ysize_blocks = block_rect.ysize();
    let dc = shared.dc();
    let dc_stride = dc.pixels_per_row() as usize;
    let inv_global_scale = shared.quantizer.inv_global_scale();
    let cs = &frame_header.chroma_subsampling;
    let ac_type = coefficients.image_type();
    let accumulate = !coefficients.is_empty();
    let mut offset = 0;
    let color_correlation = shared.cmap.base();
    let mut scaled_qtable = [0i32; 64 * 3];
    let mut jpeg_c_map = [0usize; 3];
    let mut jpeg_is_gray = false;
    let mut dcoff = [0i32; 3];
    if let Some(jpeg) = jpeg_data.as_deref() {
        if !color_correlation.is_jpeg_compatible() {
            return Err(StatusCode::GenericError);
        }
        jpeg_is_gray = jpeg.components.len() == 1;
        if frame_header.color_transform == super::frame_header::ColorTransform::XYB {
            return Err(StatusCode::GenericError);
        }
        jpeg_c_map = super::frame_header::jpeg_order(frame_header.color_transform, jpeg_is_gray)
            .map(|c| c as usize);
        let encodings = shared.matrices.encodings();
        let Some(encoding) = encodings.first() else {
            return Err(StatusCode::GenericError);
        };
        let super::quant_weights::QuantEncodingData::Raw { qtable, qtable_den } = encoding.data()
        else {
            return Err(StatusCode::GenericError);
        };
        if (*qtable_den - 1.0f32 / (8 * 255) as f32).abs() > 1e-8f32 {
            return Err(StatusCode::GenericError);
        }
        let qtable = qtable
            .as_deref()
            .expect("source RAW quantization allocation");
        if qtable.len() != 3 * 8 * 8 {
            return Err(StatusCode::GenericError);
        }
        for c in 0..3 {
            if frame_header.color_transform == super::frame_header::ColorTransform::NONE {
                // The source divides before validating this table entry.
                dcoff[c] = 1024 / qtable[64 * c];
            }
            for i in 0..64 {
                let num = qtable[64 + i];
                let den = qtable[64 * c + i];
                if num <= 0 || den <= 0 || num >= 65536 || den >= 65536 {
                    return Err(StatusCode::GenericError);
                }
                scaled_qtable[64 * c + (i % 8) * 8 + i / 8] =
                    (1 << super::chroma_from_luma::CFL_FIXED_POINT_PRECISION) * num / den;
            }
        }
    }
    let hshift: [usize; 3] = std::array::from_fn(|c| cs.h_shift(c));
    let vshift: [usize; 3] = std::array::from_fn(|c| cs.v_shift(c));
    let rects: [Rect; 3] = std::array::from_fn(|c| {
        Rect::new(
            block_rect.x0() >> hshift[c],
            block_rect.y0() >> vshift[c],
            xsize_blocks >> hshift[c],
            ysize_blocks >> vshift[c],
        )
    });
    for (c, r) in rects.iter().enumerate() {
        if !r.is_inside(Rect::new(0, 0, dc.plane(c).xsize(), dc.plane(c).ysize())) {
            return Err(StatusCode::GenericError);
        }
    }
    for by in 0..ysize_blocks {
        get_block.start_row(by);
        let sby: [usize; 3] = std::array::from_fn(|c| by >> vshift[c]);
        let row_quant = &shared.raw_quant_field.row(block_rect.y0() + by)[block_rect.x0()..];
        let dc_rows = std::array::from_fn(|c| {
            let plane = dc.plane(c);
            let index = (rects[c].y0() + sby[c]) * dc_stride + rects[c].x0();
            &plane.pixels().expect("source DC allocation")[index..]
        });
        let ty = (block_rect.y0() + by) / COLOR_TILE_DIM_IN_BLOCKS;
        let acs_row = shared.ac_strategy.const_rect_row(block_rect, by);
        let row_cmap_x = shared.cmap.ytox_map.row(ty);
        let row_cmap_b = shared.cmap.ytob_map.row(ty);
        let mut bx = 0;
        for tx in 0..xsize_blocks.div_ceil(COLOR_TILE_DIM_IN_BLOCKS) {
            let abs_tx = tx + block_rect.x0() / COLOR_TILE_DIM_IN_BLOCKS;
            // vardct kernels: on a SIMD target the ratios are computed
            // inside the target's DecodeGroupImpl, contracted (FMA).
            let (x_cc_mul, b_cc_mul) = match simd {
                Some(_) => (
                    color_correlation.y_to_x_ratio_simd(i32::from(row_cmap_x[abs_tx])),
                    color_correlation.y_to_b_ratio_simd(i32::from(row_cmap_b[abs_tx])),
                ),
                None => (
                    color_correlation.y_to_x_ratio(i32::from(row_cmap_x[abs_tx])),
                    color_correlation.y_to_b_ratio(i32::from(row_cmap_b[abs_tx])),
                ),
            };
            while bx < xsize_blocks && bx < (tx + 1) * COLOR_TILE_DIM_IN_BLOCKS {
                let sbx = std::array::from_fn(|c| bx >> hshift[c]);
                let acs = acs_row.get(bx);
                let llf_x = acs.covered_blocks_x();
                if !acs.is_first_block() {
                    bx += llf_x;
                    continue;
                }
                let log2_covered_blocks = acs.log2_covered_blocks();
                let covered_blocks = 1usize << log2_covered_blocks;
                let size = covered_blocks * DCT_BLOCK_SIZE;
                let mut qblock = if accumulate {
                    coefficients.from_offset(offset)
                } else {
                    if draw != DrawMode::Draw {
                        return Err(StatusCode::GenericError);
                    }
                    match ac_type {
                        AcType::I16 => {
                            buffers.qblock16[..size * 3].fill(0);
                            let (x, rest) = buffers.qblock16[..size * 3].split_at_mut(size);
                            let (y, b) = rest.split_at_mut(size);
                            [AcPtrMut::I16(x), AcPtrMut::I16(y), AcPtrMut::I16(b)]
                        }
                        AcType::I32 => {
                            buffers.qblock[..size * 3].fill(0);
                            let (x, rest) = buffers.qblock[..size * 3].split_at_mut(size);
                            let (y, b) = rest.split_at_mut(size);
                            [AcPtrMut::I32(x), AcPtrMut::I32(y), AcPtrMut::I32(b)]
                        }
                    }
                };
                get_block.load_block(bx, by, acs, size, log2_covered_blocks, &mut qblock)?;
                offset += size;
                if draw == DrawMode::DontDraw {
                    bx += llf_x;
                    continue;
                }
                if let Some(jpeg) = jpeg_data.as_deref_mut() {
                    if acs.strategy() != AcStrategyType::DCT {
                        return Err(StatusCode::GenericError);
                    }
                    let mut transposed_dct_y = [0i32; 64];
                    for c in [1, 0, 2] {
                        if jpeg_is_gray && c != 1 {
                            continue;
                        }
                        if sbx[c] << hshift[c] != bx || sby[c] << vshift[c] != by {
                            continue;
                        }
                        let component = &mut jpeg.components[jpeg_c_map[c]];
                        let start = (component.width_in_blocks as usize * (rects[c].y0() + sby[c])
                            + rects[c].x0()
                            + sbx[c])
                            * 64;
                        let jpeg_pos = &mut component.coeffs[start..start + 64];
                        let AcPtrMut::I32(transposed_dct) = &mut qblock[c] else {
                            panic!("source JPEG reconstruction uses i32 coefficients");
                        };
                        // Mutate the actual accumulated qblock, as the source does.
                        for x in 0..8 {
                            for y in x + 1..8 {
                                transposed_dct.swap(y * 8 + x, x * 8 + y);
                            }
                        }
                        if !cs.is_444() || (row_cmap_x[abs_tx] == 0 && row_cmap_b[abs_tx] == 0) {
                            for i in 0..64 {
                                jpeg_pos[i] = transposed_dct[i]
                                    .clamp(i16::MIN as i32, i16::MAX as i32)
                                    as i16;
                            }
                        } else if c == 1 {
                            for i in 0..64 {
                                transposed_dct_y[i] = transposed_dct[i];
                                jpeg_pos[i] = transposed_dct[i]
                                    .clamp(i16::MIN as i32, i16::MAX as i32)
                                    as i16;
                            }
                        } else {
                            let factor = if c == 0 {
                                row_cmap_x[abs_tx]
                            } else {
                                row_cmap_b[abs_tx]
                            };
                            let scale = super::chroma_from_luma::ColorCorrelation::ratio_jpeg(
                                i32::from(factor),
                            );
                            let precision = super::chroma_from_luma::CFL_FIXED_POINT_PRECISION;
                            let round = 1i32 << (precision - 1);
                            for i in 0..64 {
                                // Highway signed vector Add/Mul retain low 32 bits.
                                let coeff_scale = scaled_qtable[c * size + i]
                                    .wrapping_mul(scale)
                                    .wrapping_add(round)
                                    >> precision;
                                let cfl_factor = transposed_dct_y[i]
                                    .wrapping_mul(coeff_scale)
                                    .wrapping_add(round)
                                    >> precision;
                                jpeg_pos[i] = transposed_dct[i]
                                    .wrapping_add(cfl_factor)
                                    .clamp(i16::MIN as i32, i16::MAX as i32)
                                    as i16;
                            }
                        }
                        let dc = dc_rows[c][sbx[c]] - dcoff[c] as f32;
                        // Clamp1 uses ordered comparisons; defined source casts
                        // require a finite value after this clamp.
                        #[allow(clippy::manual_clamp)]
                        // Preserve original ordered Clamp1 comparisons.
                        let dc = if dc < -2047.0 {
                            -2047.0
                        } else if dc > 2047.0 {
                            2047.0
                        } else {
                            dc
                        };
                        assert!(
                            dc.is_finite(),
                            "source float-to-int conversion must be defined"
                        );
                        jpeg_pos[0] = dc as i16;
                        if jpeg_pos.iter().any(|&v| !(-4095..=4095).contains(&v)) {
                            return Err(StatusCode::GenericError);
                        }
                    }
                    bx += llf_x;
                    continue;
                }
                // vardct kernels: the SIMD DequantBlock on both targets; on
                // AVX2 it executes the original's vendor-specific rcpps
                // (working decision 2026-11-13 evening, delegated, revisable
                // by the owner; port ADR 0004).
                match simd {
                    Some(t) => super::dec_group_simd::dequant_block_simd(
                        t,
                        super::dec_group_simd::DequantBlockArgs {
                            inv_global_scale,
                            quant: row_quant[bx],
                            x_dm_multiplier,
                            b_dm_multiplier,
                            x_cc_mul,
                            b_cc_mul,
                            kind: acs.strategy(),
                            size,
                            quantizer: shared.quantizer,
                            matrices: shared.matrices,
                            covered_blocks: acs.covered_blocks_y() * acs.covered_blocks_x(),
                            sbx,
                            dc_rows,
                            dc_stride,
                            biases,
                            qblock: &const_blocks(&qblock),
                            block: &mut *buffers.block,
                            scratch: &mut *buffers.scratch,
                        },
                    ),
                    None => dequant_block(
                        inv_global_scale,
                        row_quant[bx],
                        x_dm_multiplier,
                        b_dm_multiplier,
                        x_cc_mul,
                        b_cc_mul,
                        acs.strategy(),
                        size,
                        shared.quantizer,
                        shared.matrices,
                        acs.covered_blocks_y() * acs.covered_blocks_x(),
                        sbx,
                        dc_rows,
                        dc_stride,
                        biases,
                        &const_blocks(&qblock),
                        buffers.block,
                        buffers.scratch,
                    ),
                }
                for c in [1, 0, 2] {
                    if (sbx[c] << hshift[c] != bx) || (sby[c] << vshift[c] != by) {
                        continue;
                    }
                    let (plane, rect) = render_pipeline_input.get_buffer_mut(c);
                    let stride = plane.pixels_per_row() as usize;
                    let index =
                        (rect.y0() + sby[c] * BLOCK_DIM) * stride + rect.x0() + sbx[c] * BLOCK_DIM;
                    let pixels =
                        &mut plane.pixels_mut().expect("source pipeline allocation")[index..];
                    // vardct kernels: the SIMD TransformToPixels.
                    match simd {
                        Some(t) => super::dec_transforms_simd::transform_to_pixels_simd(
                            t,
                            acs.strategy(),
                            &mut buffers.block[c * size..],
                            pixels,
                            stride,
                            buffers.scratch,
                        ),
                        None => transform_to_pixels(
                            acs.strategy(),
                            &mut buffers.block[c * size..],
                            pixels,
                            stride,
                            buffers.scratch,
                        ),
                    }
                }
                bx += llf_x;
            }
        }
    }
    Ok(())
}

/// Original null-JPEG specialization retained for existing callers.
#[allow(clippy::too_many_arguments)]
pub fn decode_group_impl_pixels(
    frame_header: &FrameHeader<'_>,
    get_block: &mut impl GetBlock,
    buffers: GroupDecBuffers<'_>,
    shared: &PassesSharedState<'_>,
    coefficients: &mut AcImage,
    x_dm_multiplier: f32,
    b_dm_multiplier: f32,
    biases: &[f32; 4],
    group_idx: usize,
    render_pipeline_input: &mut RenderPipelineInput<'_, '_>,
    draw: DrawMode,
) -> Status {
    decode_group_impl(
        frame_header,
        get_block,
        buffers,
        shared,
        coefficients,
        x_dm_multiplier,
        b_dm_multiplier,
        biases,
        group_idx,
        render_pipeline_input,
        None,
        draw,
    )
}
