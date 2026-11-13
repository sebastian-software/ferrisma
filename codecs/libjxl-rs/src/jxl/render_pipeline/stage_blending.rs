// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar BlendingStage/factory from render_pipeline/stage_blending.{h,cc}.
//! The immutable header is borrowed or shared. Reference slots retain their
//! existing owners; each row borrows them only for that call. PerformBlending's
//! temporary plane separates foreground reads from its in-place output writes.
//! HWY_DYNAMIC_DISPATCH(GetBlendingStage) selects a per-target copy of this
//! stage, but its body has no Highway operation: it only clips the span and
//! calls the non-Highway PerformBlending (blending.cc, alpha.cc). Every target
//! therefore uses this one stage (port ADR 0004; replayed against the NEON
//! oracle in tests/simd_stages2.rs).
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::blending::perform_blending_in;
use crate::jxl::color_encoding_internal::ColorEncoding;
use crate::jxl::dec_cache::PassesDecoderState;
use crate::jxl::dec_patch_dictionary::{PatchBlendMode, PatchBlending, ReferenceFrame};
use crate::jxl::frame_header::{BlendMode, BlendingInfo, FrameHeader, FrameOrigin};
use crate::jxl::image_metadata::ExtraChannelInfo;
use crate::jxl::memory_manager_internal::MemoryManagerPtr;
use std::cell::RefCell;
use std::rc::Rc;

enum Header<'a, 'm> {
    Borrowed(&'a FrameHeader<'m>),
    Shared(Rc<FrameHeader<'m>>),
}
impl<'m> Header<'_, 'm> {
    fn get(&self) -> &FrameHeader<'m> {
        match self {
            Self::Borrowed(h) => h,
            Self::Shared(h) => h,
        }
    }
}
enum References<'a, 'm> {
    Borrowed(&'a [ReferenceFrame<'m>; 4]),
    Shared(Rc<RefCell<[ReferenceFrame<'m>; 4]>>),
}
impl<'m> References<'_, 'm> {
    fn with<R>(&self, f: impl FnOnce(&[ReferenceFrame<'m>; 4]) -> R) -> R {
        match self {
            Self::Borrowed(r) => f(r),
            Self::Shared(r) => f(&r.borrow()),
        }
    }
}
pub struct BlendingStage<'a, 'm> {
    memory_manager: MemoryManagerPtr,
    frame_header: Header<'a, 'm>,
    references: References<'a, 'm>,
    info: BlendingInfo,
    initialized: Status,
    image_xsize: usize,
    image_ysize: usize,
    blending_info: Vec<PatchBlending>,
    extra_channel_info: &'m [ExtraChannelInfo],
    zeroes: Vec<f32>,
}
impl<'a, 'm> BlendingStage<'a, 'm> {
    fn new(
        memory_manager: MemoryManagerPtr,
        header: Header<'a, 'm>,
        references: References<'a, 'm>,
        metadata_xyb: bool,
        color_encoding_is_original: bool,
        _frame_color_encoding: &ColorEncoding,
    ) -> Self {
        let h = header.get();
        let metadata = h
            .nonserialized_metadata
            .expect("source requires frame metadata");
        let image_xsize = metadata.xsize();
        let image_ysize = metadata.ysize();
        let info = h.blending_info.clone();
        let mut result = Self {
            memory_manager,
            frame_header: header,
            references,
            info,
            initialized: Ok(()),
            image_xsize,
            image_ysize,
            blending_info: Vec::new(),
            extra_channel_info: &metadata.m.extra_channel_info,
            zeroes: Vec::new(),
        };
        let initialized = result.references.with(|refs| {
            let bg = refs[result.info.source as usize]
                .frame
                .as_deref()
                .expect("source reference frame pointer must be non-null");
            if bg.xsize() == 0 || bg.ysize() == 0 {
                result.zeroes.resize(image_xsize, 0.0);
            } else if refs[result.info.source as usize].ib_is_in_xyb {
                return Err(StatusCode::GenericError);
            } else if result
                .frame_header
                .get()
                .extra_channel_blending_info
                .iter()
                .any(|info| {
                    let bg = refs[info.source as usize].frame.as_deref().unwrap();
                    bg.xsize() == 0 || bg.ysize() == 0
                })
            {
                result.zeroes.resize(image_xsize, 0.0);
            }
            let verify = |bg: &crate::jxl::image_bundle::ImageBundle<'_>| {
                if bg.xsize() != 0
                    && bg.ysize() != 0
                    && (bg.xsize() < image_xsize
                        || bg.ysize() < image_ysize
                        || bg.origin.x0 != 0
                        || bg.origin.y0 != 0)
                {
                    Err(StatusCode::GenericError)
                } else {
                    Ok(())
                }
            };
            let mut ok = verify(bg);
            for info in &result.frame_header.get().extra_channel_blending_info {
                if ok.is_ok() {
                    ok = verify(refs[info.source as usize].frame.as_deref().unwrap());
                }
            }
            ok?;
            if metadata_xyb && !color_encoding_is_original {
                return Err(StatusCode::GenericError);
            }
            Ok(())
        });
        result.initialized = initialized;
        if result.initialized.is_err() {
            return result;
        }
        result.blending_info.resize(
            result.frame_header.get().extra_channel_blending_info.len() + 1,
            PatchBlending::default(),
        );
        fn make(info: &BlendingInfo, pb: &mut PatchBlending) {
            pb.alpha_channel = info.alpha_channel;
            pb.clamp = info.clamp;
            match info.mode {
                BlendMode::REPLACE => pb.mode = PatchBlendMode::REPLACE,
                BlendMode::ADD => pb.mode = PatchBlendMode::ADD,
                BlendMode::MUL => pb.mode = PatchBlendMode::MUL,
                BlendMode::BLEND => pb.mode = PatchBlendMode::BLEND_ABOVE,
                BlendMode::ALPHA_WEIGHTED_ADD => pb.mode = PatchBlendMode::ALPHA_WEIGHTED_ADD_ABOVE,
                _ => {}
            }
        }
        make(&result.info, &mut result.blending_info[0]);
        for (info, pb) in result
            .frame_header
            .get()
            .extra_channel_blending_info
            .iter()
            .zip(&mut result.blending_info[1..])
        {
            make(info, pb);
        }
        result
    }
    /// Initialized private state exposed for the independent source observer.
    pub fn constructor_sizes(&self) -> [usize; 4] {
        [
            self.image_xsize,
            self.image_ysize,
            self.zeroes.len(),
            self.blending_info.len(),
        ]
    }
    pub fn blendings(&self) -> &[PatchBlending] {
        &self.blending_info
    }
}
impl RenderPipelineStage for BlendingStage<'_, '_> {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn is_initialized(&self) -> Status {
        self.initialized
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        mut width: usize,
        x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        self.initialized?;
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let origin = self.frame_header.get().frame_origin;
        let mut bg_x = origin.x0 as isize + x as isize;
        let bg_y = origin.y0 as isize + y as isize;
        let mut offset = 0i32;
        if bg_x + width as isize <= 0
            || origin.x0 as isize >= self.image_xsize as isize
            || bg_y < 0
            || bg_y >= self.image_ysize as isize
        {
            return Ok(());
        }
        if bg_x < 0 {
            offset = (offset as isize - bg_x) as i32;
            width = width.wrapping_add_signed(bg_x);
            bg_x = 0;
        }
        if (bg_x as usize).wrapping_add(width) > self.image_xsize {
            width = (self.image_xsize as isize - bg_x).max(0) as usize;
        }
        let num_c = input.len().min(self.extra_channel_info.len() + 3);
        let rows: Vec<_> = (0..num_c)
            .map(|c| self.get_input_row(input, c, 0).offset(offset as isize))
            .collect();
        let tmp = self.references.with(|refs| {
            let bg = refs[self.info.source as usize].frame.as_deref().unwrap();
            let mut background = Vec::with_capacity(num_c);
            for c in 0..num_c {
                let row = if c < 3 {
                    if bg.xsize() != 0 && bg.ysize() != 0 {
                        &bg.color().plane_row(c, bg_y as usize)[bg_x as usize..]
                    } else {
                        &self.zeroes
                    }
                } else {
                    let ec_bg = refs[self.frame_header.get().extra_channel_blending_info[c - 3]
                        .source as usize]
                        .frame
                        .as_deref()
                        .unwrap();
                    if ec_bg.xsize() != 0 && ec_bg.ysize() != 0 {
                        &ec_bg.extra_channels()[c - 3].row(bg_y as usize)[bg_x as usize..]
                    } else {
                        &self.zeroes
                    }
                };
                background.push(row);
            }
            let foreground: Vec<_> = rows.iter().map(|&r| buffers.row(r)).collect();
            perform_blending_in(
                self.memory_manager,
                &background,
                &foreground,
                0,
                width,
                &self.blending_info[0],
                &self.blending_info[1..],
                self.extra_channel_info,
            )
        })?;
        for (c, row) in rows.into_iter().enumerate() {
            buffers.row_mut(row)[..width].copy_from_slice(&tmp.row(c)[..width]);
        }
        Ok(())
    }
    fn get_channel_mode(&self, _c: usize) -> Mode {
        Mode::InPlace
    }
    fn switch_to_image_dimensions(&self) -> bool {
        true
    }
    fn get_image_dimensions(&self, x: &mut usize, y: &mut usize, origin: &mut FrameOrigin) {
        *x = self.image_xsize;
        *y = self.image_ysize;
        *origin = self.frame_header.get().frame_origin;
    }
    fn process_padding_row(
        &mut self,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        width: usize,
        x: usize,
        y: usize,
    ) {
        self.references.with(|refs| {
            let bg = refs[self.info.source as usize].frame.as_deref().unwrap();
            for c in 0..3 {
                let row = self.get_input_row(output, c, 0);
                let dest = &mut buffers.row_mut(row)[..width];
                if bg.xsize() == 0 || bg.ysize() == 0 {
                    dest.fill(0.0);
                } else {
                    dest.copy_from_slice(&bg.color().plane_row(c, y)[x..x + width]);
                }
            }
            for ec in 0..self.extra_channel_info.len() {
                let bg = refs
                    [self.frame_header.get().extra_channel_blending_info[ec].source as usize]
                    .frame
                    .as_deref()
                    .unwrap();
                let row = self.get_input_row(output, 3 + ec, 0);
                let dest = &mut buffers.row_mut(row)[..width];
                if bg.xsize() == 0 || bg.ysize() == 0 {
                    dest.fill(0.0);
                } else {
                    dest.copy_from_slice(&bg.extra_channels()[ec].row(y)[x..x + width]);
                }
            }
        });
    }
    fn get_name(&self) -> &str {
        "Blending"
    }
}
pub fn get_blending_stage<'a, 'm: 'a>(
    header: &'a FrameHeader<'m>,
    state: &PassesDecoderState<'m, '_>,
    frame_color_encoding: &ColorEncoding,
) -> Box<BlendingStage<'a, 'm>> {
    Box::new(BlendingStage::new(
        state.shared().memory_manager(),
        Header::Borrowed(header),
        References::Shared(state.shared().reference_frames.clone()),
        state.shared().metadata.unwrap().m.xyb_encoded,
        state.output_encoding_info.color_encoding_is_original,
        frame_color_encoding,
    ))
}
pub fn get_blending_stage_borrowed<'a, 'm: 'a>(
    header: &'a FrameHeader<'m>,
    references: &'a [ReferenceFrame<'m>; 4],
    metadata_xyb: bool,
    color_encoding_is_original: bool,
    frame_color_encoding: &ColorEncoding,
) -> Box<BlendingStage<'a, 'm>> {
    Box::new(BlendingStage::new(
        MemoryManagerPtr::NATIVE,
        Header::Borrowed(header),
        References::Borrowed(references),
        metadata_xyb,
        color_encoding_is_original,
        frame_color_encoding,
    ))
}
pub fn get_blending_stage_shared<'m>(
    header: Rc<FrameHeader<'m>>,
    state: &PassesDecoderState<'m, '_>,
    frame_color_encoding: &ColorEncoding,
) -> Box<BlendingStage<'m, 'm>> {
    Box::new(BlendingStage::new(
        state.shared().memory_manager(),
        Header::Shared(header),
        References::Shared(state.shared().reference_frames.clone()),
        state.shared().metadata.unwrap().m.xyb_encoded,
        state.output_encoding_info.color_encoding_is_original,
        frame_color_encoding,
    ))
}
