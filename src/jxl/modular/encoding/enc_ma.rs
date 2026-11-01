// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! TokenizeTree from lib/jxl/modular/encoding/enc_ma.cc. Tree learning and
//! sample collection remain absent. VecDeque preserves the original BFS queue.

use super::dec_ma::{PropertyDecisionNode, Tree};
use super::ma_common::*;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::enc_ans::Token;
use crate::jxl::modular::options::Predictor;
use crate::jxl::pack_signed::pack_signed;
use std::collections::VecDeque;

pub fn tokenize_tree(
    tree: &[PropertyDecisionNode],
    tokens: &mut Vec<Token>,
    decoder_tree: &mut Tree,
) -> Status {
    if tree.len() > MAX_TREE_SIZE {
        return Err(StatusCode::GenericError);
    }
    let mut queue = VecDeque::from([0i32]);
    let mut leaf_id = 0usize;
    decoder_tree.clear();
    while let Some(cur) = queue.pop_front() {
        let node = &tree[cur as usize];
        if node.property < -1 {
            return Err(StatusCode::GenericError);
        }
        tokens.push(Token::new(
            PROPERTY_CONTEXT as u32,
            (i32::from(node.property) + 1) as u32,
        ));
        if node.property == -1 {
            tokens.push(Token::new(PREDICTOR_CONTEXT as u32, node.predictor.0));
            tokens.push(Token::new(
                OFFSET_CONTEXT as u32,
                pack_signed(node.predictor_offset as i32),
            ));
            // Num0BitsBelowLS1Bit_Nonzero requires a nonzero multiplier.
            assert_ne!(node.multiplier, 0);
            let mul_log = node.multiplier.trailing_zeros();
            let mul_bits = (node.multiplier >> mul_log) - 1;
            tokens.push(Token::new(MULTIPLIER_LOG_CONTEXT as u32, mul_log));
            tokens.push(Token::new(MULTIPLIER_BITS_CONTEXT as u32, mul_bits));
            if node.predictor.0 >= Predictor::BEST.0 {
                return Err(StatusCode::GenericError);
            }
            decoder_tree.push(PropertyDecisionNode::new(
                -1,
                0,
                leaf_id as i32,
                0,
                node.predictor,
                node.predictor_offset,
                node.multiplier,
            ));
            leaf_id += 1;
            continue;
        }
        decoder_tree.push(PropertyDecisionNode::new(
            i32::from(node.property),
            node.splitval,
            (decoder_tree.len() + queue.len() + 1) as i32,
            (decoder_tree.len() + queue.len() + 2) as i32,
            Predictor::ZERO,
            0,
            1,
        ));
        queue.push_back(node.lchild as i32);
        queue.push_back(node.rchild as i32);
        tokens.push(Token::new(
            SPLIT_VAL_CONTEXT as u32,
            pack_signed(node.splitval),
        ));
    }
    Ok(())
}
