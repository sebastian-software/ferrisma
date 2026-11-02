// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete lib/jxl/render_pipeline/stage_patches.{h,cc}. Source pointers
//! become borrows of the dictionary, its shared references, and metadata.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::Status;
use crate::jxl::dec_patch_dictionary::{PatchDictionary, ReferenceFrame};
use crate::jxl::image_metadata::ExtraChannelInfo;
use std::cell::RefCell;
use std::rc::Rc;
enum PatchSources<'a, 'm> {
    Borrowed {
        patches: &'a PatchDictionary,
        references: &'a [ReferenceFrame<'m>; 4],
    },
    Shared {
        patches: Rc<RefCell<PatchDictionary>>,
        references: Rc<RefCell<[ReferenceFrame<'m>; 4]>>,
    },
}
pub struct PatchDictionaryStage<'a, 'm> {
    sources: PatchSources<'a, 'm>,
    extra_channel_info: &'a [ExtraChannelInfo],
}
impl RenderPipelineStage for PatchDictionaryStage<'_, '_> {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        let rows: Vec<_> = (0..3 + self.extra_channel_info.len())
            .map(|c| self.get_input_row(input, c, 0).offset(-(left as isize)))
            .collect();
        let add = |patches: &PatchDictionary,
                   references: &[ReferenceFrame<'_>; 4],
                   buffers: &mut RowBuffers<'_>| {
            patches.add_one_row(
                &rows,
                buffers,
                y,
                x.wrapping_sub(left),
                width.wrapping_add(left).wrapping_add(right),
                self.extra_channel_info,
                references,
            )
        };
        match &self.sources {
            PatchSources::Borrowed {
                patches,
                references,
            } => add(patches, references, buffers),
            // A sequential source call observes the current shared owners.
            // Neither borrow escapes this call or spans an external callback.
            // The decoder may replace frames or decode a new dictionary after
            // return; cloning these Rc handles never clones their image data.
            PatchSources::Shared {
                patches,
                references,
            } => add(&patches.borrow(), &references.borrow(), buffers),
        }
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 + self.extra_channel_info.len() {
            Mode::InPlace
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "Patches"
    }
}
pub fn get_patches_stage<'a, 'm>(
    patches: &'a PatchDictionary,
    references: &'a [ReferenceFrame<'m>; 4],
    extra_channel_info: &'a [ExtraChannelInfo],
) -> Box<dyn RenderPipelineStage + 'a> {
    Box::new(PatchDictionaryStage {
        sources: PatchSources::Borrowed {
            patches,
            references,
        },
        extra_channel_info,
    })
}

/// Source pipeline pointer binding when pipeline and resources share a decoder
/// owner. Handles retain identity; all dictionary/reference reads happen under
/// local immutable borrows, so later frame replacement remains visible.
pub fn get_patches_stage_shared<'a, 'm: 'a>(
    patches: Rc<RefCell<PatchDictionary>>,
    references: Rc<RefCell<[ReferenceFrame<'m>; 4]>>,
    extra_channel_info: &'a [ExtraChannelInfo],
) -> Box<dyn RenderPipelineStage + 'a> {
    Box::new(PatchDictionaryStage {
        sources: PatchSources::Shared {
            patches,
            references,
        },
        extra_channel_info,
    })
}
