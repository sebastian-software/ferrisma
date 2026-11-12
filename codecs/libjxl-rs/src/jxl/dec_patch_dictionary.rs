// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Complete patch dictionary from lib/jxl/dec_patch_dictionary.{h,cc}.
//! The containing shared state retains an owner-relative reference binding;
//! each decoding/rendering call borrows its actual reference-frame owner.
use super::image_bundle::ImageBundle;

#[derive(Default)]
pub struct ReferenceFrame<'a> {
    pub frame: Option<Box<ImageBundle<'a>>>,
    pub ib_is_in_xyb: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PatchBlendMode(pub u8);
impl Default for PatchBlendMode {
    fn default() -> Self {
        Self::NONE
    }
}
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
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchBlending {
    pub mode: PatchBlendMode,
    pub alpha_channel: u32,
    pub clamp: bool,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchReferencePosition {
    pub reference: usize,
    pub x0: usize,
    pub y0: usize,
    pub xsize: usize,
    pub ysize: usize,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchPosition {
    pub x: usize,
    pub y: usize,
    pub ref_pos_idx: usize,
}
#[derive(Clone, Copy, Debug, Default)]
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
    memory_manager: MemoryManagerPtr,
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
        Self::new_in(MemoryManagerPtr::NATIVE)
    }
    /// The source PatchDictionary(JxlMemoryManager*) constructor.
    pub fn new_in(memory_manager: MemoryManagerPtr) -> Self {
        Self {
            memory_manager,
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
    pub fn clear(&mut self) {
        self.positions.clear();
        self.ref_positions.clear();
        self.blendings.clear();
        self.compute_patch_tree();
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager.is_some()
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

use super::base::status::{Status, StatusCode};
use super::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use super::dec_bit_reader::BitReader;
use super::image_metadata::ExtraChannelInfo;
use super::memory_manager_internal::MemoryManagerPtr;
use super::patch_dictionary_internal::*;
use super::render_pipeline::render_pipeline_stage::{Row, RowBuffers};

pub const NUM_PATCH_BLEND_MODES: u8 = 8;
pub fn uses_alpha(mode: PatchBlendMode) -> bool {
    matches!(
        mode,
        PatchBlendMode::BLEND_ABOVE
            | PatchBlendMode::BLEND_BELOW
            | PatchBlendMode::ALPHA_WEIGHTED_ADD_ABOVE
            | PatchBlendMode::ALPHA_WEIGHTED_ADD_BELOW
    )
}
pub fn uses_clamp(mode: PatchBlendMode) -> bool {
    uses_alpha(mode) || mode == PatchBlendMode::MUL
}

impl PatchDictionary {
    pub fn positions(&self) -> &[PatchPosition] {
        &self.positions
    }
    pub fn ref_positions(&self) -> &[PatchReferencePosition] {
        &self.ref_positions
    }
    pub fn blendings(&self) -> &[PatchBlending] {
        &self.blendings
    }
    /// Decoder and stage callers pass the same shared reference owner denoted by
    /// SetShared. No self pointer or cloned image owner is retained.
    #[expect(
        clippy::too_many_arguments,
        reason = "Retain the source Decode parameters and the reference owner"
    )]
    pub fn decode(
        &mut self,
        memory_manager: MemoryManagerPtr,
        br: &mut BitReader<'_>,
        xsize: usize,
        ysize: usize,
        num_extra_channels: usize,
        uses_extra_channels: &mut bool,
        references: &[ReferenceFrame<'_>; 4],
    ) -> Status {
        self.positions.clear();
        self.ref_positions.clear();
        self.blendings.clear();
        self.blendings_stride = Some(num_extra_channels + 1);
        let stride = num_extra_channels + 1;
        let mut context_map = Vec::new();
        let mut code = ANSCode::default();
        decode_histograms(
            memory_manager,
            br,
            NUM_PATCH_DICTIONARY_CONTEXTS,
            &mut code,
            &mut context_map,
            false,
        )?;
        let mut decoder = ANSSymbolReader::create(&code, br, 0)?;
        let mut read_num = |context| decoder.read_hybrid_uint(context, br, &context_map);
        let num_ref_patch = read_num(NUM_REF_PATCH_CONTEXT);
        let num_pixels = xsize.wrapping_mul(ysize);
        let max_ref_patches = 1024 + num_pixels / 4;
        let max_patches = max_ref_patches * 4;
        let max_blending_infos = max_patches * 4;
        if num_ref_patch > max_ref_patches {
            return Err(StatusCode::GenericError);
        }
        let mut total_patches = 0;
        let mut next_size = 1;
        for _ in 0..num_ref_patch {
            let reference = read_num(REFERENCE_FRAME_CONTEXT);
            if reference >= 4
                || references[reference]
                    .frame
                    .as_ref()
                    .map_or(0, |f| f.xsize())
                    == 0
            {
                return Err(StatusCode::GenericError);
            }
            if !references[reference].ib_is_in_xyb {
                return Err(StatusCode::GenericError);
            }
            let ib = references[reference].frame.as_ref().unwrap();
            let ref_pos = PatchReferencePosition {
                reference,
                x0: read_num(PATCH_REFERENCE_POSITION_CONTEXT),
                y0: read_num(PATCH_REFERENCE_POSITION_CONTEXT),
                xsize: read_num(PATCH_SIZE_CONTEXT) + 1,
                ysize: read_num(PATCH_SIZE_CONTEXT) + 1,
            };
            if ref_pos.x0.wrapping_add(ref_pos.xsize) > ib.xsize()
                || ref_pos.y0.wrapping_add(ref_pos.ysize) > ib.ysize()
            {
                return Err(StatusCode::GenericError);
            }
            let mut id_count = read_num(PATCH_COUNT_CONTEXT);
            if id_count > max_patches {
                return Err(StatusCode::GenericError);
            }
            id_count += 1;
            total_patches += id_count;
            if total_patches > max_patches {
                return Err(StatusCode::GenericError);
            }
            if next_size < total_patches {
                next_size *= 2;
                next_size = next_size.min(max_patches);
            }
            if next_size * stride > max_blending_infos {
                return Err(StatusCode::GenericError);
            }
            self.positions
                .reserve(next_size.saturating_sub(self.positions.len()));
            self.blendings
                .reserve((next_size * stride).saturating_sub(self.blendings.len()));
            for i in 0..id_count {
                let mut pos = PatchPosition {
                    ref_pos_idx: self.ref_positions.len(),
                    ..PatchPosition::default()
                };
                if i == 0 {
                    pos.x = read_num(PATCH_POSITION_CONTEXT);
                    pos.y = read_num(PATCH_POSITION_CONTEXT);
                } else {
                    let dx =
                        super::pack_signed::unpack_signed(read_num(PATCH_OFFSET_CONTEXT)) as isize;
                    if dx < 0 && dx.unsigned_abs() > self.positions.last().unwrap().x {
                        return Err(StatusCode::GenericError);
                    }
                    pos.x = self.positions.last().unwrap().x.wrapping_add_signed(dx);
                    let dy =
                        super::pack_signed::unpack_signed(read_num(PATCH_OFFSET_CONTEXT)) as isize;
                    if dy < 0 && dy.unsigned_abs() > self.positions.last().unwrap().y {
                        return Err(StatusCode::GenericError);
                    }
                    pos.y = self.positions.last().unwrap().y.wrapping_add_signed(dy);
                }
                if pos.x.wrapping_add(ref_pos.xsize) > xsize
                    || pos.y.wrapping_add(ref_pos.ysize) > ysize
                {
                    return Err(StatusCode::GenericError);
                }
                for j in 0..stride {
                    let blend_mode = read_num(PATCH_BLEND_MODE_CONTEXT) as u32;
                    if blend_mode >= u32::from(NUM_PATCH_BLEND_MODES) {
                        return Err(StatusCode::GenericError);
                    }
                    let mode = PatchBlendMode(blend_mode as u8);
                    if uses_alpha(mode) || (mode != PatchBlendMode::NONE && j > 0) {
                        *uses_extra_channels = true;
                    }
                    let alpha_channel = if uses_alpha(mode) && num_extra_channels > 1 {
                        let alpha = read_num(PATCH_ALPHA_CHANNEL_CONTEXT) as u32;
                        if alpha as usize >= num_extra_channels {
                            return Err(StatusCode::GenericError);
                        }
                        alpha
                    } else {
                        0
                    };
                    let clamp = uses_clamp(mode) && read_num(PATCH_CLAMP_CONTEXT) != 0;
                    self.blendings.push(PatchBlending {
                        mode,
                        alpha_channel,
                        clamp,
                    });
                }
                self.positions.push(pos);
            }
            self.ref_positions.push(ref_pos);
        }
        self.positions.shrink_to_fit();
        if !decoder.check_ans_final_state() {
            return Err(StatusCode::GenericError);
        }
        self.compute_patch_tree();
        Ok(())
    }
    pub fn patch_tree(&self) -> &[PatchTreeNode] {
        &self.patch_tree
    }
    pub fn num_patches(&self) -> &[usize] {
        &self.num_patches
    }
    pub fn sorted_patches_y0(&self) -> &[(usize, usize)] {
        &self.sorted_patches_y0
    }
    pub fn sorted_patches_y1(&self) -> &[(usize, usize)] {
        &self.sorted_patches_y1
    }
    pub fn get_references(&self) -> i32 {
        self.ref_positions
            .iter()
            .fold(0, |result, r| result | (1 << r.reference))
    }
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    fn compute_patch_tree(&mut self) {
        self.patch_tree.clear();
        self.num_patches.clear();
        self.sorted_patches_y0.clear();
        self.sorted_patches_y1.clear();
        if self.positions.is_empty() {
            return;
        }
        #[derive(Clone, Copy, Default)]
        struct Interval {
            idx: usize,
            y0: usize,
            y1: usize,
        }
        let mut intervals: Vec<_> = self
            .positions
            .iter()
            .enumerate()
            .map(|(idx, p)| Interval {
                idx,
                y0: p.y,
                y1: p.y + self.ref_positions[p.ref_pos_idx].ysize,
            })
            .collect();
        super::base::libcpp_sort::sort_by(&mut intervals, |a, b| a.y1 < b.y1);
        self.num_patches.resize(intervals.last().unwrap().y1, 0);
        for iv in &intervals {
            for y in iv.y0..iv.y1 {
                self.num_patches[y] += 1;
            }
        }
        self.patch_tree.push(PatchTreeNode {
            num: intervals.len(),
            ..PatchTreeNode::default()
        });
        let mut next = 0;
        while next < self.patch_tree.len() {
            let start = self.patch_tree[next].start;
            let end = start + self.patch_tree[next].num;
            super::base::libcpp_sort::sort_by(&mut intervals[start..end], |a, b| a.y0 < b.y0);
            let middle_idx = start + self.patch_tree[next].num / 2;
            let center = intervals[middle_idx].y0;
            let mut right_start = middle_idx;
            while right_start < end && intervals[right_start].y0 == center {
                right_start += 1;
            }
            super::base::libcpp_sort::sort_by(&mut intervals[start..right_start], |a, b| {
                a.y1 < b.y1
            });
            let mut left_end = right_start;
            while left_end > start && intervals[left_end - 1].y1 > center {
                left_end -= 1;
            }
            self.patch_tree[next] = PatchTreeNode {
                num: right_start - left_end,
                start: self.sorted_patches_y0.len(),
                y_center: center,
                left_child: -1,
                right_child: -1,
            };
            for iv in intervals[left_end..right_start].iter().rev() {
                self.sorted_patches_y1.push((iv.y1, iv.idx));
            }
            super::base::libcpp_sort::sort_by(&mut intervals[left_end..right_start], |a, b| {
                a.y0 < b.y0
            });
            for iv in &intervals[left_end..right_start] {
                self.sorted_patches_y0.push((iv.y0, iv.idx));
            }
            if left_end > start {
                self.patch_tree[next].left_child = self.patch_tree.len() as isize;
                self.patch_tree.push(PatchTreeNode {
                    start,
                    num: left_end - start,
                    ..PatchTreeNode::default()
                });
            }
            if right_start < end {
                self.patch_tree[next].right_child = self.patch_tree.len() as isize;
                self.patch_tree.push(PatchTreeNode {
                    start: right_start,
                    num: end - right_start,
                    ..PatchTreeNode::default()
                });
            }
            next += 1;
        }
    }
    pub fn get_patches_for_row(&self, y: usize) -> Vec<usize> {
        let mut result = Vec::new();
        if y < self.num_patches.len() && self.num_patches[y] > 0 {
            result.reserve(self.num_patches[y]);
            let mut tree_idx: isize = 0;
            while tree_idx != -1 {
                let node = &self.patch_tree[tree_idx as usize];
                if y <= node.y_center {
                    for &(start, idx) in &self.sorted_patches_y0[node.start..node.start + node.num]
                    {
                        if y < start {
                            break;
                        }
                        result.push(idx);
                    }
                    tree_idx = if y < node.y_center {
                        node.left_child
                    } else {
                        -1
                    };
                } else {
                    for &(end, idx) in &self.sorted_patches_y1[node.start..node.start + node.num] {
                        if y >= end {
                            break;
                        }
                        result.push(idx);
                    }
                    tree_idx = node.right_child;
                }
            }
            result.sort_unstable();
        }
        result
    }
    #[expect(
        clippy::too_many_arguments,
        reason = "Retain source row arguments with explicit storage and reference-owner borrows."
    )]
    pub fn add_one_row(
        &self,
        inout: &[Row],
        buffers: &mut RowBuffers<'_>,
        y: usize,
        x0: usize,
        xsize: usize,
        extra_channel_info: &[ExtraChannelInfo],
        references: &[ReferenceFrame<'_>; 4],
    ) -> Status {
        let num_ec = extra_channel_info.len();
        let stride = self.blendings_stride.ok_or(StatusCode::GenericError)?;
        if num_ec + 1 > stride {
            return Err(StatusCode::GenericError);
        }
        for pos_idx in self.get_patches_for_row(y) {
            let pos = self.positions[pos_idx];
            let r = self.ref_positions[pos.ref_pos_idx];
            if y < pos.y || y >= pos.y.wrapping_add(r.ysize) {
                return Err(StatusCode::GenericError);
            }
            if pos.x >= x0.wrapping_add(xsize) || pos.x.wrapping_add(r.xsize) < x0 {
                continue;
            }
            let patch_x0 = pos.x.max(x0);
            let patch_x1 = pos.x.wrapping_add(r.xsize).min(x0.wrapping_add(xsize));
            let ib = references[r.reference].frame.as_ref().unwrap();
            // The source forms potentially-before-row pointers and only then
            // offsets them to the overlap. Resolve their final accessed ranges
            // instead, so no invalid intermediate reference can exist in Rust.
            let from_x = r.x0.wrapping_add(patch_x0).wrapping_sub(pos.x);
            let from_y = r.y0.wrapping_add(y).wrapping_sub(pos.y);
            let bg: Vec<_> = inout
                .iter()
                .map(|&row| &buffers.row(row)[patch_x0.wrapping_sub(x0)..])
                .collect();
            let fg: Vec<_> = (0..3 + num_ec)
                .map(|c| {
                    if c < 3 {
                        &ib.color().plane_row(c, from_y)[from_x..]
                    } else {
                        &ib.extra_channels()[c - 3].row(from_y)[from_x..]
                    }
                })
                .collect();
            let bi = pos_idx * stride;
            let tmp = super::blending::perform_blending_in(
                self.memory_manager,
                &bg,
                &fg,
                0,
                patch_x1.wrapping_sub(patch_x0),
                &self.blendings[bi],
                &self.blendings[bi + 1..bi + 1 + num_ec],
                extra_channel_info,
            )?;
            if patch_x1 == patch_x0 {
                continue;
            }
            for (c, &row) in inout.iter().take(3 + num_ec).enumerate() {
                buffers.row_mut(row)[patch_x0.wrapping_sub(x0)..patch_x1.wrapping_sub(x0)]
                    .copy_from_slice(&tmp.row(c)[..patch_x1.wrapping_sub(patch_x0)]);
            }
        }
        Ok(())
    }
}
