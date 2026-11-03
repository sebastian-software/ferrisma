// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AC block entropy decoding and GetBlockFromBitstream scheduling from
//! lib/jxl/dec_group.cc. Typed borrows replace ACPtr and owner-relative row
//! offsets replace cached C++ pointers. Rendering is in dec_group_render.

use super::ac_context::{BlockCtxMap, zero_density_context};
use super::ac_strategy::AcStrategy;
use super::base::status::{Status, StatusCode};
use super::coeff_order::{COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use super::coeff_order_fwd::CoeffOrder;
use super::dct_util::AcPtrMut;
use super::dec_ans::ANSSymbolReader;
use super::dec_bit_reader::BitReader;
use super::entropy_coder::predict_from_top_and_left;
use super::frame_dimensions::DCT_BLOCK_SIZE;

/// Decodes one quantized AC block, retaining its LLF coefficients and adding
/// this pass to existing coefficients. All source array, context, strategy,
/// and predictor preconditions apply. `shift` must be less than usize::BITS;
/// each coefficient plus its existing value must fit source ptrdiff_t before
/// the source's implementation-defined signed narrowing (low bits on Clang).
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn decode_ac_var_block<const USES_LZ77: bool>(
    ctx_offset: usize,
    log2_covered_blocks: usize,
    row_nzeros: &mut [i32],
    row_nzeros_top: Option<&[i32]>,
    nzeros_stride: usize,
    c: usize,
    bx: usize,
    _by: usize,
    lbx: usize,
    acs: AcStrategy,
    coeff_order: &[CoeffOrder],
    br: &mut BitReader<'_>,
    decoder: &mut ANSSymbolReader<'_>,
    context_map: &[u8],
    qdc_row: &[u8],
    qf_row: &[i32],
    block_ctx_map: &BlockCtxMap,
    mut block: AcPtrMut<'_>,
    shift: usize,
) -> Status {
    let covered_blocks = 1usize << log2_covered_blocks;
    let size = covered_blocks * DCT_BLOCK_SIZE;
    let predicted_nzeros = predict_from_top_and_left(row_nzeros_top, row_nzeros, bx, 32);
    let ord = usize::from(STRATEGY_ORDER[usize::from(acs.raw_strategy())]);
    let order = &coeff_order[COEFF_ORDER_OFFSET[3 * ord + c] * DCT_BLOCK_SIZE..];
    let block_ctx = block_ctx_map.context(i32::from(qdc_row[lbx]), qf_row[bx] as u32, ord, c);
    // The source adds size_t ctx_offset, then narrows to int32_t before passing
    // the result back to the decoder's size_t context parameter.
    let nzero_ctx = (block_ctx_map.non_zero_context(predicted_nzeros as u32, block_ctx as u32)
        as usize)
        .wrapping_add(ctx_offset) as i32;
    let mut nzeros =
        decoder.read_hybrid_uint_inlined::<USES_LZ77>(nzero_ctx as usize, br, context_map);
    if nzeros > size - covered_blocks {
        return Err(StatusCode::GenericError);
    }
    for y in 0..acs.covered_blocks_y() {
        for x in 0..acs.covered_blocks_x() {
            row_nzeros[bx + x + y * nzeros_stride] =
                ((nzeros + covered_blocks - 1) >> log2_covered_blocks) as i32;
        }
    }
    let histo_offset = ctx_offset
        .wrapping_add(block_ctx_map.zero_density_contexts_offset(block_ctx as u32) as usize);
    let mut prev = usize::from(nzeros <= size / 16);
    for (k, &position) in order.iter().enumerate().take(size).skip(covered_blocks) {
        if nzeros == 0 {
            break;
        }
        let ctx = histo_offset.wrapping_add(zero_density_context(
            nzeros,
            k,
            covered_blocks,
            log2_covered_blocks,
            prev,
        ));
        let u_coeff = decoder.read_hybrid_uint_inlined::<USES_LZ77>(ctx, br, context_map);
        let magnitude = u_coeff >> 1;
        let neg_sign = (!u_coeff) & 1;
        let coeff = ((magnitude ^ neg_sign.wrapping_sub(1)) << shift) as isize;
        match &mut block {
            AcPtrMut::I16(values) => {
                let value = &mut values[position as usize];
                *value = (*value as isize).wrapping_add(coeff) as i16;
            }
            AcPtrMut::I32(values) => {
                let value = &mut values[position as usize];
                *value = (*value as isize).wrapping_add(coeff) as i32;
            }
        }
        prev = usize::from(u_coeff != 0);
        nzeros -= prev;
    }
    if nzeros != 0 {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}

/// Original GetBlock virtual boundary used by the group traversal.
pub trait GetBlock {
    fn start_row(&mut self, by: usize);
    fn load_block(
        &mut self,
        bx: usize,
        by: usize,
        acs: AcStrategy,
        size: usize,
        log2_covered_blocks: usize,
        block: &mut [AcPtrMut<'_>; 3],
    ) -> Status;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrawMode {
    Draw = 0,
    DontDraw = 1,
}

/// GetBlockFromBitstream from dec_group.cc. References to pass tables and
/// mutable nonzero owners replace its cached pointer members. StartRow records
/// the original row bindings as an index; LoadBlock reborrows those same rows.
pub struct GetBlockFromBitstream<'a, 'r, 'input> {
    shift_for_pass: &'a [u32],
    coeff_orders: &'a [CoeffOrder],
    coeff_order_size: usize,
    context_map: &'a [Vec<u8>],
    pub decoders: Vec<ANSSymbolReader<'a>>,
    readers: &'r mut [BitReader<'input>],
    num_passes: usize,
    ctx_offset: [usize; super::common::MAX_NUM_PASSES],
    nzeros_stride: usize,
    num_nzeroes: &'r mut [super::image::Image3<i32>; super::common::MAX_NUM_PASSES],
    block_ctx_map: &'a BlockCtxMap,
    qf: &'a super::image::Plane<i32>,
    quant_dc: &'a super::image::Plane<u8>,
    rect: super::base::rect::Rect,
    hshift: [usize; 3],
    vshift: [usize; 3],
    row_by: usize,
}
impl<'a, 'r, 'input> GetBlockFromBitstream<'a, 'r, 'input> {
    #[allow(clippy::too_many_arguments)] // Preserve the source initialization inputs.
    pub fn init(
        frame_header: &'a super::frame_header::FrameHeader<'_>,
        readers: &'r mut [BitReader<'input>],
        num_passes: usize,
        histo_selector_bits: usize,
        rect: super::base::rect::Rect,
        num_nzeroes: &'r mut [super::image::Image3<i32>; super::common::MAX_NUM_PASSES],
        shared: &'a super::passes_state::PassesSharedState<'_>,
        code: &'a [super::dec_ans::ANSCode],
        context_map: &'a [Vec<u8>],
        first_pass: usize,
    ) -> super::base::status::StatusOr<Self> {
        let hshift = std::array::from_fn(|c| frame_header.chroma_subsampling.h_shift(c));
        let vshift = std::array::from_fn(|c| frame_header.chroma_subsampling.v_shift(c));
        let coeff_order_size = shared.coeff_order_size;
        let coeff_orders = &shared.coeff_orders[first_pass * coeff_order_size..];
        let context_map = &context_map[first_pass..];
        let shift_for_pass = &frame_header.passes.shift[first_pass..];
        let mut ctx_offset = [0; super::common::MAX_NUM_PASSES];
        let mut decoders = Vec::with_capacity(num_passes);
        for pass in 0..num_passes {
            let mut cur_histogram = 0;
            if histo_selector_bits != 0 {
                cur_histogram = readers[pass].read_bits(histo_selector_bits) as usize;
            }
            if cur_histogram >= shared.num_histograms {
                return Err(StatusCode::GenericError);
            }
            ctx_offset[pass] = cur_histogram * shared.block_ctx_map.num_ac_contexts() as usize;
            decoders.push(ANSSymbolReader::create(
                &code[pass + first_pass],
                &mut readers[pass],
                0,
            )?);
        }
        let nzeros_stride = num_nzeroes[0].pixels_per_row() as usize;
        for image in num_nzeroes.iter().take(num_passes) {
            if nzeros_stride != image.pixels_per_row() as usize {
                return Err(StatusCode::GenericError);
            }
        }
        Ok(Self {
            shift_for_pass,
            coeff_orders,
            coeff_order_size,
            context_map,
            decoders,
            readers,
            num_passes,
            ctx_offset,
            nzeros_stride,
            num_nzeroes,
            block_ctx_map: &shared.block_ctx_map,
            qf: &shared.raw_quant_field,
            quant_dc: &shared.quant_dc,
            rect,
            hshift,
            vshift,
            row_by: 0,
        })
    }
    pub fn check_ans_final_state(&self) -> Status {
        for decoder in &self.decoders {
            if !decoder.check_ans_final_state() {
                return Err(StatusCode::GenericError);
            }
        }
        Ok(())
    }
}
impl GetBlock for GetBlockFromBitstream<'_, '_, '_> {
    fn start_row(&mut self, by: usize) {
        self.row_by = by;
    }
    fn load_block(
        &mut self,
        bx: usize,
        by: usize,
        acs: AcStrategy,
        _size: usize,
        log2_covered_blocks: usize,
        block: &mut [AcPtrMut<'_>; 3],
    ) -> Status {
        let qf_row = &self.qf.row(self.rect.y0() + self.row_by)[self.rect.x0()..];
        let quant_dc_row = &self.quant_dc.row(self.rect.y0() + self.row_by)[self.rect.x0()..];
        for c in [1, 0, 2] {
            let sbx = bx >> self.hshift[c];
            let sby = by >> self.vshift[c];
            if (sbx << self.hshift[c] != bx) || (sby << self.vshift[c] != by) {
                continue;
            }
            for pass in 0..self.num_passes {
                let bound_row = self.row_by >> self.vshift[c];
                let plane = self.num_nzeroes[pass].plane_mut(c);
                let pixels = plane
                    .pixels_mut()
                    .expect("source requires a nonzero image allocation");
                let (previous, row) = pixels.split_at_mut(bound_row * self.nzeros_stride);
                let top = if bound_row == 0 {
                    None
                } else {
                    Some(&previous[(bound_row - 1) * self.nzeros_stride..])
                };
                let block = match &mut block[c] {
                    AcPtrMut::I16(values) => AcPtrMut::I16(values),
                    AcPtrMut::I32(values) => AcPtrMut::I32(values),
                };
                let order = &self.coeff_orders[pass * self.coeff_order_size..];
                let reader = &mut self.readers[pass];
                let decoder = &mut self.decoders[pass];
                if decoder.uses_lz77() {
                    decode_ac_var_block::<true>(
                        self.ctx_offset[pass],
                        log2_covered_blocks,
                        row,
                        top,
                        self.nzeros_stride,
                        c,
                        sbx,
                        sby,
                        bx,
                        acs,
                        order,
                        reader,
                        decoder,
                        &self.context_map[pass],
                        quant_dc_row,
                        qf_row,
                        self.block_ctx_map,
                        block,
                        self.shift_for_pass[pass] as usize,
                    )?;
                } else {
                    decode_ac_var_block::<false>(
                        self.ctx_offset[pass],
                        log2_covered_blocks,
                        row,
                        top,
                        self.nzeros_stride,
                        c,
                        sbx,
                        sby,
                        bx,
                        acs,
                        order,
                        reader,
                        decoder,
                        &self.context_map[pass],
                        quant_dc_row,
                        qf_row,
                        self.block_ctx_map,
                        block,
                        self.shift_for_pass[pass] as usize,
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// The decoder fields used by the regular AC-group pixel path. Field borrows
/// allow the render pipeline to remain mutably borrowed by its input token.
pub struct AcGroupDecoderState<'a, 'metadata> {
    pub shared: &'a super::passes_state::PassesSharedState<'metadata>,
    pub code: &'a [super::dec_ans::ANSCode],
    pub context_map: &'a [Vec<u8>],
    pub coefficients: &'a mut super::dct_util::AcImage,
    pub x_dm_multiplier: f32,
    pub b_dm_multiplier: f32,
    pub quant_biases: &'a [f32; 4],
}

/// The nonzero-pass branch of DecodeGroup, with optional original JPEG output.
/// The caller supplies genuine AC passes, an initialized cache, and i32 AC
/// storage for JPEG. DC-only preview remains a separate source boundary.
#[allow(clippy::too_many_arguments)] // Preserve the reached source arguments.
pub fn decode_ac_group(
    frame_header: &super::frame_header::FrameHeader<'_>,
    readers: &mut [BitReader<'_>],
    num_passes: std::num::NonZeroUsize,
    group_idx: usize,
    state: AcGroupDecoderState<'_, '_>,
    group_dec_cache: &mut super::dec_cache::GroupDecCache,
    render_pipeline_input: &mut super::render_pipeline::simple_render_pipeline::RenderPipelineInput<
        '_,
        '_,
    >,
    jpeg_data: Option<&mut super::jpeg::jpeg_data::JpegData>,
    first_pass: usize,
    force_draw: bool,
    should_run_pipeline: Option<&mut bool>,
) -> Status {
    let num_passes = num_passes.get();
    let draw = if num_passes + first_pass == frame_header.passes.num_passes as usize || force_draw {
        DrawMode::Draw
    } else {
        DrawMode::DontDraw
    };
    if let Some(should_run_pipeline) = should_run_pipeline {
        *should_run_pipeline = draw != DrawMode::DontDraw;
    }
    if state.shared.num_histograms == 0 {
        return Err(StatusCode::GenericError);
    }
    let histo_selector_bits = if state.shared.num_histograms == 1 {
        0
    } else {
        usize::BITS as usize - (state.shared.num_histograms - 1).leading_zeros() as usize
    };
    let (buffers, num_nzeroes) = group_dec_cache.split_buffers();
    let mut get_block = GetBlockFromBitstream::init(
        frame_header,
        readers,
        num_passes,
        histo_selector_bits,
        state.shared.frame_dim.block_group_rect(group_idx),
        num_nzeroes,
        state.shared,
        state.code,
        state.context_map,
        first_pass,
    )?;
    super::dec_group_render::decode_group_impl(
        frame_header,
        &mut get_block,
        buffers,
        state.shared,
        state.coefficients,
        state.x_dm_multiplier,
        state.b_dm_multiplier,
        state.quant_biases,
        group_idx,
        render_pipeline_input,
        jpeg_data,
        draw,
    )?;
    get_block.check_ans_final_state()
}

/// Original null-JPEG specialization retained for existing pixel callers.
#[allow(clippy::too_many_arguments)]
pub fn decode_ac_group_pixels(
    frame_header: &super::frame_header::FrameHeader<'_>,
    readers: &mut [BitReader<'_>],
    num_passes: std::num::NonZeroUsize,
    group_idx: usize,
    state: AcGroupDecoderState<'_, '_>,
    group_dec_cache: &mut super::dec_cache::GroupDecCache,
    render_pipeline_input: &mut super::render_pipeline::simple_render_pipeline::RenderPipelineInput<
        '_,
        '_,
    >,
    first_pass: usize,
    force_draw: bool,
    should_run_pipeline: Option<&mut bool>,
) -> Status {
    decode_ac_group(
        frame_header,
        readers,
        num_passes,
        group_idx,
        state,
        group_dec_cache,
        render_pipeline_input,
        None,
        first_pass,
        force_draw,
        should_run_pipeline,
    )
}
