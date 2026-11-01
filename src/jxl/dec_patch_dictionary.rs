// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Patch/reference storage, constructor, HasAny, and the same-owner SetShared
//! binding from lib/jxl/dec_patch_dictionary.h. Decoding, tree construction,
//! clearing, blending, and rendering remain unported. The containing shared
//! state's array pointer is an owner-relative member binding, not a self-reference.
use super::image_bundle::ImageBundle;

#[derive(Default)]
pub struct ReferenceFrame<'a> {
    pub frame: Option<Box<ImageBundle<'a>>>,
    pub ib_is_in_xyb: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PatchBlendMode(pub u8);
impl PatchBlendMode {
    pub const NONE: Self = Self(0);
    pub const REPLACE: Self = Self(1);
    pub const ADD: Self = Self(2);
    pub const MUL: Self = Self(3);
    pub const BLEND_ABOVE: Self = Self(4);
    pub const BLEND_BELOW: Self = Self(5);
    pub const ALPHA_WEIGHTED_ADD_ABOVE: Self = Self(6);
    pub const ALPHA_WEIGHTED_ADD_BELOW: Self = Self(7);
}
pub struct PatchBlending {
    pub mode: PatchBlendMode,
    pub alpha_channel: u32,
    pub clamp: bool,
}
pub struct PatchReferencePosition {
    pub reference: usize,
    pub x0: usize,
    pub y0: usize,
    pub xsize: usize,
    pub ysize: usize,
}
pub struct PatchPosition {
    pub x: usize,
    pub y: usize,
    pub ref_pos_idx: usize,
}
pub struct PatchTreeNode {
    pub left_child: isize,
    pub right_child: isize,
    pub y_center: usize,
    pub start: usize,
    pub num: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceFrameBinding {
    SharedState,
}
pub struct PatchDictionary {
    memory_manager_present: bool,
    reference_frames: Option<ReferenceFrameBinding>,
    positions: Vec<PatchPosition>,
    ref_positions: Vec<PatchReferencePosition>,
    blendings: Vec<PatchBlending>,
    blendings_stride: Option<usize>,
    patch_tree: Vec<PatchTreeNode>,
    num_patches: Vec<usize>,
    sorted_patches_y0: Vec<(usize, usize)>,
    sorted_patches_y1: Vec<(usize, usize)>,
}
impl Default for PatchDictionary {
    fn default() -> Self {
        Self::new()
    }
}
impl PatchDictionary {
    pub fn new() -> Self {
        Self {
            memory_manager_present: true,
            reference_frames: None,
            positions: Vec::new(),
            ref_positions: Vec::new(),
            blendings: Vec::new(),
            blendings_stride: None,
            patch_tree: Vec::new(),
            num_patches: Vec::new(),
            sorted_patches_y0: Vec::new(),
            sorted_patches_y1: Vec::new(),
        }
    }
    pub fn bind_own_reference_frames(&mut self) {
        self.reference_frames = Some(ReferenceFrameBinding::SharedState);
    }
    pub fn has_any(&self) -> bool {
        !self.positions.is_empty()
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager_present
    }
    pub fn reference_frame_binding(&self) -> Option<ReferenceFrameBinding> {
        self.reference_frames
    }
    pub fn blendings_stride(&self) -> Option<usize> {
        self.blendings_stride
    }
    pub fn storage_sizes(&self) -> [usize; 7] {
        [
            self.positions.len(),
            self.ref_positions.len(),
            self.blendings.len(),
            self.patch_tree.len(),
            self.num_patches.len(),
            self.sorted_patches_y0.len(),
            self.sorted_patches_y1.len(),
        ]
    }
}
