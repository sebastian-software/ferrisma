// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Tree storage, validation, and parsing from modular/encoding/dec_ma.{h,cc}.
//! Owned vectors replace source workspaces; tree indices retain their widths.

use super::ma_common::*;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use crate::jxl::dec_bit_reader::BitReader;
use crate::jxl::modular::options::{NUM_MODULAR_PREDICTORS, Predictor};
use crate::jxl::pack_signed::unpack_signed;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PropertyDecisionNode {
    pub splitval: i32,
    pub property: i16,
    pub lchild: u32,
    pub rchild: u32,
    pub predictor: Predictor,
    pub predictor_offset: i64,
    pub multiplier: u32,
}
impl Default for PropertyDecisionNode {
    fn default() -> Self {
        Self::leaf(Predictor::ZERO, 0, 1)
    }
}
impl PropertyDecisionNode {
    pub const fn new(
        p: i32,
        splitval: i32,
        lchild: i32,
        rchild: i32,
        predictor: Predictor,
        predictor_offset: i64,
        multiplier: u32,
    ) -> Self {
        Self {
            splitval,
            property: p as i16,
            lchild: lchild as u32,
            rchild: rchild as u32,
            predictor,
            predictor_offset,
            multiplier,
        }
    }
    pub const fn leaf(predictor: Predictor, offset: i64, multiplier: u32) -> Self {
        Self::new(-1, 0, 0, 0, predictor, offset, multiplier)
    }
    pub const fn split(p: i32, splitval: i32, lchild: i32, rchild: i32) -> Self {
        let rchild = if rchild == -1 { lchild + 1 } else { rchild };
        Self::new(p, splitval, lchild, rchild, Predictor::ZERO, 0, 1)
    }
}
pub type Tree = Vec<PropertyDecisionNode>;

enum NextAction {
    CheckAndGoLeft,
    GoRight,
    Pop,
}
struct WorkItem {
    node_index: usize,
    orig_l: i32,
    orig_u: i32,
    action: NextAction,
}
impl WorkItem {
    fn new(node_index: usize) -> Self {
        Self {
            node_index,
            orig_l: 0,
            orig_u: 0,
            action: NextAction::CheckAndGoLeft,
        }
    }
}
fn validate_tree(tree: &[PropertyDecisionNode]) -> Status {
    if tree.is_empty() {
        return Ok(());
    }
    let mut num_properties = 0;
    for node in tree {
        if i32::from(node.property) >= num_properties {
            num_properties = i32::from(node.property) + 1;
        }
    }
    let mut property_ranges = vec![(i32::MIN, i32::MAX); num_properties as usize];
    const HEIGHT_LIMIT: usize = 2048;
    let mut stack = vec![WorkItem::new(0)];
    while !stack.is_empty() {
        if stack.len() >= HEIGHT_LIMIT {
            return Err(StatusCode::GenericError);
        }
        let item = stack.last_mut().unwrap();
        let node = &tree[item.node_index];
        match item.action {
            NextAction::CheckAndGoLeft => {
                let p = node.property;
                if p == -1 {
                    stack.pop();
                    continue;
                }
                let v = node.splitval;
                let (l, u) = property_ranges[p as usize];
                if l > v || u <= v {
                    return Err(StatusCode::GenericError);
                }
                item.orig_l = l;
                item.orig_u = u;
                item.action = NextAction::GoRight;
                property_ranges[p as usize].0 = v + 1;
                stack.push(WorkItem::new(node.lchild as usize));
            }
            NextAction::GoRight => {
                item.action = NextAction::Pop;
                property_ranges[node.property as usize].0 = item.orig_l;
                property_ranges[node.property as usize].1 = node.splitval;
                stack.push(WorkItem::new(node.rchild as usize));
            }
            NextAction::Pop => {
                property_ranges[node.property as usize].1 = item.orig_u;
                stack.pop();
            }
        }
    }
    Ok(())
}
fn decode_tree_tokens(
    br: &mut BitReader<'_>,
    reader: &mut ANSSymbolReader<'_>,
    context_map: &[u8],
    tree: &mut Tree,
    tree_size_limit: usize,
) -> Status {
    let mut leaf_id = 0usize;
    let mut to_decode = 1usize;
    tree.clear();
    while to_decode > 0 {
        br.all_reads_within_bounds()?;
        if tree.len() > tree_size_limit {
            return Err(StatusCode::GenericError);
        }
        to_decode -= 1;
        let prop1 = reader.read_hybrid_uint(PROPERTY_CONTEXT, br, context_map) as u32;
        if prop1 > 256 {
            return Err(StatusCode::GenericError);
        }
        let property = prop1.wrapping_sub(1) as i32;
        if property == -1 {
            let predictor = reader.read_hybrid_uint(PREDICTOR_CONTEXT, br, context_map);
            if predictor >= NUM_MODULAR_PREDICTORS {
                return Err(StatusCode::GenericError);
            }
            let predictor_offset = i64::from(unpack_signed(reader.read_hybrid_uint(
                OFFSET_CONTEXT,
                br,
                context_map,
            )));
            let mul_log = reader.read_hybrid_uint(MULTIPLIER_LOG_CONTEXT, br, context_map) as u32;
            if mul_log >= 31 {
                return Err(StatusCode::GenericError);
            }
            let mul_bits = reader.read_hybrid_uint(MULTIPLIER_BITS_CONTEXT, br, context_map) as u32;
            if mul_bits >= (1u32 << (31 - mul_log)) - 1 {
                return Err(StatusCode::GenericError);
            }
            let multiplier = (mul_bits + 1) << mul_log;
            tree.push(PropertyDecisionNode::new(
                -1,
                0,
                leaf_id as i32,
                0,
                Predictor(predictor as u32),
                predictor_offset,
                multiplier,
            ));
            leaf_id += 1;
            continue;
        }
        let splitval = unpack_signed(reader.read_hybrid_uint(SPLIT_VAL_CONTEXT, br, context_map));
        tree.push(PropertyDecisionNode::new(
            property,
            splitval,
            (tree.len() + to_decode + 1) as i32,
            (tree.len() + to_decode + 2) as i32,
            Predictor::ZERO,
            0,
            1,
        ));
        to_decode += 2;
    }
    validate_tree(tree)
}
pub fn decode_tree(br: &mut BitReader<'_>, tree: &mut Tree, tree_size_limit: usize) -> Status {
    let mut context_map = Vec::new();
    let mut code = ANSCode::default();
    decode_histograms(br, NUM_TREE_CONTEXTS, &mut code, &mut context_map, false)?;
    if code.degenerate_symbols[context_map[PROPERTY_CONTEXT] as usize] > 0 {
        return Err(StatusCode::GenericError);
    }
    let mut reader = ANSSymbolReader::create(&code, br, 0)?;
    decode_tree_tokens(
        br,
        &mut reader,
        &context_map,
        tree,
        tree_size_limit.min(MAX_TREE_SIZE),
    )?;
    if !reader.check_ans_final_state() {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
