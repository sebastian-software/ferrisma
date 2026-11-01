// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar contexts and BlockCtxMap from `lib/jxl/ac_context.h`.

use super::coeff_order_fwd::NUM_ORDERS;

pub const DCT_ORDER_CONTEXT_START: u32 = 0;
pub const NON_ZERO_BUCKETS: u32 = 37;
pub const COEFF_FREQ_CONTEXT: [u16; 64] = [
    0xBAD, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19,
    19, 20, 20, 21, 21, 22, 22, 23, 23, 23, 23, 24, 24, 24, 24, 25, 25, 25, 25, 26, 26, 26, 26, 27,
    27, 27, 27, 28, 28, 28, 28, 29, 29, 29, 29, 30, 30, 30, 30,
];
pub const COEFF_NUM_NONZERO_CONTEXT: [u16; 64] = [
    0xBAD, 0, 31, 62, 62, 93, 93, 93, 93, 123, 123, 123, 123, 152, 152, 152, 152, 152, 152, 152,
    152, 180, 180, 180, 180, 180, 180, 180, 180, 180, 180, 180, 180, 206, 206, 206, 206, 206, 206,
    206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206, 206,
    206, 206, 206, 206, 206, 206,
];
pub const ZERO_DENSITY_CONTEXT_COUNT: i32 = 458;
pub const ZERO_DENSITY_CONTEXT_LIMIT: i32 = 474;

/// Retains the source bounds, including invalid streams whose nonzero count
/// and scan position sum exceeds 64. Both normalized indices must be in 1..64.
pub fn zero_density_context(
    mut nonzeros_left: usize,
    mut k: usize,
    covered_blocks: usize,
    log2_covered_blocks: usize,
    prev: usize,
) -> usize {
    debug_assert_eq!(1usize << log2_covered_blocks, covered_blocks);
    nonzeros_left =
        nonzeros_left.wrapping_add(covered_blocks).wrapping_sub(1) >> log2_covered_blocks;
    k >>= log2_covered_blocks;
    debug_assert!(k > 0 && k < 64);
    debug_assert!(nonzeros_left > 0 && nonzeros_left < 64);
    (usize::from(COEFF_NUM_NONZERO_CONTEXT[nonzeros_left]) + usize::from(COEFF_FREQ_CONTEXT[k]))
        .wrapping_mul(2)
        .wrapping_add(prev)
}

pub const DEFAULT_CTX_MAP: [u8; 39] = [
    0, 1, 2, 2, 3, 3, 4, 5, 6, 6, 6, 6, 6, 7, 8, 9, 9, 10, 11, 12, 13, 14, 14, 14, 14, 14, 7, 8, 9,
    9, 10, 11, 12, 13, 14, 14, 14, 14, 14,
];
pub struct BlockCtxMap {
    pub dc_thresholds: [Vec<i32>; 3],
    pub qf_thresholds: Vec<u32>,
    pub ctx_map: Vec<u8>,
    pub num_ctxs: usize,
    pub num_dc_ctxs: usize,
}
impl BlockCtxMap {
    /// The channel/order/DC indices must address the caller's complete map.
    pub fn context(&self, dc_idx: i32, qf: u32, ord: usize, c: usize) -> usize {
        let mut qf_idx = 0;
        for &t in &self.qf_thresholds {
            if qf > t {
                qf_idx += 1;
            }
        }
        let mut idx = if c < 2 { c ^ 1 } else { 2 };
        idx = idx.wrapping_mul(usize::from(NUM_ORDERS)).wrapping_add(ord);
        idx = idx
            .wrapping_mul(self.qf_thresholds.len() + 1)
            .wrapping_add(qf_idx);
        idx = idx
            .wrapping_mul(self.num_dc_ctxs)
            .wrapping_add(dc_idx as usize);
        usize::from(self.ctx_map[idx])
    }

    pub const fn zero_density_contexts_offset(&self, block_ctx: u32) -> u32 {
        self.num_ctxs
            .wrapping_mul(NON_ZERO_BUCKETS as usize)
            .wrapping_add((ZERO_DENSITY_CONTEXT_COUNT as u32).wrapping_mul(block_ctx) as usize)
            as u32
    }

    pub const fn num_ac_contexts(&self) -> u32 {
        self.num_ctxs
            .wrapping_mul((NON_ZERO_BUCKETS + ZERO_DENSITY_CONTEXT_COUNT as u32) as usize)
            as u32
    }

    pub fn non_zero_context(&self, mut non_zeros: u32, block_ctx: u32) -> u32 {
        if non_zeros >= 64 {
            non_zeros = 64;
        }
        let ctx = if non_zeros < 8 {
            non_zeros
        } else {
            4 + non_zeros / 2
        };
        (ctx as usize)
            .wrapping_mul(self.num_ctxs)
            .wrapping_add(block_ctx as usize) as u32
    }
}
impl Default for BlockCtxMap {
    fn default() -> Self {
        let dc_thresholds = std::array::from_fn(|_| Vec::new());
        let qf_thresholds = Vec::new();
        let ctx_map = DEFAULT_CTX_MAP.to_vec();
        let num_ctxs = usize::from(*ctx_map.iter().max().unwrap()) + 1;
        Self {
            dc_thresholds,
            qf_thresholds,
            ctx_map,
            num_ctxs,
            num_dc_ctxs: 1,
        }
    }
}
