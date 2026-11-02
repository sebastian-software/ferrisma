// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Shared storage/constructors from lib/jxl/passes_state.h. Pointer bindings to
//! containing members use owner-relative selectors; matrix operations receive
//! the existing same-owner borrow. InitializePassesSharedState retains complete
//! allocation/assignment order and owner-relative DC-frame bindings.
use super::ac_context::BlockCtxMap;
use super::ac_strategy::AcStrategyImage;
use super::base::status::{Status, StatusCode};
use super::chroma_from_luma::ColorCorrelationMap;
use super::coeff_order::COEFF_ORDER_MAX_SIZE;
use super::coeff_order_fwd::CoeffOrder;
use super::dec_patch_dictionary::{PatchDictionary, ReferenceFrame};
use super::frame_dimensions::FrameDimensions;
use super::frame_header::{FLAG_USE_DC_FRAME, FrameEncoding, FrameHeader};
use super::image::{Image3, Plane};
use super::image_bundle::ImageBundle;
use super::image_metadata::CodecMetadata;
use super::image_ops::zero_fill_image;
use super::noise::NoiseParams;
use super::quant_weights::DequantMatrices;
use super::quantizer::Quantizer;
use super::splines::Splines;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Default)]
pub struct ImageFeatures<'a> {
    pub noise_params: Rc<RefCell<NoiseParams>>,
    pub patches: Rc<RefCell<PatchDictionary>>,
    pub splines: Rc<RefCell<Splines<'a>>>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DcSource {
    Storage,
    Frame(usize),
}
pub struct PassesSharedState<'a> {
    memory_manager_present: bool,
    pub metadata: Option<&'a CodecMetadata>,
    pub frame_dim: FrameDimensions,
    pub ac_strategy: AcStrategyImage,
    pub matrices: DequantMatrices,
    pub quantizer: Quantizer,
    pub raw_quant_field: Plane<i32>,
    pub epf_sharpness: Plane<u8>,
    pub cmap: ColorCorrelationMap,
    pub image_features: ImageFeatures<'a>,
    pub coeff_order_size: usize,
    pub coeff_orders: Vec<CoeffOrder>,
    pub quant_dc: Plane<u8>,
    pub dc_storage: Image3<f32>,
    pub dc_source: DcSource,
    pub block_ctx_map: BlockCtxMap,
    pub dc_frames: [Image3<f32>; 4],
    pub reference_frames: Rc<RefCell<[ReferenceFrame<'a>; 4]>>,
    pub num_histograms: usize,
}
impl Default for PassesSharedState<'_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> PassesSharedState<'a> {
    pub fn new() -> Self {
        let memory_manager_present = true;
        let metadata = None;
        let frame_dim = FrameDimensions::default();
        let ac_strategy = AcStrategyImage::default();
        let matrices = DequantMatrices::default();
        let quantizer = Quantizer::new(&matrices);
        let raw_quant_field = Plane::default();
        let epf_sharpness = Plane::default();
        let cmap = ColorCorrelationMap::default();
        let image_features = ImageFeatures::default();
        let coeff_order_size = 0;
        let coeff_orders = Vec::new();
        let quant_dc = Plane::default();
        let dc_storage = Image3::default();
        let dc_source = DcSource::Storage;
        let block_ctx_map = BlockCtxMap::default();
        let dc_frames = std::array::from_fn(|_| Image3::default());
        let mut reference_frames = std::array::from_fn(|_| ReferenceFrame::default());
        let num_histograms = 0;
        for reference in &mut reference_frames {
            reference.frame = Some(Box::new(ImageBundle::new(None)));
        }
        Self {
            memory_manager_present,
            metadata,
            frame_dim,
            ac_strategy,
            matrices,
            quantizer,
            raw_quant_field,
            epf_sharpness,
            cmap,
            image_features,
            coeff_order_size,
            coeff_orders,
            quant_dc,
            dc_storage,
            dc_source,
            block_ctx_map,
            dc_frames,
            reference_frames: Rc::new(RefCell::new(reference_frames)),
            num_histograms,
        }
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager_present
    }
    pub fn dc(&self) -> &Image3<f32> {
        match self.dc_source {
            DcSource::Storage => &self.dc_storage,
            DcSource::Frame(i) => &self.dc_frames[i],
        }
    }
    pub fn bind_own_reference_frames(&mut self) {
        self.image_features
            .patches
            .borrow_mut()
            .bind_own_reference_frames();
    }
}

pub fn initialize_passes_shared_state<'a>(
    header: &FrameHeader<'a>,
    shared: &mut PassesSharedState<'a>,
    encoder: bool,
) -> Status {
    let metadata = header
        .nonserialized_metadata
        .ok_or(StatusCode::GenericError)?;
    shared.metadata = Some(metadata);
    shared.frame_dim = header.to_frame_dimensions();
    shared.bind_own_reference_frames();
    let dim = &shared.frame_dim;
    shared.ac_strategy = AcStrategyImage::create(dim.xsize_blocks, dim.ysize_blocks)?;
    shared.raw_quant_field = Plane::create(dim.xsize_blocks, dim.ysize_blocks)?;
    shared.epf_sharpness = Plane::create(dim.xsize_blocks, dim.ysize_blocks)?;
    // The original call uses ColorCorrelationMap::Create's default xyb=true.
    shared.cmap = ColorCorrelationMap::create(dim.xsize, dim.ysize, true)?;
    shared.coeff_order_size = COEFF_ORDER_MAX_SIZE;
    if encoder
        && shared.coeff_orders.len()
            < (header.passes.num_passes as usize).wrapping_mul(COEFF_ORDER_MAX_SIZE)
        && header.encoding == FrameEncoding::VAR_DCT
    {
        shared.coeff_orders.resize(
            (header.passes.num_passes as usize).wrapping_mul(COEFF_ORDER_MAX_SIZE),
            0,
        );
    }
    shared.quant_dc = Plane::create(dim.xsize_blocks, dim.ysize_blocks)?;
    let use_dc_frame = header.flags & FLAG_USE_DC_FRAME != 0;
    if !encoder && use_dc_frame {
        if header.dc_level == 4 {
            return Err(StatusCode::GenericError);
        }
        shared.dc_storage = Image3::default();
        shared.dc_source = DcSource::Frame(header.dc_level as usize);
        if shared.dc().xsize() == 0 {
            return Err(StatusCode::GenericError);
        }
        zero_fill_image(&mut shared.quant_dc);
    } else {
        shared.dc_storage = Image3::create(dim.xsize_blocks, dim.ysize_blocks)?;
        shared.dc_source = DcSource::Storage;
    }
    Ok(())
}
