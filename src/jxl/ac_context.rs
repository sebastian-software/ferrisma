// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! BlockCtxMap storage/default construction from lib/jxl/ac_context.h.
//! Context lookup helpers and other AC context functions remain unported.
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
