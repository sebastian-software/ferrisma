// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/ans_params.h`.

pub const ANS_LOG_TAB_SIZE: u32 = 12;
pub const ANS_TAB_SIZE: u32 = 1 << ANS_LOG_TAB_SIZE;
pub const ANS_TAB_MASK: u32 = ANS_TAB_SIZE - 1;
pub const PREFIX_MAX_ALPHABET_SIZE: usize = 4096;
pub const ANS_MAX_ALPHABET_SIZE: usize = 256;
pub const PREFIX_MAX_BITS: usize = 15;
pub const ANS_SIGNATURE: u32 = 0x13;
