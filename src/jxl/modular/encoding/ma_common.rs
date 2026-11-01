// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Tree contexts and size limit from lib/jxl/modular/encoding/ma_common.h.

pub const SPLIT_VAL_CONTEXT: usize = 0;
pub const PROPERTY_CONTEXT: usize = 1;
pub const PREDICTOR_CONTEXT: usize = 2;
pub const OFFSET_CONTEXT: usize = 3;
pub const MULTIPLIER_LOG_CONTEXT: usize = 4;
pub const MULTIPLIER_BITS_CONTEXT: usize = 5;
pub const NUM_TREE_CONTEXTS: usize = 6;
pub const MAX_TREE_SIZE: usize = 1 << 22;
