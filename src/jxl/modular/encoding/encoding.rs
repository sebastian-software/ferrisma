// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! GroupHeader, FilterTree, and TreeToLookupTable from modular/encoding/
//! encoding.{h,cc}. Residual/image decoding remains absent. Conditional arrays
//! use vectors of exactly the source lengths (zero or 2 * PROP_RANGE_FAST).
//! Rust initializes LUT storage; callers must only observe source-defined data.

use super::context_predict::{
    EXTRA_PROPS_PER_CHANNEL, FlatDecisionNode, FlatTree, GRADIENT_PROP, NUM_NONREF_PROPERTIES,
    WP_PROP, weighted,
};
use super::dec_ma::PropertyDecisionNode;
use crate::jxl::base::status::Status;
use crate::jxl::field_encodings::{Fields, U32Enc, bits_offset, val};
use crate::jxl::fields::{Visitor, bundle};
use crate::jxl::modular::options::{NUM_STATIC_PROPERTIES, Predictor};
use crate::jxl::modular::transform::transform::Transform;
use std::collections::VecDeque;

pub const PROP_RANGE_FAST: i32 = 512 << 4;
#[derive(Clone, Debug)]
pub struct GroupHeader {
    pub use_global_tree: bool,
    pub wp_header: weighted::Header,
    pub transforms: Vec<Transform>,
}
impl Default for GroupHeader {
    fn default() -> Self {
        let mut h = Self {
            use_global_tree: false,
            wp_header: weighted::Header::default(),
            transforms: Vec::new(),
        };
        bundle::init(&mut h);
        h
    }
}
impl Fields for GroupHeader {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.bool(false, &mut self.use_global_tree)?;
        visitor.visit_nested(&mut self.wp_header)?;
        let mut num_transforms = self.transforms.len() as u32;
        visitor.u32(
            U32Enc::new(val(0), val(1), bits_offset(4, 2), bits_offset(8, 18)),
            0,
            &mut num_transforms,
        )?;
        if visitor.is_reading() {
            self.transforms
                .resize_with(num_transforms as usize, Transform::default);
        }
        for i in 0..num_transforms as usize {
            visitor.visit_nested(&mut self.transforms[i])?;
        }
        Ok(())
    }
}

pub fn filter_tree(
    global_tree: &[PropertyDecisionNode],
    static_props: &mut [i32; NUM_STATIC_PROPERTIES as usize],
    num_props: &mut usize,
    use_wp: &mut bool,
    wp_only: &mut bool,
    gradient_only: &mut bool,
) -> FlatTree {
    *num_props = 0;
    let mut has_wp = false;
    let mut has_non_wp = false;
    *gradient_only = true;
    let mut output = Vec::new();
    let mut nodes = VecDeque::from([0usize]);
    while let Some(mut cur) = nodes.pop_front() {
        while i32::from(global_tree[cur].property) < NUM_STATIC_PROPERTIES as i32
            && global_tree[cur].property != -1
        {
            cur = if static_props[global_tree[cur].property as usize] > global_tree[cur].splitval {
                global_tree[cur].lchild as usize
            } else {
                global_tree[cur].rchild as usize
            };
        }
        let node = &global_tree[cur];
        let mut flat = FlatDecisionNode::default();
        if node.property == -1 {
            flat.property0 = -1;
            flat.child_id = node.lchild;
            flat.set_predictor(node.predictor);
            flat.set_predictor_offset(node.predictor_offset as i32);
            flat.set_multiplier(node.multiplier as i32);
            *gradient_only &= flat.predictor() == Predictor::GRADIENT;
            has_wp |= flat.predictor() == Predictor::WEIGHTED;
            has_non_wp |= flat.predictor() != Predictor::WEIGHTED;
            output.push(flat);
            continue;
        }
        flat.child_id = (output.len() + nodes.len() + 1) as u32;
        flat.property0 = i32::from(node.property);
        *num_props = (*num_props).max((flat.property0 + 1) as usize);
        flat.set_splitval0(node.splitval);
        for i in 0..2 {
            let mut cur_child = if i == 0 { node.lchild } else { node.rchild } as usize;
            while i32::from(global_tree[cur_child].property) < NUM_STATIC_PROPERTIES as i32
                && global_tree[cur_child].property != -1
            {
                let child = &global_tree[cur_child];
                cur_child = if static_props[child.property as usize] > child.splitval {
                    child.lchild as usize
                } else {
                    child.rchild as usize
                };
            }
            let child = &global_tree[cur_child];
            if child.property == -1 {
                flat.set_property(i, 0);
                flat.set_splitval(i, 0);
                nodes.push_back(cur_child);
                nodes.push_back(cur_child);
            } else {
                flat.set_property(i, child.property);
                flat.set_splitval(i, child.splitval);
                nodes.push_back(child.lchild as usize);
                nodes.push_back(child.rchild as usize);
                *num_props = (*num_props).max((i32::from(flat.properties()[i]) + 1) as usize);
            }
        }
        let mut mark_property = |p: i32| {
            if p == WP_PROP as i32 {
                has_wp = true;
            } else if p >= NUM_STATIC_PROPERTIES as i32 {
                has_non_wp = true;
            }
            if p >= NUM_STATIC_PROPERTIES as i32 && p != GRADIENT_PROP as i32 {
                *gradient_only = false;
            }
        };
        for p in flat.properties() {
            mark_property(i32::from(p));
        }
        mark_property(flat.property0);
        output.push(flat);
    }
    if *num_props > NUM_NONREF_PROPERTIES {
        *num_props = (*num_props - NUM_NONREF_PROPERTIES).div_ceil(EXTRA_PROPS_PER_CHANNEL)
            * EXTRA_PROPS_PER_CHANNEL
            + NUM_NONREF_PROPERTIES;
    } else {
        *num_props = NUM_NONREF_PROPERTIES;
    }
    *use_wp = has_wp;
    *wp_only = has_wp && !has_non_wp;
    output
}

pub trait TreeLutWord: Copy + Default {
    fn from_context(context: u32) -> Self;
}
impl TreeLutWord for u8 {
    fn from_context(context: u32) -> Self {
        context as Self
    }
}
impl TreeLutWord for u16 {
    fn from_context(context: u32) -> Self {
        context as Self
    }
}
impl TreeLutWord for u32 {
    fn from_context(context: u32) -> Self {
        context
    }
}
pub struct TreeLut<T, const HAS_OFFSETS: bool, const HAS_MULTIPLIERS: bool> {
    pub context_lookup: [T; 2 * PROP_RANGE_FAST as usize],
    pub offsets: Vec<i8>,
    pub multipliers: Vec<i8>,
}
impl<T: TreeLutWord, const O: bool, const M: bool> Default for TreeLut<T, O, M> {
    fn default() -> Self {
        Self {
            context_lookup: [T::default(); 2 * PROP_RANGE_FAST as usize],
            offsets: vec![0; if O { 2 * PROP_RANGE_FAST as usize } else { 0 }],
            multipliers: vec![0; if M { 2 * PROP_RANGE_FAST as usize } else { 0 }],
        }
    }
}
pub fn tree_to_lookup_table<T: TreeLutWord, const O: bool, const M: bool>(
    tree: &[FlatDecisionNode],
    lut: &mut TreeLut<T, O, M>,
) -> bool {
    struct TreeRange {
        begin: i32,
        end: i32,
        pos: usize,
    }
    let mut ranges = vec![TreeRange {
        begin: -PROP_RANGE_FAST - 1,
        end: PROP_RANGE_FAST - 1,
        pos: 0,
    }];
    while let Some(cur) = ranges.pop() {
        if cur.begin < -PROP_RANGE_FAST - 1
            || cur.begin >= PROP_RANGE_FAST - 1
            || cur.end > PROP_RANGE_FAST - 1
        {
            return false;
        }
        let node = &tree[cur.pos];
        if node.property0 == -1 {
            if node.predictor_offset() < i32::from(i8::MIN)
                || node.predictor_offset() > i32::from(i8::MAX)
            {
                return false;
            }
            if node.multiplier() < i32::from(i8::MIN) || node.multiplier() > i32::from(i8::MAX) {
                return false;
            }
            if !M && node.multiplier() != 1 {
                return false;
            }
            if !O && node.predictor_offset() != 0 {
                return false;
            }
            for i in cur.begin + 1..cur.end + 1 {
                let j = (i + PROP_RANGE_FAST) as usize;
                lut.context_lookup[j] = T::from_context(node.child_id);
                if M {
                    lut.multipliers[j] = node.multiplier() as i8;
                }
                if O {
                    lut.offsets[j] = node.predictor_offset() as i8;
                }
            }
            continue;
        }
        if i32::from(node.properties()[0]) >= NUM_STATIC_PROPERTIES as i32 {
            ranges.push(TreeRange {
                begin: node.splitvals()[0],
                end: cur.end,
                pos: node.child_id as usize,
            });
            ranges.push(TreeRange {
                begin: node.splitval0(),
                end: node.splitvals()[0],
                pos: node.child_id.wrapping_add(1) as usize,
            });
        } else {
            ranges.push(TreeRange {
                begin: node.splitval0(),
                end: cur.end,
                pos: node.child_id as usize,
            });
        }
        if i32::from(node.properties()[1]) >= NUM_STATIC_PROPERTIES as i32 {
            ranges.push(TreeRange {
                begin: node.splitvals()[1],
                end: node.splitval0(),
                pos: node.child_id.wrapping_add(2) as usize,
            });
            ranges.push(TreeRange {
                begin: cur.begin,
                end: node.splitvals()[1],
                pos: node.child_id.wrapping_add(3) as usize,
            });
        } else {
            ranges.push(TreeRange {
                begin: cur.begin,
                end: node.splitval0(),
                pos: node.child_id.wrapping_add(2) as usize,
            });
        }
    }
    true
}
