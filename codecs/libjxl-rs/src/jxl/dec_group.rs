// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AC block entropy decoding and GetBlockFromBitstream scheduling from
//! lib/jxl/dec_group.cc. Typed borrows replace ACPtr and owner-relative row
//! offsets replace cached C++ pointers. Rendering is in dec_group_render.
//!
//! The group bodies read the frame's shared state through
//! `GroupDecoderShared`, a `Sync` view of the PassesSharedState fields they
//! use, so that AC groups can decode concurrently on a runner's threads (port
//! ADR 0003, "Concurrent decoder tasks"). The view lends the DC image and the
//! base color correlation from the guards its caller holds; the entry points
//! that take a `PassesSharedState` build it for the length of their call.

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

/// The PassesSharedState fields a group body reads, as plain shared
/// references (port ADR 0003, "Concurrent decoder tasks"). The source passes
/// `dec_state->shared`; its owner also holds `Rc`/`RefCell` handles
/// (reference frames, image features, the color-correlation base), so it is
/// not `Sync`. This view is: the DC image and the base correlation are lent
/// from `Ref` guards that the view's creator holds while it is in use.
pub struct GroupDecoderShared<'a> {
    pub frame_dim: super::frame_dimensions::FrameDimensions,
    pub ac_strategy: &'a super::ac_strategy::AcStrategyImage,
    pub matrices: &'a super::quant_weights::DequantMatrices,
    pub quantizer: &'a super::quantizer::Quantizer,
    pub raw_quant_field: &'a super::image::Plane<i32>,
    pub cmap: ColorCorrelationView<'a>,
    pub coeff_order_size: usize,
    pub coeff_orders: &'a [CoeffOrder],
    pub quant_dc: &'a super::image::Plane<u8>,
    pub block_ctx_map: &'a BlockCtxMap,
    pub num_histograms: usize,
    // None while the DC image is a DC frame's, which another frame's output
    // stage may borrow mutably: the caller then lends it per group
    // (`with_dc`).
    dc: Option<&'a super::image::Image3<f32>>,
    memory_manager: super::memory_manager_internal::MemoryManagerPtr,
}
/// The ColorCorrelationMap fields a group body reads, with its base
/// correlation lent from the caller's guard.
pub struct ColorCorrelationView<'a> {
    pub ytox_map: &'a super::image::Plane<i8>,
    pub ytob_map: &'a super::image::Plane<i8>,
    base: &'a super::chroma_from_luma::ColorCorrelation,
}
impl<'a> ColorCorrelationView<'a> {
    pub fn base(&self) -> &'a super::chroma_from_luma::ColorCorrelation {
        self.base
    }
}
impl<'a> GroupDecoderShared<'a> {
    /// `dc` is `shared.dc()` (or None, see `with_dc`) and `cmap_base` is
    /// `shared.cmap.base()`, both borrowed from guards the caller holds
    /// while the view is used.
    pub fn new(
        shared: &'a super::passes_state::PassesSharedState<'_>,
        dc: Option<&'a super::image::Image3<f32>>,
        cmap_base: &'a super::chroma_from_luma::ColorCorrelation,
    ) -> Self {
        Self {
            frame_dim: shared.frame_dim,
            ac_strategy: &shared.ac_strategy,
            matrices: &shared.matrices,
            quantizer: &shared.quantizer,
            raw_quant_field: &shared.raw_quant_field,
            cmap: ColorCorrelationView {
                ytox_map: &shared.cmap.ytox_map,
                ytob_map: &shared.cmap.ytob_map,
                base: cmap_base,
            },
            coeff_order_size: shared.coeff_order_size,
            coeff_orders: &shared.coeff_orders,
            quant_dc: &shared.quant_dc,
            block_ctx_map: &shared.block_ctx_map,
            num_histograms: shared.num_histograms,
            dc,
            memory_manager: shared.memory_manager(),
        }
    }
    pub fn dc(&self) -> &'a super::image::Image3<f32> {
        self.dc.expect("the view's creator lends the DC image")
    }
    /// The same view with `dc` as its DC image.
    pub fn with_dc<'b>(&self, dc: &'b super::image::Image3<f32>) -> GroupDecoderShared<'b>
    where
        'a: 'b,
    {
        GroupDecoderShared {
            frame_dim: self.frame_dim,
            ac_strategy: self.ac_strategy,
            matrices: self.matrices,
            quantizer: self.quantizer,
            raw_quant_field: self.raw_quant_field,
            cmap: ColorCorrelationView {
                ytox_map: self.cmap.ytox_map,
                ytob_map: self.cmap.ytob_map,
                base: self.cmap.base,
            },
            coeff_order_size: self.coeff_order_size,
            coeff_orders: self.coeff_orders,
            quant_dc: self.quant_dc,
            block_ctx_map: self.block_ctx_map,
            num_histograms: self.num_histograms,
            dc: Some(dc),
            memory_manager: self.memory_manager,
        }
    }
    pub fn memory_manager(&self) -> super::memory_manager_internal::MemoryManagerPtr {
        self.memory_manager
    }
}
/// Runs `f` with the view of `shared`, holding its DC and base-correlation
/// guards for the call.
pub fn with_group_shared<R>(
    shared: &super::passes_state::PassesSharedState<'_>,
    f: impl FnOnce(&GroupDecoderShared<'_>) -> R,
) -> R {
    let dc = shared.dc();
    let base = shared.cmap.base();
    f(&GroupDecoderShared::new(shared, Some(&dc), &base))
}
/// The shared tables GetBlockFromBitstream::Init reads.
struct BlockTables<'a> {
    coeff_order_size: usize,
    coeff_orders: &'a [CoeffOrder],
    num_histograms: usize,
    block_ctx_map: &'a BlockCtxMap,
    raw_quant_field: &'a super::image::Plane<i32>,
    quant_dc: &'a super::image::Plane<u8>,
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
        let tables = BlockTables {
            coeff_order_size: shared.coeff_order_size,
            coeff_orders: &shared.coeff_orders,
            num_histograms: shared.num_histograms,
            block_ctx_map: &shared.block_ctx_map,
            raw_quant_field: &shared.raw_quant_field,
            quant_dc: &shared.quant_dc,
        };
        Self::init_tables(
            frame_header,
            readers,
            num_passes,
            histo_selector_bits,
            rect,
            num_nzeroes,
            tables,
            code,
            context_map,
            first_pass,
        )
    }
    /// Init through the shared view.
    #[allow(clippy::too_many_arguments)] // Preserve the source initialization inputs.
    pub fn init_shared(
        frame_header: &'a super::frame_header::FrameHeader<'_>,
        readers: &'r mut [BitReader<'input>],
        num_passes: usize,
        histo_selector_bits: usize,
        rect: super::base::rect::Rect,
        num_nzeroes: &'r mut [super::image::Image3<i32>; super::common::MAX_NUM_PASSES],
        shared: &GroupDecoderShared<'a>,
        code: &'a [super::dec_ans::ANSCode],
        context_map: &'a [Vec<u8>],
        first_pass: usize,
    ) -> super::base::status::StatusOr<Self> {
        let tables = BlockTables {
            coeff_order_size: shared.coeff_order_size,
            coeff_orders: shared.coeff_orders,
            num_histograms: shared.num_histograms,
            block_ctx_map: shared.block_ctx_map,
            raw_quant_field: shared.raw_quant_field,
            quant_dc: shared.quant_dc,
        };
        Self::init_tables(
            frame_header,
            readers,
            num_passes,
            histo_selector_bits,
            rect,
            num_nzeroes,
            tables,
            code,
            context_map,
            first_pass,
        )
    }
    #[allow(clippy::too_many_arguments)] // Preserve the source initialization inputs.
    fn init_tables(
        frame_header: &'a super::frame_header::FrameHeader<'_>,
        readers: &'r mut [BitReader<'input>],
        num_passes: usize,
        histo_selector_bits: usize,
        rect: super::base::rect::Rect,
        num_nzeroes: &'r mut [super::image::Image3<i32>; super::common::MAX_NUM_PASSES],
        shared: BlockTables<'a>,
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
            block_ctx_map: shared.block_ctx_map,
            qf: shared.raw_quant_field,
            quant_dc: shared.quant_dc,
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
    /// PassesDecoderState::upsampler8x; only the zero-pass first draw uses it.
    pub upsampler8x: Option<
        &'a mut (dyn super::render_pipeline::render_pipeline_stage::RenderPipelineStage + 'static),
    >,
}

/// AcGroupDecoderState with the shared state as its `Sync` view.
pub struct AcGroupDecoderView<'a, 'v> {
    pub shared: &'a GroupDecoderShared<'v>,
    pub code: &'a [super::dec_ans::ANSCode],
    pub context_map: &'a [Vec<u8>],
    pub coefficients: &'a mut super::dct_util::AcImage,
    pub x_dm_multiplier: f32,
    pub b_dm_multiplier: f32,
    pub quant_biases: &'a [f32; 4],
    /// PassesDecoderState::upsampler8x; only the zero-pass first draw uses it.
    pub upsampler8x: Option<
        &'a mut (dyn super::render_pipeline::render_pipeline_stage::RenderPipelineStage + 'static),
    >,
}

/// Complete DecodeGroup, with optional original JPEG output. Zero new passes
/// with a draw and no earlier pass render the 8x-upsampled DC image (Flush).
/// Otherwise the caller supplies `num_passes` genuine AC readers (zero for a
/// forced draw of stored coefficients) and an initialized cache. `thread` is
/// the storage location the source passes to the upsampler.
#[allow(clippy::too_many_arguments)] // Preserve the reached source arguments.
pub fn decode_ac_group(
    frame_header: &super::frame_header::FrameHeader<'_>,
    readers: &mut [BitReader<'_>],
    num_passes: usize,
    group_idx: usize,
    state: AcGroupDecoderState<'_, '_>,
    group_dec_cache: &mut super::dec_cache::GroupDecCache,
    thread: usize,
    render_pipeline_input: &mut super::render_pipeline::simple_render_pipeline::RenderPipelineInput<
        '_,
        '_,
    >,
    jpeg_data: Option<&mut super::jpeg::jpeg_data::JpegData>,
    first_pass: usize,
    force_draw: bool,
    dc_only: bool,
    should_run_pipeline: Option<&mut bool>,
) -> Status {
    with_group_shared(state.shared, |shared| {
        let view = AcGroupDecoderView {
            shared,
            code: state.code,
            context_map: state.context_map,
            coefficients: state.coefficients,
            x_dm_multiplier: state.x_dm_multiplier,
            b_dm_multiplier: state.b_dm_multiplier,
            quant_biases: state.quant_biases,
            upsampler8x: state.upsampler8x,
        };
        decode_ac_group_shared(
            frame_header,
            readers,
            num_passes,
            group_idx,
            view,
            group_dec_cache,
            thread,
            render_pipeline_input,
            jpeg_data,
            first_pass,
            force_draw,
            dc_only,
            should_run_pipeline,
        )
    })
}

/// DecodeGroup through the shared view; see `decode_ac_group`.
#[allow(clippy::too_many_arguments)] // Preserve the reached source arguments.
pub fn decode_ac_group_shared(
    frame_header: &super::frame_header::FrameHeader<'_>,
    readers: &mut [BitReader<'_>],
    num_passes: usize,
    group_idx: usize,
    state: AcGroupDecoderView<'_, '_>,
    group_dec_cache: &mut super::dec_cache::GroupDecCache,
    thread: usize,
    render_pipeline_input: &mut super::render_pipeline::simple_render_pipeline::RenderPipelineInput<
        '_,
        '_,
    >,
    jpeg_data: Option<&mut super::jpeg::jpeg_data::JpegData>,
    first_pass: usize,
    force_draw: bool,
    dc_only: bool,
    should_run_pipeline: Option<&mut bool>,
) -> Status {
    let draw = if num_passes + first_pass == frame_header.passes.num_passes as usize || force_draw {
        DrawMode::Draw
    } else {
        DrawMode::DontDraw
    };
    if let Some(should_run_pipeline) = should_run_pipeline {
        *should_run_pipeline = draw != DrawMode::DontDraw;
    }
    if draw == DrawMode::Draw && num_passes == 0 && first_pass == 0 {
        return draw_upsampled_dc(
            frame_header,
            group_idx,
            state,
            group_dec_cache,
            thread,
            render_pipeline_input,
        );
    }
    let mut histo_selector_bits = 0;
    if dc_only {
        if num_passes != 0 {
            return Err(StatusCode::GenericError);
        }
    } else {
        if state.shared.num_histograms == 0 {
            return Err(StatusCode::GenericError);
        }
        histo_selector_bits = if state.shared.num_histograms == 1 {
            0
        } else {
            usize::BITS as usize - (state.shared.num_histograms - 1).leading_zeros() as usize
        };
    }
    let (buffers, num_nzeroes) = group_dec_cache.split_buffers();
    let mut get_block = GetBlockFromBitstream::init_shared(
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
    super::dec_group_render::decode_group_impl_shared(
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

/// DecodeGroup's zero-pass first-draw branch: copy the group's DC rectangle with
/// two padding samples into the cache buffer, mirror-pad it and run the 8x
/// upsampler into each color input buffer. Owner-relative rows replace the
/// source's dc_buffer and pipeline row pointers, including its minus-offset rows.
fn draw_upsampled_dc(
    frame_header: &super::frame_header::FrameHeader<'_>,
    group_idx: usize,
    state: AcGroupDecoderView<'_, '_>,
    group_dec_cache: &mut super::dec_cache::GroupDecCache,
    thread: usize,
    render_pipeline_input: &mut super::render_pipeline::simple_render_pipeline::RenderPipelineInput<
        '_,
        '_,
    >,
) -> Status {
    use super::base::rect::Rect;
    use super::render_pipeline::render_pipeline_stage::{
        RENDER_PIPELINE_X_OFFSET, Row, RowBuffers, RowStorage,
    };
    group_dec_cache.init_dc_buffer_once_in(state.shared.memory_manager())?;
    let upsampler = state
        .upsampler8x
        .expect("source Init creates the 8x upsampler before any group draw");
    let dc = state.shared.dc();
    let cs = &frame_header.chroma_subsampling;
    for c in [0, 1, 2] {
        let hs = cs.h_shift(c);
        let vs = cs.v_shift(c);
        // The source reuses the cache's filter input storage here.
        let src_rect_precs = state.shared.frame_dim.block_group_rect(group_idx);
        let src_rect = Rect::new(
            src_rect_precs.x0() >> hs,
            src_rect_precs.y0() >> vs,
            src_rect_precs.xsize() >> hs,
            src_rect_precs.ysize() >> vs,
        );
        let copy_rect = Rect::new(
            RENDER_PIPELINE_X_OFFSET,
            2,
            src_rect.xsize(),
            src_rect.ysize(),
        );
        super::image_ops::copy_image_to_with_padding(
            src_rect,
            dc.plane(c),
            2,
            copy_rect,
            &mut group_dec_cache.dc_buffer,
        )?;
        // Mirror padding. Interleaving left and right padding handles DC size 1.
        for y in 0..src_rect.ysize().wrapping_add(4) {
            let xend = RENDER_PIPELINE_X_OFFSET
                .wrapping_add(dc.plane(c).xsize() >> hs)
                .wrapping_sub(src_rect.x0());
            for ix in 0..2 {
                let row = group_dec_cache.dc_buffer.row_mut(y);
                if src_rect.x0() == 0 {
                    row[RENDER_PIPELINE_X_OFFSET - ix - 1] = row[RENDER_PIPELINE_X_OFFSET + ix];
                }
                if src_rect.x0() + src_rect.xsize() + 2 >= (dc.xsize() >> hs) {
                    row[xend + ix] = row[xend - ix - 1];
                }
            }
        }
        let (upsampling_dst, dst_rect) = render_pipeline_input.get_buffer_mut(c);
        if !dst_rect.is_inside(Rect::new(
            0,
            0,
            upsampling_dst.xsize(),
            upsampling_dst.ysize(),
        )) {
            return Err(StatusCode::GenericError);
        }
        let dc_ysize = (dc.plane(c).ysize() >> vs) as i64;
        for y in src_rect.y0()..src_rect.y0() + src_rect.ysize() {
            let input_rows = vec![
                (0..5i64)
                    .map(|iy| Row {
                        storage: RowStorage::Input,
                        channel: 0,
                        y: (super::image_ops::mirror(y as i64 + iy - 2, dc_ysize) as usize)
                            .wrapping_add(2)
                            .wrapping_sub(src_rect.y0()),
                        x: 0,
                    })
                    .collect(),
            ];
            let output_rows = vec![
                (0..8)
                    .map(|iy| Row {
                        storage: RowStorage::Output,
                        channel: 0,
                        y: dst_rect.y0() + ((y - src_rect.y0()) << 3) + iy,
                        x: dst_rect.x0().wrapping_sub(RENDER_PIPELINE_X_OFFSET),
                    })
                    .collect(),
            ];
            let mut buffers = RowBuffers {
                input: std::slice::from_mut(&mut group_dec_cache.dc_buffer),
                output: std::slice::from_mut(&mut *upsampling_dst),
            };
            // Arguments set to zero are not used by the upsampler.
            upsampler.process_row(
                &input_rows,
                &output_rows,
                &mut buffers,
                0,
                0,
                src_rect.xsize(),
                0,
                0,
                thread,
            )?;
        }
    }
    Ok(())
}

/// GetBlockFromEncoder from dec_group.cc: the encoder's quantized AC images,
/// one per pass, as the coefficient source of the decoder in the loop. Row
/// slices replace the cached `rows[i][c]` pointers; `offset` advances over the
/// group row exactly as the source member does.
pub struct GetBlockFromEncoder<'a> {
    rows: Vec<[&'a [i32]; 3]>,
    offset: usize,
    shift_for_pass: &'a [u32],
}
impl<'a> GetBlockFromEncoder<'a> {
    /// The source Create(ac, group_idx, shift_for_pass): every pass image
    /// must hold 32-bit coefficients, as its JXL_ENSURE requires.
    pub fn create(
        ac: &'a [Option<Box<super::dct_util::AcImage>>],
        group_idx: usize,
        shift_for_pass: &'a [u32],
    ) -> super::base::status::StatusOr<Self> {
        let mut rows = Vec::with_capacity(ac.len());
        for image in ac {
            let image = image
                .as_deref()
                .expect("source passes a non-null ACImage per pass");
            if image.image_type() != super::dct_util::AcType::I32 {
                return Err(StatusCode::GenericError);
            }
            rows.push(std::array::from_fn(|c| {
                match image.plane_row(c, group_idx, 0) {
                    super::dct_util::ConstAcPtr::I32(row) => row,
                    super::dct_util::ConstAcPtr::I16(_) => unreachable!("type checked above"),
                }
            }));
        }
        Ok(Self {
            rows,
            offset: 0,
            shift_for_pass,
        })
    }
}
impl GetBlock for GetBlockFromEncoder<'_> {
    fn start_row(&mut self, _by: usize) {}
    fn load_block(
        &mut self,
        _bx: usize,
        _by: usize,
        _acs: AcStrategy,
        size: usize,
        _log2_covered_blocks: usize,
        block: &mut [AcPtrMut<'_>; 3],
    ) -> Status {
        // JXL_ENSURE(ac_type == ACType::k32): the traversal passes the decoder
        // state's coefficient type as the block variant.
        for c in 0..3 {
            let AcPtrMut::I32(block) = &mut block[c] else {
                return Err(StatusCode::GenericError);
            };
            for (i, rows) in self.rows.iter().enumerate() {
                for k in 0..size {
                    // The source's int32 product and sum; Clang's two's
                    // complement lowering on the defined (non-overflowing)
                    // domain the encoder's quantized values stay in.
                    block[k] = block[k].wrapping_add(
                        rows[c][self.offset + k].wrapping_mul(1i32 << self.shift_for_pass[i]),
                    );
                }
            }
        }
        self.offset += size;
        Ok(())
    }
}

/// DecodeGroupForRoundtrip from dec_group.cc: decode one group of the
/// encoder's own quantized coefficients into the prepared render-pipeline
/// input, always drawing. The group cache is initialized for zero passes and
/// every valid strategy, as in the source. Its unused `aux_out` argument is
/// omitted; `state` supplies the PassesDecoderState fields DecodeGroupImpl
/// reads (its ANS code and context map are not read on this path).
#[allow(clippy::too_many_arguments)] // Preserve the reached source arguments.
pub fn decode_group_for_roundtrip(
    frame_header: &super::frame_header::FrameHeader<'_>,
    ac: &[Option<Box<super::dct_util::AcImage>>],
    group_idx: usize,
    state: AcGroupDecoderState<'_, '_>,
    group_dec_cache: &mut super::dec_cache::GroupDecCache,
    render_pipeline_input: &mut super::render_pipeline::simple_render_pipeline::RenderPipelineInput<
        '_,
        '_,
    >,
    jpeg_data: Option<&mut super::jpeg::jpeg_data::JpegData>,
) -> Status {
    let memory_manager = state.shared.memory_manager();
    let mut get_block = GetBlockFromEncoder::create(ac, group_idx, &frame_header.passes.shift[..])?;
    group_dec_cache.init_once_in(
        memory_manager,
        0,
        (1usize << super::ac_strategy::NUM_VALID_STRATEGIES) - 1,
    )?;
    let (buffers, _) = group_dec_cache.split_buffers();
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
        DrawMode::Draw,
    )
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
        num_passes.get(),
        group_idx,
        state,
        group_dec_cache,
        0,
        render_pipeline_input,
        None,
        first_pass,
        force_draw,
        false,
        should_run_pipeline,
    )
}
