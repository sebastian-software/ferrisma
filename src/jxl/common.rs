// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! SpeedTier and the pass-count bound from `lib/jxl/common.h`.
//! Other shared constants remain unported.

pub const MAX_NUM_PASSES: usize = 11;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum SpeedTier {
    TectonicPlate = -1,
    Glacier = 0,
    Tortoise = 1,
    Kitten = 2,
    Squirrel = 3,
    Wombat = 4,
    Hare = 5,
    Cheetah = 6,
    Falcon = 7,
    Thunder = 8,
    Lightning = 9,
}
